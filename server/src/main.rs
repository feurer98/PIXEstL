//! pixestl-server — async HTTP wrapper around the `pixestl` CLI.
//!
//! Endpoints (all under /api):
//!   POST /jobs                 multipart: image, palette, settings(JSON), format
//!                              -> 202 { "jobId": "..." }
//!   GET  /jobs/:id             -> { "status", "error"?, "filename"? }
//!   GET  /jobs/:id/download    -> generated 3MF/ZIP bytes (when status == done)
//!   GET  /health               -> "ok"
//!
//! The job runs the `pixestl` binary as a subprocess. The binary is resolved
//! from $PIXESTL_BIN, else next to this server's executable, else `pixestl` on
//! PATH. See README.md for the settings -> CLI flag mapping.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use axum::body::Body;
use axum::extract::{Multipart, Path as AxPath, State};
use axum::http::{header, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use tempfile::TempDir;
use tokio::process::Command;
use uuid::Uuid;

// ─── State & job model ───────────────────────────────────────────────────────

#[derive(Clone)]
struct AppState {
    jobs: Arc<Mutex<HashMap<String, Job>>>,
    pixestl_bin: PathBuf,
}

#[derive(Clone, Copy, PartialEq)]
enum JobStatus {
    Queued,
    Running,
    Done,
    Error,
}

impl JobStatus {
    fn as_str(self) -> &'static str {
        match self {
            JobStatus::Queued => "queued",
            JobStatus::Running => "running",
            JobStatus::Done => "done",
            JobStatus::Error => "error",
        }
    }
}

struct Job {
    status: JobStatus,
    filename: String,
    output_path: Option<PathBuf>,
    error: Option<String>,
    // Keeps the temp dir (and thus the generated file) alive while the job exists.
    _dir: Arc<TempDir>,
}

// ─── Frontend settings payload (mirrors src/lib/types.ts Settings) ───────────

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Settings {
    width: f64,
    height: f64,
    plate_thickness: f64,
    curve: f64,
    color_pixel_width: f64,
    color_layer_thickness: f64,
    color_layers: u32,
    pixel_method: String,   // "additive" | "full"
    color_matching: String, // "cie-lab" | "rgb"
    ams_colors: u32,
    texture_pixel_width: f64,
    texture_min: f64,
    texture_max: f64,
    texture_color: String,
    enable_color: bool,
    enable_texture: bool,
}

/// Translate the frontend settings + paths + output format into `pixestl` args.
fn build_args(s: &Settings, palette: &Path, input: &Path, output: &Path) -> Vec<String> {
    let mut a: Vec<String> = vec![
        "--palette".into(), palette.display().to_string(),
        "--input".into(), input.display().to_string(),
        "--output".into(), output.display().to_string(),
        "--width".into(), s.width.to_string(),
        "--height".into(), s.height.to_string(),
        "--color-pixel-width".into(), s.color_pixel_width.to_string(),
        "--color-layer-thickness".into(), s.color_layer_thickness.to_string(),
        "--color-layers".into(), s.color_layers.to_string(),
        "--texture-pixel-width".into(), s.texture_pixel_width.to_string(),
        "--texture-min".into(), s.texture_min.to_string(),
        "--texture-max".into(), s.texture_max.to_string(),
        "--texture-color".into(), s.texture_color.clone(),
        "--plate-thickness".into(), s.plate_thickness.to_string(),
        "--color-distance".into(), s.color_matching.clone(),
        "--pixel-method".into(), s.pixel_method.clone(),
        "--color-number".into(), s.ams_colors.to_string(),
        "--curve".into(), s.curve.to_string(),
    ];
    if !s.enable_color {
        a.push("--no-color".into());
    }
    if !s.enable_texture {
        a.push("--no-texture".into());
    }
    a
}

/// True for palette keys that are `#rrggbb` hex colors.
fn is_hex_key(k: &str) -> bool {
    k.len() == 7 && k.starts_with('#') && k[1..].chars().all(|c| c.is_ascii_hexdigit())
}

/// Apply the frontend's filament toggles (V-MODEL-13) to a hex-keyed PIXEstL
/// palette: each color's `active` flag is set to whether it is in `active`
/// (case-insensitive). Returns `None` if the palette is not a hex-keyed object
/// (then the caller keeps the original bytes).
fn apply_active_toggles(palette_bytes: &[u8], active: &[String]) -> Option<Vec<u8>> {
    let mut value: serde_json::Value = serde_json::from_slice(palette_bytes).ok()?;
    let obj = value.as_object_mut()?;
    let active_lower: HashSet<String> = active.iter().map(|s| s.to_lowercase()).collect();

    let mut changed = false;
    for (key, val) in obj.iter_mut() {
        if is_hex_key(key) {
            if let Some(entry) = val.as_object_mut() {
                let on = active_lower.contains(&key.to_lowercase());
                entry.insert("active".into(), serde_json::Value::Bool(on));
                changed = true;
            }
        }
    }
    if changed {
        serde_json::to_vec(&value).ok()
    } else {
        None
    }
}

// ─── Handlers ────────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct CreateResp {
    #[serde(rename = "jobId")]
    job_id: String,
}

type ApiError = (StatusCode, String);

fn bad(msg: impl Into<String>) -> ApiError {
    (StatusCode::BAD_REQUEST, msg.into())
}

async fn create_job(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, ApiError> {
    let mut image: Option<(String, Vec<u8>)> = None;
    let mut palette: Option<Vec<u8>> = None;
    let mut settings_raw: Option<String> = None;
    let mut format: Option<String> = None;
    let mut active_colors_raw: Option<String> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| bad(format!("invalid multipart: {e}")))?
    {
        let name = field.name().unwrap_or_default().to_string();
        let file_name = field.file_name().map(|s| s.to_string());
        match name.as_str() {
            "image" => {
                let bytes = field.bytes().await.map_err(|e| bad(format!("image: {e}")))?;
                image = Some((file_name.unwrap_or_else(|| "input.png".into()), bytes.to_vec()));
            }
            "palette" => {
                let bytes = field.bytes().await.map_err(|e| bad(format!("palette: {e}")))?;
                palette = Some(bytes.to_vec());
            }
            "settings" => {
                settings_raw = Some(field.text().await.map_err(|e| bad(format!("settings: {e}")))?);
            }
            "format" => {
                format = Some(field.text().await.map_err(|e| bad(format!("format: {e}")))?);
            }
            "activeColors" => {
                active_colors_raw =
                    Some(field.text().await.map_err(|e| bad(format!("activeColors: {e}")))?);
            }
            _ => {}
        }
    }

    let (img_name, img_bytes) = image.ok_or_else(|| bad("missing 'image' file"))?;
    let mut palette_bytes = palette.ok_or_else(|| bad("missing 'palette' file"))?;

    // Apply the UI filament toggles to the palette before generation (V-MODEL-13).
    if let Some(raw) = active_colors_raw {
        let active: Vec<String> = serde_json::from_str(&raw)
            .map_err(|e| bad(format!("invalid activeColors JSON: {e}")))?;
        if let Some(patched) = apply_active_toggles(&palette_bytes, &active) {
            palette_bytes = patched;
        }
    }
    let settings: Settings = serde_json::from_str(&settings_raw.ok_or_else(|| bad("missing 'settings'"))?)
        .map_err(|e| bad(format!("invalid settings JSON: {e}")))?;
    let format = format.unwrap_or_else(|| "3mf".into());
    if format != "3mf" && format != "zip" {
        return Err(bad("format must be '3mf' or 'zip'"));
    }

    // Stage inputs in a temp dir that lives as long as the job.
    let dir = TempDir::new().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let ext = Path::new(&img_name)
        .extension()
        .and_then(|e| e.to_str())
        .filter(|e| e.len() <= 5)
        .unwrap_or("png");
    let input_path = dir.path().join(format!("input.{ext}"));
    let palette_path = dir.path().join("palette.json");
    let filename = format!("lithophane.{format}");
    let output_path = dir.path().join(&filename);

    tokio::fs::write(&input_path, &img_bytes)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    tokio::fs::write(&palette_path, &palette_bytes)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let args = build_args(&settings, &palette_path, &input_path, &output_path);

    let id = Uuid::new_v4().to_string();
    let dir = Arc::new(dir);
    {
        let mut jobs = state.jobs.lock().unwrap();
        jobs.insert(
            id.clone(),
            Job {
                status: JobStatus::Queued,
                filename: filename.clone(),
                output_path: None,
                error: None,
                _dir: dir.clone(),
            },
        );
    }

    // Run the generation in the background; the client polls GET /jobs/:id.
    let bin = state.pixestl_bin.clone();
    let jobs = state.jobs.clone();
    let job_id = id.clone();
    tokio::spawn(async move {
        set_status(&jobs, &job_id, JobStatus::Running);
        let result = Command::new(&bin).args(&args).output().await;
        let mut map = jobs.lock().unwrap();
        if let Some(job) = map.get_mut(&job_id) {
            match result {
                Ok(out) if out.status.success() => {
                    job.status = JobStatus::Done;
                    job.output_path = Some(output_path);
                }
                Ok(out) => {
                    job.status = JobStatus::Error;
                    let msg = String::from_utf8_lossy(&out.stderr);
                    job.error = Some(if msg.trim().is_empty() {
                        format!("pixestl exited with {}", out.status)
                    } else {
                        msg.trim().to_string()
                    });
                }
                Err(e) => {
                    job.status = JobStatus::Error;
                    job.error = Some(format!("failed to run pixestl: {e}"));
                }
            }
        }
    });

    Ok((StatusCode::ACCEPTED, Json(CreateResp { job_id: id })))
}

fn set_status(jobs: &Arc<Mutex<HashMap<String, Job>>>, id: &str, status: JobStatus) {
    if let Some(job) = jobs.lock().unwrap().get_mut(id) {
        job.status = status;
    }
}

#[derive(Serialize)]
struct JobView {
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filename: Option<String>,
}

async fn get_job(
    State(state): State<AppState>,
    AxPath(id): AxPath<String>,
) -> Result<Json<JobView>, ApiError> {
    let jobs = state.jobs.lock().unwrap();
    let job = jobs.get(&id).ok_or_else(|| (StatusCode::NOT_FOUND, "unknown job".into()))?;
    Ok(Json(JobView {
        status: job.status.as_str().to_string(),
        error: job.error.clone(),
        filename: (job.status == JobStatus::Done).then(|| job.filename.clone()),
    }))
}

async fn download(
    State(state): State<AppState>,
    AxPath(id): AxPath<String>,
) -> Result<Response, ApiError> {
    let (path, filename) = {
        let jobs = state.jobs.lock().unwrap();
        let job = jobs.get(&id).ok_or_else(|| (StatusCode::NOT_FOUND, "unknown job".into()))?;
        match (&job.status, &job.output_path) {
            (JobStatus::Done, Some(p)) => (p.clone(), job.filename.clone()),
            _ => return Err((StatusCode::CONFLICT, "job not finished".into())),
        }
    };

    let bytes = tokio::fs::read(&path)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let content_type = if filename.ends_with(".zip") {
        "application/zip"
    } else {
        "model/3mf"
    };

    let mut resp = Response::new(Body::from(bytes));
    resp.headers_mut()
        .insert(header::CONTENT_TYPE, HeaderValue::from_static(content_type));
    if let Ok(v) = HeaderValue::from_str(&format!("attachment; filename=\"{filename}\"")) {
        resp.headers_mut().insert(header::CONTENT_DISPOSITION, v);
    }
    Ok(resp)
}

async fn health() -> &'static str {
    "ok"
}

// ─── Binary resolution & bootstrap ───────────────────────────────────────────

/// Resolve the `pixestl` binary: $PIXESTL_BIN, else next to this executable,
/// else bare `pixestl` (found via PATH at spawn time).
fn resolve_pixestl_bin() -> PathBuf {
    if let Ok(p) = std::env::var("PIXESTL_BIN") {
        return PathBuf::from(p);
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let candidate = dir.join("pixestl");
            if candidate.exists() {
                return candidate;
            }
        }
    }
    PathBuf::from("pixestl")
}

#[tokio::main]
async fn main() {
    let pixestl_bin = resolve_pixestl_bin();
    let state = AppState {
        jobs: Arc::new(Mutex::new(HashMap::new())),
        pixestl_bin: pixestl_bin.clone(),
    };

    let app = Router::new()
        .route("/api/jobs", post(create_job))
        .route("/api/jobs/:id", get(get_job))
        .route("/api/jobs/:id/download", get(download))
        .route("/api/health", get(health))
        // Permissive CORS is fine for local dev; restrict in production.
        .layer(tower_http::cors::CorsLayer::permissive())
        .with_state(state);

    let port = std::env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(8787u16);
    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|e| panic!("cannot bind {addr}: {e}"));

    eprintln!("pixestl-server listening on http://{addr}");
    eprintln!("using pixestl binary: {}", pixestl_bin.display());
    axum::serve(listener, app).await.expect("server error");
}

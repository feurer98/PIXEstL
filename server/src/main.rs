//! pixestl-server — async HTTP wrapper around the `pixestl` CLI.
//!
//! Endpoints (all under /api):
//!   POST /jobs                 multipart: image, palette, settings(JSON), format
//!                              -> 202 { "jobId": "..." }
//!   GET  /jobs/:id             -> { "status", "error"?, "filename"? }
//!   GET  /jobs/:id/download    -> generated 3MF/ZIP bytes (when done); deletes job
//!   GET  /health               -> "ok"
//!
//! The job runs the `pixestl` binary as a subprocess. The binary is resolved
//! from $PIXESTL_BIN, else next to this server's executable, else `pixestl` on
//! PATH. See README.md for the settings -> CLI flag mapping.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{RwLock, Semaphore};

use axum::body::Body;
use axum::extract::{DefaultBodyLimit, Multipart, Path as AxPath, State};
use axum::http::{header, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use tempfile::TempDir;
use tokio::process::Command;
use tower_http::services::{ServeDir, ServeFile};
use uuid::Uuid;

// ─── State & job model ───────────────────────────────────────────────────────

#[derive(Clone)]
struct AppState {
    jobs: Arc<RwLock<HashMap<String, Job>>>,
    pixestl_bin: PathBuf,
    /// Limits the number of concurrently running `pixestl` subprocesses.
    sem: Arc<Semaphore>,
    /// Hard wall-clock limit for a single generation; runaway jobs are killed.
    job_timeout: Duration,
    /// Reject jobs whose estimated mesh exceeds this many triangles (OOM guard).
    max_triangles: u64,
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
        "--palette".into(),
        palette.display().to_string(),
        "--input".into(),
        input.display().to_string(),
        "--output".into(),
        output.display().to_string(),
        "--width".into(),
        s.width.to_string(),
        "--height".into(),
        s.height.to_string(),
        "--color-pixel-width".into(),
        s.color_pixel_width.to_string(),
        "--color-layer-thickness".into(),
        s.color_layer_thickness.to_string(),
        "--color-layers".into(),
        s.color_layers.to_string(),
        "--texture-pixel-width".into(),
        s.texture_pixel_width.to_string(),
        "--texture-min".into(),
        s.texture_min.to_string(),
        "--texture-max".into(),
        s.texture_max.to_string(),
        "--texture-color".into(),
        s.texture_color.clone(),
        "--plate-thickness".into(),
        s.plate_thickness.to_string(),
        "--color-distance".into(),
        s.color_matching.clone(),
        "--pixel-method".into(),
        s.pixel_method.clone(),
        "--color-number".into(),
        s.ams_colors.to_string(),
        "--curve".into(),
        s.curve.to_string(),
    ];
    if !s.enable_color {
        a.push("--no-color".into());
    }
    if !s.enable_texture {
        a.push("--no-texture".into());
    }
    a
}

/// Conservative upper-bound estimate of the triangles the generator will emit.
/// The whole mesh is held in memory (≈72 bytes/triangle) and then serialized, so
/// an oversized request can OOM the process — and a dead backend makes the
/// reverse proxy answer 502. We estimate the size up front and reject with a
/// clean 400 instead of letting the container get OOM-killed.
fn estimate_triangles(s: &Settings) -> u64 {
    let cells = |pixel_width: f64| -> u64 {
        if pixel_width <= 0.0 || !pixel_width.is_finite() {
            return u64::MAX; // invalid → treat as "too large", rejected by the caller
        }
        let cols = (s.width / pixel_width).ceil().max(0.0) as u64;
        let rows = (s.height / pixel_width).ceil().max(0.0) as u64;
        cols.saturating_mul(rows)
    };
    let mut tris: u64 = 0;
    if s.enable_texture {
        // ~2 surface triangles per cell, plus walls/base — use 4× as a safe bound.
        tris = tris.saturating_add(cells(s.texture_pixel_width).saturating_mul(4));
    }
    if s.enable_color {
        // Color cubes use run-length merging; ~2 triangles per cell per layer.
        let per_cell = 2u64.saturating_mul(s.color_layers.max(1) as u64);
        tris = tris.saturating_add(cells(s.color_pixel_width).saturating_mul(per_cell));
    }
    tris
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
            match val.as_object_mut() {
                Some(entry) => {
                    let on = active_lower.contains(&key.to_lowercase());
                    entry.insert("active".into(), serde_json::Value::Bool(on));
                    changed = true;
                }
                None => return None,
            }
        }
    }
    if changed {
        serde_json::to_vec(&value).ok()
    } else {
        None
    }
}

/// Returns `Err(bad(...))` if any f64 setting is non-finite (NaN, ±Inf).
/// Without this guard, `f64::to_string()` can produce `"inf"` / `"NaN"` which
/// the CLI rejects with a non-zero exit code, causing a 500 instead of a 400.
fn validate_settings(s: &Settings) -> Result<(), ApiError> {
    let fields: &[(&str, f64)] = &[
        ("width", s.width),
        ("height", s.height),
        ("plateThickness", s.plate_thickness),
        ("curve", s.curve),
        ("colorPixelWidth", s.color_pixel_width),
        ("colorLayerThickness", s.color_layer_thickness),
        ("texturePixelWidth", s.texture_pixel_width),
        ("textureMin", s.texture_min),
        ("textureMax", s.texture_max),
    ];
    for (name, val) in fields {
        if !val.is_finite() {
            return Err(bad(format!(
                "settings.{name} must be a finite number, got {val}"
            )));
        }
    }

    // Dimensions and pixel widths must be positive — a zero/negative pixel width
    // would divide into an absurd grid (and divide-by-zero in size estimates).
    let positive: &[(&str, f64)] = &[
        ("width", s.width),
        ("height", s.height),
        ("colorPixelWidth", s.color_pixel_width),
        ("texturePixelWidth", s.texture_pixel_width),
    ];
    for (name, val) in positive {
        if *val <= 0.0 {
            return Err(bad(format!(
                "settings.{name} must be greater than 0, got {val}"
            )));
        }
    }

    if !["cie-lab", "rgb"].contains(&s.color_matching.as_str()) {
        return Err(bad(format!(
            "settings.colorMatching must be one of [\"cie-lab\", \"rgb\"], got {:?}",
            s.color_matching
        )));
    }

    if !["additive", "full"].contains(&s.pixel_method.as_str()) {
        return Err(bad(format!(
            "settings.pixelMethod must be one of [\"additive\", \"full\"], got {:?}",
            s.pixel_method
        )));
    }

    if s.texture_color.len() != 7
        || !s.texture_color.starts_with('#')
        || !s.texture_color[1..].chars().all(|c| c.is_ascii_hexdigit())
    {
        return Err(bad(format!(
            "settings.textureColor must be a valid #rrggbb hex color, got {:?}",
            s.texture_color
        )));
    }

    Ok(())
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
                let bytes = field
                    .bytes()
                    .await
                    .map_err(|e| bad(format!("image: {e}")))?;
                image = Some((
                    file_name.unwrap_or_else(|| "input.png".into()),
                    bytes.to_vec(),
                ));
            }
            "palette" => {
                let bytes = field
                    .bytes()
                    .await
                    .map_err(|e| bad(format!("palette: {e}")))?;
                palette = Some(bytes.to_vec());
            }
            "settings" => {
                settings_raw = Some(
                    field
                        .text()
                        .await
                        .map_err(|e| bad(format!("settings: {e}")))?,
                );
            }
            "format" => {
                format = Some(
                    field
                        .text()
                        .await
                        .map_err(|e| bad(format!("format: {e}")))?,
                );
            }
            "activeColors" => {
                active_colors_raw = Some(
                    field
                        .text()
                        .await
                        .map_err(|e| bad(format!("activeColors: {e}")))?,
                );
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
    let settings: Settings =
        serde_json::from_str(&settings_raw.ok_or_else(|| bad("missing 'settings'"))?)
            .map_err(|e| bad(format!("invalid settings JSON: {e}")))?;
    validate_settings(&settings)?;

    // Reject models that would exhaust memory before we even spawn the CLI.
    let est_triangles = estimate_triangles(&settings);
    if est_triangles > state.max_triangles {
        return Err(bad(format!(
            "Modell zu groß (~{est_triangles} Dreiecke, Limit {}). \
             Bitte Breite/Höhe reduzieren oder die Pixelbreite erhöhen.",
            state.max_triangles
        )));
    }

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
        let mut jobs = state.jobs.write().await;
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
    let sem = state.sem.clone();
    let job_timeout = state.job_timeout;
    let job_id = id.clone();
    tokio::spawn(async move {
        // Acquire a permit before spawning the subprocess; released on drop.
        let _permit = sem.acquire_owned().await;
        set_status(&jobs, &job_id, JobStatus::Running).await;

        // Spawn with a hard timeout. `kill_on_drop` guarantees the child is
        // killed when the timeout future is dropped, so a runaway generation can
        // never hold memory/CPU forever (which would OOM the container → 502).
        let outcome: Result<PathBuf, String> = {
            use std::process::Stdio;
            match Command::new(&bin)
                .args(&args)
                .stdin(Stdio::null())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .kill_on_drop(true)
                .spawn()
            {
                Err(e) => Err(format!("failed to run pixestl: {e}")),
                Ok(child) => {
                    match tokio::time::timeout(job_timeout, child.wait_with_output()).await {
                        Err(_elapsed) => Err(format!(
                            "Zeitüberschreitung nach {}s — bitte kleinere Maße oder eine \
                             größere Pixelbreite wählen",
                            job_timeout.as_secs()
                        )),
                        Ok(Err(e)) => Err(format!("failed to run pixestl: {e}")),
                        Ok(Ok(out)) if out.status.success() => Ok(output_path),
                        Ok(Ok(out)) => {
                            let msg = String::from_utf8_lossy(&out.stderr);
                            Err(if msg.trim().is_empty() {
                                format!("pixestl exited with {}", out.status)
                            } else {
                                msg.trim().to_string()
                            })
                        }
                    }
                }
            }
        };

        let mut map = jobs.write().await;
        if let Some(job) = map.get_mut(&job_id) {
            match outcome {
                Ok(path) => {
                    job.status = JobStatus::Done;
                    job.output_path = Some(path);
                }
                Err(msg) => {
                    job.status = JobStatus::Error;
                    job.error = Some(msg);
                }
            }
        }
    });

    Ok((StatusCode::ACCEPTED, Json(CreateResp { job_id: id })))
}

async fn set_status(jobs: &Arc<RwLock<HashMap<String, Job>>>, id: &str, status: JobStatus) {
    if let Some(job) = jobs.write().await.get_mut(id) {
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
    let mut jobs = state.jobs.write().await;
    let job = jobs
        .get(&id)
        .ok_or_else(|| (StatusCode::NOT_FOUND, "unknown job".into()))?;
    let view = Json(JobView {
        status: job.status.as_str().to_string(),
        error: job.error.clone(),
        filename: (job.status == JobStatus::Done).then(|| job.filename.clone()),
    });
    // Error jobs are never downloaded; remove them on first read to avoid leaking
    // the job entry (and its empty TempDir) forever.
    if job.status == JobStatus::Error {
        jobs.remove(&id);
    }
    Ok(view)
}

async fn download(
    State(state): State<AppState>,
    AxPath(id): AxPath<String>,
) -> Result<Response, ApiError> {
    let (path, filename) = {
        let jobs = state.jobs.read().await;
        let job = jobs
            .get(&id)
            .ok_or_else(|| (StatusCode::NOT_FOUND, "unknown job".into()))?;
        match (&job.status, &job.output_path) {
            (JobStatus::Done, Some(p)) => (p.clone(), job.filename.clone()),
            _ => return Err((StatusCode::CONFLICT, "job not finished".into())),
        }
    };

    let bytes = tokio::fs::read(&path).await;
    // Remove the job and drop the TempDir regardless of read success or failure.
    state.jobs.write().await.remove(&id);
    let bytes = bytes.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

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

fn build_app(state: AppState) -> Router {
    let static_dir = std::env::var("STATIC_DIR").unwrap_or_else(|_| "static".into());
    let spa = ServeDir::new(&static_dir)
        .not_found_service(ServeFile::new(format!("{static_dir}/index.html")));

    Router::new()
        .route("/api/jobs", post(create_job))
        .route("/api/jobs/:id", get(get_job))
        .route("/api/jobs/:id/download", get(download))
        .route("/api/health", get(health))
        .fallback_service(spa)
        .layer(DefaultBodyLimit::max(64 * 1024 * 1024))
        .layer(tower_http::cors::CorsLayer::permissive())
        .with_state(state)
}

#[tokio::main]
async fn main() {
    let pixestl_bin = resolve_pixestl_bin();

    // Concurrency: each generation already saturates the CPU (rayon) and holds
    // its whole mesh in RAM, so running many at once mostly multiplies memory.
    // Default to a small number; raise via PIXESTL_MAX_JOBS if you have the RAM.
    let max_jobs = std::env::var("PIXESTL_MAX_JOBS")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|n| *n > 0)
        .unwrap_or_else(|| {
            std::thread::available_parallelism()
                .map(|n| n.get().min(2))
                .unwrap_or(2)
        });

    // Per-job wall-clock timeout (kept below the frontend's 5-minute poll budget
    // so the user gets a clear server error rather than a generic client timeout).
    let job_timeout = Duration::from_secs(
        std::env::var("PIXESTL_JOB_TIMEOUT_SECS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .filter(|n| *n > 0)
            .unwrap_or(240),
    );

    // OOM guard: reject requests whose estimated mesh exceeds this. Lower it on
    // memory-constrained hosts (a triangle is ≈72 bytes, held twice while saving).
    let max_triangles = std::env::var("PIXESTL_MAX_TRIANGLES")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .filter(|n| *n > 0)
        .unwrap_or(20_000_000);

    let state = AppState {
        jobs: Arc::new(RwLock::new(HashMap::new())),
        pixestl_bin: pixestl_bin.clone(),
        sem: Arc::new(Semaphore::new(max_jobs)),
        job_timeout,
        max_triangles,
    };

    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8787u16);
    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|e| panic!("cannot bind {addr}: {e}"));

    eprintln!("pixestl-server listening on http://{addr}");
    eprintln!("using pixestl binary: {}", pixestl_bin.display());
    eprintln!(
        "limits: max_jobs={max_jobs}, job_timeout={}s, max_triangles={max_triangles}",
        job_timeout.as_secs()
    );
    axum::serve(listener, build_app(state))
        .await
        .expect("server error");
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Method, Request, StatusCode};
    use std::collections::HashMap;
    use std::path::{Path, PathBuf};
    use std::sync::Arc;
    use std::time::Duration;
    use tempfile::TempDir;
    use tokio::sync::{RwLock, Semaphore};
    use tower::ServiceExt;

    fn test_state() -> AppState {
        AppState {
            jobs: Arc::new(RwLock::new(HashMap::new())),
            pixestl_bin: PathBuf::from("pixestl"),
            sem: Arc::new(Semaphore::new(2)),
            job_timeout: Duration::from_secs(240),
            max_triangles: 20_000_000,
        }
    }

    fn base_settings() -> Settings {
        Settings {
            width: 100.0,
            height: 75.0,
            plate_thickness: 2.0,
            curve: 0.0,
            color_pixel_width: 1.0,
            color_layer_thickness: 0.12,
            color_layers: 5,
            pixel_method: "additive".to_string(),
            color_matching: "cie-lab".to_string(),
            ams_colors: 4,
            texture_pixel_width: 1.0,
            texture_min: 0.0,
            texture_max: 2.0,
            texture_color: "#FFFFFF".to_string(),
            enable_color: true,
            enable_texture: true,
        }
    }

    fn multipart_body(boundary: &str, parts: &[(&str, Option<&str>, &[u8])]) -> Vec<u8> {
        let mut body = Vec::new();
        for (name, filename, content) in parts {
            body.extend_from_slice(format!("--{boundary}\r\n").as_bytes());
            if let Some(fname) = filename {
                body.extend_from_slice(
                    format!(
                        "Content-Disposition: form-data; name=\"{name}\"; filename=\"{fname}\"\r\n"
                    )
                    .as_bytes(),
                );
            } else {
                body.extend_from_slice(
                    format!("Content-Disposition: form-data; name=\"{name}\"\r\n").as_bytes(),
                );
            }
            body.extend_from_slice(b"\r\n");
            body.extend_from_slice(content);
            body.extend_from_slice(b"\r\n");
        }
        body.extend_from_slice(format!("--{boundary}--\r\n").as_bytes());
        body
    }

    fn default_settings_json() -> String {
        serde_json::json!({
            "width": 100.0,
            "height": 75.0,
            "plateThickness": 2.0,
            "curve": 0.0,
            "colorPixelWidth": 1.0,
            "colorLayerThickness": 0.12,
            "colorLayers": 5,
            "pixelMethod": "additive",
            "colorMatching": "cie-lab",
            "amsColors": 4,
            "texturePixelWidth": 1.0,
            "textureMin": 0.0,
            "textureMax": 2.0,
            "textureColor": "#FFFFFF",
            "enableColor": true,
            "enableTexture": true
        })
        .to_string()
    }

    #[tokio::test]
    async fn test_health() {
        let app = build_app(test_state());
        let req = Request::builder()
            .method(Method::GET)
            .uri("/api/health")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(resp.into_body(), 64).await.unwrap();
        assert_eq!(bytes.as_ref(), b"ok");
    }

    #[tokio::test]
    async fn test_get_unknown_job() {
        let app = build_app(test_state());
        let req = Request::builder()
            .method(Method::GET)
            .uri("/api/jobs/no-such-id")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_job_missing_image() {
        let app = build_app(test_state());
        let boundary = "test_boundary";
        let settings = default_settings_json();
        let body = multipart_body(
            boundary,
            &[
                ("settings", None, settings.as_bytes()),
                ("format", None, b"3mf"),
            ],
        );
        let req = Request::builder()
            .method(Method::POST)
            .uri("/api/jobs")
            .header(
                "content-type",
                format!("multipart/form-data; boundary={boundary}"),
            )
            .body(Body::from(body))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_create_job_invalid_format() {
        let app = build_app(test_state());
        let boundary = "test_boundary";
        let settings = default_settings_json();
        let palette = r##"{"#FFFFFF":{"name":"White","active":true,"layers":{"Filled":5}}}"##;
        let body = multipart_body(
            boundary,
            &[
                ("image", Some("test.png"), b"\x89PNG"),
                ("palette", Some("palette.json"), palette.as_bytes()),
                ("settings", None, settings.as_bytes()),
                ("format", None, b"stl"),
            ],
        );
        let req = Request::builder()
            .method(Method::POST)
            .uri("/api/jobs")
            .header(
                "content-type",
                format!("multipart/form-data; boundary={boundary}"),
            )
            .body(Body::from(body))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_create_job_returns_job_id() {
        let app = build_app(test_state());
        let boundary = "test_boundary";
        let settings = default_settings_json();
        let palette = r##"{"#FFFFFF":{"name":"White","active":true,"layers":{"Filled":5}}}"##;
        let body = multipart_body(
            boundary,
            &[
                ("image", Some("input.png"), b"\x89PNG fake"),
                ("palette", Some("palette.json"), palette.as_bytes()),
                ("settings", None, settings.as_bytes()),
                ("format", None, b"3mf"),
            ],
        );
        let req = Request::builder()
            .method(Method::POST)
            .uri("/api/jobs")
            .header(
                "content-type",
                format!("multipart/form-data; boundary={boundary}"),
            )
            .body(Body::from(body))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::ACCEPTED);
        let bytes = axum::body::to_bytes(resp.into_body(), 256).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert!(json["jobId"].as_str().is_some());
    }

    #[tokio::test]
    async fn test_download_not_ready() {
        let state = test_state();
        let job_id = "test-job-queued".to_string();
        {
            let dir = Arc::new(TempDir::new().unwrap());
            let mut jobs = state.jobs.write().await;
            jobs.insert(
                job_id.clone(),
                Job {
                    status: JobStatus::Queued,
                    filename: "lithophane.3mf".to_string(),
                    output_path: None,
                    error: None,
                    _dir: dir,
                },
            );
        }
        let app = build_app(state);
        let req = Request::builder()
            .method(Method::GET)
            .uri(format!("/api/jobs/{job_id}/download"))
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::CONFLICT);
    }

    #[tokio::test]
    async fn test_download_removes_job() {
        let state = test_state();
        let job_id = "test-job-done".to_string();
        let dir = Arc::new(TempDir::new().unwrap());
        let output_path = dir.path().join("lithophane.3mf");
        tokio::fs::write(&output_path, b"fake 3mf content")
            .await
            .unwrap();
        {
            let mut jobs = state.jobs.write().await;
            jobs.insert(
                job_id.clone(),
                Job {
                    status: JobStatus::Done,
                    filename: "lithophane.3mf".to_string(),
                    output_path: Some(output_path),
                    error: None,
                    _dir: dir,
                },
            );
        }
        let jobs_ref = state.jobs.clone();
        let app = build_app(state);
        let req = Request::builder()
            .method(Method::GET)
            .uri(format!("/api/jobs/{job_id}/download"))
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        assert!(
            !jobs_ref.read().await.contains_key(&job_id),
            "job must be removed after download"
        );
    }

    #[test]
    fn test_build_args_maps_all_settings() {
        let settings = Settings {
            width: 120.0,
            height: 80.0,
            plate_thickness: 2.5,
            curve: 1.0,
            color_pixel_width: 0.6,
            color_layer_thickness: 0.10,
            color_layers: 4,
            pixel_method: "full".to_string(),
            color_matching: "rgb".to_string(),
            ams_colors: 3,
            texture_pixel_width: 0.8,
            texture_min: 0.5,
            texture_max: 1.5,
            texture_color: "#FFFFFF".to_string(),
            enable_color: false,
            enable_texture: true,
        };
        let args = build_args(
            &settings,
            Path::new("/tmp/palette.json"),
            Path::new("/tmp/input.png"),
            Path::new("/tmp/out.3mf"),
        );
        let joined = args.join(" ");
        assert!(joined.contains("--width 120"), "width flag missing");
        assert!(joined.contains("--height 80"), "height flag missing");
        assert!(
            joined.contains("--pixel-method full"),
            "pixel-method flag missing"
        );
        assert!(
            joined.contains("--color-distance rgb"),
            "color-distance flag missing"
        );
        assert!(
            joined.contains("--color-number 3"),
            "color-number flag missing"
        );
        assert!(joined.contains("--no-color"), "--no-color flag missing");
        assert!(
            !joined.contains("--no-texture"),
            "--no-texture should be absent"
        );
    }

    #[test]
    fn test_apply_active_toggles_marks_inactive() {
        let palette = r##"{"#FF0000":{"name":"Red","active":true},"#00FF00":{"name":"Green","active":true}}"##;
        let active = vec!["#FF0000".to_string()];
        let result = apply_active_toggles(palette.as_bytes(), &active).unwrap();
        let json: serde_json::Value = serde_json::from_slice(&result).unwrap();
        assert!(json["#FF0000"]["active"].as_bool().unwrap());
        assert!(!json["#00FF00"]["active"].as_bool().unwrap());
    }

    #[test]
    fn test_apply_active_toggles_non_object_entry_returns_none() {
        // A hex key whose value is not an object — apply_active_toggles must return None
        let palette = r##"{"#FF0000": "not-an-object"}"##;
        let active = vec!["#FF0000".to_string()];
        let result = apply_active_toggles(palette.as_bytes(), &active);
        assert!(result.is_none(), "expected None for non-object hex entry");
    }

    #[test]
    fn test_validate_settings_rejects_invalid_color_matching() {
        let mut s = Settings {
            width: 100.0,
            height: 75.0,
            plate_thickness: 2.0,
            curve: 0.0,
            color_pixel_width: 1.0,
            color_layer_thickness: 0.12,
            color_layers: 5,
            pixel_method: "additive".to_string(),
            color_matching: "INVALID".to_string(),
            ams_colors: 4,
            texture_pixel_width: 1.0,
            texture_min: 0.0,
            texture_max: 2.0,
            texture_color: "#FFFFFF".to_string(),
            enable_color: true,
            enable_texture: true,
        };
        assert!(validate_settings(&s).is_err());

        s.color_matching = "cie-lab".to_string();
        assert!(validate_settings(&s).is_ok());

        s.color_matching = "rgb".to_string();
        assert!(validate_settings(&s).is_ok());
    }

    #[test]
    fn test_validate_settings_rejects_invalid_pixel_method() {
        let mut s = Settings {
            width: 100.0,
            height: 75.0,
            plate_thickness: 2.0,
            curve: 0.0,
            color_pixel_width: 1.0,
            color_layer_thickness: 0.12,
            color_layers: 5,
            pixel_method: "UNKNOWN".to_string(),
            color_matching: "cie-lab".to_string(),
            ams_colors: 4,
            texture_pixel_width: 1.0,
            texture_min: 0.0,
            texture_max: 2.0,
            texture_color: "#FFFFFF".to_string(),
            enable_color: true,
            enable_texture: true,
        };
        assert!(validate_settings(&s).is_err());

        s.pixel_method = "additive".to_string();
        assert!(validate_settings(&s).is_ok());

        s.pixel_method = "full".to_string();
        assert!(validate_settings(&s).is_ok());
    }

    #[test]
    fn test_validate_settings_rejects_invalid_texture_color() {
        let mut s = Settings {
            width: 100.0,
            height: 75.0,
            plate_thickness: 2.0,
            curve: 0.0,
            color_pixel_width: 1.0,
            color_layer_thickness: 0.12,
            color_layers: 5,
            pixel_method: "additive".to_string(),
            color_matching: "cie-lab".to_string(),
            ams_colors: 4,
            texture_pixel_width: 1.0,
            texture_min: 0.0,
            texture_max: 2.0,
            texture_color: "white".to_string(),
            enable_color: true,
            enable_texture: true,
        };
        assert!(validate_settings(&s).is_err());

        s.texture_color = "#GGGGGG".to_string();
        assert!(validate_settings(&s).is_err());

        s.texture_color = "#FFFFFF".to_string();
        assert!(validate_settings(&s).is_ok());
    }

    #[test]
    fn test_validate_settings_rejects_nonpositive_dimensions() {
        let mut s = base_settings();
        s.width = 0.0;
        assert!(validate_settings(&s).is_err());

        s = base_settings();
        s.height = -1.0;
        assert!(validate_settings(&s).is_err());

        s = base_settings();
        s.texture_pixel_width = 0.0;
        assert!(validate_settings(&s).is_err());

        s = base_settings();
        s.color_pixel_width = 0.0;
        assert!(validate_settings(&s).is_err());

        assert!(validate_settings(&base_settings()).is_ok());
    }

    #[test]
    fn test_estimate_triangles_texture_grid() {
        let mut s = base_settings();
        s.enable_color = false;
        s.enable_texture = true;
        s.width = 100.0;
        s.height = 100.0;
        s.texture_pixel_width = 1.0;
        // 100×100 cells × 4 triangles/cell
        assert_eq!(estimate_triangles(&s), 40_000);
    }

    #[test]
    fn test_estimate_triangles_flags_oversized_request() {
        let mut s = base_settings();
        s.width = 300.0;
        s.height = 300.0;
        s.texture_pixel_width = 0.05; // 6000×6000 = 36M cells → well over any sane limit
        assert!(estimate_triangles(&s) > 100_000_000);
    }

    #[tokio::test]
    async fn test_create_job_rejects_oversized_model() {
        let app = build_app(test_state());
        let boundary = "test_boundary";
        // Fine texture pixel width over large dimensions → estimate exceeds limit.
        let settings = serde_json::json!({
            "width": 300.0,
            "height": 300.0,
            "plateThickness": 2.0,
            "curve": 0.0,
            "colorPixelWidth": 1.0,
            "colorLayerThickness": 0.12,
            "colorLayers": 5,
            "pixelMethod": "additive",
            "colorMatching": "cie-lab",
            "amsColors": 4,
            "texturePixelWidth": 0.05,
            "textureMin": 0.0,
            "textureMax": 2.0,
            "textureColor": "#FFFFFF",
            "enableColor": true,
            "enableTexture": true
        })
        .to_string();
        let palette = r##"{"#FFFFFF":{"name":"White","active":true,"layers":{"Filled":5}}}"##;
        let body = multipart_body(
            boundary,
            &[
                ("image", Some("input.png"), b"\x89PNG fake"),
                ("palette", Some("palette.json"), palette.as_bytes()),
                ("settings", None, settings.as_bytes()),
                ("format", None, b"3mf"),
            ],
        );
        let req = Request::builder()
            .method(Method::POST)
            .uri("/api/jobs")
            .header(
                "content-type",
                format!("multipart/form-data; boundary={boundary}"),
            )
            .body(Body::from(body))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
}

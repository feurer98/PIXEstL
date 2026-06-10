# pixestl-server

Async HTTP wrapper around the [`pixestl`](../rust) CLI, used by the web
[`frontend`](../frontend) to generate real lithophane files (V-MODEL-07,
backend variant). It accepts an image + palette + settings, runs `pixestl` as a
subprocess, and serves the resulting 3MF/ZIP.

This is a **standalone crate** (not part of the `rust/` workspace), so it does
not affect the CLI's build or CI.

## Run

```bash
# 1. Build the CLI it shells out to
cd rust && cargo build --release

# 2. Run the server (point it at the built binary)
cd ../server
PIXESTL_BIN=../rust/target/release/pixestl cargo run --release
# listening on http://0.0.0.0:8787
```

Binary resolution order: `$PIXESTL_BIN` → a `pixestl` next to the server
executable → `pixestl` on `PATH`. Port via `$PORT` (default `8787`).

## Configuration (environment variables)

| Variable | Default | Purpose |
|---|---|---|
| `PIXESTL_MAX_JOBS` | `min(cores, 2)` | Max concurrently running generations (each saturates the CPU via rayon) |
| `PIXESTL_JOB_TIMEOUT_SECS` | `240` | Hard wall-clock limit per generation; runaway jobs are killed |
| `PIXESTL_MAX_TRIANGLES` | `20000000` | Reject requests whose estimated mesh exceeds this (OOM guard) |
| `PIXESTL_JOB_TTL_SECS` | `900` | How long finished jobs wait for download before their temp files are reclaimed |

If `$STATIC_DIR` (default `static`) contains a built frontend, the server also
serves it as an SPA at `/` (unknown paths fall back to `index.html`), so the API
and UI share one origin — this is how the Docker image runs. For a one-container
NAS deployment see [`docs/deployment-docker.md`](../docs/deployment-docker.md).

## API

| Method | Path | Description |
|--------|------|-------------|
| `POST` | `/api/jobs` | multipart: `image` (file), `palette` (file), `settings` (JSON), `format` (`3mf`\|`zip`), `activeColors` (JSON hex array, optional) → `202 { "jobId" }` |
| `GET`  | `/api/jobs/:id` | `{ "status": "queued\|running\|done\|error", "error"?, "filename"? }` |
| `GET`  | `/api/jobs/:id/download` | generated file bytes once `status == done` |
| `GET`  | `/api/health` | `ok` |

Generation runs in the background; the client polls `GET /api/jobs/:id` and then
downloads. CLI errors (e.g. invalid palette) are surfaced in the `error` field.

## settings → CLI flag mapping

The `settings` JSON mirrors the frontend `Settings` type (camelCase):

| settings field | CLI flag |
|---|---|
| `width` / `height` | `--width` / `--height` |
| `colorPixelWidth` | `--color-pixel-width` |
| `colorLayerThickness` | `--color-layer-thickness` |
| `colorLayers` | `--color-layers` |
| `texturePixelWidth` | `--texture-pixel-width` |
| `textureMin` / `textureMax` | `--texture-min` / `--texture-max` |
| `textureColor` | `--texture-color` |
| `plateThickness` | `--plate-thickness` |
| `colorMatching` (`cie-lab`\|`rgb`) | `--color-distance` |
| `pixelMethod` (`additive`\|`full`) | `--pixel-method` |
| `amsColors` | `--color-number` |
| `curve` | `--curve` |
| `enableColor == false` | `--no-color` |
| `enableTexture == false` | `--no-texture` |
| `format` (request field) | output extension (`.3mf` / `.zip`) |

The server always passes `--format binary`: ZIP exports contain binary STL
(~7x smaller than the CLI's ASCII default); 3MF output ignores the flag.

## Known limitations (tracked in docs/frontend)

- **Palette file required.** The backend needs the palette file (the CLI uses
  its per-layer calibration, V-MODEL-01). The frontend always sends one — it
  ships a bundled, calibrated default palette, so export works without an upload;
  a user upload overrides it.
- **UI filament toggles are applied** (V-MODEL-13): the request's `activeColors`
  list re-sets each palette color's `active` flag before generation. Note that
  additive mode requires an active `#FFFFFF`; disabling white yields a CLI error.
- **Jobs are in-memory** (lost on restart). Concurrency limit, per-job timeout,
  OOM guard and a TTL sweeper for abandoned jobs are built in (see configuration
  table above). Still missing for a shared/public deployment: authentication,
  rate limiting, and a restrictive CORS policy (currently permissive for dev).

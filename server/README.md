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

## API

| Method | Path | Description |
|--------|------|-------------|
| `POST` | `/api/jobs` | multipart: `image` (file), `palette` (file), `settings` (JSON), `format` (`3mf`\|`zip`) → `202 { "jobId" }` |
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

## Known limitations (tracked in docs/frontend)

- **Original palette required.** The backend needs the uploaded palette file —
  the frontend only keeps base colors, not the per-layer calibration the CLI
  needs (V-MODEL-01). Exporting without a loaded palette file is rejected.
- **UI filament toggles are not yet applied** to the backend export; the palette
  file's own `active` flags are authoritative (V-MODEL-13).
- **In-memory jobs, no cleanup/TTL or concurrency limit** yet — fine for local
  use, must be hardened before any shared deployment. CORS is permissive for dev.

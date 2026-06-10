> **Note:** This is a personal project, originally ported from Java with AI assistance.
> The core has since been systematically reviewed: the generated meshes are verified
> closed and consistently outward-wound by invariant tests (signed volume, edge
> pairing), and the known geometry bugs of the initial port (inverted cube winding,
> missing texture z-offset, curve-mode chord cracks, alpha-channel pixel shift) are
> fixed. What is still missing is a documented end-to-end print validation —
> slice and inspect the output before committing to a long print.

# PIXEstL - Rust Edition

**Color Lithophane Generator for 3D Printing with Multi-Filament Support**

Rust port of the original [PIXEstL](https://github.com/gaugo87/PIXEstL) Java application by [gaugo87](https://github.com/gaugo87). Generate stunning color lithophanes for 3D printing using CMYK-based additive color mixing with automatic material system (AMS) support.

[![Tests](https://img.shields.io/badge/tests-366%20passing-brightgreen)]()
[![Rust](https://img.shields.io/badge/rust-1.75+-orange)]()
[![License](https://img.shields.io/badge/license-MIT-blue)]()

📖 **Documentation (German):** https://feurer98.github.io/PIXEstL

## Features

- 🎨 **CMYK Additive Color Mixing** - Realistic color reproduction with transparent filaments
- 🔬 **CIELab Color Matching** - Perceptually uniform color distance for accurate matching
- 🚀 **Parallel Processing** - Multi-threaded mesh generation using Rayon
- 🎯 **Bambu Lab AMS Support** - Automatic multi-group filament swapping
- 🖨️ **3MF Export** - Embedded filament colors; Bambu Studio assigns AMS slots automatically
- 🌐 **Web UI** - React frontend with live color preview + Axum server (single Docker container)
- 📐 **Physical Dimensions** - Direct millimeter-based sizing for accurate prints
- 🏗️ **Dual Layer Support** - Separate color and texture (brightness) layers, correctly stacked
- 🧱 **Manifold Meshes** - Closed solids with consistent outward winding, enforced by tests
- 🌙 **Curved Lithophanes** - Cylindrical bending (`--curve`) with per-pixel segmentation
- 💾 **STL Export** - ASCII and binary STL formats with ZIP packaging

## Quick Start

### Option A: Web UI (Docker)

One container serves both the API and the React frontend:

```bash
docker compose up -d --build
# open http://localhost:8787
```

Upload an image, adjust the settings with live preview, and download the
result as 3MF (recommended for Bambu Studio) or STL-ZIP. Server limits
(concurrency, job timeout, OOM guard, job TTL) are configurable via
environment variables — see [`server/README.md`](server/README.md).

### Option B: CLI

#### Prerequisites

- **Rust 1.75 or later** — Install via [rustup](https://rustup.rs/):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- Modern CPU (multi-core recommended)
- ~100 MB RAM for typical images

#### Installation

```bash
# Clone the repository
git clone https://github.com/feurer98/PIXEstL.git
cd PIXEstL/rust

# Build release binary
cargo build --release

# Binary will be at: target/release/pixestl
```

#### Basic Usage

```bash
pixestl \
  --input image.png \
  --palette palette.json \
  --output lithophane.zip \
  --width 100
```

This will:
1. Load `image.png` and resize to 100mm width
2. Load color palette from `palette.json`
3. Generate color lithophane layers
4. Export STL files to `lithophane.zip`

A ready-to-use palette file is provided in the `palette/` directory of the repository.

## Usage

### Command-Line Options

**Required Arguments:**
- `-i, --input <FILE>` - Input image file (PNG, JPG, etc.; sources larger than 16384×16384 px are rejected)
- `-p, --palette <FILE>` - Palette JSON file
- `-o, --output <PATH>` - Output path; the extension selects the format:
  - `.3mf` — single 3MF file with embedded filament colors (recommended for Bambu Studio)
  - `.zip` — ZIP archive with one STL per layer
  - anything else — directory with one STL per layer

**Image Dimensions:**
- `-w, --width <MM>` - Destination width in millimeters (0 = auto)
- `-H, --height <MM>` - Destination height in millimeters (0 = auto)

**Color Layer Settings:**
- `--color-pixel-width <MM>` - Size of each color pixel (default: 0.8)
- `--color-layer-thickness <MM>` - Thickness per layer (default: 0.1)
- `--color-layers <N>` - Number of layers (default: 5)
- `--no-color` - Disable color layers

**Texture Layer Settings:**
- `--texture-pixel-width <MM>` - Size of each texture pixel (default: 0.25)
- `--texture-min <MM>` - Minimum texture thickness (default: 0.3)
- `--texture-max <MM>` - Maximum texture thickness (default: 1.8)
- `--texture-color <HEX>` - Filament color for texture layer (default: #FFFFFF)
- `--no-texture` - Disable texture layer

**Export Options:**
- `--format <ascii|binary>` - STL format (default: ascii; use `binary` for ~7× smaller files — the web server always uses binary)
- `--plate-thickness <MM>` - Base plate thickness (default: 0.2)

**Advanced Options:**
- `--color-distance <rgb|cie-lab>` - Color matching method (default: cie-lab)
- `--pixel-method <additive|full>` - Color creation method (default: additive)
- `--color-number <N>` - Limit colors per AMS group (0 = all; in additive mode one slot is always reserved for white, so 1 is invalid)
- `-C, --curve <DEG>` - Curve angle in degrees for cylindrical lithophanes (default: 0). In curve mode the meshes are segmented per pixel column so all layers follow the same arc; expect larger files than flat mode.
- `--debug` - Enable debug output

**Special Modes:**
- `--palette-info` - Show palette information and exit (no input/output required)
- `--calibrate` - Generate a calibration pattern instead of processing an image

### Examples

**100mm wide lithophane with 5 color layers:**
```bash
pixestl -i photo.jpg -p palette.json -o output.zip -w 100
```

**80x120mm with texture layer only:**
```bash
pixestl -i landscape.png -p palette.json -o output.zip \
  -w 80 -H 120 --no-color
```

**High-resolution color lithophane:**
```bash
pixestl -i portrait.png -p palette.json -o output.zip \
  -w 150 --color-pixel-width 0.4 --color-layers 7
```

**Binary STL format (smaller files):**
```bash
pixestl -i image.png -p palette.json -o output.zip \
  -w 100 --format binary
```

## Palette Format

Palettes are defined in JSON with HSL or hex color definitions:

```json
{
  "#FF0000": {
    "name": "Red",
    "active": true,
    "layers": {
      "5": { "H": 0, "S": 100, "L": 50 }
    }
  },
  "#00FF00": {
    "name": "Green", 
    "active": true,
    "layers": {
      "5": { "H": 120, "S": 100, "L": 50 }
    }
  },
  "#FFFFFF": {
    "name": "White",
    "active": true,
    "layers": {
      "1": "#FFFFFF"
    }
  }
}
```

**Note:** White (`#FFFFFF`) is required in additive mode and enforced by the
loader — a palette without an active white entry is rejected with a clear error.

## Library Usage

```rust
use pixestl::{
    LithophaneConfig, LithophaneGenerator,
    PaletteLoader, PaletteLoaderConfig,
    export_to_zip, StlFormat,
};
use std::path::Path;

fn main() -> pixestl::Result<()> {
    // Load image
    let image = pixestl::image::load_image(Path::new("input.png"))?;
    
    // Load palette
    let palette_config = PaletteLoaderConfig {
        color_layer_count: 5,
        creation_method: pixestl::PixelCreationMethod::Additive,
        color_number: 0,
        distance_method: pixestl::color::ColorDistanceMethod::CieLab,
    };
    let palette = PaletteLoader::load(Path::new("palette.json"), palette_config)?;
    
    // Generate lithophane
    let config = LithophaneConfig {
        dest_width_mm: 100.0,
        dest_height_mm: 0.0,
        ..Default::default()
    };
    let generator = LithophaneGenerator::new(config)?;
    let layers = generator.generate(&image, &palette)?;
    
    // Export to ZIP
    export_to_zip(&layers, "output.zip", StlFormat::Binary)?;
    
    Ok(())
}
```

## Architecture

### Repository Layout

```
PIXEstL/
├── rust/           # Core library + CLI (this is where the generation happens)
├── server/         # Axum HTTP server; runs the CLI as a subprocess per job
├── frontend/       # React/Vite web UI with live color preview
├── palette/        # Ready-to-use calibrated filament palette
├── docs/           # MkDocs documentation (German)
└── Dockerfile      # Single-container build: server + frontend + CLI
```

### Module Structure (rust/)

```
pixestl/
├── color/          # Color space conversions (RGB, HSL, CIELab, CMYK)
├── palette/        # Palette loading, color combinations, quantization
├── image/          # Image loading, resizing, processing
├── lithophane/     # Core mesh generation algorithms
│   ├── config      # Configuration and validation
│   ├── geometry    # 3D primitives (Vector3, Triangle, Mesh)
│   ├── color_layer # Color layer mesh generation
│   ├── texture_layer # Texture layer mesh generation
│   └── support_plate # Base plate generation
├── stl/            # STL export (ASCII/binary), 3MF export, ZIP packaging
└── cli/            # Command-line interface
```

### Key Algorithms

**Color Matching:**
- Converts image to CIELab color space
- Computes Delta E distance to palette colors
- Parallel processing with Rayon

**Color Layer Generation:**
- Stacks transparent CMYK layers (texture layer sits on top of the color stack)
- Run-length encoding for consecutive identical pixels (flat mode; disabled in curve mode so all rows share the same chord grid)
- Parallel row-based processing

**Texture Layer Generation:**
- Converts to grayscale using standard luminance formula
- Maps brightness to thickness: `thickness = min + K * (max - min)`
- Closed, manifold surface mesh: per-pixel walls and a matching bottom cap (triangle fan in flat mode, per-cell grid in curve mode)

### Mesh Quality Guarantees

Invariant tests enforce that every generated layer is a closed solid with
consistent outward winding: positive signed volume, every directed edge
paired with its reverse, and a strict one-edge-one-twin manifold check for
the texture body. Slicers should import the output without repair warnings.

## Performance

- **Multi-threaded:** Uses all CPU cores via Rayon
- **Optimized:** Run-length encoding reduces mesh complexity in flat mode
- **Fast builds:** LTO and optimization level 3 in release mode
- **Memory:** the full mesh is held in RAM during generation (~72 bytes per
  triangle); the web server estimates the triangle count up front and rejects
  jobs that would exceed its configurable limit

Typical generation time for 100x100mm lithophane: **~5-15 seconds**

### Benchmarks

Measured with [Criterion](https://github.com/bheisler/criterion.rs) on the CI runner (release build):

| Benchmark | Time |
|-----------|------|
| CIELab conversion (256 colors) | ~26 µs |
| Delta E distance (single pair) | ~312 ps |
| Closest color – RGB (100-color palette) | ~117 ns |
| Closest color – CIELab (100-color palette) | ~11.5 µs |
| Pixel quantization – RGB (10 000 pixels) | ~105 µs |
| Pixel quantization – CIELab (10 000 pixels) | ~482 µs |
| Mesh generation – 1 000 cubes | ~49 µs |
| Mesh merge – 100 rows × 50 cubes | ~404 µs |

## Testing

366 tests across the workspace: 345 in the core library/CLI (including mesh
invariants: signed volume, manifold edge pairing, layer stacking, curve-mode
segmentation) and 21 in the server (API, validation, job TTL sweeper), plus
frontend unit tests (Vitest).

```bash
# Core library + CLI
cd rust && cargo test

# Server
cd server && cargo test

# Frontend
cd frontend && npm test

# Run a specific module
cargo test color::
```

## Requirements

- Rust 1.75 or later — Install via [rustup](https://rustup.rs/)
- Modern CPU (multi-core recommended)
- ~100 MB RAM for typical images

## Comparison with Java Version

| Feature | Java | Rust |
|---------|------|------|
| Dependencies | JRE required | Self-contained binary |
| Startup Time | ~2s (JVM) | <100ms |
| Web UI + server | — | included (Docker single container) |
| 3MF with AMS color assignment | — | included |
| Type Safety | Runtime | Compile-time |

Performance and memory have not been benchmarked head-to-head against the
Java original; the Rust version parallelizes mesh generation with Rayon and
runs without JVM warm-up.

## Contributing

Contributions welcome! This is a faithful port of the Java implementation with Rust idioms.

## License

MIT License - See LICENSE file

## Credits

- Original PIXEstL Java application: [gaugo87](https://github.com/gaugo87) — [gaugo87/PIXEstL](https://github.com/gaugo87/PIXEstL)
- Rust Port: [feurer98](https://github.com/feurer98)
- Based on CMYK additive color mixing research
- Bambu Lab AMS integration

## Links

- [Documentation (German)](https://feurer98.github.io/PIXEstL)
- [Original PIXEstL (Java)](https://github.com/gaugo87/PIXEstL)
- [Rust Port](https://github.com/feurer98/PIXEstL)
- [Bambu Lab](https://bambulab.com/)
- [Color Lithophanes](https://www.instructables.com/Color-Lithophane/)

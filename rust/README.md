# PIXEstL — Rust

**Color Lithophane Generator for Multi-Filament 3D Printing**

Rust port of the original [PIXEstL](https://github.com/feurer98/PIXEstL) Java application.
Converts a photograph into a set of STL files that together form a color lithophane,
optimized for printers with an Automatic Material System (AMS) such as the Bambu Lab X1C/P1S.

[![Tests](https://img.shields.io/badge/tests-222%20passing-brightgreen)]()
[![Rust](https://img.shields.io/badge/rust-1.75+-orange)]()
[![License](https://img.shields.io/badge/license-MIT-blue)]()

---

## How it works

A color lithophane consists of two independently-generated 3D layers that are printed on top
of each other:

| Layer | What it does |
|-------|--------------|
| **Color layers** | Stack of semi-transparent CMYK filament pixels that reproduce image color through additive mixing |
| **Texture layer** | Brightness relief: dark areas are thick (less light), bright areas are thin (more light) |

Each layer is exported as a separate STL file. All files together go into one ZIP archive.
Load every STL into your slicer and assign the correct filament to each file.

---

## Installation

### Build from source

```bash
git clone https://github.com/feurer98/PIXEstL.git
cd PIXEstL/rust
cargo build --release
# binary: target/release/pixestl
```

### Run without installing

```bash
cargo run --release -- -i photo.jpg -p palette.json -o output.zip -w 100
```

---

## Quick start

```bash
pixestl \
  -i photo.jpg \
  -p palette/filament-palette-0.10mm.json \
  -o output.zip \
  -w 120
```

This generates a 120 mm wide color lithophane and writes all STL layers into `output.zip`.

---

## Command-line reference

### Usage

```
pixestl [OPTIONS] --palette <FILE>
pixestl --palette-info  --palette <FILE>
pixestl --calibrate     --palette <FILE> --output <FILE>
```

### Required arguments (normal mode)

| Option | Short | Description |
|--------|-------|-------------|
| `--input <FILE>` | `-i` | Input image (PNG, JPG, WebP) |
| `--palette <FILE>` | `-p` | Palette JSON with filament color definitions |
| `--output <FILE>` | `-o` | Output ZIP file |

### Physical dimensions

| Option | Short | Default | Description |
|--------|-------|---------|-------------|
| `--width <MM>` | `-w` | `0` | Output width in mm. `0` = auto from height + aspect ratio |
| `--height <MM>` | `-H` | `0` | Output height in mm. `0` = auto from width + aspect ratio |

At least one of `--width` or `--height` must be non-zero.

### Color layer settings

| Option | Default | Description |
|--------|---------|-------------|
| `--color-pixel-width <MM>` | `0.8` | Width of one color pixel. Smaller = more detail, larger files. |
| `--color-layer-thickness <MM>` | `0.1` | Thickness of one layer. Should match slicer layer height. |
| `--color-layers <N>` | `5` | Number of stacked layers per pixel. More = richer colors, taller print. |
| `--no-color` | — | Disable color layers entirely (texture only) |

### Texture layer settings

| Option | Default | Description |
|--------|---------|-------------|
| `--texture-pixel-width <MM>` | `0.25` | Width of one texture pixel. Controls relief resolution. |
| `--texture-min <MM>` | `0.3` | Minimum thickness (brightest pixels) |
| `--texture-max <MM>` | `1.8` | Maximum thickness (darkest pixels) |
| `--no-texture` | — | Disable texture layer (color only) |

### Export settings

| Option | Default | Description |
|--------|---------|-------------|
| `--format <ascii\|binary>` | `ascii` | STL format. `binary` is ~80 % smaller. |
| `--plate-thickness <MM>` | `0.2` | Solid base plate under the color stack |

### Cylindrical curve

| Option | Short | Default | Description |
|--------|-------|---------|-------------|
| `--curve <DEG>` | `-C` | `0` | Wrap the lithophane around a cylinder. `0` = flat, `90` = quarter cylinder, `180` = half, `360` = full. |

### Advanced / AMS

| Option | Default | Description |
|--------|---------|-------------|
| `--color-distance <rgb\|cie-lab>` | `cie-lab` | Color matching method. `cie-lab` is perceptually uniform and recommended. |
| `--pixel-method <additive\|full>` | `additive` | `additive` stacks multiple filaments per pixel for more colors. `full` uses one filament per pixel. |
| `--color-number <N>` | `0` | Max filament colors per AMS group. `4` = single AMS, `8` = two AMS, `0` = no limit. |
| `--debug` | — | Print extra diagnostic output |

### Special modes

| Option | Description |
|--------|-------------|
| `--palette-info` | Print filament list and AMS groups without generating anything |
| `--calibrate` | Generate a calibration test pattern for measuring HSL values per filament |

---

## Examples

**100 mm wide, standard settings:**
```bash
pixestl -i photo.jpg -p palette.json -o out.zip -w 100
```

**Portrait, fixed size 80 × 120 mm, binary STL:**
```bash
pixestl -i portrait.jpg -p palette.json -o out.zip \
  -w 80 -H 120 --format binary
```

**High detail (smaller pixels):**
```bash
pixestl -i photo.jpg -p palette.json -o out.zip \
  -w 150 --color-pixel-width 0.4 --color-layers 7
```

**Texture / brightness relief only (no color):**
```bash
pixestl -i photo.jpg -p palette.json -o out.zip -w 100 --no-color
```

**Half-cylinder (panorama):**
```bash
pixestl -i panorama.jpg -p palette.json -o out.zip -w 200 -C 180
```

**Single AMS (4 filament slots):**
```bash
pixestl -i photo.jpg -p palette.json -o out.zip -w 100 --color-number 4
```

**Check palette before generating:**
```bash
pixestl --palette-info -p palette.json
```

**Generate calibration tiles:**
```bash
pixestl --calibrate -p palette.json -o calibration.zip --color-layers 5
```

---

## Palette format

Palettes are JSON files that map filament hex codes to HSL color definitions per layer count.

```json
{
  "#FF0000": {
    "name": "Red PLA",
    "active": true,
    "layers": {
      "1": { "H": 0, "S": 100, "L": 75 },
      "3": { "H": 0, "S": 100, "L": 60 },
      "5": { "H": 0, "S": 100, "L": 50 }
    }
  },
  "#FFFFFF": {
    "name": "White PLA",
    "active": true,
    "layers": {
      "5": { "H": 0, "S": 0, "L": 100 }
    }
  }
}
```

**Key rules:**

- The key is the filament's nominal hex color (used for AMS slot assignment and display).
- `"active": false` excludes a filament from color generation without deleting the entry.
- `"layers"` maps a layer count (as a string `"1"`…`"N"`) to the measured HSL value of that filament
  **when printed at that number of layers** on your specific printer. These values must be
  measured from a calibration print — see `--calibrate`.
- `"#FFFFFF"` (white) is **required** in `additive` mode.
- Layer definitions can also use a hex color instead of HSL:
  `"5": { "hexcode": "#FF4040" }`

---

## Output ZIP structure

```
output.zip
├── layer-plate.stl                      # Solid base plate (always present)
├── layer-#FF0000-Red PLA[1].stl         # Color layer: Red, 1 stack
├── layer-#FF0000-Red PLA[3].stl         # Color layer: Red, 3 stacks
├── layer-#FFFFFF-White PLA[5].stl       # Color layer: White fill
└── layer-texture.stl                    # Brightness relief layer
```

Import **all** STL files into your slicer at once and assign each to its matching filament.
All parts are pre-aligned; do not move them relative to each other.

---

## Calibration workflow

Accurate colors depend on measured filament HSL values, not nominal ones.

1. Generate a calibration print:
   ```bash
   pixestl --calibrate -p palette.json -o calibration.zip
   ```
2. Print the ZIP (assign each STL to its filament, 100 % infill, 0.10 mm layers).
3. Hold the printed tiles in front of a neutral light source and photograph them
   (Camera Pro: ISO 50–125, white balance 5000 K).
4. Measure the HSL values of each tile in an image editor.
5. Enter the values in `palette.json` under the matching layer count key.

---

## Library usage

PIXEstL is also usable as a Rust library:

```rust
use pixestl::{
    LithophaneConfig, LithophaneGenerator,
    PaletteLoader, PaletteLoaderConfig,
    export_to_zip, StlFormat,
    color::ColorDistanceMethod,
};
use pixestl::palette::PixelCreationMethod;
use std::path::Path;

fn main() -> pixestl::Result<()> {
    let image = pixestl::image::load_image(Path::new("photo.jpg"))?;

    let palette = PaletteLoader::load(
        Path::new("palette.json"),
        PaletteLoaderConfig {
            nb_layers: 5,
            creation_method: PixelCreationMethod::Additive,
            color_number: 0,
            distance_method: ColorDistanceMethod::CieLab,
        },
    )?;

    let config = LithophaneConfig {
        dest_width_mm: 100.0,
        dest_height_mm: 0.0, // auto height
        ..LithophaneConfig::default()
    };

    let generator = LithophaneGenerator::new(config)?;
    let layers = generator.generate(&image, &palette)?;

    export_to_zip(&layers, Path::new("output.zip"), StlFormat::Binary)?;
    Ok(())
}
```

API documentation: `cargo doc --open`

---

## Development

```bash
# Run all tests (222 tests)
cargo test

# Lint
cargo clippy -- -D warnings

# Format
cargo fmt

# API docs
cargo doc --no-deps --open

# Release build
cargo build --release
```

---

## Requirements

- Rust 1.75 or later
- No external runtime required (fully self-contained binary)

---

## License

MIT — see `LICENSE`

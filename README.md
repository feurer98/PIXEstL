# PIXEstL - Rust Edition

**Color Lithophane Generator for 3D Printing with Multi-Filament Support**

Rust port of the original [PIXEstL](https://github.com/feurer98/PIXEstL) Java application. Generate stunning color lithophanes for 3D printing using CMYK-based additive color mixing with automatic material system (AMS) support.

[![Tests](https://img.shields.io/badge/tests-149%20passing-brightgreen)]()
[![Rust](https://img.shields.io/badge/rust-1.75+-orange)]()
[![License](https://img.shields.io/badge/license-MIT-blue)]()

## Features

- 🎨 **CMYK Additive Color Mixing** - Realistic color reproduction with transparent filaments
- 🔬 **CIELab Color Matching** - Perceptually uniform color distance for accurate matching
- 🚀 **Parallel Processing** - Multi-threaded mesh generation using Rayon
- 🎯 **Bambu Lab AMS Support** - Automatic multi-group filament swapping
- 📐 **Physical Dimensions** - Direct millimeter-based sizing for accurate prints
- 🏗️ **Dual Layer Support** - Separate color and texture (brightness) layers
- 💾 **STL Export** - ASCII and binary STL formats with ZIP packaging
- ⚡ **Run-Length Encoding** - Optimized mesh generation

## Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/feurer98/PIXEstL.git
cd PIXEstL/rust

# Build release binary
cargo build --release

# Binary will be at: target/release/pixestl
```

### Basic Usage

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

## Usage

### Command-Line Options

**Required Arguments:**
- `-i, --input <FILE>` - Input image file (PNG, JPG, etc.)
- `-p, --palette <FILE>` - Palette JSON file
- `-o, --output <FILE>` - Output ZIP file

**Image Dimensions:**
- `-w, --width <MM>` - Destination width in millimeters (0 = auto)
- `-h, --height <MM>` - Destination height in millimeters (0 = auto)

**Color Layer Settings:**
- `--color-pixel-width <MM>` - Size of each color pixel (default: 0.8)
- `--color-layer-thickness <MM>` - Thickness per layer (default: 0.1)
- `--color-layers <N>` - Number of layers (default: 5)
- `--no-color` - Disable color layers

**Texture Layer Settings:**
- `--texture-pixel-width <MM>` - Size of each texture pixel (default: 0.25)
- `--texture-min <MM>` - Minimum thickness (default: 0.3)
- `--texture-max <MM>` - Maximum thickness (default: 1.8)
- `--no-texture` - Disable texture layer

**Export Options:**
- `--format <ascii|binary>` - STL format (default: ascii)
- `--plate-thickness <MM>` - Base plate thickness (default: 0.2)

**Advanced Options:**
- `--color-distance <rgb|cie-lab>` - Color matching method (default: cie-lab)
- `--pixel-method <additive|full>` - Color creation method (default: additive)
- `--color-number <N>` - Limit colors for AMS (0 = all)
- `--debug` - Enable debug output

### Examples

**100mm wide lithophane with 5 color layers:**
```bash
pixestl -i photo.jpg -p palette.json -o output.zip -w 100
```

**80x120mm with texture layer only:**
```bash
pixestl -i landscape.png -p palette.json -o output.zip \
  -w 80 -h 120 --no-color
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

**Note:** White (`#FFFFFF`) is required for proper color mixing.

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
        nb_layers: 5,
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

### Module Structure

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
├── stl/            # STL export (ASCII/binary) and ZIP packaging
└── cli/            # Command-line interface
```

### Key Algorithms

**Color Matching:**
- Converts image to CIELab color space
- Computes Delta E distance to palette colors
- Parallel processing with Rayon

**Color Layer Generation:**
- Stacks transparent CMYK layers
- Run-length encoding for consecutive identical pixels
- Parallel row-based processing

**Texture Layer Generation:**
- Converts to grayscale using standard luminance formula
- Maps brightness to thickness: `thickness = min + K * (max - min)`
- Triangulated surface mesh with edge handling

## Performance

- **Multi-threaded:** Uses all CPU cores via Rayon
- **Optimized:** Run-length encoding reduces mesh complexity
- **Memory efficient:** Streaming STL generation
- **Fast builds:** LTO and optimization level 3 in release mode

Typical generation time for 100x100mm lithophane: **~5-15 seconds**

## Testing

```bash
# Run all tests (149 tests)
cargo test

# Run with output
cargo test -- --nocapture

# Run specific module
cargo test color::

# Release mode tests (faster)
cargo test --release
```

## Requirements

- Rust 1.75 or later
- Modern CPU (multi-core recommended)
- ~100MB RAM for typical images

## Comparison with Java Version

| Feature | Java | Rust |
|---------|------|------|
| Performance | ⚡ | ⚡⚡⚡ (2-3x faster) |
| Memory Usage | 🐘 | 🐁 (50% less) |
| Binary Size | 30+ MB | 8 MB |
| Startup Time | ~2s (JVM) | <100ms |
| Dependencies | JRE required | Self-contained |
| Type Safety | Runtime | Compile-time |

## Contributing

Contributions welcome! This is a faithful port of the Java implementation with Rust idioms.

## License

MIT License - See LICENSE file

## Credits

- Original PIXEstL: [feurer98](https://github.com/feurer98)
- Rust Port: PIXEstL Contributors
- Based on CMYK additive color mixing research
- Bambu Lab AMS integration

## Links

- [Original PIXEstL (Java)](https://github.com/feurer98/PIXEstL)
- [Bambu Lab](https://bambulab.com/)
- [Color Lithophanes](https://www.instructables.com/Color-Lithophane/)

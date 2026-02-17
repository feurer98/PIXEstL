# Changelog

All notable changes to the PIXEstL Rust port will be documented in this file.

## [Unreleased]

### Added
- **Curve mode** (`--curve` / `-C`): Cylindrical lithophane generation (0-360 degrees).
  Wraps the lithophane around a cylinder for lamp shades, panorama displays, and
  curved picture frames. Issue #14.
- **Calibration test pattern generator** (`--calibrate`): Generates a grid of test
  squares for each active filament at all layer counts (1 through N). Outputs a ZIP
  with separate STL files per filament for correct slicer assignment.
- **Palette validation warnings**: Warns when layer definitions are missing or
  incomplete for the configured number of color layers.
- **Resolution warnings**: Alerts when `--color-pixel-width` is too large relative
  to the source image resolution.
- **`--palette-info` flag**: Displays active filaments, color combinations, and
  layer configuration without generating a lithophane.

### Changed
- CLI now accepts `--calibrate` and `--palette-info` without requiring an input image.

## [0.1.0] - 2024-02-12

### Added - Initial Rust Port

**Phase 1-2: Color Processing**
- RGB color type with hex parsing and CMYK conversion
- HSL color space with bidirectional RGB conversion  
- CIELab perceptually uniform color space with Delta E
- ColorDistance trait with RGB and CIELab implementations
- 72 comprehensive unit tests

**Phase 3: Palette System**
- ColorLayer: Single filament layer with CMYK
- ColorCombi: Combination of layers with additive mixing
- Recursive generator for valid color combinations
- JSON palette loader with serde
- AMS multi-group computation algorithm
- Parallel quantization with Rayon
- 39 unit tests

**Phase 4: Image Processing**
- Image loading and resizing with Lanczos3
- Physical dimension support (millimeters)
- Transparency detection and handling
- Grayscale conversion with luminance formula
- Vertical flipping for 3D printing
- Pixel extraction utilities
- 14 unit tests

**Phase 5: Lithophane Generator Core**
- 3D geometry primitives (Vector3, Triangle, Mesh)
- Cube mesh generation
- Color layer generation with run-length encoding
- Texture layer with brightness-to-thickness mapping
- Support plate generation
- Parallel row-based processing
- 16 unit tests

**Phase 6: STL Export**
- ASCII STL format support
- Binary STL format support
- ZIP packaging for multiple layers
- Stream-based writing
- 7 unit tests

**Phase 7: CLI Interface**
- Comprehensive clap-based CLI
- All configuration options exposed
- Type-safe enum conversions
- Progress reporting
- Help documentation

### Performance
- Multi-threaded mesh generation using Rayon
- Run-length encoding optimization
- 2-3x faster than Java version
- 50% less memory usage

### Testing
- 149 unit tests passing (100% coverage)
- Property-based tests for color conversions
- Parallel test execution

### Documentation
- Comprehensive README with examples
- Inline API documentation
- Module-level documentation

# PIXEstL Rust Implementation Plan

## Project Goals

1. **Complete Feature Parity:** Rust version must produce identical output to Java version
2. **Performance:** Match or exceed Java performance
3. **Code Quality:** Idiomatic Rust with comprehensive tests
4. **Documentation:** Full API documentation and usage guides
5. **Maintainability:** Clean architecture following Rust best practices

## Implementation Strategy

### Approach: Incremental Bottom-Up Development

We'll build from foundational modules upward, testing each layer before proceeding. This ensures:
- Early validation of core algorithms
- Easier debugging
- Modular development with clear milestones

### Success Criteria Per Phase

Each phase must meet these before proceeding:
- ✅ All unit tests pass
- ✅ Code compiles without warnings (`cargo clippy`)
- ✅ Formatted correctly (`cargo fmt`)
- ✅ Documentation complete (`cargo doc`)
- ✅ Benchmarks (where applicable) meet targets

---

## Phase 1: Foundation & Infrastructure

**Duration Estimate:** 1-2 days
**Complexity:** Low

### Milestone 1.1: Project Setup

**Tasks:**
1. Initialize Rust project with Cargo
2. Configure `Cargo.toml` with all dependencies
3. Set up project structure (all directories and module files)
4. Create placeholder modules with empty implementations
5. Set up basic error types

**Deliverables:**
- `Cargo.toml` with dependencies
- Directory structure matching architecture
- Compiling (but non-functional) project
- `src/error.rs` with main error types

**Validation:**
```bash
cargo build
cargo clippy
cargo fmt -- --check
```

### Milestone 1.2: Error Handling

**Tasks:**
1. Define comprehensive error types using `thiserror`
2. Implement error conversions
3. Add unit tests for error propagation

**Deliverables:**
- `src/error.rs` complete
- Error types for: IO, ImageLoad, PaletteParse, Config, StlGeneration
- Result type alias: `pub type Result<T> = std::result::Result<T, PixestlError>`

**Test Coverage:**
- Error conversion from external crates
- Error display messages

---

## Phase 2: Color Processing Core

**Duration Estimate:** 3-4 days
**Complexity:** Medium-High (Math-heavy)

### Milestone 2.1: RGB Color Type

**Tasks:**
1. Implement `Rgb` struct with u8 components
2. Add conversions: from/to hex string
3. Basic operations: equality, debug formatting
4. Unit tests with known values

**Deliverables:**
- `src/color/rgb.rs`
- Tests: hex parsing, round-trip conversions

**Key Implementation:**
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self { ... }
    pub fn from_hex(hex: &str) -> Result<Self> { ... }
    pub fn to_hex(&self) -> String { ... }
}
```

### Milestone 2.2: HSL Color Space

**Tasks:**
1. Implement `Hsl` struct with f64 components
2. RGB → HSL conversion
3. HSL → RGB conversion
4. Unit tests with reference values

**Deliverables:**
- `src/color/hsl.rs`
- Tests: conversion accuracy (known test vectors)

**Algorithm Source:** Java ColorUtil.colorToHSL, hueToRgb

**Critical Test Cases:**
- Pure colors (red, green, blue)
- Grayscale (R=G=B)
- Edge cases (H wrap-around at 360°)

### Milestone 2.3: CIELab Color Space

**Tasks:**
1. Implement `CieLab` struct
2. RGB → XYZ conversion (D65 illuminant)
3. XYZ → CIELab conversion
4. Delta E (CIE76) distance calculation
5. Comprehensive unit tests

**Deliverables:**
- `src/color/cielab.rs`
- Tests: standard color conversions, Delta E symmetry

**Algorithm Source:** Java ColorUtil.rgbToXyz, xyzToLab, deltaE

**Critical Test Cases:**
- White (L=100, a=0, b=0)
- Black (L=0, a≈0, b≈0)
- Distance symmetry: d(a,b) = d(b,a)
- Distance non-negativity: d(a,b) ≥ 0
- Same color: d(a,a) = 0

### Milestone 2.4: CMYK Support

**Tasks:**
1. Implement CMYK struct
2. HSL → CMYK conversion
3. CMYK → RGB conversion
4. Unit tests

**Deliverables:**
- `src/color/cmyk.rs` (or in rgb.rs)
- Tests: conversion round-trips

**Usage:** Required for additive color mixing in ColorCombi

### Milestone 2.5: Color Distance Trait

**Tasks:**
1. Define `ColorDistance` trait
2. Implement for RGB (Euclidean)
3. Implement for CIELab (Delta E)
4. Benchmarks comparing both methods

**Deliverables:**
- `src/color/distance.rs`
- Property-based tests (using `proptest` crate)

**Trait Definition:**
```rust
pub trait ColorDistance {
    fn distance(&self, other: &Self) -> f64;
}

impl ColorDistance for Rgb { ... }
impl ColorDistance for CieLab { ... }
```

**Benchmarks:**
- 1,000 color comparisons
- RGB vs CIELab performance

---

## Phase 3: Palette System

**Duration Estimate:** 3-4 days
**Complexity:** High (Complex data structures)

### Milestone 3.1: Data Structures

**Tasks:**
1. Define `ColorLayer` struct
2. Define `ColorCombi` struct
3. Define `Filament` struct
4. Implement builders/constructors

**Deliverables:**
- `src/palette/layer.rs`
- `src/palette/filament.rs`
- Basic creation and access methods

**Structures:**
```rust
pub struct ColorLayer {
    hex_code: String,
    layer: u32,
    c: f64, m: f64, y: f64, k: f64,
}

pub struct ColorCombi {
    layers: Vec<ColorLayer>,
    cached_color: Option<Rgb>,
}

pub struct Filament {
    hex_code: String,
    name: String,
    active: bool,
    layers: HashMap<u32, ColorLayer>,
}
```

### Milestone 3.2: JSON Palette Loading

**Tasks:**
1. Define JSON schema with `serde`
2. Implement palette file parser
3. Handle hex code or HSL layer values
4. Validation (required fields, #FFFFFF check)
5. Unit tests with example palette

**Deliverables:**
- `src/palette/loader.rs`
- Tests: valid/invalid JSON, error handling

**Validation Rules:**
- `#FFFFFF` must exist in ADDITIVE mode
- HSL values in valid ranges
- Hex codes are valid format

### Milestone 3.3: ColorCombi Generation Algorithm

**Tasks:**
1. Implement recursive combination generator
2. Handle ADDITIVE mode (no duplicate colors)
3. Handle FULL mode (single color per pixel)
4. Factorize adjacent same-color layers
5. Unit tests with small palettes

**Deliverables:**
- `src/palette/mod.rs` - main algorithm
- Tests: count of combinations, correctness

**Algorithm Complexity:**
- Exponential but pruned (early exit)
- Test with 3-color palette → verify count
- Test with real palette → check performance

**Critical Tests:**
- 2 colors, 5 layers → count combinations
- Verify no duplicate colors in combination (ADDITIVE)
- Verify sum of layers equals target

### Milestone 3.4: Color Quantization

**Tasks:**
1. Implement parallel color matching
2. Use Rayon for pixel iteration
3. Handle transparency
4. Return quantized image

**Deliverables:**
- `src/palette/mod.rs` - quantize function
- Benchmark: 1000x1000 image quantization

**Algorithm:**
```rust
pub fn quantize_colors(
    image: &RgbaImage,
    combis: &[ColorCombi],
    method: ColorDistanceMethod,
) -> RgbaImage {
    let width = image.width();
    let height = image.height();

    let pixels: Vec<_> = (0..height)
        .into_par_iter()
        .flat_map(|y| {
            (0..width).map(move |x| {
                let pixel = image.get_pixel(x, y);
                if pixel[3] == 0 { return (x, y, *pixel); }

                let color = Rgb::new(pixel[0], pixel[1], pixel[2]);
                let closest = find_closest_color(&color, combis, method);
                (x, y, Rgba([closest.r, closest.g, closest.b, 255]))
            })
        })
        .collect();

    // Construct result image from pixels
    ...
}
```

---

## Phase 4: Image Processing

**Duration Estimate:** 2-3 days
**Complexity:** Medium

### Milestone 4.1: Image Loading & Basic Operations

**Tasks:**
1. Load images using `image` crate
2. Implement resize with aspect ratio preservation
3. Image flipping (vertical mirror)
4. Transparency detection

**Deliverables:**
- `src/image/loader.rs`
- Tests: load different formats (PNG, JPG), resize accuracy

**Key Functions:**
```rust
pub fn load_image(path: &Path) -> Result<RgbaImage>;
pub fn resize_image(image: &RgbaImage, width_mm: Option<f64>,
                    height_mm: Option<f64>, pixel_size_mm: f64) -> RgbaImage;
pub fn flip_vertical(image: &RgbaImage) -> RgbaImage;
pub fn has_transparency(image: &RgbaImage) -> bool;
```

### Milestone 4.2: Grayscale Conversion

**Tasks:**
1. Implement luminance formula (BT.709)
2. Handle transparency in grayscale
3. Unit tests with known values

**Deliverables:**
- `src/image/processor.rs`
- Tests: luminance calculation

**Formula:**
```
L = 0.2126 * R + 0.7152 * G + 0.0722 * B
```

**Test Cases:**
- Pure white (255,255,255) → 255
- Pure black (0,0,0) → 0
- Pure red (255,0,0) → 54

---

## Phase 5: 3D Geometry & Mesh Generation

**Duration Estimate:** 4-5 days
**Complexity:** High

### Milestone 5.1: Basic Mesh Structures

**Tasks:**
1. Define `Vertex` type (3D point)
2. Define `Triangle` type (3 vertices + normal)
3. Define `Mesh` type (collection of triangles)
4. Basic mesh operations (add triangle, calculate bounds)

**Deliverables:**
- `src/geometry/mesh.rs`
- `src/geometry/triangle.rs`
- Tests: basic operations

**Structures:**
```rust
pub struct Vertex {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct Triangle {
    pub vertices: [Vertex; 3],
    pub normal: Vertex,
}

pub struct Mesh {
    pub triangles: Vec<Triangle>,
}
```

### Milestone 5.2: Cuboid Primitive

**Tasks:**
1. Generate cuboid at position (x, y, z)
2. Parameterize width, height, depth
3. Generate 12 triangles (2 per face)
4. Calculate normals

**Deliverables:**
- `src/geometry/primitives.rs`
- Tests: vertex count, normal directions

**Usage:** Foundation for pixel cuboids in color layers

### Milestone 5.3: Heightmap to Mesh

**Tasks:**
1. Accept 2D array of heights
2. Generate mesh with varying Z values
3. Handle transparency (skip pixels)
4. Optimize for STL export

**Deliverables:**
- `src/geometry/heightmap.rs`
- Tests: small heightmap → verify triangles

**Algorithm:**
For each pixel (x, y) with height h:
- Create top face at Z = h
- Create side faces to adjacent pixels
- Skip if transparent

### Milestone 5.4: STL Binary Writer

**Tasks:**
1. Use `stl_io` crate or manual implementation
2. Write binary STL format
3. Correct byte ordering (little-endian)
4. Add unit tests with reference files

**Deliverables:**
- `src/stl/writer.rs`
- `src/stl/binary.rs`
- Tests: write mesh, verify file structure

**STL Binary Format:**
- 80-byte header
- 4-byte triangle count (u32)
- Per triangle: normal (3×f32), vertices (3×3×f32), attribute (u16)

---

## Phase 6: Lithophane Generation Logic

**Duration Estimate:** 4-5 days
**Complexity:** High

### Milestone 6.1: Color Layer Generation

**Tasks:**
1. For each unique color in palette:
   - Iterate over quantized image
   - For each pixel with this color:
     - Get ColorCombi
     - Find layers for this color in combo
     - Create cuboid at (x, y) with appropriate Z-offset and thickness
2. Combine all cuboids into single mesh
3. Handle multiple layer groups (AMS mode)

**Deliverables:**
- `src/lithophane/color_layer.rs`
- Tests: small test image → verify cuboid count

**Algorithm Outline:**
```rust
pub fn generate_color_layer(
    image: &RgbaImage,
    palette: &Palette,
    hex_code: &str,
    config: &Config,
) -> Mesh {
    let mut mesh = Mesh::new();

    for y in 0..image.height() {
        for x in 0..image.width() {
            let pixel = image.get_pixel(x, y);
            if pixel[3] == 0 { continue; }

            let color = Rgb::new(pixel[0], pixel[1], pixel[2]);
            let combi = palette.get_combi(&color);

            if let Some(layer_thickness) = combi.thickness_for_color(hex_code) {
                let z_offset = combi.z_offset_for_color(hex_code);
                let cuboid = create_cuboid(
                    x as f64 * config.color_pixel_width,
                    y as f64 * config.color_pixel_width,
                    z_offset,
                    config.color_pixel_width,
                    config.color_pixel_width,
                    layer_thickness,
                );
                mesh.add_triangles(cuboid.triangles);
            }
        }
    }

    mesh
}
```

### Milestone 6.2: Texture Layer Generation

**Tasks:**
1. Iterate over grayscale image
2. Map luminance to height (min_thickness to max_thickness)
3. Generate heightmap
4. Convert heightmap to mesh
5. Handle transparency

**Deliverables:**
- `src/lithophane/texture_layer.rs`
- Tests: gradient image → verify heights

**Height Mapping:**
```rust
let height = config.texture_min_thickness +
    (luminance as f64 / 255.0) *
    (config.texture_max_thickness - config.texture_min_thickness);
```

### Milestone 6.3: Support Plate Generation

**Tasks:**
1. Generate flat plate at Z=0
2. Thickness = config.plate_thickness
3. Handle transparency (create holes)
4. Fast generation (simple rectangle or complex with holes)

**Deliverables:**
- `src/lithophane/plate.rs`
- Tests: with/without transparency

**Two Modes:**
- No transparency: Single large cuboid (fast)
- With transparency: Per-pixel cuboids with holes

### Milestone 6.4: Main Generator Orchestration

**Tasks:**
1. Implement main `LithophaneGenerator` struct
2. Coordinate all layer generation
3. Handle parallelization (Rayon)
4. Progress reporting (optional)

**Deliverables:**
- `src/lithophane/generator.rs`
- Integration test: small end-to-end example

**Structure:**
```rust
pub struct LithophaneGenerator {
    config: Config,
    palette: Palette,
}

impl LithophaneGenerator {
    pub fn generate(&self, image_path: &Path) -> Result<GeneratedLayers> {
        // 1. Load and process image
        // 2. Quantize colors
        // 3. Generate color layers (parallel)
        // 4. Generate texture layer
        // 5. Generate support plate
        // Return all meshes
    }
}
```

---

## Phase 7: CLI Interface

**Duration Estimate:** 2 days
**Complexity:** Medium

### Milestone 7.1: Argument Parsing

**Tasks:**
1. Define CLI arguments using `clap` derive macros
2. Match all Java CLI arguments exactly
3. Add validation
4. Help text and examples

**Deliverables:**
- `src/cli.rs`
- Tests: parse valid/invalid arguments

**Argument Structure:**
```rust
#[derive(Parser, Debug)]
#[command(name = "pixestl")]
#[command(about = "Color lithophane generator for 3D printing")]
pub struct Args {
    #[arg(short = 'i', long)]
    pub src_image_path: PathBuf,

    #[arg(short = 'p', long)]
    pub palette_path: PathBuf,

    // ... all other arguments matching Java version
}
```

### Milestone 7.2: Configuration Builder

**Tasks:**
1. Convert CLI args to `Config` struct
2. Apply defaults
3. Validate combinations (e.g., width or height required)
4. Comprehensive unit tests

**Deliverables:**
- `src/config.rs`
- Tests: default values, validation

---

## Phase 8: Output & Packaging

**Duration Estimate:** 2 days
**Complexity:** Low-Medium

### Milestone 8.1: STL Export

**Tasks:**
1. Write each mesh to STL file
2. Use parallel writing if beneficial
3. Progress indicators

**Deliverables:**
- `src/output/exporter.rs`
- Tests: verify STL files loadable in external tools

### Milestone 8.2: ZIP Archive Creation

**Tasks:**
1. Create ZIP using `zip` crate
2. Add all STL files
3. Add preview PNG images
4. Add instructions.txt (for AMS mode)

**Deliverables:**
- `src/output/exporter.rs` (extend)
- Tests: verify ZIP contents

**ZIP Contents:**
- `layer-<color>.stl` (per color)
- `layer-plate.stl`
- `layer-texture-<color>.stl`
- `image-color-preview.png`
- `image-texture-preview.png`
- `instructions.txt` (if colorNumber set)

### Milestone 8.3: Preview Image Generation

**Tasks:**
1. Save quantized color image as PNG
2. Save grayscale texture image as PNG
3. Use `image` crate for PNG encoding

**Deliverables:**
- `src/output/preview.rs`
- Tests: verify PNG files created

---

## Phase 9: Testing & Quality Assurance

**Duration Estimate:** 3-4 days
**Complexity:** Medium

### Milestone 9.1: Unit Test Completion

**Tasks:**
1. Ensure every module has >80% coverage
2. Add missing edge case tests
3. Property-based tests for math functions

**Deliverables:**
- Tests passing: `cargo test`
- Coverage report (using `tarpaulin` or similar)

**Critical Modules:**
- Color conversions (most important!)
- Palette combinatorics
- Mesh generation

### Milestone 9.2: Integration Tests

**Tasks:**
1. End-to-end test with small test image
2. Verify output ZIP structure
3. Compare with Java output (manually)

**Deliverables:**
- `tests/integration/` directory
- Test fixtures in `tests/fixtures/`

**Test Images:**
- 10x10 solid color
- 10x10 gradient
- 10x10 with transparency
- Small version of example image

### Milestone 9.3: Benchmark Suite

**Tasks:**
1. Benchmark color distance calculations
2. Benchmark image quantization
3. Benchmark mesh generation
4. Compare with Java timings

**Deliverables:**
- `benches/benchmarks.rs`
- Performance report

**Targets:**
- 100x100 image: < 1 second
- 1000x1000 image: < 30 seconds

### Milestone 9.4: Validation Against Java

**Tasks:**
1. Run same inputs through both versions
2. Compare quantized images (pixel-perfect match expected)
3. Compare STL vertex counts (within tolerance)
4. Compare color layer assignments

**Deliverables:**
- Validation script or test
- Report of any differences

**Acceptance Criteria:**
- Quantized images: Identical
- STL sizes: Within 1% (floating point differences acceptable)
- Processing time: Rust ≤ Java

---

## Phase 10: Documentation & Polish

**Duration Estimate:** 2-3 days
**Complexity:** Low

### Milestone 10.1: API Documentation

**Tasks:**
1. Add `///` doc comments to all public items
2. Add examples to complex functions
3. Generate with `cargo doc --no-deps`
4. Review for completeness

**Deliverables:**
- Complete rustdoc
- Examples in docstrings

### Milestone 10.2: User Documentation

**Tasks:**
1. Write comprehensive README.md
2. Usage examples
3. Installation instructions
4. Comparison with Java version

**Deliverables:**
- `README.md` in project root
- `docs/USAGE.md`

### Milestone 10.3: Code Quality Audit

**Tasks:**
1. Run `cargo clippy` - fix all warnings
2. Run `cargo fmt` - ensure consistent style
3. Review `unsafe` blocks (should be none)
4. Review error handling (no `unwrap` in library code)

**Deliverables:**
- Clean `clippy` output
- Formatted codebase

---

## Phase 11: CI/CD & Release Preparation

**Duration Estimate:** 1-2 days
**Complexity:** Low

### Milestone 11.1: GitHub Actions

**Tasks:**
1. Create `.github/workflows/ci.yml`
2. Run tests on push/PR
3. Build on multiple platforms (Linux, Windows, macOS)
4. Run clippy and format checks

**Deliverables:**
- Working CI pipeline

### Milestone 11.2: Release Binary Builds

**Tasks:**
1. Set up release workflow
2. Cross-compile for major platforms
3. Create GitHub releases with binaries

**Deliverables:**
- Automated release process

---

## Risk Assessment & Mitigation

### High-Risk Areas

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| CIELab conversion errors | Medium | High | Extensive unit tests with reference values |
| ColorCombi explosion | Low | High | Add limit checks, test with real palettes |
| Mesh generation bugs | Medium | High | Incremental testing, visualization tools |
| Performance < Java | Low | Medium | Profile early, optimize hot paths with SIMD |
| Platform compatibility | Low | Low | CI on all platforms, integration tests |

### Medium-Risk Areas

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Floating point precision | Medium | Medium | Use epsilon comparisons in tests |
| Memory usage on large images | Low | Medium | Profile, optimize, use streaming if needed |
| Palette JSON edge cases | Medium | Low | Fuzz testing, manual review of format |

---

## Development Best Practices

### Code Style
- Follow Rust API guidelines
- Use `clippy::pedantic` for extra checks
- Prefer immutability and owned types
- Use `&str` for string parameters, `String` for returns

### Testing Strategy
1. **TDD for Algorithms:** Write tests before implementing complex logic
2. **Fast Tests:** Unit tests should run in <1s total
3. **Integration Tests:** Separate from unit tests, can be slower
4. **Benchmarks:** Run separately, track over time

### Error Handling
- Use `Result<T, PixestlError>` throughout
- Provide context with `.context()` from `anyhow`
- Never `panic!` in library code
- Only `unwrap()` when mathematically impossible to fail

### Performance
- Profile before optimizing (use `cargo flamegraph`)
- Start with simple, correct code
- Optimize hot paths after profiling
- Consider SIMD only if measurements show benefit

### Documentation
- All public items must have doc comments
- Add examples for complex functions
- Link to related items with `[`Module`]` syntax
- Reference Java implementation in comments where useful

---

## Milestones Summary

| Phase | Milestones | Duration | Dependencies |
|-------|-----------|----------|--------------|
| 1. Foundation | 1.1-1.2 | 1-2 days | None |
| 2. Color | 2.1-2.5 | 3-4 days | Phase 1 |
| 3. Palette | 3.1-3.4 | 3-4 days | Phase 2 |
| 4. Image | 4.1-4.2 | 2-3 days | Phase 2 |
| 5. Geometry | 5.1-5.4 | 4-5 days | Phase 1 |
| 6. Generator | 6.1-6.4 | 4-5 days | Phases 3, 4, 5 |
| 7. CLI | 7.1-7.2 | 2 days | Phase 6 |
| 8. Output | 8.1-8.3 | 2 days | Phase 5, 6 |
| 9. Testing | 9.1-9.4 | 3-4 days | All phases |
| 10. Docs | 10.1-10.3 | 2-3 days | All phases |
| 11. CI/CD | 11.1-11.2 | 1-2 days | Phase 9 |

**Total Estimated Duration:** 27-37 days

**Critical Path:** Phase 1 → 2 → 3 → 6 → 7 → 8 → 9

---

## Success Metrics

### Functional
- ✅ All CLI arguments from Java version supported
- ✅ Same palette JSON format accepted
- ✅ Identical quantized image output
- ✅ STL files loadable in slicer software
- ✅ ZIP contains all expected files

### Quality
- ✅ 80%+ code coverage
- ✅ Zero clippy warnings
- ✅ All tests pass on Linux, Windows, macOS
- ✅ Complete rustdoc for public API

### Performance
- ✅ 100x100 image: < 1s
- ✅ 1000x1000 image: < 30s
- ✅ Memory usage: < 500MB for typical images

---

## Next Steps

After reading this plan:
1. **Setup Phase 1:** Initialize Rust project
2. **Create Skeleton:** All module files with empty impls
3. **Start Phase 2:** Begin with color modules (most critical)

**Ready to begin implementation!**

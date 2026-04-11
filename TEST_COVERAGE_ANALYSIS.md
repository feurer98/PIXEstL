# PIXEstL Test Coverage Analysis

## Current State

The codebase has **~235 tests** (212 unit + 7 integration + 14 doc tests + 2 benchmarks) across
18 modules. All tests pass. The project uses `cargo-tarpaulin` in CI for coverage reporting and
uploads to Codecov. Dev-dependencies include `proptest` and `assert_cmd`, but neither is used in
the test suite yet.

### Per-Module Summary

| Module | File | Tests | Assessment |
|---|---|---|---|
| Color / RGB | `src/color/rgb.rs` | 16 | Good |
| Color / HSL | `src/color/hsl.rs` | 18 | Good |
| Color / CIELab | `src/color/cielab.rs` | 15 | Good |
| Color / Distance | `src/color/distance.rs` | 13 | Good |
| Geometry | `src/lithophane/geometry.rs` | 23 | Good |
| Config | `src/lithophane/config.rs` | 14 | Good |
| Color Layer (mesh) | `src/lithophane/color_layer.rs` | 11 | Good |
| Texture Layer | `src/lithophane/texture_layer.rs` | 6 | Adequate |
| Calibration | `src/lithophane/calibration.rs` | 8 | Adequate |
| Image | `src/image/mod.rs` | 14 | Good (but missing error paths) |
| Palette core | `src/palette/mod.rs` | 5 | Adequate |
| Palette Loader | `src/palette/loader.rs` | 12 | Good |
| Palette ColorCombi | `src/palette/color_combi.rs` | 12 | Good |
| Palette ColorLayer | `src/palette/color_layer.rs` | 10 | Good |
| Palette Generator | `src/palette/generator.rs` | 10 | Good |
| Palette Quantize | `src/palette/quantize.rs` | 6 | Adequate |
| STL / 3MF Export | `src/stl/mod.rs` | 12 | Good |
| Filament | `src/filament/mod.rs` | 8 | Good |
| Error | `src/error.rs` | 2 | **Minimal** |
| **Generator** | **`src/lithophane/generator.rs`** | **0** | **Untested** |
| **Support Plate** | **`src/lithophane/support_plate.rs`** | **0** | **Untested** |
| **CLI** | **`src/cli/mod.rs`** | **0** | **Untested** |
| **Layer (NamedLayer)** | **`src/lithophane/layer.rs`** | **0** | **Untested** |

---

## Priority 1 -- Critical Gaps (Zero Coverage)

### 1. `LithophaneGenerator` (`src/lithophane/generator.rs`)

This is the **core orchestrator** of the entire application. It has two public methods
(`new`, `generate`) and several private helpers, none of which have unit tests. It is only exercised
indirectly by the three integration tests in `tests/integration_test.rs`.

**Recommended tests:**

- `new()` with a valid config succeeds
- `new()` with an invalid config propagates the validation error
- `effective_dimensions()` when both `dest_width_mm` and `dest_height_mm` are 0 (derives from
  image size)
- `effective_dimensions()` when only width is set (height is 0)
- `effective_dimensions()` when only height is set (width is 0)
- `generate()` with color layer disabled (`config.color_layer = false`) produces only a texture
  layer
- `generate()` with texture layer disabled (`config.no_texture = true`) produces color layers + plate
- `generate()` with curve > 0 applies curve transformation to all layers
- `generate_color_layers()` with an empty palette returns `Err(InvalidPalette)`
- `pixels_to_image()` with an empty 0-row input returns a 0x0 image
- `pixels_to_image()` roundtrip: known pixel grid -> image -> extract should match

### 2. `support_plate` (`src/lithophane/support_plate.rs`)

A small module (1 public function), but completely untested.

**Recommended tests:**

- Generated plate mesh has exactly 12 triangles (one cube = 6 faces x 2 triangles)
- Plate dimensions match `image_width * color_pixel_width` x `image_height * color_pixel_width`
- Plate z-position is negative (sits below the color stack)
- Plate with `plate_thickness = 0.0` produces a degenerate flat mesh

### 3. CLI (`src/cli/mod.rs`)

This is ~500 lines of code with zero tests. The project already has `assert_cmd` and `predicates`
in dev-dependencies -- they were added specifically for CLI testing but are **unused**.

**Recommended tests:**

- `to_lithophane_config()`: verify the CLI-to-config mapping for all fields, including negated
  flags (`--no-color` -> `color_layer: false`, `--no-texture` -> `texture_layer: false`)
- `compute_effective_width_mm()`: width given, height given, both given, neither given
- `export_layers()`: routing to ZIP / 3MF / directory based on file extension
- End-to-end CLI tests (via `assert_cmd`):
  - `pixestl --help` exits 0 and prints usage
  - `pixestl` with missing required args exits non-zero
  - `pixestl -i nonexistent.png -p palette.json -o out.zip` prints an image-load error
  - Successful run with the test image and test palette produces a valid ZIP output

### 4. `NamedLayer` (`src/lithophane/layer.rs`)

Simple data-holder struct, but `new()` and `without_color()` constructors are untested.

**Recommended tests:**

- `new()` stores name, mesh, and hex_color
- `without_color()` sets `hex_color` to `None`

---

## Priority 2 -- Error Handling Gaps

### 5. `PixestlError` (`src/error.rs`)

Only 2 of 8 error variants have Display tests. The `From` impls (for `serde_json::Error`,
`std::io::Error`, `zip::result::ZipError`) are completely untested.

**Recommended tests:**

- Display output for every variant: `ImageLoad`, `Json`, `Config`, `Io`, `Zip`,
  `InvalidHexCode`, `InvalidPalette`, `Export`
- `From<serde_json::Error>` conversion
- `From<std::io::Error>` conversion
- `From<zip::result::ZipError>` conversion

### 6. Image loading error paths (`src/image/mod.rs`)

`load_image()` is never tested -- not even for the happy path. All image tests create in-memory
images and bypass file I/O entirely.

**Recommended tests:**

- `load_image()` with a valid test PNG succeeds
- `load_image()` with a nonexistent path returns `Err(PixestlError::ImageLoad)`
- `load_image()` with a corrupt file returns an error
- `resize_image()` with zero width and zero height returns an error

### 7. STL/3MF export error paths (`src/stl/mod.rs`)

Export functions are tested for correctness but not for failure modes.

**Recommended tests:**

- `export_to_dir()` with a non-writable directory path returns `Err`
- `export_to_zip()` to an invalid path returns `Err`
- `write_stl()` with a writer that fails mid-write returns `Err`

---

## Priority 3 -- Edge Cases and Robustness

### 8. Geometry edge cases (`src/lithophane/geometry.rs`)

- `Vector3::normalize()` on a zero-length vector (currently returns self silently -- worth a test
  to document this behavior)
- `Mesh::merge_owned()` -- not tested at all, only `merge()` is
- `Mesh::apply_curve()` with very small widths (potential division by near-zero)
- `Mesh::with_capacity()` -- unused in tests
- Cross product of parallel vectors (result should be zero vector)

### 9. Palette quantization edge cases (`src/palette/quantize.rs`)

Only 6 tests for a module that handles all pixel-to-palette mapping.

**Recommended tests:**

- Quantization of an image where every pixel is transparent (should produce empty result or
  pass-through)
- Quantization with a single-color palette (all pixels map to that color)
- Quantization accuracy: known input colors should map to the nearest palette entry
  (verify with both CIELab and RGB methods)
- `quantize_with_stats()` verifying stat fields: `unique_colors`, `total_pixels`, `color_usage`

### 10. Color layer mesh generation edge cases (`src/lithophane/color_layer.rs`)

- Layer with all-transparent image (no geometry generated)
- Single-pixel image (minimum valid case)
- Image where the target hex code doesn't match any pixels (empty mesh)

### 11. Texture layer edge cases (`src/lithophane/texture_layer.rs`)

- 1x1 pixel image
- Image with uniform brightness (all triangles at same height)
- Image where transparency is present (behavior undefined in current tests)

---

## Priority 4 -- Leveraging Unused Infrastructure

### 12. Property-based testing with `proptest`

`proptest` is in dev-dependencies but unused. It would add significant value for:

- **Color roundtrips**: `RGB -> HSL -> RGB`, `RGB -> CIELab -> RGB`, `RGB -> CMYK -> RGB`
  should produce results within tolerance for arbitrary inputs
- **Color distance properties**: symmetry (`d(a,b) == d(b,a)`), non-negativity, identity
  (`d(a,a) == 0`) for arbitrary colors
- **Geometry invariants**: `normalize().length() ≈ 1.0` for any non-zero vector, triangle normal
  is perpendicular to edges, mesh triangle count after merge equals sum of inputs
- **Config validation**: randomly generated configs with invalid ranges should always fail
  validation

### 13. CLI integration tests with `assert_cmd`

`assert_cmd` and `predicates` are in dev-dependencies but unused. These should power the CLI tests
described in Priority 1, item 3.

---

## Priority 5 -- Structural Improvements

### 14. Missing doc-tests

Several public API functions lack doc-test examples:

- `Palette::find_closest()`
- `Palette::hex_color_groups()`
- `LithophaneGenerator::new()` and `generate()`
- `export_to_dir()`, `export_to_zip()`, `export_to_3mf()`
- `generate_calibration_pattern()`

Adding doc-tests both documents the API and provides lightweight regression tests.

### 15. Benchmark coverage

The benchmark suite (`benches/benchmarks.rs`) covers color operations and mesh basics, but misses:

- Full lithophane generation pipeline (end-to-end performance baseline)
- STL binary export throughput (write performance for large meshes)
- Image resizing performance at various scales
- Palette quantization at scale (large image + large palette)

---

## Suggested Implementation Order

| Step | Area | Effort | Impact |
|---|---|---|---|
| 1 | Generator unit tests | Medium | High -- core logic currently only tested indirectly |
| 2 | CLI tests with `assert_cmd` | Medium | High -- 500 LOC untested, tools already available |
| 3 | Error variant Display + From tests | Small | Medium -- documents error contract |
| 4 | `load_image()` error path tests | Small | Medium -- validates I/O boundary |
| 5 | Support plate tests | Small | Low-Medium -- small module, easy win |
| 6 | `proptest` roundtrip properties | Medium | Medium -- catches edge cases automatically |
| 7 | Quantization edge cases | Small | Medium -- important for output correctness |
| 8 | Geometry edge cases | Small | Low -- already well-tested, fills gaps |
| 9 | Export error path tests | Small | Low -- failure modes are straightforward |
| 10 | Doc-tests for public API | Medium | Low -- documentation + lightweight testing |

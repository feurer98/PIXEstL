//! Integration tests for PIXEstL

use pixestl::lithophane::{LithophaneConfig, PixelCreationMethod};

// ── Full Pipeline Helpers ────────────────────────────────────────────────────

/// Builds a small RGBA test image (top half red, bottom half white).
fn test_image(width: u32, height: u32) -> image::DynamicImage {
    use image::{ImageBuffer, Rgba};
    let buf = ImageBuffer::from_fn(width, height, |_x, y| {
        if y < height / 2 {
            Rgba([255u8, 0, 0, 255]) // Red, fully opaque
        } else {
            Rgba([255u8, 255, 255, 255]) // White, fully opaque
        }
    });
    image::DynamicImage::ImageRgba8(buf)
}

/// Writes a minimal two-color (Red + White) palette JSON to a temp file.
fn test_palette_file() -> tempfile::NamedTempFile {
    use std::io::Write;
    let mut f = tempfile::NamedTempFile::new().unwrap();
    // r##"…"## needed because the JSON keys contain "#" followed by letters,
    // which would otherwise terminate an r#"…"# raw-string literal.
    let json = r##"{
  "#FF0000": {
    "name": "Red",
    "active": true,
    "layers": {
      "5": { "H": 0, "S": 100, "L": 50 },
      "3": { "H": 0, "S": 100, "L": 65 }
    }
  },
  "#FFFFFF": {
    "name": "White",
    "active": true,
    "layers": {
      "5": { "H": 0, "S": 0, "L": 100 }
    }
  }
}"##;
    write!(f, "{json}").unwrap();
    f
}

#[test]
fn test_config_validation() {
    // Invalid: no layers enabled
    let config = LithophaneConfig {
        color_layer: false,
        texture_layer: false,
        ..Default::default()
    };
    assert!(config.validate().is_err());

    // Invalid: zero pixel width
    let config = LithophaneConfig {
        color_pixel_width: 0.0,
        ..Default::default()
    };
    assert!(config.validate().is_err());

    // Invalid: max < min thickness
    let config = LithophaneConfig {
        texture_min_thickness: 2.0,
        texture_max_thickness: 1.0,
        ..Default::default()
    };
    assert!(config.validate().is_err());

    // Valid config
    let config = LithophaneConfig::default();
    assert!(config.validate().is_ok());
}

#[test]
fn test_pixel_creation_method() {
    let method = PixelCreationMethod::Additive;
    assert_eq!(method, PixelCreationMethod::Additive);

    let method = PixelCreationMethod::Full;
    assert_eq!(method, PixelCreationMethod::Full);
}

#[test]
fn test_config_defaults() {
    let config = LithophaneConfig::default();
    assert_eq!(config.color_pixel_width, 0.8);
    assert_eq!(config.color_pixel_layer_thickness, 0.1);
    assert_eq!(config.color_pixel_layer_number, 5);
    assert_eq!(config.texture_pixel_width, 0.25);
    assert_eq!(config.texture_min_thickness, 0.3);
    assert_eq!(config.texture_max_thickness, 1.8);
    assert_eq!(config.plate_thickness, 0.2);
    assert!(config.color_layer);
    assert!(config.texture_layer);
}

#[test]
fn test_total_color_layer_height() {
    let config = LithophaneConfig {
        color_pixel_layer_thickness: 0.1,
        color_pixel_layer_number: 5,
        ..Default::default()
    };
    assert_eq!(config.total_color_layer_height(), 0.5);
}

// ── Full Pipeline Integration Tests ─────────────────────────────────────────

/// End-to-end test: image → palette → color-layer generation → ZIP export.
///
/// Verifies that the complete color-layer pipeline produces non-empty STL meshes
/// and that the ZIP archive contains one `.stl` entry per generated layer.
#[test]
fn test_full_pipeline_color_layer_only() {
    use pixestl::palette::{PaletteLoader, PaletteLoaderConfig};
    use pixestl::stl::{export_to_zip, StlFormat};
    use pixestl::LithophaneGenerator;

    // 1. Programmatic 8×8 test image (no file I/O for image)
    let image = test_image(8, 8);

    // 2. Palette loaded from a temp JSON file
    let palette_file = test_palette_file();
    let palette = PaletteLoader::load(palette_file.path(), PaletteLoaderConfig::default())
        .expect("palette must load");
    assert!(palette.color_count() > 0, "palette must have colors");

    // 3. Generator configured for color-layer-only output
    let config = LithophaneConfig {
        dest_width_mm: 8.0,
        dest_height_mm: 8.0,
        color_pixel_width: 1.0,
        texture_layer: false, // only color layer
        ..LithophaneConfig::default()
    };
    let generator = LithophaneGenerator::new(config).expect("config must be valid");

    // 4. Generate layers
    let layers = generator
        .generate(&image, &palette)
        .expect("generation must succeed");
    assert!(!layers.is_empty(), "must produce at least one layer");

    // Every mesh must contain triangles
    for (name, mesh) in &layers {
        assert!(mesh.triangle_count() > 0, "layer '{name}' has no triangles");
    }

    // 5. Export to ZIP
    let zip_tmp = tempfile::NamedTempFile::new().unwrap();
    export_to_zip(&layers, zip_tmp.path(), StlFormat::Binary).expect("ZIP export must succeed");

    // 6. Verify ZIP structure
    let zip_bytes = std::fs::read(zip_tmp.path()).unwrap();
    let mut archive =
        zip::ZipArchive::new(std::io::Cursor::new(zip_bytes)).expect("output must be a valid ZIP");

    assert_eq!(
        archive.len(),
        layers.len(),
        "ZIP must contain exactly one entry per layer"
    );
    for i in 0..archive.len() {
        let entry = archive.by_index(i).unwrap();
        assert!(
            entry.name().ends_with(".stl"),
            "ZIP entry '{}' must have .stl extension",
            entry.name()
        );
        // Binary STL: 80-byte header + 4-byte count + ≥50 bytes per triangle
        assert!(
            entry.size() > 84,
            "STL entry '{}' must be larger than an empty header (84 bytes)",
            entry.name()
        );
    }
}

/// End-to-end test: image → texture-layer generation → ZIP export.
///
/// No palette quantization is involved; the generator converts the image to
/// grayscale internally and maps brightness to layer thickness.
#[test]
fn test_full_pipeline_texture_layer_only() {
    use pixestl::palette::{PaletteLoader, PaletteLoaderConfig};
    use pixestl::stl::{export_to_zip, StlFormat};
    use pixestl::LithophaneGenerator;

    let image = test_image(8, 8);

    // A valid palette is still required by the function signature, even though
    // it is not accessed when color_layer is disabled.
    let palette_file = test_palette_file();
    let palette = PaletteLoader::load(palette_file.path(), PaletteLoaderConfig::default())
        .expect("palette must load");

    let config = LithophaneConfig {
        dest_width_mm: 8.0,
        dest_height_mm: 8.0,
        texture_pixel_width: 1.0,
        color_layer: false, // only texture layer
        texture_layer: true,
        ..LithophaneConfig::default()
    };
    let generator = LithophaneGenerator::new(config).expect("config must be valid");

    let layers = generator
        .generate(&image, &palette)
        .expect("generation must succeed");
    assert!(!layers.is_empty(), "must produce at least one layer");

    for (name, mesh) in &layers {
        assert!(mesh.triangle_count() > 0, "layer '{name}' has no triangles");
    }

    let zip_tmp = tempfile::NamedTempFile::new().unwrap();
    export_to_zip(&layers, zip_tmp.path(), StlFormat::Ascii).expect("ZIP export must succeed");

    let zip_bytes = std::fs::read(zip_tmp.path()).unwrap();
    let mut archive =
        zip::ZipArchive::new(std::io::Cursor::new(zip_bytes)).expect("output must be a valid ZIP");

    assert_eq!(archive.len(), layers.len());
    for i in 0..archive.len() {
        let entry = archive.by_index(i).unwrap();
        assert!(entry.name().ends_with(".stl"));
        // ASCII STL is always larger than just the header
        assert!(entry.size() > 0);
    }
}

/// End-to-end test combining both color and texture layers.
///
/// Exercises the full generation path: support plate + color layers +
/// texture layer all exported into one ZIP archive.
#[test]
fn test_full_pipeline_both_layers() {
    use pixestl::palette::{PaletteLoader, PaletteLoaderConfig};
    use pixestl::stl::{export_to_zip, StlFormat};
    use pixestl::LithophaneGenerator;

    let image = test_image(8, 8);

    let palette_file = test_palette_file();
    let palette = PaletteLoader::load(palette_file.path(), PaletteLoaderConfig::default())
        .expect("palette must load");

    // Default config already enables both color_layer and texture_layer
    let config = LithophaneConfig {
        dest_width_mm: 8.0,
        dest_height_mm: 8.0,
        color_pixel_width: 1.0,
        texture_pixel_width: 1.0,
        ..LithophaneConfig::default()
    };
    let generator = LithophaneGenerator::new(config).expect("config must be valid");

    let layers = generator
        .generate(&image, &palette)
        .expect("generation must succeed");

    // Expect: support plate + at least one color layer + one texture layer
    assert!(
        layers.len() >= 3,
        "expected ≥ 3 layers (plate + color + texture), got {}",
        layers.len()
    );

    let has_plate = layers.iter().any(|(name, _)| name == "layer-plate");
    let has_texture = layers.iter().any(|(name, _)| name == "layer-texture");
    assert!(has_plate, "support plate layer must be present");
    assert!(has_texture, "texture layer must be present");

    for (name, mesh) in &layers {
        assert!(mesh.triangle_count() > 0, "layer '{name}' has no triangles");
    }

    let zip_tmp = tempfile::NamedTempFile::new().unwrap();
    export_to_zip(&layers, zip_tmp.path(), StlFormat::Binary).expect("ZIP export must succeed");

    let zip_bytes = std::fs::read(zip_tmp.path()).unwrap();
    let archive =
        zip::ZipArchive::new(std::io::Cursor::new(zip_bytes)).expect("output must be a valid ZIP");
    assert_eq!(
        archive.len(),
        layers.len(),
        "ZIP must contain one file per layer"
    );
}

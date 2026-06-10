//! Main lithophane generator orchestrator

use crate::color::Rgb;
use crate::error::{PixestlError, Result};
use crate::image::{
    convert_to_grayscale, extract_pixels, flip_vertical, has_transparent_pixel, resize_image,
};
use crate::lithophane::config::LithophaneConfig;
use crate::lithophane::geometry::Vector3;
use crate::lithophane::layer::NamedLayer;
use crate::lithophane::{color_layer, support_plate, texture_layer};
use crate::palette::{quantize_image_opt, Palette};
use image::{DynamicImage, RgbaImage};

pub struct LithophaneGenerator {
    config: LithophaneConfig,
}

impl LithophaneGenerator {
    pub fn new(config: LithophaneConfig) -> Result<Self> {
        config.validate()?;
        Ok(Self { config })
    }

    pub fn generate(&self, image: &DynamicImage, palette: &Palette) -> Result<Vec<NamedLayer>> {
        let mut layers = Vec::new();

        // When neither --width nor --height is specified (both are 0), derive the physical
        // dimensions from the source image using color_pixel_width as the scale factor.
        // This ensures color and texture layers cover the same physical area.
        let (eff_width_mm, eff_height_mm) = self.effective_dimensions(image);

        let color_image = if self.config.color_layer {
            let resized = resize_image(
                image,
                eff_width_mm,
                eff_height_mm,
                self.config.color_pixel_width,
            )?;

            // Transparent pixels stay None at their position; filtering them
            // out would shift every pixel right of a hole to the left and
            // erase the transparency information for the plate/edge logic.
            let pixels = extract_pixels(&resized);

            let palette_colors: Vec<Rgb> = palette.colors().collect();
            let quantized_pixels =
                quantize_image_opt(&pixels, &palette_colors, self.config.color_distance_method)?;

            let quantized = pixels_to_image(quantized_pixels);
            Some(flip_vertical(&quantized))
        } else {
            None
        };

        let texture_image = if self.config.texture_layer {
            let resized = resize_image(
                image,
                eff_width_mm,
                eff_height_mm,
                self.config.texture_pixel_width,
            )?;

            let grayscale = convert_to_grayscale(&resized);
            Some(flip_vertical(&grayscale))
        } else {
            None
        };

        if let Some(ref color_img) = color_image {
            if !has_transparent_pixel(color_img) {
                let plate = support_plate::generate_support_plate(color_img, &self.config)?;
                layers.push(NamedLayer::without_color("layer-plate".to_string(), plate));
            }
        }

        if let Some(ref color_img) = color_image {
            let color_layers = self.generate_color_layers(color_img, palette)?;
            layers.extend(color_layers);
        }

        if let Some(ref texture_img) = texture_image {
            let mut texture_mesh =
                texture_layer::generate_texture_layer(texture_img, &self.config)?;
            // The texture relief sits on top of the color stack (Java original:
            // CSGThreadTextureRow translates by colorPixelLayerThickness * layerCount).
            // Without this offset the texture and color solids would interpenetrate.
            if self.config.color_layer {
                texture_mesh.translate_mut(Vector3::new(
                    0.0,
                    0.0,
                    self.config.total_color_layer_height(),
                ));
            }
            layers.push(NamedLayer::new(
                "layer-texture".to_string(),
                texture_mesh,
                Some(self.config.texture_color.clone()),
            ));
        }

        // Apply curve transformation if configured
        if self.config.curve > 0.0 {
            let total_width = self.compute_total_width(image);
            for layer in &mut layers {
                layer.mesh =
                    std::mem::take(&mut layer.mesh).apply_curve(self.config.curve, total_width);
            }
        }

        Ok(layers)
    }

    /// Returns the effective physical dimensions (width_mm, height_mm) to use for resizing.
    ///
    /// When both `dest_width_mm` and `dest_height_mm` are 0 (not specified by the user),
    /// derives the physical size from the source image at 1 source pixel per color pixel.
    /// This prevents a zero-dimension crash and keeps color/texture layers physically aligned.
    fn effective_dimensions(&self, image: &DynamicImage) -> (f64, f64) {
        let w = self.config.dest_width_mm;
        let h = self.config.dest_height_mm;
        if w > 0.0 || h > 0.0 {
            (w, h)
        } else {
            // No explicit size: map 1 source pixel → 1 color pixel
            (
                image.width() as f64 * self.config.color_pixel_width,
                image.height() as f64 * self.config.color_pixel_width,
            )
        }
    }

    /// Computes the total width of the lithophane in mm for curve transformation.
    fn compute_total_width(&self, image: &DynamicImage) -> f64 {
        let (w, _) = self.effective_dimensions(image);
        w
    }

    fn generate_color_layers(
        &self,
        image: &RgbaImage,
        palette: &Palette,
    ) -> Result<Vec<NamedLayer>> {
        let mut layers = Vec::new();
        let hex_color_groups = palette.hex_color_groups();

        if hex_color_groups.is_empty() {
            return Err(PixestlError::InvalidPalette(
                "No color groups found in palette".to_string(),
            ));
        }

        for (group_idx, hex_codes) in hex_color_groups.iter().enumerate() {
            let mut color_names = Vec::new();
            for hex_code in hex_codes {
                if let Some(name) = palette.get_color_name(hex_code) {
                    color_names.push(name.to_string());
                }
            }

            let layer_name = if color_names.is_empty() {
                format!("layer-{}", group_idx + 1)
            } else {
                format!("layer-{}", color_names.join("+"))
            };

            // Use the first hex code in the group as the representative color for 3MF export.
            // For single-filament groups (the common case) this is exact.
            let representative_color = hex_codes.first().cloned();

            let mesh =
                color_layer::generate_color_layer(image, palette, hex_codes, &self.config, -1, -1)?;

            layers.push(NamedLayer::new(layer_name, mesh, representative_color));
        }

        Ok(layers)
    }
}

/// Builds an RGBA image from quantized pixels; `None` becomes a transparent
/// pixel so downstream transparency handling (plate skip, edge trimming in
/// the color layer) sees the original holes.
fn pixels_to_image(pixels: Vec<Vec<Option<Rgb>>>) -> RgbaImage {
    use image::{ImageBuffer, Rgba};

    let height = pixels.len() as u32;
    let width = if height > 0 {
        pixels[0].len() as u32
    } else {
        0
    };

    ImageBuffer::from_fn(width, height, |x, y| {
        match pixels
            .get(y as usize)
            .and_then(|row| row.get(x as usize))
            .copied()
            .flatten()
        {
            Some(rgb) => Rgba([rgb.r, rgb.g, rgb.b, 255]),
            None => Rgba([0, 0, 0, 0]),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::palette::{PaletteLoader, PaletteLoaderConfig};
    use image::{ImageBuffer, Rgba};
    use std::io::Write as _;

    fn default_config() -> LithophaneConfig {
        LithophaneConfig::default()
    }

    fn make_image(width: u32, height: u32, color: [u8; 4]) -> DynamicImage {
        let buf = ImageBuffer::from_fn(width, height, |_x, _y| Rgba(color));
        DynamicImage::ImageRgba8(buf)
    }

    fn make_red_white_image(width: u32, height: u32) -> DynamicImage {
        let buf = ImageBuffer::from_fn(width, height, |_x, y| {
            if y < height / 2 {
                Rgba([255u8, 0, 0, 255])
            } else {
                Rgba([255u8, 255, 255, 255])
            }
        });
        DynamicImage::ImageRgba8(buf)
    }

    fn test_palette_file() -> tempfile::NamedTempFile {
        let mut f = tempfile::NamedTempFile::new().unwrap();
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

    fn load_test_palette() -> Palette {
        let file = test_palette_file();
        PaletteLoader::load(file.path(), PaletteLoaderConfig::default()).unwrap()
    }

    // ── LithophaneGenerator::new ───────────────────────────────────────────

    #[test]
    fn test_new_with_valid_config() {
        let config = default_config();
        assert!(LithophaneGenerator::new(config).is_ok());
    }

    #[test]
    fn test_new_with_invalid_config_propagates_error() {
        let config = LithophaneConfig {
            color_pixel_width: 0.0,
            ..default_config()
        };
        let result = LithophaneGenerator::new(config);
        assert!(result.is_err());
    }

    // ── effective_dimensions ────────────────────────────────────────────────

    #[test]
    fn test_effective_dimensions_both_zero_derives_from_image() {
        let config = LithophaneConfig {
            dest_width_mm: 0.0,
            dest_height_mm: 0.0,
            color_pixel_width: 0.5,
            ..default_config()
        };
        let gen = LithophaneGenerator::new(config).unwrap();
        let image = make_image(10, 20, [255, 0, 0, 255]);

        let (w, h) = gen.effective_dimensions(&image);
        assert!((w - 5.0).abs() < 1e-10, "expected 10*0.5=5.0, got {w}");
        assert!((h - 10.0).abs() < 1e-10, "expected 20*0.5=10.0, got {h}");
    }

    #[test]
    fn test_effective_dimensions_width_set() {
        let config = LithophaneConfig {
            dest_width_mm: 50.0,
            dest_height_mm: 0.0,
            ..default_config()
        };
        let gen = LithophaneGenerator::new(config).unwrap();
        let image = make_image(10, 20, [255, 0, 0, 255]);

        let (w, h) = gen.effective_dimensions(&image);
        assert!((w - 50.0).abs() < 1e-10);
        assert!((h - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_effective_dimensions_height_set() {
        let config = LithophaneConfig {
            dest_width_mm: 0.0,
            dest_height_mm: 30.0,
            ..default_config()
        };
        let gen = LithophaneGenerator::new(config).unwrap();
        let image = make_image(10, 20, [255, 0, 0, 255]);

        let (w, h) = gen.effective_dimensions(&image);
        assert!((w - 0.0).abs() < 1e-10);
        assert!((h - 30.0).abs() < 1e-10);
    }

    #[test]
    fn test_effective_dimensions_both_set() {
        let config = LithophaneConfig {
            dest_width_mm: 50.0,
            dest_height_mm: 30.0,
            ..default_config()
        };
        let gen = LithophaneGenerator::new(config).unwrap();
        let image = make_image(10, 20, [255, 0, 0, 255]);

        let (w, h) = gen.effective_dimensions(&image);
        assert!((w - 50.0).abs() < 1e-10);
        assert!((h - 30.0).abs() < 1e-10);
    }

    // ── generate: color-only ────────────────────────────────────────────────

    #[test]
    fn test_generate_color_only_produces_plate_and_color_layers() {
        let palette = load_test_palette();
        let image = make_red_white_image(8, 8);

        let config = LithophaneConfig {
            dest_width_mm: 8.0,
            dest_height_mm: 8.0,
            color_pixel_width: 1.0,
            color_layer: true,
            texture_layer: false,
            ..default_config()
        };
        let gen = LithophaneGenerator::new(config).unwrap();
        let layers = gen.generate(&image, &palette).unwrap();

        assert!(!layers.is_empty());
        assert!(
            layers.iter().any(|l| l.name == "layer-plate"),
            "should include support plate"
        );
        // Should have no texture layer
        assert!(
            !layers.iter().any(|l| l.name == "layer-texture"),
            "should not include texture layer"
        );
        // All layers must have geometry
        for layer in &layers {
            assert!(
                layer.mesh.triangle_count() > 0,
                "layer '{}' is empty",
                layer.name
            );
        }
    }

    // ── generate: texture-only ──────────────────────────────────────────────

    #[test]
    fn test_generate_texture_only_produces_texture_layer() {
        let palette = load_test_palette();
        let image = make_red_white_image(8, 8);

        let config = LithophaneConfig {
            dest_width_mm: 8.0,
            dest_height_mm: 8.0,
            texture_pixel_width: 1.0,
            color_layer: false,
            texture_layer: true,
            ..default_config()
        };
        let gen = LithophaneGenerator::new(config).unwrap();
        let layers = gen.generate(&image, &palette).unwrap();

        assert_eq!(
            layers.len(),
            1,
            "texture-only should produce exactly 1 layer"
        );
        assert_eq!(layers[0].name, "layer-texture");
        assert!(layers[0].mesh.triangle_count() > 0);
    }

    // ── generate: both layers ───────────────────────────────────────────────

    #[test]
    fn test_generate_both_layers_includes_plate_color_and_texture() {
        let palette = load_test_palette();
        let image = make_red_white_image(8, 8);

        let config = LithophaneConfig {
            dest_width_mm: 8.0,
            dest_height_mm: 8.0,
            color_pixel_width: 1.0,
            texture_pixel_width: 1.0,
            color_layer: true,
            texture_layer: true,
            ..default_config()
        };
        let gen = LithophaneGenerator::new(config).unwrap();
        let layers = gen.generate(&image, &palette).unwrap();

        assert!(
            layers.len() >= 3,
            "expected plate + color + texture, got {}",
            layers.len()
        );
        assert!(layers.iter().any(|l| l.name == "layer-plate"));
        assert!(layers.iter().any(|l| l.name == "layer-texture"));
    }

    // ── generate: curve transformation ──────────────────────────────────────

    #[test]
    fn test_generate_with_curve_applies_transformation() {
        let palette = load_test_palette();
        let image = make_red_white_image(8, 8);

        let config_flat = LithophaneConfig {
            dest_width_mm: 8.0,
            dest_height_mm: 8.0,
            color_pixel_width: 1.0,
            texture_layer: false,
            curve: 0.0,
            ..default_config()
        };
        let gen_flat = LithophaneGenerator::new(config_flat).unwrap();
        let layers_flat = gen_flat.generate(&image, &palette).unwrap();

        let config_curved = LithophaneConfig {
            dest_width_mm: 8.0,
            dest_height_mm: 8.0,
            color_pixel_width: 1.0,
            texture_layer: false,
            curve: 180.0,
            ..default_config()
        };
        let gen_curved = LithophaneGenerator::new(config_curved).unwrap();
        let layers_curved = gen_curved.generate(&image, &palette).unwrap();

        // Same layers; curve mode segments per pixel column (plate and RLE),
        // so curved layers have at least as many triangles as flat ones.
        assert_eq!(layers_flat.len(), layers_curved.len());
        for (flat, curved) in layers_flat.iter().zip(layers_curved.iter()) {
            assert_eq!(flat.name, curved.name);
            assert!(
                curved.mesh.triangle_count() >= flat.mesh.triangle_count(),
                "layer '{}': curved {} < flat {}",
                flat.name,
                curved.mesh.triangle_count(),
                flat.mesh.triangle_count()
            );
        }

        // The plate (8 pixel columns) must be segmented per column.
        let plate = layers_curved
            .iter()
            .find(|l| l.name == "layer-plate")
            .unwrap();
        assert_eq!(plate.mesh.triangle_count(), 12 * 8);
    }

    // ── generate: texture sits on top of the color stack ────────────────────

    fn min_z(mesh: &crate::lithophane::geometry::Mesh) -> f64 {
        mesh.triangles
            .iter()
            .flat_map(|t| [t.v0.z, t.v1.z, t.v2.z])
            .fold(f64::INFINITY, f64::min)
    }

    #[test]
    fn test_texture_layer_starts_above_color_stack() {
        let palette = load_test_palette();
        let image = make_red_white_image(8, 8);

        let config = LithophaneConfig {
            dest_width_mm: 8.0,
            dest_height_mm: 8.0,
            color_pixel_width: 1.0,
            texture_pixel_width: 1.0,
            color_layer: true,
            texture_layer: true,
            ..default_config()
        };
        let stack_height = config.total_color_layer_height();
        let gen = LithophaneGenerator::new(config).unwrap();
        let layers = gen.generate(&image, &palette).unwrap();

        let texture = layers.iter().find(|l| l.name == "layer-texture").unwrap();
        assert!(
            (min_z(&texture.mesh) - stack_height).abs() < 1e-9,
            "texture must start at color stack height {stack_height}, got {}",
            min_z(&texture.mesh)
        );

        // Color layers themselves stay within [0, stack_height].
        for layer in layers
            .iter()
            .filter(|l| l.name != "layer-texture" && l.name != "layer-plate")
        {
            assert!(min_z(&layer.mesh) >= -1e-9, "color layer below z=0");
        }
    }

    #[test]
    fn test_texture_only_stays_at_z_zero() {
        let palette = load_test_palette();
        let image = make_red_white_image(8, 8);

        let config = LithophaneConfig {
            dest_width_mm: 8.0,
            dest_height_mm: 8.0,
            texture_pixel_width: 1.0,
            color_layer: false,
            texture_layer: true,
            ..default_config()
        };
        let gen = LithophaneGenerator::new(config).unwrap();
        let layers = gen.generate(&image, &palette).unwrap();

        assert!(
            min_z(&layers[0].mesh).abs() < 1e-9,
            "texture-only lithophane must rest on z=0"
        );
    }

    // ── generate: texture layer gets correct hex color ──────────────────────

    #[test]
    fn test_generate_texture_layer_carries_texture_color() {
        let palette = load_test_palette();
        let image = make_red_white_image(8, 8);

        let config = LithophaneConfig {
            dest_width_mm: 8.0,
            dest_height_mm: 8.0,
            texture_pixel_width: 1.0,
            color_layer: false,
            texture_layer: true,
            texture_color: "#AABBCC".to_string(),
            ..default_config()
        };
        let gen = LithophaneGenerator::new(config).unwrap();
        let layers = gen.generate(&image, &palette).unwrap();

        let texture = layers.iter().find(|l| l.name == "layer-texture").unwrap();
        assert_eq!(texture.hex_color.as_deref(), Some("#AABBCC"));
    }

    // ── pixels_to_image ─────────────────────────────────────────────────────

    #[test]
    fn test_pixels_to_image_empty() {
        let pixels: Vec<Vec<Option<Rgb>>> = vec![];
        let img = pixels_to_image(pixels);
        assert_eq!(img.width(), 0);
        assert_eq!(img.height(), 0);
    }

    #[test]
    fn test_pixels_to_image_roundtrip() {
        let red = Rgb::new(255, 0, 0);
        let green = Rgb::new(0, 255, 0);
        let blue = Rgb::new(0, 0, 255);
        let white = Rgb::new(255, 255, 255);

        let pixels = vec![vec![Some(red), Some(green)], vec![Some(blue), Some(white)]];
        let img = pixels_to_image(pixels);

        assert_eq!(img.width(), 2);
        assert_eq!(img.height(), 2);

        assert_eq!(img.get_pixel(0, 0).0, [255, 0, 0, 255]);
        assert_eq!(img.get_pixel(1, 0).0, [0, 255, 0, 255]);
        assert_eq!(img.get_pixel(0, 1).0, [0, 0, 255, 255]);
        assert_eq!(img.get_pixel(1, 1).0, [255, 255, 255, 255]);
    }

    #[test]
    fn test_pixels_to_image_single_pixel() {
        let pixels = vec![vec![Some(Rgb::new(128, 64, 32))]];
        let img = pixels_to_image(pixels);
        assert_eq!(img.width(), 1);
        assert_eq!(img.height(), 1);
        assert_eq!(img.get_pixel(0, 0).0, [128, 64, 32, 255]);
    }

    #[test]
    fn test_pixels_to_image_preserves_transparent_position() {
        // A hole in the middle of a row must stay at its position; pixels to
        // its right must NOT shift left.
        let red = Rgb::new(255, 0, 0);
        let blue = Rgb::new(0, 0, 255);
        let pixels = vec![vec![Some(red), None, Some(blue)]];
        let img = pixels_to_image(pixels);

        assert_eq!(img.width(), 3, "row width must include the hole");
        assert_eq!(img.get_pixel(0, 0).0, [255, 0, 0, 255]);
        assert_eq!(img.get_pixel(1, 0).0[3], 0, "hole must stay transparent");
        assert_eq!(img.get_pixel(2, 0).0, [0, 0, 255, 255]);
    }

    // ── Transparenz-Pipeline ────────────────────────────────────────────────

    fn make_image_with_transparent_hole(width: u32, height: u32) -> DynamicImage {
        let buf = ImageBuffer::from_fn(width, height, |x, y| {
            if x == width / 2 && y == height / 2 {
                Rgba([0u8, 0, 0, 0])
            } else {
                Rgba([255u8, 0, 0, 255])
            }
        });
        DynamicImage::ImageRgba8(buf)
    }

    #[test]
    fn test_transparent_image_skips_support_plate() {
        // Regression: transparency was erased during quantization (pixels
        // were filtered out and re-emitted fully opaque), so the plate was
        // generated even for images with holes.
        let palette = load_test_palette();
        let image = make_image_with_transparent_hole(8, 8);

        let config = LithophaneConfig {
            dest_width_mm: 8.0,
            dest_height_mm: 8.0,
            color_pixel_width: 1.0,
            color_layer: true,
            texture_layer: false,
            ..default_config()
        };
        let gen = LithophaneGenerator::new(config).unwrap();
        let layers = gen.generate(&image, &palette).unwrap();

        assert!(
            !layers.iter().any(|l| l.name == "layer-plate"),
            "image with transparency must not get a support plate"
        );
        assert!(
            layers.iter().any(|l| l.mesh.triangle_count() > 0),
            "opaque pixels must still produce geometry"
        );
    }
}

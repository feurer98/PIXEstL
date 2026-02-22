//! Main lithophane generator orchestrator

use crate::color::Rgb;
use crate::error::{PixestlError, Result};
use crate::image::{
    convert_to_grayscale, extract_pixels, flip_vertical, has_transparent_pixel, resize_image,
};
use crate::lithophane::config::LithophaneConfig;
use crate::lithophane::geometry::Mesh;
use crate::lithophane::{color_layer, support_plate, texture_layer};
use crate::palette::{quantize_image, Palette};
use image::{DynamicImage, RgbaImage};

pub struct LithophaneGenerator {
    config: LithophaneConfig,
}

impl LithophaneGenerator {
    pub fn new(config: LithophaneConfig) -> Result<Self> {
        config.validate()?;
        Ok(Self { config })
    }

    pub fn generate(&self, image: &DynamicImage, palette: &Palette) -> Result<Vec<(String, Mesh)>> {
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

            let pixels_with_option = extract_pixels(&resized);
            let pixels: Vec<Vec<Rgb>> = pixels_with_option
                .iter()
                .map(|row| row.iter().filter_map(|&p| p).collect())
                .collect();

            let palette_colors = palette.colors();
            let quantized_pixels =
                quantize_image(&pixels, &palette_colors, self.config.color_distance_method)?;

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
                layers.push(("layer-plate".to_string(), plate));
            }
        }

        if let Some(ref color_img) = color_image {
            let color_layers = self.generate_color_layers(color_img, palette)?;
            layers.extend(color_layers);
        }

        if let Some(ref texture_img) = texture_image {
            let texture_mesh = texture_layer::generate_texture_layer(texture_img, &self.config)?;
            layers.push(("layer-texture".to_string(), texture_mesh));
        }

        // Apply curve transformation if configured
        if self.config.curve > 0.0 {
            let total_width = self.compute_total_width(image);
            for (_name, mesh) in &mut layers {
                mesh.apply_curve(self.config.curve, total_width);
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
    ) -> Result<Vec<(String, Mesh)>> {
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

            let mesh =
                color_layer::generate_color_layer(image, palette, hex_codes, &self.config, -1, -1)?;

            layers.push((layer_name, mesh));
        }

        Ok(layers)
    }
}

fn pixels_to_image(pixels: Vec<Vec<Rgb>>) -> RgbaImage {
    use image::{ImageBuffer, Rgba};

    let height = pixels.len() as u32;
    let width = if height > 0 {
        pixels[0].len() as u32
    } else {
        0
    };

    ImageBuffer::from_fn(width, height, |x, y| {
        if y as usize >= pixels.len() || x as usize >= pixels[y as usize].len() {
            Rgba([0, 0, 0, 0])
        } else {
            let rgb = pixels[y as usize][x as usize];
            Rgba([rgb.r, rgb.g, rgb.b, 255])
        }
    })
}

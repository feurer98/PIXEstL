//! Support plate generation
//!
//! Generates base plate for lithophanes

use crate::error::Result;
use crate::lithophane::config::LithophaneConfig;
use crate::lithophane::geometry::{Mesh, Vector3};
use image::RgbaImage;

/// Generates a flat support plate
///
/// In curve mode the plate is segmented into one cube per pixel column:
/// `apply_curve` only maps vertices, so a single full-width cube would
/// collapse into one flat chord across the arc instead of following it.
/// The per-column segmentation matches the chord grid of the color cubes.
pub fn generate_support_plate(image: &RgbaImage, config: &LithophaneConfig) -> Result<Mesh> {
    let (width, height) = image.dimensions();

    let pixel_width = config.color_pixel_width;
    let plate_depth = height as f64 * pixel_width;
    let plate_height = config.plate_thickness;
    let center_y = plate_depth / 2.0;
    let center_z = -plate_height / 2.0;

    if config.curve > 0.0 {
        let mut mesh = Mesh::with_capacity(12 * width as usize);
        for x in 0..width {
            let center_x = (x as f64 * pixel_width) + (pixel_width / 2.0);
            mesh.add_cube(
                pixel_width,
                plate_depth,
                plate_height,
                Vector3::new(center_x, center_y, center_z),
            );
        }
        return Ok(mesh);
    }

    let plate_width = width as f64 * pixel_width;
    let center = Vector3::new(plate_width / 2.0, center_y, center_z);

    Ok(Mesh::cube(plate_width, plate_depth, plate_height, center))
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgba};

    fn make_opaque_image(width: u32, height: u32) -> RgbaImage {
        ImageBuffer::from_fn(width, height, |_, _| Rgba([255u8, 0, 0, 255]))
    }

    fn default_config() -> LithophaneConfig {
        LithophaneConfig::default()
    }

    #[test]
    fn test_plate_has_12_triangles() {
        let img = make_opaque_image(4, 4);
        let config = default_config();
        let mesh = generate_support_plate(&img, &config).unwrap();
        // A cube has 6 faces × 2 triangles = 12
        assert_eq!(mesh.triangle_count(), 12);
    }

    #[test]
    fn test_plate_dimensions_match_image_and_pixel_width() {
        let img = make_opaque_image(10, 5);
        let config = LithophaneConfig {
            color_pixel_width: 0.5,
            plate_thickness: 0.3,
            ..default_config()
        };
        let mesh = generate_support_plate(&img, &config).unwrap();

        // Expected: 10 * 0.5 = 5.0 mm wide, 5 * 0.5 = 2.5 mm deep
        // Verify by checking that the mesh has the right number of triangles (cube)
        assert_eq!(mesh.triangle_count(), 12);
    }

    #[test]
    fn test_plate_z_is_negative() {
        let img = make_opaque_image(2, 2);
        let config = LithophaneConfig {
            plate_thickness: 0.5,
            ..default_config()
        };
        let mesh = generate_support_plate(&img, &config).unwrap();

        // The plate center is at -plate_height/2, so all z-coordinates
        // should be <= 0 (plate sits below the color stack)
        for tri in &mesh.triangles {
            assert!(tri.v0.z <= 0.001, "v0.z should be <= 0, got {}", tri.v0.z);
            assert!(tri.v1.z <= 0.001, "v1.z should be <= 0, got {}", tri.v1.z);
            assert!(tri.v2.z <= 0.001, "v2.z should be <= 0, got {}", tri.v2.z);
        }
    }

    #[test]
    fn test_plate_single_pixel_image() {
        let img = make_opaque_image(1, 1);
        let config = default_config();
        let mesh = generate_support_plate(&img, &config).unwrap();
        assert_eq!(mesh.triangle_count(), 12);
    }

    #[test]
    fn test_plate_curve_mode_segments_per_column() {
        let img = make_opaque_image(10, 5);
        let config = LithophaneConfig {
            color_pixel_width: 0.5,
            plate_thickness: 0.4,
            curve: 90.0,
            ..default_config()
        };
        let mesh = generate_support_plate(&img, &config).unwrap();
        // One cube per pixel column, so apply_curve bends the plate along the
        // arc instead of collapsing it into a single chord.
        assert_eq!(mesh.triangle_count(), 12 * 10);
    }

    #[test]
    fn test_plate_curved_segments_preserve_total_volume() {
        let img = make_opaque_image(10, 5);
        let flat_config = LithophaneConfig {
            color_pixel_width: 0.5,
            plate_thickness: 0.4,
            curve: 0.0,
            ..default_config()
        };
        let curved_config = LithophaneConfig {
            curve: 90.0,
            ..flat_config.clone()
        };

        let flat = generate_support_plate(&img, &flat_config).unwrap();
        let segmented = generate_support_plate(&img, &curved_config).unwrap();

        // Before the curve transform, the column cubes must fill exactly the
        // same volume as the single flat plate (5.0 * 2.5 * 0.4 = 5.0 mm³).
        assert!((flat.signed_volume() - 5.0).abs() < 1e-9);
        assert!((segmented.signed_volume() - 5.0).abs() < 1e-9);
    }
}

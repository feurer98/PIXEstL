//! Support plate generation
//!
//! Generates base plate for lithophanes

use crate::error::Result;
use crate::lithophane::config::LithophaneConfig;
use crate::lithophane::geometry::{Mesh, Vector3};
use image::RgbaImage;

/// Generates a flat support plate
pub fn generate_support_plate(image: &RgbaImage, config: &LithophaneConfig) -> Result<Mesh> {
    let (width, height) = image.dimensions();

    let plate_width = width as f64 * config.color_pixel_width;
    let plate_depth = height as f64 * config.color_pixel_width;
    let plate_height = config.plate_thickness;

    let center = Vector3::new(plate_width / 2.0, plate_depth / 2.0, -plate_height / 2.0);

    let mesh = Mesh::cube(plate_width, plate_depth, plate_height, center);

    Ok(mesh)
}

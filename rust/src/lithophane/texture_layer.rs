//! Texture layer mesh generation
//!
//! Generates triangulated surface based on brightness-to-thickness mapping.
//! Based on Java CSGThreadTextureRow class.

use crate::color::Rgb;
use crate::error::Result;
use crate::lithophane::config::LithophaneConfig;
use crate::lithophane::geometry::{Mesh, Triangle, Vector3};
use image::RgbaImage;
use rayon::prelude::*;

/// Calculates pixel height based on brightness
///
/// Uses K (black) component from CMYK conversion
fn get_pixel_height(
    image: &RgbaImage,
    x: u32,
    y: u32,
    min_thickness: f64,
    max_thickness: f64,
) -> f64 {
    let pixel = image.get_pixel(x, y);
    let rgb = Rgb::new(pixel[0], pixel[1], pixel[2]);
    let cmyk = rgb.to_cmyk();

    // K value (darkness) determines thickness
    let k = cmyk.k;
    k * (max_thickness - min_thickness) + min_thickness
}

/// Generates texture layer mesh
///
/// Based on Java CSGThreadTextureRow
pub fn generate_texture_layer(image: &RgbaImage, config: &LithophaneConfig) -> Result<Mesh> {
    let (width, height) = image.dimensions();

    // Process rows in parallel
    let row_meshes: Vec<Mesh> = (0..height - 1)
        .into_par_iter()
        .map(|y| process_texture_row(image, y, width, height, config))
        .collect();

    // Merge all row meshes with pre-allocation
    let total_triangles: usize = row_meshes.iter().map(|m| m.triangle_count()).sum();
    let mut final_mesh = Mesh::with_capacity(total_triangles);
    for row_mesh in row_meshes {
        final_mesh.merge_owned(row_mesh);
    }

    Ok(final_mesh)
}

/// Processes a single row of quads for the texture layer mesh.
///
/// For each pixel quad (2x2 group of adjacent pixels), generates two triangles
/// forming the surface, plus edge wall triangles along the image borders. The Z
/// height of each vertex is determined by the pixel's CMYK K (darkness) value,
/// creating a relief surface where darker pixels are thicker.
fn process_texture_row(
    image: &RgbaImage,
    y: u32,
    width: u32,
    height: u32,
    config: &LithophaneConfig,
) -> Mesh {
    let mut mesh = Mesh::new();
    let pixel_width = config.texture_pixel_width;
    let min_thickness = config.texture_min_thickness;
    let max_thickness = config.texture_max_thickness;

    for x in 0..width - 1 {
        let i = x as f64 * pixel_width;
        let j = y as f64 * pixel_width;
        let i1 = (x + 1) as f64 * pixel_width;
        let j1 = (y + 1) as f64 * pixel_width;

        let h00 = get_pixel_height(image, x, y, min_thickness, max_thickness);
        let h10 = get_pixel_height(image, x + 1, y, min_thickness, max_thickness);
        let h01 = get_pixel_height(image, x, y + 1, min_thickness, max_thickness);
        let h11 = get_pixel_height(image, x + 1, y + 1, min_thickness, max_thickness);

        // Create two triangles for this quad
        let t1 = Triangle::new(
            Vector3::new(i, j, h00),
            Vector3::new(i, j1, h01),
            Vector3::new(i1, j, h10),
        );

        let t2 = Triangle::new(
            Vector3::new(i1, j1, h11),
            Vector3::new(i, j1, h01),
            Vector3::new(i1, j, h10),
        );

        mesh.add_triangle(t1);
        mesh.add_triangle(t2);

        // Add edge triangles for borders
        if x == 0 {
            add_left_edge(&mut mesh, i, j, j1, h00, h01);
        }
        if y == 0 {
            add_top_edge(&mut mesh, i, i1, j, h00, h10);
        }
        if x == width - 2 {
            add_right_edge(&mut mesh, i1, j, j1, h10, h11);
        }
        if y == height - 2 {
            add_bottom_edge(&mut mesh, i, i1, j1, h01, h11);
        }
    }

    mesh
}

fn add_left_edge(mesh: &mut Mesh, i: f64, j: f64, j1: f64, h00: f64, h01: f64) {
    mesh.add_triangle(Triangle::new(
        Vector3::new(i, j, h00),
        Vector3::new(i, j1, h01),
        Vector3::new(i, j1, 0.0),
    ));
    mesh.add_triangle(Triangle::new(
        Vector3::new(i, j, h00),
        Vector3::new(i, j, 0.0),
        Vector3::new(i, j1, 0.0),
    ));
}

fn add_top_edge(mesh: &mut Mesh, i: f64, i1: f64, j: f64, h00: f64, h10: f64) {
    mesh.add_triangle(Triangle::new(
        Vector3::new(i, j, h00),
        Vector3::new(i1, j, h10),
        Vector3::new(i1, j, 0.0),
    ));
    mesh.add_triangle(Triangle::new(
        Vector3::new(i, j, h00),
        Vector3::new(i, j, 0.0),
        Vector3::new(i1, j, 0.0),
    ));
}

fn add_right_edge(mesh: &mut Mesh, i1: f64, j: f64, j1: f64, h10: f64, h11: f64) {
    mesh.add_triangle(Triangle::new(
        Vector3::new(i1, j, h10),
        Vector3::new(i1, j1, h11),
        Vector3::new(i1, j1, 0.0),
    ));
    mesh.add_triangle(Triangle::new(
        Vector3::new(i1, j, h10),
        Vector3::new(i1, j, 0.0),
        Vector3::new(i1, j1, 0.0),
    ));
}

fn add_bottom_edge(mesh: &mut Mesh, i: f64, i1: f64, j1: f64, h01: f64, h11: f64) {
    mesh.add_triangle(Triangle::new(
        Vector3::new(i, j1, h01),
        Vector3::new(i1, j1, h11),
        Vector3::new(i1, j1, 0.0),
    ));
    mesh.add_triangle(Triangle::new(
        Vector3::new(i, j1, h01),
        Vector3::new(i, j1, 0.0),
        Vector3::new(i1, j1, 0.0),
    ));
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use image::{ImageBuffer, Rgba};

    fn create_uniform_image(width: u32, height: u32, color: [u8; 3]) -> RgbaImage {
        ImageBuffer::from_fn(width, height, |_, _| {
            Rgba([color[0], color[1], color[2], 255])
        })
    }

    #[test]
    fn test_get_pixel_height_white() {
        // White pixel: K=0, should return min_thickness
        let image = create_uniform_image(1, 1, [255, 255, 255]);
        let height = get_pixel_height(&image, 0, 0, 0.3, 1.8);
        assert_relative_eq!(height, 0.3, epsilon = 0.01);
    }

    #[test]
    fn test_get_pixel_height_black() {
        // Black pixel: K=1, should return max_thickness
        let image = create_uniform_image(1, 1, [0, 0, 0]);
        let height = get_pixel_height(&image, 0, 0, 0.3, 1.8);
        assert_relative_eq!(height, 1.8, epsilon = 0.01);
    }

    #[test]
    fn test_get_pixel_height_gray() {
        // Mid-gray: K ≈ 0.5, height should be between min and max
        let image = create_uniform_image(1, 1, [128, 128, 128]);
        let height = get_pixel_height(&image, 0, 0, 0.3, 1.8);
        assert!(height > 0.3 && height < 1.8);
    }

    #[test]
    fn test_generate_texture_layer_2x2() {
        // 2x2 image produces 1x1 grid of quads = 2 surface triangles
        // Plus edge triangles: left(2) + top(2) + right(2) + bottom(2) = 8
        // Total = 2 + 8 = 10
        let image = create_uniform_image(2, 2, [128, 128, 128]);
        let config = LithophaneConfig::default();
        let mesh = generate_texture_layer(&image, &config).unwrap();
        assert_eq!(mesh.triangle_count(), 10);
    }

    #[test]
    fn test_generate_texture_layer_3x3() {
        // 3x3 image produces 2x2 grid of quads = 4 quads * 2 triangles = 8 surface triangles
        // Edge triangles: left(2*2) + top(2*2) + right(2*2) + bottom(2*2) = 16
        // Total = 8 + 16 = 24
        let image = create_uniform_image(3, 3, [128, 128, 128]);
        let config = LithophaneConfig::default();
        let mesh = generate_texture_layer(&image, &config).unwrap();
        assert_eq!(mesh.triangle_count(), 24);
    }

    #[test]
    fn test_texture_heights_monotonic_with_darkness() {
        // Darker pixels should produce taller heights
        let light = create_uniform_image(1, 1, [200, 200, 200]);
        let dark = create_uniform_image(1, 1, [50, 50, 50]);

        let h_light = get_pixel_height(&light, 0, 0, 0.3, 1.8);
        let h_dark = get_pixel_height(&dark, 0, 0, 0.3, 1.8);

        assert!(
            h_dark > h_light,
            "Dark ({}) should be taller than light ({})",
            h_dark,
            h_light
        );
    }
}

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

    // Merge all row meshes
    let mut final_mesh = Mesh::new();
    for row_mesh in row_meshes {
        final_mesh.merge(&row_mesh);
    }

    Ok(final_mesh)
}

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

//! Color layer mesh generation
//!
//! Generates 3D meshes for color layers using stacked cubes.
//! Based on Java CSGThreadColorRow class.

use crate::color::Rgb;
use crate::error::Result;
use crate::image::is_pixel_transparent;
use crate::lithophane::config::LithophaneConfig;
use crate::lithophane::geometry::{Mesh, Vector3};
use crate::palette::Palette;
use image::RgbaImage;
use rayon::prelude::*;

/// Checks if a pixel has any transparent neighbors
fn has_transparent_neighbor(image: &RgbaImage, x: u32, y: u32) -> bool {
    let (width, height) = image.dimensions();

    for dy in -1..=1_i32 {
        for dx in -1..=1_i32 {
            if dx == 0 && dy == 0 {
                continue;
            }

            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            if nx < 0 || ny < 0 || nx >= width as i32 || ny >= height as i32 {
                continue;
            }

            let pixel = image.get_pixel(nx as u32, ny as u32);
            if is_pixel_transparent(pixel) {
                return true;
            }
        }
    }

    false
}

/// Generates mesh for a single color layer
///
/// Based on Java CSGThreadColorRow.run()
pub fn generate_color_layer(
    image: &RgbaImage,
    palette: &Palette,
    hex_codes: &[String],
    config: &LithophaneConfig,
    layer_offset: i32,
    layer_max: i32,
) -> Result<Mesh> {
    let (width, height) = image.dimensions();
    let has_transparency = crate::image::has_transparent_pixel(image);

    // Process rows in parallel
    let row_meshes: Vec<Mesh> = (0..height)
        .into_par_iter()
        .map(|y| {
            process_row(
                image,
                palette,
                hex_codes,
                config,
                y,
                width,
                has_transparency,
                layer_offset,
                layer_max,
            )
        })
        .collect();

    // Merge all row meshes
    let mut final_mesh = Mesh::new();
    for row_mesh in row_meshes {
        final_mesh.merge(&row_mesh);
    }

    Ok(final_mesh)
}

/// Processes a single row of pixels
#[allow(clippy::too_many_arguments)]
fn process_row(
    image: &RgbaImage,
    palette: &Palette,
    hex_codes: &[String],
    config: &LithophaneConfig,
    y: u32,
    width: u32,
    has_transparency: bool,
    layer_offset: i32,
    layer_max: i32,
) -> Mesh {
    let mut mesh = Mesh::new();

    for hex_code in hex_codes {
        let mut x = 0;

        while x < width {
            let pixel = image.get_pixel(x, y);
            if is_pixel_transparent(pixel) {
                x += 1;
                continue;
            }

            if has_transparency && has_transparent_neighbor(image, x, y) {
                x += 1;
                continue;
            }

            let pixel_rgb = Rgb::new(pixel[0], pixel[1], pixel[2]);

            // Run-length encoding
            let mut k = 1;
            while x + k < width {
                let next_pixel = image.get_pixel(x + k, y);
                let next_rgb = Rgb::new(next_pixel[0], next_pixel[1], next_pixel[2]);

                if next_rgb != pixel_rgb || has_transparent_neighbor(image, x + k, y) {
                    break;
                }

                k += 1;
            }

            if let Some(color_combi) = palette.get_combi(&pixel_rgb) {
                let layers = color_combi.layers_with_hex(hex_code);

                for (layer_index, layer) in layers.iter().enumerate() {
                    let layer_height = layer.layer();
                    if layer_height == 0 {
                        continue;
                    }

                    let layer_before = color_combi
                        .layer_position(hex_code, layer_index)
                        .unwrap_or(0);

                    let (adjusted_height, adjusted_before) =
                        if layer_offset != -1 && layer_max != -1 {
                            apply_layer_offset(layer_height, layer_before, layer_offset, layer_max)
                        } else {
                            (layer_height, layer_before)
                        };

                    if adjusted_height == 0 {
                        continue;
                    }

                    let pixel_width = config.color_pixel_width;
                    let one_pixel_height_size = config.color_pixel_layer_thickness;
                    let cur_pixel_height = one_pixel_height_size * adjusted_height as f64;
                    let cur_pixel_height_adjust =
                        (cur_pixel_height / 2.0) + (adjusted_before as f64 * one_pixel_height_size);

                    let cube_width = pixel_width * k as f64;
                    let cube_depth = pixel_width;
                    let cube_height = cur_pixel_height;

                    let center_x = (x as f64 * pixel_width) + (cube_width / 2.0);
                    let center_y = y as f64 * pixel_width;
                    let center_z = cur_pixel_height_adjust;

                    let center = Vector3::new(center_x, center_y, center_z);
                    let cube = Mesh::cube(cube_width, cube_depth, cube_height, center);

                    mesh.merge(&cube);
                }
            }

            x += k;
        }
    }

    mesh
}

fn apply_layer_offset(
    layer_height: u32,
    layer_before: usize,
    offset: i32,
    layer_max: i32,
) -> (u32, usize) {
    let offset = offset as usize;
    let layer_max = layer_max as usize;

    if layer_before >= offset + layer_max {
        return (0, 0);
    }

    if layer_before < offset {
        if layer_before + (layer_height as usize) < offset {
            return (0, 0);
        }

        let delta = offset - layer_before;
        let mut new_height = layer_height - delta as u32;
        let new_before = 0;

        if new_height as usize > layer_max {
            new_height = layer_max as u32;
        }

        (new_height, new_before)
    } else {
        let new_before = layer_before.saturating_sub(offset);

        let mut new_height = layer_height;

        if new_height as usize + new_before > layer_max {
            let delta = (new_height as usize + new_before) - layer_max;
            new_height -= delta as u32;
        }

        (new_height, new_before)
    }
}

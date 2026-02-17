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

    // Merge all row meshes with pre-allocation
    let total_triangles: usize = row_meshes.iter().map(|m| m.triangle_count()).sum();
    let mut final_mesh = Mesh::with_capacity(total_triangles);
    for row_mesh in row_meshes {
        final_mesh.merge_owned(row_mesh);
    }

    Ok(final_mesh)
}

/// Processes a single row of pixels to generate cube meshes for color layers.
///
/// For each hex code in the palette, scans the row left-to-right using run-length
/// encoding (RLE) to merge consecutive same-color pixels into wider cubes. For each
/// RLE run, looks up the `ColorCombi` from the palette and generates a cube for each
/// layer of that hex code's contribution.
///
/// Transparent pixels and pixels adjacent to transparent neighbors are skipped
/// to avoid artifacts at transparency boundaries.
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

                if next_rgb != pixel_rgb
                    || (has_transparency && has_transparent_neighbor(image, x + k, y))
                {
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

/// Clips a layer's height and position to fit within a visible window.
///
/// When generating multi-group AMS layers, each STL file only represents a
/// subset of the total layer stack. This function clips a layer that spans
/// `[layer_before, layer_before + layer_height)` to the visible window
/// `[offset, offset + layer_max)`.
///
/// Returns `(0, 0)` if the layer is entirely outside the window.
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

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgba};

    fn create_opaque_image(width: u32, height: u32, color: [u8; 3]) -> RgbaImage {
        ImageBuffer::from_fn(width, height, |_, _| {
            Rgba([color[0], color[1], color[2], 255])
        })
    }

    fn create_image_with_transparent_center(width: u32, height: u32) -> RgbaImage {
        ImageBuffer::from_fn(width, height, |x, y| {
            if x == width / 2 && y == height / 2 {
                Rgba([0, 0, 0, 0]) // transparent center
            } else {
                Rgba([255, 0, 0, 255]) // opaque red
            }
        })
    }

    // --- apply_layer_offset tests ---

    #[test]
    fn test_apply_layer_offset_no_offset() {
        // offset=0, layer_max=5: no clipping
        let (h, b) = apply_layer_offset(3, 0, 0, 5);
        assert_eq!(h, 3);
        assert_eq!(b, 0);
    }

    #[test]
    fn test_apply_layer_offset_entirely_above_range() {
        // Layer starts at position 10, but offset+max = 0+5 = 5
        let (h, _) = apply_layer_offset(3, 10, 0, 5);
        assert_eq!(h, 0); // entirely clipped
    }

    #[test]
    fn test_apply_layer_offset_entirely_below_offset() {
        // Layer at position 0 with height 2, offset=5: layer ends at 2 < 5
        let (h, _) = apply_layer_offset(2, 0, 5, 5);
        assert_eq!(h, 0); // entirely clipped
    }

    #[test]
    fn test_apply_layer_offset_partial_clip_from_bottom() {
        // Layer at position 1 with height 4, offset=3, max=5
        // Layer spans [1, 5), offset window is [3, 8)
        // After clipping: delta = 3-1 = 2, new_height = 4-2 = 2, new_before = 0
        let (h, b) = apply_layer_offset(4, 1, 3, 5);
        assert_eq!(h, 2);
        assert_eq!(b, 0);
    }

    #[test]
    fn test_apply_layer_offset_partial_clip_from_top() {
        // Layer at position 3 with height 5, offset=0, max=5
        // Layer spans [3, 8), window is [0, 5)
        // new_before = 3-0 = 3, new_height = 5 but 5+3=8 > 5, so delta=3, height=2
        let (h, b) = apply_layer_offset(5, 3, 0, 5);
        assert_eq!(h, 2);
        assert_eq!(b, 3);
    }

    #[test]
    fn test_apply_layer_offset_exact_boundary() {
        // Layer at position 0, height 5, offset=0, max=5: exactly fills window
        let (h, b) = apply_layer_offset(5, 0, 0, 5);
        assert_eq!(h, 5);
        assert_eq!(b, 0);
    }

    #[test]
    fn test_apply_layer_offset_clamp_to_max() {
        // Layer at position 0 with height 10, offset=2, max=3
        // layer_before < offset: delta = 2-0 = 2, new_height = 10-2 = 8, clamped to max=3
        let (h, b) = apply_layer_offset(10, 0, 2, 3);
        assert_eq!(h, 3);
        assert_eq!(b, 0);
    }

    // --- has_transparent_neighbor tests ---

    #[test]
    fn test_has_transparent_neighbor_all_opaque() {
        let image = create_opaque_image(3, 3, [255, 0, 0]);
        assert!(!has_transparent_neighbor(&image, 1, 1));
    }

    #[test]
    fn test_has_transparent_neighbor_with_transparent() {
        let image = create_image_with_transparent_center(3, 3);
        // Pixel at (0, 0) has neighbor at (1, 1) which is transparent
        assert!(has_transparent_neighbor(&image, 0, 0));
    }

    #[test]
    fn test_has_transparent_neighbor_corner_pixel() {
        let image = create_opaque_image(3, 3, [255, 0, 0]);
        // Corner pixel (0, 0) in all-opaque image
        assert!(!has_transparent_neighbor(&image, 0, 0));
    }

    #[test]
    fn test_has_transparent_neighbor_edge_pixel() {
        let image = create_opaque_image(3, 3, [255, 0, 0]);
        // Edge pixel (1, 0) in all-opaque image
        assert!(!has_transparent_neighbor(&image, 1, 0));
    }
}

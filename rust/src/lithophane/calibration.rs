//! Calibration test pattern generator
//!
//! Generates a grid of test squares for palette calibration.
//! Each active filament gets one row with squares at increasing layer counts
//! (1 through N layers). When printed, users can photograph these squares
//! and measure HSL values for accurate palette configuration.

use crate::error::Result;
use crate::lithophane::config::LithophaneConfig;
use crate::lithophane::geometry::{Mesh, Vector3};
use crate::palette::loader::PaletteColorEntry;
use std::collections::HashMap;

/// Size of each calibration square in mm
const SQUARE_SIZE: f64 = 10.0;

/// Gap between squares in mm
const SQUARE_GAP: f64 = 2.0;

/// Generates calibration test pattern layers.
///
/// Returns a list of (layer_name, mesh) tuples:
/// - One base plate ("calibration-plate")
/// - One STL per active filament with test squares at each layer count
///
/// Grid layout:
/// ```text
///   1 layer   2 layers  3 layers  ... N layers
///   [  □  ]   [  □  ]   [  □  ]       [  □  ]   ← Filament 1
///   [  □  ]   [  □  ]   [  □  ]       [  □  ]   ← Filament 2
///   ...
/// ```
#[allow(clippy::cast_precision_loss)]
pub fn generate_calibration_pattern(
    palette_data: &HashMap<String, PaletteColorEntry>,
    config: &LithophaneConfig,
) -> Result<Vec<(String, Mesh)>> {
    let mut layers = Vec::new();

    // Collect active filaments sorted by hex code for deterministic output
    let mut active_filaments: Vec<(&String, &PaletteColorEntry)> = palette_data
        .iter()
        .filter(|(_, entry)| entry.active && entry.layers.is_some())
        .collect();
    active_filaments.sort_by_key(|(hex, _)| hex.to_string());

    let nb_layers = config.color_pixel_layer_number;
    let layer_thickness = config.color_pixel_layer_thickness;
    let plate_thickness = config.plate_thickness;
    let num_filaments = active_filaments.len();
    let num_columns = nb_layers as usize;

    // Grid dimensions
    let grid_width =
        num_columns as f64 * SQUARE_SIZE + (num_columns - 1).max(0) as f64 * SQUARE_GAP;
    let grid_depth =
        num_filaments as f64 * SQUARE_SIZE + (num_filaments.saturating_sub(1)) as f64 * SQUARE_GAP;

    // Generate base plate
    let plate_center = Vector3::new(grid_width / 2.0, grid_depth / 2.0, -plate_thickness / 2.0);
    let plate = Mesh::cube(grid_width, grid_depth, plate_thickness, plate_center);
    layers.push(("calibration-plate".to_string(), plate));

    // Generate test squares for each filament
    for (row_idx, (hex_code, entry)) in active_filaments.iter().enumerate() {
        let mut filament_mesh = Mesh::new();
        let row_y = row_idx as f64 * (SQUARE_SIZE + SQUARE_GAP);

        for layer_count in 1..=nb_layers {
            let col_idx = (layer_count - 1) as usize;
            let col_x = col_idx as f64 * (SQUARE_SIZE + SQUARE_GAP);

            let cube_height = layer_count as f64 * layer_thickness;
            let center = Vector3::new(
                col_x + SQUARE_SIZE / 2.0,
                row_y + SQUARE_SIZE / 2.0,
                cube_height / 2.0,
            );

            let cube = Mesh::cube(SQUARE_SIZE, SQUARE_SIZE, cube_height, center);
            filament_mesh.merge(&cube);
        }

        let layer_name = format!("calibration-{}", sanitize_filename(&entry.name, hex_code));
        layers.push((layer_name, filament_mesh));
    }

    Ok(layers)
}

/// Returns the grid dimensions (width_mm, depth_mm) for a calibration pattern.
#[must_use]
#[allow(clippy::cast_precision_loss)]
pub fn calibration_grid_dimensions(num_filaments: usize, nb_layers: u32) -> (f64, f64) {
    let num_columns = nb_layers as usize;
    let width =
        num_columns as f64 * SQUARE_SIZE + (num_columns.saturating_sub(1)) as f64 * SQUARE_GAP;
    let depth =
        num_filaments as f64 * SQUARE_SIZE + (num_filaments.saturating_sub(1)) as f64 * SQUARE_GAP;
    (width, depth)
}

/// Sanitizes a filament name for use as a filename.
fn sanitize_filename(name: &str, hex_code: &str) -> String {
    let sanitized: String = name
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect();

    if sanitized.is_empty() {
        hex_code.replace('#', "")
    } else {
        sanitized
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::palette::loader::PaletteColorEntry;

    fn create_test_palette_data() -> HashMap<String, PaletteColorEntry> {
        use crate::palette::loader::LayerDefinition;

        let mut data = HashMap::new();

        // Red filament with layers
        let mut red_layers = HashMap::new();
        for i in 1..=5 {
            red_layers.insert(
                i.to_string(),
                LayerDefinition::Hsl {
                    h: 0.0,
                    s: 100.0,
                    l: 50.0 + (i as f64 * 5.0),
                },
            );
        }
        data.insert(
            "#FF0000".to_string(),
            PaletteColorEntry {
                name: "Red".to_string(),
                active: true,
                layers: Some(red_layers),
            },
        );

        // White filament
        let mut white_layers = HashMap::new();
        white_layers.insert(
            "5".to_string(),
            LayerDefinition::Hsl {
                h: 0.0,
                s: 0.0,
                l: 100.0,
            },
        );
        data.insert(
            "#FFFFFF".to_string(),
            PaletteColorEntry {
                name: "White".to_string(),
                active: true,
                layers: Some(white_layers),
            },
        );

        // Inactive filament (should be excluded)
        let mut blue_layers = HashMap::new();
        blue_layers.insert(
            "5".to_string(),
            LayerDefinition::Hsl {
                h: 240.0,
                s: 100.0,
                l: 50.0,
            },
        );
        data.insert(
            "#0000FF".to_string(),
            PaletteColorEntry {
                name: "Blue".to_string(),
                active: false,
                layers: Some(blue_layers),
            },
        );

        data
    }

    #[test]
    fn test_generate_calibration_pattern_basic() {
        let palette_data = create_test_palette_data();
        let config = LithophaneConfig::default();

        let layers = generate_calibration_pattern(&palette_data, &config).unwrap();

        // Should have: 1 plate + 2 active filaments = 3 layers
        assert_eq!(layers.len(), 3);
        assert_eq!(layers[0].0, "calibration-plate");
        assert!(layers[0].1.triangle_count() > 0);
    }

    #[test]
    fn test_calibration_pattern_correct_filament_count() {
        let palette_data = create_test_palette_data();
        let config = LithophaneConfig::default();

        let layers = generate_calibration_pattern(&palette_data, &config).unwrap();

        // 2 active filaments (Red, White) + 1 plate = 3
        // Blue is inactive and excluded
        let filament_layers: Vec<_> = layers
            .iter()
            .filter(|(name, _)| name != "calibration-plate")
            .collect();
        assert_eq!(filament_layers.len(), 2);
    }

    #[test]
    fn test_calibration_pattern_squares_per_filament() {
        let palette_data = create_test_palette_data();
        let config = LithophaneConfig {
            color_pixel_layer_number: 5,
            ..LithophaneConfig::default()
        };

        let layers = generate_calibration_pattern(&palette_data, &config).unwrap();

        // Each filament mesh should have 5 cubes (1 per layer count)
        // Each cube has 12 triangles → 5 * 12 = 60 triangles per filament
        for (name, mesh) in &layers {
            if name != "calibration-plate" {
                assert_eq!(
                    mesh.triangle_count(),
                    60,
                    "Filament '{}' should have 60 triangles (5 cubes * 12)",
                    name
                );
            }
        }
    }

    #[test]
    fn test_calibration_plate_has_triangles() {
        let palette_data = create_test_palette_data();
        let config = LithophaneConfig::default();

        let layers = generate_calibration_pattern(&palette_data, &config).unwrap();

        let plate = &layers[0];
        assert_eq!(plate.0, "calibration-plate");
        assert_eq!(plate.1.triangle_count(), 12); // One cube = 12 triangles
    }

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("Red PLA+", "#FF0000"), "Red_PLA_");
        assert_eq!(sanitize_filename("Cyan-V2", "#00FFFF"), "Cyan-V2");
        assert_eq!(sanitize_filename("", "#AABBCC"), "AABBCC");
        assert_eq!(sanitize_filename("White", "#FFFFFF"), "White");
    }

    #[test]
    fn test_calibration_grid_dimensions() {
        let (width, depth) = calibration_grid_dimensions(3, 5);
        // 5 columns: 5 * 10 + 4 * 2 = 58mm
        assert!((width - 58.0).abs() < 0.001);
        // 3 rows: 3 * 10 + 2 * 2 = 34mm
        assert!((depth - 34.0).abs() < 0.001);
    }

    #[test]
    fn test_calibration_single_layer() {
        let mut data = HashMap::new();
        let mut layers = HashMap::new();
        layers.insert(
            "1".to_string(),
            crate::palette::loader::LayerDefinition::Hsl {
                h: 0.0,
                s: 100.0,
                l: 50.0,
            },
        );
        data.insert(
            "#FF0000".to_string(),
            PaletteColorEntry {
                name: "Red".to_string(),
                active: true,
                layers: Some(layers),
            },
        );

        let config = LithophaneConfig {
            color_pixel_layer_number: 1,
            ..LithophaneConfig::default()
        };

        let result = generate_calibration_pattern(&data, &config).unwrap();
        // 1 plate + 1 filament = 2 layers
        assert_eq!(result.len(), 2);
        // 1 cube = 12 triangles
        assert_eq!(result[1].1.triangle_count(), 12);
    }
}

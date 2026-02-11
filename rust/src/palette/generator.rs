//! Recursive ColorCombi generator algorithm
//!
//! This module implements the complex recursive algorithm for generating
//! all possible color combinations from a palette of ColorLayers.
//!
//! Based on Java Palette.createMultiCombi and computeCombination methods

use super::{ColorCombi, ColorLayer};

/// Generates all valid ColorCombi combinations from a list of ColorLayers
///
/// Based on Java Palette.createMultiCombi
///
/// # Algorithm
///
/// 1. For each ColorLayer in the list:
///    - Create a ColorCombi with just that layer
///    - Recursively compute all valid combinations
/// 2. Filter to only combinations with exact layer count
///
/// # Arguments
///
/// * `restrict_colors` - Optional list of hex codes to restrict to
/// * `color_layers` - Available color layers
/// * `nb_layers_target` - Target number of layers (e.g., 5)
///
/// # Returns
///
/// Vector of all valid ColorCombis with exactly `nb_layers_target` layers
pub fn create_multi_combi(
    restrict_colors: Option<&[String]>,
    color_layers: &[ColorLayer],
    nb_layers_target: u32,
) -> Vec<ColorCombi> {
    let mut color_combi_list = Vec::new();

    for (i, color_layer) in color_layers.iter().enumerate() {
        // Skip if not in restriction list
        if let Some(restrict) = restrict_colors {
            if !restrict.contains(&color_layer.hex_code().to_string()) {
                continue;
            }
        }

        // Create base combination
        let base_combi = ColorCombi::new(color_layer.clone());
        color_combi_list.push(base_combi.clone());

        // Recursively compute combinations if more layers available
        if i + 1 < color_layers.len() {
            let recursive_combis =
                compute_combination(restrict_colors, &base_combi, color_layers, nb_layers_target);
            color_combi_list.extend(recursive_combis);
        }
    }

    // Filter to only exact layer count
    color_combi_list
        .into_iter()
        .filter(|c| c.total_layers() == nb_layers_target)
        .collect()
}

/// Recursively computes all valid combinations by adding layers
///
/// Based on Java Palette.computeCombination
///
/// # Arguments
///
/// * `restrict_colors` - Optional list of hex codes to restrict to
/// * `current_combi` - The current ColorCombi being built
/// * `color_layers` - Available color layers to add
/// * `nb_layers_max` - Maximum number of layers allowed
///
/// # Returns
///
/// Vector of all valid ColorCombis that can be formed
fn compute_combination(
    restrict_colors: Option<&[String]>,
    current_combi: &ColorCombi,
    color_layers: &[ColorLayer],
    nb_layers_max: u32,
) -> Vec<ColorCombi> {
    let mut color_combi_list = Vec::new();

    for (i, color_layer) in color_layers.iter().enumerate() {
        // Skip if not in restriction list
        if let Some(restrict) = restrict_colors {
            if !restrict.contains(&color_layer.hex_code().to_string()) {
                continue;
            }
        }

        let layer_count = color_layer.layer();

        // Skip if would exceed max layers
        if current_combi.total_layers() + layer_count > nb_layers_max {
            continue;
        }

        // Skip if would have too many unique colors (optimization)
        // Note: This is a heuristic from the Java code
        if current_combi.total_colors() >= color_layers.len() {
            break;
        }

        // Try to combine
        if let Some(new_combi) =
            current_combi.combine_with_layer(color_layer.clone(), nb_layers_max)
        {
            // Add if exact match
            if new_combi.total_layers() == nb_layers_max {
                color_combi_list.push(new_combi.clone());
            }

            // Recurse if more layers available
            if i + 1 < color_layers.len() {
                let recursive_combis =
                    compute_combination(restrict_colors, &new_combi, color_layers, nb_layers_max);
                color_combi_list.extend(recursive_combis);
            }
        }
    }

    color_combi_list
}

/// Combines multiple ColorCombi lists by pairing each element
///
/// Based on Java Palette.computeColorsByGroup logic
///
/// # Algorithm
///
/// For groups [A1, A2] and [B1, B2]:
/// Result = [A1+B1, A1+B2, A2+B1, A2+B2]
///
/// # Arguments
///
/// * `group1` - First group of ColorCombis
/// * `group2` - Second group of ColorCombis
///
/// # Returns
///
/// Vector of all pairwise combinations
pub fn combine_combi_groups(group1: &[ColorCombi], group2: &[ColorCombi]) -> Vec<ColorCombi> {
    let mut result = Vec::new();

    for combi1 in group1 {
        for combi2 in group2 {
            result.push(combi1.combine_with_combi(combi2));
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_red_layer(layers: u32) -> ColorLayer {
        ColorLayer::from_cmyk("#FF0000".to_string(), layers, 0.0, 1.0, 1.0, 0.0)
    }

    fn create_green_layer(layers: u32) -> ColorLayer {
        ColorLayer::from_cmyk("#00FF00".to_string(), layers, 1.0, 0.0, 1.0, 0.0)
    }

    fn create_blue_layer(layers: u32) -> ColorLayer {
        ColorLayer::from_cmyk("#0000FF".to_string(), layers, 1.0, 1.0, 0.0, 0.0)
    }

    fn create_white_layer(layers: u32) -> ColorLayer {
        ColorLayer::from_cmyk("#FFFFFF".to_string(), layers, 0.0, 0.0, 0.0, 0.0)
    }

    #[test]
    fn test_create_multi_combi_single_layer() {
        let layers = vec![create_red_layer(5)];

        let combis = create_multi_combi(None, &layers, 5);

        assert_eq!(combis.len(), 1);
        assert_eq!(combis[0].total_layers(), 5);
        assert_eq!(combis[0].total_colors(), 1);
    }

    #[test]
    fn test_create_multi_combi_two_layers() {
        let layers = vec![create_red_layer(3), create_green_layer(2)];

        let combis = create_multi_combi(None, &layers, 5);

        // Should generate:
        // - Red[3] + Green[2] = 5 layers ✓
        assert_eq!(combis.len(), 1);
        assert_eq!(combis[0].total_layers(), 5);
        assert_eq!(combis[0].total_colors(), 2);
    }

    #[test]
    fn test_create_multi_combi_multiple_combinations() {
        let layers = vec![
            create_red_layer(1),
            create_green_layer(2),
            create_blue_layer(2),
        ];

        let combis = create_multi_combi(None, &layers, 5);

        // Possible combinations that sum to 5:
        // - Red[1] + Green[2] + Blue[2] = 5 ✓
        // (Note: Red[1]+Red[1]+Red[1]+Red[1]+Red[1] won't work due to duplicate check)

        assert!(combis.len() >= 1);
        assert!(combis.iter().all(|c| c.total_layers() == 5));
    }

    #[test]
    fn test_create_multi_combi_with_restriction() {
        let layers = vec![
            create_red_layer(3),
            create_green_layer(2),
            create_blue_layer(2),
        ];

        let restrict = vec!["#FF0000".to_string(), "#00FF00".to_string()];

        let combis = create_multi_combi(Some(&restrict), &layers, 5);

        // Should only use Red and Green (Blue is restricted)
        assert!(combis.iter().all(|c| {
            c.layers()
                .iter()
                .all(|l| l.hex_code() == "#FF0000" || l.hex_code() == "#00FF00")
        }));
    }

    #[test]
    fn test_create_multi_combi_filters_exact_count() {
        let layers = vec![create_red_layer(3), create_green_layer(1)];

        let combis = create_multi_combi(None, &layers, 5);

        // Should NOT include Red[3] alone or Green[1] alone
        // Should NOT include Red[3]+Green[1] (only 4 layers)
        assert!(combis.iter().all(|c| c.total_layers() == 5));
    }

    #[test]
    fn test_combine_combi_groups() {
        let red = create_red_layer(5);
        let green = create_green_layer(5);

        let group1 = vec![ColorCombi::new(red)];
        let group2 = vec![ColorCombi::new(green)];

        let combined = combine_combi_groups(&group1, &group2);

        assert_eq!(combined.len(), 1); // 1 × 1 = 1
        assert_eq!(combined[0].total_colors(), 2);
        assert_eq!(combined[0].total_layers(), 10); // 5 + 5
    }

    #[test]
    fn test_combine_combi_groups_multiple() {
        let red = create_red_layer(3);
        let green = create_green_layer(2);
        let blue = create_blue_layer(3);
        let white = create_white_layer(2);

        let group1 = vec![ColorCombi::new(red), ColorCombi::new(green)];
        let group2 = vec![ColorCombi::new(blue), ColorCombi::new(white)];

        let combined = combine_combi_groups(&group1, &group2);

        assert_eq!(combined.len(), 4); // 2 × 2 = 4

        // All should have 2 colors
        assert!(combined.iter().all(|c| c.total_colors() == 2));

        // Layer counts: red(3)+blue(3)=6, red(3)+white(2)=5, green(2)+blue(3)=5, green(2)+white(2)=4
        let layer_counts: Vec<u32> = combined.iter().map(|c| c.total_layers()).collect();
        assert!(layer_counts.contains(&6));
        assert!(layer_counts.contains(&5));
        assert!(layer_counts.contains(&4));
    }

    #[test]
    fn test_compute_combination_no_duplicates() {
        let red1 = create_red_layer(2);
        let red2 = create_red_layer(3);

        let layers = vec![red1, red2];
        let base = ColorCombi::new(create_red_layer(1));

        let combis = compute_combination(None, &base, &layers, 5);

        // Should not be able to add red again (duplicate hex code)
        assert_eq!(combis.len(), 0);
    }

    #[test]
    fn test_compute_combination_respects_max_layers() {
        let red = create_red_layer(3);
        let green = create_green_layer(4); // Would exceed max

        let layers = vec![green];
        let base = ColorCombi::new(red);

        let combis = compute_combination(None, &base, &layers, 5);

        // 3 + 4 = 7 > 5, should be empty
        assert_eq!(combis.len(), 0);
    }

    #[test]
    fn test_realistic_palette_scenario() {
        // Realistic scenario: 3 colors, target 5 layers
        let layers = vec![
            create_red_layer(1),
            create_red_layer(2),
            create_green_layer(1),
            create_green_layer(2),
            create_white_layer(1),
            create_white_layer(2),
        ];

        let combis = create_multi_combi(None, &layers, 5);

        // Should generate many valid combinations
        assert!(combis.len() > 0);
        assert!(combis.iter().all(|c| c.total_layers() == 5));

        // Each should have unique color combinations
        for combi in &combis {
            println!("{}", combi);
        }
    }
}

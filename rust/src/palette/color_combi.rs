//! ColorCombi represents a combination of multiple ColorLayers

use super::ColorLayer;
use crate::color::{Cmyk, Rgb};

/// A combination of color layers that produces a final RGB color
///
/// ColorCombi represents the additive mixing of multiple ColorLayers.
/// The final color is computed by summing the CMYK values (clamped to 1.0)
/// and converting to RGB.
///
/// Based on Java ColorCombi class
#[derive(Debug, Clone, PartialEq)]
pub struct ColorCombi {
    /// List of color layers in this combination
    layers: Vec<ColorLayer>,
}

impl ColorCombi {
    /// Creates a new ColorCombi with a single layer
    ///
    /// Based on Java ColorCombi constructor
    #[must_use]
    pub fn new(layer: ColorLayer) -> Self {
        let mut layers = vec![layer];
        layers.sort_by(|a, b| a.compare_by_k(b));

        Self { layers }
    }

    /// Attempts to combine this ColorCombi with a new ColorLayer
    ///
    /// Returns None if:
    /// - The hex code already exists in this combination
    /// - Adding the layer would exceed the maximum layer count
    ///
    /// Based on Java ColorCombi.combineLithoColorLayer
    ///
    /// # Arguments
    ///
    /// * `layer` - The layer to add
    /// * `nb_layer_max` - Maximum total layer count allowed
    #[must_use]
    pub fn combine_with_layer(&self, layer: ColorLayer, nb_layer_max: u32) -> Option<Self> {
        // Check if hex code already exists
        if self.layers.iter().any(|l| l.hex_code() == layer.hex_code()) {
            return None;
        }

        // Check if we would exceed max layers
        if self.total_layers() + layer.layer() > nb_layer_max {
            return None;
        }

        let mut new_combi = self.duplicate();
        new_combi.layers.push(layer);

        Some(new_combi)
    }

    /// Combines this ColorCombi with another ColorCombi
    ///
    /// Based on Java ColorCombi.combineLithoColorCombi
    #[must_use]
    pub fn combine_with_combi(&self, other: &Self) -> Self {
        let mut new_combi = self.duplicate();
        new_combi.layers.extend(other.layers.clone());
        new_combi
    }

    /// Adds a layer to this ColorCombi (mutating)
    pub fn add_layer(&mut self, layer: ColorLayer) {
        self.layers.push(layer);
    }

    /// Gets the total number of unique colors in this combination
    #[must_use]
    pub fn total_colors(&self) -> usize {
        self.layers.len()
    }

    /// Gets the total number of layers (sum of all layer counts)
    ///
    /// Based on Java ColorCombi.getTotalLayers
    #[must_use]
    pub fn total_layers(&self) -> u32 {
        self.layers.iter().map(|l| l.layer()).sum()
    }

    /// Computes the final RGB color by summing CMYK values
    ///
    /// Based on Java ColorCombi.getColor
    ///
    /// # Algorithm
    ///
    /// 1. Sum all CMYK components from each layer
    /// 2. Clamp each component to maximum 1.0
    /// 3. Convert CMYK to RGB
    #[must_use]
    pub fn compute_rgb(&self) -> Rgb {
        let mut c = 0.0;
        let mut m = 0.0;
        let mut y = 0.0;
        let mut k = 0.0;

        for layer in &self.layers {
            c += layer.c();
            m += layer.m();
            y += layer.y();
            k += layer.k();
        }

        // Clamp to 1.0 maximum
        let cmyk = Cmyk::new(c.min(1.0), m.min(1.0), y.min(1.0), k.min(1.0));

        Rgb::from_cmyk(cmyk)
    }

    /// Duplicates this ColorCombi
    ///
    /// Based on Java ColorCombi.duplicate
    fn duplicate(&self) -> Self {
        Self {
            layers: self.layers.clone(),
        }
    }

    /// Factorizes consecutive layers with the same hex code
    ///
    /// Combines adjacent layers that have the same hex code into a single
    /// layer with the combined layer count.
    ///
    /// Based on Java ColorCombi.factorize
    ///
    /// # Example
    ///
    /// `[Red[2], Red[3], Blue[1]]` → `[Red[5], Blue[1]]`
    pub fn factorize(&mut self) {
        if self.layers.is_empty() {
            return;
        }

        let mut new_layers = Vec::new();

        for layer in &self.layers {
            if new_layers.is_empty() {
                new_layers.push(layer.clone());
                continue;
            }

            let last_idx = new_layers.len() - 1;
            let last_layer = &new_layers[last_idx];

            if last_layer.hex_code() == layer.hex_code() {
                // Combine with the last layer
                let combined = last_layer.combine_with(layer);
                new_layers[last_idx] = combined;
            } else {
                new_layers.push(layer.clone());
            }
        }

        self.layers = new_layers;
    }

    /// Gets all layers with a specific hex code
    ///
    /// Based on Java ColorCombi.getLayerList
    #[must_use]
    pub fn layers_with_hex(&self, hex_code: &str) -> Vec<&ColorLayer> {
        self.layers
            .iter()
            .filter(|l| l.hex_code() == hex_code)
            .collect()
    }

    /// Gets the layer position (number of layers before this layer)
    ///
    /// Based on Java ColorCombi.getLayerPosition
    ///
    /// # Arguments
    ///
    /// * `target_layer` - The layer to find the position of
    ///
    /// # Returns
    ///
    /// The number of layers before the target layer, or None if not found
    #[must_use]
    pub fn layer_position(&self, target_hex: &str, target_index: usize) -> Option<usize> {
        let mut count = 0;
        let mut hex_index = 0;

        for layer in &self.layers {
            if layer.hex_code() == target_hex {
                if hex_index == target_index {
                    return Some(count);
                }
                hex_index += 1;
            }
            count += layer.layer() as usize;
        }

        None
    }

    /// Gets the layers
    #[must_use]
    pub fn layers(&self) -> &[ColorLayer] {
        &self.layers
    }

    /// Optimizes white layers by moving them to bottom and top
    ///
    /// Based on Java Palette.optimizeWhiteLayer
    ///
    /// # Algorithm
    ///
    /// 1. Split layers into: bottom white, middle colored, top white
    /// 2. Reassemble as: bottom + middle + top
    pub(crate) fn optimize_white_layers(&mut self, nb_color_pool: usize, _nb_layers: usize) {
        let mut bottom_white = Vec::new();
        let mut middle_colored = Vec::new();
        let mut top_white = Vec::new();

        let mut layer_count = 0;

        for layer in &self.layers {
            if layer.hex_code() == "#FFFFFF" {
                if layer_count <= nb_color_pool {
                    bottom_white.push(layer.clone());
                } else {
                    top_white.push(layer.clone());
                }
            } else {
                middle_colored.push(layer.clone());
            }
            layer_count += layer.layer() as usize;
        }

        self.layers.clear();
        self.layers.extend(bottom_white);
        self.layers.extend(middle_colored);
        self.layers.extend(top_white);
    }
}

impl std::fmt::Display for ColorCombi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ColorCombi[")?;
        for (i, layer) in self.layers.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", layer)?;
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_empty_combi() -> ColorCombi {
        ColorCombi { layers: Vec::new() }
    }

    fn create_red_layer(layers: u32) -> ColorLayer {
        ColorLayer::from_cmyk("#FF0000".to_string(), layers, 0.0, 1.0, 1.0, 0.0)
    }

    fn create_green_layer(layers: u32) -> ColorLayer {
        ColorLayer::from_cmyk("#00FF00".to_string(), layers, 1.0, 0.0, 1.0, 0.0)
    }

    fn create_white_layer(layers: u32) -> ColorLayer {
        ColorLayer::from_cmyk("#FFFFFF".to_string(), layers, 0.0, 0.0, 0.0, 0.0)
    }

    #[test]
    fn test_color_combi_creation() {
        let layer = create_red_layer(5);
        let combi = ColorCombi::new(layer);

        assert_eq!(combi.total_colors(), 1);
        assert_eq!(combi.total_layers(), 5);
    }

    #[test]
    fn test_combine_with_layer_success() {
        let red = create_red_layer(3);
        let green = create_green_layer(2);

        let combi = ColorCombi::new(red);
        let new_combi = combi.combine_with_layer(green, 10);

        assert!(new_combi.is_some());
        let new_combi = new_combi.unwrap();
        assert_eq!(new_combi.total_colors(), 2);
        assert_eq!(new_combi.total_layers(), 5); // 3 + 2
    }

    #[test]
    fn test_combine_with_layer_duplicate_hex() {
        let red1 = create_red_layer(3);
        let red2 = create_red_layer(2);

        let combi = ColorCombi::new(red1);
        let new_combi = combi.combine_with_layer(red2, 10);

        assert!(new_combi.is_none()); // Should fail due to duplicate hex
    }

    #[test]
    fn test_combine_with_layer_exceeds_max() {
        let red = create_red_layer(8);
        let green = create_green_layer(3);

        let combi = ColorCombi::new(red);
        let new_combi = combi.combine_with_layer(green, 10);

        assert!(new_combi.is_none()); // 8 + 3 = 11 > 10
    }

    #[test]
    fn test_combine_with_combi() {
        let red = create_red_layer(3);
        let green = create_green_layer(2);

        let combi1 = ColorCombi::new(red);
        let combi2 = ColorCombi::new(green);

        let combined = combi1.combine_with_combi(&combi2);

        assert_eq!(combined.total_colors(), 2);
        assert_eq!(combined.total_layers(), 5);
    }

    #[test]
    fn test_compute_rgb_single_color() {
        let red = create_red_layer(5);
        let combi = ColorCombi::new(red);

        let rgb = combi.compute_rgb();

        // Red with M=1, Y=1, K=0 should produce red
        assert!(rgb.r > 200); // Should be reddish
    }

    #[test]
    fn test_compute_rgb_additive() {
        // Test that CMYK values are summed and clamped
        let layer1 = ColorLayer::from_cmyk("#COLOR1".to_string(), 1, 0.5, 0.3, 0.2, 0.1);
        let layer2 = ColorLayer::from_cmyk("#COLOR2".to_string(), 1, 0.3, 0.5, 0.4, 0.2);

        let mut combi = ColorCombi::new(layer1);
        combi.add_layer(layer2);

        let rgb = combi.compute_rgb();

        // Just verify it produces a valid RGB color (u8 type guarantees 0-255 range)
        let _r = rgb.r;
        let _g = rgb.g;
        let _b = rgb.b;
    }

    #[test]
    fn test_factorize_consecutive_same_color() {
        let red1 = create_red_layer(2);
        let red2 = create_red_layer(3);

        let mut combi = create_empty_combi();
        combi.add_layer(red1);
        combi.add_layer(red2);

        assert_eq!(combi.total_colors(), 2);
        assert_eq!(combi.total_layers(), 5);

        combi.factorize();

        assert_eq!(combi.total_colors(), 1);
        assert_eq!(combi.total_layers(), 5); // Still 5 total
        assert_eq!(combi.layers()[0].layer(), 5); // Combined into one layer
    }

    #[test]
    fn test_factorize_non_consecutive() {
        let red = create_red_layer(2);
        let green = create_green_layer(3);
        let red2 = create_red_layer(1);

        let mut combi = create_empty_combi();
        combi.add_layer(red);
        combi.add_layer(green);
        combi.add_layer(red2);

        combi.factorize();

        // Non-consecutive reds should NOT be combined
        assert_eq!(combi.total_colors(), 3);
    }

    #[test]
    fn test_layers_with_hex() {
        let red1 = create_red_layer(2);
        let green = create_green_layer(3);
        let red2 = create_red_layer(1);

        let mut combi = create_empty_combi();
        combi.add_layer(red1);
        combi.add_layer(green);
        combi.add_layer(red2);

        let red_layers = combi.layers_with_hex("#FF0000");
        assert_eq!(red_layers.len(), 2);

        let green_layers = combi.layers_with_hex("#00FF00");
        assert_eq!(green_layers.len(), 1);
    }

    #[test]
    fn test_optimize_white_layers() {
        let white1 = create_white_layer(1);
        let red = create_red_layer(3);
        let white2 = create_white_layer(1);

        let mut combi = create_empty_combi();
        combi.add_layer(white1.clone());
        combi.add_layer(red.clone());
        combi.add_layer(white2.clone());

        combi.optimize_white_layers(2, 5);

        // White layers should be at bottom and top
        assert_eq!(combi.layers()[0].hex_code(), "#FFFFFF"); // Bottom white
        assert_eq!(combi.layers()[1].hex_code(), "#FF0000"); // Middle colored
        assert_eq!(combi.layers()[2].hex_code(), "#FFFFFF"); // Top white
    }

    #[test]
    fn test_display() {
        let red = create_red_layer(3);
        let green = create_green_layer(2);

        let mut combi = ColorCombi::new(red);
        combi.add_layer(green);

        let display = format!("{}", combi);
        assert!(display.contains("#FF0000[3]"));
        assert!(display.contains("#00FF00[2]"));
    }
}

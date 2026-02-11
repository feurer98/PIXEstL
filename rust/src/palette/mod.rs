//! Palette management and color quantization
//!
//! This module provides functionality for:
//! - Loading color palettes from JSON files
//! - Combining colors additively using CMYK color space
//! - Quantizing images to palette colors
//! - Managing AMS (Automatic Material System) color groups

pub mod color_combi;
pub mod color_layer;
pub mod generator;
pub mod loader;
pub mod quantize;

pub use color_combi::ColorCombi;
pub use color_layer::ColorLayer;
pub use generator::{combine_combi_groups, create_multi_combi};
pub use loader::PaletteLoader;

use crate::color::{find_closest_color, ColorDistanceMethod, Rgb};
use crate::error::Result;
use std::collections::HashMap;

/// A palette of colors with combinations for lithophane generation
#[derive(Debug, Clone)]
pub struct Palette {
    /// Map from RGB color to its ColorCombi representation
    quantized_colors: HashMap<Rgb, ColorCombi>,

    /// Map from hex code to color name
    hex_codes_map: HashMap<String, String>,

    /// Number of layers per color pixel
    nb_layers: u32,

    /// Number of AMS groups (for multi-color printing)
    nb_groups: usize,

    /// Total layer count (nb_layers * nb_groups)
    layer_count: usize,

    /// Color groups for AMS (each group contains hex codes)
    hex_color_group_list: Vec<Vec<String>>,
}

impl Palette {
    /// Creates a new empty palette
    pub fn new(nb_layers: u32) -> Self {
        Self {
            quantized_colors: HashMap::new(),
            hex_codes_map: HashMap::new(),
            nb_layers,
            nb_groups: 0,
            layer_count: 0,
            hex_color_group_list: Vec::new(),
        }
    }

    /// Gets the number of colors in the palette
    pub fn color_count(&self) -> usize {
        self.quantized_colors.len()
    }

    /// Gets all available RGB colors
    pub fn colors(&self) -> Vec<Rgb> {
        self.quantized_colors.keys().copied().collect()
    }

    /// Gets the ColorCombi for a specific RGB color
    pub fn get_combi(&self, color: &Rgb) -> Option<&ColorCombi> {
        self.quantized_colors.get(color)
    }

    /// Gets the color name for a hex code
    pub fn get_color_name(&self, hex_code: &str) -> Option<&str> {
        self.hex_codes_map.get(hex_code).map(|s| s.as_str())
    }

    /// Gets all hex codes in the palette
    pub fn hex_codes(&self) -> Vec<String> {
        self.hex_codes_map.keys().cloned().collect()
    }

    /// Gets the number of AMS groups
    pub fn nb_groups(&self) -> usize {
        self.nb_groups
    }

    /// Gets the total layer count
    pub fn layer_count(&self) -> usize {
        self.layer_count
    }

    /// Gets the hex color groups for AMS
    pub fn hex_color_groups(&self) -> &[Vec<String>] {
        &self.hex_color_group_list
    }

    /// Finds the closest palette color to the given RGB color
    pub fn find_closest(&self, color: &Rgb, method: ColorDistanceMethod) -> Option<Rgb> {
        let colors = self.colors();
        if colors.is_empty() {
            return None;
        }
        Some(find_closest_color(color, &colors, method))
    }

    /// Adds a color combination to the palette
    pub(crate) fn add_combi(&mut self, combi: ColorCombi) {
        let color = combi.compute_rgb();
        self.quantized_colors.insert(color, combi);
    }

    /// Sets the hex codes map
    pub(crate) fn set_hex_codes(&mut self, map: HashMap<String, String>) {
        self.hex_codes_map = map;
    }

    /// Sets the number of groups
    pub(crate) fn set_nb_groups(&mut self, nb_groups: usize) {
        self.nb_groups = nb_groups;
        self.layer_count = self.nb_layers as usize * nb_groups;
    }

    /// Sets the hex color groups
    pub(crate) fn set_hex_color_groups(&mut self, groups: Vec<Vec<String>>) {
        self.hex_color_group_list = groups;
    }

    /// Optimizes white layers (moves white to bottom and top)
    pub(crate) fn optimize_white_layers(&mut self, nb_color_pool: usize) {
        for combi in self.quantized_colors.values_mut() {
            combi.optimize_white_layers(nb_color_pool, self.nb_layers as usize);
        }
    }

    /// Factorizes all color combinations
    pub(crate) fn factorize_all(&mut self) {
        for combi in self.quantized_colors.values_mut() {
            combi.factorize();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palette_creation() {
        let palette = Palette::new(5);
        assert_eq!(palette.color_count(), 0);
        assert_eq!(palette.nb_groups(), 0);
        assert_eq!(palette.layer_count(), 0);
    }

    #[test]
    fn test_palette_add_combi() {
        let mut palette = Palette::new(5);

        let layer = ColorLayer::new("#FF0000".to_string(), 5, 0.0, 1.0, 0.5);
        let combi = ColorCombi::new(layer);

        palette.add_combi(combi);
        assert_eq!(palette.color_count(), 1);
    }

    #[test]
    fn test_palette_find_closest() {
        let mut palette = Palette::new(5);

        let red_layer = ColorLayer::new("#FF0000".to_string(), 5, 0.0, 100.0, 50.0);
        palette.add_combi(ColorCombi::new(red_layer));

        let target = Rgb::new(250, 10, 10); // Close to red
        let closest = palette.find_closest(&target, ColorDistanceMethod::Rgb);

        assert!(closest.is_some());
    }

    #[test]
    fn test_palette_hex_codes() {
        let mut palette = Palette::new(5);
        let mut map = HashMap::new();
        map.insert("#FF0000".to_string(), "Red".to_string());
        map.insert("#00FF00".to_string(), "Green".to_string());

        palette.set_hex_codes(map);

        assert_eq!(palette.get_color_name("#FF0000"), Some("Red"));
        assert_eq!(palette.get_color_name("#00FF00"), Some("Green"));
        assert_eq!(palette.get_color_name("#0000FF"), None);
    }

    #[test]
    fn test_palette_groups() {
        let mut palette = Palette::new(5);
        palette.set_nb_groups(3);

        assert_eq!(palette.nb_groups(), 3);
        assert_eq!(palette.layer_count(), 15); // 5 * 3
    }
}

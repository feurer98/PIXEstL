//! ColorLayer represents a single layer of a filament color in CMYK color space

use crate::color::{Cmyk, Hsl};
use std::cmp::Ordering;

/// A single color layer with CMYK values
///
/// Represents one layer of filament with a specific color and thickness.
/// The layer count indicates how many physical layers this represents.
///
/// Based on Java ColorLayer class
#[derive(Debug, Clone, PartialEq)]
pub struct ColorLayer {
    /// Hex code of the color (e.g., "#FF0000")
    hex_code: String,

    /// Number of layers (thickness)
    layer: u32,

    /// CMYK color components (0.0-1.0)
    cmyk: Cmyk,
}

impl ColorLayer {
    /// Creates a new ColorLayer from hex code and HSL values
    ///
    /// Based on Java ColorLayer constructor with HSL
    ///
    /// # Arguments
    ///
    /// * `hex_code` - Hex color code (e.g., "#FF0000")
    /// * `layer` - Number of layers (thickness)
    /// * `h` - Hue in degrees (0.0-360.0)
    /// * `s` - Saturation in percent (0.0-100.0)
    /// * `l` - Lightness in percent (0.0-100.0)
    ///
    /// # Example
    ///
    /// ```
    /// use pixestl::palette::ColorLayer;
    ///
    /// let red_layer = ColorLayer::new("#FF0000".to_string(), 5, 0.0, 100.0, 50.0);
    /// assert_eq!(red_layer.layer(), 5);
    /// ```
    #[must_use]
    pub fn new(hex_code: String, layer: u32, h: f64, s: f64, l: f64) -> Self {
        let hsl = Hsl::new(h, s, l);
        let cmyk = hsl.to_cmyk();

        Self {
            hex_code,
            layer,
            cmyk,
        }
    }

    /// Creates a ColorLayer from hex code and CMYK values directly
    ///
    /// Based on Java ColorLayer constructor with CMYK
    #[must_use]
    pub fn from_cmyk(hex_code: String, layer: u32, c: f64, m: f64, y: f64, k: f64) -> Self {
        Self {
            hex_code,
            layer,
            cmyk: Cmyk::new(c, m, y, k),
        }
    }

    /// Gets the hex code
    #[must_use]
    pub fn hex_code(&self) -> &str {
        &self.hex_code
    }

    /// Gets the number of layers
    #[must_use]
    pub fn layer(&self) -> u32 {
        self.layer
    }

    /// Gets the CMYK values
    #[must_use]
    pub fn cmyk(&self) -> &Cmyk {
        &self.cmyk
    }

    /// Gets the Cyan component
    #[must_use]
    pub fn c(&self) -> f64 {
        self.cmyk.c
    }

    /// Gets the Magenta component
    #[must_use]
    pub fn m(&self) -> f64 {
        self.cmyk.m
    }

    /// Gets the Yellow component
    #[must_use]
    pub fn y(&self) -> f64 {
        self.cmyk.y
    }

    /// Gets the Key (Black) component
    #[must_use]
    pub fn k(&self) -> f64 {
        self.cmyk.k
    }

    /// Compares two ColorLayers by K value (descending order)
    ///
    /// Based on Java ColorLayer.LayerComparator
    /// Sorts by K value in descending order (darker colors first)
    #[must_use]
    pub fn compare_by_k(&self, other: &Self) -> Ordering {
        // Reverse comparison for descending order
        other.k().partial_cmp(&self.k()).unwrap_or(Ordering::Equal)
    }

    /// Combines this layer with another by incrementing the layer count
    ///
    /// Returns a new ColorLayer with the combined layer count.
    /// The hex_code and CMYK values must be identical.
    ///
    /// # Panics
    ///
    /// Panics if the hex codes don't match
    #[must_use]
    pub fn combine_with(&self, other: &Self) -> Self {
        assert_eq!(
            self.hex_code, other.hex_code,
            "Cannot combine layers with different hex codes"
        );
        assert_eq!(
            self.cmyk, other.cmyk,
            "Cannot combine layers with different CMYK values"
        );

        Self {
            hex_code: self.hex_code.clone(),
            layer: self.layer + other.layer,
            cmyk: self.cmyk,
        }
    }
}

impl std::fmt::Display for ColorLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}[{}]", self.hex_code, self.layer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_color_layer_creation_from_hsl() {
        let layer = ColorLayer::new("#FF0000".to_string(), 5, 0.0, 100.0, 50.0);

        assert_eq!(layer.hex_code(), "#FF0000");
        assert_eq!(layer.layer(), 5);

        // Red should have M=1, Y=1, K=0
        assert_relative_eq!(layer.m(), 1.0, epsilon = 0.01);
        assert_relative_eq!(layer.y(), 1.0, epsilon = 0.01);
        assert_relative_eq!(layer.k(), 0.0, epsilon = 0.01);
    }

    #[test]
    fn test_color_layer_from_cmyk() {
        let layer = ColorLayer::from_cmyk("#00FF00".to_string(), 3, 0.5, 0.0, 0.5, 0.2);

        assert_eq!(layer.hex_code(), "#00FF00");
        assert_eq!(layer.layer(), 3);
        assert_relative_eq!(layer.c(), 0.5, epsilon = 1e-10);
        assert_relative_eq!(layer.m(), 0.0, epsilon = 1e-10);
        assert_relative_eq!(layer.y(), 0.5, epsilon = 1e-10);
        assert_relative_eq!(layer.k(), 0.2, epsilon = 1e-10);
    }

    #[test]
    fn test_color_layer_white() {
        // White should have C=0, M=0, Y=0, K=0
        let white = ColorLayer::new("#FFFFFF".to_string(), 5, 0.0, 0.0, 100.0);

        assert_relative_eq!(white.c(), 0.0, epsilon = 0.01);
        assert_relative_eq!(white.m(), 0.0, epsilon = 0.01);
        assert_relative_eq!(white.y(), 0.0, epsilon = 0.01);
        assert_relative_eq!(white.k(), 0.0, epsilon = 0.01);
    }

    #[test]
    fn test_color_layer_black() {
        // Black should have K=1
        let black = ColorLayer::new("#000000".to_string(), 5, 0.0, 0.0, 0.0);

        assert_relative_eq!(black.k(), 1.0, epsilon = 0.01);
    }

    #[test]
    fn test_compare_by_k_descending() {
        let darker = ColorLayer::from_cmyk("#000000".to_string(), 1, 0.0, 0.0, 0.0, 0.8);
        let lighter = ColorLayer::from_cmyk("#AAAAAA".to_string(), 1, 0.0, 0.0, 0.0, 0.2);

        // Darker (higher K) should come before lighter (lower K)
        assert_eq!(darker.compare_by_k(&lighter), Ordering::Less);
        assert_eq!(lighter.compare_by_k(&darker), Ordering::Greater);
    }

    #[test]
    fn test_compare_by_k_equal() {
        let layer1 = ColorLayer::from_cmyk("#FF0000".to_string(), 1, 0.0, 1.0, 1.0, 0.0);
        let layer2 = ColorLayer::from_cmyk("#00FF00".to_string(), 1, 1.0, 0.0, 1.0, 0.0);

        // Same K value should be equal
        assert_eq!(layer1.compare_by_k(&layer2), Ordering::Equal);
    }

    #[test]
    fn test_combine_with() {
        let layer1 = ColorLayer::from_cmyk("#FF0000".to_string(), 2, 0.0, 1.0, 1.0, 0.0);
        let layer2 = ColorLayer::from_cmyk("#FF0000".to_string(), 3, 0.0, 1.0, 1.0, 0.0);

        let combined = layer1.combine_with(&layer2);

        assert_eq!(combined.hex_code(), "#FF0000");
        assert_eq!(combined.layer(), 5); // 2 + 3
        assert_relative_eq!(combined.m(), 1.0, epsilon = 1e-10);
    }

    #[test]
    #[should_panic(expected = "Cannot combine layers with different hex codes")]
    fn test_combine_with_different_hex_codes() {
        let layer1 = ColorLayer::from_cmyk("#FF0000".to_string(), 2, 0.0, 1.0, 1.0, 0.0);
        let layer2 = ColorLayer::from_cmyk("#00FF00".to_string(), 3, 1.0, 0.0, 1.0, 0.0);

        let _ = layer1.combine_with(&layer2);
    }

    #[test]
    fn test_display() {
        let layer = ColorLayer::new("#FF0000".to_string(), 5, 0.0, 100.0, 50.0);
        assert_eq!(format!("{}", layer), "#FF0000[5]");
    }

    #[test]
    fn test_sorting_by_k() {
        let mut layers = [
            ColorLayer::from_cmyk("#AAAAAA".to_string(), 1, 0.0, 0.0, 0.0, 0.2), // Light
            ColorLayer::from_cmyk("#000000".to_string(), 1, 0.0, 0.0, 0.0, 0.9), // Dark
            ColorLayer::from_cmyk("#666666".to_string(), 1, 0.0, 0.0, 0.0, 0.5), // Medium
        ];

        layers.sort_by(|a, b| a.compare_by_k(b));

        // Should be sorted dark to light (descending K)
        assert!(layers[0].k() >= layers[1].k());
        assert!(layers[1].k() >= layers[2].k());
    }
}

//! RGB color space representation and conversions

use crate::color::Cmyk;
use crate::error::{PixestlError, Result};
use std::fmt;

/// RGB color with 8-bit per channel (0-255)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    /// Creates a new RGB color
    ///
    /// # Example
    ///
    /// ```
    /// use pixestl::color::Rgb;
    ///
    /// let red = Rgb::new(255, 0, 0);
    /// assert_eq!(red.r, 255);
    /// ```
    #[must_use]
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Creates an RGB color from a hex string (e.g., "#FF0000" or "#ff0000")
    ///
    /// # Example
    ///
    /// ```
    /// use pixestl::color::Rgb;
    ///
    /// let red = Rgb::from_hex("#FF0000").unwrap();
    /// assert_eq!(red, Rgb::new(255, 0, 0));
    /// ```
    pub fn from_hex(hex: &str) -> Result<Self> {
        let hex = hex.trim();

        if !hex.starts_with('#') || hex.len() != 7 {
            return Err(PixestlError::InvalidHexCode(hex.to_string()));
        }

        let r = u8::from_str_radix(&hex[1..3], 16)
            .map_err(|_| PixestlError::InvalidHexCode(hex.to_string()))?;
        let g = u8::from_str_radix(&hex[3..5], 16)
            .map_err(|_| PixestlError::InvalidHexCode(hex.to_string()))?;
        let b = u8::from_str_radix(&hex[5..7], 16)
            .map_err(|_| PixestlError::InvalidHexCode(hex.to_string()))?;

        Ok(Self::new(r, g, b))
    }

    /// Converts RGB to hex string (e.g., "#FF0000")
    ///
    /// # Example
    ///
    /// ```
    /// use pixestl::color::Rgb;
    ///
    /// let red = Rgb::new(255, 0, 0);
    /// assert_eq!(red.to_hex(), "#FF0000");
    /// ```
    #[must_use]
    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    /// Converts RGB to normalized floating point values (0.0-1.0)
    #[must_use]
    pub fn to_f64(&self) -> (f64, f64, f64) {
        (
            f64::from(self.r) / 255.0,
            f64::from(self.g) / 255.0,
            f64::from(self.b) / 255.0,
        )
    }

    /// Creates RGB from normalized floating point values (0.0-1.0)
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn from_f64(r: f64, g: f64, b: f64) -> Self {
        Self::new(
            (r * 255.0).round().clamp(0.0, 255.0) as u8,
            (g * 255.0).round().clamp(0.0, 255.0) as u8,
            (b * 255.0).round().clamp(0.0, 255.0) as u8,
        )
    }

    /// Converts RGB to CMYK color space
    ///
    /// Based on Java ColorUtil.colorToCMYK implementation
    #[must_use]
    pub fn to_cmyk(&self) -> Cmyk {
        let (r, g, b) = self.to_f64();

        let k = 1.0 - r.max(g).max(b);

        let (c, m, y) = if k < 1.0 {
            let c = (1.0 - r - k) / (1.0 - k);
            let m = (1.0 - g - k) / (1.0 - k);
            let y = (1.0 - b - k) / (1.0 - k);
            (c, m, y)
        } else {
            (0.0, 0.0, 0.0)
        };

        Cmyk::new(c, m, y, k)
    }

    /// Creates RGB from CMYK color space
    ///
    /// Based on Java ColorUtil.cmykToColor implementation
    #[must_use]
    pub fn from_cmyk(cmyk: Cmyk) -> Self {
        let cmyk = cmyk.clamp();

        let r = (1.0 - cmyk.c) * (1.0 - cmyk.k);
        let g = (1.0 - cmyk.m) * (1.0 - cmyk.k);
        let b = (1.0 - cmyk.y) * (1.0 - cmyk.k);

        Self::from_f64(r, g, b)
    }
}

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RGB({}, {}, {})", self.r, self.g, self.b)
    }
}

impl From<(u8, u8, u8)> for Rgb {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self::new(r, g, b)
    }
}

impl From<Rgb> for (u8, u8, u8) {
    fn from(rgb: Rgb) -> Self {
        (rgb.r, rgb.g, rgb.b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_rgb_creation() {
        let color = Rgb::new(255, 128, 64);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);
    }

    #[test]
    fn test_from_hex_valid() {
        let red = Rgb::from_hex("#FF0000").unwrap();
        assert_eq!(red, Rgb::new(255, 0, 0));

        let green = Rgb::from_hex("#00FF00").unwrap();
        assert_eq!(green, Rgb::new(0, 255, 0));

        let blue = Rgb::from_hex("#0000FF").unwrap();
        assert_eq!(blue, Rgb::new(0, 0, 255));

        // Lowercase should work too
        let white = Rgb::from_hex("#ffffff").unwrap();
        assert_eq!(white, Rgb::new(255, 255, 255));
    }

    #[test]
    fn test_from_hex_invalid() {
        assert!(Rgb::from_hex("FF0000").is_err()); // Missing #
        assert!(Rgb::from_hex("#FF00").is_err()); // Too short
        assert!(Rgb::from_hex("#FF00000").is_err()); // Too long
        assert!(Rgb::from_hex("#GGGGGG").is_err()); // Invalid hex
    }

    #[test]
    fn test_to_hex() {
        let red = Rgb::new(255, 0, 0);
        assert_eq!(red.to_hex(), "#FF0000");

        let color = Rgb::new(171, 205, 239);
        assert_eq!(color.to_hex(), "#ABCDEF");
    }

    #[test]
    fn test_hex_roundtrip() {
        let original = Rgb::new(123, 45, 67);
        let hex = original.to_hex();
        let restored = Rgb::from_hex(&hex).unwrap();
        assert_eq!(original, restored);
    }

    #[test]
    fn test_to_f64() {
        let color = Rgb::new(255, 128, 0);
        let (r, g, b) = color.to_f64();
        assert_relative_eq!(r, 1.0, epsilon = 1e-10);
        assert_relative_eq!(g, 0.502, epsilon = 0.001);
        assert_relative_eq!(b, 0.0, epsilon = 1e-10);
    }

    #[test]
    fn test_from_f64() {
        let color = Rgb::from_f64(1.0, 0.5, 0.0);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 0);
    }

    #[test]
    fn test_f64_roundtrip() {
        let original = Rgb::new(255, 128, 64);
        let (r, g, b) = original.to_f64();
        let restored = Rgb::from_f64(r, g, b);
        // Allow small rounding error
        assert!((original.r as i16 - restored.r as i16).abs() <= 1);
        assert!((original.g as i16 - restored.g as i16).abs() <= 1);
        assert!((original.b as i16 - restored.b as i16).abs() <= 1);
    }

    #[test]
    fn test_rgb_to_cmyk_white() {
        let white = Rgb::new(255, 255, 255);
        let cmyk = white.to_cmyk();
        assert_relative_eq!(cmyk.c, 0.0, epsilon = 1e-10);
        assert_relative_eq!(cmyk.m, 0.0, epsilon = 1e-10);
        assert_relative_eq!(cmyk.y, 0.0, epsilon = 1e-10);
        assert_relative_eq!(cmyk.k, 0.0, epsilon = 1e-10);
    }

    #[test]
    fn test_rgb_to_cmyk_black() {
        let black = Rgb::new(0, 0, 0);
        let cmyk = black.to_cmyk();
        assert_relative_eq!(cmyk.k, 1.0, epsilon = 1e-10);
    }

    #[test]
    fn test_rgb_to_cmyk_red() {
        let red = Rgb::new(255, 0, 0);
        let cmyk = red.to_cmyk();
        assert_relative_eq!(cmyk.c, 0.0, epsilon = 1e-10);
        assert_relative_eq!(cmyk.m, 1.0, epsilon = 1e-10);
        assert_relative_eq!(cmyk.y, 1.0, epsilon = 1e-10);
        assert_relative_eq!(cmyk.k, 0.0, epsilon = 1e-10);
    }

    #[test]
    fn test_cmyk_to_rgb_white() {
        let cmyk = Cmyk::new(0.0, 0.0, 0.0, 0.0);
        let rgb = Rgb::from_cmyk(cmyk);
        assert_eq!(rgb, Rgb::new(255, 255, 255));
    }

    #[test]
    fn test_cmyk_to_rgb_black() {
        let cmyk = Cmyk::new(0.0, 0.0, 0.0, 1.0);
        let rgb = Rgb::from_cmyk(cmyk);
        assert_eq!(rgb, Rgb::new(0, 0, 0));
    }

    #[test]
    fn test_cmyk_roundtrip() {
        let original = Rgb::new(128, 64, 200);
        let cmyk = original.to_cmyk();
        let restored = Rgb::from_cmyk(cmyk);
        // Allow small rounding errors
        assert!((original.r as i16 - restored.r as i16).abs() <= 1);
        assert!((original.g as i16 - restored.g as i16).abs() <= 1);
        assert!((original.b as i16 - restored.b as i16).abs() <= 1);
    }

    #[test]
    fn test_display() {
        let color = Rgb::new(255, 128, 64);
        assert_eq!(format!("{}", color), "RGB(255, 128, 64)");
    }

    #[test]
    fn test_tuple_conversions() {
        let tuple = (255, 128, 64);
        let rgb: Rgb = tuple.into();
        assert_eq!(rgb, Rgb::new(255, 128, 64));

        let back: (u8, u8, u8) = rgb.into();
        assert_eq!(back, tuple);
    }
}

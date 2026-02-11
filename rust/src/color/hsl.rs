//! HSL (Hue, Saturation, Lightness) color space representation

use crate::color::{Cmyk, Rgb};
use std::fmt;

/// HSL color representation
///
/// - H (Hue): 0.0-360.0 degrees
/// - S (Saturation): 0.0-100.0 percent
/// - L (Lightness): 0.0-100.0 percent
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hsl {
    /// Hue in degrees (0.0-360.0)
    pub h: f64,
    /// Saturation in percent (0.0-100.0)
    pub s: f64,
    /// Lightness in percent (0.0-100.0)
    pub l: f64,
}

impl Hsl {
    /// Creates a new HSL color
    ///
    /// # Example
    ///
    /// ```
    /// use pixestl::color::Hsl;
    ///
    /// let red = Hsl::new(0.0, 100.0, 50.0);
    /// ```
    pub fn new(h: f64, s: f64, l: f64) -> Self {
        Self { h, s, l }
    }

    /// Converts HSL to RGB
    ///
    /// Based on Java ColorUtil.hslToCmyk and hueToRgb implementation
    pub fn to_rgb(&self) -> Rgb {
        let s = self.s / 100.0;
        let l = self.l / 100.0;

        if s == 0.0 {
            // Achromatic (gray)
            return Rgb::from_f64(l, l, l);
        }

        let q = if l < 0.5 {
            l * (1.0 + s)
        } else {
            l + s - l * s
        };

        let p = 2.0 * l - q;
        let hk = self.h / 360.0;

        let r = hue_to_rgb(p, q, hk + 1.0 / 3.0);
        let g = hue_to_rgb(p, q, hk);
        let b = hue_to_rgb(p, q, hk - 1.0 / 3.0);

        Rgb::from_f64(r, g, b)
    }

    /// Converts HSL to CMYK
    ///
    /// Based on Java ColorUtil.hslToCmyk implementation
    pub fn to_cmyk(&self) -> Cmyk {
        let s = self.s / 100.0;
        let l = self.l / 100.0;

        if s == 0.0 {
            // Achromatic (gray)
            let k = 1.0 - l;
            return Cmyk::new(0.0, 0.0, 0.0, k);
        }

        let q = if l < 0.5 {
            l * (1.0 + s)
        } else {
            l + s - l * s
        };

        let p = 2.0 * l - q;
        let hk = self.h / 360.0;

        let r = hue_to_rgb(p, q, hk + 1.0 / 3.0);
        let g = hue_to_rgb(p, q, hk);
        let b = hue_to_rgb(p, q, hk - 1.0 / 3.0);

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
}

impl From<Rgb> for Hsl {
    /// Converts RGB to HSL
    ///
    /// Based on Java ColorUtil.colorToHSL implementation
    fn from(rgb: Rgb) -> Self {
        let (r, g, b) = rgb.to_f64();

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);

        let l = (max + min) / 2.0;

        let s = if max == min {
            0.0
        } else {
            let delta = max - min;
            delta / (1.0 - (2.0 * l - 1.0).abs())
        };

        let h = if max == min {
            0.0
        } else if max == r {
            (60.0 * ((g - b) / (max - min)) + 360.0) % 360.0
        } else if max == g {
            60.0 * ((b - r) / (max - min)) + 120.0
        } else {
            60.0 * ((r - g) / (max - min)) + 240.0
        };

        Hsl::new(h, s * 100.0, l * 100.0)
    }
}

impl fmt::Display for Hsl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HSL({:.1}°, {:.1}%, {:.1}%)", self.h, self.s, self.l)
    }
}

/// Helper function for HSL to RGB conversion
///
/// Based on Java ColorUtil.hueToRgb implementation
fn hue_to_rgb(p: f64, q: f64, t: f64) -> f64 {
    let mut t = t;
    if t < 0.0 {
        t += 1.0;
    }
    if t > 1.0 {
        t -= 1.0;
    }

    if t < 1.0 / 6.0 {
        p + (q - p) * 6.0 * t
    } else if t < 1.0 / 2.0 {
        q
    } else if t < 2.0 / 3.0 {
        p + (q - p) * (2.0 / 3.0 - t) * 6.0
    } else {
        p
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_hsl_creation() {
        let hsl = Hsl::new(120.0, 50.0, 75.0);
        assert_eq!(hsl.h, 120.0);
        assert_eq!(hsl.s, 50.0);
        assert_eq!(hsl.l, 75.0);
    }

    #[test]
    fn test_rgb_to_hsl_red() {
        let red = Rgb::new(255, 0, 0);
        let hsl = Hsl::from(red);
        assert_relative_eq!(hsl.h, 0.0, epsilon = 0.1);
        assert_relative_eq!(hsl.s, 100.0, epsilon = 0.1);
        assert_relative_eq!(hsl.l, 50.0, epsilon = 0.1);
    }

    #[test]
    fn test_rgb_to_hsl_green() {
        let green = Rgb::new(0, 255, 0);
        let hsl = Hsl::from(green);
        assert_relative_eq!(hsl.h, 120.0, epsilon = 0.1);
        assert_relative_eq!(hsl.s, 100.0, epsilon = 0.1);
        assert_relative_eq!(hsl.l, 50.0, epsilon = 0.1);
    }

    #[test]
    fn test_rgb_to_hsl_blue() {
        let blue = Rgb::new(0, 0, 255);
        let hsl = Hsl::from(blue);
        assert_relative_eq!(hsl.h, 240.0, epsilon = 0.1);
        assert_relative_eq!(hsl.s, 100.0, epsilon = 0.1);
        assert_relative_eq!(hsl.l, 50.0, epsilon = 0.1);
    }

    #[test]
    fn test_rgb_to_hsl_white() {
        let white = Rgb::new(255, 255, 255);
        let hsl = Hsl::from(white);
        assert_relative_eq!(hsl.s, 0.0, epsilon = 0.1);
        assert_relative_eq!(hsl.l, 100.0, epsilon = 0.1);
    }

    #[test]
    fn test_rgb_to_hsl_black() {
        let black = Rgb::new(0, 0, 0);
        let hsl = Hsl::from(black);
        assert_relative_eq!(hsl.s, 0.0, epsilon = 0.1);
        assert_relative_eq!(hsl.l, 0.0, epsilon = 0.1);
    }

    #[test]
    fn test_rgb_to_hsl_gray() {
        let gray = Rgb::new(128, 128, 128);
        let hsl = Hsl::from(gray);
        assert_relative_eq!(hsl.s, 0.0, epsilon = 0.1);
        assert_relative_eq!(hsl.l, 50.2, epsilon = 0.5); // ~50%
    }

    #[test]
    fn test_hsl_to_rgb_red() {
        let hsl = Hsl::new(0.0, 100.0, 50.0);
        let rgb = hsl.to_rgb();
        assert_eq!(rgb.r, 255);
        assert_eq!(rgb.g, 0);
        assert_eq!(rgb.b, 0);
    }

    #[test]
    fn test_hsl_to_rgb_green() {
        let hsl = Hsl::new(120.0, 100.0, 50.0);
        let rgb = hsl.to_rgb();
        assert_eq!(rgb.r, 0);
        assert_eq!(rgb.g, 255);
        assert_eq!(rgb.b, 0);
    }

    #[test]
    fn test_hsl_to_rgb_blue() {
        let hsl = Hsl::new(240.0, 100.0, 50.0);
        let rgb = hsl.to_rgb();
        assert_eq!(rgb.r, 0);
        assert_eq!(rgb.g, 0);
        assert_eq!(rgb.b, 255);
    }

    #[test]
    fn test_hsl_to_rgb_white() {
        let hsl = Hsl::new(0.0, 0.0, 100.0);
        let rgb = hsl.to_rgb();
        assert_eq!(rgb, Rgb::new(255, 255, 255));
    }

    #[test]
    fn test_hsl_to_rgb_black() {
        let hsl = Hsl::new(0.0, 0.0, 0.0);
        let rgb = hsl.to_rgb();
        assert_eq!(rgb, Rgb::new(0, 0, 0));
    }

    #[test]
    fn test_hsl_to_rgb_gray() {
        let hsl = Hsl::new(0.0, 0.0, 50.0);
        let rgb = hsl.to_rgb();
        // Should be approximately gray
        assert!((rgb.r as i16 - 128).abs() <= 2);
        assert!((rgb.g as i16 - 128).abs() <= 2);
        assert!((rgb.b as i16 - 128).abs() <= 2);
    }

    #[test]
    fn test_hsl_roundtrip() {
        let original = Rgb::new(200, 100, 50);
        let hsl = Hsl::from(original);
        let restored = hsl.to_rgb();

        // Allow small rounding errors (within 2 due to floating point)
        assert!((original.r as i16 - restored.r as i16).abs() <= 2);
        assert!((original.g as i16 - restored.g as i16).abs() <= 2);
        assert!((original.b as i16 - restored.b as i16).abs() <= 2);
    }

    #[test]
    fn test_hsl_to_cmyk_cyan() {
        let hsl = Hsl::new(180.0, 100.0, 50.0);
        let cmyk = hsl.to_cmyk();

        // Cyan should have C=1, M=0, Y=0, K=0
        assert_relative_eq!(cmyk.c, 1.0, epsilon = 0.01);
        assert_relative_eq!(cmyk.m, 0.0, epsilon = 0.01);
        assert_relative_eq!(cmyk.y, 0.0, epsilon = 0.01);
        assert_relative_eq!(cmyk.k, 0.0, epsilon = 0.01);
    }

    #[test]
    fn test_hsl_to_cmyk_gray() {
        let hsl = Hsl::new(0.0, 0.0, 50.0);
        let cmyk = hsl.to_cmyk();

        assert_relative_eq!(cmyk.c, 0.0, epsilon = 0.01);
        assert_relative_eq!(cmyk.m, 0.0, epsilon = 0.01);
        assert_relative_eq!(cmyk.y, 0.0, epsilon = 0.01);
        assert_relative_eq!(cmyk.k, 0.5, epsilon = 0.01);
    }

    #[test]
    fn test_hue_wraparound() {
        // Hue should wrap around at 360 degrees
        let hsl1 = Hsl::new(0.0, 100.0, 50.0);
        let hsl2 = Hsl::new(360.0, 100.0, 50.0);

        let rgb1 = hsl1.to_rgb();
        let rgb2 = hsl2.to_rgb();

        assert_eq!(rgb1, rgb2);
    }

    #[test]
    fn test_display() {
        let hsl = Hsl::new(180.5, 75.3, 50.1);
        assert_eq!(format!("{}", hsl), "HSL(180.5°, 75.3%, 50.1%)");
    }
}

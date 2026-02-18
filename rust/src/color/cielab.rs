//! CIELab-Farbraum – wahrnehmungsgleichmäßige Farbdarstellung
//!
//! ## Was ist CIELab?
//!
//! CIELab (auch CIELAB oder L\*a\*b\* genannt) ist ein Farbraum, der so gestaltet ist,
//! dass gleiche euklidische Abstände zwischen Farben annähernd gleichen Wahrnehmungsunterschieden
//! entsprechen. Das heißt: Ein Delta E von 1.0 bedeutet überall im Farbraum ungefähr denselben
//! wahrnehmbaren Farbunterschied.
//!
//! ## Umrechnungs-Pipeline: RGB → XYZ → CIELab
//!
//! **Schritt 1: RGB → Lineares RGB (Gamma-Korrektur entfernen)**
//!
//! sRGB-Werte (0–255) werden zunächst auf `[0,1]` normiert, dann linearisiert:
//! - Wenn `n > 0.04045`:  `n_linear = ((n + 0.055) / 1.055)^2.4`
//! - Sonst:               `n_linear = n / 12.92`
//!
//! **Schritt 2: Lineares RGB → XYZ (D65-Beleuchtung)**
//!
//! Multiplikation mit der ICC-Standardmatrix für D65:
//! ```text
//! X = 0.412456 * R + 0.357576 * G + 0.180438 * B
//! Y = 0.212673 * R + 0.715152 * G + 0.072175 * B
//! Z = 0.019334 * R + 0.119192 * G + 0.950304 * B
//! ```
//! Der Weißpunkt D65 entspricht Tageslicht (X=95.047, Y=100.0, Z=108.883).
//!
//! **Schritt 3: XYZ → L\*a\*b\***
//!
//! Nach Division durch den Weißpunkt wird die nichtlineare Funktion f(t) angewendet:
//! - Wenn `t > ε = (6/29)³ ≈ 0.008856`:  `f(t) = t^(1/3)` (Kubikwurzel)
//! - Sonst: `f(t) = t/3 * (29/6)² + 4/29`
//!
//! Daraus folgen die Lab-Koordinaten:
//! ```text
//! L* = 116 * f(Y/Yn) - 16        (Helligkeit: 0=Schwarz, 100=Weiß)
//! a* = 500 * (f(X/Xn) - f(Y/Yn)) (Grün–Rot-Achse)
//! b* = 200 * (f(Y/Yn) - f(Z/Zn)) (Blau–Gelb-Achse)
//! ```
//!
//! ## Delta E (CIE76)
//!
//! Der Farbabstand wird als euklidischer Abstand im Lab-Raum berechnet:
//! `ΔE = √(ΔL² + Δa² + Δb²)`
//!
//! Faustregel: ΔE < 2.3 gilt als für Menschen kaum wahrnehmbar.

use crate::color::Rgb;
use std::fmt;

/// D65 standard illuminant reference white point (X component)
const D65_X: f64 = 95.047;
/// D65 standard illuminant reference white point (Y component)
const D65_Y: f64 = 100.000;
/// D65 standard illuminant reference white point (Z component)
const D65_Z: f64 = 108.883;

/// sRGB companding threshold for linearization
const SRGB_THRESHOLD: f64 = 0.04045;

/// CIELab linearization epsilon: (6/29)^3
const LAB_EPSILON: f64 = 0.008_856;

/// CIELab linearization kappa: 1 / (3 * (6/29)^2)
const LAB_KAPPA: f64 = 7.787_037; // 1.0 / (3.0 * (6.0/29.0)^2)

/// CIELab color representation
///
/// - L: Lightness (0.0-100.0)
/// - a: Green-Red axis (typically -128 to +128)
/// - b: Blue-Yellow axis (typically -128 to +128)
///
/// CIELab is a perceptually uniform color space, meaning that the Euclidean
/// distance between two colors corresponds approximately to human perception
/// of color difference.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CieLab {
    pub l: f64,
    pub a: f64,
    pub b: f64,
}

impl CieLab {
    /// Creates a new CIELab color
    #[must_use]
    pub fn new(l: f64, a: f64, b: f64) -> Self {
        Self { l, a, b }
    }

    /// Calculates the CIE76 Delta E color difference
    ///
    /// Returns the Euclidean distance in the CIELab color space.
    /// Values less than 2.3 are generally considered to be visually
    /// indistinguishable.
    ///
    /// Based on Java ColorUtil.deltaE implementation
    ///
    /// # Example
    ///
    /// ```
    /// use pixestl::color::{CieLab, Rgb};
    ///
    /// let red = CieLab::from(Rgb::new(255, 0, 0));
    /// let orange = CieLab::from(Rgb::new(255, 128, 0));
    /// let distance = red.delta_e(&orange);
    /// assert!(distance > 0.0);
    /// ```
    #[must_use]
    pub fn delta_e(&self, other: &CieLab) -> f64 {
        let dl = self.l - other.l;
        let da = self.a - other.a;
        let db = self.b - other.b;

        (dl * dl + da * da + db * db).sqrt()
    }
}

impl From<Rgb> for CieLab {
    /// Converts RGB to CIELab
    ///
    /// Conversion pipeline: RGB → XYZ (D65) → CIELab
    /// Based on Java ColorUtil.rgbToLab implementation
    fn from(rgb: Rgb) -> Self {
        let xyz = rgb_to_xyz(rgb);
        xyz_to_lab(xyz)
    }
}

impl fmt::Display for CieLab {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CIELab(L={:.2}, a={:.2}, b={:.2})",
            self.l, self.a, self.b
        )
    }
}

/// XYZ color representation (intermediate for RGB → CIELab)
#[derive(Debug, Clone, Copy)]
struct Xyz {
    x: f64,
    y: f64,
    z: f64,
}

/// Converts RGB to XYZ color space using D65 illuminant
///
/// Based on Java ColorUtil.rgbToXyz implementation
#[allow(clippy::many_single_char_names)]
fn rgb_to_xyz(rgb: Rgb) -> Xyz {
    let (r, g, b) = rgb.to_f64();

    // Apply gamma correction (sRGB → linear RGB)
    let r_linear = pivot_rgb_to_xyz(r);
    let g_linear = pivot_rgb_to_xyz(g);
    let b_linear = pivot_rgb_to_xyz(b);

    // Convert to XYZ using D65 illuminant transformation matrix
    let x = r_linear * 0.412_456_4 + g_linear * 0.357_576_1 + b_linear * 0.180_437_5;
    let y = r_linear * 0.212_672_9 + g_linear * 0.715_152_2 + b_linear * 0.072_175_0;
    let z = r_linear * 0.019_333_9 + g_linear * 0.119_192_0 + b_linear * 0.950_304_1;

    Xyz {
        x: x * 100.0,
        y: y * 100.0,
        z: z * 100.0,
    }
}

/// Gamma correction for RGB → XYZ conversion
///
/// Based on Java ColorUtil.pivotRgbToXyz implementation
fn pivot_rgb_to_xyz(n: f64) -> f64 {
    if n > SRGB_THRESHOLD {
        ((n + 0.055) / 1.055).powf(2.4)
    } else {
        n / 12.92
    }
}

/// Converts XYZ to CIELab color space
///
/// Uses D65 reference white point:
/// - X_n = 95.047
/// - Y_n = 100.000
/// - Z_n = 108.883
///
/// Based on Java ColorUtil.xyzToLab implementation
#[allow(clippy::many_single_char_names)]
fn xyz_to_lab(xyz: Xyz) -> CieLab {
    let x = xyz.x / D65_X;
    let y = xyz.y / D65_Y;
    let z = xyz.z / D65_Z;

    // Apply Lab transformation function
    let fx = if x > 0.0 { pivot_xyz_to_lab(x) } else { 0.0 };
    let fy = if y > 0.0 { pivot_xyz_to_lab(y) } else { 0.0 };
    let fz = if z > 0.0 { pivot_xyz_to_lab(z) } else { 0.0 };

    let l = (116.0 * fy - 16.0).max(0.0);
    let a = (fx - fy) * 500.0;
    let b = (fy - fz) * 200.0;

    CieLab::new(l, a, b)
}

/// Lab transformation function for XYZ → CIELab conversion
///
/// Based on Java ColorUtil.pivotXyzToLab implementation
fn pivot_xyz_to_lab(n: f64) -> f64 {
    if n > LAB_EPSILON {
        n.cbrt()
    } else {
        n * LAB_KAPPA + 4.0 / 29.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_cielab_creation() {
        let lab = CieLab::new(50.0, 25.0, -50.0);
        assert_eq!(lab.l, 50.0);
        assert_eq!(lab.a, 25.0);
        assert_eq!(lab.b, -50.0);
    }

    #[test]
    fn test_rgb_to_cielab_white() {
        let white = Rgb::new(255, 255, 255);
        let lab = CieLab::from(white);

        assert_relative_eq!(lab.l, 100.0, epsilon = 0.1);
        assert_relative_eq!(lab.a, 0.0, epsilon = 0.5);
        assert_relative_eq!(lab.b, 0.0, epsilon = 0.5);
    }

    #[test]
    fn test_rgb_to_cielab_black() {
        let black = Rgb::new(0, 0, 0);
        let lab = CieLab::from(black);

        assert_relative_eq!(lab.l, 0.0, epsilon = 0.1);
        // a and b can be small non-zero values for black
    }

    #[test]
    fn test_rgb_to_cielab_red() {
        let red = Rgb::new(255, 0, 0);
        let lab = CieLab::from(red);

        // Red should have high L (around 53) and high positive a
        assert!(lab.l > 50.0 && lab.l < 55.0);
        assert!(lab.a > 75.0); // Red-green axis, positive for red
    }

    #[test]
    fn test_rgb_to_cielab_green() {
        let green = Rgb::new(0, 255, 0);
        let lab = CieLab::from(green);

        // Green should have high L and negative a
        assert!(lab.l > 85.0 && lab.l < 90.0);
        assert!(lab.a < -75.0); // Red-green axis, negative for green
    }

    #[test]
    fn test_rgb_to_cielab_blue() {
        let blue = Rgb::new(0, 0, 255);
        let lab = CieLab::from(blue);

        // Blue should have moderate L and large negative b
        assert!(lab.l > 30.0 && lab.l < 35.0);
        assert!(lab.b < -100.0); // Blue-yellow axis, negative for blue
    }

    #[test]
    fn test_delta_e_same_color() {
        let color = CieLab::from(Rgb::new(128, 64, 200));
        let distance = color.delta_e(&color);
        assert_relative_eq!(distance, 0.0, epsilon = 1e-10);
    }

    #[test]
    fn test_delta_e_symmetry() {
        let red = CieLab::from(Rgb::new(255, 0, 0));
        let green = CieLab::from(Rgb::new(0, 255, 0));

        let d1 = red.delta_e(&green);
        let d2 = green.delta_e(&red);

        assert_relative_eq!(d1, d2, epsilon = 1e-10);
    }

    #[test]
    fn test_delta_e_non_negative() {
        let color1 = CieLab::from(Rgb::new(100, 150, 200));
        let color2 = CieLab::from(Rgb::new(110, 140, 210));

        let distance = color1.delta_e(&color2);
        assert!(distance >= 0.0);
    }

    #[test]
    fn test_delta_e_different_colors() {
        let red = CieLab::from(Rgb::new(255, 0, 0));
        let green = CieLab::from(Rgb::new(0, 255, 0));

        let distance = red.delta_e(&green);

        // Red and green are very different, Delta E should be large
        assert!(distance > 80.0);
    }

    #[test]
    fn test_delta_e_similar_colors() {
        let color1 = CieLab::from(Rgb::new(100, 100, 100));
        let color2 = CieLab::from(Rgb::new(101, 101, 101));

        let distance = color1.delta_e(&color2);

        // Very similar colors should have small Delta E
        assert!(distance < 2.0);
    }

    #[test]
    fn test_delta_e_white_black() {
        let white = CieLab::from(Rgb::new(255, 255, 255));
        let black = CieLab::from(Rgb::new(0, 0, 0));

        let distance = white.delta_e(&black);

        // Maximum lightness difference
        assert!(distance > 99.0);
    }

    #[test]
    fn test_display() {
        let lab = CieLab::new(53.24, 80.09, 67.20);
        let display = format!("{}", lab);
        assert!(display.contains("53.24"));
        assert!(display.contains("80.09"));
        assert!(display.contains("67.20"));
    }

    // Property-based tests
    #[test]
    fn test_delta_e_triangle_inequality() {
        // For any three colors a, b, c:
        // delta_e(a, c) <= delta_e(a, b) + delta_e(b, c)
        let a = CieLab::from(Rgb::new(255, 0, 0));
        let b = CieLab::from(Rgb::new(0, 255, 0));
        let c = CieLab::from(Rgb::new(0, 0, 255));

        let d_ac = a.delta_e(&c);
        let d_ab = a.delta_e(&b);
        let d_bc = b.delta_e(&c);

        assert!(d_ac <= d_ab + d_bc + 1e-10); // Small epsilon for floating point
    }

    #[test]
    fn test_cielab_gray_scale() {
        // Grayscale colors should have a≈0 and b≈0
        for gray in [0, 32, 64, 128, 192, 255] {
            let rgb = Rgb::new(gray, gray, gray);
            let lab = CieLab::from(rgb);

            assert!(lab.a.abs() < 1.0, "Gray {} has a={}", gray, lab.a);
            assert!(lab.b.abs() < 1.0, "Gray {} has b={}", gray, lab.b);
        }
    }
}

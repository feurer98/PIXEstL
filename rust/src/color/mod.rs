//! Color space representations and conversions
//!
//! This module provides types and functions for working with different color spaces:
//! - RGB: Standard 8-bit per channel representation
//! - HSL: Hue, Saturation, Lightness
//! - CIELab: Perceptually uniform color space
//! - CMYK: Cyan, Magenta, Yellow, Key (Black) for printing

pub mod cielab;
pub mod distance;
pub mod hsl;
pub mod rgb;

pub use cielab::CieLab;
pub use distance::{
    find_closest_color, find_closest_color_precomputed, ColorDistance, ColorDistanceMethod,
};
pub use hsl::Hsl;
pub use rgb::Rgb;

/// CMYK-Farbdarstellung für Druckfarben (Werte im Bereich 0.0–1.0)
///
/// CMYK steht für Cyan, Magenta, Yellow (Gelb) und Key (Schwarz).
/// Dieses Modell wird im PIXEstL-Projekt genutzt, um die optische Mischung
/// von Filamenten zu berechnen, wenn mehrere Farben übereinandergedruckt werden.
///
/// Jeder Kanal liegt im Bereich `[0.0, 1.0]`, wobei `0.0` keinen und `1.0` maximalen
/// Farbanteil bedeutet.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cmyk {
    /// Cyan-Anteil (0.0 = kein Cyan, 1.0 = volles Cyan)
    pub c: f64,
    /// Magenta-Anteil (0.0 = kein Magenta, 1.0 = volles Magenta)
    pub m: f64,
    /// Gelb-Anteil (0.0 = kein Gelb, 1.0 = volles Gelb)
    pub y: f64,
    /// Schwarz-Anteil / Key (0.0 = kein Schwarz, 1.0 = volles Schwarz)
    pub k: f64,
}

impl Cmyk {
    /// Erstellt eine neue CMYK-Farbe.
    ///
    /// # Arguments
    ///
    /// * `c` - Cyan-Anteil (0.0–1.0)
    /// * `m` - Magenta-Anteil (0.0–1.0)
    /// * `y` - Gelb-Anteil (0.0–1.0)
    /// * `k` - Schwarz-Anteil (0.0–1.0)
    ///
    /// # Example
    ///
    /// ```
    /// use pixestl::color::Cmyk;
    /// let cyan = Cmyk::new(1.0, 0.0, 0.0, 0.0);
    /// ```
    #[must_use]
    pub fn new(c: f64, m: f64, y: f64, k: f64) -> Self {
        Self { c, m, y, k }
    }

    /// Begrenzt alle Kanalwerte auf den gültigen Bereich \[0.0, 1.0\].
    ///
    /// # Returns
    ///
    /// Eine neue `Cmyk`-Farbe mit geklemmten Werten.
    ///
    /// # Example
    ///
    /// ```
    /// use pixestl::color::Cmyk;
    /// let out_of_range = Cmyk::new(-0.5, 1.5, 0.3, 0.0);
    /// let clamped = out_of_range.clamp();
    /// assert_eq!(clamped.c, 0.0);
    /// assert_eq!(clamped.m, 1.0);
    /// ```
    #[must_use]
    pub fn clamp(&self) -> Self {
        Self {
            c: self.c.clamp(0.0, 1.0),
            m: self.m.clamp(0.0, 1.0),
            y: self.y.clamp(0.0, 1.0),
            k: self.k.clamp(0.0, 1.0),
        }
    }
}

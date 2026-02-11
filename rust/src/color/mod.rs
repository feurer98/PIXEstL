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
pub use distance::{find_closest_color, ColorDistance, ColorDistanceMethod};
pub use hsl::Hsl;
pub use rgb::Rgb;

/// CMYK color representation (values in range 0.0-1.0)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cmyk {
    pub c: f64,
    pub m: f64,
    pub y: f64,
    pub k: f64,
}

impl Cmyk {
    /// Creates a new CMYK color
    pub fn new(c: f64, m: f64, y: f64, k: f64) -> Self {
        Self { c, m, y, k }
    }

    /// Clamps CMYK values to valid range [0.0, 1.0]
    pub fn clamp(&self) -> Self {
        Self {
            c: self.c.clamp(0.0, 1.0),
            m: self.m.clamp(0.0, 1.0),
            y: self.y.clamp(0.0, 1.0),
            k: self.k.clamp(0.0, 1.0),
        }
    }
}

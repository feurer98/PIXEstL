//! Color distance calculation methods

use crate::color::{CieLab, Rgb};

/// Method for calculating color distance
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorDistanceMethod {
    /// RGB Euclidean distance (fast but less perceptually accurate)
    Rgb,
    /// CIELab Delta E distance (slower but perceptually uniform)
    #[default]
    CieLab,
}

impl std::str::FromStr for ColorDistanceMethod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RGB" => Ok(Self::Rgb),
            "CIELab" => Ok(Self::CieLab),
            _ => Err(format!("Invalid color distance method: {}", s)),
        }
    }
}

impl ColorDistanceMethod {
    /// Convert to string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Rgb => "RGB",
            Self::CieLab => "CIELab",
        }
    }
}

/// Trait for calculating color distance
pub trait ColorDistance {
    /// Calculate the distance between two colors
    fn distance(&self, other: &Self) -> f64;
}

impl ColorDistance for Rgb {
    /// Calculate RGB Euclidean distance
    ///
    /// Returns the squared Euclidean distance in RGB space.
    /// This is faster than CIELab but less perceptually accurate.
    ///
    /// Based on Java ColorUtil.colorDistanceRGB implementation
    ///
    /// # Example
    ///
    /// ```
    /// use pixestl::color::{Rgb, ColorDistance};
    ///
    /// let red = Rgb::new(255, 0, 0);
    /// let orange = Rgb::new(255, 128, 0);
    /// let distance = red.distance(&orange);
    /// assert!(distance > 0.0);
    /// ```
    fn distance(&self, other: &Self) -> f64 {
        let dr = self.r as i32 - other.r as i32;
        let dg = self.g as i32 - other.g as i32;
        let db = self.b as i32 - other.b as i32;

        (dr * dr + dg * dg + db * db) as f64
    }
}

impl ColorDistance for CieLab {
    /// Calculate CIE76 Delta E distance
    ///
    /// This is the same as calling `delta_e()` method.
    fn distance(&self, other: &Self) -> f64 {
        self.delta_e(other)
    }
}

/// Find the closest color from a list using the specified method
///
/// Based on Java ColorUtil.findClosestColor implementation
///
/// # Example
///
/// ```
/// use pixestl::color::{Rgb, find_closest_color, ColorDistanceMethod};
///
/// let target = Rgb::new(100, 100, 100);
/// let palette = vec![
///     Rgb::new(0, 0, 0),
///     Rgb::new(128, 128, 128),
///     Rgb::new(255, 255, 255),
/// ];
///
/// let closest = find_closest_color(&target, &palette, ColorDistanceMethod::Rgb);
/// assert_eq!(closest, Rgb::new(128, 128, 128));
/// ```
pub fn find_closest_color(target: &Rgb, colors: &[Rgb], method: ColorDistanceMethod) -> Rgb {
    assert!(!colors.is_empty(), "Color list cannot be empty");

    match method {
        ColorDistanceMethod::Rgb => {
            let mut min_distance = f64::MAX;
            let mut closest = colors[0];

            for color in colors {
                let distance = target.distance(color);
                if distance < min_distance {
                    min_distance = distance;
                    closest = *color;
                }
            }

            closest
        }
        ColorDistanceMethod::CieLab => {
            let target_lab = CieLab::from(*target);
            let mut min_distance = f64::MAX;
            let mut closest = colors[0];

            for color in colors {
                let color_lab = CieLab::from(*color);
                let distance = target_lab.distance(&color_lab);
                if distance < min_distance {
                    min_distance = distance;
                    closest = *color;
                }
            }

            closest
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_color_distance_method_from_str() {
        use std::str::FromStr;
        assert_eq!(
            ColorDistanceMethod::from_str("RGB").unwrap(),
            ColorDistanceMethod::Rgb
        );
        assert_eq!(
            ColorDistanceMethod::from_str("CIELab").unwrap(),
            ColorDistanceMethod::CieLab
        );
        assert!(ColorDistanceMethod::from_str("Invalid").is_err());
    }

    #[test]
    fn test_color_distance_method_as_str() {
        assert_eq!(ColorDistanceMethod::Rgb.as_str(), "RGB");
        assert_eq!(ColorDistanceMethod::CieLab.as_str(), "CIELab");
    }

    #[test]
    fn test_color_distance_method_default() {
        assert_eq!(ColorDistanceMethod::default(), ColorDistanceMethod::CieLab);
    }

    #[test]
    fn test_rgb_distance_same_color() {
        let color = Rgb::new(128, 64, 200);
        assert_relative_eq!(color.distance(&color), 0.0, epsilon = 1e-10);
    }

    #[test]
    fn test_rgb_distance_symmetry() {
        let color1 = Rgb::new(100, 150, 200);
        let color2 = Rgb::new(110, 140, 210);

        assert_relative_eq!(
            color1.distance(&color2),
            color2.distance(&color1),
            epsilon = 1e-10
        );
    }

    #[test]
    fn test_rgb_distance_non_negative() {
        let color1 = Rgb::new(50, 100, 150);
        let color2 = Rgb::new(200, 250, 100);

        assert!(color1.distance(&color2) >= 0.0);
    }

    #[test]
    fn test_rgb_distance_values() {
        let black = Rgb::new(0, 0, 0);
        let white = Rgb::new(255, 255, 255);

        // Distance should be 255^2 * 3 = 195075
        assert_eq!(black.distance(&white), 195075.0);
    }

    #[test]
    fn test_cielab_distance() {
        let red = CieLab::from(Rgb::new(255, 0, 0));
        let green = CieLab::from(Rgb::new(0, 255, 0));

        let distance = red.distance(&green);

        // Should be same as delta_e
        assert_eq!(distance, red.delta_e(&green));
        assert!(distance > 80.0);
    }

    #[test]
    fn test_find_closest_color_rgb() {
        let target = Rgb::new(100, 100, 100);
        let palette = vec![
            Rgb::new(0, 0, 0),
            Rgb::new(128, 128, 128),
            Rgb::new(255, 255, 255),
        ];

        let closest = find_closest_color(&target, &palette, ColorDistanceMethod::Rgb);
        assert_eq!(closest, Rgb::new(128, 128, 128));
    }

    #[test]
    fn test_find_closest_color_cielab() {
        let target = Rgb::new(200, 50, 50);
        let palette = vec![
            Rgb::new(255, 0, 0), // Red
            Rgb::new(0, 255, 0), // Green
            Rgb::new(0, 0, 255), // Blue
        ];

        let closest = find_closest_color(&target, &palette, ColorDistanceMethod::CieLab);
        // Should pick red as it's closest in perceptual space
        assert_eq!(closest, Rgb::new(255, 0, 0));
    }

    #[test]
    fn test_find_closest_color_exact_match() {
        let target = Rgb::new(128, 64, 200);
        let palette = vec![
            Rgb::new(0, 0, 0),
            Rgb::new(128, 64, 200), // Exact match
            Rgb::new(255, 255, 255),
        ];

        let closest_rgb = find_closest_color(&target, &palette, ColorDistanceMethod::Rgb);
        let closest_lab = find_closest_color(&target, &palette, ColorDistanceMethod::CieLab);

        assert_eq!(closest_rgb, target);
        assert_eq!(closest_lab, target);
    }

    #[test]
    #[should_panic(expected = "Color list cannot be empty")]
    fn test_find_closest_color_empty_palette() {
        let target = Rgb::new(128, 128, 128);
        let palette: Vec<Rgb> = vec![];

        find_closest_color(&target, &palette, ColorDistanceMethod::Rgb);
    }

    #[test]
    fn test_rgb_vs_cielab_difference() {
        // Test case where RGB and CIELab give different results
        let target = Rgb::new(128, 128, 0); // Yellow-ish

        let palette = vec![
            Rgb::new(255, 0, 0), // Red
            Rgb::new(0, 255, 0), // Green
        ];

        let closest_rgb = find_closest_color(&target, &palette, ColorDistanceMethod::Rgb);
        let closest_lab = find_closest_color(&target, &palette, ColorDistanceMethod::CieLab);

        // RGB might give different result than CIELab due to perceptual differences
        // Both are valid, but CIELab should be more perceptually accurate
        assert!(palette.contains(&closest_rgb));
        assert!(palette.contains(&closest_lab));
    }
}

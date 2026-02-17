//! Configuration for lithophane generation

use crate::color::ColorDistanceMethod;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelCreationMethod {
    Additive,
    Full,
}

#[derive(Debug, Clone)]
pub struct LithophaneConfig {
    pub dest_width_mm: f64,
    pub dest_height_mm: f64,
    pub color_pixel_width: f64,
    pub color_pixel_layer_thickness: f64,
    pub color_pixel_layer_number: u32,
    pub color_layer: bool,
    pub texture_pixel_width: f64,
    pub texture_min_thickness: f64,
    pub texture_max_thickness: f64,
    pub texture_layer: bool,
    pub plate_thickness: f64,
    pub pixel_creation_method: PixelCreationMethod,
    pub color_number: usize,
    pub color_distance_method: ColorDistanceMethod,
    pub curve: f64,
    pub debug: bool,
    pub low_memory: bool,
    pub layer_thread_max_number: usize,
    pub row_thread_number: usize,
}

impl Default for LithophaneConfig {
    fn default() -> Self {
        Self {
            dest_width_mm: 0.0,
            dest_height_mm: 0.0,
            color_pixel_width: 0.8,
            color_pixel_layer_thickness: 0.1,
            color_pixel_layer_number: 5,
            color_layer: true,
            texture_pixel_width: 0.25,
            texture_min_thickness: 0.3,
            texture_max_thickness: 1.8,
            texture_layer: true,
            plate_thickness: 0.2,
            pixel_creation_method: PixelCreationMethod::Additive,
            color_number: 0,
            color_distance_method: ColorDistanceMethod::CieLab,
            curve: 0.0,
            debug: false,
            low_memory: false,
            layer_thread_max_number: 0,
            row_thread_number: num_cpus::get(),
        }
    }
}

impl LithophaneConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn validate(&self) -> crate::error::Result<()> {
        if self.color_pixel_width <= 0.0 {
            return Err(crate::error::PixestlError::Config(
                "color_pixel_width must be positive".to_string(),
            ));
        }
        if self.texture_pixel_width <= 0.0 {
            return Err(crate::error::PixestlError::Config(
                "texture_pixel_width must be positive".to_string(),
            ));
        }
        if self.color_pixel_layer_thickness <= 0.0 {
            return Err(crate::error::PixestlError::Config(
                "color_pixel_layer_thickness must be positive".to_string(),
            ));
        }
        if self.color_pixel_layer_number == 0 {
            return Err(crate::error::PixestlError::Config(
                "color_pixel_layer_number must be positive".to_string(),
            ));
        }
        if self.texture_min_thickness <= 0.0 {
            return Err(crate::error::PixestlError::Config(
                "texture_min_thickness must be positive".to_string(),
            ));
        }
        if self.texture_max_thickness <= self.texture_min_thickness {
            return Err(crate::error::PixestlError::Config(
                "texture_max_thickness must be greater than texture_min_thickness".to_string(),
            ));
        }
        if self.plate_thickness < 0.0 {
            return Err(crate::error::PixestlError::Config(
                "plate_thickness must be non-negative".to_string(),
            ));
        }
        if !self.color_layer && !self.texture_layer {
            return Err(crate::error::PixestlError::Config(
                "At least one of color_layer or texture_layer must be enabled".to_string(),
            ));
        }
        if self.curve < 0.0 || self.curve > 360.0 {
            return Err(crate::error::PixestlError::Config(
                "curve must be between 0 and 360 degrees".to_string(),
            ));
        }
        Ok(())
    }

    pub fn total_color_layer_height(&self) -> f64 {
        self.color_pixel_layer_thickness * self.color_pixel_layer_number as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_is_valid() {
        let config = LithophaneConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_invalid_color_pixel_width() {
        let config = LithophaneConfig {
            color_pixel_width: 0.0,
            ..LithophaneConfig::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_texture_pixel_width() {
        let config = LithophaneConfig {
            texture_pixel_width: -1.0,
            ..LithophaneConfig::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_layer_thickness() {
        let config = LithophaneConfig {
            color_pixel_layer_thickness: 0.0,
            ..LithophaneConfig::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_layer_number() {
        let config = LithophaneConfig {
            color_pixel_layer_number: 0,
            ..LithophaneConfig::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_texture_thickness_range() {
        let defaults = LithophaneConfig::default();
        let config = LithophaneConfig {
            texture_max_thickness: defaults.texture_min_thickness,
            ..defaults
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_plate_thickness() {
        let config = LithophaneConfig {
            plate_thickness: -0.1,
            ..LithophaneConfig::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_no_layers_enabled() {
        let config = LithophaneConfig {
            color_layer: false,
            texture_layer: false,
            ..LithophaneConfig::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_total_color_layer_height() {
        let config = LithophaneConfig {
            color_pixel_layer_thickness: 0.1,
            color_pixel_layer_number: 5,
            ..LithophaneConfig::default()
        };
        assert!((config.total_color_layer_height() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_valid_curve_zero() {
        let config = LithophaneConfig {
            curve: 0.0,
            ..LithophaneConfig::default()
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_valid_curve_360() {
        let config = LithophaneConfig {
            curve: 360.0,
            ..LithophaneConfig::default()
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_valid_curve_90() {
        let config = LithophaneConfig {
            curve: 90.0,
            ..LithophaneConfig::default()
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_invalid_curve_negative() {
        let config = LithophaneConfig {
            curve: -10.0,
            ..LithophaneConfig::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_curve_over_360() {
        let config = LithophaneConfig {
            curve: 400.0,
            ..LithophaneConfig::default()
        };
        assert!(config.validate().is_err());
    }
}

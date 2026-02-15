//! Configuration structures for PIXEstL

use crate::error::Result;
use std::path::PathBuf;

/// Main configuration for lithophane generation
#[derive(Debug, Clone)]
pub struct Config {
    pub src_image_path: PathBuf,
    pub palette_path: PathBuf,
    pub dest_zip_path: Option<PathBuf>,
    pub dest_image_width: Option<f64>,
    pub dest_image_height: Option<f64>,
    pub plate_thickness: f64,
    pub color_pixel_width: f64,
    pub color_pixel_layer_thickness: f64,
    pub color_pixel_layer_number: u32,
    pub texture_pixel_width: f64,
    pub texture_min_thickness: f64,
    pub texture_max_thickness: f64,
    pub color_number: Option<u32>,
    pub color_distance_method: crate::color::ColorDistanceMethod,
    pub generate_color_layers: bool,
    pub generate_texture_layer: bool,
    pub debug: bool,
}

impl Config {
    /// Creates a new configuration builder
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }
}

/// Builder for Config
#[derive(Default)]
pub struct ConfigBuilder {
    config: Config,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            src_image_path: PathBuf::new(),
            palette_path: PathBuf::new(),
            dest_zip_path: None,
            dest_image_width: None,
            dest_image_height: None,
            plate_thickness: 0.2,
            color_pixel_width: 0.8,
            color_pixel_layer_thickness: 0.1,
            color_pixel_layer_number: 5,
            texture_pixel_width: 0.25,
            texture_min_thickness: 0.2,
            texture_max_thickness: 2.5,
            color_number: None,
            color_distance_method: crate::color::ColorDistanceMethod::default(),
            generate_color_layers: true,
            generate_texture_layer: true,
            debug: false,
        }
    }
}

impl ConfigBuilder {
    pub fn src_image_path<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.config.src_image_path = path.into();
        self
    }

    pub fn palette_path<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.config.palette_path = path.into();
        self
    }

    pub fn dest_image_width(mut self, width: f64) -> Self {
        self.config.dest_image_width = Some(width);
        self
    }

    pub fn build(self) -> Result<Config> {
        Ok(self.config)
    }
}

use crate::color::ColorDistanceMethod;

/// Generation mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum PixelCreationMethod {
    /// Additive color mixing with multiple layers
    Additive,
    /// Full color layers (no mixing)
    Full,
}

/// Palette loader configuration
#[derive(Debug, Clone)]
pub struct PaletteLoaderConfig {
    /// Number of layers per color pixel
    pub nb_layers: u32,
    /// Pixel creation method
    pub creation_method: PixelCreationMethod,
    /// Number of colors to use (0 = all active colors)
    pub color_number: usize,
    /// Color distance method for quantization
    pub distance_method: ColorDistanceMethod,
}

impl Default for PaletteLoaderConfig {
    fn default() -> Self {
        Self {
            nb_layers: 5,
            creation_method: PixelCreationMethod::Additive,
            color_number: 0,
            distance_method: ColorDistanceMethod::CieLab,
        }
    }
}

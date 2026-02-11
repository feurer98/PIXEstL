//! JSON Palette loader

use crate::error::Result;
use crate::palette::Palette;
use std::path::Path;

/// Loads palettes from JSON files
pub struct PaletteLoader;

impl PaletteLoader {
    /// Loads a palette from a JSON file
    ///
    /// # TODO
    ///
    /// - Implement JSON parsing with serde
    /// - Generate all ColorCombis recursively
    /// - Set up AMS groups
    pub fn load(_path: &Path, _nb_layers: u32) -> Result<Palette> {
        // TODO: Implement full JSON loading
        Ok(Palette::new(5))
    }
}

//! JSON Palette loader with serde

use crate::color::{ColorDistanceMethod, Rgb};
use crate::error::{PixestlError, Result};
use crate::palette::{create_multi_combi, ColorCombi, ColorLayer, Palette};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Layer definition in JSON (HSL or hexcode)
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum LayerDefinition {
    /// HSL format
    Hsl {
        #[serde(rename = "H")]
        h: f64,
        #[serde(rename = "S")]
        s: f64,
        #[serde(rename = "L")]
        l: f64,
    },
    /// Hexcode format
    Hexcode { hexcode: String },
}

/// Color entry in the palette JSON
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PaletteColorEntry {
    pub name: String,
    #[serde(default = "default_active")]
    pub active: bool,
    #[serde(default)]
    pub layers: Option<HashMap<String, LayerDefinition>>,
}

fn default_active() -> bool {
    true
}

/// Generation mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

/// Loads palettes from JSON files
pub struct PaletteLoader;

impl PaletteLoader {
    /// Loads a palette from a JSON file
    ///
    /// Based on Java Palette constructor
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the JSON palette file
    /// * `config` - Loader configuration
    ///
    /// # Returns
    ///
    /// A fully initialized Palette with all color combinations computed
    ///
    /// # Example
    ///
    /// ```no_run
    /// use pixestl::palette::{PaletteLoader, PaletteLoaderConfig};
    /// use std::path::Path;
    ///
    /// let config = PaletteLoaderConfig::default();
    /// let palette = PaletteLoader::load(Path::new("palette.json"), config).unwrap();
    /// println!("Loaded {} colors", palette.color_count());
    /// ```
    pub fn load(path: &Path, config: PaletteLoaderConfig) -> Result<Palette> {
        // Read and parse JSON
        let json_content = fs::read_to_string(path)?;

        let palette_data: HashMap<String, PaletteColorEntry> = serde_json::from_str(&json_content)?;

        let mut palette = Palette::new(config.nb_layers);

        // Build hex codes map
        let mut hex_codes_map = HashMap::new();
        for (hex_code, entry) in &palette_data {
            hex_codes_map.insert(hex_code.clone(), entry.name.clone());
        }
        palette.set_hex_codes(hex_codes_map.clone());

        // Collect active colors
        let mut hex_color_list: Vec<String> = palette_data
            .iter()
            .filter(|(_, entry)| entry.active && entry.layers.is_some())
            .map(|(hex, _): (&String, &PaletteColorEntry)| hex.clone())
            .collect();

        // Sort by hex code for deterministic order
        hex_color_list.sort();

        // Validate white color in additive mode
        if config.creation_method == PixelCreationMethod::Additive
            && !hex_color_list.contains(&"#FFFFFF".to_string())
        {
            return Err(PixestlError::InvalidPalette(
                "\"#FFFFFF\" not found in the palette. The code \"#FFFFFF\" is mandatory in additive mode.".to_string()
            ));
        }

        // Create ColorLayers
        let color_layers = Self::create_color_layers(&palette_data, &config)?;

        // Compute colors by group
        Self::compute_colors_by_group(&mut palette, &color_layers, &hex_color_list, &config)?;

        Ok(palette)
    }

    /// Creates ColorLayers from palette data
    fn create_color_layers(
        palette_data: &HashMap<String, PaletteColorEntry>,
        config: &PaletteLoaderConfig,
    ) -> Result<Vec<ColorLayer>> {
        let mut color_layers = Vec::new();

        for (hex_code, entry) in palette_data {
            if !entry.active {
                continue;
            }

            match config.creation_method {
                PixelCreationMethod::Additive => {
                    if let Some(layers) = &entry.layers {
                        for (layer_key, layer_def) in layers {
                            let layer_num: u32 = layer_key.parse().map_err(|_| {
                                PixestlError::InvalidPalette(format!(
                                    "Invalid layer number: {}",
                                    layer_key
                                ))
                            })?;

                            let (h, s, l) = match layer_def {
                                LayerDefinition::Hsl { h, s, l } => (*h, *s, *l),
                                LayerDefinition::Hexcode { hexcode } => {
                                    let rgb = Rgb::from_hex(hexcode)?;
                                    let hsl = crate::color::Hsl::from(rgb);
                                    (hsl.h, hsl.s, hsl.l)
                                }
                            };

                            color_layers.push(ColorLayer::new(
                                hex_code.clone(),
                                layer_num,
                                h,
                                s,
                                l,
                            ));
                        }
                    }
                }
                PixelCreationMethod::Full => {
                    // In full mode, use the hex code directly
                    let rgb = Rgb::from_hex(hex_code)?;
                    let hsl = crate::color::Hsl::from(rgb);

                    color_layers.push(ColorLayer::new(
                        hex_code.clone(),
                        config.nb_layers,
                        hsl.h,
                        hsl.s,
                        hsl.l,
                    ));
                }
            }
        }

        // Sort by K value (darkness) descending
        color_layers.sort_by(|a, b| a.compare_by_k(b));

        Ok(color_layers)
    }

    /// Computes all color combinations by group (AMS support)
    ///
    /// Based on Java Palette.computeColorsByGroup
    fn compute_colors_by_group(
        palette: &mut Palette,
        color_layers: &[ColorLayer],
        hex_color_list: &[String],
        config: &PaletteLoaderConfig,
    ) -> Result<()> {
        // Remove white from the list for grouping
        let working_hex_list: Vec<String> = hex_color_list
            .iter()
            .filter(|h| *h != "#FFFFFF")
            .cloned()
            .collect();

        // Determine number of colors per group
        let nb_color_pool =
            if config.creation_method == PixelCreationMethod::Additive && config.color_number > 0 {
                config.color_number.saturating_sub(1)
            } else {
                working_hex_list.len()
            };

        // Calculate number of groups
        let nb_groups = if nb_color_pool > 0 {
            working_hex_list.len().div_ceil(nb_color_pool)
        } else {
            1
        };

        // Create groups
        let mut hex_color_groups: Vec<Vec<String>> = (0..nb_groups).map(|_| Vec::new()).collect();

        for (i, hex_code) in working_hex_list.iter().enumerate() {
            let group_idx = i / nb_color_pool;
            if group_idx < hex_color_groups.len() {
                hex_color_groups[group_idx].push(hex_code.clone());
            }
        }

        // Add white to each group
        for group in &mut hex_color_groups {
            group.push("#FFFFFF".to_string());
        }

        // Generate combinations for each group
        let mut color_combi_list_list: Vec<Vec<ColorCombi>> = Vec::new();

        for group in &hex_color_groups {
            let combis = create_multi_combi(Some(group), color_layers, config.nb_layers);
            color_combi_list_list.push(combis);
        }

        // Combine groups progressively
        let mut temp_color_combi_list = vec![color_combi_list_list[0].clone()];

        for i in 0..nb_groups - 1 {
            let mut temp_list = Vec::new();

            for combi_i in &temp_color_combi_list[i] {
                for combi_i1 in &color_combi_list_list[i + 1] {
                    temp_list.push(combi_i.combine_with_combi(combi_i1));
                }
            }

            temp_color_combi_list.push(temp_list);
        }

        let final_combi_list = temp_color_combi_list.last().ok_or_else(|| {
            PixestlError::InvalidPalette("No color combination groups were generated".to_string())
        })?;

        // Set group information
        palette.set_nb_groups(nb_groups);

        // Factorize and add all combinations
        for mut combi in final_combi_list.clone() {
            combi.factorize();
            palette.add_combi(combi);
        }

        // Optimize white layers
        palette.optimize_white_layers(nb_color_pool);

        // Initialize hex color group list for AMS
        Self::init_hex_color_group_list(palette, &hex_color_groups, nb_color_pool);

        Ok(())
    }

    /// Initializes hex color group list for AMS
    ///
    /// Based on Java Palette.initHexColorGroupList
    fn init_hex_color_group_list(
        palette: &mut Palette,
        hex_color_groups: &[Vec<String>],
        nb_color_pool: usize,
    ) {
        let mut hex_color_group_list: Vec<Vec<String>> =
            (0..nb_color_pool).map(|_| Vec::new()).collect();

        for group in hex_color_groups {
            let group_without_white: Vec<String> =
                group.iter().filter(|h| *h != "#FFFFFF").cloned().collect();

            for (i, hex_code) in group_without_white.iter().enumerate() {
                if i < nb_color_pool {
                    hex_color_group_list[i].push(hex_code.clone());
                }
            }
        }

        // Add white group at the end
        hex_color_group_list.push(vec!["#FFFFFF".to_string()]);

        palette.set_hex_color_groups(hex_color_group_list);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_test_palette_json() -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        let json = "{\n\
  \"#FF0000\": {\n\
    \"name\": \"Red\",\n\
    \"active\": true,\n\
    \"layers\": {\n\
      \"5\": { \"H\": 0, \"S\": 100, \"L\": 50 },\n\
      \"3\": { \"H\": 0, \"S\": 100, \"L\": 60 },\n\
      \"2\": { \"H\": 0, \"S\": 100, \"L\": 70 }\n\
    }\n\
  },\n\
  \"#00FF00\": {\n\
    \"name\": \"Green\",\n\
    \"active\": true,\n\
    \"layers\": {\n\
      \"5\": { \"H\": 120, \"S\": 100, \"L\": 50 },\n\
      \"3\": { \"H\": 120, \"S\": 100, \"L\": 60 }\n\
    }\n\
  },\n\
  \"#FFFFFF\": {\n\
    \"name\": \"White\",\n\
    \"active\": true,\n\
    \"layers\": {\n\
      \"5\": { \"H\": 0, \"S\": 0, \"L\": 100 }\n\
    }\n\
  },\n\
  \"#0000FF\": {\n\
    \"name\": \"Blue (Inactive)\",\n\
    \"active\": false,\n\
    \"layers\": {\n\
      \"5\": { \"H\": 240, \"S\": 100, \"L\": 50 }\n\
    }\n\
  }\n\
}";
        write!(file, "{}", json).unwrap();
        file
    }

    #[test]
    fn test_load_palette_basic() {
        let file = create_test_palette_json();
        let config = PaletteLoaderConfig::default();

        let palette = PaletteLoader::load(file.path(), config).unwrap();

        // Should have loaded active colors
        assert!(palette.color_count() > 0);

        // Should have hex codes
        assert_eq!(palette.get_color_name("#FF0000"), Some("Red"));
        assert_eq!(palette.get_color_name("#00FF00"), Some("Green"));
        assert_eq!(palette.get_color_name("#FFFFFF"), Some("White"));
        assert_eq!(palette.get_color_name("#0000FF"), Some("Blue (Inactive)"));
    }

    #[test]
    fn test_load_palette_requires_white() {
        let mut file = NamedTempFile::new().unwrap();
        let json = "{\n\
  \"#FF0000\": {\n\
    \"name\": \"Red\",\n\
    \"active\": true,\n\
    \"layers\": {\n\
      \"5\": { \"H\": 0, \"S\": 100, \"L\": 50 }\n\
    }\n\
  }\n\
}";
        write!(file, "{}", json).unwrap();

        let config = PaletteLoaderConfig::default();
        let result = PaletteLoader::load(file.path(), config);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("\"#FFFFFF\" not found"));
    }

    #[test]
    fn test_load_palette_hexcode_layer_def() {
        let mut file = NamedTempFile::new().unwrap();
        let json = "{\n\
  \"#FF0000\": {\n\
    \"name\": \"Red\",\n\
    \"active\": true,\n\
    \"layers\": {\n\
      \"5\": { \"hexcode\": \"#FF0000\" }\n\
    }\n\
  },\n\
  \"#FFFFFF\": {\n\
    \"name\": \"White\",\n\
    \"active\": true,\n\
    \"layers\": {\n\
      \"5\": { \"H\": 0, \"S\": 0, \"L\": 100 }\n\
    }\n\
  }\n\
}";
        write!(file, "{}", json).unwrap();

        let config = PaletteLoaderConfig::default();
        let palette = PaletteLoader::load(file.path(), config).unwrap();

        assert!(palette.color_count() > 0);
    }

    #[test]
    fn test_create_color_layers() {
        let mut palette_data = HashMap::new();

        let mut layers = HashMap::new();
        layers.insert(
            "5".to_string(),
            LayerDefinition::Hsl {
                h: 0.0,
                s: 100.0,
                l: 50.0,
            },
        );

        palette_data.insert(
            "#FF0000".to_string(),
            PaletteColorEntry {
                name: "Red".to_string(),
                active: true,
                layers: Some(layers),
            },
        );

        let config = PaletteLoaderConfig::default();
        let color_layers = PaletteLoader::create_color_layers(&palette_data, &config).unwrap();

        assert_eq!(color_layers.len(), 1);
        assert_eq!(color_layers[0].hex_code(), "#FF0000");
        assert_eq!(color_layers[0].layer(), 5);
    }

    #[test]
    fn test_palette_groups() {
        let file = create_test_palette_json();
        let config = PaletteLoaderConfig::default();

        let palette = PaletteLoader::load(file.path(), config).unwrap();

        assert!(palette.nb_groups() > 0);
        assert!(!palette.hex_color_groups().is_empty());
    }
}

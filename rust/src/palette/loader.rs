//! JSON Palette loader with serde

use crate::color::Rgb;
use crate::error::{PixestlError, Result};
use crate::palette::config::{PaletteLoaderConfig, PixelCreationMethod};
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

/// Loads palettes from JSON files
pub struct PaletteLoader;

impl PaletteLoader {
    /// Loads raw palette data from a JSON file without computing combinations.
    ///
    /// Useful for displaying palette information or validating the palette
    /// before running a full generation.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the JSON palette file
    ///
    /// # Returns
    ///
    /// A HashMap of hex codes to PaletteColorEntry data
    pub fn load_raw(path: &Path) -> Result<HashMap<String, PaletteColorEntry>> {
        let json_content = fs::read_to_string(path)?;
        let palette_data: HashMap<String, PaletteColorEntry> = serde_json::from_str(&json_content)?;
        Ok(palette_data)
    }

    /// Validates palette completeness against the target layer count.
    ///
    /// Checks each active filament for missing or incomplete layer definitions
    /// and returns a list of warning messages. These are warnings, not errors —
    /// the palette may still work but with suboptimal color accuracy.
    ///
    /// # Arguments
    ///
    /// * `palette_data` - Raw palette data from `load_raw()`
    /// * `target_layers` - Number of color layers configured (--color-layers)
    ///
    /// # Returns
    ///
    /// A list of warning strings. Empty if the palette is complete.
    pub fn validate_completeness(
        palette_data: &HashMap<String, PaletteColorEntry>,
        target_layers: u32,
    ) -> Vec<String> {
        let mut warnings = Vec::new();

        // Sortierung für deterministische Ausgabe
        let mut entries: Vec<_> = palette_data.iter().collect();
        entries.sort_by_key(|(a, _)| *a);

        for (hex_code, entry) in &entries {
            if !entry.active {
                continue;
            }

            if let Some(layers) = &entry.layers {
                let mut defined_layers: Vec<u32> = layers
                    .keys()
                    .filter_map(|k| k.parse::<u32>().ok())
                    .collect();
                defined_layers.sort();

                if defined_layers.is_empty() {
                    warnings.push(format!(
                        "Filament {} ('{}') ist aktiv, hat aber keine gueltigen Layer-Definitionen.",
                        hex_code, entry.name
                    ));
                    continue;
                }

                let max_defined = *defined_layers
                    .iter()
                    .max()
                    .expect("invariant: non-empty after is_empty check");

                // Warn if only 1 layer defined but target is higher
                if defined_layers.len() == 1 && target_layers > 1 {
                    warnings.push(format!(
                        "Filament {} ('{}') hat nur Schicht {} definiert, aber {} Schichten \
                        sind konfiguriert (--color-layers {}). Fuer bessere Farbergebnisse: \
                        HSL-Werte fuer Schichten 1 bis {} definieren.",
                        hex_code,
                        entry.name,
                        defined_layers[0],
                        target_layers,
                        target_layers,
                        target_layers
                    ));
                }
                // Warn if max defined layer is less than target
                else if max_defined < target_layers && defined_layers.len() > 1 {
                    warnings.push(format!(
                        "Filament {} ('{}') hat Schichten bis {} definiert, aber {} Schichten \
                        sind konfiguriert. Schichten {} bis {} fehlen.",
                        hex_code,
                        entry.name,
                        max_defined,
                        target_layers,
                        max_defined + 1,
                        target_layers
                    ));
                }
            } else {
                warnings.push(format!(
                    "Filament {} ('{}') ist aktiv, hat aber keine Layer-Definitionen.",
                    hex_code, entry.name
                ));
            }
        }

        warnings
    }

    /// Loads a palette from a JSON file.
    ///
    /// Reads the file, parses it, and delegates to [`Self::load_from_raw`].
    /// If you have already called [`Self::load_raw`] (e.g. to display warnings),
    /// use [`Self::load_from_raw`] directly to avoid reading the file twice.
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
        Self::load_from_raw(Self::load_raw(path)?, config)
    }

    /// Builds a fully initialized Palette from already-parsed palette data.
    ///
    /// Use this when you have already called [`Self::load_raw`] to validate or
    /// display palette information, so the file is not read a second time.
    pub fn load_from_raw(
        palette_data: HashMap<String, PaletteColorEntry>,
        config: PaletteLoaderConfig,
    ) -> Result<Palette> {
        let mut palette = Palette::new(config.color_layer_count);

        let hex_codes_map: HashMap<String, String> = palette_data
            .iter()
            .map(|(hex, entry)| (hex.clone(), entry.name.clone()))
            .collect();
        palette.set_hex_codes(hex_codes_map);

        let mut hex_color_list: Vec<String> = palette_data
            .iter()
            .filter(|(_, entry)| entry.active && entry.layers.is_some())
            .map(|(hex, _)| hex.clone())
            .collect();
        hex_color_list.sort();

        if config.creation_method == PixelCreationMethod::Additive
            && !hex_color_list.contains(&"#FFFFFF".to_string())
        {
            return Err(PixestlError::InvalidPalette(
                "\"#FFFFFF\" not found in the palette. The code \"#FFFFFF\" is mandatory in additive mode.".to_string()
            ));
        }

        let color_layers = Self::create_color_layers(&palette_data, &config)?;
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
                                    "Invalid layer number: {layer_key}"
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
                        config.color_layer_count,
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

    /// Orchestrates AMS group generation, combination computation, and palette population.
    fn compute_colors_by_group(
        palette: &mut Palette,
        color_layers: &[ColorLayer],
        hex_color_list: &[String],
        config: &PaletteLoaderConfig,
    ) -> Result<()> {
        let (hex_color_groups, nb_color_pool) = Self::build_ams_groups(hex_color_list, config)?;
        let combi_groups = Self::generate_combis_per_group(
            &hex_color_groups,
            color_layers,
            config.color_layer_count,
        );
        let final_combis = Self::merge_groups(combi_groups)?;

        palette.set_nb_groups(hex_color_groups.len());
        for mut combi in final_combis {
            combi.factorize()?;
            palette.add_combi(combi);
        }
        palette.optimize_white_layers();
        Self::init_hex_color_group_list(palette, &hex_color_groups, nb_color_pool);

        Ok(())
    }

    /// Divides active non-white colours into AMS groups and appends white to each group.
    ///
    /// Returns `(groups, nb_color_pool)` where `nb_color_pool` is the number of
    /// non-white slots per group (used later by `optimize_white_layers` and
    /// `init_hex_color_group_list`).
    fn build_ams_groups(
        hex_color_list: &[String],
        config: &PaletteLoaderConfig,
    ) -> Result<(Vec<Vec<String>>, usize)> {
        let working_hex_list: Vec<String> = hex_color_list
            .iter()
            .filter(|h| *h != "#FFFFFF")
            .cloned()
            .collect();

        // In Additive mode one slot is always reserved for white, so color_number=1
        // would leave 0 non-white slots — guard against division-by-zero below.
        let nb_color_pool =
            if config.creation_method == PixelCreationMethod::Additive && config.color_number > 0 {
                config.color_number.saturating_sub(1)
            } else {
                working_hex_list.len()
            };

        if nb_color_pool == 0 && !working_hex_list.is_empty() {
            return Err(PixestlError::Config(
                "color_number must be at least 2 in additive mode \
                 (1 slot is always reserved for white)"
                    .to_string(),
            ));
        }

        let nb_groups = if nb_color_pool > 0 {
            working_hex_list.len().div_ceil(nb_color_pool)
        } else {
            1
        };

        let mut hex_color_groups: Vec<Vec<String>> = (0..nb_groups).map(|_| Vec::new()).collect();
        for (i, hex_code) in working_hex_list.iter().enumerate() {
            hex_color_groups[i / nb_color_pool].push(hex_code.clone());
        }
        for group in &mut hex_color_groups {
            group.push("#FFFFFF".to_string());
        }

        Ok((hex_color_groups, nb_color_pool))
    }

    /// Generates all valid `ColorCombi`s for every AMS group independently.
    fn generate_combis_per_group(
        hex_color_groups: &[Vec<String>],
        color_layers: &[ColorLayer],
        color_layer_count: u32,
    ) -> Vec<Vec<ColorCombi>> {
        hex_color_groups
            .iter()
            .map(|group| create_multi_combi(Some(group), color_layers, color_layer_count))
            .collect()
    }

    /// Merges per-group combination lists via progressive cartesian product.
    ///
    /// For groups `[A1, A2]` and `[B1, B2]` the result is
    /// `[A1+B1, A1+B2, A2+B1, A2+B2]`.
    fn merge_groups(combi_groups: Vec<Vec<ColorCombi>>) -> Result<Vec<ColorCombi>> {
        let mut iter = combi_groups.into_iter();
        let first = iter.next().ok_or_else(|| {
            PixestlError::InvalidPalette("No color combination groups were generated".to_string())
        })?;

        Ok(iter.fold(first, |accumulated, next_group| {
            accumulated
                .iter()
                .flat_map(|a| next_group.iter().map(move |b| a.combine_with_combi(b)))
                .collect()
        }))
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
    use std::io::Write as _;
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

    #[test]
    fn test_load_raw() {
        let file = create_test_palette_json();
        let data = PaletteLoader::load_raw(file.path()).unwrap();

        assert!(data.contains_key("#FF0000"));
        assert!(data.contains_key("#FFFFFF"));
        assert_eq!(data["#FF0000"].name, "Red");
        assert!(data["#FF0000"].active);
        assert!(!data["#0000FF"].active);
    }

    #[test]
    fn test_validate_completeness_no_warnings() {
        // Palette with all 5 layers defined — should produce no warnings
        let mut data = HashMap::new();
        let mut layers = HashMap::new();
        for i in 1..=5 {
            layers.insert(
                i.to_string(),
                LayerDefinition::Hsl {
                    h: 200.0,
                    s: 100.0,
                    l: 80.0 - (i as f64 * 8.0),
                },
            );
        }
        data.insert(
            "#0086D6".to_string(),
            PaletteColorEntry {
                name: "Cyan".to_string(),
                active: true,
                layers: Some(layers),
            },
        );

        let warnings = PaletteLoader::validate_completeness(&data, 5);
        assert!(
            warnings.is_empty(),
            "Expected no warnings, got: {:?}",
            warnings
        );
    }

    #[test]
    fn test_validate_completeness_single_layer_warning() {
        // Palette with only layer 5 defined but 5 layers configured
        let mut data = HashMap::new();
        let mut layers = HashMap::new();
        layers.insert(
            "5".to_string(),
            LayerDefinition::Hsl {
                h: 200.0,
                s: 100.0,
                l: 50.0,
            },
        );
        data.insert(
            "#0086D6".to_string(),
            PaletteColorEntry {
                name: "Cyan".to_string(),
                active: true,
                layers: Some(layers),
            },
        );

        let warnings = PaletteLoader::validate_completeness(&data, 5);
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("nur Schicht 5 definiert"));
        assert!(warnings[0].contains("5 Schichten"));
    }

    #[test]
    fn test_validate_completeness_missing_upper_layers() {
        // Palette with layers 1-3 defined but 5 layers configured
        let mut data = HashMap::new();
        let mut layers = HashMap::new();
        for i in 1..=3 {
            layers.insert(
                i.to_string(),
                LayerDefinition::Hsl {
                    h: 200.0,
                    s: 100.0,
                    l: 80.0 - (i as f64 * 8.0),
                },
            );
        }
        data.insert(
            "#0086D6".to_string(),
            PaletteColorEntry {
                name: "Cyan".to_string(),
                active: true,
                layers: Some(layers),
            },
        );

        let warnings = PaletteLoader::validate_completeness(&data, 5);
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("Schichten bis 3 definiert"));
        assert!(warnings[0].contains("4 bis 5 fehlen"));
    }

    #[test]
    fn test_validate_completeness_inactive_no_warning() {
        // Inactive filament with incomplete layers — should produce no warning
        let mut data = HashMap::new();
        let mut layers = HashMap::new();
        layers.insert(
            "5".to_string(),
            LayerDefinition::Hsl {
                h: 240.0,
                s: 100.0,
                l: 50.0,
            },
        );
        data.insert(
            "#0000FF".to_string(),
            PaletteColorEntry {
                name: "Blue".to_string(),
                active: false,
                layers: Some(layers),
            },
        );

        let warnings = PaletteLoader::validate_completeness(&data, 5);
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_validate_completeness_no_layers_warning() {
        // Active filament with no layers defined
        let mut data = HashMap::new();
        data.insert(
            "#FF0000".to_string(),
            PaletteColorEntry {
                name: "Red".to_string(),
                active: true,
                layers: None,
            },
        );

        let warnings = PaletteLoader::validate_completeness(&data, 5);
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("keine Layer-Definitionen"));
    }

    #[test]
    fn test_validate_completeness_single_layer_config_no_warning() {
        // Palette with 1 layer defined and 1 layer configured — should be fine
        let mut data = HashMap::new();
        let mut layers = HashMap::new();
        layers.insert(
            "1".to_string(),
            LayerDefinition::Hsl {
                h: 200.0,
                s: 100.0,
                l: 50.0,
            },
        );
        data.insert(
            "#0086D6".to_string(),
            PaletteColorEntry {
                name: "Cyan".to_string(),
                active: true,
                layers: Some(layers),
            },
        );

        let warnings = PaletteLoader::validate_completeness(&data, 1);
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_load_color_number_one_returns_error() {
        // Regression test for B-01: --color-number 1 in Additive mode must not panic
        // (division-by-zero in the AMS group-assignment loop).
        let file = create_test_palette_json();
        let config = PaletteLoaderConfig {
            color_number: 1,
            creation_method: PixelCreationMethod::Additive,
            ..PaletteLoaderConfig::default()
        };

        let result = PaletteLoader::load(file.path(), config);
        assert!(
            result.is_err(),
            "color_number=1 with non-white filaments must return an error, not panic"
        );
        let msg = result.unwrap_err().to_string();
        assert!(
            msg.contains("color_number must be at least 2"),
            "error message should mention the minimum: {msg}"
        );
    }

    #[test]
    fn test_load_color_number_one_white_only_is_ok() {
        // Edge-case: palette with only #FFFFFF active; working_hex_list is empty,
        // so color_number=1 should not trigger the guard.
        let mut file = tempfile::NamedTempFile::new().unwrap();
        let json = r##"{
  "#FFFFFF": {
    "name": "White",
    "active": true,
    "layers": { "5": { "H": 0, "S": 0, "L": 100 } }
  }
}"##;
        use std::io::Write as _;
        write!(file, "{json}").unwrap();

        let config = PaletteLoaderConfig {
            color_number: 1,
            creation_method: PixelCreationMethod::Additive,
            ..PaletteLoaderConfig::default()
        };

        // Only white → working_hex_list is empty → guard must not fire
        let result = PaletteLoader::load(file.path(), config);
        assert!(
            result.is_ok(),
            "white-only palette with color_number=1 should succeed: {result:?}"
        );
    }

    // ── B-05: load_from_raw ─────────────────────────────────────────────────

    #[test]
    fn test_load_from_raw_equivalent_to_load() {
        // load_from_raw(load_raw(path)) must produce the same palette as load(path)
        let file = create_test_palette_json();
        let config = PaletteLoaderConfig::default();

        let via_load = PaletteLoader::load(file.path(), config.clone()).unwrap();
        let raw = PaletteLoader::load_raw(file.path()).unwrap();
        let via_raw = PaletteLoader::load_from_raw(raw, config).unwrap();

        assert_eq!(via_load.color_count(), via_raw.color_count());
        assert_eq!(via_load.nb_groups(), via_raw.nb_groups());
        assert_eq!(via_load.hex_color_groups(), via_raw.hex_color_groups());
        // Name map is identical
        for hex in ["#FF0000", "#00FF00", "#FFFFFF", "#0000FF"] {
            assert_eq!(via_load.get_color_name(hex), via_raw.get_color_name(hex));
        }
    }

    #[test]
    fn test_load_from_raw_rejects_missing_white() {
        let raw = {
            let mut m = std::collections::HashMap::new();
            let mut layers = std::collections::HashMap::new();
            layers.insert(
                "5".to_string(),
                LayerDefinition::Hsl {
                    h: 0.0,
                    s: 100.0,
                    l: 50.0,
                },
            );
            m.insert(
                "#FF0000".to_string(),
                PaletteColorEntry {
                    name: "Red".to_string(),
                    active: true,
                    layers: Some(layers),
                },
            );
            m
        };
        let result = PaletteLoader::load_from_raw(raw, PaletteLoaderConfig::default());
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("#FFFFFF"));
    }

    // ── B-07: build_ams_groups / merge_groups ───────────────────────────────

    #[test]
    fn test_build_ams_groups_single_group() {
        // No color_number limit → all non-white colours end up in one group
        let hex_list = vec![
            "#FF0000".to_string(),
            "#00FF00".to_string(),
            "#FFFFFF".to_string(),
        ];
        let config = PaletteLoaderConfig::default(); // color_number = 0

        let (groups, nb_color_pool) = PaletteLoader::build_ams_groups(&hex_list, &config).unwrap();

        assert_eq!(groups.len(), 1);
        assert!(
            groups[0].contains(&"#FFFFFF".to_string()),
            "white must be in group"
        );
        assert_eq!(nb_color_pool, 2); // Red + Green
    }

    #[test]
    fn test_build_ams_groups_splits_into_two_groups() {
        // 4 colours + white, color_number=3 → pool=2 → 2 groups of 2 non-white + white each
        let hex_list = vec![
            "#FF0000".to_string(),
            "#00FF00".to_string(),
            "#0000FF".to_string(),
            "#FFFF00".to_string(),
            "#FFFFFF".to_string(),
        ];
        let config = PaletteLoaderConfig {
            color_number: 3, // 3-1=2 non-white per group
            creation_method: PixelCreationMethod::Additive,
            ..PaletteLoaderConfig::default()
        };

        let (groups, nb_color_pool) = PaletteLoader::build_ams_groups(&hex_list, &config).unwrap();

        assert_eq!(groups.len(), 2);
        assert_eq!(nb_color_pool, 2);
        // Every group must end with white
        for group in &groups {
            assert!(group.contains(&"#FFFFFF".to_string()));
        }
    }

    #[test]
    fn test_build_ams_groups_color_number_one_errors() {
        let hex_list = vec!["#FF0000".to_string(), "#FFFFFF".to_string()];
        let config = PaletteLoaderConfig {
            color_number: 1,
            creation_method: PixelCreationMethod::Additive,
            ..PaletteLoaderConfig::default()
        };
        let result = PaletteLoader::build_ams_groups(&hex_list, &config);
        assert!(result.is_err());
    }

    #[test]
    fn test_merge_groups_single_group_is_identity() {
        let layer =
            crate::palette::ColorLayer::from_cmyk("#FF0000".to_string(), 5, 0.0, 1.0, 1.0, 0.0);
        let group = vec![crate::palette::ColorCombi::new(layer)];
        let merged = PaletteLoader::merge_groups(vec![group.clone()]).unwrap();
        assert_eq!(merged.len(), group.len());
    }

    #[test]
    fn test_merge_groups_two_groups_cartesian_product() {
        let make_combi = |hex: &str| {
            let layer =
                crate::palette::ColorLayer::from_cmyk(hex.to_string(), 5, 0.0, 0.0, 0.0, 0.0);
            crate::palette::ColorCombi::new(layer)
        };
        let group_a = vec![make_combi("#FF0000"), make_combi("#00FF00")]; // 2
        let group_b = vec![make_combi("#0000FF"), make_combi("#FFFF00")]; // 2

        let merged = PaletteLoader::merge_groups(vec![group_a, group_b]).unwrap();

        assert_eq!(merged.len(), 4); // 2 × 2
                                     // Every result must contain exactly 2 colours (one from each group)
        assert!(merged.iter().all(|c| c.total_colors() == 2));
    }

    #[test]
    fn test_merge_groups_empty_returns_error() {
        let result = PaletteLoader::merge_groups(vec![]);
        assert!(result.is_err());
    }
}

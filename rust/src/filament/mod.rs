//! Filament assignment and mapping for 3MF export
//!
//! Zentrales Modul für die Zuordnung von Filamenten zu Objekten in 3MF-Dateien.
//! Sammelt einzigartige Filamentfarben aus den Layern, weist jedem Layer einen
//! Extruder-Slot zu und generiert die für Bambu Studio erforderlichen Konfigurationsdateien:
//!
//! - `Metadata/model_settings.config` — Objekt-zu-Extruder-Zuordnung
//! - `Metadata/project_settings.config` — Projekt-Filamente (Farben, Typen)

use crate::lithophane::layer::NamedLayer;

/// Mapping von Layern zu Filament-Slots für die 3MF-Ausgabe.
///
/// Wird aus den NamedLayern erstellt und speichert:
/// - Die geordnete Liste einzigartiger Farben (Filament-Slots)
/// - Die Zuordnung jedes Layers zu seinem Extruder-Index (1-basiert)
pub struct FilamentMapping {
    /// Unique hex colors in order of first appearance (these become filament slots)
    colors: Vec<String>,
    /// For each layer: the 1-based extruder index (defaults to 1 for layers without color)
    extruder_indices: Vec<u32>,
}

impl FilamentMapping {
    /// Erstellt eine Filament-Zuordnung aus den gegebenen Layern.
    ///
    /// Sammelt alle einzigartigen Hex-Farben (Reihenfolge des ersten Auftretens)
    /// und ordnet jedem Layer den passenden Extruder-Slot zu.
    /// Layer ohne Farbe (z.B. Grundplatte) erhalten Extruder 1 als Fallback.
    pub fn from_layers(layers: &[NamedLayer]) -> Self {
        let mut colors: Vec<String> = Vec::new();
        let mut extruder_indices: Vec<u32> = Vec::new();

        for layer in layers {
            let extruder = if let Some(ref hex) = layer.hex_color {
                // Find or add the color to our list
                let pos = colors.iter().position(|c| c == hex);
                let idx = match pos {
                    Some(i) => i,
                    None => {
                        colors.push(hex.clone());
                        colors.len() - 1
                    }
                };
                (idx + 1) as u32 // 1-based
            } else {
                1 // Fallback for layers without color (plate, etc.)
            };
            extruder_indices.push(extruder);
        }

        Self {
            colors,
            extruder_indices,
        }
    }

    /// Anzahl der einzigartigen Filament-Slots.
    pub fn filament_count(&self) -> usize {
        self.colors.len()
    }

    /// Die geordnete Liste der Filament-Farben (Hex-Codes, z.B. `"#FF0000"`).
    pub fn colors(&self) -> &[String] {
        &self.colors
    }

    /// Hex-Farbe als RGBA für Bambu Studio (z.B. `"#FF0000"` → `"#FF0000FF"`).
    fn hex_to_rgba(hex: &str) -> String {
        if hex.len() == 7 && hex.starts_with('#') {
            format!("{}FF", hex)
        } else {
            hex.to_string()
        }
    }

    /// Extruder-Index (1-basiert) für einen Layer.
    pub fn extruder_for_layer(&self, layer_index: usize) -> u32 {
        self.extruder_indices
            .get(layer_index)
            .copied()
            .unwrap_or(1)
    }

    /// Generiert `Metadata/model_settings.config` XML für Bambu Studio.
    ///
    /// Enthält:
    /// 1. `<object>` mit `<metadata key="extruder">` (Filamentslot pro Objekt)
    /// 2. `<part>` Sub-Elemente (Bambu erwartet mindestens ein Part)
    /// 3. `<plate>` mit `<filament>` Einträgen und `<model_instance>` Verknüpfungen
    ///
    /// Die object-IDs beginnen bei 2 (ID 1 = ColorGroup-Resource).
    pub fn generate_model_settings_config(&self, layers: &[NamedLayer]) -> String {
        let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<config>\n");

        // Object-Einträge mit part-Sub-Elementen (zwingend für Bambu-Erkennung)
        for (idx, layer) in layers.iter().enumerate() {
            let object_id = (idx + 2) as u32; // ID 1 = ColorGroup
            let extruder = self.extruder_for_layer(idx);
            xml.push_str(&format!(
                "  <object id=\"{object_id}\">\n\
                 \x20   <metadata key=\"name\" value=\"{name}\"/>\n\
                 \x20   <metadata key=\"extruder\" value=\"{extruder}\"/>\n\
                 \x20   <part id=\"1\" subtype=\"normal_part\">\n\
                 \x20     <metadata key=\"name\" value=\"{name}\"/>\n\
                 \x20     <metadata key=\"matrix\" value=\"1 0 0 0 0 1 0 0 0 0 1 0 0 0 0 1\"/>\n\
                 \x20     <metadata key=\"source_volume_id\" value=\"0\"/>\n\
                 \x20   </part>\n\
                 \x20 </object>\n",
                name = layer.name,
            ));
        }

        // Plate-Abschnitt
        let filament_map: String = (1..=self.colors.len())
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(" ");

        xml.push_str(&format!(
            "  <plate>\n\
             \x20   <metadata key=\"plater_id\" value=\"1\"/>\n\
             \x20   <metadata key=\"locked\" value=\"false\"/>\n\
             \x20   <metadata key=\"filament_map\" value=\"{filament_map}\"/>\n",
        ));

        // Filament-Einträge mit RGBA-Farben
        for (i, color) in self.colors.iter().enumerate() {
            let rgba = Self::hex_to_rgba(color);
            xml.push_str(&format!(
                "    <filament id=\"{}\" type=\"PLA\" color=\"{}\" used_m=\"0\" used_g=\"0\"/>\n",
                i + 1,
                rgba
            ));
        }

        // model_instance Verknüpfungen
        for (idx, _layer) in layers.iter().enumerate() {
            let object_id = (idx + 2) as u32;
            let identify_id = (idx + 1) as u32;
            xml.push_str(&format!(
                "    <model_instance>\n\
                 \x20     <metadata key=\"object_id\" value=\"{object_id}\"/>\n\
                 \x20     <metadata key=\"instance_id\" value=\"0\"/>\n\
                 \x20     <metadata key=\"identify_id\" value=\"{identify_id}\"/>\n\
                 \x20   </model_instance>\n"
            ));
        }
        xml.push_str("  </plate>\n");

        xml.push_str("</config>");
        xml
    }

    /// Generiert `Metadata/project_settings.config` für Bambu Studio.
    ///
    /// Diese Datei definiert die Projekt-Filamente auf Projektebene.
    /// Ohne diese Datei verwendet Bambu Studio die Standard-Druckerprofil-Filamente
    /// (typischerweise 4 Slots) und ignoriert zusätzliche Extruder-Zuweisungen.
    ///
    /// Format: INI-artiger Key-Value-Block, wie von BambuStudio/OrcaSlicer erwartet.
    pub fn generate_project_settings_config(&self) -> String {
        let mut config = String::new();

        // Header
        config.push_str("; generated by PIXEstL\n");

        // filament_colour: semicolon-separated RGBA hex values
        let colors_str: String = self
            .colors
            .iter()
            .map(|c| Self::hex_to_rgba(c))
            .collect::<Vec<_>>()
            .join(";");
        config.push_str(&format!("filament_colour = {}\n", colors_str));

        // filament_type: semicolon-separated type strings
        let types_str: String = self
            .colors
            .iter()
            .map(|_| "PLA")
            .collect::<Vec<_>>()
            .join(";");
        config.push_str(&format!("filament_type = {}\n", types_str));

        // filament_settings_id: generic preset IDs
        let settings_str: String = self
            .colors
            .iter()
            .map(|_| "\"Default Setting\"")
            .collect::<Vec<_>>()
            .join(";");
        config.push_str(&format!("filament_settings_id = {}\n", settings_str));

        config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lithophane::geometry::Mesh;

    #[test]
    fn test_filament_mapping_basic() {
        let mesh = Mesh::new();
        let layers = vec![
            NamedLayer::new(
                "layer-Red".to_string(),
                mesh.clone(),
                Some("#FF0000".to_string()),
            ),
            NamedLayer::new(
                "layer-Green".to_string(),
                mesh.clone(),
                Some("#00FF00".to_string()),
            ),
            NamedLayer::new("layer-plate".to_string(), mesh.clone(), None),
        ];

        let mapping = FilamentMapping::from_layers(&layers);

        assert_eq!(mapping.filament_count(), 2);
        assert_eq!(mapping.colors(), &["#FF0000", "#00FF00"]);
        assert_eq!(mapping.extruder_for_layer(0), 1); // Red → slot 1
        assert_eq!(mapping.extruder_for_layer(1), 2); // Green → slot 2
        assert_eq!(mapping.extruder_for_layer(2), 1); // plate → fallback 1
    }

    #[test]
    fn test_filament_mapping_many_colors() {
        let mesh = Mesh::new();
        let colors_list = [
            "#000000", "#0086D6", "#69B1CF", "#D7C599", "#E5008E", "#F5A0B8", "#FFEA00",
            "#FFFFFF",
        ];
        let mut layers: Vec<NamedLayer> = colors_list
            .iter()
            .enumerate()
            .map(|(i, c)| {
                NamedLayer::new(
                    format!("layer-{}", i + 1),
                    mesh.clone(),
                    Some(c.to_string()),
                )
            })
            .collect();
        layers.push(NamedLayer::new(
            "layer-plate".to_string(),
            mesh.clone(),
            None,
        ));

        let mapping = FilamentMapping::from_layers(&layers);

        assert_eq!(mapping.filament_count(), 8);
        for i in 0..8 {
            assert_eq!(mapping.extruder_for_layer(i), (i + 1) as u32);
        }
        assert_eq!(mapping.extruder_for_layer(8), 1); // plate fallback
    }

    #[test]
    fn test_hex_to_rgba() {
        assert_eq!(FilamentMapping::hex_to_rgba("#FF0000"), "#FF0000FF");
        assert_eq!(FilamentMapping::hex_to_rgba("#00FF00"), "#00FF00FF");
        assert_eq!(FilamentMapping::hex_to_rgba("#FFFFFF"), "#FFFFFFFF");
        // Already RGBA
        assert_eq!(FilamentMapping::hex_to_rgba("#FF0000FF"), "#FF0000FF");
    }

    #[test]
    fn test_model_settings_config_output() {
        let mesh = Mesh::new();
        let layers = vec![
            NamedLayer::new(
                "layer-Red".to_string(),
                mesh.clone(),
                Some("#FF0000".to_string()),
            ),
            NamedLayer::new(
                "layer-Green".to_string(),
                mesh.clone(),
                Some("#00FF00".to_string()),
            ),
            NamedLayer::new("layer-plate".to_string(), mesh.clone(), None),
        ];

        let mapping = FilamentMapping::from_layers(&layers);
        let xml = mapping.generate_model_settings_config(&layers);

        // Object-IDs und Extruder-Zuweisung
        assert!(xml.contains(r#"id="2""#));
        assert!(xml.contains(r#"key="extruder" value="1""#));
        assert!(xml.contains(r#"id="3""#));
        assert!(xml.contains(r#"key="extruder" value="2""#));
        assert!(xml.contains(r#"id="4""#));

        // Part-Sub-Elemente
        assert!(xml.contains(r#"subtype="normal_part""#));

        // Filament-Einträge mit RGBA
        assert!(
            xml.contains(r##"<filament id="1" type="PLA" color="#FF0000FF""##),
            "Filament 1 (Red) RGBA entry missing; config: {xml}"
        );
        assert!(
            xml.contains(r##"<filament id="2" type="PLA" color="#00FF00FF""##),
            "Filament 2 (Green) RGBA entry missing; config: {xml}"
        );

        // Plate und model_instance
        assert!(xml.contains("<plate>"));
        assert!(xml.contains("plater_id"));
        assert!(xml.contains("object_id"));
        assert!(xml.contains("identify_id"));
    }

    #[test]
    fn test_project_settings_config_output() {
        let mesh = Mesh::new();
        let layers = vec![
            NamedLayer::new(
                "layer-Red".to_string(),
                mesh.clone(),
                Some("#FF0000".to_string()),
            ),
            NamedLayer::new(
                "layer-Green".to_string(),
                mesh.clone(),
                Some("#00FF00".to_string()),
            ),
        ];

        let mapping = FilamentMapping::from_layers(&layers);
        let config = mapping.generate_project_settings_config();

        assert!(config.contains("filament_colour = #FF0000FF;#00FF00FF"));
        assert!(config.contains("filament_type = PLA;PLA"));
        assert!(config.contains("filament_settings_id = "));
    }

    #[test]
    fn test_duplicate_colors_share_slot() {
        let mesh = Mesh::new();
        let layers = vec![
            NamedLayer::new(
                "layer-A".to_string(),
                mesh.clone(),
                Some("#FF0000".to_string()),
            ),
            NamedLayer::new(
                "layer-B".to_string(),
                mesh.clone(),
                Some("#FF0000".to_string()),
            ),
        ];

        let mapping = FilamentMapping::from_layers(&layers);

        assert_eq!(mapping.filament_count(), 1);
        assert_eq!(mapping.extruder_for_layer(0), 1);
        assert_eq!(mapping.extruder_for_layer(1), 1);
    }
}

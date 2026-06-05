//! Filament-Slot-Zuordnung für den 3MF-Export
//!
//! Sammelt die einzigartigen Filamentfarben aus den Layern und weist jedem
//! Layer einen 1-basierten Extruder-Slot zu (Layer ohne Farbe erhalten Slot 1).
//!
//! Die Erzeugung der Bambu-Studio-Konfigurationsdateien
//! (`Metadata/model_settings.config`, `Metadata/project_settings.config`)
//! baut auf dieser Zuordnung auf und ist im 3MF-Exporter (`stl::threemf`)
//! angesiedelt.

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

    /// Extruder-Index (1-basiert) für einen Layer.
    pub fn extruder_for_layer(&self, layer_index: usize) -> u32 {
        self.extruder_indices.get(layer_index).copied().unwrap_or(1)
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
            "#000000", "#0086D6", "#69B1CF", "#D7C599", "#E5008E", "#F5A0B8", "#FFEA00", "#FFFFFF",
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

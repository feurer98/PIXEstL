//! 3MF-Export mit Farbmetadaten für Bambu Studio

use crate::error::{PixestlError, Result};
use crate::filament::FilamentMapping;
use crate::lithophane::layer::NamedLayer;
use crate::stl::StlFormat;
use std::collections::HashMap;

fn get_or_add_vertex(
    mesh: &mut lib3mf_core::model::Mesh,
    vertex_map: &mut HashMap<(u32, u32, u32), u32>,
    x: f32,
    y: f32,
    z: f32,
) -> u32 {
    let key = (x.to_bits(), y.to_bits(), z.to_bits());
    if let Some(&idx) = vertex_map.get(&key) {
        idx
    } else {
        let idx = mesh.add_vertex(x, y, z);
        vertex_map.insert(key, idx);
        idx
    }
}

/// Generiert das `Metadata/model_settings.config` XML für Bambu Studio.
///
/// Delegiert an `FilamentMapping::generate_model_settings_config()`.
/// Wird nur noch für Abwärtskompatibilität in Tests verwendet.
#[cfg(test)]
fn generate_model_settings_config(layers: &[NamedLayer], colors: &[&str]) -> String {
    let mapping = FilamentMapping::from_layers(layers);
    debug_assert_eq!(
        mapping
            .colors()
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>(),
        colors,
        "FilamentMapping color order mismatch"
    );
    mapping.generate_model_settings_config(layers)
}

/// Exportiert mehrere Layer in eine `.3mf`-Datei mit eingebetteten Farbmetadaten.
///
/// Jeder Layer mit einem `hex_color` wird als Objekt mit Farbzuweisung in der
/// 3MF-Datei gespeichert. Bambu Studio erkennt diese Farben beim Import
/// automatisch und ordnet sie den nächstgelegenen AMS-Slots zu.
///
/// Zusätzlich wird `Metadata/model_settings.config` generiert, damit Bambu Studio
/// die Filamentslot-Zuweisung pro Objekt korrekt lesen kann.
///
/// # Arguments
///
/// * `layers`      - Slice aus `NamedLayer`; jedes Element wird ein 3MF-Objekt
/// * `output_path` - Pfad zur Ausgabe-3MF-Datei
/// * `_format`     - Wird ignoriert; 3MF verwendet kein STL-Format intern
///
/// # Errors
///
/// Gibt `PixestlError::Io` oder `PixestlError::Export` zurück bei Schreibfehlern.
pub fn export_to_3mf<P: AsRef<std::path::Path>>(
    layers: &[NamedLayer],
    output_path: P,
    _format: StlFormat,
) -> Result<()> {
    use lib3mf_core::model::{
        BuildItem, Color, ColorGroup, Geometry, Mesh as Lib3mfMesh, Model, Object, ObjectType,
        ResourceId, Unit,
    };
    use std::fs::File;

    // Filament-Zuordnung zentral berechnen
    let filament_mapping = FilamentMapping::from_layers(layers);

    let mut model = Model {
        unit: Unit::Millimeter,
        ..Default::default()
    };

    let colors = filament_mapping.colors();

    let color_group_id = ResourceId(1);
    if !colors.is_empty() {
        let color_group = ColorGroup {
            id: color_group_id,
            colors: colors
                .iter()
                .map(|hex| Color::from_hex(hex).unwrap_or(Color::new(128, 128, 128, 255)))
                .collect(),
        };
        model
            .resources
            .add_color_group(color_group)
            .map_err(|e| PixestlError::Export(e.to_string()))?;
    }

    for (idx, layer) in layers.iter().enumerate() {
        let object_id = ResourceId((idx + 2) as u32);

        let mut mesh = Lib3mfMesh::new();
        let mut vertex_map: HashMap<(u32, u32, u32), u32> = HashMap::new();
        for tri in &layer.mesh.triangles {
            let v0 = get_or_add_vertex(
                &mut mesh,
                &mut vertex_map,
                tri.v0.x as f32,
                tri.v0.y as f32,
                tri.v0.z as f32,
            );
            let v1 = get_or_add_vertex(
                &mut mesh,
                &mut vertex_map,
                tri.v1.x as f32,
                tri.v1.y as f32,
                tri.v1.z as f32,
            );
            let v2 = get_or_add_vertex(
                &mut mesh,
                &mut vertex_map,
                tri.v2.x as f32,
                tri.v2.y as f32,
                tri.v2.z as f32,
            );
            mesh.add_triangle(v0, v1, v2);
        }

        let pindex = layer
            .hex_color
            .as_deref()
            .and_then(|hex| colors.iter().position(|c| c == hex))
            .map(|i| i as u32);
        let pid = pindex.map(|_| color_group_id);

        let object = Object {
            id: object_id,
            object_type: ObjectType::Model,
            name: Some(layer.name.clone()),
            part_number: None,
            uuid: None,
            pid,
            thumbnail: None,
            pindex,
            geometry: Geometry::Mesh(mesh),
        };
        model
            .resources
            .add_object(object)
            .map_err(|e| PixestlError::Export(e.to_string()))?;

        model.build.items.push(BuildItem {
            object_id,
            ..Default::default()
        });
    }

    // Bambu-Konfigurationsdateien als Attachments direkt ins Modell einfügen.
    // lib3mf-core schreibt model.attachments automatisch ins ZIP – kein Two-Pass nötig.
    let model_config_xml = filament_mapping.generate_model_settings_config(layers);
    let project_config = filament_mapping.generate_project_settings_config();

    model.attachments.insert(
        "Metadata/model_settings.config".to_string(),
        model_config_xml.into_bytes(),
    );
    // Ohne diese Datei verwendet Bambu Studio die Standard-Druckerprofil-Filamente
    // und ignoriert zusätzliche Extruder-Zuweisungen über 4 Slots hinaus.
    model.attachments.insert(
        "Metadata/project_settings.config".to_string(),
        project_config.into_bytes(),
    );

    let output_file = File::create(output_path).map_err(PixestlError::Io)?;
    model
        .write(output_file)
        .map_err(|e| PixestlError::Export(e.to_string()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lithophane::geometry::{Mesh, Triangle, Vector3};

    #[test]
    fn test_export_to_3mf_unit_and_z_range() {
        use std::io::Read;
        use tempfile::NamedTempFile;

        let mut mesh = Mesh::new();
        mesh.add_triangle(Triangle::new(
            Vector3::new(0.0, 0.0, 0.3),
            Vector3::new(1.0, 0.0, 0.3),
            Vector3::new(0.0, 1.0, 1.8),
        ));
        let layers = vec![NamedLayer::new(
            "test".to_string(),
            mesh,
            Some("#FF0000".to_string()),
        )];

        let tmp = NamedTempFile::new().unwrap();
        export_to_3mf(&layers, tmp.path(), StlFormat::Binary).unwrap();

        let file = std::fs::File::open(tmp.path()).unwrap();
        let mut zip = zip::ZipArchive::new(file).unwrap();
        let mut model_file = zip.by_name("3D/3dmodel.model").unwrap();
        let mut content = String::new();
        model_file.read_to_string(&mut content).unwrap();

        assert!(
            content.contains("unit=\"millimeter\""),
            "3MF must declare unit=millimeter; got: {}",
            &content[..500.min(content.len())]
        );
        assert!(
            content.contains("z=\"1.8"),
            "Max Z vertex should be ~1.8mm; content snippet: {}",
            &content[..500.min(content.len())]
        );
    }

    #[test]
    fn test_export_to_3mf_bambu_model_settings_config() {
        use std::io::Read;
        use tempfile::NamedTempFile;

        let mut mesh = Mesh::new();
        mesh.add_triangle(Triangle::new(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
        ));
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

        let tmp = NamedTempFile::new().unwrap();
        export_to_3mf(&layers, tmp.path(), StlFormat::Binary).unwrap();

        let file = std::fs::File::open(tmp.path()).unwrap();
        let mut zip = zip::ZipArchive::new(file).unwrap();

        // model_settings.config muss existieren
        let config = {
            let mut config_file = zip.by_name("Metadata/model_settings.config").unwrap();
            let mut config = String::new();
            config_file.read_to_string(&mut config).unwrap();
            config
        };

        // Objekt 2 = layer-Red → extruder 1
        assert!(config.contains(r#"id="2""#), "Object id=2 must exist");
        assert!(
            config.contains(r#"key="extruder" value="1""#),
            "layer-Red should be extruder 1; config: {config}"
        );
        // Objekt 3 = layer-Green → extruder 2
        assert!(config.contains(r#"id="3""#), "Object id=3 must exist");
        assert!(
            config.contains(r#"key="extruder" value="2""#),
            "layer-Green should be extruder 2; config: {config}"
        );
        // Objekt 4 = layer-plate (kein hex_color) → extruder 1 (Fallback)
        assert!(config.contains(r#"id="4""#), "Object id=4 must exist");

        // Filament-Einträge für Bambu Studio Slot-Erkennung (RGBA)
        assert!(
            config.contains(r##"<filament id="1" type="PLA" color="#FF0000FF""##),
            "Filament 1 (Red) RGBA entry missing; config: {config}"
        );
        assert!(
            config.contains(r##"<filament id="2" type="PLA" color="#00FF00FF""##),
            "Filament 2 (Green) RGBA entry missing; config: {config}"
        );

        // project_settings.config muss existieren (für Filament-Erkennung in Bambu Studio)
        let project_config = {
            let mut project_file = zip
                .by_name("Metadata/project_settings.config")
                .expect("project_settings.config must exist in 3MF");
            let mut buf = String::new();
            project_file.read_to_string(&mut buf).unwrap();
            buf
        };

        // project_settings.config ist JSON (nicht INI)
        let json: serde_json::Value = serde_json::from_str(&project_config)
            .expect("project_settings.config must be valid JSON");
        let colours = json["filament_colour"]
            .as_array()
            .expect("filament_colour must be a JSON array");
        assert_eq!(colours.len(), 2, "Expected 2 filament colors");
        assert_eq!(colours[0].as_str().unwrap(), "#FF0000FF");
        assert_eq!(colours[1].as_str().unwrap(), "#00FF00FF");
        let types = json["filament_type"]
            .as_array()
            .expect("filament_type must be a JSON array");
        assert_eq!(types.len(), 2);
        assert_eq!(types[0].as_str().unwrap(), "PLA");
    }

    #[test]
    fn test_generate_model_settings_config() {
        let mesh = Mesh::new();
        let layers = vec![
            NamedLayer::new(
                "layer-A".to_string(),
                mesh.clone(),
                Some("#AA0000".to_string()),
            ),
            NamedLayer::new(
                "layer-B".to_string(),
                mesh.clone(),
                Some("#00BB00".to_string()),
            ),
            NamedLayer::new("layer-plate".to_string(), mesh.clone(), None),
        ];
        let colors = vec!["#AA0000", "#00BB00"];
        let xml = generate_model_settings_config(&layers, &colors);

        // Objekt-IDs und Extruder-Zuweisung
        assert!(xml.contains(r#"id="2""#));
        assert!(xml.contains(r#"value="1""#)); // extruder 1 für layer-A
        assert!(xml.contains(r#"id="3""#));
        assert!(xml.contains(r#"value="2""#)); // extruder 2 für layer-B
        assert!(xml.contains(r#"id="4""#)); // layer-plate, extruder=1 Fallback

        // Part-Sub-Elemente müssen vorhanden sein (Bambu-Pflicht)
        assert!(xml.contains(r#"subtype="normal_part""#));
        assert!(xml.contains("source_volume_id"));
        assert!(xml.contains("matrix"));

        // Plate-Abschnitt mit model_instance (Bambu-Verknüpfung)
        assert!(xml.contains("<plate>"));
        assert!(xml.contains("plater_id"));
        assert!(xml.contains("object_id"));
        assert!(xml.contains("identify_id"));

        // Filament-Einträge müssen im Plate-Abschnitt vorhanden sein (RGBA)
        assert!(
            xml.contains(r##"<filament id="1" type="PLA" color="#AA0000FF""##),
            "Filament 1 entry missing; config: {xml}"
        );
        assert!(
            xml.contains(r##"<filament id="2" type="PLA" color="#00BB00FF""##),
            "Filament 2 entry missing; config: {xml}"
        );

        assert!(xml.starts_with("<?xml"));
        assert!(xml.contains("<config>"));
        assert!(xml.ends_with("</config>"));
    }

    #[test]
    fn test_generate_model_settings_config_many_colors() {
        let mesh = Mesh::new();
        let colors_list = [
            "#000000", "#0086D6", "#69B1CF", "#F5A0B8", "#D7C599", "#E5008E", "#FFEA00", "#FFFFFF",
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

        let colors: Vec<&str> = colors_list.to_vec();
        let xml = generate_model_settings_config(&layers, &colors);

        // Alle 8 Extruder-Werte müssen korrekt zugewiesen sein
        for i in 1..=8 {
            assert!(
                xml.contains(&format!(r#"value="{i}""#)),
                "Extruder value {i} missing; config: {xml}"
            );
        }

        // Alle 8 Filament-Einträge müssen im Plate-Abschnitt vorhanden sein (RGBA)
        for (i, color) in colors_list.iter().enumerate() {
            let rgba = if color.len() == 7 {
                format!("{}FF", color)
            } else {
                color.to_string()
            };
            assert!(
                xml.contains(&format!(
                    r#"<filament id="{}" type="PLA" color="{}""#,
                    i + 1,
                    rgba
                )),
                "Filament {} entry missing; config: {xml}",
                i + 1
            );
        }

        // Plate-Objekt (ohne Farbe) fällt auf extruder=1 zurück
        let plate_object_id = (colors_list.len() + 2) as u32;
        assert!(
            xml.contains(&format!(r#"id="{plate_object_id}""#)),
            "Plate object id={plate_object_id} missing; config: {xml}"
        );
    }

    #[test]
    fn test_export_to_3mf_vertex_deduplication() {
        // A single quad (2 triangles sharing 2 vertices)
        let mut mesh = Mesh::new();
        mesh.add_triangle(Triangle::new(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
        ));
        mesh.add_triangle(Triangle::new(
            Vector3::new(1.0, 1.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
        ));
        let layers = vec![NamedLayer::new("quad".to_string(), mesh, None)];

        // Build the lib3mf mesh via the same dedup logic
        use lib3mf_core::model::Mesh as Lib3mfMesh;
        let mut lib_mesh = Lib3mfMesh::new();
        let mut vertex_map: HashMap<(u32, u32, u32), u32> = HashMap::new();
        for layer in &layers {
            for tri in &layer.mesh.triangles {
                let _v0 = get_or_add_vertex(
                    &mut lib_mesh,
                    &mut vertex_map,
                    tri.v0.x as f32,
                    tri.v0.y as f32,
                    tri.v0.z as f32,
                );
                let _v1 = get_or_add_vertex(
                    &mut lib_mesh,
                    &mut vertex_map,
                    tri.v1.x as f32,
                    tri.v1.y as f32,
                    tri.v1.z as f32,
                );
                let _v2 = get_or_add_vertex(
                    &mut lib_mesh,
                    &mut vertex_map,
                    tri.v2.x as f32,
                    tri.v2.y as f32,
                    tri.v2.z as f32,
                );
            }
        }
        // 2 triangles × 3 unique verts = 4 unique positions (not 6 without dedup)
        assert_eq!(
            lib_mesh.vertices.len(),
            4,
            "Expected 4 unique vertices for a quad, got {}",
            lib_mesh.vertices.len()
        );
    }
}

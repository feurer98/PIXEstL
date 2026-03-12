//! STL-Dateiexport (ASCII und Binär)
//!
//! Dieses Modul schreibt 3D-Meshes im STL-Format, das von den meisten
//! 3D-Druckern und Slicern unterstützt wird.
//!
//! - **ASCII-STL**: Lesbar, aber größer (~10× mehr Speicher als Binär)
//! - **Binär-STL**: Kompaktes Format, geringere Dateigröße
//!
//! Für die Ausgabe mehrerer Layer (Farbschichten + Textur) wird ein ZIP-Archiv
//! erstellt, das je eine `.stl`-Datei pro Layer enthält.

use crate::error::{PixestlError, Result};
use crate::filament::FilamentMapping;
use crate::lithophane::geometry::{Mesh, Triangle, Vector3};
use crate::lithophane::layer::NamedLayer;
use std::collections::HashMap;
use std::io::Write;

/// STL-Ausgabeformat
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StlFormat {
    /// Menschenlesbares Text-Format
    Ascii,
    /// Kompaktes Binärformat (empfohlen)
    Binary,
}

/// Schreibt ein 3D-Mesh als STL-Datei in den angegebenen Writer.
///
/// # Arguments
///
/// * `mesh`   - Das zu exportierende 3D-Mesh
/// * `writer` - Ziel-Writer (z.B. `File`, `Vec<u8>`)
/// * `format` - Ausgabeformat (`Ascii` oder `Binary`)
/// * `name`   - Name des Solids (erscheint im STL-Header)
///
/// # Errors
///
/// Gibt `PixestlError::Io` zurück, wenn das Schreiben fehlschlägt.
///
/// # Returns
///
/// `Ok(())` bei Erfolg.
pub fn write_stl<W: Write>(
    mesh: &Mesh,
    writer: &mut W,
    format: StlFormat,
    name: &str,
) -> Result<()> {
    match format {
        StlFormat::Ascii => write_ascii_stl(mesh, writer, name),
        StlFormat::Binary => write_binary_stl(mesh, writer, name),
    }
}

fn write_ascii_stl<W: Write>(mesh: &Mesh, writer: &mut W, name: &str) -> Result<()> {
    writeln!(writer, "solid {}", name).map_err(PixestlError::Io)?;

    for triangle in &mesh.triangles {
        write_ascii_triangle(writer, triangle)?;
    }

    writeln!(writer, "endsolid {}", name).map_err(PixestlError::Io)?;
    Ok(())
}

fn write_ascii_triangle<W: Write>(writer: &mut W, triangle: &Triangle) -> Result<()> {
    let normal = triangle.normal();
    writeln!(
        writer,
        "facet normal {} {} {}",
        normal.x, normal.y, normal.z
    )
    .map_err(PixestlError::Io)?;
    writeln!(writer, "  outer loop").map_err(PixestlError::Io)?;
    write_ascii_vertex(writer, &triangle.v0)?;
    write_ascii_vertex(writer, &triangle.v1)?;
    write_ascii_vertex(writer, &triangle.v2)?;
    writeln!(writer, "  endloop").map_err(PixestlError::Io)?;
    writeln!(writer, "endfacet").map_err(PixestlError::Io)?;
    Ok(())
}

fn write_ascii_vertex<W: Write>(writer: &mut W, vertex: &Vector3) -> Result<()> {
    writeln!(writer, "    vertex {} {} {}", vertex.x, vertex.y, vertex.z)
        .map_err(PixestlError::Io)?;
    Ok(())
}

fn write_binary_stl<W: Write>(mesh: &Mesh, writer: &mut W, name: &str) -> Result<()> {
    let mut header = [0u8; 80];
    let name_bytes = name.as_bytes();
    let copy_len = name_bytes.len().min(80);
    header[..copy_len].copy_from_slice(&name_bytes[..copy_len]);
    writer.write_all(&header).map_err(PixestlError::Io)?;

    let triangle_count = mesh.triangles.len() as u32;
    writer
        .write_all(&triangle_count.to_le_bytes())
        .map_err(PixestlError::Io)?;

    for triangle in &mesh.triangles {
        write_binary_triangle(writer, triangle)?;
    }
    Ok(())
}

fn write_binary_triangle<W: Write>(writer: &mut W, triangle: &Triangle) -> Result<()> {
    let normal = triangle.normal();
    write_f32_vec3(writer, &normal)?;
    write_f32_vec3(writer, &triangle.v0)?;
    write_f32_vec3(writer, &triangle.v1)?;
    write_f32_vec3(writer, &triangle.v2)?;
    writer.write_all(&[0u8, 0u8]).map_err(PixestlError::Io)?;
    Ok(())
}

fn write_f32_vec3<W: Write>(writer: &mut W, vec: &Vector3) -> Result<()> {
    writer
        .write_all(&(vec.x as f32).to_le_bytes())
        .map_err(PixestlError::Io)?;
    writer
        .write_all(&(vec.y as f32).to_le_bytes())
        .map_err(PixestlError::Io)?;
    writer
        .write_all(&(vec.z as f32).to_le_bytes())
        .map_err(PixestlError::Io)?;
    Ok(())
}

/// Exportiert mehrere Layer als einzelne `.stl`-Dateien in ein Verzeichnis.
///
/// Das Verzeichnis wird erstellt, falls es noch nicht existiert.
/// Jeder Layer erhält eine eigene Datei `<name>.stl`.
///
/// # Arguments
///
/// * `layers`   - Slice aus `(Name, Mesh)`-Paaren
/// * `dir_path` - Pfad zum Ausgabeverzeichnis
/// * `format`   - STL-Ausgabeformat für alle Layer
///
/// # Errors
///
/// Gibt `PixestlError::Io` zurück, wenn das Verzeichnis nicht erstellt oder
/// eine Datei nicht geschrieben werden kann.
pub fn export_to_dir<P: AsRef<std::path::Path>>(
    layers: &[NamedLayer],
    dir_path: P,
    format: StlFormat,
) -> Result<()> {
    use std::fs;

    let dir = dir_path.as_ref();
    fs::create_dir_all(dir).map_err(PixestlError::Io)?;

    for layer in layers {
        let path = dir.join(format!("{}.stl", layer.name));
        let mut file = fs::File::create(&path).map_err(PixestlError::Io)?;
        write_stl(&layer.mesh, &mut file, format, &layer.name)?;
    }

    Ok(())
}

/// Exportiert mehrere Layer (je eine STL-Datei) in ein ZIP-Archiv.
///
/// Jeder Layer wird als eigene `.stl`-Datei im Archiv abgelegt.
/// Das ZIP-Archiv wird mit Deflate-Kompression erstellt.
///
/// # Arguments
///
/// * `layers`      - Slice aus `(Name, Mesh)`-Paaren; jedes Paar wird eine `.stl`-Datei
/// * `output_path` - Pfad zur Ausgabe-ZIP-Datei
/// * `format`      - STL-Ausgabeformat für alle Layer
///
/// # Errors
///
/// Gibt `PixestlError::Io` zurück, wenn die Datei nicht erstellt werden kann,
/// oder `PixestlError::Zip` bei ZIP-Schreibfehlern.
///
/// # Returns
///
/// `Ok(())` bei Erfolg.
pub fn export_to_zip<P: AsRef<std::path::Path>>(
    layers: &[NamedLayer],
    output_path: P,
    format: StlFormat,
) -> Result<()> {
    use std::fs::File;
    use zip::write::SimpleFileOptions;
    use zip::ZipWriter;

    let file = File::create(output_path).map_err(PixestlError::Io)?;
    let mut zip = ZipWriter::new(file);

    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    for layer in layers {
        let filename = format!("{}.stl", layer.name);
        zip.start_file(filename, options)
            .map_err(PixestlError::Zip)?;
        write_stl(&layer.mesh, &mut zip, format, &layer.name)?;
    }

    zip.finish().map_err(PixestlError::Zip)?;
    Ok(())
}

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
    // Legacy-Wrapper: baut ein FilamentMapping aus den übergebenen Farben
    let mapping = FilamentMapping::from_layers(layers);
    // Verify that the mapping produces the same color order
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
/// Gibt `PixestlError::Io` oder `PixestlError::Other` zurück bei Schreibfehlern.
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
    use std::io::Cursor;

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
            .map_err(|e| PixestlError::Other(e.to_string()))?;
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
            .map_err(|e| PixestlError::Other(e.to_string()))?;

        model.build.items.push(BuildItem {
            object_id,
            transform: glam::Mat4::IDENTITY,
            part_number: None,
            uuid: None,
            path: None,
            printable: None,
        });
    }

    // Schritt 1: 3MF-Modell in Puffer schreiben (Two-Pass für Bambu-Metadaten)
    let mut buf: Vec<u8> = Vec::new();
    model
        .write(Cursor::new(&mut buf))
        .map_err(|e| PixestlError::Other(e.to_string()))?;

    // Schritt 2: Bambu-Konfigurationsdateien generieren
    let model_config_xml = filament_mapping.generate_model_settings_config(layers);
    let project_config = filament_mapping.generate_project_settings_config();

    // Schritt 3: Puffer als ZIP öffnen, alle Einträge in Ausgabe-ZIP kopieren
    //            und Bambu-Metadaten hinzufügen
    let output_file = File::create(output_path).map_err(PixestlError::Io)?;
    let mut zip_out = zip::ZipWriter::new(output_file);
    let mut zip_in = zip::ZipArchive::new(Cursor::new(buf)).map_err(PixestlError::Zip)?;

    for i in 0..zip_in.len() {
        let entry = zip_in.by_index_raw(i).map_err(PixestlError::Zip)?;
        zip_out.raw_copy_file(entry).map_err(PixestlError::Zip)?;
    }

    let options =
        zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);

    // model_settings.config: Objekt-zu-Extruder-Zuordnung
    zip_out
        .start_file("Metadata/model_settings.config", options)
        .map_err(PixestlError::Zip)?;
    zip_out
        .write_all(model_config_xml.as_bytes())
        .map_err(PixestlError::Io)?;

    // project_settings.config: Projekt-Filamente (Farben, Typen)
    // Ohne diese Datei verwendet Bambu Studio die Standard-Druckerprofil-Filamente
    // und ignoriert zusätzliche Extruder-Zuweisungen über 4 Slots hinaus.
    zip_out
        .start_file("Metadata/project_settings.config", options)
        .map_err(PixestlError::Zip)?;
    zip_out
        .write_all(project_config.as_bytes())
        .map_err(PixestlError::Io)?;

    zip_out.finish().map_err(PixestlError::Zip)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_ascii_stl_empty() {
        let mesh = Mesh::new();
        let mut output = Vec::new();
        write_ascii_stl(&mesh, &mut output, "test").unwrap();
        let result = String::from_utf8(output).unwrap();
        assert!(result.contains("solid test"));
        assert!(result.contains("endsolid test"));
    }

    #[test]
    fn test_write_ascii_stl_single_triangle() {
        let mut mesh = Mesh::new();
        mesh.add_triangle(Triangle::new(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
        ));
        let mut output = Vec::new();
        write_ascii_stl(&mesh, &mut output, "test").unwrap();
        let result = String::from_utf8(output).unwrap();
        assert!(result.contains("solid test"));
        assert!(result.contains("facet normal"));
        assert!(result.contains("vertex 0 0 0"));
        assert!(result.contains("vertex 1 0 0"));
        assert!(result.contains("vertex 0 1 0"));
        assert!(result.contains("endfacet"));
    }

    #[test]
    fn test_write_binary_stl_empty() {
        let mesh = Mesh::new();
        let mut output = Vec::new();
        write_binary_stl(&mesh, &mut output, "test").unwrap();
        assert_eq!(output.len(), 84);
        let count = u32::from_le_bytes([output[80], output[81], output[82], output[83]]);
        assert_eq!(count, 0);
    }

    #[test]
    fn test_write_binary_stl_single_triangle() {
        let mut mesh = Mesh::new();
        mesh.add_triangle(Triangle::new(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
        ));
        let mut output = Vec::new();
        write_binary_stl(&mesh, &mut output, "test").unwrap();
        assert_eq!(output.len(), 134);
        let count = u32::from_le_bytes([output[80], output[81], output[82], output[83]]);
        assert_eq!(count, 1);
    }

    #[test]
    fn test_write_ascii_cube() {
        let mesh = Mesh::cube(2.0, 2.0, 2.0, Vector3::zero());
        let mut output = Vec::new();
        write_ascii_stl(&mesh, &mut output, "cube").unwrap();
        let result = String::from_utf8(output).unwrap();
        assert_eq!(result.matches("facet normal").count(), 12);
    }

    #[test]
    fn test_stl_format_equality() {
        assert_eq!(StlFormat::Ascii, StlFormat::Ascii);
        assert_ne!(StlFormat::Ascii, StlFormat::Binary);
    }

    #[test]
    fn test_write_stl_wrappers() {
        let mesh = Mesh::new();
        let mut output = Vec::new();
        write_stl(&mesh, &mut output, StlFormat::Ascii, "test").unwrap();
        assert!(String::from_utf8(output).unwrap().contains("solid test"));

        let mut output = Vec::new();
        write_stl(&mesh, &mut output, StlFormat::Binary, "test").unwrap();
        assert_eq!(output.len(), 84);
    }

    #[test]
    fn test_export_to_3mf_unit_and_z_range() {
        use crate::lithophane::layer::NamedLayer;
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
        use crate::lithophane::layer::NamedLayer;
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
        // object_id für plate = 10 (8 Farblayer + 1 plate, +2 offset → 11, aber plate ist index 8 → 10)
        let plate_object_id = (colors_list.len() + 2) as u32;
        assert!(
            xml.contains(&format!(r#"id="{plate_object_id}""#)),
            "Plate object id={plate_object_id} missing; config: {xml}"
        );
    }

    #[test]
    fn test_export_to_3mf_vertex_deduplication() {
        use crate::lithophane::layer::NamedLayer;

        // A single quad (2 triangles sharing 2 vertices)
        let mut mesh = Mesh::new();
        // Triangle 1: (0,0,0) (1,0,0) (0,1,0)
        // Triangle 2: (1,1,0) (1,0,0) (0,1,0)  <- shares v1=(1,0,0) and v2=(0,1,0)
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

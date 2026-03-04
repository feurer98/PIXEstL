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

/// Exportiert mehrere Layer in eine `.3mf`-Datei mit eingebetteten Farbmetadaten.
///
/// Jeder Layer mit einem `hex_color` wird als Objekt mit Farbzuweisung in der
/// 3MF-Datei gespeichert. Bambu Studio erkennt diese Farben beim Import
/// automatisch und ordnet sie den nächstgelegenen AMS-Slots zu.
///
/// Layer ohne Farbe (Grundplatte, Textur) werden als farblose Objekte exportiert.
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

    let mut model = Model {
        unit: Unit::Millimeter,
        ..Default::default()
    };

    // Unique Farben sammeln (Reihenfolge erster Auftreten)
    let mut colors: Vec<&str> = Vec::new();
    for layer in layers {
        if let Some(ref hex) = layer.hex_color {
            if !colors.contains(&hex.as_str()) {
                colors.push(hex.as_str());
            }
        }
    }

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
            .and_then(|hex| colors.iter().position(|&c| c == hex))
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

    let file = File::create(output_path).map_err(PixestlError::Io)?;
    model
        .write(file)
        .map_err(|e| PixestlError::Other(e.to_string()))?;
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

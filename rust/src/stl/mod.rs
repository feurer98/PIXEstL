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

const CONTENT_TYPES_XML: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="model" ContentType="application/vnd.ms-package.3dmanufacturing-3dmodel+xml"/>
</Types>"#;

const RELS_XML: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Type="http://schemas.microsoft.com/3dmanufacturing/2013/01/3dmodel"
    Target="/3D/3dmodel.model" Id="rel0"/>
</Relationships>"#;

/// Exportiert mehrere Layer in eine `.3mf`-Datei mit eingebetteten Farbmetadaten.
///
/// Jeder Layer mit einem `hex_color` wird als `<object>` mit `pid`/`pindex`-Attribut
/// in der 3MF-Datei gespeichert. Bambu Studio erkennt diese Farben beim Import
/// automatisch und ordnet sie den nächstgelegenen AMS-Slots zu.
///
/// Layer ohne Farbe (Grundplatte, Textur) werden als farblose Objekte exportiert.
///
/// # Arguments
///
/// * `layers`      - Slice aus `NamedLayer`; jedes Element wird ein 3MF-Objekt
/// * `output_path` - Pfad zur Ausgabe-3MF-Datei
/// * `_format`     - Wird ignoriert; 3MF-Objekte werden immer als ASCII-XML exportiert
///
/// # Errors
///
/// Gibt `PixestlError::Io` oder `PixestlError::Zip` zurück bei Schreibfehlern.
pub fn export_to_3mf<P: AsRef<std::path::Path>>(
    layers: &[NamedLayer],
    output_path: P,
    _format: StlFormat,
) -> Result<()> {
    use std::fs::File;
    use zip::write::SimpleFileOptions;
    use zip::ZipWriter;

    let file = File::create(output_path).map_err(PixestlError::Io)?;
    let mut zip = ZipWriter::new(file);

    let opts = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    zip.start_file("[Content_Types].xml", opts).map_err(PixestlError::Zip)?;
    zip.write_all(CONTENT_TYPES_XML.as_bytes()).map_err(PixestlError::Io)?;

    zip.start_file("_rels/.rels", opts).map_err(PixestlError::Zip)?;
    zip.write_all(RELS_XML.as_bytes()).map_err(PixestlError::Io)?;

    zip.start_file("3D/3dmodel.model", opts).map_err(PixestlError::Zip)?;
    write_3dmodel(&mut zip, layers)?;

    zip.finish().map_err(PixestlError::Zip)?;
    Ok(())
}

/// Schreibt das 3MF-Hauptdokument (`3dmodel.model`) in den Writer.
fn write_3dmodel<W: Write>(writer: &mut W, layers: &[NamedLayer]) -> Result<()> {
    // Collect unique hex colors in first-seen order.
    // Using a Vec for lookup is fine: filament count is always small (≤32).
    let mut colors: Vec<&str> = Vec::new();
    for layer in layers {
        if let Some(ref hex) = layer.hex_color {
            if !colors.contains(&hex.as_str()) {
                colors.push(hex.as_str());
            }
        }
    }

    writeln!(writer, r#"<?xml version="1.0" encoding="utf-8"?>"#).map_err(PixestlError::Io)?;
    writeln!(
        writer,
        r#"<model unit="millimeter" xml:lang="en-US""#
    ).map_err(PixestlError::Io)?;
    writeln!(
        writer,
        r#"  xmlns="http://schemas.microsoft.com/3dmanufacturing/core/2015/02""#
    ).map_err(PixestlError::Io)?;
    writeln!(
        writer,
        r#"  xmlns:m="http://schemas.microsoft.com/3dmanufacturing/material/2015/02">"#
    ).map_err(PixestlError::Io)?;
    writeln!(writer, "  <resources>").map_err(PixestlError::Io)?;

    if !colors.is_empty() {
        writeln!(writer, r#"    <m:colorgroup id="1">"#).map_err(PixestlError::Io)?;
        for color in &colors {
            writeln!(writer, r#"      <m:color color="{color}"/>"#).map_err(PixestlError::Io)?;
        }
        writeln!(writer, "    </m:colorgroup>").map_err(PixestlError::Io)?;
    }

    for (idx, layer) in layers.iter().enumerate() {
        let object_id = idx + 2; // id 1 is reserved for the colorgroup

        let color_attrs = layer.hex_color.as_deref().and_then(|hex| {
            colors.iter().position(|&c| c == hex).map(|pindex| {
                format!(r#" pid="1" pindex="{pindex}""#)
            })
        }).unwrap_or_default();

        writeln!(
            writer,
            r#"    <object id="{object_id}" type="model"{color_attrs}>"#
        ).map_err(PixestlError::Io)?;
        writeln!(writer, "      <mesh>").map_err(PixestlError::Io)?;

        writeln!(writer, "        <vertices>").map_err(PixestlError::Io)?;
        for tri in &layer.mesh.triangles {
            for v in [&tri.v0, &tri.v1, &tri.v2] {
                writeln!(
                    writer,
                    r#"          <vertex x="{}" y="{}" z="{}"/>"#,
                    v.x, v.y, v.z
                ).map_err(PixestlError::Io)?;
            }
        }
        writeln!(writer, "        </vertices>").map_err(PixestlError::Io)?;

        writeln!(writer, "        <triangles>").map_err(PixestlError::Io)?;
        for (i, _) in layer.mesh.triangles.iter().enumerate() {
            let base = i * 3;
            writeln!(
                writer,
                r#"          <triangle v1="{}" v2="{}" v3="{}"/>"#,
                base, base + 1, base + 2
            ).map_err(PixestlError::Io)?;
        }
        writeln!(writer, "        </triangles>").map_err(PixestlError::Io)?;

        writeln!(writer, "      </mesh>").map_err(PixestlError::Io)?;
        writeln!(writer, "    </object>").map_err(PixestlError::Io)?;
    }

    writeln!(writer, "  </resources>").map_err(PixestlError::Io)?;
    writeln!(writer, "  <build>").map_err(PixestlError::Io)?;
    for (idx, _) in layers.iter().enumerate() {
        writeln!(writer, r#"    <item objectid="{}"/>"#, idx + 2).map_err(PixestlError::Io)?;
    }
    writeln!(writer, "  </build>").map_err(PixestlError::Io)?;
    writeln!(writer, "</model>").map_err(PixestlError::Io)?;

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
}

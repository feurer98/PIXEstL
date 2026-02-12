//! STL file export

use crate::error::{PixestlError, Result};
use crate::lithophane::geometry::{Mesh, Triangle, Vector3};
use std::io::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StlFormat {
    Ascii,
    Binary,
}

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
    writeln!(writer, "facet normal {} {} {}", normal.x, normal.y, normal.z)
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
    writer.write_all(&triangle_count.to_le_bytes()).map_err(PixestlError::Io)?;

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
    writer.write_all(&(vec.x as f32).to_le_bytes()).map_err(PixestlError::Io)?;
    writer.write_all(&(vec.y as f32).to_le_bytes()).map_err(PixestlError::Io)?;
    writer.write_all(&(vec.z as f32).to_le_bytes()).map_err(PixestlError::Io)?;
    Ok(())
}

pub fn export_to_zip<P: AsRef<std::path::Path>>(
    layers: &[(String, Mesh)],
    output_path: P,
    format: StlFormat,
) -> Result<()> {
    use std::fs::File;
    use zip::write::SimpleFileOptions;
    use zip::ZipWriter;

    let file = File::create(output_path).map_err(PixestlError::Io)?;
    let mut zip = ZipWriter::new(file);

    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);

    for (layer_name, mesh) in layers {
        let filename = format!("{}.stl", layer_name);
        zip.start_file(filename, options).map_err(PixestlError::Zip)?;
        write_stl(mesh, &mut zip, format, layer_name)?;
    }

    zip.finish().map_err(PixestlError::Zip)?;
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

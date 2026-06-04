//! Export in Verzeichnisse und ZIP-Archive

use crate::error::{PixestlError, Result};
use crate::lithophane::layer::NamedLayer;
use crate::stl::writer::write_stl;
use crate::stl::StlFormat;

/// Exportiert mehrere Layer als einzelne `.stl`-Dateien in ein Verzeichnis.
///
/// Das Verzeichnis wird erstellt, falls es noch nicht existiert.
/// Jeder Layer erhält eine eigene Datei `<name>.stl`.
///
/// # Arguments
///
/// * `layers`   - Slice aus `NamedLayer`-Einträgen
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
/// * `layers`      - Slice aus `NamedLayer`-Einträgen
/// * `output_path` - Pfad zur Ausgabe-ZIP-Datei
/// * `format`      - STL-Ausgabeformat für alle Layer
///
/// # Errors
///
/// Gibt `PixestlError::Io` zurück, wenn die Datei nicht erstellt werden kann,
/// oder `PixestlError::Zip` bei ZIP-Schreibfehlern.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lithophane::geometry::{Mesh, Vector3};

    #[test]
    fn test_export_to_dir_creates_directory_and_files() {
        let mesh = Mesh::cube(1.0, 1.0, 1.0, Vector3::zero());
        let layers = vec![
            NamedLayer::new(
                "color".to_string(),
                mesh.clone(),
                Some("#FF0000".to_string()),
            ),
            NamedLayer::without_color("plate".to_string(), mesh),
        ];

        let dir = tempfile::tempdir().unwrap();
        let out = dir.path().join("stl_subdir");
        export_to_dir(&layers, &out, StlFormat::Binary).unwrap();

        assert!(out.join("color.stl").exists());
        assert!(out.join("plate.stl").exists());
    }

    #[test]
    fn test_export_to_zip_empty_layers() {
        let layers: Vec<NamedLayer> = vec![];
        let tmp = tempfile::NamedTempFile::new().unwrap();
        export_to_zip(&layers, tmp.path(), StlFormat::Binary).unwrap();

        let file = std::fs::File::open(tmp.path()).unwrap();
        let archive = zip::ZipArchive::new(file).unwrap();
        assert_eq!(archive.len(), 0);
    }
}

//! STL-Dateiexport (ASCII und Binär), ZIP-Archive und 3MF
//!
//! Dieses Modul schreibt 3D-Meshes im STL-Format, das von den meisten
//! 3D-Druckern und Slicern unterstützt wird.
//!
//! - **ASCII-STL**: Lesbar, aber größer (~10× mehr Speicher als Binär)
//! - **Binär-STL**: Kompaktes Format, geringere Dateigröße
//!
//! Für die Ausgabe mehrerer Layer (Farbschichten + Textur) wird ein ZIP-Archiv
//! erstellt, das je eine `.stl`-Datei pro Layer enthält.

mod archive;
mod threemf;
mod writer;

pub use archive::{export_to_dir, export_to_zip};
pub use threemf::export_to_3mf;
pub use writer::write_stl;

/// STL-Ausgabeformat
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum StlFormat {
    /// Menschenlesbares Text-Format
    Ascii,
    /// Kompaktes Binärformat (empfohlen)
    Binary,
}

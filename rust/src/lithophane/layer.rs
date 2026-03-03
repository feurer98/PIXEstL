//! Named 3D layer carrying optional filament color metadata

use crate::lithophane::geometry::Mesh;

/// A named 3D mesh layer with an optional filament color.
///
/// The `hex_color` field carries the palette hex code (e.g. `"#FF4400"`) of the filament
/// that this layer represents. It is `None` for layers that do not correspond to a single
/// filament (texture layer, base plate).
///
/// This color is used by exporters like `export_to_3mf` to embed sRGB color annotations
/// into the output file so that Bambu Studio can auto-assign AMS slots.
pub struct NamedLayer {
    pub name: String,
    pub mesh: Mesh,
    /// Palette hex color, e.g. `"#FF4400"`. `None` for texture/plate layers.
    pub hex_color: Option<String>,
}

impl NamedLayer {
    pub fn new(name: String, mesh: Mesh, hex_color: Option<String>) -> Self {
        Self { name, mesh, hex_color }
    }

    pub fn without_color(name: String, mesh: Mesh) -> Self {
        Self { name, mesh, hex_color: None }
    }
}

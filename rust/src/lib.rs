//! PIXEstL - Color Lithophane Generator
//!
//! This library provides functionality to generate color lithophanes for 3D printing
//! with multi-color filaments using CMYK-based additive color mixing.

pub mod cli;
pub mod color;
pub mod error;
pub mod image;
pub mod lithophane;
pub mod palette;
pub mod stl;

pub use error::{PixestlError, Result};
pub use lithophane::{LithophaneConfig, LithophaneGenerator, Mesh, Triangle, Vector3};
pub use palette::{Palette, PaletteLoader, PaletteLoaderConfig, PixelCreationMethod};
pub use stl::{export_to_zip, write_stl, StlFormat};

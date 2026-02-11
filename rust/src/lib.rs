//! PIXEstL - Color Lithophane Generator
//!
//! This library provides functionality to generate color lithophanes for 3D printing
//! with multi-color filaments using CMYK-based additive color mixing.
//!
//! # Example
//!
//! ```no_run
//! use pixestl::{Config, Palette, LithophaneGenerator};
//! use std::path::Path;
//!
//! # fn main() -> antml:Result<()> {
//! let config = Config::builder()
//!     .src_image_path("input.png")
//!     .palette_path("palette.json")
//!     .dest_image_width(100.0)
//!     .build()?;
//!
//! let palette = Palette::load(&config.palette_path)?;
//! let generator = LithophaneGenerator::new(config, palette);
//! generator.generate()?;
//! # Ok(())
//! # }
//! ```

pub mod cli;
pub mod color;
pub mod config;
pub mod error;
pub mod geometry;
pub mod image;
pub mod lithophane;
pub mod output;
pub mod palette;
pub mod stl;

pub use config::Config;
pub use error::{PixestlError, Result};
pub use lithophane::LithophaneGenerator;
pub use palette::Palette;

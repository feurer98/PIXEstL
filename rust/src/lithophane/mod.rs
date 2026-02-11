//! Lithophane generation core
//!
//! This module provides functionality for:
//! - Converting quantized images to 3D lithophane meshes
//! - Color layer generation (stacked cubes)
//! - Texture layer generation (brightness-based depth)
//! - Support plate generation
//! - Parallel mesh generation using Rayon

pub mod config;
pub mod generator;
pub mod geometry;
pub mod color_layer;
pub mod texture_layer;
pub mod support_plate;

pub use config::{LithophaneConfig, PixelCreationMethod};
pub use generator::LithophaneGenerator;
pub use geometry::{Mesh, Triangle, Vector3};

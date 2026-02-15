//! Command-line interface for PIXEstL

use crate::color::ColorDistanceMethod;
use crate::error::Result;
use crate::image::load_image;
use crate::lithophane::{LithophaneConfig, PixelCreationMethod as LithoPixelMethod};
use crate::palette::{
    PaletteLoader, PaletteLoaderConfig, PixelCreationMethod as PalettePixelMethod,
};
use crate::stl::{export_to_zip, StlFormat};
use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum CliStlFormat {
    Ascii,
    Binary,
}

impl From<CliStlFormat> for StlFormat {
    fn from(format: CliStlFormat) -> Self {
        match format {
            CliStlFormat::Ascii => StlFormat::Ascii,
            CliStlFormat::Binary => StlFormat::Binary,
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum CliColorDistance {
    Rgb,
    CieLab,
}

impl From<CliColorDistance> for ColorDistanceMethod {
    fn from(method: CliColorDistance) -> Self {
        match method {
            CliColorDistance::Rgb => ColorDistanceMethod::Rgb,
            CliColorDistance::CieLab => ColorDistanceMethod::CieLab,
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum CliPixelMethod {
    Additive,
    Full,
}

impl From<CliPixelMethod> for LithoPixelMethod {
    fn from(method: CliPixelMethod) -> Self {
        match method {
            CliPixelMethod::Additive => LithoPixelMethod::Additive,
            CliPixelMethod::Full => LithoPixelMethod::Full,
        }
    }
}

impl From<CliPixelMethod> for PalettePixelMethod {
    fn from(method: CliPixelMethod) -> Self {
        match method {
            CliPixelMethod::Additive => PalettePixelMethod::Additive,
            CliPixelMethod::Full => PalettePixelMethod::Full,
        }
    }
}

#[derive(Parser, Debug)]
#[command(name = "pixestl")]
#[command(author = "PIXEstL Contributors")]
#[command(version)]
#[command(about = "Generate color lithophanes for 3D printing", long_about = None)]
pub struct Cli {
    #[arg(short = 'i', long, value_name = "FILE")]
    pub input: PathBuf,

    #[arg(short = 'p', long, value_name = "FILE")]
    pub palette: PathBuf,

    #[arg(short = 'o', long, value_name = "FILE")]
    pub output: PathBuf,

    #[arg(short = 'w', long, default_value = "0")]
    pub width: f64,

    #[arg(short = 'h', long, default_value = "0")]
    pub height: f64,

    #[arg(long, default_value = "0.8")]
    pub color_pixel_width: f64,

    #[arg(long, default_value = "0.1")]
    pub color_layer_thickness: f64,

    #[arg(long, default_value = "5")]
    pub color_layers: u32,

    #[arg(long, default_value = "0.25")]
    pub texture_pixel_width: f64,

    #[arg(long, default_value = "0.3")]
    pub texture_min: f64,

    #[arg(long, default_value = "1.8")]
    pub texture_max: f64,

    #[arg(long, default_value = "0.2")]
    pub plate_thickness: f64,

    #[arg(long)]
    pub no_color: bool,

    #[arg(long)]
    pub no_texture: bool,

    #[arg(long, value_enum, default_value = "ascii")]
    pub format: CliStlFormat,

    #[arg(long, value_enum, default_value = "cie-lab")]
    pub color_distance: CliColorDistance,

    #[arg(long, value_enum, default_value = "additive")]
    pub pixel_method: CliPixelMethod,

    #[arg(long, default_value = "0")]
    pub color_number: usize,

    #[arg(long)]
    pub debug: bool,
}

impl Cli {
    pub fn to_lithophane_config(&self) -> LithophaneConfig {
        LithophaneConfig {
            dest_width_mm: self.width,
            dest_height_mm: self.height,
            color_pixel_width: self.color_pixel_width,
            color_pixel_layer_thickness: self.color_layer_thickness,
            color_pixel_layer_number: self.color_layers,
            color_layer: !self.no_color,
            texture_pixel_width: self.texture_pixel_width,
            texture_min_thickness: self.texture_min,
            texture_max_thickness: self.texture_max,
            texture_layer: !self.no_texture,
            plate_thickness: self.plate_thickness,
            pixel_creation_method: self.pixel_method.into(),
            color_number: self.color_number,
            color_distance_method: self.color_distance.into(),
            curve: 0.0,
            debug: self.debug,
            low_memory: false,
            layer_thread_max_number: 0,
            row_thread_number: num_cpus::get(),
        }
    }

    pub fn run(&self) -> Result<()> {
        println!("PIXEstL - Color Lithophane Generator");
        println!("=====================================\n");

        println!("Loading image: {}", self.input.display());
        let image = load_image(&self.input)?;
        println!(
            "  Image size: {}x{} pixels\n",
            image.width(),
            image.height()
        );

        println!("Loading palette: {}", self.palette.display());
        let palette_config = PaletteLoaderConfig {
            nb_layers: self.color_layers,
            creation_method: self.pixel_method.into(),
            color_number: self.color_number,
            distance_method: self.color_distance.into(),
        };
        let palette = PaletteLoader::load(&self.palette, palette_config)?;
        println!("  Colors found: {}", palette.colors().len());
        println!("  Color groups: {}\n", palette.hex_color_groups().len());

        println!("Generating lithophane layers...");
        let config = self.to_lithophane_config();
        let generator = crate::lithophane::LithophaneGenerator::new(config)?;
        let layers = generator.generate(&image, &palette)?;
        println!("  Generated {} layer(s)", layers.len());
        for (name, mesh) in &layers {
            println!("    - {}: {} triangles", name, mesh.triangle_count());
        }
        println!();

        println!("Exporting to: {}", self.output.display());
        export_to_zip(&layers, &self.output, self.format.into())?;
        println!("  Format: {:?}\n", self.format);

        println!("✓ Generation complete!");
        Ok(())
    }
}

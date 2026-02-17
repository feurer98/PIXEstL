//! Command-line interface for PIXEstL

use crate::color::ColorDistanceMethod;
use crate::error::Result;
use crate::image::load_image;
use crate::lithophane::{LithophaneConfig, PixelCreationMethod as LithoPixelMethod};
use crate::palette::{
    PaletteColorEntry, PaletteLoader, PaletteLoaderConfig,
    PixelCreationMethod as PalettePixelMethod,
};
use crate::stl::{export_to_zip, StlFormat};
use clap::{Parser, ValueEnum};
use std::collections::HashMap;
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
    /// Show palette information and exit (no generation)
    #[arg(long)]
    pub palette_info: bool,

    #[arg(
        short = 'i',
        long,
        value_name = "FILE",
        required_unless_present_any = ["palette_info", "calibrate"]
    )]
    pub input: Option<PathBuf>,

    #[arg(short = 'p', long, value_name = "FILE")]
    pub palette: PathBuf,

    #[arg(
        short = 'o',
        long,
        value_name = "FILE",
        required_unless_present_any = ["palette_info"]
    )]
    pub output: Option<PathBuf>,

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

    /// Curve angle in degrees (0=flat, 90=quarter cylinder, 180=half, 360=full cylinder)
    #[arg(short = 'C', long, default_value = "0")]
    pub curve: f64,

    /// Generate calibration test pattern instead of lithophane (no image needed)
    #[arg(long)]
    pub calibrate: bool,

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
            curve: self.curve,
            debug: self.debug,
            low_memory: false,
            layer_thread_max_number: 0,
            row_thread_number: num_cpus::get(),
        }
    }

    pub fn run(&self) -> Result<()> {
        if self.palette_info {
            return self.run_palette_info();
        }

        if self.calibrate {
            return self.run_calibrate();
        }

        // input and output are guaranteed present by clap (required_unless_present)
        let input = self.input.as_ref().unwrap();
        let output = self.output.as_ref().unwrap();

        println!("PIXEstL - Color Lithophane Generator");
        println!("=====================================\n");

        // --- Load and validate palette ---
        println!("Loading palette: {}", self.palette.display());
        let raw_palette = PaletteLoader::load_raw(&self.palette)?;
        self.print_palette_warnings(&raw_palette);

        let palette_config = PaletteLoaderConfig {
            nb_layers: self.color_layers,
            creation_method: self.pixel_method.into(),
            color_number: self.color_number,
            distance_method: self.color_distance.into(),
        };
        let palette = PaletteLoader::load(&self.palette, palette_config)?;
        println!("  Colors found: {}", palette.colors().len());
        println!("  Color groups: {}\n", palette.hex_color_groups().len());

        // --- Load image and check resolution ---
        println!("Loading image: {}", input.display());
        let image = load_image(input)?;
        println!("  Image size: {}x{} pixels", image.width(), image.height());
        self.print_resolution_warning(image.width(), image.height());
        println!();

        // --- Generate lithophane ---
        println!("Generating lithophane layers...");
        let config = self.to_lithophane_config();
        if config.curve > 0.0 {
            println!("  Curve: {:.0} degrees", config.curve);
        }
        let generator = crate::lithophane::LithophaneGenerator::new(config)?;
        let layers = generator.generate(&image, &palette)?;
        println!("  Generated {} layer(s)", layers.len());
        for (name, mesh) in &layers {
            println!("    - {}: {} triangles", name, mesh.triangle_count());
        }
        println!();

        // --- Export ---
        println!("Exporting to: {}", output.display());
        export_to_zip(&layers, output, self.format.into())?;
        println!("  Format: {:?}\n", self.format);

        println!("Done!");
        Ok(())
    }

    /// Generates calibration test pattern and exports it.
    fn run_calibrate(&self) -> Result<()> {
        let output = self.output.as_ref().ok_or_else(|| {
            crate::error::PixestlError::Config(
                "Output path (-o) is required for calibration".to_string(),
            )
        })?;

        println!("PIXEstL - Kalibrierungs-Testmuster");
        println!("==================================\n");

        // Load palette
        println!("Lade Palette: {}", self.palette.display());
        let raw_palette = PaletteLoader::load_raw(&self.palette)?;
        self.print_palette_warnings(&raw_palette);

        let active_count = raw_palette
            .values()
            .filter(|e| e.active && e.layers.is_some())
            .count();
        println!("  Aktive Filamente: {}", active_count);
        println!("  Schichten: {}\n", self.color_layers);

        // Generate calibration pattern
        let config = self.to_lithophane_config();
        let (grid_w, grid_d) = crate::lithophane::calibration::calibration_grid_dimensions(
            active_count,
            self.color_layers,
        );
        println!(
            "Generiere Kalibrierungsmuster: {:.0}mm x {:.0}mm",
            grid_w, grid_d
        );
        println!(
            "  {} Filamente x {} Schichtstufen = {} Testfelder",
            active_count,
            self.color_layers,
            active_count * self.color_layers as usize
        );

        let layers =
            crate::lithophane::calibration::generate_calibration_pattern(&raw_palette, &config)?;

        for (name, mesh) in &layers {
            println!("  - {}: {} Dreiecke", name, mesh.triangle_count());
        }
        println!();

        // Export
        println!("Exportiere nach: {}", output.display());
        export_to_zip(&layers, output, self.format.into())?;
        println!("  Format: {:?}\n", self.format);

        println!("Fertig!");
        println!();
        println!("--- Kalibrierungs-Anleitung ---");
        println!("1. Drucken Sie die ZIP-Datei mit Ihrem Multi-Color-Drucker.");
        println!("   Jede STL-Datei entspricht einem Filament.");
        println!(
            "2. Jede Reihe zeigt ein Filament bei 1 bis {} Schichten.",
            self.color_layers
        );
        println!("3. Fotografieren Sie das Ergebnis bei neutraler Beleuchtung");
        println!("   (Camera Pro: ISO 50-125, WB 5000K).");
        println!("4. Messen Sie die HSL-Werte jedes Feldes und tragen Sie sie");
        println!("   in die Palette-JSON ein.");

        Ok(())
    }

    /// Displays palette information and exits without generating.
    fn run_palette_info(&self) -> Result<()> {
        println!("PIXEstL - Palette Information");
        println!("============================\n");

        println!("Datei:          {}", self.palette.display());
        println!("Farbschichten:  {}", self.color_layers);
        println!("Methode:        {:?}\n", self.pixel_method);

        // Load raw palette data for display
        let raw_data = PaletteLoader::load_raw(&self.palette)?;

        // Separate active and inactive filaments
        let mut active: Vec<_> = raw_data
            .iter()
            .filter(|(_, e)| e.active && e.layers.is_some())
            .collect();
        active.sort_by_key(|(hex, _)| hex.to_string());

        let mut inactive: Vec<_> = raw_data
            .iter()
            .filter(|(_, e)| !e.active || e.layers.is_none())
            .collect();
        inactive.sort_by_key(|(hex, _)| hex.to_string());

        // Display active filaments
        println!("Aktive Filamente ({}):", active.len());
        for (hex, entry) in &active {
            Self::print_filament_info(hex, entry, self.color_layers);
        }

        // Display inactive filaments
        if !inactive.is_empty() {
            println!("\nInaktive Filamente ({}):", inactive.len());
            for (hex, entry) in &inactive {
                println!("  {}  {}", hex, entry.name);
            }
        }

        // Try to load full palette for combination count and AMS info
        let palette_config = PaletteLoaderConfig {
            nb_layers: self.color_layers,
            creation_method: self.pixel_method.into(),
            color_number: self.color_number,
            distance_method: self.color_distance.into(),
        };

        println!();
        match PaletteLoader::load(&self.palette, palette_config) {
            Ok(palette) => {
                println!("Farbkombinationen: {}", palette.color_count());
                println!("AMS-Gruppen:       {}", palette.nb_groups());

                // Show AMS group assignment if more than 1 group
                let groups = palette.hex_color_groups();
                if groups.len() > 1 {
                    println!("\nAMS-Gruppen-Zuordnung:");
                    for (i, group) in groups.iter().enumerate() {
                        let names: Vec<String> = group
                            .iter()
                            .map(|hex| {
                                raw_data
                                    .get(hex.as_str())
                                    .map(|e| e.name.as_str())
                                    .unwrap_or(hex.as_str())
                                    .to_string()
                            })
                            .collect();
                        println!("  Slot {}: {}", i + 1, names.join(", "));
                    }
                }
            }
            Err(e) => {
                println!("Farbkombinationen konnten nicht berechnet werden: {}", e);
            }
        }

        // Validation warnings
        let warnings = PaletteLoader::validate_completeness(&raw_data, self.color_layers);
        if warnings.is_empty() {
            println!("\nWarnungen: keine");
        } else {
            println!("\nWarnungen ({}):", warnings.len());
            for w in &warnings {
                eprintln!("  - {}", w);
            }
        }

        // Explanatory note about layer numbers (addresses Issue #12)
        println!("\n--- Hinweis zu Layer-Nummern ---");
        println!("Die Nummern in \"layers\" (z.B. \"1\", \"3\", \"5\") definieren die Farbdichte:");
        println!("  \"1\" = 1 Schicht dieses Filaments (duenn, hell, transparent)");
        println!("  \"5\" = 5 Schichten uebereinander (dick, dunkel, gesaettigt)");
        println!("Diese Nummern sind NICHT die physische Position im STL.");
        println!("PIXEstL kombiniert verschiedene Filamente mit verschiedenen Schichtanzahlen,");
        println!("um die Zielfarbe jedes Pixels bestmoeglich zu approximieren.");

        Ok(())
    }

    /// Prints information about a single filament entry.
    fn print_filament_info(hex: &str, entry: &PaletteColorEntry, target_layers: u32) {
        if let Some(layers) = &entry.layers {
            let mut layer_nums: Vec<u32> = layers.keys().filter_map(|k| k.parse().ok()).collect();
            layer_nums.sort();

            let layer_str: String = layer_nums
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(", ");

            let completeness = if layer_nums.len() as u32 >= target_layers {
                "vollstaendig".to_string()
            } else {
                format!("{} von {} definiert", layer_nums.len(), target_layers)
            };

            println!(
                "  {}  {:<35} Schichten: {:<15} ({})",
                hex, entry.name, layer_str, completeness
            );
        }
    }

    /// Prints palette validation warnings to stderr.
    fn print_palette_warnings(&self, raw_data: &HashMap<String, PaletteColorEntry>) {
        let warnings = PaletteLoader::validate_completeness(raw_data, self.color_layers);
        if !warnings.is_empty() {
            eprintln!();
            for w in &warnings {
                eprintln!("  [Warnung] {}", w);
            }
            eprintln!(
                "  [Hinweis] Layer-Nummern in der Palette definieren Farbdichte-Stufen \
                (Anzahl uebereinander gedruckter Schichten), nicht die physische Position im STL."
            );
            eprintln!();
        }
    }

    /// Prints a resolution warning if the image has significantly more pixels
    /// than the effective color resolution.
    fn print_resolution_warning(&self, image_width: u32, image_height: u32) {
        // Calculate effective width in mm
        let effective_width_mm = if self.width > 0.0 {
            self.width
        } else if self.height > 0.0 && image_height > 0 {
            (image_width as f64 * self.height) / image_height as f64
        } else {
            return; // Cannot calculate without dimensions
        };

        if effective_width_mm <= 0.0 || self.color_pixel_width <= 0.0 {
            return;
        }

        let effective_color_pixels = (effective_width_mm / self.color_pixel_width) as u32;

        // Warn if source image has at least 3x more pixels than effective output
        // (significant downsampling that could cause visible quality loss)
        if effective_color_pixels > 0 && image_width >= effective_color_pixels * 3 {
            let suggested_pixel_width = effective_width_mm / image_width as f64;
            eprintln!(
                "  [Hinweis] Das Bild hat {} Pixel Breite, aber bei {:.0}mm / {:.2}mm \
                Pixelgroesse entstehen nur {} Farbpixel.",
                image_width, effective_width_mm, self.color_pixel_width, effective_color_pixels
            );
            eprintln!(
                "            Fuer schaerfere Ergebnisse: --color-pixel-width {:.2} \
                (= {} Farbpixel)",
                suggested_pixel_width.max(0.1),
                (effective_width_mm / suggested_pixel_width.max(0.1)) as u32
            );
        }
    }
}

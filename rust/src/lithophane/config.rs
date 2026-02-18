//! Konfiguration für die Lithophan-Generierung
//!
//! Dieses Modul enthält alle Parameter, die steuern, wie ein 3D-druckbares
//! Lithophan aus einem Bild erzeugt wird. Die wichtigsten Konzepte:
//!
//! - **Farbschicht** (`color_layer`): Gestapelte Kunststoffwürfel kodieren Farben via AMS.
//!   Die Schichtdicke bestimmt die optische Farbintensität.
//! - **Texturschicht** (`texture_layer`): Eine Relief-Oberfläche, deren Dicke umgekehrt
//!   proportional zur Bildhelligkeit ist – dunkle Pixel werden dicker (undurchsichtiger).
//! - **Stützplatte** (`plate`): Eine flache Basis, die alle Farbschichten trägt.

use crate::color::ColorDistanceMethod;

/// Methode zur Pixel-Erstellung beim Drucken der Farbschichten
///
/// Steuert, wie die einzelnen Farbpixel als 3D-Geometrie erzeugt werden:
/// - `Additive`: Nur die tatsächlich benötigten Schichten werden hinzugefügt.
/// - `Full`: Jeder Pixel wird vollständig mit allen Schichten befüllt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelCreationMethod {
    /// Nur die benötigten Schichten werden additiv aufgebaut (spart Material).
    Additive,
    /// Jeder Pixel wird mit der vollständigen Schichtzahl befüllt.
    Full,
}

/// Vollständige Konfiguration für die Lithophan-Generierung
///
/// Alle Felder steuern gemeinsam die Geometrie und das Druckverhalten.
/// Der Standardwert (`Default`) ist für die meisten Anwendungsfälle geeignet.
#[derive(Debug, Clone)]
pub struct LithophaneConfig {
    /// Zielbreite des Lithophans in Millimetern (0 = aus Seitenverhältnis berechnen)
    pub dest_width_mm: f64,
    /// Zielhöhe des Lithophans in Millimetern (0 = aus Seitenverhältnis berechnen)
    pub dest_height_mm: f64,
    /// Breite eines Farbpixels in mm (entspricht der Nozzle-Größe, z.B. 0.8)
    pub color_pixel_width: f64,
    /// Dicke einer einzelnen Druckschicht in mm (z.B. 0.1)
    pub color_pixel_layer_thickness: f64,
    /// Anzahl der Farbschichten pro Pixel (bestimmt Farbintensität, z.B. 5)
    pub color_pixel_layer_number: u32,
    /// Ob eine Farbschicht generiert werden soll
    pub color_layer: bool,
    /// Breite eines Texturpixels in mm (kleiner als color_pixel_width für mehr Detail)
    pub texture_pixel_width: f64,
    /// Minimale Texturdicke in mm (für weiße/helle Pixel)
    pub texture_min_thickness: f64,
    /// Maximale Texturdicke in mm (für schwarze/dunkle Pixel)
    pub texture_max_thickness: f64,
    /// Ob eine Texturschicht generiert werden soll
    pub texture_layer: bool,
    /// Dicke der Basisplatte in mm
    pub plate_thickness: f64,
    /// Methode zur Pixel-Erstellung (Additive oder Full)
    pub pixel_creation_method: PixelCreationMethod,
    /// Anzahl der zu verwendenden Farben (0 = alle aktiven Farben)
    pub color_number: usize,
    /// Methode zur Farbabstandsberechnung (RGB oder CIELab)
    pub color_distance_method: ColorDistanceMethod,
    /// Krümmungswinkel in Grad (0 = flach, 90 = Viertelzylinder, 360 = voller Zylinder)
    pub curve: f64,
    /// Debug-Ausgaben aktivieren
    pub debug: bool,
    /// Speichersparender Modus (weniger parallele Verarbeitung)
    pub low_memory: bool,
    /// Maximale Thread-Anzahl für Layer-Verarbeitung (0 = unbegrenzt)
    pub layer_thread_max_number: usize,
    /// Thread-Anzahl für Zeilen-Verarbeitung (Standard: CPU-Anzahl)
    pub row_thread_number: usize,
}

impl Default for LithophaneConfig {
    fn default() -> Self {
        Self {
            dest_width_mm: 0.0,
            dest_height_mm: 0.0,
            color_pixel_width: 0.8,
            color_pixel_layer_thickness: 0.1,
            color_pixel_layer_number: 5,
            color_layer: true,
            texture_pixel_width: 0.25,
            texture_min_thickness: 0.3,
            texture_max_thickness: 1.8,
            texture_layer: true,
            plate_thickness: 0.2,
            pixel_creation_method: PixelCreationMethod::Additive,
            color_number: 0,
            color_distance_method: ColorDistanceMethod::CieLab,
            curve: 0.0,
            debug: false,
            low_memory: false,
            layer_thread_max_number: 0,
            row_thread_number: num_cpus::get(),
        }
    }
}

impl LithophaneConfig {
    /// Prüft die Konfiguration auf Gültigkeit.
    ///
    /// # Errors
    ///
    /// Gibt einen `PixestlError::Config`-Fehler zurück, wenn:
    /// - `color_pixel_width`, `texture_pixel_width` oder `color_pixel_layer_thickness` nicht positiv sind
    /// - `color_pixel_layer_number` null ist
    /// - `texture_min_thickness` nicht positiv ist
    /// - `texture_max_thickness` nicht größer als `texture_min_thickness` ist
    /// - `plate_thickness` negativ ist
    /// - weder `color_layer` noch `texture_layer` aktiviert ist
    /// - `curve` außerhalb des Bereichs [0, 360] liegt
    pub fn validate(&self) -> crate::error::Result<()> {
        if self.color_pixel_width <= 0.0 {
            return Err(crate::error::PixestlError::Config(
                "color_pixel_width must be positive".to_string(),
            ));
        }
        if self.texture_pixel_width <= 0.0 {
            return Err(crate::error::PixestlError::Config(
                "texture_pixel_width must be positive".to_string(),
            ));
        }
        if self.color_pixel_layer_thickness <= 0.0 {
            return Err(crate::error::PixestlError::Config(
                "color_pixel_layer_thickness must be positive".to_string(),
            ));
        }
        if self.color_pixel_layer_number == 0 {
            return Err(crate::error::PixestlError::Config(
                "color_pixel_layer_number must be positive".to_string(),
            ));
        }
        if self.texture_min_thickness <= 0.0 {
            return Err(crate::error::PixestlError::Config(
                "texture_min_thickness must be positive".to_string(),
            ));
        }
        if self.texture_max_thickness <= self.texture_min_thickness {
            return Err(crate::error::PixestlError::Config(
                "texture_max_thickness must be greater than texture_min_thickness".to_string(),
            ));
        }
        if self.plate_thickness < 0.0 {
            return Err(crate::error::PixestlError::Config(
                "plate_thickness must be non-negative".to_string(),
            ));
        }
        if !self.color_layer && !self.texture_layer {
            return Err(crate::error::PixestlError::Config(
                "At least one of color_layer or texture_layer must be enabled".to_string(),
            ));
        }
        if self.curve < 0.0 || self.curve > 360.0 {
            return Err(crate::error::PixestlError::Config(
                "curve must be between 0 and 360 degrees".to_string(),
            ));
        }
        Ok(())
    }

    /// Berechnet die Gesamthöhe aller Farbschichten in mm.
    ///
    /// # Returns
    ///
    /// `color_pixel_layer_thickness * color_pixel_layer_number` als `f64`-Wert in Millimetern.
    pub fn total_color_layer_height(&self) -> f64 {
        self.color_pixel_layer_thickness * self.color_pixel_layer_number as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_is_valid() {
        let config = LithophaneConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_invalid_color_pixel_width() {
        let config = LithophaneConfig {
            color_pixel_width: 0.0,
            ..LithophaneConfig::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_texture_pixel_width() {
        let config = LithophaneConfig {
            texture_pixel_width: -1.0,
            ..LithophaneConfig::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_layer_thickness() {
        let config = LithophaneConfig {
            color_pixel_layer_thickness: 0.0,
            ..LithophaneConfig::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_layer_number() {
        let config = LithophaneConfig {
            color_pixel_layer_number: 0,
            ..LithophaneConfig::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_texture_thickness_range() {
        let defaults = LithophaneConfig::default();
        let config = LithophaneConfig {
            texture_max_thickness: defaults.texture_min_thickness,
            ..defaults
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_plate_thickness() {
        let config = LithophaneConfig {
            plate_thickness: -0.1,
            ..LithophaneConfig::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_no_layers_enabled() {
        let config = LithophaneConfig {
            color_layer: false,
            texture_layer: false,
            ..LithophaneConfig::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_total_color_layer_height() {
        let config = LithophaneConfig {
            color_pixel_layer_thickness: 0.1,
            color_pixel_layer_number: 5,
            ..LithophaneConfig::default()
        };
        assert!((config.total_color_layer_height() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_valid_curve_zero() {
        let config = LithophaneConfig {
            curve: 0.0,
            ..LithophaneConfig::default()
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_valid_curve_360() {
        let config = LithophaneConfig {
            curve: 360.0,
            ..LithophaneConfig::default()
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_valid_curve_90() {
        let config = LithophaneConfig {
            curve: 90.0,
            ..LithophaneConfig::default()
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_invalid_curve_negative() {
        let config = LithophaneConfig {
            curve: -10.0,
            ..LithophaneConfig::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_curve_over_360() {
        let config = LithophaneConfig {
            curve: 400.0,
            ..LithophaneConfig::default()
        };
        assert!(config.validate().is_err());
    }
}

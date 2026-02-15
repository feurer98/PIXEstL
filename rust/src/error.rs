//! Error types for PIXEstL

use std::path::PathBuf;
use thiserror::Error;

/// Main error type for PIXEstL operations
#[derive(Error, Debug)]
pub enum PixestlError {
    /// Failed to load or process an image
    #[error("Failed to load image from {path}: {source}")]
    ImageLoad {
        path: PathBuf,
        source: image::ImageError,
    },

    /// Image processing error
    #[error("Image processing error: {0}")]
    ImageProcess(String),

    /// Failed to parse palette file
    #[error("Failed to parse palette from {path}: {message}")]
    PaletteParse { path: PathBuf, message: String },

    /// JSON parsing error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    Config(String),

    /// STL generation failed
    #[error("STL generation failed: {0}")]
    StlGeneration(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// ZIP creation error
    #[error("ZIP creation error: {0}")]
    Zip(#[from] zip::result::ZipError),

    /// Color conversion error
    #[error("Color conversion error: {0}")]
    ColorConversion(String),

    /// Invalid hex color code
    #[error("Invalid hex color code: {0}")]
    InvalidHexCode(String),

    /// Required color missing from palette
    #[error("Required color {0} missing from palette")]
    MissingColor(String),

    /// Invalid palette configuration
    #[error("Invalid palette: {0}")]
    InvalidPalette(String),

    /// General error with context
    #[error("{0}")]
    Other(String),
}

/// Result type alias for PIXEstL operations
pub type Result<T> = std::result::Result<T, PixestlError>;

impl From<image::ImageError> for PixestlError {
    fn from(err: image::ImageError) -> Self {
        PixestlError::ImageProcess(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = PixestlError::Config("test error".to_string());
        assert_eq!(err.to_string(), "Invalid configuration: test error");
    }

    #[test]
    fn test_invalid_hex_code_error() {
        let err = PixestlError::InvalidHexCode("#GGGGGG".to_string());
        assert!(err.to_string().contains("Invalid hex color code"));
    }
}

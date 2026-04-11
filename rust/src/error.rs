//! Error types for PIXEstL

use std::path::PathBuf;
use thiserror::Error;

/// Main error type for PIXEstL operations
#[derive(Error, Debug)]
pub enum PixestlError {
    /// Failed to load an image file
    #[error("Failed to load image from {path}: {source}")]
    ImageLoad {
        path: PathBuf,
        source: image::ImageError,
    },

    /// JSON parsing error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Invalid configuration or parameter
    #[error("Invalid configuration: {0}")]
    Config(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// ZIP creation error
    #[error("ZIP creation error: {0}")]
    Zip(#[from] zip::result::ZipError),

    /// Invalid hex color code
    #[error("Invalid hex color code: {0}")]
    InvalidHexCode(String),

    /// Invalid palette configuration
    #[error("Invalid palette: {0}")]
    InvalidPalette(String),

    /// 3MF export error (lib3mf)
    #[error("3MF export error: {0}")]
    Export(String),
}

/// Result type alias for PIXEstL operations
pub type Result<T> = std::result::Result<T, PixestlError>;

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

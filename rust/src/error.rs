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

    // ── Display for every variant ───────────────────────────────────────────

    #[test]
    fn test_display_config() {
        let err = PixestlError::Config("test error".to_string());
        assert_eq!(err.to_string(), "Invalid configuration: test error");
    }

    #[test]
    fn test_display_invalid_hex_code() {
        let err = PixestlError::InvalidHexCode("#GGGGGG".to_string());
        assert_eq!(err.to_string(), "Invalid hex color code: #GGGGGG");
    }

    #[test]
    fn test_display_invalid_palette() {
        let err = PixestlError::InvalidPalette("missing white".to_string());
        assert_eq!(err.to_string(), "Invalid palette: missing white");
    }

    #[test]
    fn test_display_export() {
        let err = PixestlError::Export("write failed".to_string());
        assert_eq!(err.to_string(), "3MF export error: write failed");
    }

    #[test]
    fn test_display_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file missing");
        let err = PixestlError::Io(io_err);
        assert!(err.to_string().starts_with("IO error:"));
        assert!(err.to_string().contains("file missing"));
    }

    #[test]
    fn test_display_json() {
        // Provoke a real serde_json error
        let json_err = serde_json::from_str::<serde_json::Value>("not json").unwrap_err();
        let err = PixestlError::Json(json_err);
        assert!(err.to_string().starts_with("JSON error:"));
    }

    #[test]
    fn test_display_image_load() {
        // Construct an ImageLoad variant manually
        let path = PathBuf::from("/tmp/fake.png");
        // Use a decoding error from a known-bad buffer
        let img_err = image::ImageReader::new(std::io::Cursor::new(b"not an image"))
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap_err();
        let err = PixestlError::ImageLoad {
            path: path.clone(),
            source: img_err,
        };
        let msg = err.to_string();
        assert!(msg.contains("/tmp/fake.png"), "should contain the path: {msg}");
    }

    // ── From impls ──────────────────────────────────────────────────────────

    #[test]
    fn test_from_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "no access");
        let err: PixestlError = io_err.into();
        match err {
            PixestlError::Io(e) => assert_eq!(e.kind(), std::io::ErrorKind::PermissionDenied),
            other => panic!("expected Io variant, got: {other}"),
        }
    }

    #[test]
    fn test_from_json_error() {
        let json_err = serde_json::from_str::<String>("42").unwrap_err();
        let err: PixestlError = json_err.into();
        assert!(matches!(err, PixestlError::Json(_)));
    }

    #[test]
    fn test_from_zip_error() {
        let zip_err = zip::result::ZipError::FileNotFound;
        let err: PixestlError = zip_err.into();
        assert!(matches!(err, PixestlError::Zip(_)));
        assert!(err.to_string().contains("ZIP creation error"));
    }
}

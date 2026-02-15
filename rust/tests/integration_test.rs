//! Integration tests for PIXEstL

use pixestl::lithophane::{LithophaneConfig, PixelCreationMethod};

#[test]
fn test_config_validation() {
    // Invalid: no layers enabled
    let config = LithophaneConfig {
        color_layer: false,
        texture_layer: false,
        ..Default::default()
    };
    assert!(config.validate().is_err());

    // Invalid: zero pixel width
    let config = LithophaneConfig {
        color_pixel_width: 0.0,
        ..Default::default()
    };
    assert!(config.validate().is_err());

    // Invalid: max < min thickness
    let config = LithophaneConfig {
        texture_min_thickness: 2.0,
        texture_max_thickness: 1.0,
        ..Default::default()
    };
    assert!(config.validate().is_err());

    // Valid config
    let config = LithophaneConfig::default();
    assert!(config.validate().is_ok());
}

#[test]
fn test_pixel_creation_method() {
    let method = PixelCreationMethod::Additive;
    assert_eq!(method, PixelCreationMethod::Additive);

    let method = PixelCreationMethod::Full;
    assert_eq!(method, PixelCreationMethod::Full);
}

#[test]
fn test_config_defaults() {
    let config = LithophaneConfig::default();
    assert_eq!(config.color_pixel_width, 0.8);
    assert_eq!(config.color_pixel_layer_thickness, 0.1);
    assert_eq!(config.color_pixel_layer_number, 5);
    assert_eq!(config.texture_pixel_width, 0.25);
    assert_eq!(config.texture_min_thickness, 0.3);
    assert_eq!(config.texture_max_thickness, 1.8);
    assert_eq!(config.plate_thickness, 0.2);
    assert!(config.color_layer);
    assert!(config.texture_layer);
}

#[test]
fn test_total_color_layer_height() {
    let config = LithophaneConfig {
        color_pixel_layer_thickness: 0.1,
        color_pixel_layer_number: 5,
        ..Default::default()
    };
    assert_eq!(config.total_color_layer_height(), 0.5);
}

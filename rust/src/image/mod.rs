//! Image processing utilities for lithophane generation
//!
//! This module provides functionality for:
//! - Loading and decoding images
//! - Resizing based on physical dimensions (mm)
//! - Converting to grayscale
//! - Handling transparency
//! - Flipping images for 3D printing

use crate::color::Rgb;
use crate::error::{PixestlError, Result};
use image::{DynamicImage, ImageBuffer, Rgba, RgbaImage};
use std::path::Path;

/// Loads an image from a file path
///
/// # Arguments
///
/// * `path` - Path to the image file
///
/// # Returns
///
/// A DynamicImage that can be processed further
///
/// # Example
///
/// ```no_run
/// use pixestl::image::load_image;
/// use std::path::Path;
///
/// let img = load_image(Path::new("input.png")).unwrap();
/// println!("Image size: {}x{}", img.width(), img.height());
/// ```
pub fn load_image(path: &Path) -> Result<DynamicImage> {
    image::open(path).map_err(|e| PixestlError::ImageLoad {
        path: path.to_path_buf(),
        source: e,
    })
}

/// Checks if the aspect ratio is preserved
///
/// Based on Java ImageUtil.checkRatio
///
/// Prints a warning if the source and destination aspect ratios differ
#[must_use]
#[allow(clippy::float_cmp)]
pub fn check_ratio(
    image_width: u32,
    image_height: u32,
    dest_width_mm: f64,
    dest_height_mm: f64,
) -> Option<String> {
    if dest_width_mm == 0.0 || dest_height_mm == 0.0 {
        return None;
    }

    let ratio_src = f64::from(image_width) / f64::from(image_height);
    let ratio_dest = dest_width_mm / dest_height_mm;

    // Round to 2 decimal places for comparison
    let ratio_src_rounded = (ratio_src * 100.0).round() / 100.0;
    let ratio_dest_rounded = (ratio_dest * 100.0).round() / 100.0;

    if ratio_src_rounded != ratio_dest_rounded {
        Some(format!(
            "Warning: The image ratio is not preserved. (Source ratio: {:.2}; Destination ratio: {:.2})",
            ratio_src, ratio_dest
        ))
    } else {
        None
    }
}

/// Resizes an image based on physical dimensions
///
/// Based on Java ImageUtil.resizeImage
///
/// # Arguments
///
/// * `image` - Input image
/// * `width_mm` - Desired width in millimeters (0 = auto-calculate)
/// * `height_mm` - Desired height in millimeters (0 = auto-calculate)
/// * `pixel_mm` - Size of each pixel in millimeters
///
/// # Returns
///
/// Resized image as RgbaImage
///
/// # Example
///
/// ```no_run
/// use pixestl::image::{load_image, resize_image};
/// use std::path::Path;
///
/// let img = load_image(Path::new("input.png")).unwrap();
/// let resized = resize_image(&img, 100.0, 0.0, 0.8).unwrap();
/// ```
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss
)]
#[allow(clippy::float_cmp)]
pub fn resize_image(
    image: &DynamicImage,
    width_mm: f64,
    height_mm: f64,
    pixel_mm: f64,
) -> Result<RgbaImage> {
    let (src_width, src_height) = (image.width(), image.height());

    let (nb_pixel_width, nb_pixel_height) = if width_mm != 0.0 && height_mm == 0.0 {
        // Width specified, calculate height proportionally
        let nb_pixel_width = (width_mm / pixel_mm) as u32;
        let height_mm_calc = (f64::from(src_height) * width_mm) / f64::from(src_width);
        let nb_pixel_height = (height_mm_calc / pixel_mm) as u32;
        (nb_pixel_width, nb_pixel_height)
    } else if width_mm == 0.0 && height_mm != 0.0 {
        // Height specified, calculate width proportionally
        let nb_pixel_height = (height_mm / pixel_mm) as u32;
        let width_mm_calc = (f64::from(src_width) * height_mm) / f64::from(src_height);
        let nb_pixel_width = (width_mm_calc / pixel_mm) as u32;
        (nb_pixel_width, nb_pixel_height)
    } else {
        // Both specified
        let nb_pixel_width = (width_mm / pixel_mm) as u32;
        let nb_pixel_height = (height_mm / pixel_mm) as u32;
        (nb_pixel_width, nb_pixel_height)
    };

    if nb_pixel_width == 0 || nb_pixel_height == 0 {
        return Err(PixestlError::ImageProcess(
            "Calculated image dimensions are zero".to_string(),
        ));
    }

    // Use Lanczos3 for high-quality resizing
    let resized = image.resize_exact(
        nb_pixel_width,
        nb_pixel_height,
        image::imageops::FilterType::Lanczos3,
    );

    Ok(resized.to_rgba8())
}

/// Checks if an image has any transparent pixels
///
/// Based on Java ImageUtil.hasATransparentPixel
#[must_use]
pub fn has_transparent_pixel(image: &RgbaImage) -> bool {
    image.pixels().any(|pixel| pixel[3] < 255)
}

/// Checks if a specific pixel is transparent
///
/// Based on Java ColorUtil.transparentPixel
#[must_use]
#[inline]
pub fn is_pixel_transparent(pixel: &Rgba<u8>) -> bool {
    pixel[3] < 255
}

/// Converts an image to grayscale (for texture layers)
///
/// Based on Java ImageUtil.convertToBlackAndWhite
///
/// Uses the standard luminance formula:
/// L = 0.2126*R + 0.7152*G + 0.0722*B
///
/// Preserves alpha channel for transparent pixels
#[must_use]
pub fn convert_to_grayscale(image: &RgbaImage) -> RgbaImage {
    let (width, height) = image.dimensions();
    let mut result = ImageBuffer::new(width, height);

    for (x, y, pixel) in image.enumerate_pixels() {
        if is_pixel_transparent(pixel) {
            // Preserve transparency
            result.put_pixel(x, y, Rgba([0, 0, 0, 0]));
        } else {
            // Calculate luminance
            #[allow(clippy::cast_possible_truncation)]
            let luminance = (0.2126 * f64::from(pixel[0])
                + 0.7152 * f64::from(pixel[1])
                + 0.0722 * f64::from(pixel[2])) as u8;

            result.put_pixel(x, y, Rgba([luminance, luminance, luminance, 255]));
        }
    }

    result
}

/// Flips an image vertically (for 3D printing)
///
/// Based on Java ImageUtil.flipImage
#[must_use]
pub fn flip_vertical(image: &RgbaImage) -> RgbaImage {
    image::imageops::flip_vertical(image)
}

/// Extracts pixels from an image as RGB colors
///
/// Skips transparent pixels by returning None
///
/// # Returns
///
/// 2D vector of `Option<Rgb>` where `None` indicates transparent pixel
#[must_use]
pub fn extract_pixels(image: &RgbaImage) -> Vec<Vec<Option<Rgb>>> {
    let (width, height) = image.dimensions();
    let mut pixels = Vec::with_capacity(height as usize);

    for y in 0..height {
        let mut row = Vec::with_capacity(width as usize);
        for x in 0..width {
            let pixel = image.get_pixel(x, y);

            if is_pixel_transparent(pixel) {
                row.push(None);
            } else {
                row.push(Some(Rgb::new(pixel[0], pixel[1], pixel[2])));
            }
        }
        pixels.push(row);
    }

    pixels
}

/// Extracts pixels as a flat RGB vector (for quantization)
///
/// Transparent pixels are excluded
#[must_use]
pub fn extract_pixels_flat(image: &RgbaImage) -> Vec<Rgb> {
    image
        .pixels()
        .filter(|p| !is_pixel_transparent(p))
        .map(|p| Rgb::new(p[0], p[1], p[2]))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_image(width: u32, height: u32) -> RgbaImage {
        ImageBuffer::from_fn(width, height, |x, _y| {
            if x < width / 2 {
                Rgba([255, 0, 0, 255]) // Red
            } else {
                Rgba([0, 255, 0, 255]) // Green
            }
        })
    }

    fn create_transparent_image(width: u32, height: u32) -> RgbaImage {
        ImageBuffer::from_fn(width, height, |x, _y| {
            if x < width / 2 {
                Rgba([255, 0, 0, 255]) // Opaque red
            } else {
                Rgba([0, 255, 0, 0]) // Transparent green
            }
        })
    }

    #[test]
    fn test_check_ratio_preserved() {
        let warning = check_ratio(800, 600, 100.0, 75.0);
        assert!(warning.is_none()); // 800/600 = 4/3, 100/75 = 4/3
    }

    #[test]
    fn test_check_ratio_not_preserved() {
        let warning = check_ratio(800, 600, 100.0, 100.0);
        assert!(warning.is_some()); // 800/600 != 100/100
    }

    #[test]
    fn test_check_ratio_zero_dimensions() {
        let warning = check_ratio(800, 600, 0.0, 0.0);
        assert!(warning.is_none()); // Should return None for zero dimensions
    }

    #[test]
    fn test_resize_image_width_only() {
        let img = DynamicImage::ImageRgba8(create_test_image(100, 50));
        let resized = resize_image(&img, 80.0, 0.0, 0.8).unwrap();

        // 80mm / 0.8mm = 100 pixels width
        // Height proportional: 50 pixels
        assert_eq!(resized.width(), 100);
        assert_eq!(resized.height(), 50);
    }

    #[test]
    fn test_resize_image_height_only() {
        let img = DynamicImage::ImageRgba8(create_test_image(100, 50));
        let resized = resize_image(&img, 0.0, 40.0, 0.8).unwrap();

        // 40mm / 0.8mm = 50 pixels height
        // Width proportional: 100 pixels
        assert_eq!(resized.width(), 100);
        assert_eq!(resized.height(), 50);
    }

    #[test]
    fn test_resize_image_both_dimensions() {
        let img = DynamicImage::ImageRgba8(create_test_image(100, 50));
        let resized = resize_image(&img, 40.0, 20.0, 0.5).unwrap();

        // 40mm / 0.5mm = 80 pixels width
        // 20mm / 0.5mm = 40 pixels height
        assert_eq!(resized.width(), 80);
        assert_eq!(resized.height(), 40);
    }

    #[test]
    fn test_has_transparent_pixel_opaque() {
        let img = create_test_image(10, 10);
        assert!(!has_transparent_pixel(&img));
    }

    #[test]
    fn test_has_transparent_pixel_transparent() {
        let img = create_transparent_image(10, 10);
        assert!(has_transparent_pixel(&img));
    }

    #[test]
    fn test_is_pixel_transparent() {
        assert!(is_pixel_transparent(&Rgba([255, 0, 0, 0])));
        assert!(is_pixel_transparent(&Rgba([255, 0, 0, 128])));
        assert!(!is_pixel_transparent(&Rgba([255, 0, 0, 255])));
    }

    #[test]
    fn test_convert_to_grayscale() {
        let mut img = ImageBuffer::new(2, 1);
        img.put_pixel(0, 0, Rgba([255, 0, 0, 255])); // Red
        img.put_pixel(1, 0, Rgba([0, 255, 0, 255])); // Green

        let gray = convert_to_grayscale(&img);

        let pixel0 = gray.get_pixel(0, 0);
        let pixel1 = gray.get_pixel(1, 0);

        // Red: L = 0.2126*255 = ~54
        assert!((pixel0[0] as i32 - 54).abs() < 2);
        assert_eq!(pixel0[0], pixel0[1]);
        assert_eq!(pixel0[0], pixel0[2]);

        // Green: L = 0.7152*255 = ~182
        assert!((pixel1[0] as i32 - 182).abs() < 2);
        assert_eq!(pixel1[0], pixel1[1]);
        assert_eq!(pixel1[0], pixel1[2]);
    }

    #[test]
    fn test_convert_to_grayscale_preserves_transparency() {
        let mut img = ImageBuffer::new(2, 1);
        img.put_pixel(0, 0, Rgba([255, 0, 0, 255])); // Opaque
        img.put_pixel(1, 0, Rgba([255, 0, 0, 0])); // Transparent

        let gray = convert_to_grayscale(&img);

        assert_eq!(gray.get_pixel(0, 0)[3], 255); // Opaque
        assert_eq!(gray.get_pixel(1, 0)[3], 0); // Transparent
    }

    #[test]
    fn test_flip_vertical() {
        let mut img = ImageBuffer::new(2, 2);
        img.put_pixel(0, 0, Rgba([255, 0, 0, 255])); // Top-left: Red
        img.put_pixel(1, 0, Rgba([0, 255, 0, 255])); // Top-right: Green
        img.put_pixel(0, 1, Rgba([0, 0, 255, 255])); // Bottom-left: Blue
        img.put_pixel(1, 1, Rgba([255, 255, 0, 255])); // Bottom-right: Yellow

        let flipped = flip_vertical(&img);

        // After flip: bottom becomes top
        assert_eq!(flipped.get_pixel(0, 0)[0], 0); // Was blue (bottom-left)
        assert_eq!(flipped.get_pixel(0, 0)[2], 255);

        assert_eq!(flipped.get_pixel(0, 1)[0], 255); // Was red (top-left)
        assert_eq!(flipped.get_pixel(0, 1)[1], 0);
    }

    #[test]
    fn test_extract_pixels() {
        let mut img = ImageBuffer::new(2, 2);
        img.put_pixel(0, 0, Rgba([255, 0, 0, 255]));
        img.put_pixel(1, 0, Rgba([0, 255, 0, 0])); // Transparent
        img.put_pixel(0, 1, Rgba([0, 0, 255, 255]));
        img.put_pixel(1, 1, Rgba([255, 255, 0, 255]));

        let pixels = extract_pixels(&img);

        assert_eq!(pixels.len(), 2); // 2 rows
        assert_eq!(pixels[0].len(), 2); // 2 columns

        assert!(pixels[0][0].is_some());
        assert!(pixels[0][1].is_none()); // Transparent
        assert!(pixels[1][0].is_some());
        assert!(pixels[1][1].is_some());

        assert_eq!(pixels[0][0].unwrap(), Rgb::new(255, 0, 0));
    }

    #[test]
    fn test_extract_pixels_flat() {
        let mut img = ImageBuffer::new(2, 2);
        img.put_pixel(0, 0, Rgba([255, 0, 0, 255]));
        img.put_pixel(1, 0, Rgba([0, 255, 0, 0])); // Transparent
        img.put_pixel(0, 1, Rgba([0, 0, 255, 255]));
        img.put_pixel(1, 1, Rgba([255, 255, 0, 255]));

        let pixels = extract_pixels_flat(&img);

        assert_eq!(pixels.len(), 3); // 4 pixels - 1 transparent = 3
        assert!(pixels.contains(&Rgb::new(255, 0, 0)));
        assert!(pixels.contains(&Rgb::new(0, 0, 255)));
        assert!(pixels.contains(&Rgb::new(255, 255, 0)));
    }
}

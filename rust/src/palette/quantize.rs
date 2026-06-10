//! Image quantization to palette colors with parallel processing

use crate::color::{find_closest_color_precomputed, CieLab, ColorDistanceMethod, Rgb};
use crate::error::Result;
use rayon::prelude::*;

/// Quantizes pixels to the closest palette colors in parallel
///
/// Based on Java Palette.quantizeColors with ExecutorService
///
/// # Arguments
///
/// * `pixels` - Input pixel data as RGB colors
/// * `palette_colors` - Available palette colors
/// * `method` - Color distance method to use
///
/// # Returns
///
/// Quantized pixel data with each pixel replaced by the closest palette color
///
/// # Example
///
/// ```
/// use pixestl::color::{Rgb, ColorDistanceMethod};
/// use pixestl::palette::quantize_pixels;
///
/// let pixels = vec![
///     Rgb::new(255, 0, 0),
///     Rgb::new(128, 128, 128),
/// ];
///
/// let palette = vec![
///     Rgb::new(255, 0, 0),
///     Rgb::new(0, 0, 0),
///     Rgb::new(255, 255, 255),
/// ];
///
/// let quantized = quantize_pixels(&pixels, &palette, ColorDistanceMethod::Rgb).unwrap();
/// assert_eq!(quantized.len(), pixels.len());
/// ```
pub fn quantize_pixels(
    pixels: &[Rgb],
    palette_colors: &[Rgb],
    method: ColorDistanceMethod,
) -> Result<Vec<Rgb>> {
    if palette_colors.is_empty() {
        return Ok(pixels.to_vec());
    }

    // Pre-compute CIELab values for palette colors once
    let palette_labs: Vec<CieLab> = palette_colors.iter().map(|c| CieLab::from(*c)).collect();

    // Use Rayon for parallel processing
    let quantized: Vec<Rgb> = pixels
        .par_iter()
        .map(|pixel| {
            find_closest_color_precomputed(pixel, palette_colors, &palette_labs, method)
                .expect("palette is non-empty")
        })
        .collect();

    Ok(quantized)
}

/// Quantizes an image (2D pixel array) to palette colors
///
/// # Arguments
///
/// * `image_data` - 2D array of pixels (row-major order)
/// * `width` - Image width
/// * `height` - Image height
/// * `palette_colors` - Available palette colors
/// * `method` - Color distance method to use
///
/// # Returns
///
/// Quantized image data in row-major order
pub fn quantize_image(
    image_data: &[Vec<Rgb>],
    palette_colors: &[Rgb],
    method: ColorDistanceMethod,
) -> Result<Vec<Vec<Rgb>>> {
    if palette_colors.is_empty() {
        return Ok(image_data.to_vec());
    }

    // Pre-compute CIELab values for palette colors once
    let palette_labs: Vec<CieLab> = palette_colors.iter().map(|c| CieLab::from(*c)).collect();

    // Process each row in parallel
    let quantized: Vec<Vec<Rgb>> = image_data
        .par_iter()
        .map(|row| {
            row.iter()
                .map(|pixel| {
                    find_closest_color_precomputed(pixel, palette_colors, &palette_labs, method)
                        .expect("palette is non-empty")
                })
                .collect()
        })
        .collect();

    Ok(quantized)
}

/// Quantizes an image while preserving transparent pixels in place.
///
/// `None` entries (transparent pixels from [`crate::image::extract_pixels`])
/// stay `None` at their original position, so the quantized image keeps the
/// exact geometry of the source. Dropping them instead would shift every
/// pixel right of a transparent hole to the left.
///
/// # Arguments
///
/// * `image_data` - 2D array of optional pixels (row-major, `None` = transparent)
/// * `palette_colors` - Available palette colors
/// * `method` - Color distance method to use
pub fn quantize_image_opt(
    image_data: &[Vec<Option<Rgb>>],
    palette_colors: &[Rgb],
    method: ColorDistanceMethod,
) -> Result<Vec<Vec<Option<Rgb>>>> {
    if palette_colors.is_empty() {
        return Ok(image_data.to_vec());
    }

    // Pre-compute CIELab values for palette colors once
    let palette_labs: Vec<CieLab> = palette_colors.iter().map(|c| CieLab::from(*c)).collect();

    let quantized: Vec<Vec<Option<Rgb>>> = image_data
        .par_iter()
        .map(|row| {
            row.iter()
                .map(|pixel| {
                    pixel.map(|p| {
                        find_closest_color_precomputed(&p, palette_colors, &palette_labs, method)
                            .expect("palette is non-empty")
                    })
                })
                .collect()
        })
        .collect();

    Ok(quantized)
}

/// Information about quantization results
#[derive(Debug, Clone)]
pub struct QuantizationStats {
    /// Number of unique colors used from the palette
    pub colors_used: usize,
    /// Total pixels processed
    pub total_pixels: usize,
    /// Color usage histogram (color -> count)
    pub color_usage: std::collections::HashMap<Rgb, usize>,
}

/// Quantizes pixels and collects statistics
///
/// # Arguments
///
/// * `pixels` - Input pixel data
/// * `palette_colors` - Available palette colors
/// * `method` - Color distance method to use
///
/// # Returns
///
/// Tuple of (quantized pixels, statistics)
pub fn quantize_with_stats(
    pixels: &[Rgb],
    palette_colors: &[Rgb],
    method: ColorDistanceMethod,
) -> Result<(Vec<Rgb>, QuantizationStats)> {
    let quantized = quantize_pixels(pixels, palette_colors, method)?;

    // Count color usage
    let mut color_usage = std::collections::HashMap::new();
    for color in &quantized {
        *color_usage.entry(*color).or_insert(0) += 1;
    }

    let stats = QuantizationStats {
        colors_used: color_usage.len(),
        total_pixels: pixels.len(),
        color_usage,
    };

    Ok((quantized, stats))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantize_pixels_basic() {
        let pixels = vec![
            Rgb::new(255, 0, 0),   // Red
            Rgb::new(128, 128, 0), // Yellowish
            Rgb::new(0, 255, 0),   // Green
        ];

        let palette = vec![
            Rgb::new(255, 0, 0), // Red
            Rgb::new(0, 255, 0), // Green
        ];

        let quantized = quantize_pixels(&pixels, &palette, ColorDistanceMethod::Rgb).unwrap();

        assert_eq!(quantized.len(), pixels.len());
        assert_eq!(quantized[0], Rgb::new(255, 0, 0)); // Red stays red
        assert_eq!(quantized[2], Rgb::new(0, 255, 0)); // Green stays green
    }

    #[test]
    fn test_quantize_pixels_empty_palette() {
        let pixels = vec![Rgb::new(128, 128, 128)];
        let palette = vec![];

        let quantized = quantize_pixels(&pixels, &palette, ColorDistanceMethod::Rgb).unwrap();

        assert_eq!(quantized, pixels); // No change if palette is empty
    }

    #[test]
    fn test_quantize_image_2d() {
        let image = vec![
            vec![Rgb::new(255, 0, 0), Rgb::new(128, 0, 0)],
            vec![Rgb::new(0, 255, 0), Rgb::new(0, 128, 0)],
        ];

        let palette = vec![
            Rgb::new(255, 0, 0), // Red
            Rgb::new(0, 255, 0), // Green
        ];

        let quantized = quantize_image(&image, &palette, ColorDistanceMethod::Rgb).unwrap();

        assert_eq!(quantized.len(), image.len());
        assert_eq!(quantized[0].len(), image[0].len());
    }

    #[test]
    fn test_quantize_with_stats() {
        let pixels = vec![
            Rgb::new(255, 0, 0),
            Rgb::new(255, 0, 0),
            Rgb::new(0, 255, 0),
        ];

        let palette = vec![
            Rgb::new(255, 0, 0), // Red
            Rgb::new(0, 255, 0), // Green
        ];

        let (quantized, stats) =
            quantize_with_stats(&pixels, &palette, ColorDistanceMethod::Rgb).unwrap();

        assert_eq!(quantized.len(), 3);
        assert_eq!(stats.total_pixels, 3);
        assert_eq!(stats.colors_used, 2); // Red and Green used
        assert_eq!(stats.color_usage[&Rgb::new(255, 0, 0)], 2); // Red used 2 times
        assert_eq!(stats.color_usage[&Rgb::new(0, 255, 0)], 1); // Green used 1 time
    }

    #[test]
    fn test_quantize_large_image_parallel() {
        // Test that parallel processing works
        let pixels: Vec<Rgb> = (0..10000)
            .map(|i| Rgb::new((i % 256) as u8, ((i / 256) % 256) as u8, 128))
            .collect();

        let palette = vec![
            Rgb::new(0, 0, 128),
            Rgb::new(128, 0, 128),
            Rgb::new(255, 0, 128),
            Rgb::new(0, 128, 128),
            Rgb::new(128, 128, 128),
        ];

        let quantized = quantize_pixels(&pixels, &palette, ColorDistanceMethod::Rgb).unwrap();

        assert_eq!(quantized.len(), pixels.len());
        // All pixels should be quantized to one of the palette colors
        assert!(quantized.iter().all(|c| palette.contains(c)));
    }

    #[test]
    fn test_quantize_single_color_palette() {
        let pixels = vec![
            Rgb::new(255, 0, 0),
            Rgb::new(0, 255, 0),
            Rgb::new(0, 0, 255),
            Rgb::new(128, 128, 128),
        ];
        let palette = vec![Rgb::new(42, 42, 42)];

        let quantized = quantize_pixels(&pixels, &palette, ColorDistanceMethod::Rgb).unwrap();
        // Every pixel must map to the only available color
        assert!(quantized.iter().all(|c| *c == Rgb::new(42, 42, 42)));
    }

    #[test]
    fn test_quantize_exact_match() {
        let palette = vec![
            Rgb::new(255, 0, 0),
            Rgb::new(0, 255, 0),
            Rgb::new(0, 0, 255),
        ];
        // Pixels that are exactly palette colors should map to themselves
        let quantized = quantize_pixels(&palette, &palette, ColorDistanceMethod::CieLab).unwrap();
        assert_eq!(quantized, palette);
    }

    #[test]
    fn test_quantize_image_empty_palette() {
        let image = vec![vec![Rgb::new(100, 100, 100), Rgb::new(200, 200, 200)]];
        let palette = vec![];
        let quantized = quantize_image(&image, &palette, ColorDistanceMethod::Rgb).unwrap();
        assert_eq!(quantized, image); // Unchanged
    }

    #[test]
    fn test_quantize_image_opt_preserves_none_positions() {
        let image = vec![
            vec![Some(Rgb::new(250, 5, 5)), None, Some(Rgb::new(5, 5, 250))],
            vec![None, Some(Rgb::new(250, 5, 5)), None],
        ];
        let palette = vec![Rgb::new(255, 0, 0), Rgb::new(0, 0, 255)];

        let quantized = quantize_image_opt(&image, &palette, ColorDistanceMethod::Rgb).unwrap();

        // Row lengths and None positions are untouched; colors are quantized.
        assert_eq!(quantized[0].len(), 3);
        assert_eq!(quantized[0][0], Some(Rgb::new(255, 0, 0)));
        assert_eq!(quantized[0][1], None);
        assert_eq!(quantized[0][2], Some(Rgb::new(0, 0, 255)));
        assert_eq!(quantized[1], vec![None, Some(Rgb::new(255, 0, 0)), None]);
    }

    #[test]
    fn test_quantize_image_opt_empty_palette() {
        let image = vec![vec![Some(Rgb::new(100, 100, 100)), None]];
        let palette = vec![];
        let quantized = quantize_image_opt(&image, &palette, ColorDistanceMethod::Rgb).unwrap();
        assert_eq!(quantized, image); // Unchanged
    }

    #[test]
    fn test_quantize_with_stats_single_color() {
        let pixels = vec![
            Rgb::new(10, 20, 30),
            Rgb::new(11, 21, 31),
            Rgb::new(12, 22, 32),
        ];
        let palette = vec![Rgb::new(0, 0, 0)];

        let (quantized, stats) =
            quantize_with_stats(&pixels, &palette, ColorDistanceMethod::Rgb).unwrap();

        assert_eq!(stats.colors_used, 1);
        assert_eq!(stats.total_pixels, 3);
        assert_eq!(stats.color_usage[&Rgb::new(0, 0, 0)], 3);
        assert!(quantized.iter().all(|c| *c == Rgb::new(0, 0, 0)));
    }

    #[test]
    fn test_quantize_with_stats_empty_pixels() {
        let pixels: Vec<Rgb> = vec![];
        let palette = vec![Rgb::new(255, 0, 0)];

        let (quantized, stats) =
            quantize_with_stats(&pixels, &palette, ColorDistanceMethod::Rgb).unwrap();

        assert!(quantized.is_empty());
        assert_eq!(stats.total_pixels, 0);
        assert_eq!(stats.colors_used, 0);
    }

    #[test]
    fn test_quantize_cielab_vs_rgb() {
        let pixel = Rgb::new(200, 100, 50);

        let palette = vec![
            Rgb::new(255, 0, 0),   // Red
            Rgb::new(0, 255, 0),   // Green
            Rgb::new(0, 0, 255),   // Blue
            Rgb::new(255, 128, 0), // Orange
        ];

        let q_rgb = quantize_pixels(&[pixel], &palette, ColorDistanceMethod::Rgb).unwrap();
        let q_lab = quantize_pixels(&[pixel], &palette, ColorDistanceMethod::CieLab).unwrap();

        // Both should produce valid palette colors
        assert!(palette.contains(&q_rgb[0]));
        assert!(palette.contains(&q_lab[0]));

        // They might differ due to different distance metrics
        // (This is expected and not an error)
    }
}

//! Image quantization to palette colors

use crate::color::{ColorDistanceMethod, Rgb};
use crate::error::Result;

/// Quantizes an image to palette colors using parallel processing
///
/// # TODO
///
/// - Implement with Rayon for parallelization
/// - Handle transparent pixels
/// - Return quantized image
pub fn quantize_image(
    _image_data: &[Rgb],
    _width: usize,
    _height: usize,
    _palette_colors: &[Rgb],
    _method: ColorDistanceMethod,
) -> Result<Vec<Rgb>> {
    // TODO: Implement parallel quantization
    Ok(Vec::new())
}

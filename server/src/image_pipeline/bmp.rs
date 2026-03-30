use image::codecs::bmp::BmpEncoder;
use image::{ExtendedColorType, RgbImage};

pub fn encode_bmp_24(image: &RgbImage) -> Result<Vec<u8>, image::ImageError> {
    let mut bytes = Vec::new();
    let mut encoder = BmpEncoder::new(&mut bytes);
    encoder.encode(
        image.as_raw(),
        image.width(),
        image.height(),
        ExtendedColorType::Rgb8,
    )?;
    Ok(bytes)
}

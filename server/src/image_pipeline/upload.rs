use std::fmt;
use std::io::Cursor;
use std::path::{Path, PathBuf};

use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView, ImageFormat, ImageReader, RgbImage};

use crate::config::{UPLOAD_IMAGE_HEIGHT, UPLOAD_IMAGE_WIDTH, input_image_temp_path_from_dir};

#[derive(Clone, Debug)]
pub struct UploadSuccess {
    pub source_format: UploadFormat,
    pub width: u32,
    pub height: u32,
    pub normalized: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UploadFormat {
    Png,
    Jpeg,
    Gif,
    Bmp,
    WebP,
}

impl fmt::Display for UploadFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Png => "PNG",
            Self::Jpeg => "JPEG",
            Self::Gif => "GIF",
            Self::Bmp => "BMP",
            Self::WebP => "WebP",
        };
        write!(f, "{value}")
    }
}

#[derive(Debug)]
pub enum UploadError {
    EmptyBody,
    InvalidMultipart(String),
    UnsupportedMediaType(String),
    Decode(image::ImageError),
    Save(PathBuf, std::io::Error),
}

pub fn decode_upload_image(bytes: &[u8]) -> Result<(Vec<u8>, UploadSuccess), UploadError> {
    if bytes.is_empty() {
        return Err(UploadError::EmptyBody);
    }

    let format = detect_upload_format(bytes)?;
    let image = decode_image(bytes, format)?;
    let normalized_image = normalize_image(image);
    let mut encoded = Cursor::new(Vec::new());
    DynamicImage::ImageRgb8(normalized_image.image.clone())
        .write_to(&mut encoded, ImageFormat::Png)
        .map_err(UploadError::Decode)?;

    Ok((
        encoded.into_inner(),
        UploadSuccess {
            source_format: format,
            width: normalized_image.image.width(),
            height: normalized_image.image.height(),
            normalized: normalized_image.normalized,
        },
    ))
}

pub fn replace_input_image(content_dir: &Path, png_bytes: &[u8]) -> Result<(), UploadError> {
    let target = crate::config::input_image_path_from_dir(content_dir);
    let temp = input_image_temp_path_from_dir(content_dir);

    std::fs::write(&temp, png_bytes).map_err(|err| UploadError::Save(temp.clone(), err))?;

    if let Err(err) = std::fs::rename(&temp, &target) {
        let _ = std::fs::remove_file(&temp);
        return Err(UploadError::Save(target, err));
    }

    Ok(())
}

pub fn upload_success_outcome() -> crate::logging::LogOutcome {
    crate::logging::LogOutcome::UploadSuccess
}

pub fn upload_error_outcome(error: &UploadError) -> crate::logging::LogOutcome {
    match error {
        UploadError::Save(_, _) => crate::logging::LogOutcome::UploadSaveFailed,
        UploadError::EmptyBody
        | UploadError::InvalidMultipart(_)
        | UploadError::UnsupportedMediaType(_)
        | UploadError::Decode(_) => crate::logging::LogOutcome::UploadInvalid,
    }
}

fn detect_upload_format(bytes: &[u8]) -> Result<UploadFormat, UploadError> {
    let image_format = image::guess_format(bytes).map_err(|_| {
        UploadError::UnsupportedMediaType(
            "unsupported upload format; accepted formats are PNG, JPG, GIF, BMP, WebP".to_string(),
        )
    })?;

    match image_format {
        ImageFormat::Png => Ok(UploadFormat::Png),
        ImageFormat::Jpeg => Ok(UploadFormat::Jpeg),
        ImageFormat::Gif => Ok(UploadFormat::Gif),
        ImageFormat::Bmp => Ok(UploadFormat::Bmp),
        ImageFormat::WebP => Ok(UploadFormat::WebP),
        _ => Err(UploadError::UnsupportedMediaType(
            "unsupported upload format; accepted formats are PNG, JPG, GIF, BMP, WebP".to_string(),
        )),
    }
}

fn decode_image(bytes: &[u8], format: UploadFormat) -> Result<DynamicImage, UploadError> {
    let image_format = match format {
        UploadFormat::Png => ImageFormat::Png,
        UploadFormat::Jpeg => ImageFormat::Jpeg,
        UploadFormat::Gif => ImageFormat::Gif,
        UploadFormat::Bmp => ImageFormat::Bmp,
        UploadFormat::WebP => ImageFormat::WebP,
    };

    let cursor = Cursor::new(bytes);
    ImageReader::with_format(cursor, image_format)
        .decode()
        .map_err(UploadError::Decode)
}

struct NormalizedImage {
    image: RgbImage,
    normalized: bool,
}

fn normalize_image(image: DynamicImage) -> NormalizedImage {
    let original = image.dimensions();
    let normalized = image
        .resize_to_fill(
            UPLOAD_IMAGE_WIDTH,
            UPLOAD_IMAGE_HEIGHT,
            FilterType::Lanczos3,
        )
        .to_rgb8();
    let normalized_applied = original != (UPLOAD_IMAGE_WIDTH, UPLOAD_IMAGE_HEIGHT)
        || !matches!(image, DynamicImage::ImageRgb8(_));

    NormalizedImage {
        image: normalized,
        normalized: normalized_applied,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{Rgb, RgbImage};

    fn make_bytes(format: ImageFormat, width: u32, height: u32) -> Vec<u8> {
        let mut image = RgbImage::from_pixel(width, height, Rgb([20, 40, 60]));
        image.put_pixel(
            width.saturating_sub(1),
            height.saturating_sub(1),
            Rgb([200, 10, 10]),
        );
        let mut cursor = Cursor::new(Vec::new());
        DynamicImage::ImageRgb8(image)
            .write_to(&mut cursor, format)
            .expect("encode test image");
        cursor.into_inner()
    }

    #[test]
    fn decodes_supported_formats_and_normalizes_dimensions() {
        for format in [
            ImageFormat::Png,
            ImageFormat::Jpeg,
            ImageFormat::Gif,
            ImageFormat::Bmp,
            ImageFormat::WebP,
        ] {
            let (bytes, success) =
                decode_upload_image(&make_bytes(format, 64, 96)).expect("decode upload");
            assert!(!bytes.is_empty());
            assert_eq!(success.width, UPLOAD_IMAGE_WIDTH);
            assert_eq!(success.height, UPLOAD_IMAGE_HEIGHT);
            assert!(success.normalized);
        }
    }

    #[test]
    fn empty_body_is_rejected() {
        let err = decode_upload_image(&[]).expect_err("empty should fail");
        assert!(matches!(err, UploadError::EmptyBody));
    }

    #[test]
    fn unsupported_media_is_rejected() {
        let err = decode_upload_image(b"not-an-image").expect_err("invalid should fail");
        assert!(matches!(err, UploadError::UnsupportedMediaType(_)));
    }

    #[test]
    fn save_replaces_input_image_via_temp_file() {
        let dir = std::env::temp_dir().join(format!("upload-save-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("create dir");
        let png_bytes = make_bytes(ImageFormat::Png, 32, 32);

        replace_input_image(&dir, &png_bytes).expect("replace image");

        let saved =
            std::fs::read(crate::config::input_image_path_from_dir(&dir)).expect("read saved");
        assert_eq!(saved, png_bytes);
        assert!(!input_image_temp_path_from_dir(&dir).exists());

        let _ = std::fs::remove_dir_all(&dir);
    }
}

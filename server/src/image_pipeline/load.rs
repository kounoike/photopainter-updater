use std::path::Path;

use image::{ImageReader, RgbImage};

use crate::image_pipeline::ImageLoadError;

pub fn load_input_image(path: &Path) -> Result<RgbImage, ImageLoadError> {
    if !path.exists() {
        return Err(ImageLoadError::Missing(path.to_path_buf()));
    }

    let reader = ImageReader::open(path).map_err(|err| match err.kind() {
        std::io::ErrorKind::NotFound => ImageLoadError::Missing(path.to_path_buf()),
        _ => ImageLoadError::Io(path.to_path_buf(), err),
    })?;

    reader
        .decode()
        .map(|image| image.to_rgb8())
        .map_err(|err| ImageLoadError::Decode(path.to_path_buf(), err))
}

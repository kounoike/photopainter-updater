pub mod binary;
pub mod bmp;
pub mod dither;
pub mod load;

use std::path::{Path, PathBuf};

use image::{ImageBuffer, RgbImage};

use crate::config::{CompareSplit, DitherOptions, RenderOptions};

pub use binary::encode_binary_frame;
#[cfg(test)]
pub use binary::payload_checksum;
pub use bmp::encode_bmp_24;
pub use dither::{apply_reference_dither, boost_saturation, rotate_right_90};
pub use load::load_input_image;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResponseFormat {
    Bmp,
    Binary,
}

#[derive(Debug)]
pub enum ImageLoadError {
    Missing(PathBuf),
    Io(PathBuf, std::io::Error),
    Decode(PathBuf, image::ImageError),
}

#[derive(Debug)]
pub enum TransformError {
    Encode(image::ImageError),
}

#[derive(Debug)]
pub struct ImagePipelineRequest {
    pub input_path: PathBuf,
    pub response_format: ResponseFormat,
    pub render_options: RenderOptions,
}

#[derive(Debug)]
pub enum ResponsePayload {
    Bmp(Vec<u8>),
    Binary(Vec<u8>),
}

pub fn render_response(
    input: &RgbImage,
    response_format: ResponseFormat,
    options: RenderOptions,
) -> Result<ResponsePayload, TransformError> {
    match response_format {
        ResponseFormat::Bmp => Ok(ResponsePayload::Bmp(render_bmp_response(input, options)?)),
        ResponseFormat::Binary => Ok(ResponsePayload::Binary(render_binary_frame_response(
            input, options,
        )?)),
    }
}

fn render_profile_image(input: &RgbImage, options: DitherOptions) -> RgbImage {
    let saturated = boost_saturation(input, options.saturation_mode);
    let dithered = apply_reference_dither(&saturated, options);
    rotate_right_90(&dithered)
}

pub fn render_transformed_image(input: &RgbImage, options: RenderOptions) -> RgbImage {
    let candidate = render_profile_image(input, options.dither_options);
    let Some(compare_profile) = options.compare.profile else {
        return candidate;
    };

    let compare_image = render_profile_image(input, compare_profile.default_dither_options());
    combine_split_view(&compare_image, &candidate, options.compare.split)
}

pub fn render_bmp_response(
    input: &RgbImage,
    options: RenderOptions,
) -> Result<Vec<u8>, TransformError> {
    let rotated = render_transformed_image(input, options);
    encode_bmp_24(&rotated).map_err(TransformError::Encode)
}

pub fn render_binary_frame_response(
    input: &RgbImage,
    options: RenderOptions,
) -> Result<Vec<u8>, TransformError> {
    let rotated = render_transformed_image(input, options);
    Ok(encode_binary_frame(&rotated))
}

pub fn image_load_error_outcome(error: &ImageLoadError) -> crate::logging::LogOutcome {
    match error {
        ImageLoadError::Missing(_) => crate::logging::LogOutcome::InputMissing,
        ImageLoadError::Decode(_, _) => crate::logging::LogOutcome::TransformFailed,
        ImageLoadError::Io(_, _) => crate::logging::LogOutcome::InternalError,
    }
}

pub fn transform_error_outcome(_error: &TransformError) -> crate::logging::LogOutcome {
    crate::logging::LogOutcome::InternalError
}

pub fn input_image_path_from_dir(content_dir: &Path) -> PathBuf {
    crate::config::input_image_path_from_dir(content_dir)
}

fn combine_split_view(baseline: &RgbImage, candidate: &RgbImage, split: CompareSplit) -> RgbImage {
    debug_assert_eq!(baseline.dimensions(), candidate.dimensions());
    let (width, height) = baseline.dimensions();
    let mut merged = ImageBuffer::new(width, height);

    match split {
        CompareSplit::Vertical => {
            let midpoint = width / 2;
            for y in 0..height {
                for x in 0..width {
                    let pixel = if x < midpoint {
                        *baseline.get_pixel(x, y)
                    } else {
                        *candidate.get_pixel(x, y)
                    };
                    merged.put_pixel(x, y, pixel);
                }
            }
        }
        CompareSplit::Horizontal => {
            let midpoint = height / 2;
            for y in 0..height {
                for x in 0..width {
                    let pixel = if y < midpoint {
                        *baseline.get_pixel(x, y)
                    } else {
                        *candidate.get_pixel(x, y)
                    };
                    merged.put_pixel(x, y, pixel);
                }
            }
        }
    }

    merged
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{
        BINARY_FRAME_FLAGS, BINARY_FRAME_MAGIC, BINARY_FRAME_VERSION, BINARY_HEADER_LENGTH,
        CompareOptions, CompareSplit, EPD_DISPLAY_HEIGHT, EPD_DISPLAY_WIDTH, ImageProfile,
        REFERENCE_PALETTE, RenderOptions, SATURATION_TOLERANCE, SaturationMode,
    };
    use image::ImageFormat;
    use image::{Rgb, RgbImage};

    const SATURATION_COORDS: &[(u32, u32)] = &[
        (4, 4),
        (12, 4),
        (4, 12),
        (20, 12),
        (12, 20),
        (4, 28),
        (12, 28),
        (20, 28),
    ];

    fn fixture_path(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("testdata")
            .join("image-dither-rotate")
            .join(name)
    }

    fn load_fixture(name: &str) -> RgbImage {
        load_input_image(&fixture_path(name)).expect("fixture image")
    }

    fn create_sample_png(path: &Path, pixels: &[(u32, u32, [u8; 3])], width: u32, height: u32) {
        let mut image = RgbImage::from_pixel(width, height, Rgb([255, 255, 255]));
        for &(x, y, rgb) in pixels {
            image.put_pixel(x, y, Rgb(rgb));
        }
        image
            .save_with_format(path, ImageFormat::Png)
            .expect("write png fixture");
    }

    fn palette_histogram(image: &RgbImage) -> [usize; 6] {
        let mut counts = [0usize; 6];
        for pixel in image.pixels() {
            let index = match pixel.0 {
                [0, 0, 0] => 0,
                [255, 255, 255] => 1,
                [255, 255, 0] => 2,
                [255, 0, 0] => 3,
                [0, 0, 255] => 4,
                [0, 255, 0] => 5,
                other => panic!("unexpected palette color {other:?}"),
            };
            counts[index] += 1;
        }
        counts
    }

    fn parse_binary_header(bytes: &[u8]) -> (u16, u16, u32, u32, &[u8]) {
        assert!(bytes.len() >= BINARY_HEADER_LENGTH as usize);
        assert_eq!(&bytes[0..4], &BINARY_FRAME_MAGIC);
        assert_eq!(bytes[4], BINARY_FRAME_VERSION);
        assert_eq!(bytes[5], BINARY_FRAME_FLAGS);
        let header_length = u16::from_le_bytes([bytes[6], bytes[7]]);
        let width = u16::from_le_bytes([bytes[8], bytes[9]]);
        let height = u16::from_le_bytes([bytes[10], bytes[11]]);
        let payload_length = u32::from_le_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]);
        let checksum = u32::from_le_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]);
        assert_eq!(header_length, BINARY_HEADER_LENGTH);
        let payload = &bytes[header_length as usize..];
        assert_eq!(payload.len(), payload_length as usize);
        (width, height, payload_length, checksum, payload)
    }

    fn validate_binary_frame(bytes: &[u8]) -> Result<(), &'static str> {
        if bytes.is_empty() {
            return Err("empty response body");
        }
        if bytes.len() < BINARY_HEADER_LENGTH as usize {
            return Err("binary frame header is incomplete");
        }
        if &bytes[0..4] != BINARY_FRAME_MAGIC.as_slice() {
            return Err("binary frame magic is invalid");
        }
        if bytes[4] != BINARY_FRAME_VERSION {
            return Err("binary frame version is invalid");
        }
        if bytes[5] != BINARY_FRAME_FLAGS {
            return Err("binary frame flags are invalid");
        }

        let header_length = u16::from_le_bytes([bytes[6], bytes[7]]);
        if header_length != BINARY_HEADER_LENGTH {
            return Err("binary frame header length is invalid");
        }

        let width = u16::from_le_bytes([bytes[8], bytes[9]]);
        let height = u16::from_le_bytes([bytes[10], bytes[11]]);
        if width != EPD_DISPLAY_WIDTH as u16 || height != EPD_DISPLAY_HEIGHT as u16 {
            return Err("binary frame dimensions are invalid");
        }

        let payload_length =
            u32::from_le_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]) as usize;
        let expected_length = EPD_DISPLAY_WIDTH * EPD_DISPLAY_HEIGHT / 2;
        if payload_length != expected_length {
            return Err("binary frame payload length is invalid");
        }

        if bytes.len() != BINARY_HEADER_LENGTH as usize + payload_length {
            return Err("binary frame payload is incomplete");
        }

        let expected_checksum = u32::from_le_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]);
        let actual_checksum = payload_checksum(&bytes[BINARY_HEADER_LENGTH as usize..]);
        if actual_checksum != expected_checksum {
            return Err("binary frame checksum mismatch");
        }

        Ok(())
    }

    #[test]
    fn load_input_image_reads_png_when_present() {
        let dir = std::env::temp_dir().join("load-image-fixture");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("create temp dir");
        create_sample_png(
            &input_image_path_from_dir(&dir),
            &[(0, 0, [12, 34, 56])],
            2,
            2,
        );
        let image = load_input_image(&input_image_path_from_dir(&dir)).expect("png should load");

        assert_eq!(image.width(), 2);
        assert_eq!(image.height(), 2);
        assert_eq!(image.get_pixel(0, 0).0, [12, 34, 56]);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn encode_bmp_24_writes_bmp_header_and_bit_depth() {
        let image = RgbImage::from_pixel(2, 3, Rgb([1, 2, 3]));
        let encoded = encode_bmp_24(&image).expect("bmp encoding");

        assert_eq!(&encoded[0..2], b"BM");
        assert_eq!(&encoded[28..30], &[24, 0]);
    }

    #[test]
    fn binary_frame_validation_rejects_empty_body() {
        assert_eq!(validate_binary_frame(&[]), Err("empty response body"));
    }

    #[test]
    fn binary_frame_validation_rejects_invalid_magic() {
        let image = RgbImage::from_pixel(
            EPD_DISPLAY_WIDTH as u32,
            EPD_DISPLAY_HEIGHT as u32,
            Rgb([255, 255, 255]),
        );
        let mut encoded = encode_binary_frame(&image);
        encoded[0] = b'X';

        assert_eq!(
            validate_binary_frame(&encoded),
            Err("binary frame magic is invalid")
        );
    }

    #[test]
    fn binary_frame_validation_rejects_checksum_mismatch() {
        let image = RgbImage::from_pixel(
            EPD_DISPLAY_WIDTH as u32,
            EPD_DISPLAY_HEIGHT as u32,
            Rgb([255, 255, 255]),
        );
        let mut encoded = encode_binary_frame(&image);
        let last_index = encoded.len() - 1;
        encoded[last_index] ^= 0x0f;

        assert_eq!(
            validate_binary_frame(&encoded),
            Err("binary frame checksum mismatch")
        );
    }

    #[test]
    fn encoded_binary_frame_contains_expected_header() {
        let image = RgbImage::from_pixel(
            EPD_DISPLAY_WIDTH as u32,
            EPD_DISPLAY_HEIGHT as u32,
            Rgb([255, 255, 255]),
        );
        let encoded = encode_binary_frame(&image);
        let (width, height, payload_length, checksum, payload) = parse_binary_header(&encoded);

        assert_eq!(width, EPD_DISPLAY_WIDTH as u16);
        assert_eq!(height, EPD_DISPLAY_HEIGHT as u16);
        assert_eq!(
            payload_length,
            (EPD_DISPLAY_WIDTH * EPD_DISPLAY_HEIGHT / 2) as u32
        );
        assert_eq!(payload_checksum(payload), checksum);
    }

    #[test]
    fn saturation_fixture_matches_post_pixels_within_tolerance() {
        let pre = load_fixture("pre.png");
        let post = load_fixture("post.png");
        let boosted = boost_saturation(&pre, SaturationMode::Boosted);

        for &(x, y) in SATURATION_COORDS {
            let source = pre.get_pixel(x, y).0;
            let actual = boosted.get_pixel(x, y).0;
            let expected = post.get_pixel(x, y).0;
            for channel in 0..3 {
                assert!(
                    actual[channel].abs_diff(expected[channel]) <= SATURATION_TOLERANCE,
                    "pixel ({x}, {y}) channel {channel} source {:?} expected {:?} actual {:?}",
                    source,
                    expected,
                    actual
                );
            }
        }
    }

    #[test]
    fn dithering_output_uses_only_reference_palette_colors() {
        let input = RgbImage::from_fn(4, 1, |x, _| match x {
            0 => Rgb([20, 20, 20]),
            1 => Rgb([220, 210, 30]),
            2 => Rgb([40, 80, 240]),
            _ => Rgb([30, 200, 100]),
        });

        let dithered = apply_reference_dither(
            &input,
            DitherOptions {
                use_lab: false,
                use_atkinson: false,
                use_zigzag: false,
                diffusion_rate: 1.0,
                saturation_mode: SaturationMode::Boosted,
                neutral_bias: 0.0,
                chroma_bias: 0.0,
                hue_guard: 0.0,
                blue_bias: 0.0,
                highlight_guard: 0.0,
                skin_tone_guard: 0.0,
            },
        );

        for pixel in dithered.pixels() {
            assert!(REFERENCE_PALETTE.contains(&pixel.0));
        }
    }

    #[test]
    fn rotate_right_90_maps_coordinates_clockwise() {
        let mut image = RgbImage::new(2, 3);
        image.put_pixel(0, 0, Rgb([1, 0, 0]));
        image.put_pixel(1, 0, Rgb([2, 0, 0]));
        image.put_pixel(0, 1, Rgb([3, 0, 0]));
        image.put_pixel(1, 1, Rgb([4, 0, 0]));
        image.put_pixel(0, 2, Rgb([5, 0, 0]));
        image.put_pixel(1, 2, Rgb([6, 0, 0]));

        let rotated = rotate_right_90(&image);

        assert_eq!(rotated.dimensions(), (3, 2));
        assert_eq!(rotated.get_pixel(2, 0).0, [1, 0, 0]);
        assert_eq!(rotated.get_pixel(2, 1).0, [2, 0, 0]);
        assert_eq!(rotated.get_pixel(1, 0).0, [3, 0, 0]);
        assert_eq!(rotated.get_pixel(1, 1).0, [4, 0, 0]);
        assert_eq!(rotated.get_pixel(0, 0).0, [5, 0, 0]);
        assert_eq!(rotated.get_pixel(0, 1).0, [6, 0, 0]);
    }

    #[test]
    fn pipeline_matches_reference_tendency_for_fixture_samples() {
        let pre = load_fixture("pre.png");
        let post = load_fixture("post.png");
        let options = DitherOptions {
            use_lab: false,
            use_atkinson: false,
            use_zigzag: false,
            diffusion_rate: 1.0,
            saturation_mode: SaturationMode::Boosted,
            neutral_bias: 0.0,
            chroma_bias: 0.0,
            hue_guard: 0.0,
            blue_bias: 0.0,
            highlight_guard: 0.0,
            skin_tone_guard: 0.0,
        };
        let from_pre = render_profile_image(&pre, options);
        let from_post = rotate_right_90(&apply_reference_dither(&post, options));
        let mismatch_count = from_pre
            .pixels()
            .zip(from_post.pixels())
            .filter(|(left, right)| left.0 != right.0)
            .count();
        let pre_histogram = palette_histogram(&from_pre);
        let post_histogram = palette_histogram(&from_post);
        let histogram_delta: usize = pre_histogram
            .iter()
            .zip(post_histogram)
            .map(|(left, right)| left.abs_diff(right))
            .sum();

        assert_eq!(from_pre.dimensions(), from_post.dimensions());
        assert!(
            mismatch_count <= 400,
            "fixture mismatch count too high: {mismatch_count} pre={pre_histogram:?} post={post_histogram:?} histogram_delta={histogram_delta}"
        );
        assert!(
            histogram_delta <= 120,
            "fixture histogram delta too high: {histogram_delta} pre={pre_histogram:?} post={post_histogram:?}"
        );
    }

    #[test]
    fn split_view_uses_compare_profile_on_left_and_profile_on_right() {
        let input = load_fixture("pre.png");
        let compared =
            render_profile_image(&input, ImageProfile::HueGuard.default_dither_options());
        let candidate =
            render_profile_image(&input, ImageProfile::ColorPriority.default_dither_options());
        let merged = render_transformed_image(
            &input,
            RenderOptions {
                profile: ImageProfile::ColorPriority,
                dither_options: ImageProfile::ColorPriority.default_dither_options(),
                compare: CompareOptions {
                    profile: Some(ImageProfile::HueGuard),
                    split: CompareSplit::Vertical,
                },
            },
        );

        let midpoint = merged.width() / 2;
        assert_eq!(merged.dimensions(), compared.dimensions());
        assert_eq!(*merged.get_pixel(0, 0), *compared.get_pixel(0, 0));
        assert_eq!(
            *merged.get_pixel(midpoint.max(1), 0),
            *candidate.get_pixel(midpoint.max(1), 0)
        );
    }

    #[test]
    fn adaptive_photo_profile_renders_valid_binary_frame() {
        let input = load_fixture("pre.png");
        let response = render_response(
            &input,
            ResponseFormat::Binary,
            RenderOptions {
                profile: ImageProfile::AdaptivePhoto,
                dither_options: ImageProfile::AdaptivePhoto.default_dither_options(),
                compare: CompareOptions {
                    profile: Some(ImageProfile::ColorPriority),
                    split: CompareSplit::Horizontal,
                },
            },
        )
        .expect("adaptive profile response");

        let ResponsePayload::Binary(bytes) = response else {
            panic!("expected binary response");
        };

        assert_eq!(validate_binary_frame(&bytes), Ok(()));
    }
}

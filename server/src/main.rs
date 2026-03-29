use std::env;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};

use axum::Router;
use axum::body::Body;
use axum::extract::State;
use axum::http::header::{CONTENT_TYPE, HeaderValue};
use axum::http::{Response, StatusCode};
use axum::routing::get;
use image::codecs::bmp::BmpEncoder;
use image::{ExtendedColorType, ImageBuffer, ImageReader, Rgb, RgbImage};

const INPUT_IMAGE_NAME: &str = "image.png";
const OUTPUT_IMAGE_NAME: &str = "image.bmp";
const SATURATION_SCALE: f32 = 1.52;
const SATURATION_BIAS: f32 = 0.29;
const VALUE_SCALE: f32 = 1.02;
const VALUE_SATURATION_SCALE: f32 = 0.14;
const VALUE_BIAS: f32 = 0.15;
const SATURATION_TOLERANCE: u8 = 6;
const REFERENCE_PALETTE: [[u8; 3]; 7] = [
    [0, 0, 0],
    [255, 255, 255],
    [255, 255, 0],
    [255, 0, 0],
    [0, 0, 0],
    [0, 0, 255],
    [0, 255, 0],
];

#[derive(Clone, Debug)]
struct AppState {
    content_dir: PathBuf,
}

#[derive(Debug)]
enum ImageLoadError {
    Missing(PathBuf),
    Io(PathBuf, std::io::Error),
    Decode(PathBuf, image::ImageError),
}

impl ImageLoadError {
    fn into_response(self) -> Response<Body> {
        match self {
            Self::Missing(path) => text_response(
                StatusCode::NOT_FOUND,
                format!(
                    "{INPUT_IMAGE_NAME} is not configured. Place {INPUT_IMAGE_NAME} in {} and retry.\n",
                    path.parent().unwrap_or_else(|| Path::new(".")).display()
                ),
            ),
            Self::Decode(path, err) => text_response(
                StatusCode::UNPROCESSABLE_ENTITY,
                format!(
                    "failed to decode {INPUT_IMAGE_NAME} at {}: {err}\n",
                    path.display()
                ),
            ),
            Self::Io(path, err) => text_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!(
                    "failed to read {INPUT_IMAGE_NAME} at {}: {err}\n",
                    path.display()
                ),
            ),
        }
    }
}

#[derive(Debug)]
enum TransformError {
    Encode(image::ImageError),
}

impl TransformError {
    fn into_response(self) -> Response<Body> {
        match self {
            Self::Encode(err) => text_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to encode {OUTPUT_IMAGE_NAME}: {err}\n"),
            ),
        }
    }
}

fn default_content_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("contents")
}

fn read_content_dir() -> PathBuf {
    env::var_os("CONTENT_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(default_content_dir)
}

fn input_image_path_from_dir(content_dir: &Path) -> PathBuf {
    content_dir.join(INPUT_IMAGE_NAME)
}

fn read_port() -> Result<u16, String> {
    match env::var("PORT") {
        Ok(raw) => raw
            .parse::<u16>()
            .map_err(|_| "PORT は 0-65535 の数値で指定してください".to_string()),
        Err(_) => Ok(8000),
    }
}

fn build_app(state: AppState) -> Router {
    Router::new()
        .route("/", get(serve_image))
        .route("/image.bmp", get(serve_image))
        .with_state(state)
}

async fn serve_image(State(state): State<AppState>) -> Response<Body> {
    let input_path = input_image_path_from_dir(&state.content_dir);
    let image = match load_input_image(&input_path) {
        Ok(image) => image,
        Err(err) => return err.into_response(),
    };

    match render_bmp_response(&image) {
        Ok(bytes) => bmp_response(bytes),
        Err(err) => err.into_response(),
    }
}

fn load_input_image(path: &Path) -> Result<RgbImage, ImageLoadError> {
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

fn render_bmp_response(input: &RgbImage) -> Result<Vec<u8>, TransformError> {
    let saturated = boost_saturation(input);
    let dithered = apply_reference_dither(&saturated);
    let rotated = rotate_right_90(&dithered);
    encode_bmp_24(&rotated).map_err(TransformError::Encode)
}

fn encode_bmp_24(image: &RgbImage) -> Result<Vec<u8>, image::ImageError> {
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

fn boost_saturation(image: &RgbImage) -> RgbImage {
    let mut output = RgbImage::new(image.width(), image.height());

    for (x, y, pixel) in image.enumerate_pixels() {
        let [r, g, b] = pixel.0;
        let (h, s, v) = rgb_to_hsv(r, g, b);
        let (boosted_s, boosted_v) = if s <= f32::EPSILON {
            (0.0, v)
        } else {
            (
                (s * SATURATION_SCALE + SATURATION_BIAS).clamp(0.0, 1.0),
                (v * VALUE_SCALE + s * VALUE_SATURATION_SCALE + VALUE_BIAS).clamp(0.0, 1.0),
            )
        };
        output.put_pixel(x, y, Rgb(hsv_to_rgb(h, boosted_s, boosted_v)));
    }

    output
}

fn rgb_to_hsv(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;
    let max = r.max(g.max(b));
    let min = r.min(g.min(b));
    let delta = max - min;

    if delta.abs() < f32::EPSILON {
        return (0.0, 0.0, max);
    }

    let s = if max <= f32::EPSILON {
        0.0
    } else {
        delta / max
    };
    let h_base = if (max - r).abs() < f32::EPSILON {
        ((g - b) / delta).rem_euclid(6.0)
    } else if (max - g).abs() < f32::EPSILON {
        ((b - r) / delta) + 2.0
    } else {
        ((r - g) / delta) + 4.0
    };

    (60.0 * h_base, s, max)
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> [u8; 3] {
    if s <= f32::EPSILON {
        let gray = (v * 255.0).round() as u8;
        return [gray, gray, gray];
    }

    let c = v * s;
    let h_prime = h / 60.0;
    let x = c * (1.0 - ((h_prime.rem_euclid(2.0)) - 1.0).abs());
    let (r1, g1, b1) = match h_prime {
        hp if (0.0..1.0).contains(&hp) => (c, x, 0.0),
        hp if (1.0..2.0).contains(&hp) => (x, c, 0.0),
        hp if (2.0..3.0).contains(&hp) => (0.0, c, x),
        hp if (3.0..4.0).contains(&hp) => (0.0, x, c),
        hp if (4.0..5.0).contains(&hp) => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let m = v - c;

    [
        ((r1 + m) * 255.0).round().clamp(0.0, 255.0) as u8,
        ((g1 + m) * 255.0).round().clamp(0.0, 255.0) as u8,
        ((b1 + m) * 255.0).round().clamp(0.0, 255.0) as u8,
    ]
}

fn apply_reference_dither(image: &RgbImage) -> RgbImage {
    let width = image.width() as usize;
    let height = image.height() as usize;
    let mut work = image
        .pixels()
        .map(|pixel| pixel.0.map(|channel| channel as f32))
        .collect::<Vec<_>>();
    let mut output = RgbImage::new(image.width(), image.height());

    for y in 0..height {
        for x in 0..width {
            let index = y * width + x;
            let old = work[index];
            let replacement = nearest_palette_color(old);
            output.put_pixel(x as u32, y as u32, Rgb(replacement));
            let error = [
                old[0] - replacement[0] as f32,
                old[1] - replacement[1] as f32,
                old[2] - replacement[2] as f32,
            ];

            diffuse_error(&mut work, width, height, x + 1, y, error, 7.0 / 16.0);
            if x > 0 {
                diffuse_error(&mut work, width, height, x - 1, y + 1, error, 3.0 / 16.0);
            }
            diffuse_error(&mut work, width, height, x, y + 1, error, 5.0 / 16.0);
            diffuse_error(&mut work, width, height, x + 1, y + 1, error, 1.0 / 16.0);
        }
    }

    output
}

fn diffuse_error(
    work: &mut [[f32; 3]],
    width: usize,
    height: usize,
    x: usize,
    y: usize,
    error: [f32; 3],
    factor: f32,
) {
    if x >= width || y >= height {
        return;
    }

    let pixel = &mut work[y * width + x];
    for channel in 0..3 {
        pixel[channel] += error[channel] * factor;
    }
}

fn nearest_palette_color(pixel: [f32; 3]) -> [u8; 3] {
    let mut best = REFERENCE_PALETTE[0];
    let mut best_distance = f32::MAX;

    for candidate in REFERENCE_PALETTE {
        let distance = squared_distance(pixel, candidate);
        if distance < best_distance {
            best = candidate;
            best_distance = distance;
        }
    }

    best
}

fn squared_distance(pixel: [f32; 3], candidate: [u8; 3]) -> f32 {
    let dr = pixel[0].clamp(0.0, 255.0) - candidate[0] as f32;
    let dg = pixel[1].clamp(0.0, 255.0) - candidate[1] as f32;
    let db = pixel[2].clamp(0.0, 255.0) - candidate[2] as f32;
    dr * dr + dg * dg + db * db
}

fn rotate_right_90(image: &RgbImage) -> RgbImage {
    let mut rotated = ImageBuffer::new(image.height(), image.width());

    for (x, y, pixel) in image.enumerate_pixels() {
        let new_x = image.height() - 1 - y;
        let new_y = x;
        rotated.put_pixel(new_x, new_y, *pixel);
    }

    rotated
}

fn bmp_response(bytes: Vec<u8>) -> Response<Body> {
    let mut response = Response::new(Body::from(bytes));
    *response.status_mut() = StatusCode::OK;
    response
        .headers_mut()
        .insert(CONTENT_TYPE, HeaderValue::from_static("image/bmp"));
    response
}

fn text_response(status: StatusCode, body: impl Into<Body>) -> Response<Body> {
    let mut response = Response::new(body.into());
    *response.status_mut() = status;
    response.headers_mut().insert(
        CONTENT_TYPE,
        HeaderValue::from_static("text/plain; charset=utf-8"),
    );
    response
}

fn startup_messages(content_dir: &Path, port: u16) -> Vec<String> {
    vec![
        format!(
            "Serving transformed {OUTPUT_IMAGE_NAME} from {}",
            input_image_path_from_dir(content_dir).display()
        ),
        format!("Listen: http://0.0.0.0:{port}/ and http://0.0.0.0:{port}/image.bmp"),
        format!("Local:  http://127.0.0.1:{port}/ and http://127.0.0.1:{port}/image.bmp"),
        format!("LAN:    use this host's IP address with port {port} from other devices"),
        "Stop: Ctrl+C".to_string(),
    ]
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = read_port().map_err(std::io::Error::other)?;
    let address = SocketAddr::from(([0, 0, 0, 0], port));
    let content_dir = read_content_dir();
    let state = AppState { content_dir };
    let listener = tokio::net::TcpListener::bind(address).await?;

    for line in startup_messages(&state.content_dir, port) {
        println!("{line}");
    }

    axum::serve(listener, build_app(state)).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::{Bytes, to_bytes};
    use axum::http::Request;
    use image::ImageFormat;
    use std::ffi::OsString;
    use tower::ServiceExt;

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

    fn temp_path(prefix: &str) -> PathBuf {
        let unique = format!(
            "{}-{}-{}",
            prefix,
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system time before unix epoch")
                .as_nanos()
        );
        env::temp_dir().join(unique)
    }

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

    fn create_content_dir(prefix: &str, pixels: &[(u32, u32, [u8; 3])]) -> PathBuf {
        let dir = temp_path(prefix);
        std::fs::create_dir_all(&dir).expect("create temp dir");
        create_sample_png(&input_image_path_from_dir(&dir), pixels, 2, 2);
        dir
    }

    fn cleanup_dir(path: &Path) {
        let _ = std::fs::remove_dir_all(path);
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

    async fn response_body(response: Response<Body>) -> Bytes {
        to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("read response body")
    }

    struct EnvGuard {
        key: &'static str,
        previous: Option<OsString>,
    }

    impl EnvGuard {
        fn set(key: &'static str, value: &Path) -> Self {
            let previous = env::var_os(key);
            unsafe {
                env::set_var(key, value);
            }
            Self { key, previous }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            match &self.previous {
                Some(value) => unsafe {
                    env::set_var(self.key, value);
                },
                None => unsafe {
                    env::remove_var(self.key);
                },
            }
        }
    }

    #[test]
    fn content_dir_env_override_is_used() {
        let override_dir = temp_path("contents-dir");
        std::fs::create_dir_all(&override_dir).expect("create dir");
        let _guard = EnvGuard::set("CONTENT_DIR", &override_dir);

        assert_eq!(read_content_dir(), override_dir);
        cleanup_dir(&override_dir);
    }

    #[test]
    fn startup_messages_include_bind_and_input_guidance() {
        let messages = startup_messages(Path::new("/tmp/example"), 8000);

        assert!(messages.iter().any(|line| line.contains("0.0.0.0:8000")));
        assert!(messages.iter().any(|line| line.contains("127.0.0.1:8000")));
        assert!(
            messages
                .iter()
                .any(|line| line.contains("/tmp/example/image.png"))
        );
    }

    #[test]
    fn load_input_image_reads_png_when_present() {
        let dir = create_content_dir("load-image", &[(0, 0, [12, 34, 56])]);
        let image = load_input_image(&input_image_path_from_dir(&dir)).expect("png should load");

        assert_eq!(image.width(), 2);
        assert_eq!(image.height(), 2);
        assert_eq!(image.get_pixel(0, 0).0, [12, 34, 56]);

        cleanup_dir(&dir);
    }

    #[test]
    fn encode_bmp_24_writes_bmp_header_and_bit_depth() {
        let image = RgbImage::from_pixel(2, 3, Rgb([1, 2, 3]));
        let encoded = encode_bmp_24(&image).expect("bmp encoding");

        assert_eq!(&encoded[0..2], b"BM");
        assert_eq!(&encoded[28..30], &[24, 0]);
    }

    #[tokio::test]
    async fn root_and_image_bmp_return_same_transformed_bytes() {
        let dir = create_content_dir(
            "serve-image",
            &[
                (0, 0, [255, 64, 64]),
                (1, 0, [64, 255, 64]),
                (0, 1, [64, 64, 255]),
            ],
        );
        let router = build_app(AppState {
            content_dir: dir.clone(),
        });

        let root = router
            .clone()
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .expect("root response");
        let image = router
            .oneshot(
                Request::builder()
                    .uri("/image.bmp")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("image response");

        assert_eq!(root.status(), StatusCode::OK);
        assert_eq!(image.status(), StatusCode::OK);
        assert_eq!(
            root.headers().get(CONTENT_TYPE),
            Some(&HeaderValue::from_static("image/bmp"))
        );
        assert_eq!(
            image.headers().get(CONTENT_TYPE),
            Some(&HeaderValue::from_static("image/bmp"))
        );

        let root_body = response_body(root).await;
        let image_body = response_body(image).await;

        assert_eq!(root_body, image_body);
        assert_eq!(&root_body[..2], b"BM");
        assert_eq!(&root_body[28..30], &[24, 0]);

        cleanup_dir(&dir);
    }

    #[test]
    fn saturation_fixture_matches_post_pixels_within_tolerance() {
        let pre = load_fixture("pre.png");
        let post = load_fixture("post.png");
        let boosted = boost_saturation(&pre);

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

        let dithered = apply_reference_dither(&input);

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
        let from_pre = rotate_right_90(&apply_reference_dither(&boost_saturation(&pre)));
        let from_post = rotate_right_90(&apply_reference_dither(&post));
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

    #[tokio::test]
    async fn missing_image_returns_not_found_for_both_routes() {
        let missing_dir = temp_path("missing-content");
        std::fs::create_dir_all(&missing_dir).expect("create missing dir");
        let router = build_app(AppState {
            content_dir: missing_dir.clone(),
        });

        let root = router
            .clone()
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .expect("root response");
        let image = router
            .oneshot(
                Request::builder()
                    .uri("/image.bmp")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("image response");

        assert_eq!(root.status(), StatusCode::NOT_FOUND);
        assert_eq!(image.status(), StatusCode::NOT_FOUND);
        assert_eq!(
            root.headers().get(CONTENT_TYPE),
            Some(&HeaderValue::from_static("text/plain; charset=utf-8"))
        );

        let root_body = response_body(root).await;
        let image_body = response_body(image).await;
        let root_text = String::from_utf8(root_body.to_vec()).expect("utf8 root body");
        let image_text = String::from_utf8(image_body.to_vec()).expect("utf8 image body");

        assert!(root_text.contains("image.png is not configured"));
        assert_eq!(root_text, image_text);

        cleanup_dir(&missing_dir);
    }

    #[tokio::test]
    async fn invalid_png_returns_unprocessable_entity_for_both_routes() {
        let dir = temp_path("invalid-content");
        std::fs::create_dir_all(&dir).expect("create invalid dir");
        std::fs::write(input_image_path_from_dir(&dir), b"not-a-valid-png").expect("write invalid");
        let router = build_app(AppState {
            content_dir: dir.clone(),
        });

        let root = router
            .clone()
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .expect("root response");
        let image = router
            .oneshot(
                Request::builder()
                    .uri("/image.bmp")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("image response");

        assert_eq!(root.status(), StatusCode::UNPROCESSABLE_ENTITY);
        assert_eq!(image.status(), StatusCode::UNPROCESSABLE_ENTITY);

        let root_text = String::from_utf8(response_body(root).await.to_vec()).expect("utf8 root");
        let image_text =
            String::from_utf8(response_body(image).await.to_vec()).expect("utf8 image");

        assert!(root_text.contains("failed to decode image.png"));
        assert_eq!(root_text, image_text);

        cleanup_dir(&dir);
    }

    #[tokio::test]
    async fn replacing_input_image_changes_next_fetch_result() {
        let dir = create_content_dir("replace-image", &[(0, 0, [255, 0, 0])]);
        let router = build_app(AppState {
            content_dir: dir.clone(),
        });

        let first = router
            .clone()
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .expect("first response");
        let first_body = response_body(first).await;

        create_sample_png(
            &input_image_path_from_dir(&dir),
            &[(0, 0, [0, 255, 0]), (1, 1, [0, 0, 255])],
            2,
            2,
        );

        let second = router
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .expect("second response");
        let second_body = response_body(second).await;

        assert_ne!(first_body, second_body);

        cleanup_dir(&dir);
    }
}

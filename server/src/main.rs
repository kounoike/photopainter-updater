use std::env;
use std::fmt;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use axum::Router;
use axum::body::Body;
use axum::extract::{ConnectInfo, Request, State};
use axum::http::header::{CONTENT_LENGTH, CONTENT_TYPE, HeaderValue};
use axum::http::{Method, Response, StatusCode};
use axum::routing::{any, get};
use image::codecs::bmp::BmpEncoder;
use image::{ExtendedColorType, ImageBuffer, ImageReader, Rgb, RgbImage};

const INPUT_IMAGE_NAME: &str = "image.png";
const OUTPUT_IMAGE_NAME: &str = "image.bmp";
const BINARY_OUTPUT_NAME: &str = "image.bin";
const BINARY_CONTENT_TYPE: &str = "application/vnd.photopainter-frame";
const BINARY_FRAME_MAGIC: [u8; 4] = *b"PPBF";
const BINARY_FRAME_VERSION: u8 = 1;
const BINARY_FRAME_FLAGS: u8 = 0;
const BINARY_HEADER_LENGTH: u16 = 20;
const EPD_DISPLAY_WIDTH: usize = 800;
const EPD_DISPLAY_HEIGHT: usize = 480;
const SATURATION_SCALE: f32 = 1.52;
const SATURATION_BIAS: f32 = 0.29;
const VALUE_SCALE: f32 = 1.02;
const VALUE_SATURATION_SCALE: f32 = 0.14;
const VALUE_BIAS: f32 = 0.15;
#[cfg(test)]
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

#[derive(Clone)]
struct AppState {
    content_dir: PathBuf,
    logger: Arc<dyn AccessLogger>,
    request_counter: Arc<AtomicU64>,
    use_lab: bool,
    use_atkinson: bool,
    diffusion_rate: f32,
    use_zigzag: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct AccessLogEntry {
    request_id: u64,
    timestamp: String,
    remote: String,
    method: Method,
    path: String,
    status: StatusCode,
    outcome: LogOutcome,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LogOutcome {
    Success,
    InputMissing,
    TransformFailed,
    NotFound,
    InternalError,
}

impl fmt::Display for LogOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Success => write!(f, "success"),
            Self::InputMissing => write!(f, "input-missing"),
            Self::TransformFailed => write!(f, "transform-failed"),
            Self::NotFound => write!(f, "not-found"),
            Self::InternalError => write!(f, "internal-error"),
        }
    }
}

trait AccessLogger: Send + Sync {
    fn record(&self, entry: &AccessLogEntry);
}

#[derive(Debug)]
struct StdoutAccessLogger;

impl AccessLogger for StdoutAccessLogger {
    fn record(&self, entry: &AccessLogEntry) {
        println!("{}", format_access_log_line(entry));
    }
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
        .route("/", get(serve_binary_image))
        .route("/image.bmp", get(serve_image))
        .route("/image.bin", get(serve_binary_image))
        .fallback(any(serve_not_found))
        .with_state(state)
}

async fn serve_image(State(state): State<AppState>, request: Request) -> Response<Body> {
    serve_transformed(state, request, ResponseFormat::Bmp).await
}

async fn serve_binary_image(State(state): State<AppState>, request: Request) -> Response<Body> {
    serve_transformed(state, request, ResponseFormat::Binary).await
}

async fn serve_transformed(
    state: AppState,
    request: Request,
    response_format: ResponseFormat,
) -> Response<Body> {
    let method = request.method().clone();
    let path = request.uri().path().to_string();
    let remote = extract_remote_addr(&request);
    let input_path = input_image_path_from_dir(&state.content_dir);
    let image = match load_input_image(&input_path) {
        Ok(image) => image,
        Err(err) => {
            let outcome = image_load_error_outcome(&err);
            let response = err.into_response();
            log_request(&state, method, path, remote, response.status(), outcome);
            return response;
        }
    };

    let (response, outcome) = match render_response(&image, response_format, state.use_lab, state.use_atkinson, state.diffusion_rate, state.use_zigzag) {
        Ok(response) => (response, LogOutcome::Success),
        Err(err) => {
            let outcome = transform_error_outcome(&err);
            (err.into_response(), outcome)
        }
    };
    log_request(&state, method, path, remote, response.status(), outcome);
    response
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ResponseFormat {
    Bmp,
    Binary,
}

async fn serve_not_found(State(state): State<AppState>, request: Request) -> Response<Body> {
    let method = request.method().clone();
    let path = request.uri().path().to_string();
    let remote = extract_remote_addr(&request);
    let response = text_response(StatusCode::NOT_FOUND, "route not found\n");
    log_request(
        &state,
        method,
        path,
        remote,
        response.status(),
        LogOutcome::NotFound,
    );
    response
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

fn render_response(
    input: &RgbImage,
    response_format: ResponseFormat,
    use_lab: bool,
    use_atkinson: bool,
    diffusion_rate: f32,
    use_zigzag: bool,
) -> Result<Response<Body>, TransformError> {
    match response_format {
        ResponseFormat::Bmp => Ok(bmp_response(render_bmp_response(input, use_lab, use_atkinson, diffusion_rate, use_zigzag)?)),
        ResponseFormat::Binary => Ok(binary_frame_response(render_binary_frame_response(input, use_lab, use_atkinson, diffusion_rate, use_zigzag)?)),
    }
}

fn render_transformed_image(input: &RgbImage, use_lab: bool, use_atkinson: bool, diffusion_rate: f32, use_zigzag: bool) -> RgbImage {
    let saturated = boost_saturation(input);
    let dithered = apply_reference_dither(&saturated, use_lab, use_atkinson, diffusion_rate, use_zigzag);
    rotate_right_90(&dithered)
}

fn render_bmp_response(input: &RgbImage, use_lab: bool, use_atkinson: bool, diffusion_rate: f32, use_zigzag: bool) -> Result<Vec<u8>, TransformError> {
    let rotated = render_transformed_image(input, use_lab, use_atkinson, diffusion_rate, use_zigzag);
    encode_bmp_24(&rotated).map_err(TransformError::Encode)
}

fn render_binary_frame_response(input: &RgbImage, use_lab: bool, use_atkinson: bool, diffusion_rate: f32, use_zigzag: bool) -> Result<Vec<u8>, TransformError> {
    let rotated = render_transformed_image(input, use_lab, use_atkinson, diffusion_rate, use_zigzag);
    Ok(encode_binary_frame(&rotated))
}

fn image_load_error_outcome(error: &ImageLoadError) -> LogOutcome {
    match error {
        ImageLoadError::Missing(_) => LogOutcome::InputMissing,
        ImageLoadError::Decode(_, _) => LogOutcome::TransformFailed,
        ImageLoadError::Io(_, _) => LogOutcome::InternalError,
    }
}

fn transform_error_outcome(error: &TransformError) -> LogOutcome {
    match error {
        TransformError::Encode(_) => LogOutcome::InternalError,
    }
}

fn log_request(
    state: &AppState,
    method: Method,
    path: String,
    remote: String,
    status: StatusCode,
    outcome: LogOutcome,
) {
    let entry = AccessLogEntry {
        request_id: state.request_counter.fetch_add(1, Ordering::Relaxed) + 1,
        timestamp: current_timestamp(),
        remote,
        method,
        path,
        status,
        outcome,
    };
    state.logger.record(&entry);
}

fn extract_remote_addr(request: &Request) -> String {
    if let Some(remote) = request.extensions().get::<ConnectInfo<SocketAddr>>() {
        return remote.0.to_string();
    }
    if let Some(remote) = request.extensions().get::<SocketAddr>() {
        return remote.to_string();
    }
    "-".to_string()
}

fn current_timestamp() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    format!("{}.{:03}", now.as_secs(), now.subsec_millis())
}

fn format_access_log_line(entry: &AccessLogEntry) -> String {
    format!(
        "request_id={} timestamp={} remote={} method={} path={} status={} outcome={}",
        entry.request_id,
        entry.timestamp,
        entry.remote,
        entry.method,
        entry.path,
        entry.status.as_u16(),
        entry.outcome
    )
}

fn encode_binary_frame(image: &RgbImage) -> Vec<u8> {
    let payload = pack_epaper_frame(image);
    let payload_length = payload.len() as u32;
    let checksum = payload_checksum(&payload);
    let mut bytes = Vec::with_capacity(BINARY_HEADER_LENGTH as usize + payload.len());
    bytes.extend_from_slice(&BINARY_FRAME_MAGIC);
    bytes.push(BINARY_FRAME_VERSION);
    bytes.push(BINARY_FRAME_FLAGS);
    bytes.extend_from_slice(&BINARY_HEADER_LENGTH.to_le_bytes());
    bytes.extend_from_slice(&(EPD_DISPLAY_WIDTH as u16).to_le_bytes());
    bytes.extend_from_slice(&(EPD_DISPLAY_HEIGHT as u16).to_le_bytes());
    bytes.extend_from_slice(&payload_length.to_le_bytes());
    bytes.extend_from_slice(&checksum.to_le_bytes());
    bytes.extend_from_slice(&payload);
    bytes
}

fn pack_epaper_frame(image: &RgbImage) -> Vec<u8> {
    let width = EPD_DISPLAY_WIDTH;
    let height = EPD_DISPLAY_HEIGHT;
    let width_bytes = width.div_ceil(2);
    let mut buffer = vec![0x11; width_bytes * height];

    for (x, y, pixel) in image.enumerate_pixels() {
        if x as usize >= width || y as usize >= height {
            continue;
        }
        let color = palette_index_for_rgb(pixel.0);
        let storage_x = width - 1 - x as usize;
        let storage_y = height - 1 - y as usize;
        let addr = storage_x / 2 + storage_y * width_bytes;
        let nibble = color << 4;
        if storage_x % 2 == 0 {
            buffer[addr] = (buffer[addr] & 0x0F) | nibble;
        } else {
            buffer[addr] = (buffer[addr] & 0xF0) | color;
        }
    }

    buffer
}

fn palette_index_for_rgb(pixel: [u8; 3]) -> u8 {
    match pixel {
        [0, 0, 0] => 0,
        [255, 255, 255] => 1,
        [255, 255, 0] => 2,
        [255, 0, 0] => 3,
        [0, 0, 255] => 5,
        [0, 255, 0] => 6,
        other => panic!("unexpected transformed color {other:?}"),
    }
}

fn payload_checksum(payload: &[u8]) -> u32 {
    payload
        .iter()
        .fold(0u32, |acc, byte| acc.wrapping_add(u32::from(*byte)))
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

fn apply_reference_dither(image: &RgbImage, use_lab: bool, use_atkinson: bool, diffusion_rate: f32, use_zigzag: bool) -> RgbImage {
    let width = image.width() as usize;
    let height = image.height() as usize;
    let mut work = image
        .pixels()
        .map(|pixel| pixel.0.map(|channel| channel as f32))
        .collect::<Vec<_>>();
    let mut output = RgbImage::new(image.width(), image.height());

    for y in 0..height {
        let reverse = use_zigzag && y % 2 == 1;
        let xs: Box<dyn Iterator<Item = usize>> = if reverse {
            Box::new((0..width).rev())
        } else {
            Box::new(0..width)
        };
        for x in xs {
            let index = y * width + x;
            let old = work[index];
            let clamped = [
                old[0].clamp(0.0, 255.0),
                old[1].clamp(0.0, 255.0),
                old[2].clamp(0.0, 255.0),
            ];
            let replacement = nearest_palette_color(clamped, use_lab);
            output.put_pixel(x as u32, y as u32, Rgb(replacement));
            let error = [
                (clamped[0] - replacement[0] as f32) * diffusion_rate,
                (clamped[1] - replacement[1] as f32) * diffusion_rate,
                (clamped[2] - replacement[2] as f32) * diffusion_rate,
            ];

            if use_atkinson {
                // Atkinson: distribute 1/8 error to 6 neighbors (75% total)
                // Neighbor offsets are mirrored horizontally on reverse rows.
                if !reverse {
                    diffuse_error(&mut work, width, height, x + 1, y, error, 1.0 / 8.0);
                    diffuse_error(&mut work, width, height, x + 2, y, error, 1.0 / 8.0);
                    if x > 0 {
                        diffuse_error(&mut work, width, height, x - 1, y + 1, error, 1.0 / 8.0);
                    }
                    diffuse_error(&mut work, width, height, x, y + 1, error, 1.0 / 8.0);
                    diffuse_error(&mut work, width, height, x + 1, y + 1, error, 1.0 / 8.0);
                    diffuse_error(&mut work, width, height, x, y + 2, error, 1.0 / 8.0);
                } else {
                    if x > 0 {
                        diffuse_error(&mut work, width, height, x - 1, y, error, 1.0 / 8.0);
                    }
                    if x > 1 {
                        diffuse_error(&mut work, width, height, x - 2, y, error, 1.0 / 8.0);
                    }
                    diffuse_error(&mut work, width, height, x + 1, y + 1, error, 1.0 / 8.0);
                    diffuse_error(&mut work, width, height, x, y + 1, error, 1.0 / 8.0);
                    if x > 0 {
                        diffuse_error(&mut work, width, height, x - 1, y + 1, error, 1.0 / 8.0);
                    }
                    diffuse_error(&mut work, width, height, x, y + 2, error, 1.0 / 8.0);
                }
            } else {
                // Floyd-Steinberg: distribute 100% of error to 4 neighbors
                // Neighbor offsets are mirrored horizontally on reverse rows.
                if !reverse {
                    diffuse_error(&mut work, width, height, x + 1, y, error, 7.0 / 16.0);
                    if x > 0 {
                        diffuse_error(&mut work, width, height, x - 1, y + 1, error, 3.0 / 16.0);
                    }
                    diffuse_error(&mut work, width, height, x, y + 1, error, 5.0 / 16.0);
                    diffuse_error(&mut work, width, height, x + 1, y + 1, error, 1.0 / 16.0);
                } else {
                    if x > 0 {
                        diffuse_error(&mut work, width, height, x - 1, y, error, 7.0 / 16.0);
                    }
                    diffuse_error(&mut work, width, height, x + 1, y + 1, error, 3.0 / 16.0);
                    diffuse_error(&mut work, width, height, x, y + 1, error, 5.0 / 16.0);
                    if x > 0 {
                        diffuse_error(&mut work, width, height, x - 1, y + 1, error, 1.0 / 16.0);
                    }
                }
            }
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

fn nearest_palette_color(pixel: [f32; 3], use_lab: bool) -> [u8; 3] {
    let mut best = REFERENCE_PALETTE[0];
    let mut best_distance = f32::MAX;

    for candidate in REFERENCE_PALETTE {
        let distance = if use_lab {
            lab_squared_distance(pixel, candidate)
        } else {
            squared_distance(pixel, candidate)
        };
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

fn rgb_to_lab(r: f32, g: f32, b: f32) -> [f32; 3] {
    let linearize = |c: f32| -> f32 {
        let c = c.clamp(0.0, 255.0) / 255.0;
        if c <= 0.04045 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    };
    let r = linearize(r);
    let g = linearize(g);
    let b = linearize(b);
    let x = 0.4124564 * r + 0.3575761 * g + 0.1804375 * b;
    let y = 0.2126729 * r + 0.7151522 * g + 0.0721750 * b;
    let z = 0.0193339 * r + 0.1191920 * g + 0.9503041 * b;
    let f = |t: f32| -> f32 {
        if t > 0.008856 {
            t.cbrt()
        } else {
            7.787 * t + 16.0 / 116.0
        }
    };
    let fx = f(x / 0.95047);
    let fy = f(y / 1.0);
    let fz = f(z / 1.08883);
    [116.0 * fy - 16.0, 500.0 * (fx - fy), 200.0 * (fy - fz)]
}

fn lab_squared_distance(pixel: [f32; 3], candidate: [u8; 3]) -> f32 {
    let pl = rgb_to_lab(pixel[0], pixel[1], pixel[2]);
    let cl = rgb_to_lab(candidate[0] as f32, candidate[1] as f32, candidate[2] as f32);
    let dl = pl[0] - cl[0];
    let da = pl[1] - cl[1];
    let db = pl[2] - cl[2];
    dl * dl + da * da + db * db
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

fn binary_frame_response(bytes: Vec<u8>) -> Response<Body> {
    let content_length = bytes.len();
    let mut response = Response::new(Body::from(bytes));
    *response.status_mut() = StatusCode::OK;
    response
        .headers_mut()
        .insert(CONTENT_TYPE, HeaderValue::from_static(BINARY_CONTENT_TYPE));
    response.headers_mut().insert(
        CONTENT_LENGTH,
        HeaderValue::from_str(&content_length.to_string()).expect("valid content length"),
    );
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

fn startup_messages(content_dir: &Path, port: u16, state: &AppState) -> Vec<String> {
    let color_distance = if state.use_lab { "CIE Lab" } else { "RGB" };
    let algorithm = if state.use_atkinson { "Atkinson" } else { "Floyd-Steinberg" };
    let zigzag = if state.use_zigzag { "on" } else { "off" };
    vec![
        format!(
            "Serving transformed {OUTPUT_IMAGE_NAME} from {}",
            input_image_path_from_dir(content_dir).display()
        ),
        format!("Listen: http://0.0.0.0:{port}/ and http://0.0.0.0:{port}/image.bmp"),
        format!("Local:  http://127.0.0.1:{port}/ and http://127.0.0.1:{port}/image.bmp"),
        format!("LAN:    use this host's IP address with port {port} from other devices"),
        format!("Binary: http://127.0.0.1:{port}/{BINARY_OUTPUT_NAME} for firmware clients"),
        format!("Dither: {algorithm}, color={color_distance}, rate={:.2}, zigzag={zigzag}", state.diffusion_rate),
        "Access logs: one line per request is written to stdout".to_string(),
        "Stop: Ctrl+C".to_string(),
    ]
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = read_port().map_err(std::io::Error::other)?;
    let address = SocketAddr::from(([0, 0, 0, 0], port));
    let content_dir = read_content_dir();
    let use_lab = env::var("DITHER_USE_LAB").is_ok_and(|v| v == "1");
    let use_atkinson = env::var("DITHER_USE_ATKINSON").is_ok_and(|v| v == "1");
    let diffusion_rate = env::var("DITHER_DIFFUSION_RATE")
        .ok()
        .and_then(|v| v.parse::<f32>().ok())
        .unwrap_or(1.0)
        .clamp(0.0, 1.0);
    let use_zigzag = env::var("DITHER_ZIGZAG").is_ok_and(|v| v == "1");
    let state = AppState {
        content_dir,
        logger: Arc::new(StdoutAccessLogger),
        request_counter: Arc::new(AtomicU64::new(0)),
        use_lab,
        use_atkinson,
        diffusion_rate,
        use_zigzag,
    };
    let listener = tokio::net::TcpListener::bind(address).await?;

    for line in startup_messages(&state.content_dir, port, &state) {
        println!("{line}");
    }

    axum::serve(
        listener,
        build_app(state).into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::{Bytes, to_bytes};
    use axum::http::Request;
    use image::ImageFormat;
    use std::ffi::OsString;
    use std::sync::Mutex;
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

    #[derive(Debug, Default)]
    struct TestAccessLogger {
        entries: Mutex<Vec<AccessLogEntry>>,
    }

    impl AccessLogger for TestAccessLogger {
        fn record(&self, entry: &AccessLogEntry) {
            self.entries
                .lock()
                .expect("lock test logger")
                .push(entry.clone());
        }
    }

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

    fn test_state(content_dir: PathBuf) -> (AppState, Arc<TestAccessLogger>) {
        let logger = Arc::new(TestAccessLogger::default());
        (
            AppState {
                content_dir,
                logger: logger.clone(),
                request_counter: Arc::new(AtomicU64::new(0)),
                use_lab: false,
                use_atkinson: false,
                diffusion_rate: 1.0,
                use_zigzag: false,
            },
            logger,
        )
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

    fn logged_entries(logger: &Arc<TestAccessLogger>) -> Vec<AccessLogEntry> {
        logger.entries.lock().expect("lock logger entries").clone()
    }

    fn request_with_remote(uri: &str, remote: SocketAddr) -> Request<Body> {
        let mut request = Request::builder()
            .uri(uri)
            .body(Body::empty())
            .expect("build request");
        request.extensions_mut().insert(remote);
        request
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
        let dir = temp_path("startup-messages");
        let (state, _) = test_state(dir);
        let messages = startup_messages(Path::new("/tmp/example"), 8000, &state);

        assert!(messages.iter().any(|line| line.contains("0.0.0.0:8000")));
        assert!(messages.iter().any(|line| line.contains("127.0.0.1:8000")));
        assert!(
            messages
                .iter()
                .any(|line| line.contains("/tmp/example/image.png"))
        );
        assert!(messages.iter().any(|line| line.contains("image.bin")));
        assert!(messages.iter().any(|line| line.contains("Access logs")));
        assert!(messages.iter().any(|line| line.contains("Dither:")));
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

    #[tokio::test]
    async fn root_returns_binary_and_image_bmp_returns_bmp() {
        let dir = create_content_dir(
            "serve-image",
            &[
                (0, 0, [255, 64, 64]),
                (1, 0, [64, 255, 64]),
                (0, 1, [64, 64, 255]),
            ],
        );
        let (state, logger) = test_state(dir.clone());
        let router = build_app(state);

        let root = router
            .clone()
            .oneshot(request_with_remote(
                "/",
                "192.168.0.10:40123".parse().expect("remote addr"),
            ))
            .await
            .expect("root response");
        let image = router
            .oneshot(request_with_remote(
                "/image.bmp",
                "192.168.0.11:40124".parse().expect("remote addr"),
            ))
            .await
            .expect("image response");

        assert_eq!(root.status(), StatusCode::OK);
        assert_eq!(image.status(), StatusCode::OK);
        assert_eq!(
            root.headers().get(CONTENT_TYPE),
            Some(&HeaderValue::from_static("application/vnd.photopainter-frame"))
        );
        assert_eq!(
            image.headers().get(CONTENT_TYPE),
            Some(&HeaderValue::from_static("image/bmp"))
        );

        let root_body = response_body(root).await;
        let image_body = response_body(image).await;

        assert_eq!(&root_body[..4], b"PPBF");
        assert_eq!(&image_body[..2], b"BM");
        assert_eq!(&image_body[28..30], &[24, 0]);

        let entries = logged_entries(&logger);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].path, "/");
        assert_eq!(entries[0].status, StatusCode::OK);
        assert_eq!(entries[0].outcome, LogOutcome::Success);
        assert_eq!(entries[0].remote, "192.168.0.10:40123");
        assert_eq!(entries[1].path, "/image.bmp");
        assert_eq!(entries[1].status, StatusCode::OK);
        assert_eq!(entries[1].outcome, LogOutcome::Success);
        assert_eq!(entries[1].remote, "192.168.0.11:40124");

        cleanup_dir(&dir);
    }

    #[tokio::test]
    async fn image_bin_returns_binary_frame_headers_and_payload() {
        let dir = create_content_dir(
            "serve-binary",
            &[
                (0, 0, [255, 64, 64]),
                (1, 0, [64, 255, 64]),
                (0, 1, [64, 64, 255]),
            ],
        );
        let (state, logger) = test_state(dir.clone());
        let router = build_app(state);

        let response = router
            .oneshot(request_with_remote(
                "/image.bin",
                "192.168.0.12:40125".parse().expect("remote addr"),
            ))
            .await
            .expect("binary response");

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get(CONTENT_TYPE),
            Some(&HeaderValue::from_static(BINARY_CONTENT_TYPE))
        );
        assert!(response.headers().get(CONTENT_LENGTH).is_some());

        let body = response_body(response).await;
        let (width, height, payload_length, checksum, payload) = parse_binary_header(&body);
        assert_eq!(
            (width, height),
            (EPD_DISPLAY_WIDTH as u16, EPD_DISPLAY_HEIGHT as u16)
        );
        assert_eq!(
            payload_length,
            (EPD_DISPLAY_WIDTH * EPD_DISPLAY_HEIGHT / 2) as u32
        );
        assert_eq!(payload_checksum(payload), checksum);

        let entries = logged_entries(&logger);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].path, "/image.bin");
        assert_eq!(entries[0].status, StatusCode::OK);
        assert_eq!(entries[0].outcome, LogOutcome::Success);

        cleanup_dir(&dir);
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
    fn access_log_format_is_single_line_and_contains_required_fields() {
        let entry = AccessLogEntry {
            request_id: 7,
            timestamp: "123.456".to_string(),
            remote: "192.168.0.20:5000".to_string(),
            method: Method::GET,
            path: "/image.bmp".to_string(),
            status: StatusCode::OK,
            outcome: LogOutcome::Success,
        };

        let line = format_access_log_line(&entry);

        assert!(!line.contains('\n'));
        assert!(line.contains("request_id=7"));
        assert!(line.contains("timestamp=123.456"));
        assert!(line.contains("remote=192.168.0.20:5000"));
        assert!(line.contains("method=GET"));
        assert!(line.contains("path=/image.bmp"));
        assert!(line.contains("status=200"));
        assert!(line.contains("outcome=success"));
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

        let dithered = apply_reference_dither(&input, false, false, 1.0, false);

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
        let from_pre = rotate_right_90(&apply_reference_dither(&boost_saturation(&pre), false, false, 1.0, false));
        let from_post = rotate_right_90(&apply_reference_dither(&post, false, false, 1.0, false));
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
        let (state, logger) = test_state(missing_dir.clone());
        let router = build_app(state);

        let root = router
            .clone()
            .oneshot(request_with_remote(
                "/",
                "192.168.0.30:41000".parse().expect("remote addr"),
            ))
            .await
            .expect("root response");
        let image = router
            .oneshot(request_with_remote(
                "/image.bmp",
                "192.168.0.31:41001".parse().expect("remote addr"),
            ))
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

        let entries = logged_entries(&logger);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].status, StatusCode::NOT_FOUND);
        assert_eq!(entries[0].outcome, LogOutcome::InputMissing);
        assert_eq!(entries[1].status, StatusCode::NOT_FOUND);
        assert_eq!(entries[1].outcome, LogOutcome::InputMissing);

        cleanup_dir(&missing_dir);
    }

    #[tokio::test]
    async fn missing_image_returns_not_found_for_binary_route() {
        let missing_dir = temp_path("missing-binary-content");
        std::fs::create_dir_all(&missing_dir).expect("create missing dir");
        let (state, logger) = test_state(missing_dir.clone());
        let router = build_app(state);

        let response = router
            .oneshot(request_with_remote(
                "/image.bin",
                "192.168.0.32:41002".parse().expect("remote addr"),
            ))
            .await
            .expect("binary response");

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        assert_eq!(
            response.headers().get(CONTENT_TYPE),
            Some(&HeaderValue::from_static("text/plain; charset=utf-8"))
        );

        let body = response_body(response).await;
        let text = String::from_utf8(body.to_vec()).expect("utf8 binary body");
        assert!(text.contains("image.png is not configured"));

        let entries = logged_entries(&logger);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].path, "/image.bin");
        assert_eq!(entries[0].status, StatusCode::NOT_FOUND);
        assert_eq!(entries[0].outcome, LogOutcome::InputMissing);

        cleanup_dir(&missing_dir);
    }

    #[tokio::test]
    async fn invalid_png_returns_unprocessable_entity_for_both_routes() {
        let dir = temp_path("invalid-content");
        std::fs::create_dir_all(&dir).expect("create invalid dir");
        std::fs::write(input_image_path_from_dir(&dir), b"not-a-valid-png").expect("write invalid");
        let (state, logger) = test_state(dir.clone());
        let router = build_app(state);

        let root = router
            .clone()
            .oneshot(request_with_remote(
                "/",
                "192.168.0.40:42000".parse().expect("remote addr"),
            ))
            .await
            .expect("root response");
        let image = router
            .oneshot(request_with_remote(
                "/image.bmp",
                "192.168.0.41:42001".parse().expect("remote addr"),
            ))
            .await
            .expect("image response");

        assert_eq!(root.status(), StatusCode::UNPROCESSABLE_ENTITY);
        assert_eq!(image.status(), StatusCode::UNPROCESSABLE_ENTITY);

        let root_text = String::from_utf8(response_body(root).await.to_vec()).expect("utf8 root");
        let image_text =
            String::from_utf8(response_body(image).await.to_vec()).expect("utf8 image");

        assert!(root_text.contains("failed to decode image.png"));
        assert_eq!(root_text, image_text);

        let entries = logged_entries(&logger);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].status, StatusCode::UNPROCESSABLE_ENTITY);
        assert_eq!(entries[0].outcome, LogOutcome::TransformFailed);
        assert_eq!(entries[1].status, StatusCode::UNPROCESSABLE_ENTITY);
        assert_eq!(entries[1].outcome, LogOutcome::TransformFailed);

        cleanup_dir(&dir);
    }

    #[tokio::test]
    async fn invalid_png_returns_unprocessable_entity_for_binary_route() {
        let dir = temp_path("invalid-binary-content");
        std::fs::create_dir_all(&dir).expect("create invalid dir");
        std::fs::write(input_image_path_from_dir(&dir), b"not-a-valid-png").expect("write invalid");
        let (state, logger) = test_state(dir.clone());
        let router = build_app(state);

        let response = router
            .oneshot(request_with_remote(
                "/image.bin",
                "192.168.0.42:42002".parse().expect("remote addr"),
            ))
            .await
            .expect("binary response");

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
        assert_eq!(
            response.headers().get(CONTENT_TYPE),
            Some(&HeaderValue::from_static("text/plain; charset=utf-8"))
        );

        let text = String::from_utf8(response_body(response).await.to_vec()).expect("utf8 body");
        assert!(text.contains("failed to decode image.png"));

        let entries = logged_entries(&logger);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].path, "/image.bin");
        assert_eq!(entries[0].status, StatusCode::UNPROCESSABLE_ENTITY);
        assert_eq!(entries[0].outcome, LogOutcome::TransformFailed);

        cleanup_dir(&dir);
    }

    #[tokio::test]
    async fn replacing_input_image_changes_next_fetch_result() {
        let dir = create_content_dir("replace-image", &[(0, 0, [255, 0, 0])]);
        let (state, logger) = test_state(dir.clone());
        let router = build_app(state);

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
        let entries = logged_entries(&logger);
        assert_eq!(entries.len(), 2);
        assert_ne!(entries[0].request_id, entries[1].request_id);
        assert_eq!(entries[0].path, "/");
        assert_eq!(entries[1].path, "/");

        cleanup_dir(&dir);
    }

    #[tokio::test]
    async fn unknown_path_is_logged_and_returns_not_found() {
        let dir = create_content_dir("unknown-path", &[(0, 0, [255, 0, 0])]);
        let (state, logger) = test_state(dir.clone());
        let router = build_app(state);

        let response = router
            .oneshot(request_with_remote(
                "/unknown",
                "192.168.0.50:43000".parse().expect("remote addr"),
            ))
            .await
            .expect("unknown path response");

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let text = String::from_utf8(response_body(response).await.to_vec()).expect("utf8 body");
        assert!(text.contains("route not found"));

        let entries = logged_entries(&logger);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].path, "/unknown");
        assert_eq!(entries[0].status, StatusCode::NOT_FOUND);
        assert_eq!(entries[0].outcome, LogOutcome::NotFound);
        assert_eq!(entries[0].remote, "192.168.0.50:43000");

        cleanup_dir(&dir);
    }
}

use axum::Router;
use axum::body::Body;
use axum::extract::{Request, State};
use axum::http::{Response, StatusCode};
use axum::routing::{any, get};

use crate::app::AppState;
use crate::image_pipeline::{
    ResponseFormat, image_load_error_outcome, input_image_path_from_dir, load_input_image,
    render_response, transform_error_outcome,
};
use crate::logging::{LogOutcome, extract_remote_addr, log_request};
use crate::response::{
    image_load_error_response, response_from_payload, text_response, transform_error_response,
};

pub fn build_app(state: AppState) -> Router {
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
    let pipeline_request = crate::image_pipeline::ImagePipelineRequest {
        input_path: input_image_path_from_dir(&state.content_dir),
        response_format,
        render_options: state.render_options,
    };
    let image = match load_input_image(&pipeline_request.input_path) {
        Ok(image) => image,
        Err(err) => {
            let outcome = image_load_error_outcome(&err);
            let response = image_load_error_response(&err);
            record(&state, method, path, remote, response.status(), outcome);
            return response;
        }
    };

    let response = match render_response(
        &image,
        pipeline_request.response_format,
        pipeline_request.render_options,
    ) {
        Ok(payload) => {
            let response = response_from_payload(payload);
            record(
                &state,
                method,
                path,
                remote,
                response.status(),
                LogOutcome::Success,
            );
            response
        }
        Err(err) => {
            let outcome = transform_error_outcome(&err);
            let response = transform_error_response(&err, pipeline_request.response_format);
            record(&state, method, path, remote, response.status(), outcome);
            response
        }
    };

    response
}

async fn serve_not_found(State(state): State<AppState>, request: Request) -> Response<Body> {
    let method = request.method().clone();
    let path = request.uri().path().to_string();
    let remote = extract_remote_addr(&request);
    let response = text_response(StatusCode::NOT_FOUND, "route not found\n");
    record(
        &state,
        method,
        path,
        remote,
        response.status(),
        LogOutcome::NotFound,
    );
    response
}

fn record(
    state: &AppState,
    method: axum::http::Method,
    path: String,
    remote: String,
    status: StatusCode,
    outcome: LogOutcome,
) {
    log_request(
        &state.logger,
        &state.request_counter,
        method,
        path,
        remote,
        status,
        outcome,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;
    use std::path::{Path, PathBuf};
    use std::sync::Arc;

    use axum::body::to_bytes;
    use axum::http::header::{CONTENT_LENGTH, CONTENT_TYPE, HeaderValue};
    use image::ImageFormat;
    use image::{Rgb, RgbImage};
    use std::sync::Mutex;
    use tower::ServiceExt;

    use crate::logging::{AccessLogEvent, AccessLogger};

    #[derive(Debug, Default)]
    struct TestAccessLogger {
        entries: Mutex<Vec<AccessLogEvent>>,
    }

    impl AccessLogger for TestAccessLogger {
        fn record(&self, entry: &AccessLogEvent) {
            self.entries
                .lock()
                .expect("lock logger")
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
        std::env::temp_dir().join(unique)
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
        let state = AppState {
            content_dir,
            logger: logger.clone(),
            request_counter: Arc::new(std::sync::atomic::AtomicU64::new(0)),
            render_options: crate::config::RenderOptions {
                profile: crate::config::ImageProfile::Baseline,
                dither_options: crate::config::DitherOptions {
                    use_lab: false,
                    use_atkinson: false,
                    use_zigzag: false,
                    diffusion_rate: 1.0,
                    saturation_mode: crate::config::SaturationMode::Boosted,
                    neutral_bias: 0.0,
                    chroma_bias: 0.0,
                    hue_guard: 0.0,
                },
                compare: crate::config::CompareOptions {
                    profile: None,
                    split: crate::config::CompareSplit::Vertical,
                },
            },
        };
        (state, logger)
    }

    fn cleanup_dir(path: &Path) {
        let _ = std::fs::remove_dir_all(path);
    }

    async fn response_body(response: Response<Body>) -> axum::body::Bytes {
        to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("read response body")
    }

    fn logged_entries(logger: &Arc<TestAccessLogger>) -> Vec<AccessLogEvent> {
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
            Some(&HeaderValue::from_static(
                crate::config::BINARY_CONTENT_TYPE
            ))
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
            Some(&HeaderValue::from_static(
                crate::config::BINARY_CONTENT_TYPE
            ))
        );
        assert!(response.headers().get(CONTENT_LENGTH).is_some());

        let body = response_body(response).await;
        assert_eq!(&body[..4], &crate::config::BINARY_FRAME_MAGIC);

        let entries = logged_entries(&logger);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].path, "/image.bin");
        assert_eq!(entries[0].status, StatusCode::OK);
        assert_eq!(entries[0].outcome, LogOutcome::Success);

        cleanup_dir(&dir);
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

        let root_text = String::from_utf8(response_body(root).await.to_vec()).expect("utf8");
        let image_text = String::from_utf8(response_body(image).await.to_vec()).expect("utf8");
        assert!(root_text.contains("image.png is not configured"));
        assert_eq!(root_text, image_text);

        let entries = logged_entries(&logger);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].outcome, LogOutcome::InputMissing);
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
        let text = String::from_utf8(response_body(response).await.to_vec()).expect("utf8");
        assert!(text.contains("image.png is not configured"));

        let entries = logged_entries(&logger);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].path, "/image.bin");
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
        assert_eq!(entries[0].outcome, LogOutcome::TransformFailed);
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
        let text = String::from_utf8(response_body(response).await.to_vec()).expect("utf8");
        assert!(text.contains("failed to decode image.png"));

        let entries = logged_entries(&logger);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].path, "/image.bin");
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
            .oneshot(
                Request::builder()
                    .uri("/")
                    .body(Body::empty())
                    .expect("request"),
            )
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
            .oneshot(
                Request::builder()
                    .uri("/")
                    .body(Body::empty())
                    .expect("request"),
            )
            .await
            .expect("second response");
        let second_body = response_body(second).await;

        assert_ne!(first_body, second_body);
        let entries = logged_entries(&logger);
        assert_eq!(entries.len(), 2);
        assert_ne!(entries[0].request_id, entries[1].request_id);

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
        let text = String::from_utf8(response_body(response).await.to_vec()).expect("utf8");
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

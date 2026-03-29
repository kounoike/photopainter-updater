use std::env;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};

use axum::Router;
use axum::body::Body;
use axum::extract::State;
use axum::http::header::{CONTENT_TYPE, HeaderValue};
use axum::http::{Response, StatusCode};
use axum::routing::get;

#[derive(Clone, Debug)]
struct AppState {
    image_path: PathBuf,
}

fn default_content_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("contents")
}

fn read_content_dir() -> PathBuf {
    env::var_os("CONTENT_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(default_content_dir)
}

fn image_path_from_dir(content_dir: &Path) -> PathBuf {
    content_dir.join("image.bmp")
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
    match tokio::fs::read(&state.image_path).await {
        Ok(bytes) => bmp_response(bytes),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => text_response(
            StatusCode::NOT_FOUND,
            format!(
                "image.bmp is not configured. Place image.bmp in {} and retry.\n",
                state
                    .image_path
                    .parent()
                    .unwrap_or_else(|| Path::new("."))
                    .display()
            ),
        ),
        Err(_) => text_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to read image.bmp\n",
        ),
    }
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
            "Serving image.bmp from {}",
            image_path_from_dir(content_dir).display()
        ),
        format!("Local: http://127.0.0.1:{port}/ and http://127.0.0.1:{port}/image.bmp"),
        format!("LAN:   use this host's IP address with port {port} from other devices"),
        "Stop: Ctrl+C".to_string(),
    ]
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = read_port().map_err(std::io::Error::other)?;
    let address = SocketAddr::from(([0, 0, 0, 0], port));
    let content_dir = read_content_dir();
    let state = AppState {
        image_path: image_path_from_dir(&content_dir),
    };
    let listener = tokio::net::TcpListener::bind(address).await?;

    for line in startup_messages(&content_dir, port) {
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
    use std::ffi::OsString;
    use tower::ServiceExt;

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

    struct EnvGuard {
        key: &'static str,
        previous: Option<OsString>,
    }

    impl EnvGuard {
        fn set(key: &'static str, value: &Path) -> Self {
            let previous = env::var_os(key);
            // SAFETY: Tests set and restore process env in a scoped manner.
            unsafe {
                env::set_var(key, value);
            }
            Self { key, previous }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            match &self.previous {
                Some(value) => {
                    // SAFETY: Tests restore the original process env value for this key.
                    unsafe {
                        env::set_var(self.key, value);
                    }
                }
                None => {
                    // SAFETY: Tests restore the original missing state for this key.
                    unsafe {
                        env::remove_var(self.key);
                    }
                }
            }
        }
    }

    #[tokio::test]
    async fn root_and_image_bmp_return_same_bytes() {
        let path = temp_path("image");
        let expected = b"BMtest-bmp".to_vec();
        tokio::fs::write(&path, &expected)
            .await
            .expect("write fixture");

        let router = build_app(AppState {
            image_path: path.clone(),
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

        let root_body = to_bytes(root.into_body(), usize::MAX)
            .await
            .expect("read root body");
        let image_body = to_bytes(image.into_body(), usize::MAX)
            .await
            .expect("read image body");

        assert_eq!(root_body, Bytes::from(expected.clone()));
        assert_eq!(image_body, Bytes::from(expected));

        let _ = tokio::fs::remove_file(path).await;
    }

    #[tokio::test]
    async fn missing_image_returns_not_found_for_both_routes() {
        let router = build_app(AppState {
            image_path: temp_path("missing"),
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
        assert_eq!(
            image.headers().get(CONTENT_TYPE),
            Some(&HeaderValue::from_static("text/plain; charset=utf-8"))
        );

        let root_body = to_bytes(root.into_body(), usize::MAX)
            .await
            .expect("read root body");
        assert!(
            String::from_utf8(root_body.to_vec())
                .expect("utf8 body")
                .contains("image.bmp is not configured")
        );
    }

    #[test]
    fn content_dir_env_override_is_used() {
        let override_dir = temp_path("contents-dir");
        std::fs::create_dir_all(&override_dir).expect("create dir");
        let _guard = EnvGuard::set("CONTENT_DIR", &override_dir);

        assert_eq!(read_content_dir(), override_dir);
        let _ = std::fs::remove_dir_all(read_content_dir());
    }

    #[test]
    fn startup_messages_include_local_and_lan_guidance() {
        let messages = startup_messages(Path::new("/tmp/example"), 8000);

        assert!(messages.iter().any(|line| line.contains("127.0.0.1:8000")));
        assert!(
            messages
                .iter()
                .any(|line| line.contains("host's IP address"))
        );
        assert!(
            messages
                .iter()
                .any(|line| line.contains("/tmp/example/image.bmp"))
        );
    }
}

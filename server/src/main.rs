use std::env;
use std::net::SocketAddr;
use std::path::PathBuf;

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

fn repo_image_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("contents")
        .join("image.bmp")
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
            "image.bmp is not configured. Place server/contents/image.bmp and retry.\n",
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

fn text_response(status: StatusCode, body: &'static str) -> Response<Body> {
    let mut response = Response::new(Body::from(body));
    *response.status_mut() = status;
    response.headers_mut().insert(
        CONTENT_TYPE,
        HeaderValue::from_static("text/plain; charset=utf-8"),
    );
    response
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = read_port().map_err(std::io::Error::other)?;
    let address = SocketAddr::from(([0, 0, 0, 0], port));
    let state = AppState {
        image_path: repo_image_path(),
    };
    let listener = tokio::net::TcpListener::bind(address).await?;

    println!(
        "Serving {} at http://127.0.0.1:{}/",
        state.image_path.display(),
        port
    );
    println!("Also available at http://127.0.0.1:{}/image.bmp", port);
    println!("Stop: Ctrl+C");

    axum::serve(listener, build_app(state)).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::{Bytes, to_bytes};
    use axum::http::Request;
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
    }
}

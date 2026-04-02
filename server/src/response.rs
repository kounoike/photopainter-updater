use axum::body::Body;
use axum::http::header::{CONTENT_LENGTH, CONTENT_TYPE, HeaderValue};
use axum::http::{Response, StatusCode};

use crate::config::{BINARY_CONTENT_TYPE, INPUT_IMAGE_NAME, OUTPUT_IMAGE_NAME};
use crate::image_pipeline::{
    ImageLoadError, ResponseFormat, ResponsePayload, TransformError, UploadError, UploadSuccess,
};

pub fn response_from_payload(payload: ResponsePayload) -> Response<Body> {
    match payload {
        ResponsePayload::Bmp(bytes) => bmp_response(bytes),
        ResponsePayload::Binary(bytes) => binary_frame_response(bytes),
    }
}

pub fn image_load_error_response(error: &ImageLoadError) -> Response<Body> {
    match error {
        ImageLoadError::Missing(path) => text_response(
            StatusCode::NOT_FOUND,
            format!(
                "{INPUT_IMAGE_NAME} is not configured. Place {INPUT_IMAGE_NAME} in {} and retry.\n",
                path.parent()
                    .unwrap_or_else(|| std::path::Path::new("."))
                    .display()
            ),
        ),
        ImageLoadError::Decode(path, err) => text_response(
            StatusCode::UNPROCESSABLE_ENTITY,
            format!(
                "failed to decode {INPUT_IMAGE_NAME} at {}: {err}\n",
                path.display()
            ),
        ),
        ImageLoadError::Io(path, err) => text_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!(
                "failed to read {INPUT_IMAGE_NAME} at {}: {err}\n",
                path.display()
            ),
        ),
    }
}

pub fn transform_error_response(
    error: &TransformError,
    response_format: ResponseFormat,
) -> Response<Body> {
    match error {
        TransformError::Encode(err) => {
            let output_name = match response_format {
                ResponseFormat::Bmp => OUTPUT_IMAGE_NAME,
                ResponseFormat::Binary => crate::config::BINARY_OUTPUT_NAME,
            };
            text_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to encode {output_name}: {err}\n"),
            )
        }
    }
}

pub fn upload_success_response(success: &UploadSuccess) -> Response<Body> {
    let normalization = if success.normalized {
        "normalized"
    } else {
        "stored"
    };
    text_response(
        StatusCode::OK,
        format!(
            "updated image.png from {} input; {normalization} to {}x{}\n",
            success.source_format, success.width, success.height
        ),
    )
}

pub fn upload_error_response(error: &UploadError) -> Response<Body> {
    match error {
        UploadError::EmptyBody => text_response(
            StatusCode::BAD_REQUEST,
            "request body is empty; send one image file\n",
        ),
        UploadError::InvalidMultipart(message) => {
            text_response(StatusCode::BAD_REQUEST, format!("{message}\n"))
        }
        UploadError::UnsupportedMediaType(message) => {
            text_response(StatusCode::UNSUPPORTED_MEDIA_TYPE, format!("{message}\n"))
        }
        UploadError::Decode(err) => text_response(
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
            format!("failed to decode supported upload image: {err}\n"),
        ),
        UploadError::Save(path, err) => text_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("failed to save image.png at {}: {err}\n", path.display()),
        ),
    }
}

pub fn text_response(status: StatusCode, body: impl Into<Body>) -> Response<Body> {
    let mut response = Response::new(body.into());
    *response.status_mut() = status;
    response.headers_mut().insert(
        CONTENT_TYPE,
        HeaderValue::from_static("text/plain; charset=utf-8"),
    );
    response
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

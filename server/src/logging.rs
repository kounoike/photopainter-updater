use std::fmt;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::Once;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use axum::extract::ConnectInfo;
use axum::http::{Method, Request, StatusCode};

static TRACING_INIT: Once = Once::new();

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccessLogEvent {
    pub request_id: u64,
    pub timestamp: String,
    pub remote: String,
    pub method: Method,
    pub path: String,
    pub status: StatusCode,
    pub outcome: LogOutcome,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LogOutcome {
    Success,
    InputMissing,
    TransformFailed,
    UploadSuccess,
    UploadInvalid,
    UploadSaveFailed,
    NotFound,
    InternalError,
}

impl fmt::Display for LogOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Success => write!(f, "success"),
            Self::InputMissing => write!(f, "input-missing"),
            Self::TransformFailed => write!(f, "transform-failed"),
            Self::UploadSuccess => write!(f, "upload-success"),
            Self::UploadInvalid => write!(f, "upload-invalid"),
            Self::UploadSaveFailed => write!(f, "upload-save-failed"),
            Self::NotFound => write!(f, "not-found"),
            Self::InternalError => write!(f, "internal-error"),
        }
    }
}

pub trait AccessLogger: Send + Sync {
    fn record(&self, entry: &AccessLogEvent);
}

#[derive(Debug)]
pub struct StdoutAccessLogger;

impl AccessLogger for StdoutAccessLogger {
    fn record(&self, entry: &AccessLogEvent) {
        tracing::info!(
            request_id = entry.request_id,
            timestamp = %entry.timestamp,
            remote = %entry.remote,
            method = %entry.method,
            path = %entry.path,
            status = entry.status.as_u16(),
            outcome = %entry.outcome,
            "http_request"
        );
    }
}

pub fn init_tracing() {
    TRACING_INIT.call_once(|| {
        let _ = tracing_subscriber::fmt()
            .with_target(false)
            .with_ansi(false)
            .without_time()
            .try_init();
    });
}

pub fn log_startup_messages(lines: &[String]) {
    for line in lines {
        tracing::info!("{line}");
    }
}

pub fn log_request(
    logger: &Arc<dyn AccessLogger>,
    request_counter: &AtomicU64,
    method: Method,
    path: String,
    remote: String,
    status: StatusCode,
    outcome: LogOutcome,
) {
    let entry = AccessLogEvent {
        request_id: request_counter.fetch_add(1, Ordering::Relaxed) + 1,
        timestamp: current_timestamp(),
        remote,
        method,
        path,
        status,
        outcome,
    };
    logger.record(&entry);
}

pub fn extract_remote_addr(request: &Request<axum::body::Body>) -> String {
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

#[cfg(test)]
pub fn format_access_log_line(entry: &AccessLogEvent) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn access_log_format_is_single_line_and_contains_required_fields() {
        let entry = AccessLogEvent {
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
}

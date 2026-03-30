use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;

use crate::config::{BINARY_OUTPUT_NAME, OUTPUT_IMAGE_NAME, ServerConfig};
use crate::logging::{AccessLogger, StdoutAccessLogger, init_tracing, log_startup_messages};
use crate::routes::build_app;

#[derive(Clone)]
pub struct AppState {
    pub content_dir: std::path::PathBuf,
    pub logger: Arc<dyn AccessLogger>,
    pub request_counter: Arc<AtomicU64>,
    pub dither_options: crate::config::DitherOptions,
}

impl AppState {
    pub fn from_config(config: &ServerConfig, logger: Arc<dyn AccessLogger>) -> Self {
        Self {
            content_dir: config.content_dir.clone(),
            logger,
            request_counter: Arc::new(AtomicU64::new(0)),
            dither_options: config.dither_options,
        }
    }
}

pub fn startup_messages(config: &ServerConfig) -> Vec<String> {
    let color_distance = if config.dither_options.use_lab {
        "CIE Lab"
    } else {
        "RGB"
    };
    let algorithm = if config.dither_options.use_atkinson {
        "Atkinson"
    } else {
        "Floyd-Steinberg"
    };
    let zigzag = if config.dither_options.use_zigzag {
        "on"
    } else {
        "off"
    };
    let port = config.port;

    vec![
        format!(
            "Serving transformed {OUTPUT_IMAGE_NAME} from {}",
            crate::config::input_image_path_from_dir(&config.content_dir).display()
        ),
        format!("Listen: http://0.0.0.0:{port}/ and http://0.0.0.0:{port}/image.bmp"),
        format!("Local:  http://127.0.0.1:{port}/ and http://127.0.0.1:{port}/image.bmp"),
        format!("LAN:    use this host's IP address with port {port} from other devices"),
        format!("Binary: http://127.0.0.1:{port}/{BINARY_OUTPUT_NAME} for firmware clients"),
        format!(
            "Dither: {algorithm}, color={color_distance}, rate={:.2}, zigzag={zigzag}",
            config.dither_options.diffusion_rate
        ),
        "Access logs: startup and each request are written through tracing".to_string(),
        "Stop: Ctrl+C".to_string(),
    ]
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();
    let config = crate::config::ServerConfig::from_env().map_err(std::io::Error::other)?;
    let address = SocketAddr::from(([0, 0, 0, 0], config.port));
    let listener = tokio::net::TcpListener::bind(address).await?;
    let state = AppState::from_config(&config, Arc::new(StdoutAccessLogger));

    log_startup_messages(&startup_messages(&config));

    axum::serve(
        listener,
        build_app(state).into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}

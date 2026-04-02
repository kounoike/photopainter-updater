use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;

use crate::config::{BINARY_OUTPUT_NAME, OUTPUT_IMAGE_NAME, ServerConfig, UPLOAD_ROUTE_PATH};
use crate::logging::{AccessLogger, StdoutAccessLogger, init_tracing, log_startup_messages};
use crate::routes::build_app;

#[derive(Clone)]
pub struct AppState {
    pub content_dir: std::path::PathBuf,
    pub logger: Arc<dyn AccessLogger>,
    pub request_counter: Arc<AtomicU64>,
    pub render_options: crate::config::RenderOptions,
}

impl AppState {
    pub fn from_config(config: &ServerConfig, logger: Arc<dyn AccessLogger>) -> Self {
        Self {
            content_dir: config.content_dir.clone(),
            logger,
            request_counter: Arc::new(AtomicU64::new(0)),
            render_options: config.render_options,
        }
    }
}

pub fn startup_messages(config: &ServerConfig) -> Vec<String> {
    let color_distance = if config.render_options.dither_options.use_lab {
        "CIE Lab"
    } else {
        "RGB"
    };
    let algorithm = if config.render_options.dither_options.use_atkinson {
        "Atkinson"
    } else {
        "Floyd-Steinberg"
    };
    let zigzag = if config.render_options.dither_options.use_zigzag {
        "on"
    } else {
        "off"
    };
    let saturation = match config.render_options.dither_options.saturation_mode {
        crate::config::SaturationMode::Boosted => "boosted",
        crate::config::SaturationMode::Neutral => "neutral",
    };
    let compare = if let Some(compare_profile) = config.render_options.compare.profile {
        format!(
            "{} ({}) split={}",
            compare_profile.key(),
            compare_profile.label(),
            config.render_options.compare.split.key()
        )
    } else {
        "off".to_string()
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
        format!("Upload: POST http://127.0.0.1:{port}{UPLOAD_ROUTE_PATH}"),
        format!(
            "Profile: {} ({})",
            config.render_options.profile.key(),
            config.render_options.profile.label()
        ),
        format!(
            "Dither: {algorithm}, color={color_distance}, rate={:.2}, zigzag={zigzag}, saturation={saturation}",
            config.render_options.dither_options.diffusion_rate
        ),
        format!("Compare: {compare}"),
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

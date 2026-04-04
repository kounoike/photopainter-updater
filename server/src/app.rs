use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;

use crate::config::{
    BINARY_OUTPUT_NAME, HealthListenerMode, OUTPUT_IMAGE_NAME, ServerConfig, UPLOAD_ROUTE_PATH,
};
use crate::logging::{AccessLogger, StdoutAccessLogger, init_tracing, log_startup_messages};
use crate::routes::{build_app, build_health_app};

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
    let health_listener = match config.health_listener_mode() {
        HealthListenerMode::Disabled => "Health: disabled (use main /ping)".to_string(),
        HealthListenerMode::SharedWithMain => {
            format!("Health: shared with main listener on http://127.0.0.1:{port}/ping")
        }
        HealthListenerMode::Dedicated(health_port) => {
            format!("Health: http://127.0.0.1:{health_port}/ping")
        }
    };

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
        health_listener,
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
    let state = AppState::from_config(&config, Arc::new(StdoutAccessLogger));

    log_startup_messages(&startup_messages(&config));

    let address = SocketAddr::from(([0, 0, 0, 0], config.port));
    let listener = tokio::net::TcpListener::bind(address).await?;

    match config.health_listener_mode() {
        HealthListenerMode::Disabled | HealthListenerMode::SharedWithMain => {
            axum::serve(
                listener,
                build_app(state).into_make_service_with_connect_info::<SocketAddr>(),
            )
            .await?;
        }
        HealthListenerMode::Dedicated(health_port) => {
            let health_address = SocketAddr::from(([0, 0, 0, 0], health_port));
            let health_listener = tokio::net::TcpListener::bind(health_address).await?;
            let main_state = state.clone();
            let health_state = state;
            tokio::try_join!(
                async {
                    axum::serve(
                        listener,
                        build_app(main_state).into_make_service_with_connect_info::<SocketAddr>(),
                    )
                    .await
                },
                async {
                    axum::serve(
                        health_listener,
                        build_health_app(health_state)
                            .into_make_service_with_connect_info::<SocketAddr>(),
                    )
                    .await
                }
            )?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{CompareOptions, DitherOptions, ImageProfile, RenderOptions, SaturationMode};
    use std::path::PathBuf;

    fn test_config(port: u16, health_port: Option<u16>) -> ServerConfig {
        ServerConfig {
            port,
            health_port,
            content_dir: PathBuf::from("/tmp/content"),
            render_options: RenderOptions {
                profile: ImageProfile::Baseline,
                dither_options: DitherOptions {
                    use_lab: false,
                    use_atkinson: false,
                    use_zigzag: false,
                    diffusion_rate: 1.0,
                    saturation_mode: SaturationMode::Boosted,
                    neutral_bias: 0.0,
                    chroma_bias: 0.0,
                    hue_guard: 0.0,
                },
                compare: CompareOptions {
                    profile: None,
                    split: crate::config::CompareSplit::Vertical,
                },
            },
        }
    }

    #[test]
    fn startup_messages_show_dedicated_health_port() {
        let messages = startup_messages(&test_config(8000, Some(18000)));

        assert!(messages.iter().any(|line| line.contains("Health: http://127.0.0.1:18000/ping")));
    }

    #[test]
    fn startup_messages_show_shared_health_port() {
        let messages = startup_messages(&test_config(8000, Some(8000)));

        assert!(messages.iter().any(|line| line.contains("shared with main listener")));
    }
}

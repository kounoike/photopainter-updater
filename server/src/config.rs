use std::path::{Path, PathBuf};

use envconfig::Envconfig;

pub const INPUT_IMAGE_NAME: &str = "image.png";
pub const OUTPUT_IMAGE_NAME: &str = "image.bmp";
pub const BINARY_OUTPUT_NAME: &str = "image.bin";
pub const BINARY_CONTENT_TYPE: &str = "application/vnd.photopainter-frame";
pub const BINARY_FRAME_MAGIC: [u8; 4] = *b"PPBF";
pub const BINARY_FRAME_VERSION: u8 = 1;
pub const BINARY_FRAME_FLAGS: u8 = 0;
pub const BINARY_HEADER_LENGTH: u16 = 20;
pub const EPD_DISPLAY_WIDTH: usize = 800;
pub const EPD_DISPLAY_HEIGHT: usize = 480;
pub const SATURATION_SCALE: f32 = 1.52;
pub const SATURATION_BIAS: f32 = 0.29;
pub const VALUE_SCALE: f32 = 1.02;
pub const VALUE_SATURATION_SCALE: f32 = 0.14;
pub const VALUE_BIAS: f32 = 0.15;
#[cfg(test)]
pub const SATURATION_TOLERANCE: u8 = 6;
pub const REFERENCE_PALETTE: [[u8; 3]; 7] = [
    [0, 0, 0],
    [255, 255, 255],
    [255, 255, 0],
    [255, 0, 0],
    [0, 0, 0],
    [0, 0, 255],
    [0, 255, 0],
];

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DitherOptions {
    pub use_lab: bool,
    pub use_atkinson: bool,
    pub use_zigzag: bool,
    pub diffusion_rate: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ServerConfig {
    pub port: u16,
    pub content_dir: PathBuf,
    pub dither_options: DitherOptions,
}

#[derive(Envconfig)]
struct RawServerConfig {
    #[envconfig(from = "PORT", default = "8000")]
    port: String,
    #[envconfig(from = "CONTENT_DIR")]
    content_dir: Option<String>,
    #[envconfig(from = "DITHER_USE_LAB", default = "0")]
    use_lab: String,
    #[envconfig(from = "DITHER_USE_ATKINSON", default = "0")]
    use_atkinson: String,
    #[envconfig(from = "DITHER_DIFFUSION_RATE", default = "1.0")]
    diffusion_rate: String,
    #[envconfig(from = "DITHER_ZIGZAG", default = "0")]
    use_zigzag: String,
}

impl ServerConfig {
    pub fn from_env() -> Result<Self, String> {
        let raw = RawServerConfig::init_from_env()
            .map_err(|err| format!("環境変数の読込に失敗しました: {err}"))?;
        let port = raw
            .port
            .parse::<u16>()
            .map_err(|_| "PORT は 0-65535 の数値で指定してください".to_string())?;
        let content_dir = raw
            .content_dir
            .map(PathBuf::from)
            .unwrap_or_else(default_content_dir);
        let dither_options = DitherOptions {
            use_lab: parse_bool_flag("DITHER_USE_LAB", &raw.use_lab)?,
            use_atkinson: parse_bool_flag("DITHER_USE_ATKINSON", &raw.use_atkinson)?,
            diffusion_rate: parse_diffusion_rate(&raw.diffusion_rate)?,
            use_zigzag: parse_bool_flag("DITHER_ZIGZAG", &raw.use_zigzag)?,
        };

        Ok(Self {
            port,
            content_dir,
            dither_options,
        })
    }
}

pub fn default_content_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("contents")
}

pub fn input_image_path_from_dir(content_dir: &Path) -> PathBuf {
    content_dir.join(INPUT_IMAGE_NAME)
}

fn parse_diffusion_rate(raw: &str) -> Result<f32, String> {
    raw.parse::<f32>()
        .map(|value| value.clamp(0.0, 1.0))
        .map_err(|_| "DITHER_DIFFUSION_RATE は数値で指定してください".to_string())
}

fn parse_bool_flag(name: &str, raw: &str) -> Result<bool, String> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" => Ok(true),
        "0" | "false" | "no" | "off" | "" => Ok(false),
        _ => Err(format!(
            "{name} は 0/1 または true/false で指定してください"
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsString;
    use std::sync::{Mutex, OnceLock};

    fn env_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    struct EnvGuard {
        key: &'static str,
        previous: Option<OsString>,
    }

    impl EnvGuard {
        fn set(key: &'static str, value: &str) -> Self {
            let previous = std::env::var_os(key);
            unsafe {
                std::env::set_var(key, value);
            }
            Self { key, previous }
        }

        fn unset(key: &'static str) -> Self {
            let previous = std::env::var_os(key);
            unsafe {
                std::env::remove_var(key);
            }
            Self { key, previous }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            match &self.previous {
                Some(value) => unsafe {
                    std::env::set_var(self.key, value);
                },
                None => unsafe {
                    std::env::remove_var(self.key);
                },
            }
        }
    }

    #[test]
    fn config_uses_defaults_when_env_is_missing() {
        let _lock = env_lock().lock().expect("env lock");
        let _port = EnvGuard::unset("PORT");
        let _content = EnvGuard::unset("CONTENT_DIR");
        let _lab = EnvGuard::unset("DITHER_USE_LAB");
        let _atk = EnvGuard::unset("DITHER_USE_ATKINSON");
        let _rate = EnvGuard::unset("DITHER_DIFFUSION_RATE");
        let _zigzag = EnvGuard::unset("DITHER_ZIGZAG");

        let config = ServerConfig::from_env().expect("default config");

        assert_eq!(config.port, 8000);
        assert_eq!(config.content_dir, default_content_dir());
        assert_eq!(
            config.dither_options,
            DitherOptions {
                use_lab: false,
                use_atkinson: false,
                use_zigzag: false,
                diffusion_rate: 1.0,
            }
        );
    }

    #[test]
    fn config_uses_env_overrides() {
        let _lock = env_lock().lock().expect("env lock");
        let _port = EnvGuard::set("PORT", "8100");
        let _content = EnvGuard::set("CONTENT_DIR", "/tmp/override");
        let _lab = EnvGuard::set("DITHER_USE_LAB", "1");
        let _atk = EnvGuard::set("DITHER_USE_ATKINSON", "true");
        let _rate = EnvGuard::set("DITHER_DIFFUSION_RATE", "1.4");
        let _zigzag = EnvGuard::set("DITHER_ZIGZAG", "on");

        let config = ServerConfig::from_env().expect("env config");

        assert_eq!(config.port, 8100);
        assert_eq!(config.content_dir, PathBuf::from("/tmp/override"));
        assert!(config.dither_options.use_lab);
        assert!(config.dither_options.use_atkinson);
        assert!(config.dither_options.use_zigzag);
        assert_eq!(config.dither_options.diffusion_rate, 1.0);
    }

    #[test]
    fn invalid_port_is_rejected() {
        let _lock = env_lock().lock().expect("env lock");
        let _port = EnvGuard::set("PORT", "abc");

        let err = ServerConfig::from_env().expect_err("invalid port");

        assert!(err.contains("PORT"));
    }

    #[test]
    fn invalid_bool_flag_is_rejected() {
        let _lock = env_lock().lock().expect("env lock");
        let _lab = EnvGuard::set("DITHER_USE_LAB", "maybe");

        let err = ServerConfig::from_env().expect_err("invalid bool");

        assert!(err.contains("DITHER_USE_LAB"));
    }
}

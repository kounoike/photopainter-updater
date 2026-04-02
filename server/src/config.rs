use std::path::{Path, PathBuf};

use envconfig::Envconfig;

pub const INPUT_IMAGE_NAME: &str = "image.png";
pub const OUTPUT_IMAGE_NAME: &str = "image.bmp";
pub const BINARY_OUTPUT_NAME: &str = "image.bin";
pub const UPLOAD_ROUTE_PATH: &str = "/upload";
pub const BINARY_CONTENT_TYPE: &str = "application/vnd.photopainter-frame";
pub const BINARY_FRAME_MAGIC: [u8; 4] = *b"PPBF";
pub const BINARY_FRAME_VERSION: u8 = 1;
pub const BINARY_FRAME_FLAGS: u8 = 0;
pub const BINARY_HEADER_LENGTH: u16 = 20;
pub const EPD_DISPLAY_WIDTH: usize = 800;
pub const EPD_DISPLAY_HEIGHT: usize = 480;
pub const UPLOAD_IMAGE_WIDTH: u32 = EPD_DISPLAY_HEIGHT as u32;
pub const UPLOAD_IMAGE_HEIGHT: u32 = EPD_DISPLAY_WIDTH as u32;
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SaturationMode {
    Boosted,
    Neutral,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DitherOptions {
    pub use_lab: bool,
    pub use_atkinson: bool,
    pub use_zigzag: bool,
    pub diffusion_rate: f32,
    pub saturation_mode: SaturationMode,
    pub neutral_bias: f32,
    pub chroma_bias: f32,
    pub hue_guard: f32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ImageProfile {
    Baseline,
    NoSaturationBoost,
    ColorPriority,
    HueGuard,
    ColorPriorityHueGuard,
}

impl ImageProfile {
    pub fn key(self) -> &'static str {
        match self {
            Self::Baseline => "baseline",
            Self::NoSaturationBoost => "no-sat-boost",
            Self::ColorPriority => "color-priority",
            Self::HueGuard => "hue-guard",
            Self::ColorPriorityHueGuard => "color-priority-hue-guard",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Baseline => "Baseline",
            Self::NoSaturationBoost => "No Saturation Boost",
            Self::ColorPriority => "Color Priority",
            Self::HueGuard => "Hue Guard",
            Self::ColorPriorityHueGuard => "Color Priority + Hue Guard",
        }
    }

    pub fn default_dither_options(self) -> DitherOptions {
        match self {
            Self::Baseline => DitherOptions {
                use_lab: false,
                use_atkinson: false,
                use_zigzag: false,
                diffusion_rate: 1.0,
                saturation_mode: SaturationMode::Boosted,
                neutral_bias: 0.0,
                chroma_bias: 0.0,
                hue_guard: 0.0,
            },
            Self::NoSaturationBoost => DitherOptions {
                saturation_mode: SaturationMode::Neutral,
                ..Self::Baseline.default_dither_options()
            },
            Self::ColorPriority => DitherOptions {
                use_lab: true,
                saturation_mode: SaturationMode::Neutral,
                neutral_bias: 1800.0,
                chroma_bias: -250.0,
                ..Self::Baseline.default_dither_options()
            },
            Self::HueGuard => DitherOptions {
                use_lab: true,
                saturation_mode: SaturationMode::Neutral,
                hue_guard: 4500.0,
                ..Self::Baseline.default_dither_options()
            },
            Self::ColorPriorityHueGuard => DitherOptions {
                use_lab: true,
                saturation_mode: SaturationMode::Neutral,
                neutral_bias: 1500.0,
                chroma_bias: -200.0,
                hue_guard: 3500.0,
                ..Self::Baseline.default_dither_options()
            },
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CompareSplit {
    Vertical,
    Horizontal,
}

impl CompareSplit {
    pub fn key(self) -> &'static str {
        match self {
            Self::Vertical => "vertical",
            Self::Horizontal => "horizontal",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CompareOptions {
    pub profile: Option<ImageProfile>,
    pub split: CompareSplit,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RenderOptions {
    pub profile: ImageProfile,
    pub dither_options: DitherOptions,
    pub compare: CompareOptions,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ServerConfig {
    pub port: u16,
    pub content_dir: PathBuf,
    pub render_options: RenderOptions,
}

#[derive(Envconfig)]
struct RawServerConfig {
    #[envconfig(from = "PORT", default = "8000")]
    port: String,
    #[envconfig(from = "CONTENT_DIR")]
    content_dir: Option<String>,
    #[envconfig(from = "IMAGE_PROFILE", default = "baseline")]
    image_profile: String,
    #[envconfig(from = "COMPARE_WITH_BASELINE", default = "0")]
    compare_with_baseline: String,
    #[envconfig(from = "COMPARE_PROFILE")]
    compare_profile: Option<String>,
    #[envconfig(from = "COMPARE_SPLIT", default = "vertical")]
    compare_split: String,
    #[envconfig(from = "DITHER_USE_LAB")]
    use_lab: Option<String>,
    #[envconfig(from = "DITHER_USE_ATKINSON")]
    use_atkinson: Option<String>,
    #[envconfig(from = "DITHER_DIFFUSION_RATE")]
    diffusion_rate: Option<String>,
    #[envconfig(from = "DITHER_ZIGZAG")]
    use_zigzag: Option<String>,
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
        let profile = parse_image_profile(&raw.image_profile)?;
        let mut dither_options = profile.default_dither_options();
        if let Some(raw_use_lab) = raw.use_lab.as_ref() {
            dither_options.use_lab = parse_bool_flag("DITHER_USE_LAB", raw_use_lab)?;
        }
        if let Some(raw_use_atkinson) = raw.use_atkinson.as_ref() {
            dither_options.use_atkinson = parse_bool_flag("DITHER_USE_ATKINSON", raw_use_atkinson)?;
        }
        if let Some(raw_diffusion_rate) = raw.diffusion_rate.as_ref() {
            dither_options.diffusion_rate = parse_diffusion_rate(raw_diffusion_rate)?;
        }
        if let Some(raw_use_zigzag) = raw.use_zigzag.as_ref() {
            dither_options.use_zigzag = parse_bool_flag("DITHER_ZIGZAG", raw_use_zigzag)?;
        }
        let render_options = RenderOptions {
            profile,
            dither_options,
            compare: CompareOptions {
                profile: resolve_compare_profile(
                    profile,
                    raw.compare_profile.as_deref(),
                    parse_bool_flag("COMPARE_WITH_BASELINE", &raw.compare_with_baseline)?,
                )?,
                split: parse_compare_split(&raw.compare_split)?,
            },
        };

        Ok(Self {
            port,
            content_dir,
            render_options,
        })
    }
}

pub fn default_content_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("contents")
}

pub fn input_image_path_from_dir(content_dir: &Path) -> PathBuf {
    content_dir.join(INPUT_IMAGE_NAME)
}

pub fn input_image_temp_path_from_dir(content_dir: &Path) -> PathBuf {
    content_dir.join(format!("{INPUT_IMAGE_NAME}.tmp"))
}

fn parse_diffusion_rate(raw: &str) -> Result<f32, String> {
    raw.parse::<f32>()
        .map(|value| value.clamp(0.0, 1.0))
        .map_err(|_| "DITHER_DIFFUSION_RATE は数値で指定してください".to_string())
}

fn parse_image_profile(raw: &str) -> Result<ImageProfile, String> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "baseline" => Ok(ImageProfile::Baseline),
        "no-sat-boost" => Ok(ImageProfile::NoSaturationBoost),
        "color-priority" => Ok(ImageProfile::ColorPriority),
        "hue-guard" => Ok(ImageProfile::HueGuard),
        "color-priority-hue-guard" => Ok(ImageProfile::ColorPriorityHueGuard),
        _ => Err(format!(
            "IMAGE_PROFILE は baseline / no-sat-boost / color-priority / hue-guard / color-priority-hue-guard のいずれかで指定してください"
        )),
    }
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

fn parse_compare_split(raw: &str) -> Result<CompareSplit, String> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "vertical" | "" => Ok(CompareSplit::Vertical),
        "horizontal" => Ok(CompareSplit::Horizontal),
        _ => Err("COMPARE_SPLIT は vertical または horizontal で指定してください".to_string()),
    }
}

fn resolve_compare_profile(
    active_profile: ImageProfile,
    compare_profile: Option<&str>,
    compare_with_baseline: bool,
) -> Result<Option<ImageProfile>, String> {
    if let Some(raw_compare_profile) = compare_profile {
        let trimmed = raw_compare_profile.trim();
        if trimmed.is_empty() {
            return Ok(None);
        }
        let profile = parse_image_profile(trimmed)?;
        if compare_with_baseline && profile != ImageProfile::Baseline {
            return Err(
                "COMPARE_PROFILE と COMPARE_WITH_BASELINE を併用する場合、COMPARE_PROFILE=baseline のみ指定できます"
                    .to_string(),
            );
        }
        return Ok(Some(profile));
    }

    if compare_with_baseline {
        return Ok(Some(ImageProfile::Baseline));
    }

    let _ = active_profile;
    Ok(None)
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
        let _profile = EnvGuard::unset("IMAGE_PROFILE");
        let _compare = EnvGuard::unset("COMPARE_WITH_BASELINE");
        let _compare_profile = EnvGuard::unset("COMPARE_PROFILE");
        let _split = EnvGuard::unset("COMPARE_SPLIT");
        let _lab = EnvGuard::unset("DITHER_USE_LAB");
        let _atk = EnvGuard::unset("DITHER_USE_ATKINSON");
        let _rate = EnvGuard::unset("DITHER_DIFFUSION_RATE");
        let _zigzag = EnvGuard::unset("DITHER_ZIGZAG");

        let config = ServerConfig::from_env().expect("default config");

        assert_eq!(config.port, 8000);
        assert_eq!(config.content_dir, default_content_dir());
        assert_eq!(config.render_options.profile, ImageProfile::Baseline);
        assert_eq!(
            config.render_options.dither_options,
            ImageProfile::Baseline.default_dither_options()
        );
        assert_eq!(config.render_options.compare.profile, None);
        assert_eq!(config.render_options.compare.split, CompareSplit::Vertical);
    }

    #[test]
    fn config_uses_env_overrides() {
        let _lock = env_lock().lock().expect("env lock");
        let _port = EnvGuard::set("PORT", "8100");
        let _content = EnvGuard::set("CONTENT_DIR", "/tmp/override");
        let _profile = EnvGuard::set("IMAGE_PROFILE", "color-priority");
        let _compare = EnvGuard::set("COMPARE_WITH_BASELINE", "1");
        let _compare_profile = EnvGuard::unset("COMPARE_PROFILE");
        let _split = EnvGuard::set("COMPARE_SPLIT", "horizontal");
        let _lab = EnvGuard::set("DITHER_USE_LAB", "1");
        let _atk = EnvGuard::set("DITHER_USE_ATKINSON", "true");
        let _rate = EnvGuard::set("DITHER_DIFFUSION_RATE", "1.4");
        let _zigzag = EnvGuard::set("DITHER_ZIGZAG", "on");

        let config = ServerConfig::from_env().expect("env config");

        assert_eq!(config.port, 8100);
        assert_eq!(config.content_dir, PathBuf::from("/tmp/override"));
        assert_eq!(config.render_options.profile, ImageProfile::ColorPriority);
        assert_eq!(
            config.render_options.compare.profile,
            Some(ImageProfile::Baseline)
        );
        assert_eq!(
            config.render_options.compare.split,
            CompareSplit::Horizontal
        );
        assert!(config.render_options.dither_options.use_lab);
        assert!(config.render_options.dither_options.use_atkinson);
        assert!(config.render_options.dither_options.use_zigzag);
        assert_eq!(config.render_options.dither_options.diffusion_rate, 1.0);
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

    #[test]
    fn invalid_profile_is_rejected() {
        let _lock = env_lock().lock().expect("env lock");
        let _profile = EnvGuard::set("IMAGE_PROFILE", "unknown");

        let err = ServerConfig::from_env().expect_err("invalid profile");

        assert!(err.contains("IMAGE_PROFILE"));
    }

    #[test]
    fn invalid_compare_split_is_rejected() {
        let _lock = env_lock().lock().expect("env lock");
        let _split = EnvGuard::set("COMPARE_SPLIT", "diagonal");

        let err = ServerConfig::from_env().expect_err("invalid compare split");

        assert!(err.contains("COMPARE_SPLIT"));
    }

    #[test]
    fn compare_profile_overrides_baseline_shorthand() {
        let _lock = env_lock().lock().expect("env lock");
        let _profile = EnvGuard::set("IMAGE_PROFILE", "hue-guard");
        let _compare = EnvGuard::set("COMPARE_WITH_BASELINE", "0");
        let _compare_profile = EnvGuard::set("COMPARE_PROFILE", "color-priority");

        let config = ServerConfig::from_env().expect("compare profile config");

        assert_eq!(config.render_options.profile, ImageProfile::HueGuard);
        assert_eq!(
            config.render_options.compare.profile,
            Some(ImageProfile::ColorPriority)
        );
    }

    #[test]
    fn conflicting_compare_settings_are_rejected() {
        let _lock = env_lock().lock().expect("env lock");
        let _compare = EnvGuard::set("COMPARE_WITH_BASELINE", "1");
        let _compare_profile = EnvGuard::set("COMPARE_PROFILE", "color-priority");

        let err = ServerConfig::from_env().expect_err("conflicting compare settings");

        assert!(err.contains("COMPARE_PROFILE"));
    }
}

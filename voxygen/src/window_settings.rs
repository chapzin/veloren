use serde::{Deserialize, Serialize};

#[derive(Default, Copy, Clone, Hash, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum FullscreenMode {
    Exclusive,
    #[serde(other)]
    #[default]
    Borderless,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct WindowSettings {
    pub size: [u32; 2],
    pub maximised: bool,
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            size: [1280, 720],
            maximised: false,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct FullScreenSettings {
    pub enabled: bool,
    pub mode: FullscreenMode,
    pub resolution: [u16; 2],
    pub bit_depth: Option<u16>,
    pub refresh_rate_millihertz: Option<u32>,
}

impl Default for FullScreenSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            mode: FullscreenMode::Borderless,
            resolution: [1920, 1080],
            bit_depth: None,
            refresh_rate_millihertz: None,
        }
    }
} 
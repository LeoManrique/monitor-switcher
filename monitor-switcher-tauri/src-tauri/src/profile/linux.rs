//! Linux-specific profile storage.
//!
//! Uses a simplified profile format optimized for XRandR.

use crate::display::{DisplaySettings, OutputConfig, Rotation};
use super::storage::get_profile_path;
use serde::{Deserialize, Serialize};
use std::fs;

/// Linux display profile format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinuxDisplayProfile {
    /// Profile format version
    pub version: u32,
    /// Platform identifier
    pub platform: String,
    /// Output configurations
    pub outputs: Vec<LinuxOutputConfig>,
}

/// Serializable output configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinuxOutputConfig {
    pub name: String,
    pub enabled: bool,
    pub primary: bool,
    pub width: u32,
    pub height: u32,
    pub refresh_rate: f32,
    pub pos_x: i32,
    pub pos_y: i32,
    pub rotation: String,
    pub scale: f32,
}

impl From<&OutputConfig> for LinuxOutputConfig {
    fn from(output: &OutputConfig) -> Self {
        Self {
            name: output.name.clone(),
            enabled: output.enabled,
            primary: output.primary,
            width: output.width,
            height: output.height,
            refresh_rate: output.refresh_rate,
            pos_x: output.pos_x,
            pos_y: output.pos_y,
            rotation: output.rotation.to_xrandr_arg().to_string(),
            scale: output.scale,
        }
    }
}

impl From<&LinuxOutputConfig> for OutputConfig {
    fn from(config: &LinuxOutputConfig) -> Self {
        Self {
            name: config.name.clone(),
            enabled: config.enabled,
            primary: config.primary,
            width: config.width,
            height: config.height,
            refresh_rate: config.refresh_rate,
            pos_x: config.pos_x,
            pos_y: config.pos_y,
            rotation: Rotation::from_xrandr(&config.rotation),
            scale: config.scale,
        }
    }
}

/// Save a Linux display profile.
pub fn save_linux_profile(name: &str, settings: &DisplaySettings) -> Result<(), String> {
    let profile = LinuxDisplayProfile {
        version: 1,
        platform: "linux".to_string(),
        outputs: settings.outputs.iter().map(LinuxOutputConfig::from).collect(),
    };

    let path = get_profile_path(name)?;
    let json = serde_json::to_string_pretty(&profile)
        .map_err(|e| format!("Failed to serialize profile: {}", e))?;

    fs::write(&path, json)
        .map_err(|e| format!("Failed to write profile file: {}", e))?;

    Ok(())
}

/// Load a Linux display profile.
pub fn load_linux_profile(name: &str) -> Result<DisplaySettings, String> {
    let path = get_profile_path(name)?;

    let json = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read profile file: {}", e))?;

    let profile: LinuxDisplayProfile = serde_json::from_str(&json)
        .map_err(|e| format!("Failed to parse profile: {}", e))?;

    let outputs = profile.outputs.iter().map(OutputConfig::from).collect();

    Ok(DisplaySettings { outputs })
}

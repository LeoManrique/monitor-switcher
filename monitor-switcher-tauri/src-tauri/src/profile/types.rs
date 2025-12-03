//! Profile JSON types matching the existing C#/Go format (Windows only).
//!
//! These types use PascalCase field names for backward compatibility.

#![cfg(windows)]

use serde::{Deserialize, Serialize};

/// Root object for display profile JSON serialization.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DisplayProfile {
    pub version: i32,
    pub path_info_array: Vec<PathInfo>,
    pub mode_info_array: Vec<ModeInfo>,
    pub additional_info: Vec<ProfileMonitorInfo>,
    /// DPI scaling settings per source. Added in version 2.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dpi_scale_info: Vec<DpiScaleInfo>,
}

impl Default for DisplayProfile {
    fn default() -> Self {
        Self {
            version: 1,
            path_info_array: Vec::new(),
            mode_info_array: Vec::new(),
            additional_info: Vec::new(),
            dpi_scale_info: Vec::new(),
        }
    }
}

/// Display path information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PathInfo {
    pub source_info: PathSourceInfo,
    pub target_info: PathTargetInfo,
    pub flags: u32,
}

/// Source information for a path.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PathSourceInfo {
    pub adapter_id: AdapterId,
    pub id: u32,
    pub mode_info_idx: u32,
    pub status_flags: u32,
}

/// Target information for a path.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PathTargetInfo {
    pub adapter_id: AdapterId,
    pub id: u32,
    pub mode_info_idx: u32,
    pub output_technology: u32,
    pub rotation: u32,
    pub scaling: u32,
    pub refresh_rate: Rational,
    pub scan_line_ordering: u32,
    pub target_available: bool,
    pub status_flags: u32,
}

/// Adapter identifier.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AdapterId {
    pub low_part: u32,
    pub high_part: u32,
}

/// Rational number representation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Rational {
    pub numerator: u32,
    pub denominator: u32,
}

/// Mode information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ModeInfo {
    pub info_type: u32,
    pub id: u32,
    pub adapter_id: AdapterId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_mode: Option<TargetMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_mode: Option<SourceMode>,
}

/// Target mode information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TargetMode {
    pub target_video_signal_info: VideoSignalInfo,
}

/// Video signal timing information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VideoSignalInfo {
    pub pixel_rate: i64,
    #[serde(rename = "HSyncFreq")]
    pub h_sync_freq: Rational,
    #[serde(rename = "VSyncFreq")]
    pub v_sync_freq: Rational,
    pub active_size: Region2D,
    pub total_size: Region2D,
    pub video_standard: u32,
    pub scan_line_ordering: u32,
}

/// 2D region size.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Region2D {
    pub cx: u32,
    pub cy: u32,
}

/// Source mode information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SourceMode {
    pub width: u32,
    pub height: u32,
    pub pixel_format: u32,
    pub position: Point,
}

/// 2D point.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

/// Additional monitor metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ProfileMonitorInfo {
    pub manufacture_id: u16,
    pub product_code_id: u16,
    pub valid: bool,
    #[serde(default, deserialize_with = "deserialize_null_string")]
    pub monitor_device_path: String,
    #[serde(default, deserialize_with = "deserialize_null_string")]
    pub monitor_friendly_device: String,
}

/// DPI scaling information for a display source.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DpiScaleInfo {
    /// Source ID this DPI setting applies to.
    pub source_id: u32,
    /// DPI scaling percentage (100, 125, 150, etc.).
    pub dpi_scale: u32,
}

/// Deserialize null as empty string
fn deserialize_null_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

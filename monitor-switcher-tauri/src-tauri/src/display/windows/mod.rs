//! Windows display management using CCD (Connecting and Configuring Displays) API.
//!
//! This module is ONLY compiled on Windows.
//! For Linux implementation, see `../linux/`.
//!
//! ## Module Structure
//!
//! - `api.rs` - Raw Windows CCD API calls
//! - `types.rs` - Windows-specific type definitions (LUID, DisplayConfig*, etc.)
//! - `matcher.rs` - Adapter ID matching logic for profile restoration

mod api;
mod matcher;
mod types;

// Re-export public API
pub use api::{
    get_display_settings, set_display_settings,
    get_monitor_additional_info, turn_off_monitors,
    get_dpi_scaling_info, set_dpi_scaling,
    DisplaySettings, MonitorAdditionalInfo,
};

pub use matcher::{match_adapter_ids, get_additional_info_for_modes};

pub use types::{
    LUID, DisplayConfigPathInfo, DisplayConfigModeInfo,
    DisplayConfigTargetMode, DisplayConfigSourceMode,
    DisplayConfigRational, DisplayConfig2DRegion, PointL,
    DisplayConfigPathSourceInfo, DisplayConfigPathTargetInfo,
    DisplayConfigVideoSignalInfo, DpiScalingInfo,
    MODE_INFO_TYPE_SOURCE, MODE_INFO_TYPE_TARGET,
};

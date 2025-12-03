//! Linux display type definitions.

use serde::{Deserialize, Serialize};

// ============================================================================
// Linux-Native Types
// ============================================================================

/// Output configuration for a single display.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    /// Output name (e.g., "HDMI-1", "DP-1", "eDP-1")
    pub name: String,
    /// Whether the output is enabled
    pub enabled: bool,
    /// Whether this is the primary display
    pub primary: bool,
    /// Resolution width in pixels
    pub width: u32,
    /// Resolution height in pixels
    pub height: u32,
    /// Refresh rate in Hz (e.g., 60.0, 144.0)
    pub refresh_rate: f32,
    /// X position in the virtual screen
    pub pos_x: i32,
    /// Y position in the virtual screen
    pub pos_y: i32,
    /// Rotation (normal, left, right, inverted)
    pub rotation: Rotation,
    /// Scale factor (1.0 = 100%, 2.0 = 200%)
    pub scale: f32,
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            name: String::new(),
            enabled: false,
            primary: false,
            width: 0,
            height: 0,
            refresh_rate: 60.0,
            pos_x: 0,
            pos_y: 0,
            rotation: Rotation::Normal,
            scale: 1.0,
        }
    }
}

/// Display rotation options.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Rotation {
    #[default]
    Normal,
    Left,
    Right,
    Inverted,
}

impl Rotation {
    /// Convert to xrandr rotation argument.
    pub fn to_xrandr_arg(&self) -> &'static str {
        match self {
            Rotation::Normal => "normal",
            Rotation::Left => "left",
            Rotation::Right => "right",
            Rotation::Inverted => "inverted",
        }
    }

    /// Parse from xrandr output.
    pub fn from_xrandr(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "left" => Rotation::Left,
            "right" => Rotation::Right,
            "inverted" => Rotation::Inverted,
            _ => Rotation::Normal,
        }
    }

    /// Convert to u32 value matching Windows DISPLAYCONFIG_ROTATION values.
    /// This is used for the frontend MonitorDetails struct.
    /// 1 = Identity (0°), 2 = Rotate90 (90° CW / 270° CCW),
    /// 3 = Rotate180 (180°), 4 = Rotate270 (270° CW / 90° CCW)
    pub fn to_u32(&self) -> u32 {
        match self {
            Rotation::Normal => 1,   // DISPLAYCONFIG_ROTATION_IDENTITY
            Rotation::Right => 2,    // DISPLAYCONFIG_ROTATION_ROTATE90 (90° clockwise)
            Rotation::Inverted => 3, // DISPLAYCONFIG_ROTATION_ROTATE180
            Rotation::Left => 4,     // DISPLAYCONFIG_ROTATION_ROTATE270 (90° counter-clockwise)
        }
    }
}


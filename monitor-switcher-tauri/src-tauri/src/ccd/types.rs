//! CCD API type definitions for Windows display configuration.
//!
//! These types must match the exact memory layout expected by Windows API.

/// Locally Unique Identifier for display adapters.
/// Note: Adapter IDs change on system restart, so matching must be done by other fields.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct LUID {
    pub low_part: u32,
    pub high_part: u32,
}

/// Rational number representation (used for refresh rates, frequencies).
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct DisplayConfigRational {
    pub numerator: u32,
    pub denominator: u32,
}

/// 2D region size.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct DisplayConfig2DRegion {
    pub cx: u32,
    pub cy: u32,
}

/// Point with x,y coordinates.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct PointL {
    pub x: i32,
    pub y: i32,
}

/// Source information for a display path.
/// Size: 20 bytes (8 + 4 + 4 + 4)
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct DisplayConfigPathSourceInfo {
    pub adapter_id: LUID,
    pub id: u32,
    pub mode_info_idx: u32,
    pub status_flags: u32,
}

/// Target information for a display path.
/// Size: 48 bytes
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct DisplayConfigPathTargetInfo {
    pub adapter_id: LUID,           // 8 bytes
    pub id: u32,                    // 4 bytes
    pub mode_info_idx: u32,         // 4 bytes
    pub output_technology: u32,     // 4 bytes
    pub rotation: u32,              // 4 bytes
    pub scaling: u32,               // 4 bytes
    pub refresh_rate: DisplayConfigRational, // 8 bytes
    pub scan_line_ordering: u32,    // 4 bytes
    pub target_available: u32,      // 4 bytes (BOOL)
    pub status_flags: u32,          // 4 bytes
}

/// Display path connecting a source to a target.
/// Size: 72 bytes (20 + 48 + 4)
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct DisplayConfigPathInfo {
    pub source_info: DisplayConfigPathSourceInfo,
    pub target_info: DisplayConfigPathTargetInfo,
    pub flags: u32,
}

/// Video signal timing information.
/// Size: 48 bytes (with padding)
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct DisplayConfigVideoSignalInfo {
    pub pixel_rate: u64,                // 8 bytes
    pub h_sync_freq: DisplayConfigRational, // 8 bytes
    pub v_sync_freq: DisplayConfigRational, // 8 bytes
    pub active_size: DisplayConfig2DRegion, // 8 bytes
    pub total_size: DisplayConfig2DRegion,  // 8 bytes
    pub video_standard: u32,            // 4 bytes
    pub scan_line_ordering: u32,        // 4 bytes
}

/// Target mode information.
/// Size: 48 bytes
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct DisplayConfigTargetMode {
    pub target_video_signal_info: DisplayConfigVideoSignalInfo,
}

/// Source mode information.
/// Size: 20 bytes
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct DisplayConfigSourceMode {
    pub width: u32,
    pub height: u32,
    pub pixel_format: u32,
    pub position: PointL,
}

/// Mode information for a display.
/// This is a union in C - either target_mode or source_mode is valid based on info_type.
/// Total size: 64 bytes (16 header + 48 union)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DisplayConfigModeInfo {
    pub info_type: u32,     // 4 bytes
    pub id: u32,            // 4 bytes
    pub adapter_id: LUID,   // 8 bytes
    /// Union data: 48 bytes (size of largest member - target mode)
    pub mode_data: [u8; 48],
}

impl Default for DisplayConfigModeInfo {
    fn default() -> Self {
        Self {
            info_type: 0,
            id: 0,
            adapter_id: LUID::default(),
            mode_data: [0u8; 48],
        }
    }
}

impl DisplayConfigModeInfo {
    /// Interpret mode_data as target mode.
    /// Only valid when info_type == MODE_INFO_TYPE_TARGET.
    pub fn get_target_mode(&self) -> &DisplayConfigTargetMode {
        unsafe { &*(self.mode_data.as_ptr() as *const DisplayConfigTargetMode) }
    }

    /// Interpret mode_data as source mode.
    /// Only valid when info_type == MODE_INFO_TYPE_SOURCE.
    pub fn get_source_mode(&self) -> &DisplayConfigSourceMode {
        unsafe { &*(self.mode_data.as_ptr() as *const DisplayConfigSourceMode) }
    }

    /// Set mode_data from target mode.
    pub fn set_target_mode(&mut self, tm: &DisplayConfigTargetMode) {
        let bytes = unsafe {
            std::slice::from_raw_parts(tm as *const _ as *const u8, 48)
        };
        self.mode_data.copy_from_slice(bytes);
    }

    /// Set mode_data from source mode.
    pub fn set_source_mode(&mut self, sm: &DisplayConfigSourceMode) {
        // Clear first (source mode is smaller than 48 bytes)
        self.mode_data = [0u8; 48];
        let bytes = unsafe {
            std::slice::from_raw_parts(sm as *const _ as *const u8, 20)
        };
        self.mode_data[..20].copy_from_slice(bytes);
    }
}

/// Header for device info requests.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct DisplayConfigDeviceInfoHeader {
    pub info_type: u32,
    pub size: u32,
    pub adapter_id: LUID,
    pub id: u32,
}

/// Device name and path for a target.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DisplayConfigTargetDeviceName {
    pub header: DisplayConfigDeviceInfoHeader,
    pub flags: u32,
    pub output_technology: u32,
    pub edid_manufacture_id: u16,
    pub edid_product_code_id: u16,
    pub connector_instance: u32,
    pub monitor_friendly_device_name: [u16; 64],
    pub monitor_device_path: [u16; 128],
}

impl Default for DisplayConfigTargetDeviceName {
    fn default() -> Self {
        Self {
            header: DisplayConfigDeviceInfoHeader::default(),
            flags: 0,
            output_technology: 0,
            edid_manufacture_id: 0,
            edid_product_code_id: 0,
            connector_instance: 0,
            monitor_friendly_device_name: [0u16; 64],
            monitor_device_path: [0u16; 128],
        }
    }
}

impl DisplayConfigTargetDeviceName {
    /// Get the monitor friendly name as a Rust string.
    pub fn get_friendly_name(&self) -> String {
        let end = self.monitor_friendly_device_name
            .iter()
            .position(|&c| c == 0)
            .unwrap_or(64);
        String::from_utf16_lossy(&self.monitor_friendly_device_name[..end])
    }

    /// Get the monitor device path as a Rust string.
    pub fn get_device_path(&self) -> String {
        let end = self.monitor_device_path
            .iter()
            .position(|&c| c == 0)
            .unwrap_or(128);
        String::from_utf16_lossy(&self.monitor_device_path[..end])
    }
}

// Constants for display configuration
pub const MODE_INFO_TYPE_SOURCE: u32 = 1;
pub const MODE_INFO_TYPE_TARGET: u32 = 2;

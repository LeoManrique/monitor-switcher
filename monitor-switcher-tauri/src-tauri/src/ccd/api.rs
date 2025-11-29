//! Windows CCD API bindings using windows-sys.

use super::types::*;
use std::mem;

#[cfg(windows)]
use windows_sys::Win32::Devices::Display::{
    DisplayConfigGetDeviceInfo, GetDisplayConfigBufferSizes, QueryDisplayConfig, SetDisplayConfig,
    QDC_ONLY_ACTIVE_PATHS, QDC_ALL_PATHS,
    SDC_APPLY, SDC_USE_SUPPLIED_DISPLAY_CONFIG, SDC_SAVE_TO_DATABASE,
    SDC_NO_OPTIMIZATION, SDC_ALLOW_CHANGES,
    DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME,
};


#[cfg(windows)]
use windows_sys::Win32::UI::WindowsAndMessaging::{
    PostMessageW, HWND_BROADCAST, WM_SYSCOMMAND,
};

/// Display settings containing paths and modes.
#[derive(Debug, Clone, Default)]
pub struct DisplaySettings {
    pub path_info_array: Vec<DisplayConfigPathInfo>,
    pub mode_info_array: Vec<DisplayConfigModeInfo>,
}

/// Monitor additional info (EDID data, friendly name).
#[derive(Debug, Clone, Default)]
pub struct MonitorAdditionalInfo {
    pub manufacture_id: u16,
    pub product_code_id: u16,
    pub valid: bool,
    pub monitor_device_path: String,
    pub monitor_friendly_device: String,
}

/// Get the current display configuration.
#[cfg(windows)]
pub fn get_display_settings(active_only: bool) -> Result<DisplaySettings, String> {
    let flags = if active_only {
        QDC_ONLY_ACTIVE_PATHS
    } else {
        QDC_ALL_PATHS
    };

    // Get buffer sizes
    let mut num_paths: u32 = 0;
    let mut num_modes: u32 = 0;

    let result = unsafe {
        GetDisplayConfigBufferSizes(flags, &mut num_paths, &mut num_modes)
    };

    if result != 0 {
        return Err(format!("GetDisplayConfigBufferSizes failed with error: {}", result));
    }

    if num_paths == 0 || num_modes == 0 {
        return Ok(DisplaySettings::default());
    }

    // Allocate buffers
    let mut path_info_array: Vec<DisplayConfigPathInfo> = vec![DisplayConfigPathInfo::default(); num_paths as usize];
    let mut mode_info_array: Vec<DisplayConfigModeInfo> = vec![DisplayConfigModeInfo::default(); num_modes as usize];

    // Query configuration
    let result = unsafe {
        QueryDisplayConfig(
            flags,
            &mut num_paths,
            path_info_array.as_mut_ptr() as *mut _,
            &mut num_modes,
            mode_info_array.as_mut_ptr() as *mut _,
            std::ptr::null_mut(),
        )
    };

    if result != 0 {
        return Err(format!("QueryDisplayConfig failed with error: {}", result));
    }

    // Trim to actual size
    path_info_array.truncate(num_paths as usize);
    mode_info_array.truncate(num_modes as usize);

    // Filter out invalid entries
    path_info_array.retain(|p| p.target_info.target_available != 0);
    mode_info_array.retain(|m| m.info_type != 0);

    Ok(DisplaySettings {
        path_info_array,
        mode_info_array,
    })
}

/// Apply display settings.
#[cfg(windows)]
pub fn set_display_settings(settings: &mut DisplaySettings) -> Result<(), String> {
    let flags = SDC_APPLY | SDC_USE_SUPPLIED_DISPLAY_CONFIG | SDC_SAVE_TO_DATABASE | SDC_NO_OPTIMIZATION;

    // First attempt without ALLOW_CHANGES
    let result = unsafe {
        SetDisplayConfig(
            settings.path_info_array.len() as u32,
            settings.path_info_array.as_mut_ptr() as *mut _,
            settings.mode_info_array.len() as u32,
            settings.mode_info_array.as_mut_ptr() as *mut _,
            flags,
        )
    };

    if result == 0 {
        return Ok(());
    }

    // Second attempt with ALLOW_CHANGES
    let flags_with_changes = flags | SDC_ALLOW_CHANGES;
    let result = unsafe {
        SetDisplayConfig(
            settings.path_info_array.len() as u32,
            settings.path_info_array.as_mut_ptr() as *mut _,
            settings.mode_info_array.len() as u32,
            settings.mode_info_array.as_mut_ptr() as *mut _,
            flags_with_changes,
        )
    };

    if result == 0 {
        Ok(())
    } else {
        Err(format!("SetDisplayConfig failed with error: {}", result))
    }
}

/// Get additional monitor info (EDID data, friendly name) for a target.
#[cfg(windows)]
pub fn get_monitor_additional_info(adapter_id: LUID, target_id: u32) -> MonitorAdditionalInfo {
    let mut device_name = DisplayConfigTargetDeviceName::default();
    device_name.header.info_type = DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME as u32;
    device_name.header.size = mem::size_of::<DisplayConfigTargetDeviceName>() as u32;
    device_name.header.adapter_id.low_part = adapter_id.low_part;
    device_name.header.adapter_id.high_part = adapter_id.high_part;
    device_name.header.id = target_id;

    let result = unsafe {
        DisplayConfigGetDeviceInfo(&mut device_name as *mut _ as *mut _)
    };

    if result == 0 {
        MonitorAdditionalInfo {
            manufacture_id: device_name.edid_manufacture_id,
            product_code_id: device_name.edid_product_code_id,
            valid: true,
            monitor_device_path: device_name.get_device_path(),
            monitor_friendly_device: device_name.get_friendly_name(),
        }
    } else {
        MonitorAdditionalInfo {
            valid: false,
            ..Default::default()
        }
    }
}

/// Turn off all monitors by broadcasting WM_SYSCOMMAND with SC_MONITORPOWER.
#[cfg(windows)]
pub fn turn_off_monitors() -> Result<(), String> {
    const SC_MONITORPOWER: usize = 0xF170;
    const MONITOR_OFF: isize = 2;

    // Small delay to let user release mouse/keyboard
    std::thread::sleep(std::time::Duration::from_millis(500));

    let result = unsafe {
        PostMessageW(
            HWND_BROADCAST,
            WM_SYSCOMMAND,
            SC_MONITORPOWER,
            MONITOR_OFF,
        )
    };

    if result != 0 {
        Ok(())
    } else {
        Err("Failed to send monitor power off message".to_string())
    }
}

// Non-Windows stubs for compilation on other platforms
#[cfg(not(windows))]
pub fn get_display_settings(_active_only: bool) -> Result<DisplaySettings, String> {
    Err("Display configuration is only supported on Windows".to_string())
}

#[cfg(not(windows))]
pub fn set_display_settings(_settings: &mut DisplaySettings) -> Result<(), String> {
    Err("Display configuration is only supported on Windows".to_string())
}

#[cfg(not(windows))]
pub fn get_monitor_additional_info(_adapter_id: LUID, _target_id: u32) -> MonitorAdditionalInfo {
    MonitorAdditionalInfo::default()
}

#[cfg(not(windows))]
pub fn turn_off_monitors() -> Result<(), String> {
    Err("Monitor power control is only supported on Windows".to_string())
}

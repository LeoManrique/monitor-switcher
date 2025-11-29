//! Profile storage operations.

use super::types::DisplayProfile;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;

/// Details about a single monitor extracted from a profile.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MonitorDetails {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub refresh_rate: f64,
    pub position_x: i32,
    pub position_y: i32,
    pub rotation: u32,
    pub is_primary: bool,
}

/// Get the profiles directory path.
pub fn get_profiles_dir() -> Result<PathBuf, String> {
    let app_data = dirs::config_dir()
        .ok_or("Could not find config directory")?;

    let profiles_dir = app_data.join("MonitorSwitcher").join("Profiles");

    // Create directory if it doesn't exist
    if !profiles_dir.exists() {
        fs::create_dir_all(&profiles_dir)
            .map_err(|e| format!("Failed to create profiles directory: {}", e))?;
    }

    Ok(profiles_dir)
}

/// Get the path for a specific profile.
pub fn get_profile_path(name: &str) -> Result<PathBuf, String> {
    let dir = get_profiles_dir()?;
    Ok(dir.join(format!("{}.json", sanitize_filename(name))))
}

/// List all saved profiles.
pub fn list_profiles() -> Result<Vec<String>, String> {
    let dir = get_profiles_dir()?;

    let mut profiles = Vec::new();

    let entries = fs::read_dir(&dir)
        .map_err(|e| format!("Failed to read profiles directory: {}", e))?;

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "json") {
            if let Some(stem) = path.file_stem() {
                if let Some(name) = stem.to_str() {
                    profiles.push(name.to_string());
                }
            }
        }
    }

    profiles.sort();
    Ok(profiles)
}

/// Check if a profile exists.
pub fn profile_exists(name: &str) -> Result<bool, String> {
    let path = get_profile_path(name)?;
    Ok(path.exists())
}

/// Save a profile to disk.
pub fn save_profile(name: &str, profile: &DisplayProfile) -> Result<(), String> {
    let path = get_profile_path(name)?;

    let json = serde_json::to_string_pretty(profile)
        .map_err(|e| format!("Failed to serialize profile: {}", e))?;

    fs::write(&path, json)
        .map_err(|e| format!("Failed to write profile file: {}", e))?;

    Ok(())
}

/// Load a profile from disk.
pub fn load_profile(name: &str) -> Result<DisplayProfile, String> {
    let path = get_profile_path(name)?;

    let json = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read profile file: {}", e))?;

    let profile: DisplayProfile = serde_json::from_str(&json)
        .map_err(|e| format!("Failed to parse profile: {}", e))?;

    Ok(profile)
}

/// Delete a profile from disk.
pub fn delete_profile(name: &str) -> Result<(), String> {
    let path = get_profile_path(name)?;

    if !path.exists() {
        return Err(format!("Profile '{}' does not exist", name));
    }

    fs::remove_file(&path)
        .map_err(|e| format!("Failed to delete profile: {}", e))?;

    Ok(())
}

/// Get detailed monitor information from a profile.
pub fn get_profile_details(name: &str) -> Result<Vec<MonitorDetails>, String> {
    let profile = load_profile(name)?;
    let mut monitors = Vec::new();

    // Each path in path_info_array represents an active display connection
    for (path_idx, path) in profile.path_info_array.iter().enumerate() {
        // Find the source mode for this path (contains resolution and position)
        let source_mode_idx = path.source_info.mode_info_idx as usize;
        let source_mode = profile
            .mode_info_array
            .get(source_mode_idx)
            .and_then(|m| m.source_mode.as_ref());

        // Get resolution and position from source mode
        let (width, height, position_x, position_y) = if let Some(src) = source_mode {
            (src.width, src.height, src.position.x, src.position.y)
        } else {
            // Fallback to target mode active size if source mode not found
            let target_mode_idx = path.target_info.mode_info_idx as usize;
            let target_mode = profile
                .mode_info_array
                .get(target_mode_idx)
                .and_then(|m| m.target_mode.as_ref());

            if let Some(tgt) = target_mode {
                (tgt.target_video_signal_info.active_size.cx,
                 tgt.target_video_signal_info.active_size.cy,
                 0, 0)
            } else {
                continue; // Skip if no mode info found
            }
        };

        // Get refresh rate from target info
        let refresh_rate = if path.target_info.refresh_rate.denominator > 0 {
            path.target_info.refresh_rate.numerator as f64
                / path.target_info.refresh_rate.denominator as f64
        } else {
            0.0
        };

        // Get monitor name from additional_info
        // The additional_info array has 2 entries per path (one for source, one for target)
        // We look for the first valid entry for this path
        let name = profile
            .additional_info
            .iter()
            .skip(path_idx * 2) // Each path has 2 additional_info entries
            .take(2)
            .find(|info| info.valid && !info.monitor_friendly_device.is_empty())
            .map(|info| info.monitor_friendly_device.clone())
            .unwrap_or_else(|| format!("Display {}", path_idx + 1));

        // Determine if this is the primary monitor (position 0,0)
        let is_primary = position_x == 0 && position_y == 0;

        monitors.push(MonitorDetails {
            name,
            width,
            height,
            refresh_rate,
            position_x,
            position_y,
            rotation: path.target_info.rotation,
            is_primary,
        });
    }

    Ok(monitors)
}

/// Sanitize a filename by removing invalid characters.
fn sanitize_filename(name: &str) -> String {
    let invalid_chars = ['\\', '/', ':', '*', '?', '"', '<', '>', '|'];
    let reserved_names = [
        "CON", "PRN", "AUX", "NUL",
        "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8", "COM9",
        "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
    ];

    let mut sanitized: String = name
        .chars()
        .filter(|c| !invalid_chars.contains(c))
        .collect();

    // Trim whitespace
    sanitized = sanitized.trim().to_string();

    // Check for reserved names (case-insensitive)
    if reserved_names.iter().any(|r| r.eq_ignore_ascii_case(&sanitized)) {
        sanitized = format!("_{}", sanitized);
    }

    // Ensure non-empty
    if sanitized.is_empty() {
        sanitized = "profile".to_string();
    }

    sanitized
}

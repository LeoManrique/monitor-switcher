//! Profile storage operations.

use super::types::DisplayProfile;
use std::fs;
use std::path::PathBuf;

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

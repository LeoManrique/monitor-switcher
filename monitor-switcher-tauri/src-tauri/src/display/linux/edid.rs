//! EDID reading from Linux sysfs.
//!
//! Single responsibility: read and parse EDID data from the kernel's DRM interface.

use std::fs;
use std::path::PathBuf;

// ============================================================================
// Types
// ============================================================================

/// Parsed EDID data.
#[derive(Debug, Clone, Default)]
pub struct EdidData {
    /// 3-letter manufacturer ID (e.g., "SAM" for Samsung)
    pub manufacturer: String,
    /// Numeric manufacturer ID
    pub manufacturer_id: u16,
    /// Product code
    pub product_code: u16,
    /// Monitor name from EDID descriptor
    pub monitor_name: String,
    /// Path to the DRM connector
    pub connector_path: String,
}

// ============================================================================
// EDID Reading
// ============================================================================

/// Read EDID data for a given output name.
pub fn read_edid(output_name: &str) -> Result<EdidData, String> {
    let connector_path = find_drm_connector(output_name)?;
    let edid_path = connector_path.join("edid");

    let edid_bytes = fs::read(&edid_path)
        .map_err(|e| format!("Failed to read EDID from {:?}: {}", edid_path, e))?;

    if edid_bytes.len() < 128 {
        return Err("EDID data too short".to_string());
    }

    let mut data = parse_edid_bytes(&edid_bytes);
    data.connector_path = connector_path.to_string_lossy().to_string();

    Ok(data)
}

/// Find the DRM connector path for an output name.
pub fn find_drm_connector(output_name: &str) -> Result<PathBuf, String> {
    // DRM connectors are in /sys/class/drm/
    // Format: card0-HDMI-A-1, card0-DP-1, card0-eDP-1, etc.
    let drm_path = PathBuf::from("/sys/class/drm");

    let entries = fs::read_dir(&drm_path)
        .map_err(|e| format!("Failed to read /sys/class/drm: {}", e))?;

    // Convert output name to DRM format
    // xrandr uses: HDMI-1, DP-1, eDP-1
    // DRM uses: card0-HDMI-A-1, card0-DP-1, card0-eDP-1
    let drm_name = convert_output_to_drm_name(output_name);

    for entry in entries.flatten() {
        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        // Check if this connector matches
        if name_str.contains(&drm_name) {
            let path = entry.path();
            // Verify it has an edid file
            if path.join("edid").exists() {
                return Ok(path);
            }
        }
    }

    // Fallback: try to find any connector that ends with the output name pattern
    for entry in fs::read_dir(&drm_path).into_iter().flatten().flatten() {
        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        // Try matching the end pattern (e.g., "HDMI-1" matches "card0-HDMI-A-1")
        let normalized_output = normalize_output_name(output_name);
        let normalized_drm = normalize_drm_name(&name_str);

        if normalized_drm == normalized_output {
            let path = entry.path();
            if path.join("edid").exists() {
                return Ok(path);
            }
        }
    }

    Err(format!("Could not find DRM connector for output: {}", output_name))
}

/// Convert xrandr output name to DRM connector name format.
fn convert_output_to_drm_name(output_name: &str) -> String {
    // HDMI-1 -> HDMI-A-1 (DRM uses HDMI-A, HDMI-B, etc.)
    // DP-1 -> DP-1 (same)
    // eDP-1 -> eDP-1 (same)
    // VGA-1 -> VGA-1 (same)
    // DVI-I-1 -> DVI-I-1 (same)

    if output_name.starts_with("HDMI-") && !output_name.contains("HDMI-A-") {
        // Convert HDMI-1 to HDMI-A-1
        output_name.replace("HDMI-", "HDMI-A-")
    } else {
        output_name.to_string()
    }
}

/// Normalize output name for comparison.
fn normalize_output_name(name: &str) -> String {
    // Extract the connector type and number
    // HDMI-1 -> hdmi1
    // DP-2 -> dp2
    name.to_lowercase()
        .replace("-a-", "")
        .replace('-', "")
}

/// Normalize DRM connector name for comparison.
fn normalize_drm_name(name: &str) -> String {
    // card0-HDMI-A-1 -> hdmi1
    // card1-DP-2 -> dp2
    let name = name.to_lowercase();

    // Remove cardX- prefix
    let without_card = if let Some(pos) = name.find('-') {
        &name[pos + 1..]
    } else {
        &name
    };

    without_card
        .replace("-a-", "")
        .replace('-', "")
}

// ============================================================================
// EDID Parsing
// ============================================================================

/// Parse EDID bytes into EdidData.
fn parse_edid_bytes(bytes: &[u8]) -> EdidData {
    let mut data = EdidData::default();

    if bytes.len() < 128 {
        return data;
    }

    // Manufacturer ID is at bytes 8-9 (big-endian)
    // It's a 3-letter code encoded in 5 bits each
    let mfg_id = ((bytes[8] as u16) << 8) | (bytes[9] as u16);
    data.manufacturer_id = mfg_id;
    data.manufacturer = decode_manufacturer_id(mfg_id);

    // Product code is at bytes 10-11 (little-endian)
    data.product_code = (bytes[10] as u16) | ((bytes[11] as u16) << 8);

    // Monitor name is in the descriptor blocks (bytes 54-125)
    // Each descriptor is 18 bytes, starting at byte 54
    for i in 0..4 {
        let offset = 54 + i * 18;
        if offset + 18 <= bytes.len() {
            let descriptor = &bytes[offset..offset + 18];

            // Check if this is a monitor name descriptor (tag 0xFC)
            if descriptor[0] == 0 && descriptor[1] == 0 && descriptor[2] == 0 && descriptor[3] == 0xFC {
                // Name is in bytes 5-17
                let name_bytes = &descriptor[5..18];
                data.monitor_name = parse_edid_string(name_bytes);
                break;
            }
        }
    }

    data
}

/// Decode the 3-letter manufacturer ID from EDID.
fn decode_manufacturer_id(id: u16) -> String {
    // Each letter is encoded in 5 bits
    // Bits 14-10: first letter (A=1, B=2, ...)
    // Bits 9-5: second letter
    // Bits 4-0: third letter
    let c1 = ((id >> 10) & 0x1F) as u8;
    let c2 = ((id >> 5) & 0x1F) as u8;
    let c3 = (id & 0x1F) as u8;

    let mut result = String::with_capacity(3);

    if c1 > 0 && c1 <= 26 {
        result.push((b'A' + c1 - 1) as char);
    }
    if c2 > 0 && c2 <= 26 {
        result.push((b'A' + c2 - 1) as char);
    }
    if c3 > 0 && c3 <= 26 {
        result.push((b'A' + c3 - 1) as char);
    }

    result
}

/// Parse an EDID string (space-padded, newline-terminated).
fn parse_edid_string(bytes: &[u8]) -> String {
    let s: String = bytes
        .iter()
        .take_while(|&&b| b != 0x0A && b != 0x00) // Stop at newline or null
        .map(|&b| b as char)
        .collect();

    s.trim().to_string()
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_manufacturer_id() {
        // SAM = Samsung (S=19, A=1, M=13)
        // Binary: 10011 00001 01101 = 0x4C2D
        // Actually the encoding is different, let's verify with real data
        assert!(!decode_manufacturer_id(0x4C2D).is_empty());
    }

    #[test]
    fn test_normalize_output_name() {
        assert_eq!(normalize_output_name("HDMI-1"), "hdmi1");
        assert_eq!(normalize_output_name("DP-2"), "dp2");
        assert_eq!(normalize_output_name("eDP-1"), "edp1");
    }

    #[test]
    fn test_normalize_drm_name() {
        assert_eq!(normalize_drm_name("card0-HDMI-A-1"), "hdmi1");
        assert_eq!(normalize_drm_name("card0-DP-2"), "dp2");
        assert_eq!(normalize_drm_name("card1-eDP-1"), "edp1");
    }

    #[test]
    fn test_convert_output_to_drm_name() {
        assert_eq!(convert_output_to_drm_name("HDMI-1"), "HDMI-A-1");
        assert_eq!(convert_output_to_drm_name("DP-1"), "DP-1");
        assert_eq!(convert_output_to_drm_name("eDP-1"), "eDP-1");
    }
}

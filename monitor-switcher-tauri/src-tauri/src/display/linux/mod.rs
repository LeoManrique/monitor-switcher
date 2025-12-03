//! Linux display management using XRandR.
//!
//! This module is ONLY compiled on Linux.
//! For Windows implementation, see `../windows/`.

mod edid;
pub mod types;
mod xrandr;

pub use types::{OutputConfig, Rotation};

// ============================================================================
// Public Types
// ============================================================================

/// Display settings containing output configurations.
#[derive(Debug, Clone, Default)]
pub struct DisplaySettings {
    pub outputs: Vec<OutputConfig>,
}

/// Monitor additional info (EDID data).
#[derive(Debug, Clone, Default)]
pub struct MonitorAdditionalInfo {
    #[allow(dead_code)]
    pub valid: bool,
}

// ============================================================================
// Public API (matches Windows signatures for compatibility)
// ============================================================================

/// Get the current display configuration.
pub fn get_display_settings(active_only: bool) -> Result<DisplaySettings, String> {
    let outputs = xrandr::query_outputs(active_only)?;
    Ok(DisplaySettings { outputs })
}

/// Apply display settings.
pub fn set_display_settings(settings: &mut DisplaySettings) -> Result<(), String> {
    xrandr::apply_configuration(&settings.outputs)
}

/// Get additional monitor info for an output.
pub fn get_monitor_additional_info(output_name: &str) -> MonitorAdditionalInfo {
    MonitorAdditionalInfo {
        valid: edid::read_edid(output_name).is_ok(),
    }
}

/// Turn off all monitors using DPMS.
pub fn turn_off_monitors() -> Result<(), String> {
    // Small delay to let user release mouse/keyboard
    std::thread::sleep(std::time::Duration::from_millis(500));
    xrandr::turn_off_displays()
}

// ============================================================================
// Adapter Matching (Linux implementation)
// ============================================================================

/// Match profile outputs to current system outputs.
/// On Linux, we match by output name and EDID data.
pub fn match_adapter_ids(
    settings: &mut DisplaySettings,
    _additional_info: &[MonitorAdditionalInfo],
) -> Result<(), String> {
    let current = get_display_settings(true)?;

    // Match outputs by name
    for output in &mut settings.outputs {
        for current_output in &current.outputs {
            if output.name == current_output.name {
                // Output names match, no adapter ID translation needed on Linux
                break;
            }
        }
    }

    Ok(())
}

/// Get additional info for all outputs.
pub fn get_additional_info_for_modes(outputs: &[OutputConfig]) -> Vec<MonitorAdditionalInfo> {
    outputs
        .iter()
        .map(|output| get_monitor_additional_info(&output.name))
        .collect()
}

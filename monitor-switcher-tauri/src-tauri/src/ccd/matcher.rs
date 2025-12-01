//! Adapter ID matching logic for display profiles.
//!
//! Adapter IDs (LUIDs) change on system restart, so we need to match profiles
//! to current system state using multiple fallback strategies.

use super::types::*;
use super::api::{DisplaySettings, MonitorAdditionalInfo, get_display_settings, get_monitor_additional_info};
use log::{debug, warn};

/// Match profile adapter IDs to current system adapter IDs.
/// Uses a 3-tier fallback strategy:
/// 1. Match by source/target ID pairs
/// 2. Match by monitor friendly name (EDID)
/// 3. Bulk adapter ID replacement
pub fn match_adapter_ids(
    settings: &mut DisplaySettings,
    additional_info: &[MonitorAdditionalInfo],
) -> Result<(), String> {
    // Get current display settings
    let current = get_display_settings(true)?;
    let current_additional_info = get_additional_info_for_modes(&current.mode_info_array);

    // Try tier 1: Match by source/target ID pairs
    if try_match_by_ids(settings, &current) {
        debug!("Adapter matching: Tier 1 (ID pairs) succeeded");
        return Ok(());
    }

    // Try tier 2: Match by monitor friendly name
    if try_match_by_friendly_name(settings, additional_info, &current, &current_additional_info) {
        debug!("Adapter matching: Tier 2 (friendly name) succeeded");
        return Ok(());
    }

    // Try tier 3: Bulk replacement
    if try_bulk_replacement(settings, &current) {
        debug!("Adapter matching: Tier 3 (bulk replacement) succeeded");
        return Ok(());
    }

    warn!("Adapter matching: All tiers failed, using original IDs");
    Ok(())
}

/// Tier 1: Match by source and target ID pairs.
fn try_match_by_ids(settings: &mut DisplaySettings, current: &DisplaySettings) -> bool {
    let mut matched_any = false;

    // Match paths by source/target IDs
    for path in &mut settings.path_info_array {
        for current_path in &current.path_info_array {
            if path.source_info.id == current_path.source_info.id
                && path.target_info.id == current_path.target_info.id
            {
                path.source_info.adapter_id = current_path.source_info.adapter_id;
                path.target_info.adapter_id = current_path.target_info.adapter_id;
                matched_any = true;
                break;
            }
        }
    }

    if !matched_any {
        return false;
    }

    // Match mode infos by correlating with paths
    for mode in &mut settings.mode_info_array {
        // Find a path that references this mode's adapter
        for path in &settings.path_info_array {
            if mode.info_type == MODE_INFO_TYPE_TARGET && mode.id == path.target_info.id {
                // Find current mode with same id
                for current_mode in &current.mode_info_array {
                    if current_mode.info_type == MODE_INFO_TYPE_TARGET
                        && current_mode.id == mode.id
                    {
                        mode.adapter_id = current_mode.adapter_id;
                        break;
                    }
                }
                break;
            } else if mode.info_type == MODE_INFO_TYPE_SOURCE && mode.id == path.source_info.id {
                for current_mode in &current.mode_info_array {
                    if current_mode.info_type == MODE_INFO_TYPE_SOURCE
                        && current_mode.id == mode.id
                    {
                        mode.adapter_id = current_mode.adapter_id;
                        break;
                    }
                }
                break;
            }
        }
    }

    true
}

/// Tier 2: Match by monitor friendly device name.
fn try_match_by_friendly_name(
    settings: &mut DisplaySettings,
    additional_info: &[MonitorAdditionalInfo],
    current: &DisplaySettings,
    current_additional_info: &[MonitorAdditionalInfo],
) -> bool {
    let mut matched_any = false;

    for (i, mode) in settings.mode_info_array.iter_mut().enumerate() {
        if mode.info_type != MODE_INFO_TYPE_TARGET {
            continue;
        }

        let Some(saved_info) = additional_info.get(i).filter(|info| info.valid) else {
            continue;
        };
        if saved_info.monitor_friendly_device.is_empty() {
            continue;
        }

        // Find matching current monitor by friendly name
        for (j, current_mode) in current.mode_info_array.iter().enumerate() {
            if current_mode.info_type != MODE_INFO_TYPE_TARGET {
                continue;
            }

            let Some(current_info) = current_additional_info.get(j).filter(|info| info.valid) else {
                continue;
            };

            if current_info.monitor_friendly_device == saved_info.monitor_friendly_device {
                mode.adapter_id = current_mode.adapter_id;
                mode.id = current_mode.id;
                matched_any = true;
                break;
            }
        }
    }

    if matched_any {
        // Update paths based on matched modes
        update_path_adapter_ids_from_modes(settings, current);
    }

    matched_any
}

/// Tier 3: Bulk replacement of old adapter IDs with new ones.
fn try_bulk_replacement(settings: &mut DisplaySettings, current: &DisplaySettings) -> bool {
    // Find one matching path to get the old->new adapter ID mapping
    for path in &settings.path_info_array {
        for current_path in &current.path_info_array {
            // Try to find any matching criteria
            if path.source_info.id == current_path.source_info.id {
                let old_id = path.source_info.adapter_id;
                let new_id = current_path.source_info.adapter_id;

                if old_id != new_id {
                    replace_all_adapter_ids(settings, old_id, new_id);
                    return true;
                }
            }
        }
    }

    false
}

/// Replace all occurrences of old adapter ID with new one.
fn replace_all_adapter_ids(settings: &mut DisplaySettings, old_id: LUID, new_id: LUID) {
    for path in &mut settings.path_info_array {
        if path.source_info.adapter_id == old_id {
            path.source_info.adapter_id = new_id;
        }
        if path.target_info.adapter_id == old_id {
            path.target_info.adapter_id = new_id;
        }
    }

    for mode in &mut settings.mode_info_array {
        if mode.adapter_id == old_id {
            mode.adapter_id = new_id;
        }
    }
}

/// Update path adapter IDs based on matched mode adapter IDs.
fn update_path_adapter_ids_from_modes(settings: &mut DisplaySettings, current: &DisplaySettings) {
    for path in &mut settings.path_info_array {
        // Find current path with same source/target IDs if possible
        for current_path in &current.path_info_array {
            if path.source_info.id == current_path.source_info.id {
                path.source_info.adapter_id = current_path.source_info.adapter_id;
            }
            if path.target_info.id == current_path.target_info.id {
                path.target_info.adapter_id = current_path.target_info.adapter_id;
            }
        }
    }
}

/// Get additional info for all target modes in the array.
pub fn get_additional_info_for_modes(mode_info_array: &[DisplayConfigModeInfo]) -> Vec<MonitorAdditionalInfo> {
    mode_info_array
        .iter()
        .map(|mode| {
            if mode.info_type == MODE_INFO_TYPE_TARGET {
                get_monitor_additional_info(mode.adapter_id, mode.id)
            } else {
                MonitorAdditionalInfo::default()
            }
        })
        .collect()
}

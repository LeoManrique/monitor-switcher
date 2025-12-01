//! Conversion between CCD types and profile JSON types.

use crate::ccd::{
    DisplaySettings, MonitorAdditionalInfo,
    DisplayConfigPathInfo, DisplayConfigModeInfo,
    DisplayConfigTargetMode, DisplayConfigSourceMode,
    MODE_INFO_TYPE_SOURCE, MODE_INFO_TYPE_TARGET,
    LUID, DisplayConfigRational, DisplayConfig2DRegion, PointL,
    DisplayConfigPathSourceInfo, DisplayConfigPathTargetInfo,
    DisplayConfigVideoSignalInfo,
    get_dpi_scaling_info,
};
use super::types::*;

/// Convert CCD DisplaySettings to a DisplayProfile for JSON serialization.
pub fn settings_to_profile(
    settings: &DisplaySettings,
    additional_info: &[MonitorAdditionalInfo],
) -> DisplayProfile {
    let path_info_array = settings
        .path_info_array
        .iter()
        .map(|p| path_info_to_json(p))
        .collect();

    let mode_info_array = settings
        .mode_info_array
        .iter()
        .map(|m| mode_info_to_json(m))
        .collect();

    let additional = additional_info
        .iter()
        .map(|a| ProfileMonitorInfo {
            manufacture_id: a.manufacture_id,
            product_code_id: a.product_code_id,
            valid: a.valid,
            monitor_device_path: a.monitor_device_path.clone(),
            monitor_friendly_device: a.monitor_friendly_device.clone(),
        })
        .collect();

    // Collect DPI scaling info for each source
    let dpi_scale_info: Vec<DpiScaleInfo> = settings
        .path_info_array
        .iter()
        .filter_map(|p| {
            get_dpi_scaling_info(p.source_info.adapter_id, p.source_info.id)
                .map(|info| DpiScaleInfo {
                    source_id: p.source_info.id,
                    dpi_scale: info.current,
                })
        })
        .collect();

    DisplayProfile {
        version: 1,
        path_info_array,
        mode_info_array,
        additional_info: additional,
        dpi_scale_info,
    }
}

/// Convert a DisplayProfile back to CCD DisplaySettings.
pub fn profile_to_settings(profile: &DisplayProfile) -> (DisplaySettings, Vec<MonitorAdditionalInfo>) {
    let path_info_array = profile
        .path_info_array
        .iter()
        .map(|p| path_info_from_json(p))
        .collect();

    let mode_info_array = profile
        .mode_info_array
        .iter()
        .map(|m| mode_info_from_json(m))
        .collect();

    let additional_info = profile
        .additional_info
        .iter()
        .map(|a| MonitorAdditionalInfo {
            manufacture_id: a.manufacture_id,
            product_code_id: a.product_code_id,
            valid: a.valid,
            monitor_device_path: a.monitor_device_path.clone(),
            monitor_friendly_device: a.monitor_friendly_device.clone(),
        })
        .collect();

    (
        DisplaySettings {
            path_info_array,
            mode_info_array,
        },
        additional_info,
    )
}

fn path_info_to_json(p: &DisplayConfigPathInfo) -> PathInfo {
    PathInfo {
        source_info: PathSourceInfo {
            adapter_id: AdapterId {
                low_part: p.source_info.adapter_id.low_part,
                high_part: p.source_info.adapter_id.high_part,
            },
            id: p.source_info.id,
            mode_info_idx: p.source_info.mode_info_idx,
            status_flags: p.source_info.status_flags,
        },
        target_info: PathTargetInfo {
            adapter_id: AdapterId {
                low_part: p.target_info.adapter_id.low_part,
                high_part: p.target_info.adapter_id.high_part,
            },
            id: p.target_info.id,
            mode_info_idx: p.target_info.mode_info_idx,
            output_technology: p.target_info.output_technology,
            rotation: p.target_info.rotation,
            scaling: p.target_info.scaling,
            refresh_rate: Rational {
                numerator: p.target_info.refresh_rate.numerator,
                denominator: p.target_info.refresh_rate.denominator,
            },
            scan_line_ordering: p.target_info.scan_line_ordering,
            target_available: p.target_info.target_available != 0,
            status_flags: p.target_info.status_flags,
        },
        flags: p.flags,
    }
}

fn path_info_from_json(p: &PathInfo) -> DisplayConfigPathInfo {
    DisplayConfigPathInfo {
        source_info: DisplayConfigPathSourceInfo {
            adapter_id: LUID {
                low_part: p.source_info.adapter_id.low_part,
                high_part: p.source_info.adapter_id.high_part,
            },
            id: p.source_info.id,
            mode_info_idx: p.source_info.mode_info_idx,
            status_flags: p.source_info.status_flags,
        },
        target_info: DisplayConfigPathTargetInfo {
            adapter_id: LUID {
                low_part: p.target_info.adapter_id.low_part,
                high_part: p.target_info.adapter_id.high_part,
            },
            id: p.target_info.id,
            mode_info_idx: p.target_info.mode_info_idx,
            output_technology: p.target_info.output_technology,
            rotation: p.target_info.rotation,
            scaling: p.target_info.scaling,
            refresh_rate: DisplayConfigRational {
                numerator: p.target_info.refresh_rate.numerator,
                denominator: p.target_info.refresh_rate.denominator,
            },
            scan_line_ordering: p.target_info.scan_line_ordering,
            target_available: if p.target_info.target_available { 1 } else { 0 },
            status_flags: p.target_info.status_flags,
        },
        flags: p.flags,
    }
}

fn mode_info_to_json(m: &DisplayConfigModeInfo) -> ModeInfo {
    let (target_mode, source_mode) = if m.info_type == MODE_INFO_TYPE_TARGET {
        let tm = m.get_target_mode();
        (
            Some(TargetMode {
                target_video_signal_info: VideoSignalInfo {
                    pixel_rate: tm.target_video_signal_info.pixel_rate as i64,
                    h_sync_freq: Rational {
                        numerator: tm.target_video_signal_info.h_sync_freq.numerator,
                        denominator: tm.target_video_signal_info.h_sync_freq.denominator,
                    },
                    v_sync_freq: Rational {
                        numerator: tm.target_video_signal_info.v_sync_freq.numerator,
                        denominator: tm.target_video_signal_info.v_sync_freq.denominator,
                    },
                    active_size: Region2D {
                        cx: tm.target_video_signal_info.active_size.cx,
                        cy: tm.target_video_signal_info.active_size.cy,
                    },
                    total_size: Region2D {
                        cx: tm.target_video_signal_info.total_size.cx,
                        cy: tm.target_video_signal_info.total_size.cy,
                    },
                    video_standard: tm.target_video_signal_info.video_standard,
                    scan_line_ordering: tm.target_video_signal_info.scan_line_ordering,
                },
            }),
            None,
        )
    } else if m.info_type == MODE_INFO_TYPE_SOURCE {
        let sm = m.get_source_mode();
        (
            None,
            Some(SourceMode {
                width: sm.width,
                height: sm.height,
                pixel_format: sm.pixel_format,
                position: Point {
                    x: sm.position.x,
                    y: sm.position.y,
                },
            }),
        )
    } else {
        (None, None)
    };

    ModeInfo {
        info_type: m.info_type,
        id: m.id,
        adapter_id: AdapterId {
            low_part: m.adapter_id.low_part,
            high_part: m.adapter_id.high_part,
        },
        target_mode,
        source_mode,
    }
}

fn mode_info_from_json(m: &ModeInfo) -> DisplayConfigModeInfo {
    let mut mode = DisplayConfigModeInfo {
        info_type: m.info_type,
        id: m.id,
        adapter_id: LUID {
            low_part: m.adapter_id.low_part,
            high_part: m.adapter_id.high_part,
        },
        mode_data: [0u8; 48],
    };

    if let Some(ref tm) = m.target_mode {
        let target = DisplayConfigTargetMode {
            target_video_signal_info: DisplayConfigVideoSignalInfo {
                pixel_rate: tm.target_video_signal_info.pixel_rate as u64,
                h_sync_freq: DisplayConfigRational {
                    numerator: tm.target_video_signal_info.h_sync_freq.numerator,
                    denominator: tm.target_video_signal_info.h_sync_freq.denominator,
                },
                v_sync_freq: DisplayConfigRational {
                    numerator: tm.target_video_signal_info.v_sync_freq.numerator,
                    denominator: tm.target_video_signal_info.v_sync_freq.denominator,
                },
                active_size: DisplayConfig2DRegion {
                    cx: tm.target_video_signal_info.active_size.cx,
                    cy: tm.target_video_signal_info.active_size.cy,
                },
                total_size: DisplayConfig2DRegion {
                    cx: tm.target_video_signal_info.total_size.cx,
                    cy: tm.target_video_signal_info.total_size.cy,
                },
                video_standard: tm.target_video_signal_info.video_standard,
                scan_line_ordering: tm.target_video_signal_info.scan_line_ordering,
            },
        };
        mode.set_target_mode(&target);
    } else if let Some(ref sm) = m.source_mode {
        let source = DisplayConfigSourceMode {
            width: sm.width,
            height: sm.height,
            pixel_format: sm.pixel_format,
            position: PointL {
                x: sm.position.x,
                y: sm.position.y,
            },
        };
        mode.set_source_mode(&source);
    }

    mode
}

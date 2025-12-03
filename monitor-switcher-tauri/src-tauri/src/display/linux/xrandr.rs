//! XRandR command execution and output parsing.
//!
//! Single responsibility: interact with the xrandr command-line tool.

use super::types::OutputConfig;
use super::Rotation;
use std::process::Command;

// ============================================================================
// Query Display Configuration
// ============================================================================

/// Query current display outputs using xrandr.
pub fn query_outputs(active_only: bool) -> Result<Vec<OutputConfig>, String> {
    let output = Command::new("xrandr")
        .arg("--query")
        .output()
        .map_err(|e| format!("Failed to execute xrandr: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "xrandr query failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let outputs = parse_xrandr_output(&stdout);

    if active_only {
        Ok(outputs.into_iter().filter(|o| o.enabled).collect())
    } else {
        Ok(outputs)
    }
}

/// Parse xrandr --query output into OutputConfig structs.
fn parse_xrandr_output(output: &str) -> Vec<OutputConfig> {
    let mut outputs = Vec::new();
    let mut current_output: Option<OutputConfig> = None;

    for line in output.lines() {
        // Output line format: "HDMI-1 connected primary 1920x1080+0+0 (normal left inverted right x axis y axis) 527mm x 296mm"
        // Or: "DP-1 disconnected (normal left inverted right x axis y axis)"
        if line.contains(" connected") || line.contains(" disconnected") {
            // Save previous output if any
            if let Some(out) = current_output.take() {
                outputs.push(out);
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            let name = parts[0].to_string();
            let connected = parts.get(1).map_or(false, |s| *s == "connected");

            if !connected {
                // Disconnected output - still record it but as disabled
                current_output = Some(OutputConfig {
                    name,
                    enabled: false,
                    ..Default::default()
                });
                continue;
            }

            let mut config = OutputConfig {
                name,
                enabled: false, // Will be set true if we find resolution
                ..Default::default()
            };

            // Check for primary
            let mut idx = 2;
            if parts.get(idx) == Some(&"primary") {
                config.primary = true;
                idx += 1;
            }

            // Parse geometry (e.g., "1920x1080+0+0")
            if let Some(geom) = parts.get(idx) {
                if let Some((res, pos)) = parse_geometry(geom) {
                    config.width = res.0;
                    config.height = res.1;
                    config.pos_x = pos.0;
                    config.pos_y = pos.1;
                    config.enabled = true;
                    idx += 1;
                }
            }

            // Parse rotation - it appears after geometry, before parentheses
            // Format: "DP-4 connected 1440x2560+7680+0 left (normal left...)"
            // Check if the next part is a rotation keyword
            if let Some(rotation_candidate) = parts.get(idx) {
                // It's rotation if it's not the start of parentheses
                if !rotation_candidate.starts_with('(') {
                    config.rotation = Rotation::from_xrandr(rotation_candidate);
                }
            }

            current_output = Some(config);
        }
        // Mode line format: "   1920x1080     60.00*+  50.00    59.94"
        // The asterisk (*) marks the current mode, plus (+) marks preferred
        else if line.starts_with("   ") && current_output.is_some() {
            let line = line.trim();
            if let Some(output) = current_output.as_mut() {
                // Only parse if this is the active mode (has *)
                if line.contains('*') {
                    if let Some((width, height, refresh)) = parse_mode_line(line) {
                        output.width = width;
                        output.height = height;
                        output.refresh_rate = refresh;
                        output.enabled = true;
                    }
                }
            }
        }
    }

    // Don't forget the last output
    if let Some(out) = current_output {
        outputs.push(out);
    }

    outputs
}

/// Parse geometry string like "1920x1080+0+0" into ((width, height), (x, y)).
fn parse_geometry(geom: &str) -> Option<((u32, u32), (i32, i32))> {
    // Split by 'x' first to get width and the rest
    let parts: Vec<&str> = geom.split('x').collect();
    if parts.len() != 2 {
        return None;
    }

    let width: u32 = parts[0].parse().ok()?;

    // The rest is "height+x+y" or "height-x+y" etc.
    let rest = parts[1];

    // Find the first + or - after the height
    let height_end = rest
        .chars()
        .position(|c| c == '+' || c == '-')
        .unwrap_or(rest.len());

    let height: u32 = rest[..height_end].parse().ok()?;

    if height_end >= rest.len() {
        return Some(((width, height), (0, 0)));
    }

    // Parse position
    let pos_str = &rest[height_end..];
    let (x, y) = parse_position(pos_str)?;

    Some(((width, height), (x, y)))
}

/// Parse position string like "+0+0" or "+1920+0" into (x, y).
fn parse_position(pos: &str) -> Option<(i32, i32)> {
    let mut chars = pos.chars().peekable();
    let mut x_str = String::new();
    let mut y_str = String::new();

    // Parse X
    if let Some(sign) = chars.next() {
        if sign == '+' || sign == '-' {
            if sign == '-' {
                x_str.push('-');
            }
            while let Some(&c) = chars.peek() {
                if c.is_ascii_digit() {
                    x_str.push(chars.next().unwrap());
                } else {
                    break;
                }
            }
        }
    }

    // Parse Y
    if let Some(sign) = chars.next() {
        if sign == '+' || sign == '-' {
            if sign == '-' {
                y_str.push('-');
            }
            while let Some(&c) = chars.peek() {
                if c.is_ascii_digit() {
                    y_str.push(chars.next().unwrap());
                } else {
                    break;
                }
            }
        }
    }

    let x: i32 = x_str.parse().ok()?;
    let y: i32 = y_str.parse().ok()?;

    Some((x, y))
}

/// Parse mode line like "1920x1080     60.00*+" into (width, height, refresh_rate).
fn parse_mode_line(line: &str) -> Option<(u32, u32, f32)> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.is_empty() {
        return None;
    }

    // Parse resolution
    let res_parts: Vec<&str> = parts[0].split('x').collect();
    if res_parts.len() != 2 {
        return None;
    }

    // Handle interlaced modes (e.g., "1920x1080i")
    let height_str = res_parts[1].trim_end_matches('i');

    let width: u32 = res_parts[0].parse().ok()?;
    let height: u32 = height_str.parse().ok()?;

    // Find the refresh rate with asterisk
    let mut refresh = 60.0f32;
    for part in &parts[1..] {
        if part.contains('*') {
            // Remove * and + characters
            let rate_str = part.replace(['*', '+'], "");
            if let Ok(rate) = rate_str.parse::<f32>() {
                refresh = rate;
                break;
            }
        }
    }

    Some((width, height, refresh))
}

// ============================================================================
// Apply Display Configuration
// ============================================================================

/// Apply display configuration using xrandr.
/// This will also turn off any connected outputs not in the provided list.
pub fn apply_configuration(outputs: &[OutputConfig]) -> Result<(), String> {
    // Get current outputs to find ones we need to turn off
    let current_outputs = query_outputs(false)?;
    let profile_output_names: Vec<&str> = outputs.iter().map(|o| o.name.as_str()).collect();

    let mut args = Vec::new();

    // First, turn off any connected outputs not in the profile
    for current in &current_outputs {
        if current.enabled && !profile_output_names.contains(&current.name.as_str()) {
            args.push("--output".to_string());
            args.push(current.name.clone());
            args.push("--off".to_string());
        }
    }

    // Then configure the outputs in the profile
    for output in outputs {
        args.push("--output".to_string());
        args.push(output.name.clone());

        if output.enabled {
            // Mode
            args.push("--mode".to_string());
            args.push(format!("{}x{}", output.width, output.height));

            // Refresh rate
            args.push("--rate".to_string());
            args.push(format!("{:.2}", output.refresh_rate));

            // Position
            args.push("--pos".to_string());
            args.push(format!("{}x{}", output.pos_x, output.pos_y));

            // Rotation
            args.push("--rotate".to_string());
            args.push(output.rotation.to_xrandr_arg().to_string());

            // Primary
            if output.primary {
                args.push("--primary".to_string());
            }

            // Scale (if not 1.0)
            if (output.scale - 1.0).abs() > 0.01 {
                args.push("--scale".to_string());
                args.push(format!("{}x{}", output.scale, output.scale));
            }
        } else {
            args.push("--off".to_string());
        }
    }

    let output = Command::new("xrandr")
        .args(&args)
        .output()
        .map_err(|e| format!("Failed to execute xrandr: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "xrandr failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}

// ============================================================================
// Monitor Power Control
// ============================================================================

/// Turn off all displays using DPMS.
pub fn turn_off_displays() -> Result<(), String> {
    // Try xset first (X11)
    let output = Command::new("xset")
        .args(["dpms", "force", "off"])
        .output();

    match output {
        Ok(result) if result.status.success() => Ok(()),
        _ => {
            // Fallback: try xrandr to set all outputs to off temporarily
            // This is less ideal but works in more environments
            Err("Failed to turn off monitors using DPMS. Try running: xset dpms force off".to_string())
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_geometry() {
        assert_eq!(
            parse_geometry("1920x1080+0+0"),
            Some(((1920, 1080), (0, 0)))
        );
        assert_eq!(
            parse_geometry("2560x1440+1920+0"),
            Some(((2560, 1440), (1920, 0)))
        );
        assert_eq!(
            parse_geometry("1920x1080+0+1080"),
            Some(((1920, 1080), (0, 1080)))
        );
    }

    #[test]
    fn test_parse_mode_line() {
        assert_eq!(
            parse_mode_line("1920x1080     60.00*+"),
            Some((1920, 1080, 60.0))
        );
        assert_eq!(
            parse_mode_line("2560x1440     144.00*"),
            Some((2560, 1440, 144.0))
        );
    }

    #[test]
    fn test_parse_position() {
        assert_eq!(parse_position("+0+0"), Some((0, 0)));
        assert_eq!(parse_position("+1920+0"), Some((1920, 0)));
        assert_eq!(parse_position("-100+200"), Some((-100, 200)));
    }

}

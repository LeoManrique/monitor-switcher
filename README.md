# Monitor Profile Switcher

A Windows utility for saving and restoring multi-monitor display configurations. Quickly switch between different monitor setups with a click from the system tray or via customizable hotkeys.

> **Fork Note:** This is a fork of [Martin Krämer's Monitor Profile Switcher](https://sourceforge.net/projects/monitorswitcher/), upgraded to .NET 4.8 with JSON configuration storage.

## Features

- **Save/Load Monitor Profiles** - Capture your current display configuration and restore it instantly
- **System Tray Integration** - Quick access to all profiles from the taskbar
- **Global Hotkeys** - Assign keyboard shortcuts to switch profiles without touching the mouse
- **Multi-GPU Support** - Works with systems using multiple graphics cards
- **Turn Off Monitors** - One-click option to put all displays into power saving mode

## Use Cases

- Enable/disable a TV connected via HDMI
- Switch primary display between monitors
- Change monitor arrangement (left/right positioning)
- Toggle between work and gaming display setups
- Quickly adapt to docking/undocking a laptop

## Components

| Component | Description |
|-----------|-------------|
| **MonitorSwitcher.exe** | Command-line tool for saving/loading profiles |
| **MonitorSwitcherGUI.exe** | System tray application with hotkey support |

## Requirements

- Windows 7 or later
- .NET Framework 4.8

## Installation

1. Download the latest release
2. Extract to a folder of your choice
3. Run `MonitorSwitcherGUI.exe` for the tray app, or use `MonitorSwitcher.exe` from command line

## Usage

### GUI (System Tray)

1. Run `MonitorSwitcherGUI.exe`
2. Right-click the tray icon to access the menu
3. **Save Profile** > **New Profile...** to save your current configuration
4. Click any saved profile name to load it
5. **Set Hotkeys** to assign keyboard shortcuts to profiles

### Command Line

```
MonitorSwitcher.exe -save:ProfileName.json    Save current configuration
MonitorSwitcher.exe -load:ProfileName.json    Load and apply a profile
MonitorSwitcher.exe -print                    Print current config to console
MonitorSwitcher.exe -debug -load:Profile.json Enable debug output
MonitorSwitcher.exe -noidmatch -load:Profile.json  Disable adapter ID matching
```

### Custom Settings Directory

```
MonitorSwitcherGUI.exe -settings:C:\Path\To\Settings
```

## Configuration

Profiles and settings are stored in:
```
%AppData%\MonitorSwitcher\
├── Profiles\          # Monitor profiles (.json)
│   ├── Work.json
│   └── Gaming.json
└── Hotkeys.json       # Hotkey assignments
```

## Technical Details

- Uses the Windows [CCD (Connecting and Configuring Displays) API](https://docs.microsoft.com/en-us/windows-hardware/drivers/display/ccd-apis)
- Adapter IDs change on system restart; the tool uses multiple fallback methods to match saved profiles to current hardware
- Profile format: JSON containing `PathInfoArray`, `ModeInfoArray`, and monitor EDID information

## Troubleshooting

**Profile won't load after reboot:**
The tool has multiple adapter ID matching strategies. If issues persist, try re-saving the profile.

**All monitors disabled:**
Boot into Safe Mode to fix display settings, or connect via Remote Desktop.

**Hotkeys not working:**
Ensure no other application has registered the same hotkey combination.

## Fork Changes (v0.9.0.0)

- Upgraded target framework from .NET 4.0 to .NET 4.8
- Migrated configuration storage from XML to JSON format
- Improved SetDisplayConfig call sequence (tries without `AllowChanges` flag first)
- Removed MonitorSwitcherGUIConfig project
- Various code cleanups

## Original Project

- **Website:** https://sourceforge.net/projects/monitorswitcher/
- **Author:** Martin Krämer (MartinKraemer84@gmail.com)

## License

This project is licensed under the [Mozilla Public License 2.0](http://mozilla.org/MPL/2.0/).

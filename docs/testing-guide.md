# Monitor Switcher - Testing Guide

This document outlines test cases to verify the app handles various scenarios correctly, including interactions between the GUI, system tray, and manual user changes.

## Architecture Overview

The app uses a unified event system (`profile-changed`) to keep the GUI and tray in sync:
- **Shared core functions**: `do_load_profile()` and `do_delete_profile()` are used by both GUI commands and tray menu actions
- **Event-driven updates**: Any profile change emits a `profile-changed` event that the frontend listens to
- **Automatic refresh**: The GUI refreshes after a 500ms delay to allow Windows to apply display changes

---

## Test Cases

### 1. Profile Loading

#### 1.1 Load from GUI
- [ ] Click a profile card in the main window
- [ ] Verify loading spinner appears on the card
- [ ] Verify display configuration changes
- [ ] Verify the loaded profile becomes highlighted (sky blue)
- [ ] Verify notification toast appears

#### 1.2 Load from Tray
- [ ] Right-click tray icon > Load Profile > select a profile
- [ ] Verify display configuration changes
- [ ] Open the GUI window
- [ ] Verify the correct profile is highlighted

#### 1.3 Load while GUI is open (from Tray)
- [ ] Keep GUI window open
- [ ] Load a different profile from the tray menu
- [ ] Verify GUI updates the highlighted profile within ~500ms

---

### 2. Profile Saving

#### 2.1 Save New Profile from GUI
- [ ] Click "Save Current" button
- [ ] Enter a new profile name
- [ ] Click Save
- [ ] Verify profile appears in the list
- [ ] Verify profile appears in tray menu

#### 2.2 Overwrite Existing Profile from GUI
- [ ] Click "Save Current" button
- [ ] Select an existing profile from the list
- [ ] Click Save (should show "overwrite" indicator)
- [ ] Verify profile is updated (not duplicated)

#### 2.3 Save from Tray (Overwrite)
- [ ] Right-click tray icon > Save Profile > select existing profile
- [ ] Verify profile is overwritten with current config
- [ ] Open GUI and verify the profile details are updated

#### 2.4 Save New Profile from Tray
- [ ] Right-click tray icon > Save Profile > New Profile...
- [ ] Verify save popup opens
- [ ] Complete the save process

---

### 3. Profile Deletion

#### 3.1 Delete from GUI
- [ ] Hover over a profile card
- [ ] Click the delete (trash) button
- [ ] Verify profile is removed from list
- [ ] Verify profile is removed from tray menu

#### 3.2 Delete from Tray
- [ ] Right-click tray icon > Delete Profile > select a profile
- [ ] Verify profile is removed from tray menu
- [ ] Open GUI and verify profile is removed from list

#### 3.3 Delete while GUI is open (from Tray)
- [ ] Keep GUI window open
- [ ] Delete a profile from the tray menu
- [ ] Verify GUI updates and removes the profile

#### 3.4 Delete the Active Profile
- [ ] Load a profile so it's highlighted
- [ ] Delete that same profile
- [ ] Verify no profile is highlighted (since current config no longer matches any profile)

---

### 4. Manual Display Changes

#### 4.1 Change Resolution Manually
- [ ] Load a profile so it's highlighted
- [ ] Open Windows Display Settings
- [ ] Change the resolution of one monitor
- [ ] Return to Monitor Switcher GUI
- [ ] Verify the profile is no longer highlighted (config changed)

#### 4.2 Change Refresh Rate Manually
- [ ] Load a profile so it's highlighted
- [ ] Change refresh rate via Windows or GPU control panel
- [ ] Verify profile highlighting updates (may need to focus window)

#### 4.3 Change Monitor Arrangement Manually
- [ ] Load a profile with multiple monitors
- [ ] Drag monitors to different positions in Windows Display Settings
- [ ] Verify profile is no longer highlighted

#### 4.4 Disconnect a Monitor
- [ ] Load a multi-monitor profile
- [ ] Physically disconnect one monitor (or disable in settings)
- [ ] Verify profile is no longer highlighted
- [ ] Reconnect the monitor
- [ ] Load the profile again to restore configuration

---

### 5. Active Profile Detection

#### 5.1 Initial Load
- [ ] Start the app with a known display configuration
- [ ] Verify the matching profile (if any) is highlighted on startup

#### 5.2 No Matching Profile
- [ ] Manually change display settings to a configuration with no saved profile
- [ ] Verify no profile is highlighted

#### 5.3 Profile Matching Tolerance
- [ ] The app allows ~1Hz refresh rate tolerance
- [ ] Verify profiles match even with minor refresh rate variations (e.g., 59.94 vs 60)

#### 5.4 Window Focus Refresh
- [ ] Make manual display changes while GUI is minimized
- [ ] Focus the GUI window
- [ ] Verify active profile detection updates

---

### 6. Edge Cases

#### 6.1 Rapid Profile Switching
- [ ] Quickly click multiple profiles in succession
- [ ] Verify app doesn't crash or get stuck in loading state
- [ ] Verify final profile is correctly applied and highlighted

#### 6.2 Save and Load Same Profile
- [ ] Save current config as "Test"
- [ ] Load "Test" profile
- [ ] Verify it's highlighted (should match current config)

#### 6.3 Empty Profile List
- [ ] Delete all profiles
- [ ] Verify GUI shows "No profiles saved" message
- [ ] Verify tray shows "(No profiles)" in Load submenu
- [ ] Verify Delete submenu is disabled

#### 6.4 Long Profile Names
- [ ] Create a profile with a very long name
- [ ] Verify it displays properly (truncated) in GUI and tray

#### 6.5 Special Characters in Profile Names
- [ ] Try saving profiles with special characters: `Test & Profile`, `Profile #1`
- [ ] Verify invalid filename characters are handled (should be sanitized)

#### 6.6 Turn Off Monitors
- [ ] Click "Turn Off" button or use tray menu
- [ ] Verify monitors turn off
- [ ] Move mouse/press key to wake monitors
- [ ] Verify app still functions correctly

---

### 7. Multi-Instance Prevention

#### 7.1 Single Instance
- [ ] Launch the app
- [ ] Try to launch another instance
- [ ] Verify the existing window is focused instead of creating a new instance

#### 7.2 Window Hide/Show
- [ ] Close the main window (X button)
- [ ] Verify app continues running in tray
- [ ] Click tray icon to show window again
- [ ] Verify window state is preserved

---

## Known Behaviors

1. **500ms Refresh Delay**: After loading a profile, there's a 500ms delay before the GUI updates. This allows Windows time to apply display changes.

2. **Profile Matching**: Profiles are matched by comparing:
   - Resolution (width, height)
   - Position (x, y coordinates)
   - Rotation
   - Refresh rate (with ~1Hz tolerance)
   - Monitor names are NOT used for matching (they can change)

3. **Primary Monitor**: The monitor at position (0, 0) is considered primary and shows a green checkmark indicator.

4. **Tray Menu Updates**: The tray menu automatically rebuilds when profiles are added, deleted, or when the save popup closes.

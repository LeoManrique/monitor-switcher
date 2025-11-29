package profile

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"monitor-switcher-wails/internal/ccd"
	"monitor-switcher-wails/internal/config"
)

// Save saves the current display configuration to a profile file.
func Save(name string) error {
	// Ensure directories exist
	if err := config.EnsureDirectoriesExist(); err != nil {
		return fmt.Errorf("failed to create directories: %w", err)
	}

	// Get current display settings
	settings, err := ccd.GetCurrentDisplaySettings(true)
	if err != nil {
		return fmt.Errorf("failed to get display settings: %w", err)
	}

	// Convert to profile format
	profile := ConvertToProfile(settings)

	// Get profile path
	profilePath, err := config.GetProfilePath(name)
	if err != nil {
		return fmt.Errorf("failed to get profile path: %w", err)
	}

	// Serialize to JSON with indentation
	data, err := json.MarshalIndent(profile, "", "  ")
	if err != nil {
		return fmt.Errorf("failed to serialize profile: %w", err)
	}

	// Write to file
	if err := os.WriteFile(profilePath, data, 0644); err != nil {
		return fmt.Errorf("failed to write profile: %w", err)
	}

	return nil
}

// Load loads and applies a display configuration from a profile file.
func Load(name string) error {
	// Get profile path
	profilePath, err := config.GetProfilePath(name)
	if err != nil {
		return fmt.Errorf("failed to get profile path: %w", err)
	}

	// Read profile file
	data, err := os.ReadFile(profilePath)
	if err != nil {
		return fmt.Errorf("failed to read profile: %w", err)
	}

	// Deserialize from JSON
	var profile DisplayProfile
	if err := json.Unmarshal(data, &profile); err != nil {
		return fmt.Errorf("failed to parse profile: %w", err)
	}

	// Convert to CCD settings
	settings := ConvertFromProfile(&profile)

	// Get current display settings for adapter ID matching
	current, err := ccd.GetCurrentDisplaySettings(false)
	if err != nil {
		return fmt.Errorf("failed to get current display settings: %w", err)
	}

	// Keep original settings for retry attempts
	originalSettings := cloneSettings(settings)

	// Tier 1: Match adapter IDs by path IDs
	MatchAdapterIDs(settings, current)

	// Try to apply settings
	err = ccd.ApplyDisplaySettings(settings)
	if err == nil {
		return nil
	}

	// Tier 2: Try matching by monitor name
	if len(current.AdditionalInfo) > 0 && len(profile.AdditionalInfo) > 0 {
		settings = cloneSettings(originalSettings)
		MatchByMonitorName(settings, current)

		err = ccd.ApplyDisplaySettings(settings)
		if err == nil {
			return nil
		}
	}

	// Tier 3: Try bulk replacement
	settings = cloneSettings(originalSettings)
	MatchByBulkReplacement(settings, current)

	err = ccd.ApplyDisplaySettings(settings)
	if err != nil {
		return fmt.Errorf("failed to apply display settings after all matching attempts: %w", err)
	}

	return nil
}

// Delete removes a profile file.
func Delete(name string) error {
	profilePath, err := config.GetProfilePath(name)
	if err != nil {
		return fmt.Errorf("failed to get profile path: %w", err)
	}

	if err := os.Remove(profilePath); err != nil {
		return fmt.Errorf("failed to delete profile: %w", err)
	}

	return nil
}

// List returns a list of all available profile names.
func List() ([]string, error) {
	profilesDir, err := config.GetProfilesDirectory()
	if err != nil {
		return nil, fmt.Errorf("failed to get profiles directory: %w", err)
	}

	// Ensure directory exists
	if err := config.EnsureDirectoriesExist(); err != nil {
		return nil, fmt.Errorf("failed to create directories: %w", err)
	}

	entries, err := os.ReadDir(profilesDir)
	if err != nil {
		return nil, fmt.Errorf("failed to read profiles directory: %w", err)
	}

	var profiles []string
	for _, entry := range entries {
		if !entry.IsDir() && strings.HasSuffix(entry.Name(), config.ProfileExtension) {
			name := strings.TrimSuffix(entry.Name(), config.ProfileExtension)
			profiles = append(profiles, name)
		}
	}

	return profiles, nil
}

// Exists checks if a profile with the given name exists.
func Exists(name string) (bool, error) {
	profilePath, err := config.GetProfilePath(name)
	if err != nil {
		return false, err
	}

	_, err = os.Stat(profilePath)
	if os.IsNotExist(err) {
		return false, nil
	}
	if err != nil {
		return false, err
	}
	return true, nil
}

// GetProfilePath returns the full path to a profile file.
func GetProfilePath(name string) (string, error) {
	return config.GetProfilePath(name)
}

// GetProfilesDirectory returns the directory where profiles are stored.
func GetProfilesDirectory() (string, error) {
	return config.GetProfilesDirectory()
}

// ValidateName checks if a profile name is valid for use as a filename.
func ValidateName(name string) error {
	if name == "" {
		return fmt.Errorf("profile name cannot be empty")
	}

	// Check for invalid filename characters
	invalidChars := []string{"\\", "/", ":", "*", "?", "\"", "<", ">", "|"}
	for _, char := range invalidChars {
		if strings.Contains(name, char) {
			return fmt.Errorf("profile name cannot contain '%s'", char)
		}
	}

	// Check for reserved names
	reserved := []string{"CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8", "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9"}
	upperName := strings.ToUpper(name)
	for _, r := range reserved {
		if upperName == r || strings.HasPrefix(upperName, r+".") {
			return fmt.Errorf("'%s' is a reserved name", name)
		}
	}

	return nil
}

// SanitizeName removes invalid characters from a profile name.
func SanitizeName(name string) string {
	invalidChars := []string{"\\", "/", ":", "*", "?", "\"", "<", ">", "|"}
	result := name
	for _, char := range invalidChars {
		result = strings.ReplaceAll(result, char, "")
	}
	return strings.TrimSpace(result)
}

// cloneSettings creates a deep copy of DisplaySettings.
func cloneSettings(settings *ccd.DisplaySettings) *ccd.DisplaySettings {
	clone := &ccd.DisplaySettings{
		PathInfoArray:  make([]ccd.DisplayConfigPathInfo, len(settings.PathInfoArray)),
		ModeInfoArray:  make([]ccd.DisplayConfigModeInfo, len(settings.ModeInfoArray)),
		AdditionalInfo: make([]ccd.MonitorInfo, len(settings.AdditionalInfo)),
	}
	copy(clone.PathInfoArray, settings.PathInfoArray)
	copy(clone.ModeInfoArray, settings.ModeInfoArray)
	copy(clone.AdditionalInfo, settings.AdditionalInfo)
	return clone
}

// getProfileDir returns the absolute path to the profiles directory.
func getProfileDir() (string, error) {
	profilesDir, err := config.GetProfilesDirectory()
	if err != nil {
		return "", err
	}
	return filepath.Abs(profilesDir)
}

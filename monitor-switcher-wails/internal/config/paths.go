// Package config provides application configuration and paths.
package config

import (
	"os"
	"path/filepath"
)

const (
	// AppName is the name of the application folder in AppData.
	AppName = "MonitorSwitcher"
	// ProfilesFolder is the subfolder for profile storage.
	ProfilesFolder = "Profiles"
	// ProfileExtension is the file extension for profile files.
	ProfileExtension = ".json"
)

// GetSettingsDirectory returns the path to the settings directory.
// Uses the existing MonitorSwitcher location for compatibility.
func GetSettingsDirectory() (string, error) {
	appData, err := os.UserConfigDir()
	if err != nil {
		return "", err
	}
	return filepath.Join(appData, AppName), nil
}

// GetProfilesDirectory returns the path to the profiles directory.
func GetProfilesDirectory() (string, error) {
	settingsDir, err := GetSettingsDirectory()
	if err != nil {
		return "", err
	}
	return filepath.Join(settingsDir, ProfilesFolder), nil
}

// EnsureDirectoriesExist creates the settings and profiles directories if they don't exist.
func EnsureDirectoriesExist() error {
	profilesDir, err := GetProfilesDirectory()
	if err != nil {
		return err
	}
	return os.MkdirAll(profilesDir, 0755)
}

// GetProfilePath returns the full path for a profile with the given name.
func GetProfilePath(name string) (string, error) {
	profilesDir, err := GetProfilesDirectory()
	if err != nil {
		return "", err
	}
	return filepath.Join(profilesDir, name+ProfileExtension), nil
}

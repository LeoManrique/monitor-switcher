package main

import (
	"context"
	"fmt"

	"monitor-switcher-wails/internal/power"
	"monitor-switcher-wails/internal/profile"
)

// App struct represents the application.
type App struct {
	ctx context.Context
}

// NewApp creates a new App application struct.
func NewApp() *App {
	return &App{}
}

// startup is called when the app starts.
func (a *App) startup(ctx context.Context) {
	a.ctx = ctx
}

// ListProfiles returns a list of all available profile names.
func (a *App) ListProfiles() ([]string, error) {
	profiles, err := profile.List()
	if err != nil {
		return nil, fmt.Errorf("failed to list profiles: %w", err)
	}
	return profiles, nil
}

// SaveProfile saves the current display configuration to a profile.
func (a *App) SaveProfile(name string) error {
	// Validate and sanitize name
	name = profile.SanitizeName(name)
	if err := profile.ValidateName(name); err != nil {
		return err
	}

	if err := profile.Save(name); err != nil {
		return fmt.Errorf("failed to save profile: %w", err)
	}
	return nil
}

// LoadProfile loads and applies a display configuration from a profile.
func (a *App) LoadProfile(name string) error {
	if err := profile.Load(name); err != nil {
		return fmt.Errorf("failed to load profile: %w", err)
	}
	return nil
}

// DeleteProfile removes a profile.
func (a *App) DeleteProfile(name string) error {
	if err := profile.Delete(name); err != nil {
		return fmt.Errorf("failed to delete profile: %w", err)
	}
	return nil
}

// ProfileExists checks if a profile with the given name exists.
func (a *App) ProfileExists(name string) (bool, error) {
	return profile.Exists(name)
}

// TurnOffMonitors turns off all monitors.
func (a *App) TurnOffMonitors() error {
	return power.TurnOffMonitors()
}

// GetProfilesDirectory returns the path to the profiles directory.
func (a *App) GetProfilesDirectory() (string, error) {
	return profile.GetProfilesDirectory()
}

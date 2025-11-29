package main

import (
	"context"
	"embed"

	"monitor-switcher-wails/internal/power"
	"monitor-switcher-wails/internal/profile"
	"monitor-switcher-wails/internal/tray"

	"github.com/wailsapp/wails/v2"
	"github.com/wailsapp/wails/v2/pkg/options"
	"github.com/wailsapp/wails/v2/pkg/options/assetserver"
	"github.com/wailsapp/wails/v2/pkg/options/windows"
	wailsRuntime "github.com/wailsapp/wails/v2/pkg/runtime"
)

//go:embed all:frontend/dist
var assets embed.FS

func main() {
	// Create an instance of the app structure
	app := NewApp()

	// Start system tray in a goroutine
	go tray.Run(tray.Callbacks{
		OnShow: func() {
			wailsRuntime.WindowShow(app.ctx)
			wailsRuntime.WindowSetAlwaysOnTop(app.ctx, true)
			wailsRuntime.WindowSetAlwaysOnTop(app.ctx, false)
		},
		OnLoadProfile: func(name string) {
			go func() {
				_ = profile.Load(name)
			}()
		},
		OnSaveNewProfile: func() {
			// Show the window so user can enter profile name
			wailsRuntime.WindowShow(app.ctx)
			wailsRuntime.WindowSetAlwaysOnTop(app.ctx, true)
			wailsRuntime.WindowSetAlwaysOnTop(app.ctx, false)
		},
		OnSaveToProfile: func(name string) {
			go func() {
				_ = profile.Save(name)
				tray.RefreshProfiles()
			}()
		},
		OnTurnOff: func() {
			go func() {
				_ = power.TurnOffMonitors()
			}()
		},
		OnQuit: func() {
			wailsRuntime.Quit(app.ctx)
		},
		GetProfiles: func() []string {
			profiles, _ := profile.List()
			return profiles
		},
	}, nil)

	// Create application with options
	err := wails.Run(&options.App{
		Title:     "Monitor Switcher",
		Width:     480,
		Height:    400,
		MinWidth:  400,
		MinHeight: 300,
		AssetServer: &assetserver.Options{
			Assets: assets,
		},
		BackgroundColour: &options.RGBA{R: 27, G: 38, B: 54, A: 1},
		OnStartup:        app.startup,
		OnShutdown: func(_ context.Context) {
			tray.Quit()
		},
		Bind: []interface{}{
			app,
		},
		Windows: &windows.Options{
			WebviewIsTransparent: false,
			WindowIsTranslucent:  false,
			DisableWindowIcon:    false,
		},
		// Hide to tray when closed instead of quitting
		HideWindowOnClose: true,
	})

	if err != nil {
		println("Error:", err.Error())
	}
}

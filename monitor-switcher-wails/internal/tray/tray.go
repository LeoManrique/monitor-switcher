// Package tray handles the system tray icon and menu.
package tray

import (
	"github.com/getlantern/systray"
)

// Callbacks contains the callback functions for tray menu actions.
type Callbacks struct {
	OnShow           func()
	OnLoadProfile    func(name string)
	OnSaveNewProfile func()
	OnSaveToProfile  func(name string)
	OnTurnOff        func()
	OnQuit           func()
	GetProfiles      func() []string
}

var callbacks Callbacks
var loadProfileItems []*systray.MenuItem
var saveProfileItems []*systray.MenuItem

// Run starts the system tray. This should be called from the main goroutine.
func Run(cb Callbacks, onReady func()) {
	callbacks = cb
	systray.Run(func() {
		onReadyHandler()
		if onReady != nil {
			onReady()
		}
	}, onExit)
}

// Quit exits the system tray.
func Quit() {
	systray.Quit()
}

// RefreshProfiles updates the profile menu items for both Load and Save submenus.
func RefreshProfiles() {
	profiles := callbacks.GetProfiles()

	// Update Load Profile items
	for _, item := range loadProfileItems {
		item.Hide()
	}
	for i, name := range profiles {
		if i >= len(loadProfileItems) {
			break
		}
		loadProfileItems[i].SetTitle(name)
		loadProfileItems[i].Show()
	}

	// Update Save Profile items
	for _, item := range saveProfileItems {
		item.Hide()
	}
	for i, name := range profiles {
		if i >= len(saveProfileItems) {
			break
		}
		saveProfileItems[i].SetTitle(name)
		saveProfileItems[i].Show()
	}
}

func onReadyHandler() {
	systray.SetIcon(iconTray)
	systray.SetTooltip("Monitor Profile Switcher")

	// --- Load Profile submenu ---
	mLoad := systray.AddMenuItem("Load Profile", "Load a saved display profile")
	mLoad.SetIcon(iconMonitor)

	// Pre-allocate menu items for load profiles (up to 20)
	loadProfileItems = make([]*systray.MenuItem, 20)
	for i := 0; i < 20; i++ {
		loadProfileItems[i] = mLoad.AddSubMenuItem("", "Load this profile")
		loadProfileItems[i].Hide()
	}

	// --- Save Profile submenu ---
	mSave := systray.AddMenuItem("Save Profile", "Save current display configuration")
	mSave.SetIcon(iconSave)

	// "New Profile..." option
	mSaveNew := mSave.AddSubMenuItem("New Profile...", "Save as a new profile")
	mSaveNew.SetIcon(iconNew)

	// Separator in submenu (add a disabled item as visual separator)
	mSaveSep := mSave.AddSubMenuItem("────────────", "")
	mSaveSep.Disable()

	// Pre-allocate menu items for save-to-existing profiles (up to 20)
	saveProfileItems = make([]*systray.MenuItem, 20)
	for i := 0; i < 20; i++ {
		saveProfileItems[i] = mSave.AddSubMenuItem("", "Overwrite this profile")
		saveProfileItems[i].Hide()
	}

	// Initial profile refresh
	RefreshProfiles()

	systray.AddSeparator()

	mTurnOff := systray.AddMenuItem("Turn Off All Monitors", "Turn off all monitors")
	mTurnOff.SetIcon(iconPower)

	systray.AddSeparator()

	// --- App controls ---
	mShow := systray.AddMenuItem("Open Window", "Open the Monitor Switcher window")
	mShow.SetIcon(iconWindow)

	mQuit := systray.AddMenuItem("Exit", "Exit Monitor Switcher")
	mQuit.SetIcon(iconExit)

	// Handle main menu clicks
	go func() {
		for {
			select {
			case <-mShow.ClickedCh:
				if callbacks.OnShow != nil {
					callbacks.OnShow()
				}
			case <-mSaveNew.ClickedCh:
				if callbacks.OnSaveNewProfile != nil {
					callbacks.OnSaveNewProfile()
				}
			case <-mTurnOff.ClickedCh:
				if callbacks.OnTurnOff != nil {
					callbacks.OnTurnOff()
				}
			case <-mQuit.ClickedCh:
				if callbacks.OnQuit != nil {
					callbacks.OnQuit()
				}
				systray.Quit()
				return
			}
		}
	}()

	// Handle Load Profile submenu clicks
	for i := range loadProfileItems {
		idx := i
		go func() {
			for range loadProfileItems[idx].ClickedCh {
				profiles := callbacks.GetProfiles()
				if idx < len(profiles) && callbacks.OnLoadProfile != nil {
					callbacks.OnLoadProfile(profiles[idx])
				}
			}
		}()
	}

	// Handle Save-to-existing Profile submenu clicks
	for i := range saveProfileItems {
		idx := i
		go func() {
			for range saveProfileItems[idx].ClickedCh {
				profiles := callbacks.GetProfiles()
				if idx < len(profiles) && callbacks.OnSaveToProfile != nil {
					callbacks.OnSaveToProfile(profiles[idx])
				}
			}
		}()
	}
}

func onExit() {
	// Cleanup if needed
}

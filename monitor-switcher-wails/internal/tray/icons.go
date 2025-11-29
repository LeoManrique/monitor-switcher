package tray

import (
	_ "embed"
)

// Icons for the system tray menu.
// These are 16x16 ICO icons loaded from files.

//go:embed icons/tray.ico
var iconTray []byte

//go:embed icons/monitor.ico
var iconMonitor []byte

//go:embed icons/save.ico
var iconSave []byte

//go:embed icons/new.ico
var iconNew []byte

//go:embed icons/power.ico
var iconPower []byte

//go:embed icons/window.ico
var iconWindow []byte

//go:embed icons/exit.ico
var iconExit []byte

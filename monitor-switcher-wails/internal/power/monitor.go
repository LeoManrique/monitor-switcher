// Package power provides monitor power control functionality.
package power

import (
	"time"

	"golang.org/x/sys/windows"
)

var (
	user32      = windows.NewLazySystemDLL("user32.dll")
	postMessage = user32.NewProc("PostMessageW")
)

const (
	hwndBroadcast   = 0xFFFF
	wmSysCommand    = 0x0112
	scMonitorPower  = 0xF170
	monitorOff      = 2
	monitorOn       = -1
	monitorStandby  = 1
)

// TurnOffMonitors turns off all monitors.
// A short delay is added to give the user time to release the mouse.
func TurnOffMonitors() error {
	// Wait a moment to let the user release the mouse/keyboard
	time.Sleep(500 * time.Millisecond)

	_, _, err := postMessage.Call(
		uintptr(hwndBroadcast),
		uintptr(wmSysCommand),
		uintptr(scMonitorPower),
		uintptr(monitorOff),
	)

	// PostMessage returns non-zero on success, but the error is always set
	// We ignore the error since it's not meaningful for PostMessage
	_ = err

	return nil
}

// TurnOnMonitors turns on all monitors.
func TurnOnMonitors() error {
	// monitorOn is -1, need to use ^uintptr(0) for max value representation
	_, _, err := postMessage.Call(
		uintptr(hwndBroadcast),
		uintptr(wmSysCommand),
		uintptr(scMonitorPower),
		^uintptr(0), // -1 as uintptr
	)
	_ = err
	return nil
}

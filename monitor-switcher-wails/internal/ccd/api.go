package ccd

import (
	"syscall"
	"unsafe"

	"golang.org/x/sys/windows"
)

var (
	user32                          = windows.NewLazySystemDLL("user32.dll")
	procGetDisplayConfigBufferSizes = user32.NewProc("GetDisplayConfigBufferSizes")
	procQueryDisplayConfig          = user32.NewProc("QueryDisplayConfig")
	procSetDisplayConfig            = user32.NewProc("SetDisplayConfig")
	procDisplayConfigGetDeviceInfo  = user32.NewProc("DisplayConfigGetDeviceInfo")
)

// GetDisplayConfigBufferSizes retrieves the size of buffers needed for QueryDisplayConfig.
func GetDisplayConfigBufferSizes(flags uint32) (numPathElements, numModeElements uint32, err error) {
	ret, _, _ := procGetDisplayConfigBufferSizes.Call(
		uintptr(flags),
		uintptr(unsafe.Pointer(&numPathElements)),
		uintptr(unsafe.Pointer(&numModeElements)),
	)
	if ret != 0 {
		return 0, 0, syscall.Errno(ret)
	}
	return numPathElements, numModeElements, nil
}

// QueryDisplayConfig retrieves information about all display paths.
func QueryDisplayConfig(flags uint32, pathInfoArray []DisplayConfigPathInfo, modeInfoArray []DisplayConfigModeInfo) (uint32, uint32, error) {
	numPaths := uint32(len(pathInfoArray))
	numModes := uint32(len(modeInfoArray))

	var pathPtr, modePtr unsafe.Pointer
	if len(pathInfoArray) > 0 {
		pathPtr = unsafe.Pointer(&pathInfoArray[0])
	}
	if len(modeInfoArray) > 0 {
		modePtr = unsafe.Pointer(&modeInfoArray[0])
	}

	ret, _, _ := procQueryDisplayConfig.Call(
		uintptr(flags),
		uintptr(unsafe.Pointer(&numPaths)),
		uintptr(pathPtr),
		uintptr(unsafe.Pointer(&numModes)),
		uintptr(modePtr),
		0, // currentTopologyId - not used
	)
	if ret != 0 {
		return 0, 0, syscall.Errno(ret)
	}
	return numPaths, numModes, nil
}

// SetDisplayConfig applies a display configuration.
func SetDisplayConfig(pathInfoArray []DisplayConfigPathInfo, modeInfoArray []DisplayConfigModeInfo, flags uint32) error {
	numPaths := uint32(len(pathInfoArray))
	numModes := uint32(len(modeInfoArray))

	var pathPtr, modePtr unsafe.Pointer
	if len(pathInfoArray) > 0 {
		pathPtr = unsafe.Pointer(&pathInfoArray[0])
	}
	if len(modeInfoArray) > 0 {
		modePtr = unsafe.Pointer(&modeInfoArray[0])
	}

	ret, _, _ := procSetDisplayConfig.Call(
		uintptr(numPaths),
		uintptr(pathPtr),
		uintptr(numModes),
		uintptr(modePtr),
		uintptr(flags),
	)
	if ret != 0 {
		return syscall.Errno(ret)
	}
	return nil
}

// DisplayConfigGetDeviceInfo retrieves device information for a display target.
func DisplayConfigGetDeviceInfo(deviceName *DisplayConfigTargetDeviceName) error {
	ret, _, _ := procDisplayConfigGetDeviceInfo.Call(
		uintptr(unsafe.Pointer(deviceName)),
	)
	if ret != 0 {
		return syscall.Errno(ret)
	}
	return nil
}

package ccd

import (
	"fmt"
	"unsafe"
)

// MonitorInfo contains additional information about a monitor.
type MonitorInfo struct {
	ManufactureId         uint16
	ProductCodeId         uint16
	Valid                 bool
	MonitorDevicePath     string
	MonitorFriendlyDevice string
}

// DisplaySettings holds the complete display configuration.
type DisplaySettings struct {
	PathInfoArray  []DisplayConfigPathInfo
	ModeInfoArray  []DisplayConfigModeInfo
	AdditionalInfo []MonitorInfo
}

// GetCurrentDisplaySettings queries the current display configuration.
// If activeOnly is true, only returns active display paths.
func GetCurrentDisplaySettings(activeOnly bool) (*DisplaySettings, error) {
	flags := QueryDisplayFlagsAllPaths
	if activeOnly {
		flags = QueryDisplayFlagsOnlyActivePaths
	}

	// Get buffer sizes
	numPaths, numModes, err := GetDisplayConfigBufferSizes(flags)
	if err != nil {
		return nil, fmt.Errorf("GetDisplayConfigBufferSizes failed: %w", err)
	}

	// Allocate buffers
	pathInfoArray := make([]DisplayConfigPathInfo, numPaths)
	modeInfoArray := make([]DisplayConfigModeInfo, numModes)

	// Query display config
	numPaths, numModes, err = QueryDisplayConfig(flags, pathInfoArray, modeInfoArray)
	if err != nil {
		return nil, fmt.Errorf("QueryDisplayConfig failed: %w", err)
	}

	// Trim to actual size
	pathInfoArray = pathInfoArray[:numPaths]
	modeInfoArray = modeInfoArray[:numModes]

	// Clean up mode info - remove zero entries
	validModes := make([]DisplayConfigModeInfo, 0, len(modeInfoArray))
	for _, mode := range modeInfoArray {
		if mode.InfoType != 0 {
			validModes = append(validModes, mode)
		}
	}
	modeInfoArray = validModes

	// Clean up path info - only keep available targets
	validPaths := make([]DisplayConfigPathInfo, 0, len(pathInfoArray))
	for _, path := range pathInfoArray {
		if path.TargetInfo.TargetAvailable != 0 {
			validPaths = append(validPaths, path)
		}
	}
	pathInfoArray = validPaths

	// Get additional monitor info for target modes
	additionalInfo := make([]MonitorInfo, len(modeInfoArray))
	for i, mode := range modeInfoArray {
		if mode.InfoType == ModeInfoTypeTarget {
			info, err := getMonitorAdditionalInfo(mode.AdapterId, mode.Id)
			if err != nil {
				additionalInfo[i] = MonitorInfo{Valid: false}
			} else {
				additionalInfo[i] = info
			}
		}
	}

	return &DisplaySettings{
		PathInfoArray:  pathInfoArray,
		ModeInfoArray:  modeInfoArray,
		AdditionalInfo: additionalInfo,
	}, nil
}

// ApplyDisplaySettings applies the given display configuration.
func ApplyDisplaySettings(settings *DisplaySettings) error {
	flags := SdcFlagsApply | SdcFlagsUseSuppliedDisplayConfig | SdcFlagsSaveToDatabase | SdcFlagsNoOptimization

	// First attempt without AllowChanges
	err := SetDisplayConfig(settings.PathInfoArray, settings.ModeInfoArray, flags)
	if err == nil {
		return nil
	}

	// Retry with AllowChanges flag
	flags |= SdcFlagsAllowChanges
	err = SetDisplayConfig(settings.PathInfoArray, settings.ModeInfoArray, flags)
	if err != nil {
		return fmt.Errorf("SetDisplayConfig failed: %w", err)
	}

	return nil
}

// getMonitorAdditionalInfo retrieves additional information for a monitor.
func getMonitorAdditionalInfo(adapterId LUID, targetId uint32) (MonitorInfo, error) {
	deviceName := DisplayConfigTargetDeviceName{
		Header: DisplayConfigDeviceInfoHeader{
			InfoType:  DeviceInfoTypeGetTargetName,
			Size:      uint32(unsafe.Sizeof(DisplayConfigTargetDeviceName{})),
			AdapterId: adapterId,
			Id:        targetId,
		},
	}

	err := DisplayConfigGetDeviceInfo(&deviceName)
	if err != nil {
		return MonitorInfo{}, err
	}

	return MonitorInfo{
		ManufactureId:         deviceName.EdidManufactureId,
		ProductCodeId:         deviceName.EdidProductCodeId,
		Valid:                 true,
		MonitorDevicePath:     utf16ToString(deviceName.MonitorDevicePath[:]),
		MonitorFriendlyDevice: utf16ToString(deviceName.MonitorFriendlyDeviceName[:]),
	}, nil
}

// utf16ToString converts a null-terminated UTF-16 slice to a Go string.
func utf16ToString(s []uint16) string {
	for i, v := range s {
		if v == 0 {
			return string(utf16Decode(s[:i]))
		}
	}
	return string(utf16Decode(s))
}

// utf16Decode decodes a UTF-16 slice to a rune slice.
func utf16Decode(s []uint16) []rune {
	runes := make([]rune, 0, len(s))
	for i := 0; i < len(s); i++ {
		r := rune(s[i])
		if r >= 0xD800 && r <= 0xDBFF && i+1 < len(s) {
			r2 := rune(s[i+1])
			if r2 >= 0xDC00 && r2 <= 0xDFFF {
				r = 0x10000 + ((r-0xD800)<<10 | (r2 - 0xDC00))
				i++
			}
		}
		runes = append(runes, r)
	}
	return runes
}

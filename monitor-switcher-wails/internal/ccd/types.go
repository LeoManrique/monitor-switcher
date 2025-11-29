// Package ccd provides Windows CCD (Connecting and Configuring Displays) API bindings.
package ccd

// LUID represents a locally unique identifier for display adapters.
// Note: The adapter ID changes on system restart, so matching must be done by other fields.
type LUID struct {
	LowPart  uint32
	HighPart uint32
}

// DisplayConfigRational represents a rational number (used for refresh rates, frequencies).
type DisplayConfigRational struct {
	Numerator   uint32
	Denominator uint32
}

// DisplayConfig2DRegion represents a 2D region size.
type DisplayConfig2DRegion struct {
	Cx uint32
	Cy uint32
}

// PointL represents a point with x,y coordinates.
type PointL struct {
	X int32
	Y int32
}

// DisplayConfigPathSourceInfo contains source information for a display path.
// Size: 20 bytes (8 + 4 + 4 + 4)
type DisplayConfigPathSourceInfo struct {
	AdapterId   LUID
	Id          uint32
	ModeInfoIdx uint32
	StatusFlags uint32
}

// DisplayConfigPathTargetInfo contains target information for a display path.
// Size: 48 bytes
type DisplayConfigPathTargetInfo struct {
	AdapterId        LUID   // 8 bytes
	Id               uint32 // 4 bytes
	ModeInfoIdx      uint32 // 4 bytes
	OutputTechnology uint32 // 4 bytes
	Rotation         uint32 // 4 bytes
	Scaling          uint32 // 4 bytes
	RefreshRate      DisplayConfigRational // 8 bytes
	ScanLineOrdering uint32 // 4 bytes
	TargetAvailable  uint32 // 4 bytes (BOOL)
	StatusFlags      uint32 // 4 bytes
}

// DisplayConfigPathInfo represents a display path connecting a source to a target.
// Size: 72 bytes (20 + 48 + 4)
type DisplayConfigPathInfo struct {
	SourceInfo DisplayConfigPathSourceInfo
	TargetInfo DisplayConfigPathTargetInfo
	Flags      uint32
}

// DisplayConfigVideoSignalInfo contains video signal timing information.
// Size: 40 bytes
type DisplayConfigVideoSignalInfo struct {
	PixelRate        uint64                // 8 bytes
	HSyncFreq        DisplayConfigRational // 8 bytes
	VSyncFreq        DisplayConfigRational // 8 bytes
	ActiveSize       DisplayConfig2DRegion // 8 bytes
	TotalSize        DisplayConfig2DRegion // 8 bytes
	VideoStandard    uint32                // 4 bytes - actually part of union with scanLineOrdering
	ScanLineOrdering uint32                // 4 bytes - but we need padding to make total 48
}

// DisplayConfigTargetMode contains target mode information.
// Size: 48 bytes (includes padding)
type DisplayConfigTargetMode struct {
	TargetVideoSignalInfo DisplayConfigVideoSignalInfo
}

// DisplayConfigSourceMode contains source mode information.
// Size: 20 bytes
type DisplayConfigSourceMode struct {
	Width       uint32
	Height      uint32
	PixelFormat uint32
	Position    PointL
}

// DisplayConfigDesktopImageInfo for desktop image mode
type DisplayConfigDesktopImageInfo struct {
	PathSourceSize DisplayConfig2DRegion
	DesktopImageRegion struct {
		Left   int32
		Top    int32
		Right  int32
		Bottom int32
	}
	DesktopImageClip struct {
		Left   int32
		Top    int32
		Right  int32
		Bottom int32
	}
}

// DisplayConfigModeInfo represents mode information for a display.
// This is a union in C - either TargetMode or SourceMode is valid based on InfoType.
// Total size: 64 bytes
type DisplayConfigModeInfo struct {
	InfoType  uint32 // 4 bytes
	Id        uint32 // 4 bytes
	AdapterId LUID   // 8 bytes
	// Union: 48 bytes (size of largest member - targetMode with padding)
	ModeData [48]byte
}

// DisplayConfigDeviceInfoHeader is the header for device info requests.
type DisplayConfigDeviceInfoHeader struct {
	InfoType  uint32
	Size      uint32
	AdapterId LUID
	Id        uint32
}

// DisplayConfigTargetDeviceName contains the device name and path for a target.
type DisplayConfigTargetDeviceName struct {
	Header                    DisplayConfigDeviceInfoHeader
	Flags                     uint32
	OutputTechnology          uint32
	EdidManufactureId         uint16
	EdidProductCodeId         uint16
	ConnectorInstance         uint32
	MonitorFriendlyDeviceName [64]uint16
	MonitorDevicePath         [128]uint16
}

// Constants for display configuration.
const (
	// Query flags
	QueryDisplayFlagsAllPaths        uint32 = 0x00000001
	QueryDisplayFlagsOnlyActivePaths uint32 = 0x00000002

	// SDC (Set Display Config) flags
	SdcFlagsTopologyInternal         uint32 = 0x00000001
	SdcFlagsTopologyClone            uint32 = 0x00000002
	SdcFlagsTopologyExtend           uint32 = 0x00000004
	SdcFlagsTopologyExternal         uint32 = 0x00000008
	SdcFlagsTopologySupplied         uint32 = 0x00000010
	SdcFlagsUseSuppliedDisplayConfig uint32 = 0x00000020
	SdcFlagsValidate                 uint32 = 0x00000040
	SdcFlagsApply                    uint32 = 0x00000080
	SdcFlagsNoOptimization           uint32 = 0x00000100
	SdcFlagsSaveToDatabase           uint32 = 0x00000200
	SdcFlagsAllowChanges             uint32 = 0x00000400
	SdcFlagsPathPersistIfRequired    uint32 = 0x00000800
	SdcFlagsForceModeEnumeration     uint32 = 0x00001000
	SdcFlagsAllowPathOrderChanges    uint32 = 0x00002000

	// Mode info types
	ModeInfoTypeSource uint32 = 1
	ModeInfoTypeTarget uint32 = 2

	// Device info types
	DeviceInfoTypeGetTargetName uint32 = 2

	// Status codes
	ErrorSuccess uint32 = 0
)

// GetTargetMode interprets the ModeData as a target mode.
// Only valid when InfoType == ModeInfoTypeTarget.
func (m *DisplayConfigModeInfo) GetTargetMode() *DisplayConfigTargetMode {
	return (*DisplayConfigTargetMode)(unsafePointer(&m.ModeData[0]))
}

// GetSourceMode interprets the ModeData as a source mode.
// Only valid when InfoType == ModeInfoTypeSource.
func (m *DisplayConfigModeInfo) GetSourceMode() *DisplayConfigSourceMode {
	return (*DisplayConfigSourceMode)(unsafePointer(&m.ModeData[0]))
}

// SetTargetMode sets the ModeData from a target mode.
func (m *DisplayConfigModeInfo) SetTargetMode(tm *DisplayConfigTargetMode) {
	copy(m.ModeData[:], (*[48]byte)(unsafePointer(tm))[:])
}

// SetSourceMode sets the ModeData from a source mode.
func (m *DisplayConfigModeInfo) SetSourceMode(sm *DisplayConfigSourceMode) {
	// Clear first, then copy (source mode is smaller)
	for i := range m.ModeData {
		m.ModeData[i] = 0
	}
	copy(m.ModeData[:20], (*[20]byte)(unsafePointer(sm))[:])
}

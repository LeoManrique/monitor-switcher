// Package profile handles saving and loading display profile configurations.
package profile

// DisplayProfile is the root object for display profile JSON serialization.
// Field names use PascalCase to match the existing C# JSON format.
type DisplayProfile struct {
	Version        int           `json:"Version"`
	PathInfoArray  []PathInfo    `json:"PathInfoArray"`
	ModeInfoArray  []ModeInfo    `json:"ModeInfoArray"`
	AdditionalInfo []MonitorInfo `json:"AdditionalInfo"`
}

// PathInfo represents a display path in the profile.
type PathInfo struct {
	SourceInfo PathSourceInfo `json:"SourceInfo"`
	TargetInfo PathTargetInfo `json:"TargetInfo"`
	Flags      uint32         `json:"Flags"`
}

// PathSourceInfo contains source information for a path.
type PathSourceInfo struct {
	AdapterId   AdapterId `json:"AdapterId"`
	Id          uint32    `json:"Id"`
	ModeInfoIdx uint32    `json:"ModeInfoIdx"`
	StatusFlags uint32    `json:"StatusFlags"`
}

// PathTargetInfo contains target information for a path.
type PathTargetInfo struct {
	AdapterId        AdapterId `json:"AdapterId"`
	Id               uint32    `json:"Id"`
	ModeInfoIdx      uint32    `json:"ModeInfoIdx"`
	OutputTechnology uint32    `json:"OutputTechnology"`
	Rotation         uint32    `json:"Rotation"`
	Scaling          uint32    `json:"Scaling"`
	RefreshRate      Rational  `json:"RefreshRate"`
	ScanLineOrdering uint32    `json:"ScanLineOrdering"`
	TargetAvailable  bool      `json:"TargetAvailable"`
	StatusFlags      uint32    `json:"StatusFlags"`
}

// AdapterId represents a display adapter identifier.
type AdapterId struct {
	LowPart  uint32 `json:"LowPart"`
	HighPart uint32 `json:"HighPart"`
}

// Rational represents a rational number.
type Rational struct {
	Numerator   uint32 `json:"Numerator"`
	Denominator uint32 `json:"Denominator"`
}

// ModeInfo represents mode information in the profile.
type ModeInfo struct {
	InfoType   uint32      `json:"InfoType"`
	Id         uint32      `json:"Id"`
	AdapterId  AdapterId   `json:"AdapterId"`
	TargetMode *TargetMode `json:"TargetMode,omitempty"`
	SourceMode *SourceMode `json:"SourceMode,omitempty"`
}

// TargetMode contains target mode details.
type TargetMode struct {
	TargetVideoSignalInfo VideoSignalInfo `json:"TargetVideoSignalInfo"`
}

// VideoSignalInfo contains video signal timing information.
type VideoSignalInfo struct {
	PixelRate        int64    `json:"PixelRate"`
	HSyncFreq        Rational `json:"HSyncFreq"`
	VSyncFreq        Rational `json:"VSyncFreq"`
	ActiveSize       Region2D `json:"ActiveSize"`
	TotalSize        Region2D `json:"TotalSize"`
	VideoStandard    uint32   `json:"VideoStandard"`
	ScanLineOrdering uint32   `json:"ScanLineOrdering"`
}

// Region2D represents a 2D region size.
type Region2D struct {
	Cx uint32 `json:"Cx"`
	Cy uint32 `json:"Cy"`
}

// SourceMode contains source mode details.
type SourceMode struct {
	Width       uint32 `json:"Width"`
	Height      uint32 `json:"Height"`
	PixelFormat uint32 `json:"PixelFormat"`
	Position    Point  `json:"Position"`
}

// Point represents a 2D point.
type Point struct {
	X int32 `json:"X"`
	Y int32 `json:"Y"`
}

// MonitorInfo contains additional monitor metadata.
type MonitorInfo struct {
	ManufactureId         uint16 `json:"ManufactureId"`
	ProductCodeId         uint16 `json:"ProductCodeId"`
	Valid                 bool   `json:"Valid"`
	MonitorDevicePath     string `json:"MonitorDevicePath"`
	MonitorFriendlyDevice string `json:"MonitorFriendlyDevice"`
}

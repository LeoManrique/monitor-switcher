package ccd

import "unsafe"

// unsafePointer is a helper to convert between pointer types.
// This is needed for the union type handling in DisplayConfigModeInfo.
func unsafePointer(p any) unsafe.Pointer {
	switch v := p.(type) {
	case *byte:
		return unsafe.Pointer(v)
	case *DisplayConfigTargetMode:
		return unsafe.Pointer(v)
	case *DisplayConfigSourceMode:
		return unsafe.Pointer(v)
	default:
		panic("unsafePointer: unsupported type")
	}
}

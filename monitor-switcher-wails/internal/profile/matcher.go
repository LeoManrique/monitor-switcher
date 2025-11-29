package profile

import (
	"monitor-switcher-wails/internal/ccd"
)

// MatchAdapterIDs updates the adapter IDs in the profile settings to match the current system.
// The adapter ID changes on system restart, so we need to match by other stable identifiers.
// This implements the same 3-tier matching strategy as the original C# code.
func MatchAdapterIDs(profile *ccd.DisplaySettings, current *ccd.DisplaySettings) {
	// Tier 1: Match by sourceInfo.id and targetInfo.id pairs
	matchByPathIDs(profile, current)

	// Tier 2 and 3 are only needed if Tier 1 fails during SetDisplayConfig
	// They are implemented in the LoadProfile function with retry logic
}

// matchByPathIDs matches adapter IDs by comparing source and target IDs.
// This is the primary matching strategy (Tier 1).
func matchByPathIDs(profile *ccd.DisplaySettings, current *ccd.DisplaySettings) {
	// Match path info adapter IDs
	for i := range profile.PathInfoArray {
		for j := range current.PathInfoArray {
			if profile.PathInfoArray[i].SourceInfo.Id == current.PathInfoArray[j].SourceInfo.Id &&
				profile.PathInfoArray[i].TargetInfo.Id == current.PathInfoArray[j].TargetInfo.Id {
				// Found matching path - update adapter IDs
				profile.PathInfoArray[i].SourceInfo.AdapterId = current.PathInfoArray[j].SourceInfo.AdapterId
				profile.PathInfoArray[i].TargetInfo.AdapterId = current.PathInfoArray[j].TargetInfo.AdapterId
				break
			}
		}
	}

	// Match mode info adapter IDs using the updated path info
	for i := range profile.ModeInfoArray {
		for j := range profile.PathInfoArray {
			if profile.ModeInfoArray[i].Id == profile.PathInfoArray[j].TargetInfo.Id &&
				profile.ModeInfoArray[i].InfoType == ccd.ModeInfoTypeTarget {
				// Found target mode - look for corresponding source mode
				for k := range profile.ModeInfoArray {
					if profile.ModeInfoArray[k].Id == profile.PathInfoArray[j].SourceInfo.Id &&
						profile.ModeInfoArray[k].AdapterId.LowPart == profile.ModeInfoArray[i].AdapterId.LowPart &&
						profile.ModeInfoArray[k].InfoType == ccd.ModeInfoTypeSource {
						// Update source mode adapter ID
						profile.ModeInfoArray[k].AdapterId = profile.PathInfoArray[j].SourceInfo.AdapterId
						break
					}
				}
				// Update target mode adapter ID
				profile.ModeInfoArray[i].AdapterId = profile.PathInfoArray[j].TargetInfo.AdapterId
				break
			}
		}
	}
}

// MatchByMonitorName matches adapter IDs by comparing monitor friendly names.
// This is the fallback strategy (Tier 2) used when Tier 1 fails.
func MatchByMonitorName(profile *ccd.DisplaySettings, current *ccd.DisplaySettings) {
	for i := range profile.ModeInfoArray {
		for j := range current.AdditionalInfo {
			if current.AdditionalInfo[j].MonitorFriendlyDevice != "" &&
				i < len(profile.AdditionalInfo) &&
				profile.AdditionalInfo[i].MonitorFriendlyDevice != "" &&
				current.AdditionalInfo[j].MonitorFriendlyDevice == profile.AdditionalInfo[i].MonitorFriendlyDevice {

				originalID := profile.ModeInfoArray[i].AdapterId

				// Update all path info with matching adapter ID
				for k := range profile.PathInfoArray {
					if profile.PathInfoArray[k].TargetInfo.AdapterId == originalID {
						profile.PathInfoArray[k].TargetInfo.AdapterId = current.ModeInfoArray[j].AdapterId
						profile.PathInfoArray[k].SourceInfo.AdapterId = current.ModeInfoArray[j].AdapterId
						profile.PathInfoArray[k].TargetInfo.Id = current.ModeInfoArray[j].Id
					}
				}

				// Update all mode info with matching adapter ID
				for k := range profile.ModeInfoArray {
					if profile.ModeInfoArray[k].AdapterId == originalID {
						profile.ModeInfoArray[k].AdapterId = current.ModeInfoArray[j].AdapterId
					}
				}

				profile.ModeInfoArray[i].AdapterId = current.ModeInfoArray[j].AdapterId
				profile.ModeInfoArray[i].Id = current.ModeInfoArray[j].Id
				break
			}
		}
	}
}

// MatchByBulkReplacement replaces all instances of an old adapter ID with a new one.
// This is the last resort strategy (Tier 3).
func MatchByBulkReplacement(profile *ccd.DisplaySettings, current *ccd.DisplaySettings) {
	for i := range profile.PathInfoArray {
		for j := range current.PathInfoArray {
			if profile.PathInfoArray[i].SourceInfo.Id == current.PathInfoArray[j].SourceInfo.Id &&
				profile.PathInfoArray[i].TargetInfo.Id == current.PathInfoArray[j].TargetInfo.Id {

				oldID := profile.PathInfoArray[i].SourceInfo.AdapterId.LowPart
				newID := current.PathInfoArray[j].SourceInfo.AdapterId.LowPart

				// Replace all occurrences in path info
				for k := range profile.PathInfoArray {
					if profile.PathInfoArray[k].SourceInfo.AdapterId.LowPart == oldID {
						profile.PathInfoArray[k].SourceInfo.AdapterId.LowPart = newID
					}
					if profile.PathInfoArray[k].TargetInfo.AdapterId.LowPart == oldID {
						profile.PathInfoArray[k].TargetInfo.AdapterId.LowPart = newID
					}
				}

				// Replace all occurrences in mode info
				for k := range profile.ModeInfoArray {
					if profile.ModeInfoArray[k].AdapterId.LowPart == oldID {
						profile.ModeInfoArray[k].AdapterId.LowPart = newID
					}
				}
				break
			}
		}
	}
}

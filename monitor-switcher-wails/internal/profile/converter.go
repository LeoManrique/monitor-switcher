package profile

import (
	"monitor-switcher-wails/internal/ccd"
)

// ConvertToProfile converts CCD display settings to a profile for JSON serialization.
func ConvertToProfile(settings *ccd.DisplaySettings) *DisplayProfile {
	profile := &DisplayProfile{
		Version:        1,
		PathInfoArray:  make([]PathInfo, len(settings.PathInfoArray)),
		ModeInfoArray:  make([]ModeInfo, len(settings.ModeInfoArray)),
		AdditionalInfo: make([]MonitorInfo, len(settings.AdditionalInfo)),
	}

	// Convert path info
	for i, path := range settings.PathInfoArray {
		profile.PathInfoArray[i] = PathInfo{
			SourceInfo: PathSourceInfo{
				AdapterId:   AdapterId{LowPart: path.SourceInfo.AdapterId.LowPart, HighPart: path.SourceInfo.AdapterId.HighPart},
				Id:          path.SourceInfo.Id,
				ModeInfoIdx: path.SourceInfo.ModeInfoIdx,
				StatusFlags: path.SourceInfo.StatusFlags,
			},
			TargetInfo: PathTargetInfo{
				AdapterId:        AdapterId{LowPart: path.TargetInfo.AdapterId.LowPart, HighPart: path.TargetInfo.AdapterId.HighPart},
				Id:               path.TargetInfo.Id,
				ModeInfoIdx:      path.TargetInfo.ModeInfoIdx,
				OutputTechnology: path.TargetInfo.OutputTechnology,
				Rotation:         path.TargetInfo.Rotation,
				Scaling:          path.TargetInfo.Scaling,
				RefreshRate:      Rational{Numerator: path.TargetInfo.RefreshRate.Numerator, Denominator: path.TargetInfo.RefreshRate.Denominator},
				ScanLineOrdering: path.TargetInfo.ScanLineOrdering,
				TargetAvailable:  path.TargetInfo.TargetAvailable != 0,
				StatusFlags:      path.TargetInfo.StatusFlags,
			},
			Flags: path.Flags,
		}
	}

	// Convert mode info
	for i, mode := range settings.ModeInfoArray {
		modeInfo := ModeInfo{
			InfoType:  mode.InfoType,
			Id:        mode.Id,
			AdapterId: AdapterId{LowPart: mode.AdapterId.LowPart, HighPart: mode.AdapterId.HighPart},
		}

		if mode.InfoType == ccd.ModeInfoTypeTarget {
			tm := mode.GetTargetMode()
			modeInfo.TargetMode = &TargetMode{
				TargetVideoSignalInfo: VideoSignalInfo{
					PixelRate:        int64(tm.TargetVideoSignalInfo.PixelRate),
					HSyncFreq:        Rational{Numerator: tm.TargetVideoSignalInfo.HSyncFreq.Numerator, Denominator: tm.TargetVideoSignalInfo.HSyncFreq.Denominator},
					VSyncFreq:        Rational{Numerator: tm.TargetVideoSignalInfo.VSyncFreq.Numerator, Denominator: tm.TargetVideoSignalInfo.VSyncFreq.Denominator},
					ActiveSize:       Region2D{Cx: tm.TargetVideoSignalInfo.ActiveSize.Cx, Cy: tm.TargetVideoSignalInfo.ActiveSize.Cy},
					TotalSize:        Region2D{Cx: tm.TargetVideoSignalInfo.TotalSize.Cx, Cy: tm.TargetVideoSignalInfo.TotalSize.Cy},
					VideoStandard:    tm.TargetVideoSignalInfo.VideoStandard,
					ScanLineOrdering: tm.TargetVideoSignalInfo.ScanLineOrdering,
				},
			}
		} else if mode.InfoType == ccd.ModeInfoTypeSource {
			sm := mode.GetSourceMode()
			modeInfo.SourceMode = &SourceMode{
				Width:       sm.Width,
				Height:      sm.Height,
				PixelFormat: sm.PixelFormat,
				Position:    Point{X: sm.Position.X, Y: sm.Position.Y},
			}
		}

		profile.ModeInfoArray[i] = modeInfo
	}

	// Convert additional info
	for i, info := range settings.AdditionalInfo {
		profile.AdditionalInfo[i] = MonitorInfo{
			ManufactureId:         info.ManufactureId,
			ProductCodeId:         info.ProductCodeId,
			Valid:                 info.Valid,
			MonitorDevicePath:     info.MonitorDevicePath,
			MonitorFriendlyDevice: info.MonitorFriendlyDevice,
		}
	}

	return profile
}

// ConvertFromProfile converts a profile to CCD display settings for applying.
func ConvertFromProfile(profile *DisplayProfile) *ccd.DisplaySettings {
	settings := &ccd.DisplaySettings{
		PathInfoArray:  make([]ccd.DisplayConfigPathInfo, len(profile.PathInfoArray)),
		ModeInfoArray:  make([]ccd.DisplayConfigModeInfo, len(profile.ModeInfoArray)),
		AdditionalInfo: make([]ccd.MonitorInfo, len(profile.AdditionalInfo)),
	}

	// Convert path info
	for i, path := range profile.PathInfoArray {
		targetAvailable := uint32(0)
		if path.TargetInfo.TargetAvailable {
			targetAvailable = 1
		}

		settings.PathInfoArray[i] = ccd.DisplayConfigPathInfo{
			SourceInfo: ccd.DisplayConfigPathSourceInfo{
				AdapterId:   ccd.LUID{LowPart: path.SourceInfo.AdapterId.LowPart, HighPart: path.SourceInfo.AdapterId.HighPart},
				Id:          path.SourceInfo.Id,
				ModeInfoIdx: path.SourceInfo.ModeInfoIdx,
				StatusFlags: path.SourceInfo.StatusFlags,
			},
			TargetInfo: ccd.DisplayConfigPathTargetInfo{
				AdapterId:        ccd.LUID{LowPart: path.TargetInfo.AdapterId.LowPart, HighPart: path.TargetInfo.AdapterId.HighPart},
				Id:               path.TargetInfo.Id,
				ModeInfoIdx:      path.TargetInfo.ModeInfoIdx,
				OutputTechnology: path.TargetInfo.OutputTechnology,
				Rotation:         path.TargetInfo.Rotation,
				Scaling:          path.TargetInfo.Scaling,
				RefreshRate:      ccd.DisplayConfigRational{Numerator: path.TargetInfo.RefreshRate.Numerator, Denominator: path.TargetInfo.RefreshRate.Denominator},
				ScanLineOrdering: path.TargetInfo.ScanLineOrdering,
				TargetAvailable:  targetAvailable,
				StatusFlags:      path.TargetInfo.StatusFlags,
			},
			Flags: path.Flags,
		}
	}

	// Convert mode info
	for i, mode := range profile.ModeInfoArray {
		modeInfo := ccd.DisplayConfigModeInfo{
			InfoType:  mode.InfoType,
			Id:        mode.Id,
			AdapterId: ccd.LUID{LowPart: mode.AdapterId.LowPart, HighPart: mode.AdapterId.HighPart},
		}

		if mode.InfoType == ccd.ModeInfoTypeTarget && mode.TargetMode != nil {
			tm := &ccd.DisplayConfigTargetMode{
				TargetVideoSignalInfo: ccd.DisplayConfigVideoSignalInfo{
					PixelRate:        uint64(mode.TargetMode.TargetVideoSignalInfo.PixelRate),
					HSyncFreq:        ccd.DisplayConfigRational{Numerator: mode.TargetMode.TargetVideoSignalInfo.HSyncFreq.Numerator, Denominator: mode.TargetMode.TargetVideoSignalInfo.HSyncFreq.Denominator},
					VSyncFreq:        ccd.DisplayConfigRational{Numerator: mode.TargetMode.TargetVideoSignalInfo.VSyncFreq.Numerator, Denominator: mode.TargetMode.TargetVideoSignalInfo.VSyncFreq.Denominator},
					ActiveSize:       ccd.DisplayConfig2DRegion{Cx: mode.TargetMode.TargetVideoSignalInfo.ActiveSize.Cx, Cy: mode.TargetMode.TargetVideoSignalInfo.ActiveSize.Cy},
					TotalSize:        ccd.DisplayConfig2DRegion{Cx: mode.TargetMode.TargetVideoSignalInfo.TotalSize.Cx, Cy: mode.TargetMode.TargetVideoSignalInfo.TotalSize.Cy},
					VideoStandard:    mode.TargetMode.TargetVideoSignalInfo.VideoStandard,
					ScanLineOrdering: mode.TargetMode.TargetVideoSignalInfo.ScanLineOrdering,
				},
			}
			modeInfo.SetTargetMode(tm)
		} else if mode.InfoType == ccd.ModeInfoTypeSource && mode.SourceMode != nil {
			sm := &ccd.DisplayConfigSourceMode{
				Width:       mode.SourceMode.Width,
				Height:      mode.SourceMode.Height,
				PixelFormat: mode.SourceMode.PixelFormat,
				Position:    ccd.PointL{X: mode.SourceMode.Position.X, Y: mode.SourceMode.Position.Y},
			}
			modeInfo.SetSourceMode(sm)
		}

		settings.ModeInfoArray[i] = modeInfo
	}

	// Convert additional info
	for i, info := range profile.AdditionalInfo {
		settings.AdditionalInfo[i] = ccd.MonitorInfo{
			ManufactureId:         info.ManufactureId,
			ProductCodeId:         info.ProductCodeId,
			Valid:                 info.Valid,
			MonitorDevicePath:     info.MonitorDevicePath,
			MonitorFriendlyDevice: info.MonitorFriendlyDevice,
		}
	}

	return settings
}

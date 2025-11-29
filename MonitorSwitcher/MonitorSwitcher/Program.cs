/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Windows.Forms;
using System.IO;
using System.Web.Script.Serialization;

namespace MonitorSwitcherGUI
{
    #region JSON Data Models

    /// <summary>
    /// Root object for display profile JSON serialization
    /// </summary>
    public class DisplayProfile
    {
        public int Version { get; set; } = 1;
        public List<PathInfo> PathInfoArray { get; set; } = new List<PathInfo>();
        public List<ModeInfo> ModeInfoArray { get; set; } = new List<ModeInfo>();
        public List<MonitorInfo> AdditionalInfo { get; set; } = new List<MonitorInfo>();
    }

    public class PathInfo
    {
        public PathSourceInfo SourceInfo { get; set; }
        public PathTargetInfo TargetInfo { get; set; }
        public uint Flags { get; set; }
    }

    public class PathSourceInfo
    {
        public AdapterId AdapterId { get; set; }
        public uint Id { get; set; }
        public uint ModeInfoIdx { get; set; }
        public uint StatusFlags { get; set; }
    }

    public class PathTargetInfo
    {
        public AdapterId AdapterId { get; set; }
        public uint Id { get; set; }
        public uint ModeInfoIdx { get; set; }
        public uint OutputTechnology { get; set; }
        public uint Rotation { get; set; }
        public uint Scaling { get; set; }
        public Rational RefreshRate { get; set; }
        public uint ScanLineOrdering { get; set; }
        public bool TargetAvailable { get; set; }
        public uint StatusFlags { get; set; }
    }

    public class AdapterId
    {
        public uint LowPart { get; set; }
        public uint HighPart { get; set; }
    }

    public class Rational
    {
        public uint Numerator { get; set; }
        public uint Denominator { get; set; }
    }

    public class ModeInfo
    {
        public uint InfoType { get; set; }
        public uint Id { get; set; }
        public AdapterId AdapterId { get; set; }
        public TargetMode TargetMode { get; set; }
        public SourceMode SourceMode { get; set; }
    }

    public class TargetMode
    {
        public VideoSignalInfo TargetVideoSignalInfo { get; set; }
    }

    public class VideoSignalInfo
    {
        public long PixelRate { get; set; }
        public Rational HSyncFreq { get; set; }
        public Rational VSyncFreq { get; set; }
        public Region2D ActiveSize { get; set; }
        public Region2D TotalSize { get; set; }
        public uint VideoStandard { get; set; }
        public uint ScanLineOrdering { get; set; }
    }

    public class Region2D
    {
        public uint Cx { get; set; }
        public uint Cy { get; set; }
    }

    public class SourceMode
    {
        public uint Width { get; set; }
        public uint Height { get; set; }
        public uint PixelFormat { get; set; }
        public Point Position { get; set; }
    }

    public class Point
    {
        public int X { get; set; }
        public int Y { get; set; }
    }

    public class MonitorInfo
    {
        public ushort ManufactureId { get; set; }
        public ushort ProductCodeId { get; set; }
        public bool Valid { get; set; }
        public string MonitorDevicePath { get; set; }
        public string MonitorFriendlyDevice { get; set; }
    }

    #endregion

    public class MonitorSwitcher
    {
        private static bool debug;
        private static bool noIDMatch;

        public static void DebugOutput(string text)
        {
            if (debug)
            {
                Console.WriteLine(text);
            }
        }

        #region Conversion Methods (CCD <-> JSON Models)

        private static DisplayProfile ConvertToProfile(
            CCDWrapper.DisplayConfigPathInfo[] pathInfoArray,
            CCDWrapper.DisplayConfigModeInfo[] modeInfoArray,
            CCDWrapper.MonitorAdditionalInfo[] additionalInfo)
        {
            var profile = new DisplayProfile();

            // Convert path info
            foreach (var path in pathInfoArray)
            {
                profile.PathInfoArray.Add(new PathInfo
                {
                    SourceInfo = new PathSourceInfo
                    {
                        AdapterId = new AdapterId { LowPart = path.sourceInfo.adapterId.LowPart, HighPart = path.sourceInfo.adapterId.HighPart },
                        Id = path.sourceInfo.id,
                        ModeInfoIdx = path.sourceInfo.modeInfoIdx,
                        StatusFlags = (uint)path.sourceInfo.statusFlags
                    },
                    TargetInfo = new PathTargetInfo
                    {
                        AdapterId = new AdapterId { LowPart = path.targetInfo.adapterId.LowPart, HighPart = path.targetInfo.adapterId.HighPart },
                        Id = path.targetInfo.id,
                        ModeInfoIdx = path.targetInfo.modeInfoIdx,
                        OutputTechnology = (uint)path.targetInfo.outputTechnology,
                        Rotation = (uint)path.targetInfo.rotation,
                        Scaling = (uint)path.targetInfo.scaling,
                        RefreshRate = new Rational { Numerator = path.targetInfo.refreshRate.numerator, Denominator = path.targetInfo.refreshRate.denominator },
                        ScanLineOrdering = (uint)path.targetInfo.scanLineOrdering,
                        TargetAvailable = path.targetInfo.targetAvailable,
                        StatusFlags = (uint)path.targetInfo.statusFlags
                    },
                    Flags = path.flags
                });
            }

            // Convert mode info
            foreach (var mode in modeInfoArray)
            {
                var modeInfo = new ModeInfo
                {
                    InfoType = (uint)mode.infoType,
                    Id = mode.id,
                    AdapterId = new AdapterId { LowPart = mode.adapterId.LowPart, HighPart = mode.adapterId.HighPart }
                };

                if (mode.infoType == CCDWrapper.DisplayConfigModeInfoType.Target)
                {
                    modeInfo.TargetMode = new TargetMode
                    {
                        TargetVideoSignalInfo = new VideoSignalInfo
                        {
                            PixelRate = mode.targetMode.targetVideoSignalInfo.pixelRate,
                            HSyncFreq = new Rational { Numerator = mode.targetMode.targetVideoSignalInfo.hSyncFreq.numerator, Denominator = mode.targetMode.targetVideoSignalInfo.hSyncFreq.denominator },
                            VSyncFreq = new Rational { Numerator = mode.targetMode.targetVideoSignalInfo.vSyncFreq.numerator, Denominator = mode.targetMode.targetVideoSignalInfo.vSyncFreq.denominator },
                            ActiveSize = new Region2D { Cx = mode.targetMode.targetVideoSignalInfo.activeSize.cx, Cy = mode.targetMode.targetVideoSignalInfo.activeSize.cy },
                            TotalSize = new Region2D { Cx = mode.targetMode.targetVideoSignalInfo.totalSize.cx, Cy = mode.targetMode.targetVideoSignalInfo.totalSize.cy },
                            VideoStandard = (uint)mode.targetMode.targetVideoSignalInfo.videoStandard,
                            ScanLineOrdering = (uint)mode.targetMode.targetVideoSignalInfo.ScanLineOrdering
                        }
                    };
                }
                else
                {
                    modeInfo.SourceMode = new SourceMode
                    {
                        Width = mode.sourceMode.width,
                        Height = mode.sourceMode.height,
                        PixelFormat = (uint)mode.sourceMode.pixelFormat,
                        Position = new Point { X = mode.sourceMode.position.x, Y = mode.sourceMode.position.y }
                    };
                }

                profile.ModeInfoArray.Add(modeInfo);
            }

            // Convert additional info
            foreach (var info in additionalInfo)
            {
                profile.AdditionalInfo.Add(new MonitorInfo
                {
                    ManufactureId = info.manufactureId,
                    ProductCodeId = info.productCodeId,
                    Valid = info.valid,
                    MonitorDevicePath = info.monitorDevicePath,
                    MonitorFriendlyDevice = info.monitorFriendlyDevice
                });
            }

            return profile;
        }

        private static void ConvertFromProfile(
            DisplayProfile profile,
            out CCDWrapper.DisplayConfigPathInfo[] pathInfoArray,
            out CCDWrapper.DisplayConfigModeInfo[] modeInfoArray,
            out CCDWrapper.MonitorAdditionalInfo[] additionalInfo)
        {
            // Convert path info
            pathInfoArray = new CCDWrapper.DisplayConfigPathInfo[profile.PathInfoArray.Count];
            for (int i = 0; i < profile.PathInfoArray.Count; i++)
            {
                var p = profile.PathInfoArray[i];
                pathInfoArray[i] = new CCDWrapper.DisplayConfigPathInfo
                {
                    sourceInfo = new CCDWrapper.DisplayConfigPathSourceInfo
                    {
                        adapterId = new CCDWrapper.LUID { LowPart = p.SourceInfo.AdapterId.LowPart, HighPart = p.SourceInfo.AdapterId.HighPart },
                        id = p.SourceInfo.Id,
                        modeInfoIdx = p.SourceInfo.ModeInfoIdx,
                        statusFlags = (CCDWrapper.DisplayConfigSourceStatus)p.SourceInfo.StatusFlags
                    },
                    targetInfo = new CCDWrapper.DisplayConfigPathTargetInfo
                    {
                        adapterId = new CCDWrapper.LUID { LowPart = p.TargetInfo.AdapterId.LowPart, HighPart = p.TargetInfo.AdapterId.HighPart },
                        id = p.TargetInfo.Id,
                        modeInfoIdx = p.TargetInfo.ModeInfoIdx,
                        outputTechnology = (CCDWrapper.DisplayConfigVideoOutputTechnology)p.TargetInfo.OutputTechnology,
                        rotation = (CCDWrapper.DisplayConfigRotation)p.TargetInfo.Rotation,
                        scaling = (CCDWrapper.DisplayConfigScaling)p.TargetInfo.Scaling,
                        refreshRate = new CCDWrapper.DisplayConfigRational { numerator = p.TargetInfo.RefreshRate.Numerator, denominator = p.TargetInfo.RefreshRate.Denominator },
                        scanLineOrdering = (CCDWrapper.DisplayConfigScanLineOrdering)p.TargetInfo.ScanLineOrdering,
                        targetAvailable = p.TargetInfo.TargetAvailable,
                        statusFlags = (CCDWrapper.DisplayConfigTargetStatus)p.TargetInfo.StatusFlags
                    },
                    flags = p.Flags
                };
            }

            // Convert mode info
            modeInfoArray = new CCDWrapper.DisplayConfigModeInfo[profile.ModeInfoArray.Count];
            for (int i = 0; i < profile.ModeInfoArray.Count; i++)
            {
                var m = profile.ModeInfoArray[i];
                modeInfoArray[i] = new CCDWrapper.DisplayConfigModeInfo
                {
                    infoType = (CCDWrapper.DisplayConfigModeInfoType)m.InfoType,
                    id = m.Id,
                    adapterId = new CCDWrapper.LUID { LowPart = m.AdapterId.LowPart, HighPart = m.AdapterId.HighPart }
                };

                if (m.InfoType == (uint)CCDWrapper.DisplayConfigModeInfoType.Target && m.TargetMode != null)
                {
                    modeInfoArray[i].targetMode = new CCDWrapper.DisplayConfigTargetMode
                    {
                        targetVideoSignalInfo = new CCDWrapper.DisplayConfigVideoSignalInfo
                        {
                            pixelRate = m.TargetMode.TargetVideoSignalInfo.PixelRate,
                            hSyncFreq = new CCDWrapper.DisplayConfigRational { numerator = m.TargetMode.TargetVideoSignalInfo.HSyncFreq.Numerator, denominator = m.TargetMode.TargetVideoSignalInfo.HSyncFreq.Denominator },
                            vSyncFreq = new CCDWrapper.DisplayConfigRational { numerator = m.TargetMode.TargetVideoSignalInfo.VSyncFreq.Numerator, denominator = m.TargetMode.TargetVideoSignalInfo.VSyncFreq.Denominator },
                            activeSize = new CCDWrapper.DisplayConfig2DRegion { cx = m.TargetMode.TargetVideoSignalInfo.ActiveSize.Cx, cy = m.TargetMode.TargetVideoSignalInfo.ActiveSize.Cy },
                            totalSize = new CCDWrapper.DisplayConfig2DRegion { cx = m.TargetMode.TargetVideoSignalInfo.TotalSize.Cx, cy = m.TargetMode.TargetVideoSignalInfo.TotalSize.Cy },
                            videoStandard = (CCDWrapper.D3DkmdtVideoSignalStandard)m.TargetMode.TargetVideoSignalInfo.VideoStandard,
                            ScanLineOrdering = (CCDWrapper.DisplayConfigScanLineOrdering)m.TargetMode.TargetVideoSignalInfo.ScanLineOrdering
                        }
                    };
                }
                else if (m.SourceMode != null)
                {
                    modeInfoArray[i].sourceMode = new CCDWrapper.DisplayConfigSourceMode
                    {
                        width = m.SourceMode.Width,
                        height = m.SourceMode.Height,
                        pixelFormat = (CCDWrapper.DisplayConfigPixelFormat)m.SourceMode.PixelFormat,
                        position = new CCDWrapper.PointL { x = m.SourceMode.Position.X, y = m.SourceMode.Position.Y }
                    };
                }
            }

            // Convert additional info
            additionalInfo = new CCDWrapper.MonitorAdditionalInfo[profile.AdditionalInfo.Count];
            for (int i = 0; i < profile.AdditionalInfo.Count; i++)
            {
                var a = profile.AdditionalInfo[i];
                additionalInfo[i] = new CCDWrapper.MonitorAdditionalInfo
                {
                    manufactureId = a.ManufactureId,
                    productCodeId = a.ProductCodeId,
                    valid = a.Valid,
                    monitorDevicePath = a.MonitorDevicePath,
                    monitorFriendlyDevice = a.MonitorFriendlyDevice
                };
            }
        }

        #endregion

        public static bool LoadDisplaySettings(string fileName)
        {
            DebugOutput("Loading display settings from file: " + fileName);
            if (!File.Exists(fileName))
            {
                Console.WriteLine("Failed to load display settings because file does not exist: " + fileName);
                return false;
            }

            try
            {
                // Read and deserialize JSON
                DebugOutput("Parsing JSON file");
                string json = File.ReadAllText(fileName);
                var serializer = new JavaScriptSerializer();
                var profile = serializer.Deserialize<DisplayProfile>(json);

                // Convert from JSON model to CCD structures
                CCDWrapper.DisplayConfigPathInfo[] pathInfoArray;
                CCDWrapper.DisplayConfigModeInfo[] modeInfoArray;
                CCDWrapper.MonitorAdditionalInfo[] additionalInfoArray;
                ConvertFromProfile(profile, out pathInfoArray, out modeInfoArray, out additionalInfoArray);

                DebugOutput("Parsing of JSON file successful");

                // Keep copies for retry attempts
                var pathInfoList = pathInfoArray.ToList();
                var modeInfoList = modeInfoArray.ToList();
                var additionalInfoList = additionalInfoArray.ToList();

                // Get current display settings
                DebugOutput("Getting current display settings");
                CCDWrapper.DisplayConfigPathInfo[] pathInfoArrayCurrent = new CCDWrapper.DisplayConfigPathInfo[0];
                CCDWrapper.DisplayConfigModeInfo[] modeInfoArrayCurrent = new CCDWrapper.DisplayConfigModeInfo[0];
                CCDWrapper.MonitorAdditionalInfo[] additionalInfoCurrent = new CCDWrapper.MonitorAdditionalInfo[0];

                bool statusCurrent = GetDisplaySettings(ref pathInfoArrayCurrent, ref modeInfoArrayCurrent, ref additionalInfoCurrent, false);
                if (statusCurrent)
                {
                    if (!noIDMatch)
                    {
                        // For some reason the adapterID parameter changes upon system restart, all other parameters however, especially the ID remain constant.
                        // We check the loaded settings against the current settings replacing the adapaterID with the other parameters
                        DebugOutput("Matching of adapter IDs for pathInfo");
                        for (int iPathInfo = 0; iPathInfo < pathInfoArray.Length; iPathInfo++)
                        {
                            for (int iPathInfoCurrent = 0; iPathInfoCurrent < pathInfoArrayCurrent.Length; iPathInfoCurrent++)
                            {
                                DebugOutput("\t---");
                                DebugOutput("\tIndex JSON = " + iPathInfo);
                                DebugOutput("\tIndex Current = " + iPathInfoCurrent);
                                DebugOutput("\tsourceInfo.id JSON = " + pathInfoArray[iPathInfo].sourceInfo.id);
                                DebugOutput("\tsourceInfo.id Current = " + pathInfoArrayCurrent[iPathInfoCurrent].sourceInfo.id);
                                DebugOutput("\ttargetInfo.id JSON = " + pathInfoArray[iPathInfo].targetInfo.id);
                                DebugOutput("\ttargetInfo.id Current = " + pathInfoArrayCurrent[iPathInfoCurrent].targetInfo.id);
                                if ((pathInfoArray[iPathInfo].sourceInfo.id == pathInfoArrayCurrent[iPathInfoCurrent].sourceInfo.id) &&
                                    (pathInfoArray[iPathInfo].targetInfo.id == pathInfoArrayCurrent[iPathInfoCurrent].targetInfo.id))
                                {
                                    DebugOutput("\t!!! Both IDs are a match, assigning current adapter ID !!!");
                                    pathInfoArray[iPathInfo].sourceInfo.adapterId.LowPart = pathInfoArrayCurrent[iPathInfoCurrent].sourceInfo.adapterId.LowPart;
                                    pathInfoArray[iPathInfo].targetInfo.adapterId.LowPart = pathInfoArrayCurrent[iPathInfoCurrent].targetInfo.adapterId.LowPart;
                                    break;
                                }
                                DebugOutput("\t---");
                            }
                        }

                        // Same again for modeInfo, however we get the required adapterId information from the pathInfoArray
                        DebugOutput("Matching of adapter IDs for modeInfo");
                        for (int iModeInfo = 0; iModeInfo < modeInfoArray.Length; iModeInfo++)
                        {
                            for (int iPathInfo = 0; iPathInfo < pathInfoArray.Length; iPathInfo++)
                            {
                                DebugOutput("\t---");
                                DebugOutput("\tIndex Mode = " + iModeInfo);
                                DebugOutput("\tIndex Path = " + iPathInfo);
                                DebugOutput("\tmodeInfo.id = " + modeInfoArray[iModeInfo].id);
                                DebugOutput("\tpathInfo.id = " + pathInfoArray[iPathInfo].targetInfo.id);
                                DebugOutput("\tmodeInfo.infoType = " + modeInfoArray[iModeInfo].infoType);
                                if ((modeInfoArray[iModeInfo].id == pathInfoArray[iPathInfo].targetInfo.id) &&
                                    (modeInfoArray[iModeInfo].infoType == CCDWrapper.DisplayConfigModeInfoType.Target))
                                {
                                    DebugOutput("\t\tTarget adapter id found, checking for source modeInfo and adpaterID");
                                    // We found target adapter id, now lets look for the source modeInfo and adapterID
                                    for (int iModeInfoSource = 0; iModeInfoSource < modeInfoArray.Length; iModeInfoSource++)
                                    {
                                        DebugOutput("\t\t---");
                                        DebugOutput("\t\tIndex = " + iModeInfoSource);
                                        DebugOutput("\t\tmodeInfo.id Source = " + modeInfoArray[iModeInfoSource].id);
                                        DebugOutput("\t\tpathInfo.sourceInfo.id = " + pathInfoArray[iPathInfo].sourceInfo.id);
                                        if ((modeInfoArray[iModeInfoSource].id == pathInfoArray[iPathInfo].sourceInfo.id) &&
                                            (modeInfoArray[iModeInfoSource].adapterId.LowPart == modeInfoArray[iModeInfo].adapterId.LowPart) &&
                                            (modeInfoArray[iModeInfoSource].infoType == CCDWrapper.DisplayConfigModeInfoType.Source))
                                        {
                                            DebugOutput("\t\t!!! IDs are a match, taking adpater id from pathInfo !!!");
                                            modeInfoArray[iModeInfoSource].adapterId.LowPart = pathInfoArray[iPathInfo].sourceInfo.adapterId.LowPart;
                                            break;
                                        }
                                        DebugOutput("\t\t---");
                                    }
                                    modeInfoArray[iModeInfo].adapterId.LowPart = pathInfoArray[iPathInfo].targetInfo.adapterId.LowPart;
                                    break;
                                }
                                DebugOutput("\t---");
                            }
                        }
                        DebugOutput("Done matching of adapter IDs");
                    }

                    // Set loaded display settings
                    DebugOutput("Setting up final display settings to load");
                    if (debug)
                    {
                        Console.WriteLine("\nDisplay settings to be loaded: ");
                        Console.WriteLine(PrintDisplaySettings(pathInfoArray, modeInfoArray));
                    }
                    uint numPathArrayElements = (uint)pathInfoArray.Length;
                    uint numModeInfoArrayElements = (uint)modeInfoArray.Length;

                    // First let's try without SdcFlags.AllowChanges
                    long status = CCDWrapper.SetDisplayConfig(numPathArrayElements, pathInfoArray, numModeInfoArrayElements, modeInfoArray,
                                                              CCDWrapper.SdcFlags.Apply | CCDWrapper.SdcFlags.UseSuppliedDisplayConfig | CCDWrapper.SdcFlags.SaveToDatabase | CCDWrapper.SdcFlags.NoOptimization);

                    if (status != 0)
                    {
                        Console.WriteLine("Failed to set display settings without SdcFlags.AllowChanges, ERROR: " + status.ToString());
                        Console.WriteLine("Trying again with additional SdcFlags.AllowChanges flag");
                        status = CCDWrapper.SetDisplayConfig(numPathArrayElements, pathInfoArray, numModeInfoArrayElements, modeInfoArray,
                                                              CCDWrapper.SdcFlags.Apply | CCDWrapper.SdcFlags.UseSuppliedDisplayConfig | CCDWrapper.SdcFlags.SaveToDatabase | CCDWrapper.SdcFlags.NoOptimization | CCDWrapper.SdcFlags.AllowChanges);
                    }

                    if (status != 0)
                    {
                        Console.WriteLine("Failed to set display settings using default method, ERROR: " + status.ToString());

                        if ((additionalInfoCurrent.Length > 0) && (additionalInfoList.Count > 0))
                        {
                            Console.WriteLine("Trying alternative method");
                            // Restore original settings and adapter IDs
                            pathInfoArray = pathInfoList.ToArray();
                            modeInfoArray = modeInfoList.ToArray();

                            DebugOutput("Alternative matching mode");
                            for (int iModeInfo = 0; iModeInfo < modeInfoArray.Length; iModeInfo++)
                            {
                                for (int iAdditionalInfoCurrent = 0; iAdditionalInfoCurrent < additionalInfoCurrent.Length; iAdditionalInfoCurrent++)
                                {
                                    if ((additionalInfoCurrent[iAdditionalInfoCurrent].monitorFriendlyDevice != null) && (additionalInfoList[iModeInfo].monitorFriendlyDevice != null))
                                    {
                                        if (additionalInfoCurrent[iAdditionalInfoCurrent].monitorFriendlyDevice.Equals(additionalInfoList[iModeInfo].monitorFriendlyDevice))
                                        {
                                            CCDWrapper.LUID originalID = modeInfoArray[iModeInfo].adapterId;
                                            for (int iPathInfo = 0; iPathInfo < pathInfoArray.Length; iPathInfo++)
                                            {
                                                if ((pathInfoArray[iPathInfo].targetInfo.adapterId.LowPart == originalID.LowPart) &&
                                                   (pathInfoArray[iPathInfo].targetInfo.adapterId.HighPart == originalID.HighPart))
                                                {
                                                    pathInfoArray[iPathInfo].targetInfo.adapterId = modeInfoArrayCurrent[iAdditionalInfoCurrent].adapterId;
                                                    pathInfoArray[iPathInfo].sourceInfo.adapterId = modeInfoArrayCurrent[iAdditionalInfoCurrent].adapterId;
                                                    pathInfoArray[iPathInfo].targetInfo.id = modeInfoArrayCurrent[iAdditionalInfoCurrent].id;
                                                }
                                            }
                                            for (int iModeInfoFix = 0; iModeInfoFix < modeInfoArray.Length; iModeInfoFix++)
                                            {
                                                if ((modeInfoArray[iModeInfoFix].adapterId.LowPart == originalID.LowPart) &&
                                                    (modeInfoArray[iModeInfoFix].adapterId.HighPart == originalID.HighPart))
                                                {
                                                    modeInfoArray[iModeInfoFix].adapterId = modeInfoArrayCurrent[iAdditionalInfoCurrent].adapterId;
                                                }
                                            }
                                            modeInfoArray[iModeInfo].adapterId = modeInfoArrayCurrent[iAdditionalInfoCurrent].adapterId;
                                            modeInfoArray[iModeInfo].id = modeInfoArrayCurrent[iAdditionalInfoCurrent].id;
                                            break;
                                        }
                                    }
                                }
                            }

                            if (debug)
                            {
                                Console.WriteLine("\nDisplay settings to be loaded: ");
                                Console.WriteLine(PrintDisplaySettings(pathInfoArray, modeInfoArray));
                            }

                            status = CCDWrapper.SetDisplayConfig(numPathArrayElements, pathInfoArray, numModeInfoArrayElements, modeInfoArray,
                                                                 CCDWrapper.SdcFlags.Apply | CCDWrapper.SdcFlags.UseSuppliedDisplayConfig | CCDWrapper.SdcFlags.NoOptimization | CCDWrapper.SdcFlags.SaveToDatabase);

                            if (status != 0)
                            {
                                status = CCDWrapper.SetDisplayConfig(numPathArrayElements, pathInfoArray, numModeInfoArrayElements, modeInfoArray,
                                                                     CCDWrapper.SdcFlags.Apply | CCDWrapper.SdcFlags.UseSuppliedDisplayConfig | CCDWrapper.SdcFlags.NoOptimization | CCDWrapper.SdcFlags.SaveToDatabase | CCDWrapper.SdcFlags.AllowChanges);
                            }
                        }

                        if (status != 0)
                        {
                            Console.WriteLine("Failed to set display settings using alternative method, ERROR: " + status.ToString());
                            Console.WriteLine("\nTrying yet another method for adapter ID matching:");

                            // Restore original settings and adapter IDs
                            pathInfoArray = pathInfoList.ToArray();
                            modeInfoArray = modeInfoList.ToArray();

                            for (int iPathInfo = 0; iPathInfo < pathInfoArray.Length; iPathInfo++)
                            {
                                for (int iPathInfoCurrent = 0; iPathInfoCurrent < pathInfoArrayCurrent.Length; iPathInfoCurrent++)
                                {
                                    if ((pathInfoArray[iPathInfo].sourceInfo.id == pathInfoArrayCurrent[iPathInfoCurrent].sourceInfo.id) &&
                                        (pathInfoArray[iPathInfo].targetInfo.id == pathInfoArrayCurrent[iPathInfoCurrent].targetInfo.id))
                                    {
                                        DebugOutput("\t!!! Both IDs are a match, getting new Adapter ID and replacing all other IDs !!!");
                                        uint oldID = pathInfoArray[iPathInfo].sourceInfo.adapterId.LowPart;
                                        uint newID = pathInfoArrayCurrent[iPathInfoCurrent].sourceInfo.adapterId.LowPart;
                                        for (int iPathInfoReplace = 0; iPathInfoReplace < pathInfoArray.Length; iPathInfoReplace++)
                                        {
                                            if (pathInfoArray[iPathInfoReplace].sourceInfo.adapterId.LowPart == oldID)
                                                pathInfoArray[iPathInfoReplace].sourceInfo.adapterId.LowPart = newID;
                                            if (pathInfoArray[iPathInfoReplace].targetInfo.adapterId.LowPart == oldID)
                                                pathInfoArray[iPathInfoReplace].targetInfo.adapterId.LowPart = newID;
                                        }

                                        for (int iModeInfoReplace = 0; iModeInfoReplace < modeInfoArray.Length; iModeInfoReplace++)
                                        {
                                            if (modeInfoArray[iModeInfoReplace].adapterId.LowPart == oldID)
                                            {
                                                modeInfoArray[iModeInfoReplace].adapterId.LowPart = newID;
                                            }
                                        }
                                        break;
                                    }
                                    DebugOutput("\t---");
                                }
                            }

                            DebugOutput("Setting up final display settings to load");
                            if (debug)
                            {
                                Console.WriteLine("\nDisplay settings to be loaded: ");
                                Console.WriteLine(PrintDisplaySettings(pathInfoArray, modeInfoArray));
                            }

                            status = CCDWrapper.SetDisplayConfig(numPathArrayElements, pathInfoArray, numModeInfoArrayElements, modeInfoArray,
                                                                    CCDWrapper.SdcFlags.Apply | CCDWrapper.SdcFlags.UseSuppliedDisplayConfig | CCDWrapper.SdcFlags.SaveToDatabase | CCDWrapper.SdcFlags.NoOptimization | CCDWrapper.SdcFlags.AllowChanges);

                            if (status != 0)
                            {
                                status = CCDWrapper.SetDisplayConfig(numPathArrayElements, pathInfoArray, numModeInfoArrayElements, modeInfoArray,
                                                                                            CCDWrapper.SdcFlags.Apply | CCDWrapper.SdcFlags.UseSuppliedDisplayConfig | CCDWrapper.SdcFlags.SaveToDatabase | CCDWrapper.SdcFlags.NoOptimization | CCDWrapper.SdcFlags.AllowChanges);
                            }
                        }

                        if (status != 0)
                        {
                            Console.WriteLine("Failed to set display settings using the other alternative method, ERROR: " + status.ToString());
                            return false;
                        }
                    }

                    return true;
                }

                DebugOutput("Failed to get current display settings");
                return false;
            }
            catch (Exception ex)
            {
                Console.WriteLine("Error loading display settings: " + ex.Message);
                DebugOutput("Exception: " + ex.ToString());
                return false;
            }
        }

        public static bool GetDisplaySettings(ref CCDWrapper.DisplayConfigPathInfo[] pathInfoArray, ref CCDWrapper.DisplayConfigModeInfo[] modeInfoArray, ref CCDWrapper.MonitorAdditionalInfo[] additionalInfo, bool ActiveOnly)
        {
            uint numPathArrayElements;
            uint numModeInfoArrayElements;

            DebugOutput("Getting display settings");
            CCDWrapper.QueryDisplayFlags queryFlags = CCDWrapper.QueryDisplayFlags.AllPaths;
            if (ActiveOnly)
            {
                queryFlags = CCDWrapper.QueryDisplayFlags.OnlyActivePaths;
            }

            DebugOutput("Getting buffer size");
            var status = CCDWrapper.GetDisplayConfigBufferSizes(queryFlags, out numPathArrayElements, out numModeInfoArrayElements);
            if (status == 0)
            {
                pathInfoArray = new CCDWrapper.DisplayConfigPathInfo[numPathArrayElements];
                modeInfoArray = new CCDWrapper.DisplayConfigModeInfo[numModeInfoArrayElements];
                additionalInfo = new CCDWrapper.MonitorAdditionalInfo[numModeInfoArrayElements];

                DebugOutput("Querying display config");
                status = CCDWrapper.QueryDisplayConfig(queryFlags,
                                                       ref numPathArrayElements, pathInfoArray, ref numModeInfoArrayElements,
                                                       modeInfoArray, IntPtr.Zero);

                if (status == 0)
                {
                    // cleanup of modeInfo bad elements
                    int validCount = 0;
                    foreach (CCDWrapper.DisplayConfigModeInfo modeInfo in modeInfoArray)
                    {
                        if (modeInfo.infoType != CCDWrapper.DisplayConfigModeInfoType.Zero)
                        {
                            validCount++;
                        }
                    }
                    if (validCount > 0)
                    {
                        CCDWrapper.DisplayConfigModeInfo[] tempInfoArray = new CCDWrapper.DisplayConfigModeInfo[modeInfoArray.Count()];
                        modeInfoArray.CopyTo(tempInfoArray, 0);
                        modeInfoArray = new CCDWrapper.DisplayConfigModeInfo[validCount];
                        int index = 0;
                        foreach (CCDWrapper.DisplayConfigModeInfo modeInfo in tempInfoArray)
                        {
                            if (modeInfo.infoType != CCDWrapper.DisplayConfigModeInfoType.Zero)
                            {
                                modeInfoArray[index] = modeInfo;
                                index++;
                            }
                        }
                    }

                    // cleanup of currently not available pathInfo elements
                    validCount = 0;
                    foreach (CCDWrapper.DisplayConfigPathInfo pathInfo in pathInfoArray)
                    {
                        if (pathInfo.targetInfo.targetAvailable)
                        {
                            validCount++;
                        }
                    }
                    if (validCount > 0)
                    {
                        CCDWrapper.DisplayConfigPathInfo[] tempInfoArray = new CCDWrapper.DisplayConfigPathInfo[pathInfoArray.Count()];
                        pathInfoArray.CopyTo(tempInfoArray, 0);
                        pathInfoArray = new CCDWrapper.DisplayConfigPathInfo[validCount];
                        int index = 0;
                        foreach (CCDWrapper.DisplayConfigPathInfo pathInfo in tempInfoArray)
                        {
                            if (pathInfo.targetInfo.targetAvailable)
                            {
                                pathInfoArray[index] = pathInfo;
                                index++;
                            }
                        }
                    }

                    // get the display names for all modes
                    for (var iMode = 0; iMode < modeInfoArray.Count(); iMode++)
                    {
                        if (modeInfoArray[iMode].infoType == CCDWrapper.DisplayConfigModeInfoType.Target)
                        {
                            try
                            {
                                additionalInfo[iMode] = CCDWrapper.GetMonitorAdditionalInfo(modeInfoArray[iMode].adapterId, modeInfoArray[iMode].id);
                            }
                            catch (Exception)
                            {
                                additionalInfo[iMode].valid = false;
                            }
                        }
                    }
                    return true;
                }
                else
                {
                    DebugOutput("Querying display config failed");
                }
            }
            else
            {
                DebugOutput("Getting Buffer Size Failed");
            }

            return false;
        }

        public static string PrintDisplaySettings(CCDWrapper.DisplayConfigPathInfo[] pathInfoArray, CCDWrapper.DisplayConfigModeInfo[] modeInfoArray)
        {
            var profile = ConvertToProfile(pathInfoArray, modeInfoArray, new CCDWrapper.MonitorAdditionalInfo[0]);
            var serializer = new JavaScriptSerializer();
            return serializer.Serialize(profile);
        }

        public static bool SaveDisplaySettings(string fileName)
        {
            CCDWrapper.DisplayConfigPathInfo[] pathInfoArray = new CCDWrapper.DisplayConfigPathInfo[0];
            CCDWrapper.DisplayConfigModeInfo[] modeInfoArray = new CCDWrapper.DisplayConfigModeInfo[0];
            CCDWrapper.MonitorAdditionalInfo[] additionalInfo = new CCDWrapper.MonitorAdditionalInfo[0];

            DebugOutput("Getting display config");
            bool status = GetDisplaySettings(ref pathInfoArray, ref modeInfoArray, ref additionalInfo, true);
            if (status)
            {
                if (debug)
                {
                    DebugOutput("Display settings to write:");
                    Console.WriteLine(PrintDisplaySettings(pathInfoArray, modeInfoArray));
                }

                try
                {
                    DebugOutput("Converting to JSON profile");
                    var profile = ConvertToProfile(pathInfoArray, modeInfoArray, additionalInfo);

                    DebugOutput("Writing JSON file");
                    var serializer = new JavaScriptSerializer();
                    string json = serializer.Serialize(profile);

                    // Format JSON for readability
                    json = FormatJson(json);

                    File.WriteAllText(fileName, json);
                    DebugOutput("Profile saved successfully");
                    return true;
                }
                catch (Exception ex)
                {
                    Console.WriteLine("Failed to save display settings: " + ex.Message);
                    DebugOutput("Exception: " + ex.ToString());
                }
            }
            else
            {
                Console.WriteLine("Failed to get display settings, ERROR: " + status.ToString());
            }

            return false;
        }

        /// <summary>
        /// Simple JSON formatter for better readability
        /// </summary>
        private static string FormatJson(string json)
        {
            var sb = new StringBuilder();
            int indent = 0;
            bool inString = false;

            foreach (char c in json)
            {
                if (c == '"' && (sb.Length == 0 || sb[sb.Length - 1] != '\\'))
                {
                    inString = !inString;
                    sb.Append(c);
                }
                else if (!inString)
                {
                    switch (c)
                    {
                        case '{':
                        case '[':
                            sb.Append(c);
                            sb.AppendLine();
                            indent++;
                            sb.Append(new string(' ', indent * 2));
                            break;
                        case '}':
                        case ']':
                            sb.AppendLine();
                            indent--;
                            sb.Append(new string(' ', indent * 2));
                            sb.Append(c);
                            break;
                        case ',':
                            sb.Append(c);
                            sb.AppendLine();
                            sb.Append(new string(' ', indent * 2));
                            break;
                        case ':':
                            sb.Append(c);
                            sb.Append(' ');
                            break;
                        default:
                            sb.Append(c);
                            break;
                    }
                }
                else
                {
                    sb.Append(c);
                }
            }

            return sb.ToString();
        }

        static void Main(string[] args)
        {
            debug = false;
            noIDMatch = false;

            bool validCommand = false;
            foreach (string iArg in args)
            {
                string[] argElements = iArg.Split(new char[] { ':' }, 2);

                switch (argElements[0].ToLower())
                {
                    case "-debug":
                        debug = true;
                        DebugOutput("\nDebug output enabled");
                        break;
                    case "-noidmatch":
                        noIDMatch = true;
                        DebugOutput("\nDisabled matching of adapter IDs");
                        break;
                    case "-save":
                        SaveDisplaySettings(argElements[1]);
                        validCommand = true;
                        break;
                    case "-load":
                        LoadDisplaySettings(argElements[1]);
                        validCommand = true;
                        break;
                    case "-print":
                        CCDWrapper.DisplayConfigPathInfo[] pathInfoArray = new CCDWrapper.DisplayConfigPathInfo[0];
                        CCDWrapper.DisplayConfigModeInfo[] modeInfoArray = new CCDWrapper.DisplayConfigModeInfo[0];
                        CCDWrapper.MonitorAdditionalInfo[] additionalInfo = new CCDWrapper.MonitorAdditionalInfo[0];

                        bool printStatus = GetDisplaySettings(ref pathInfoArray, ref modeInfoArray, ref additionalInfo, true);
                        if (printStatus)
                        {
                            Console.WriteLine(PrintDisplaySettings(pathInfoArray, modeInfoArray));
                        }
                        else
                        {
                            Console.WriteLine("Failed to get display settings");
                        }
                        validCommand = true;
                        break;
                }
            }

            if (!validCommand)
            {
                Console.WriteLine("Monitor Profile Switcher command line utility (version 0.9.0.0):\n");
                Console.WriteLine("Parameters to MonitorSwitcher.exe:");
                Console.WriteLine("\t -save:{jsonfile} \t save the current monitor configuration to file (full path)");
                Console.WriteLine("\t -load:{jsonfile} \t load and apply monitor configuration from file (full path)");
                Console.WriteLine("\t -debug \t\t enable debug output (parameter must come before -load or -save)");
                Console.WriteLine("\t -noidmatch \t\t disable matching of adapter IDs");
                Console.WriteLine("\t -print \t\t print current monitor configuration to console");
                Console.WriteLine("");
                Console.WriteLine("Examples:");
                Console.WriteLine("\tMonitorSwitcher.exe -save:MyProfile.json");
                Console.WriteLine("\tMonitorSwitcher.exe -load:MyProfile.json");
                Console.WriteLine("\tMonitorSwitcher.exe -debug -load:MyProfile.json");
                Console.ReadKey();
            }
        }
    }
}

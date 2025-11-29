import { useMemo } from 'react';
import type { MonitorDetails } from '../types';

interface MonitorDiagramProps {
  monitors: MonitorDetails[];
  maxHeight?: number;
}

// Format resolution as compact string
function formatResolution(width: number, height: number): string {
  if (width === 3840 && height === 2160) return '4K';
  if (width === 2560 && height === 1440) return '1440p';
  if (width === 1920 && height === 1080) return '1080p';
  if (width === 1280 && height === 720) return '720p';
  return `${width}×${height}`;
}

// Format refresh rate
function formatRefreshRate(rate: number): string {
  const rounded = Math.round(rate);
  return `${rounded}Hz`;
}

// Truncate name to fit
function truncateName(name: string, maxLen: number): string {
  if (name.length <= maxLen) return name;
  return name.substring(0, maxLen - 1) + '…';
}

export function MonitorDiagram({ monitors, maxHeight = 120 }: MonitorDiagramProps) {
  const layout = useMemo(() => {
    if (monitors.length === 0) return null;

    // Calculate actual dimensions considering rotation
    const monitorsWithDims = monitors.map((m) => {
      // Rotation: 1=0°, 2=90°, 3=180°, 4=270°
      const isRotated = m.rotation === 2 || m.rotation === 4;
      const actualWidth = isRotated ? m.height : m.width;
      const actualHeight = isRotated ? m.width : m.height;
      return { ...m, actualWidth, actualHeight };
    });

    // Find bounding box
    let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity;
    for (const m of monitorsWithDims) {
      minX = Math.min(minX, m.positionX);
      minY = Math.min(minY, m.positionY);
      maxX = Math.max(maxX, m.positionX + m.actualWidth);
      maxY = Math.max(maxY, m.positionY + m.actualHeight);
    }

    const totalWidth = maxX - minX;
    const totalHeight = maxY - minY;

    // Scale to fit container
    // For single monitors, cap the max size to avoid huge boxes
    const maxWidth = monitors.length === 1 ? 150 : 300;
    const scale = Math.min(maxWidth / totalWidth, maxHeight / totalHeight, 1);

    // Calculate container dimensions
    const containerWidth = totalWidth * scale;
    const containerHeight = totalHeight * scale;

    // Position each monitor with small gaps between them
    const gap = 0.5; // pixels gap between monitors
    const positioned = monitorsWithDims.map((m) => ({
      ...m,
      left: (m.positionX - minX) * scale + gap,
      top: (m.positionY - minY) * scale + gap,
      displayWidth: m.actualWidth * scale - gap * 2,
      displayHeight: m.actualHeight * scale - gap * 2,
    }));

    return { containerWidth, containerHeight, monitors: positioned };
  }, [monitors, maxHeight]);

  if (!layout || monitors.length === 0) {
    return (
      <div className="flex items-center justify-center h-16 text-slate-500 text-xs">
        No displays configured
      </div>
    );
  }

  return (
    <div className="overflow-x-auto">
      <div
        className="relative mx-auto"
        style={{ width: layout.containerWidth, height: layout.containerHeight }}
      >
        {layout.monitors.map((m, idx) => {
          // Determine if box is too small for full labels
          const isSmall = m.displayWidth < 60 || m.displayHeight < 50;
          const isTiny = m.displayWidth < 40 || m.displayHeight < 35;
          // Horizontal monitors (wider than tall) can show res/refresh inline
          const isHorizontal = m.displayWidth > m.displayHeight * 1.2;

          return (
            <div
              key={idx}
              className="absolute rounded transition-all duration-200 flex flex-col items-center justify-center overflow-hidden
              border border-slate-500/40 bg-gradient-to-br from-slate-600/80 to-slate-700/80"
              style={{
                left: m.left,
                top: m.top,
                width: m.displayWidth,
                height: m.displayHeight,
              }}
              title={`${m.name}\n${m.width}×${m.height} @ ${formatRefreshRate(m.refreshRate)}${m.isPrimary ? '\n(Primary)' : ''}`}
            >
              {!isTiny && (
                <>
                  {/* Monitor name */}
                  <span
                    className={`font-medium text-white truncate px-1 leading-tight ${isSmall ? 'text-[8px]' : 'text-[10px]'
                      }`}
                    style={{ maxWidth: m.displayWidth - 4 }}
                  >
                    {truncateName(m.name, isSmall ? 8 : 14)}
                  </span>

                  {/* Resolution and refresh rate */}
                  {isHorizontal ? (
                    <span className={`leading-tight ${isSmall ? 'text-[7px]' : 'text-[9px]'}`}>
                      <span className="text-slate-300">{formatResolution(m.width, m.height)}</span>
                      <span className="text-slate-500 mx-0.5">@</span>
                      <span className="text-emerald-400">{formatRefreshRate(m.refreshRate)}</span>
                    </span>
                  ) : (
                    <>
                      <span
                        className={`text-slate-300 leading-tight ${isSmall ? 'text-[7px]' : 'text-[9px]'}`}
                      >
                        {formatResolution(m.width, m.height)}
                      </span>
                      <span
                        className={`leading-tight text-emerald-400 ${isSmall ? 'text-[7px]' : 'text-[9px]'}`}
                      >
                        {formatRefreshRate(m.refreshRate)}
                      </span>
                    </>
                  )}

                  {/* Primary indicator */}
                  {m.isPrimary && (
                    <svg
                      className={`text-emerald-400 ${isSmall ? 'w-2 h-2 mt-0.5' : 'w-2.5 h-2.5 mt-0.5'}`}
                      fill="currentColor"
                      viewBox="0 0 24 24"
                    >
                      <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z" />
                    </svg>
                  )}
                </>
              )}
            </div>
          );
        })}
      </div>
    </div>
  );
}

import { useState } from 'react';
import type { ProfileDetails } from '../types';
import { MonitorDiagram } from './MonitorDiagram';

interface ProfileItemProps {
  profile: ProfileDetails;
  onLoad: (name: string) => Promise<void>;
  onDelete: (name: string) => Promise<void>;
}

// Get short names for summary
function getMonitorSummary(profile: ProfileDetails): string {
  const count = profile.monitors.length;
  if (count === 0) return 'No displays';

  const names = profile.monitors
    .map(m => {
      // Extract brand/short name from full name
      const parts = m.name.split(' ');
      return parts[0]; // First word is usually the brand
    })
    .join(', ');

  return `${count} monitor${count > 1 ? 's' : ''} · ${names}`;
}

export function ProfileItem({ profile, onLoad, onDelete }: ProfileItemProps) {
  const [isLoading, setIsLoading] = useState(false);
  const [isDeleting, setIsDeleting] = useState(false);

  const handleLoad = async () => {
    if (isLoading) return;
    setIsLoading(true);
    try {
      await onLoad(profile.name);
    } finally {
      setIsLoading(false);
    }
  };

  const handleDelete = async (e: React.MouseEvent) => {
    e.stopPropagation();
    setIsDeleting(true);
    try {
      await onDelete(profile.name);
    } finally {
      setIsDeleting(false);
    }
  };

  return (
    <div
      className={`group rounded-lg transition-all duration-200 cursor-pointer overflow-hidden border ${
        isLoading
          ? 'bg-blue-500/10 border-blue-500/30'
          : 'bg-slate-800/40 hover:bg-slate-700/50 border-slate-700/50 hover:border-slate-600/50'
      }`}
      onClick={handleLoad}
    >
      {/* Monitor diagram on top */}
      {profile.monitors.length > 0 && (
        <div className="px-3 pt-3 pb-2">
          <MonitorDiagram monitors={profile.monitors} maxHeight={80} />
        </div>
      )}

      {/* Profile info row at bottom */}
      <div className="flex flex-col items-center gap-0.5 px-3 py-2 border-t border-slate-700/30 relative">
        {/* Loading indicator */}
        {isLoading && (
          <div className="absolute left-3 top-1/2 -translate-y-1/2">
            <div className="w-3.5 h-3.5 border-2 border-blue-400/30 border-t-blue-400 rounded-full animate-spin" />
          </div>
        )}

        {/* Profile name and summary - centered */}
        <div className={`text-sm font-medium truncate max-w-full ${isLoading ? 'text-blue-300' : 'text-white'}`}>
          {profile.name}
        </div>
        <div className="text-[10px] text-slate-500 truncate max-w-full">
          {getMonitorSummary(profile)}
        </div>

        {/* Delete button */}
        {!isLoading && (
          <button
            onClick={handleDelete}
            disabled={isDeleting}
            className="absolute right-2 top-1/2 -translate-y-1/2 w-6 h-6 flex items-center justify-center text-slate-500 hover:text-red-400 hover:bg-red-500/10 disabled:opacity-50 rounded transition-colors opacity-0 group-hover:opacity-100"
            title="Delete"
          >
            {isDeleting ? (
              <div className="w-3 h-3 border-2 border-slate-400/30 border-t-slate-400 rounded-full animate-spin" />
            ) : (
              <svg className="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                <path strokeLinecap="round" strokeLinejoin="round" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1-1v3M4 7h16" />
              </svg>
            )}
          </button>
        )}
      </div>
    </div>
  );
}

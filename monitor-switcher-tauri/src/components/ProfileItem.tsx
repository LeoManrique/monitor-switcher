import { useState } from 'react';

interface ProfileItemProps {
  name: string;
  onLoad: (name: string) => Promise<void>;
  onDelete: (name: string) => Promise<void>;
}

export function ProfileItem({ name, onLoad, onDelete }: ProfileItemProps) {
  const [isLoading, setIsLoading] = useState(false);
  const [isDeleting, setIsDeleting] = useState(false);

  const handleLoad = async () => {
    if (isLoading) return;
    setIsLoading(true);
    try {
      await onLoad(name);
    } finally {
      setIsLoading(false);
    }
  };

  const handleDelete = async () => {
    if (!confirm(`Delete "${name}"?`)) return;

    setIsDeleting(true);
    try {
      await onDelete(name);
    } finally {
      setIsDeleting(false);
    }
  };

  return (
    <div
      className={`group flex items-center gap-2.5 px-2.5 py-2 rounded-md transition-all duration-150 cursor-pointer ${
        isLoading
          ? 'bg-blue-500/10 border border-blue-500/30'
          : 'bg-slate-800/40 hover:bg-slate-700/50 border border-transparent'
      }`}
      onClick={handleLoad}
    >
      {/* Profile icon */}
      <div className={`w-8 h-8 rounded-md flex items-center justify-center shrink-0 ${
        isLoading ? 'bg-blue-500/20' : 'bg-slate-700/60'
      }`}>
        {isLoading ? (
          <div className="w-3.5 h-3.5 border-2 border-blue-400/30 border-t-blue-400 rounded-full animate-spin" />
        ) : (
          <svg className="w-4 h-4 text-blue-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={1.5}>
            <path strokeLinecap="round" strokeLinejoin="round" d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
          </svg>
        )}
      </div>

      {/* Profile name */}
      <span className={`text-sm font-medium truncate flex-1 ${isLoading ? 'text-blue-300' : 'text-white'}`}>
        {name}
      </span>

      {/* Delete button */}
      {!isLoading && (
        <button
          onClick={(e) => { e.stopPropagation(); handleDelete(); }}
          disabled={isDeleting}
          className="w-6 h-6 flex items-center justify-center text-slate-500 hover:text-red-400 hover:bg-red-500/10 disabled:opacity-50 rounded transition-colors opacity-0 group-hover:opacity-100"
          title="Delete"
        >
          {isDeleting ? (
            <div className="w-3 h-3 border-2 border-slate-400/30 border-t-slate-400 rounded-full animate-spin" />
          ) : (
            <svg className="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
              <path strokeLinecap="round" strokeLinejoin="round" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1-1v3M4 7h16" />
            </svg>
          )}
        </button>
      )}
    </div>
  );
}

import { useState, useEffect, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow } from '@tauri-apps/api/window';

function SavePopup() {
  const [name, setName] = useState('');
  const [isSaving, setIsSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [existingProfiles, setExistingProfiles] = useState<string[]>([]);
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    // Focus input on mount
    setTimeout(() => inputRef.current?.focus(), 50);

    // Load existing profiles
    invoke<string[]>('list_profiles')
      .then(setExistingProfiles)
      .catch(console.error);
  }, []);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    const trimmedName = name.trim();
    if (!trimmedName) {
      setError('Please enter a profile name');
      return;
    }

    setIsSaving(true);
    setError(null);

    try {
      await invoke('save_profile', { name: trimmedName });
      await getCurrentWindow().close();
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
      setIsSaving(false);
    }
  };

  const handleCancel = async () => {
    await getCurrentWindow().close();
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Escape') {
      handleCancel();
    }
  };

  const selectProfile = (profileName: string) => {
    setName(profileName);
    inputRef.current?.focus();
  };

  const isOverwrite = existingProfiles.some(
    (p) => p.toLowerCase() === name.trim().toLowerCase()
  );

  return (
    <div
      className="h-screen bg-gradient-to-br from-slate-900 via-slate-800 to-slate-900 flex flex-col overflow-hidden rounded-lg border border-slate-700/50"
      onKeyDown={handleKeyDown}
    >
      {/* Draggable title bar */}
      <div
        data-tauri-drag-region
        className="h-10 bg-slate-800/50 flex items-center justify-between pl-3 pr-1 border-b border-slate-700/50 cursor-move shrink-0"
      >
        <div className="flex items-center gap-2" data-tauri-drag-region>
          <div className="w-6 h-6 rounded-md bg-gradient-to-br from-emerald-500 to-emerald-600 flex items-center justify-center">
            <svg className="w-3.5 h-3.5 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2.5}>
              <path strokeLinecap="round" strokeLinejoin="round" d="M12 4v16m8-8H4" />
            </svg>
          </div>
          <span className="text-sm font-medium text-slate-200 select-none" data-tauri-drag-region>
            Save Profile
          </span>
        </div>
        <button
          onClick={handleCancel}
          className="w-10 h-9 flex items-center justify-center text-slate-400 hover:text-white hover:bg-red-500 transition-colors"
        >
          <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
            <path strokeLinecap="round" strokeLinejoin="round" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      {/* Content */}
      <form onSubmit={handleSubmit} className="flex-1 p-3 flex flex-col min-h-0">
        {/* Input section */}
        <div>
          <label htmlFor="profileName" className="block text-[11px] font-medium text-slate-500 uppercase tracking-wider mb-1.5">
            Profile Name
          </label>
          <input
            ref={inputRef}
            id="profileName"
            type="text"
            value={name}
            onChange={(e) => setName(e.target.value)}
            placeholder="Enter profile name..."
            autoComplete="off"
            className="w-full px-2.5 py-2 bg-slate-800/60 text-white text-sm rounded-lg border border-slate-600/50 focus:border-blue-500 focus:ring-1 focus:ring-blue-500/30 focus:outline-none transition-all placeholder:text-slate-500"
          />
          {error && (
            <p className="mt-1.5 text-[11px] text-red-400 flex items-center gap-1">
              <svg className="w-3 h-3 shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                <path strokeLinecap="round" strokeLinejoin="round" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
              {error}
            </p>
          )}
        </div>

        {/* Existing profiles list */}
        {existingProfiles.length > 0 && (
          <div className="mt-3 flex-1 min-h-0 flex flex-col">
            <div className="flex items-center justify-between mb-2">
              <span className="text-[11px] font-medium text-slate-500 uppercase tracking-wider">
                Existing Profiles
              </span>
              <span className="text-[10px] text-slate-500 bg-slate-800/80 px-1.5 py-0.5 rounded">
                {existingProfiles.length}
              </span>
            </div>
            <div className="flex-1 overflow-y-auto">
              {existingProfiles.map((profile) => {
                const isSelected = profile.toLowerCase() === name.trim().toLowerCase();
                return (
                  <button
                    key={profile}
                    type="button"
                    onClick={() => selectProfile(profile)}
                    className={`w-full px-2 py-1.5 text-left text-sm flex items-center gap-1.5 transition-colors ${
                      isSelected
                        ? 'bg-blue-500/15 text-blue-300'
                        : 'hover:bg-slate-700/50 text-slate-300'
                    }`}
                  >
                    <svg className={`w-3.5 h-3.5 shrink-0 ${isSelected ? 'text-blue-400' : 'text-slate-500'}`} fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={1.5}>
                      <path strokeLinecap="round" strokeLinejoin="round" d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
                    </svg>
                    <span className="truncate flex-1 min-w-0">{profile}</span>
                    {isSelected && (
                      <span className="text-[9px] text-amber-400 shrink-0">overwrite</span>
                    )}
                  </button>
                );
              })}
            </div>
          </div>
        )}

        {/* Buttons */}
        <div className="flex gap-2 justify-end pt-3 mt-3 border-t border-slate-700/50">
          <button
            type="button"
            onClick={handleCancel}
            disabled={isSaving}
            className="px-3 py-1.5 text-xs font-medium text-slate-300 hover:text-white bg-slate-700/50 hover:bg-slate-600/50 rounded-md border border-slate-600/50 transition-colors disabled:opacity-50"
          >
            Cancel
          </button>
          <button
            type="submit"
            disabled={isSaving || !name.trim()}
            className={`flex items-center gap-1.5 px-4 py-1.5 text-white text-xs font-medium rounded-md transition-all disabled:bg-slate-700 disabled:text-slate-500 disabled:cursor-not-allowed ${
              isOverwrite
                ? 'bg-amber-600 hover:bg-amber-500'
                : 'bg-emerald-600 hover:bg-emerald-500'
            }`}
          >
            {isSaving ? (
              <>
                <div className="w-3 h-3 border-2 border-white/30 border-t-white rounded-full animate-spin" />
                Saving...
              </>
            ) : (
              <>
                <svg className="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                  <path strokeLinecap="round" strokeLinejoin="round" d="M5 13l4 4L19 7" />
                </svg>
                {isOverwrite ? 'Overwrite' : 'Save'}
              </>
            )}
          </button>
        </div>
      </form>
    </div>
  );
}

export default SavePopup;

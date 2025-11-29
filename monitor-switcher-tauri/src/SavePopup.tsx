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

    // Load existing profiles for duplicate check
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

    if (existingProfiles.includes(trimmedName)) {
      if (!confirm(`Profile "${trimmedName}" already exists. Overwrite?`)) {
        return;
      }
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

  return (
    <div
      className="h-screen bg-gradient-to-br from-slate-900 via-slate-800 to-slate-900 flex flex-col overflow-hidden"
      onKeyDown={handleKeyDown}
    >
      {/* Draggable title bar */}
      <div
        data-tauri-drag-region
        className="h-9 bg-slate-800/50 flex items-center justify-between pl-3 pr-1 border-b border-slate-700/50 cursor-move shrink-0"
      >
        <div className="flex items-center gap-2" data-tauri-drag-region>
          <div className="w-5 h-5 rounded bg-gradient-to-br from-emerald-500 to-emerald-600 flex items-center justify-center">
            <svg className="w-3 h-3 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2.5}>
              <path strokeLinecap="round" strokeLinejoin="round" d="M12 4v16m8-8H4" />
            </svg>
          </div>
          <span className="text-xs text-slate-200 font-medium select-none" data-tauri-drag-region>
            Save Profile
          </span>
        </div>
        <button
          onClick={handleCancel}
          className="w-7 h-7 flex items-center justify-center text-slate-400 hover:text-white hover:bg-red-500 rounded transition-all duration-150"
        >
          <svg className="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
            <path strokeLinecap="round" strokeLinejoin="round" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      {/* Content */}
      <form onSubmit={handleSubmit} className="p-3 pb-4 flex flex-col gap-3">
        <div>
          <label htmlFor="profileName" className="block text-[11px] font-medium text-slate-400 mb-1">
            Profile Name
          </label>
          <input
            ref={inputRef}
            id="profileName"
            type="text"
            value={name}
            onChange={(e) => setName(e.target.value)}
            placeholder="Enter profile name..."
            className="w-full px-2.5 py-2 bg-slate-800/80 text-white text-sm rounded-md border border-slate-600/50 focus:border-blue-500 focus:ring-1 focus:ring-blue-500/30 focus:outline-none transition-all placeholder:text-slate-500"
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

        <div className="flex gap-2 justify-end pt-2 border-t border-slate-700/50">
          <button
            type="button"
            onClick={handleCancel}
            disabled={isSaving}
            className="px-3 py-1.5 text-xs text-slate-300 hover:text-white hover:bg-slate-700/50 rounded-md transition-colors disabled:opacity-50"
          >
            Cancel
          </button>
          <button
            type="submit"
            disabled={isSaving || !name.trim()}
            className="flex items-center gap-1 px-3 py-1.5 bg-blue-600 hover:bg-blue-500 disabled:bg-slate-700 disabled:text-slate-500 disabled:cursor-not-allowed text-white text-xs font-medium rounded-md transition-all"
          >
            {isSaving ? (
              <>
                <div className="w-3 h-3 border-2 border-white/30 border-t-white rounded-full animate-spin" />
                Saving...
              </>
            ) : (
              <>
                <svg className="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                  <path strokeLinecap="round" strokeLinejoin="round" d="M5 13l4 4L19 7" />
                </svg>
                Save
              </>
            )}
          </button>
        </div>
      </form>
    </div>
  );
}

export default SavePopup;

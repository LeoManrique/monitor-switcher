import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { ProfileList } from './components/ProfileList';
import { useProfiles } from './hooks/useProfiles';
import './App.css';

function App() {
  const {
    profiles,
    activeProfile,
    isLoading,
    error,
    loadProfile,
    deleteProfile,
    turnOffMonitors,
    refresh,
  } = useProfiles();

  const appWindow = getCurrentWindow();

  // Refresh profiles when window gains focus (after popup closes)
  useEffect(() => {
    const unlisten = appWindow.onFocusChanged(({ payload: focused }) => {
      if (focused) {
        refresh();
      }
    });

    return () => {
      unlisten.then(fn => fn());
    };
  }, [refresh]);

  const [notification, setNotification] = useState<{ type: 'success' | 'error'; message: string } | null>(null);

  const showNotification = (type: 'success' | 'error', message: string) => {
    setNotification({ type, message });
    setTimeout(() => setNotification(null), 3000);
  };

  const handleLoad = async (name: string) => {
    try {
      await loadProfile(name);
      showNotification('success', `Loaded "${name}"`);
    } catch (err) {
      showNotification('error', err instanceof Error ? err.message : String(err));
    }
  };

  const handleDelete = async (name: string) => {
    try {
      await deleteProfile(name);
      showNotification('success', `Deleted "${name}"`);
    } catch (err) {
      showNotification('error', err instanceof Error ? err.message : String(err));
    }
  };

  const handleOpenSaveDialog = async () => {
    try {
      await invoke('open_save_dialog');
    } catch (err) {
      showNotification('error', err instanceof Error ? err.message : String(err));
    }
  };

  const handleTurnOff = async () => {
    try {
      await turnOffMonitors();
    } catch (err) {
      showNotification('error', err instanceof Error ? err.message : String(err));
    }
  };

  return (
    <div className="h-screen bg-gradient-to-br from-slate-900 via-slate-800 to-slate-900 flex flex-col rounded-lg overflow-hidden border border-slate-700/50">
      {/* Title bar */}
      <header
        data-tauri-drag-region
        className="h-10 bg-slate-800/50 flex items-center justify-between pl-3 pr-1 border-b border-slate-700/50 shrink-0"
      >
        <div className="flex items-center gap-2" data-tauri-drag-region>
          <div className="w-6 h-6 rounded-md bg-gradient-to-br from-blue-500 to-purple-600 flex items-center justify-center">
            <svg className="w-3.5 h-3.5 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
              <path strokeLinecap="round" strokeLinejoin="round" d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
            </svg>
          </div>
          <span className="text-sm font-medium text-slate-200 select-none" data-tauri-drag-region>
            Monitor Switcher
          </span>
        </div>

        {/* Window controls */}
        <div className="flex">
          <button
            onClick={() => appWindow.minimize()}
            className="w-10 h-9 flex items-center justify-center text-slate-400 hover:text-white hover:bg-slate-700/50 transition-colors"
          >
            <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
              <path strokeLinecap="round" strokeLinejoin="round" d="M20 12H4" />
            </svg>
          </button>
          <button
            onClick={() => appWindow.toggleMaximize()}
            className="w-10 h-9 flex items-center justify-center text-slate-400 hover:text-white hover:bg-slate-700/50 transition-colors"
          >
            <svg className="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
              <rect x="4" y="4" width="16" height="16" rx="1" />
            </svg>
          </button>
          <button
            onClick={() => appWindow.close()}
            className="w-10 h-9 flex items-center justify-center text-slate-400 hover:text-white hover:bg-red-500 transition-colors"
          >
            <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
              <path strokeLinecap="round" strokeLinejoin="round" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>
      </header>

      {/* Main content */}
      <main className="flex-1 p-3 flex flex-col gap-3 overflow-hidden">
        {/* Quick Actions - smaller, inline */}
        <div className="flex gap-2">
          <button
            onClick={handleOpenSaveDialog}
            className="flex items-center gap-1.5 px-3 py-1.5 bg-emerald-600 hover:bg-emerald-500 text-white text-xs font-medium rounded-md transition-colors"
          >
            <svg className="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2.5}>
              <path strokeLinecap="round" strokeLinejoin="round" d="M12 4v16m8-8H4" />
            </svg>
            Save Current
          </button>

          <button
            onClick={handleTurnOff}
            className="flex items-center gap-1.5 px-3 py-1.5 bg-slate-700/80 hover:bg-slate-600/80 text-slate-300 hover:text-white text-xs font-medium rounded-md border border-slate-600/50 transition-colors"
          >
            <svg className="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
              <path strokeLinecap="round" strokeLinejoin="round" d="M5.636 5.636a9 9 0 1012.728 0M12 3v9" />
            </svg>
            Turn Off
          </button>
        </div>

        {/* Error display */}
        {error && (
          <div className="px-2.5 py-2 bg-red-500/10 border border-red-500/30 rounded-md flex items-center gap-2">
            <svg className="w-3.5 h-3.5 text-red-400 shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
              <path strokeLinecap="round" strokeLinejoin="round" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
            <span className="text-red-200 text-xs">{error}</span>
          </div>
        )}

        {/* Profiles Section */}
        <div className="flex-1 flex flex-col min-h-0">
          <div className="flex items-center justify-between mb-2">
            <h2 className="text-[11px] font-medium text-slate-500 uppercase tracking-wider">Profiles</h2>
            <span className="text-[10px] text-slate-500 bg-slate-800/80 px-1.5 py-0.5 rounded">
              {profiles.length}
            </span>
          </div>
          <div className="flex-1 overflow-y-auto">
            <ProfileList
              profiles={profiles}
              activeProfile={activeProfile}
              onLoad={handleLoad}
              onDelete={handleDelete}
              isLoading={isLoading}
            />
          </div>
        </div>
      </main>

      {/* Notification toast */}
      {notification && (
        <div
          className={`fixed bottom-3 left-3 right-3 flex items-center justify-center gap-2 px-3 py-2 rounded-md shadow-xl transition-all duration-300 animate-slide-up ${
            notification.type === 'success'
              ? 'bg-emerald-600 text-white'
              : 'bg-red-600 text-white'
          }`}
        >
          {notification.type === 'success' ? (
            <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
              <path strokeLinecap="round" strokeLinejoin="round" d="M5 13l4 4L19 7" />
            </svg>
          ) : (
            <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
              <path strokeLinecap="round" strokeLinejoin="round" d="M6 18L18 6M6 6l12 12" />
            </svg>
          )}
          <span className="text-sm font-medium">{notification.message}</span>
        </div>
      )}
    </div>
  );
}

export default App;

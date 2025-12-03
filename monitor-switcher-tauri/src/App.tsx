import { useState, useEffect, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window';
import { ProfileList } from './components/ProfileList';
import { useProfiles } from './hooks/useProfiles';
import './App.css';

const WINDOW_STATE_KEY = 'monitor-switcher-window-state';

interface WindowState {
  width: number;
  height: number;
}

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
  const saveTimeoutRef = useRef<number | null>(null);

  // Restore window size on mount
  useEffect(() => {
    const restoreWindowSize = async () => {
      try {
        const saved = localStorage.getItem(WINDOW_STATE_KEY);
        if (saved) {
          const state: WindowState = JSON.parse(saved);
          if (state.width > 0 && state.height > 0) {
            await appWindow.setSize(new LogicalSize(state.width, state.height));
          }
        }
      } catch (e) {
        console.error('Failed to restore window size:', e);
      }
    };
    restoreWindowSize();
  }, []);

  // Save window size on resize (debounced)
  useEffect(() => {
    const unlisten = appWindow.onResized(({ payload: size }) => {
      // Debounce saves to avoid excessive writes
      if (saveTimeoutRef.current) {
        clearTimeout(saveTimeoutRef.current);
      }
      saveTimeoutRef.current = setTimeout(() => {
        const state: WindowState = {
          width: size.width,
          height: size.height,
        };
        localStorage.setItem(WINDOW_STATE_KEY, JSON.stringify(state));
      }, 500);
    });

    return () => {
      unlisten.then(fn => fn());
      if (saveTimeoutRef.current) {
        clearTimeout(saveTimeoutRef.current);
      }
    };
  }, []);

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
    <div className="h-screen w-screen bg-gradient-to-br from-slate-900 via-slate-800 to-slate-900 flex flex-col rounded-lg border border-slate-700/50" style={{ position: 'fixed', top: 0, left: 0 }}>
      {/* Title bar */}
      <header className="h-10 bg-slate-800/50 flex items-center justify-between pl-3 pr-1 border-b border-slate-700/50 shrink-0">
        {/* Left side - draggable area */}
        <div data-tauri-drag-region className="flex items-center gap-2 flex-1">
          <div className="w-6 h-6 rounded-md bg-gradient-to-br from-blue-500 to-purple-600 flex items-center justify-center">
            <svg className="w-3.5 h-3.5 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
              <path strokeLinecap="round" strokeLinejoin="round" d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
            </svg>
          </div>
          <span className="text-sm font-medium text-slate-200 select-none">
            Monitor Switcher
          </span>
        </div>

        {/* Window controls - NOT in drag region for hover to work */}
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
            <svg className="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={1.5}>
              <path strokeLinecap="round" strokeLinejoin="round" d="M9 17.25v1.007a3 3 0 01-.879 2.122L7.5 21h9l-.621-.621A3 3 0 0115 18.257V17.25m6-12V15a2.25 2.25 0 01-2.25 2.25H5.25A2.25 2.25 0 013 15V5.25A2.25 2.25 0 015.25 3h13.5A2.25 2.25 0 0121 5.25z" />
              <path strokeLinecap="round" strokeLinejoin="round" d="M3.5 3.5l17 17" strokeWidth={2} />
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
          <h2 className="text-[11px] font-medium text-slate-500 uppercase tracking-wider mb-2">Profiles</h2>
          <div className="flex-1 overflow-y-auto overflow-x-hidden">
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

import { useState } from 'react';
import { ProfileList } from './components/ProfileList';
import { SaveDialog } from './components/SaveDialog';
import { useProfiles } from './hooks/useProfiles';

function App() {
  const {
    profiles,
    isLoading,
    error,
    saveProfile,
    loadProfile,
    deleteProfile,
    turnOffMonitors,
  } = useProfiles();

  const [showSaveDialog, setShowSaveDialog] = useState(false);
  const [notification, setNotification] = useState<{ type: 'success' | 'error'; message: string } | null>(null);

  const showNotification = (type: 'success' | 'error', message: string) => {
    setNotification({ type, message });
    setTimeout(() => setNotification(null), 3000);
  };

  const handleLoad = async (name: string) => {
    try {
      await loadProfile(name);
      showNotification('success', `Profile "${name}" loaded`);
    } catch (err) {
      showNotification('error', err instanceof Error ? err.message : 'Failed to load profile');
    }
  };

  const handleDelete = async (name: string) => {
    try {
      await deleteProfile(name);
      showNotification('success', `Profile "${name}" deleted`);
    } catch (err) {
      showNotification('error', err instanceof Error ? err.message : 'Failed to delete profile');
    }
  };

  const handleSave = async (name: string) => {
    try {
      await saveProfile(name);
      setShowSaveDialog(false);
      showNotification('success', `Profile "${name}" saved`);
    } catch (err) {
      throw err; // Let the dialog handle the error
    }
  };

  const handleTurnOff = async () => {
    try {
      await turnOffMonitors();
    } catch (err) {
      showNotification('error', err instanceof Error ? err.message : 'Failed to turn off monitors');
    }
  };

  return (
    <div className="min-h-screen bg-gray-900 p-6">
      {/* Header */}
      <div className="mb-6">
        <h1 className="text-2xl font-bold text-white mb-2">Monitor Switcher</h1>
        <p className="text-gray-400 text-sm">Save and restore display configurations</p>
      </div>

      {/* Actions */}
      <div className="flex gap-3 mb-6">
        <button
          onClick={() => setShowSaveDialog(true)}
          className="px-4 py-2 bg-green-600 hover:bg-green-500 text-white rounded transition-colors"
        >
          Save Current
        </button>
        <button
          onClick={handleTurnOff}
          className="px-4 py-2 bg-gray-700 hover:bg-gray-600 text-white rounded transition-colors"
        >
          Turn Off Monitors
        </button>
      </div>

      {/* Error display */}
      {error && (
        <div className="mb-4 p-3 bg-red-900/50 border border-red-700 rounded text-red-200">
          {error}
        </div>
      )}

      {/* Profile list */}
      <div className="bg-gray-800 rounded-lg p-4">
        <h2 className="text-lg font-semibold text-white mb-4">Saved Profiles</h2>
        <ProfileList
          profiles={profiles}
          onLoad={handleLoad}
          onDelete={handleDelete}
          isLoading={isLoading}
        />
      </div>

      {/* Save dialog */}
      {showSaveDialog && (
        <SaveDialog
          onSave={handleSave}
          onCancel={() => setShowSaveDialog(false)}
          existingProfiles={profiles}
        />
      )}

      {/* Notification toast */}
      {notification && (
        <div
          className={`fixed bottom-4 right-4 px-4 py-3 rounded-lg shadow-lg transition-opacity ${
            notification.type === 'success'
              ? 'bg-green-600 text-white'
              : 'bg-red-600 text-white'
          }`}
        >
          {notification.message}
        </div>
      )}
    </div>
  );
}

export default App;

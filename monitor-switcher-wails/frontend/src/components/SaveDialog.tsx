import { useState } from 'react';

interface SaveDialogProps {
  onSave: (name: string) => Promise<void>;
  onCancel: () => void;
  existingProfiles: string[];
}

export function SaveDialog({ onSave, onCancel, existingProfiles }: SaveDialogProps) {
  const [name, setName] = useState('');
  const [isSaving, setIsSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);

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
      await onSave(trimmedName);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to save profile');
      setIsSaving(false);
    }
  };

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center p-4">
      <div className="bg-gray-800 rounded-lg p-6 w-full max-w-md">
        <h2 className="text-xl font-semibold text-white mb-4">Save Profile</h2>

        <form onSubmit={handleSubmit}>
          <div className="mb-4">
            <label htmlFor="profileName" className="block text-sm text-gray-300 mb-2">
              Profile Name
            </label>
            <input
              id="profileName"
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder="Enter profile name..."
              autoFocus
              className="w-full px-3 py-2 bg-gray-700 text-white rounded border border-gray-600 focus:border-blue-500 focus:outline-none"
            />
            {error && <p className="mt-2 text-sm text-red-400">{error}</p>}
          </div>

          <div className="flex gap-3 justify-end">
            <button
              type="button"
              onClick={onCancel}
              disabled={isSaving}
              className="px-4 py-2 text-gray-300 hover:text-white transition-colors"
            >
              Cancel
            </button>
            <button
              type="submit"
              disabled={isSaving}
              className="px-4 py-2 bg-blue-600 hover:bg-blue-500 disabled:bg-blue-800 disabled:cursor-not-allowed text-white rounded transition-colors"
            >
              {isSaving ? 'Saving...' : 'Save'}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}

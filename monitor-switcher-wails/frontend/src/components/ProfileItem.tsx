import { useState } from 'react';

interface ProfileItemProps {
  name: string;
  onLoad: (name: string) => void;
  onDelete: (name: string) => void;
}

export function ProfileItem({ name, onLoad, onDelete }: ProfileItemProps) {
  const [isDeleting, setIsDeleting] = useState(false);
  const [isLoading, setIsLoading] = useState(false);

  const handleLoad = async () => {
    setIsLoading(true);
    try {
      await onLoad(name);
    } finally {
      setIsLoading(false);
    }
  };

  const handleDelete = async () => {
    if (!confirm(`Delete profile "${name}"?`)) return;

    setIsDeleting(true);
    try {
      await onDelete(name);
    } finally {
      setIsDeleting(false);
    }
  };

  return (
    <div className="flex items-center justify-between p-3 bg-gray-700 rounded-lg hover:bg-gray-600 transition-colors">
      <span className="text-white font-medium truncate flex-1 mr-4">{name}</span>
      <div className="flex gap-2 shrink-0">
        <button
          onClick={handleLoad}
          disabled={isLoading}
          className="px-3 py-1.5 bg-blue-600 hover:bg-blue-500 disabled:bg-blue-800 disabled:cursor-not-allowed text-white text-sm rounded transition-colors"
        >
          {isLoading ? 'Loading...' : 'Load'}
        </button>
        <button
          onClick={handleDelete}
          disabled={isDeleting}
          className="px-3 py-1.5 bg-red-600 hover:bg-red-500 disabled:bg-red-800 disabled:cursor-not-allowed text-white text-sm rounded transition-colors"
        >
          {isDeleting ? '...' : 'Delete'}
        </button>
      </div>
    </div>
  );
}

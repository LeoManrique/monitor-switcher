import { useState, useEffect, useCallback } from 'react';
import { ListProfiles, SaveProfile, LoadProfile, DeleteProfile, TurnOffMonitors } from '../../wailsjs/go/main/App';

export function useProfiles() {
  const [profiles, setProfiles] = useState<string[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const refresh = useCallback(async () => {
    try {
      const profileList = await ListProfiles();
      setProfiles(profileList || []);
      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load profiles');
      setProfiles([]);
    } finally {
      setIsLoading(false);
    }
  }, []);

  useEffect(() => {
    refresh();
  }, [refresh]);

  const saveProfile = useCallback(async (name: string) => {
    await SaveProfile(name);
    await refresh();
  }, [refresh]);

  const loadProfile = useCallback(async (name: string) => {
    await LoadProfile(name);
  }, []);

  const deleteProfile = useCallback(async (name: string) => {
    await DeleteProfile(name);
    await refresh();
  }, [refresh]);

  const turnOffMonitors = useCallback(async () => {
    await TurnOffMonitors();
  }, []);

  return {
    profiles,
    isLoading,
    error,
    refresh,
    saveProfile,
    loadProfile,
    deleteProfile,
    turnOffMonitors,
  };
}

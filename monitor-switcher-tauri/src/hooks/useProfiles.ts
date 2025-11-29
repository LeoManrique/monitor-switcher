import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import type { ProfileDetails } from '../types';

export function useProfiles() {
  const [profiles, setProfiles] = useState<ProfileDetails[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const refresh = useCallback(async () => {
    try {
      const profileList = await invoke<ProfileDetails[]>('list_profiles_with_details');
      setProfiles(profileList || []);
      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
      setProfiles([]);
    } finally {
      setIsLoading(false);
    }
  }, []);

  useEffect(() => {
    refresh();
  }, [refresh]);

  const saveProfile = useCallback(async (name: string) => {
    await invoke('save_profile', { name });
    await refresh();
  }, [refresh]);

  const loadProfile = useCallback(async (name: string) => {
    await invoke('load_profile', { name });
  }, []);

  const deleteProfile = useCallback(async (name: string) => {
    await invoke('delete_profile', { name });
    await refresh();
  }, [refresh]);

  const turnOffMonitors = useCallback(async () => {
    await invoke('turn_off_monitors');
  }, []);

  const profileExists = useCallback(async (name: string): Promise<boolean> => {
    return await invoke('profile_exists', { name });
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
    profileExists,
  };
}

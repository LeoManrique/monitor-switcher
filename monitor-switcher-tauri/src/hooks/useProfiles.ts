import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type { ProfileDetails, MonitorDetails } from '../types';

// Compare two monitor configurations to see if they match
function monitorsMatch(a: MonitorDetails[], b: MonitorDetails[]): boolean {
  if (a.length !== b.length) return false;

  // Sort both arrays by position for consistent comparison
  const sortByPos = (m: MonitorDetails) => `${m.positionX},${m.positionY}`;
  const sortedA = [...a].sort((x, y) => sortByPos(x).localeCompare(sortByPos(y)));
  const sortedB = [...b].sort((x, y) => sortByPos(x).localeCompare(sortByPos(y)));

  for (let i = 0; i < sortedA.length; i++) {
    const ma = sortedA[i];
    const mb = sortedB[i];

    // Compare key properties (allow small refresh rate tolerance)
    if (
      ma.width !== mb.width ||
      ma.height !== mb.height ||
      ma.positionX !== mb.positionX ||
      ma.positionY !== mb.positionY ||
      ma.rotation !== mb.rotation ||
      Math.abs(ma.refreshRate - mb.refreshRate) > 1
    ) {
      return false;
    }
  }

  return true;
}

export function useProfiles() {
  const [profiles, setProfiles] = useState<ProfileDetails[]>([]);
  const [activeProfile, setActiveProfile] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const refresh = useCallback(async () => {
    try {
      const [profileList, currentMonitors] = await Promise.all([
        invoke<ProfileDetails[]>('list_profiles_with_details'),
        invoke<MonitorDetails[]>('get_current_monitors'),
      ]);

      setProfiles(profileList || []);

      // Find matching profile
      const matchingProfile = (profileList || []).find(
        (p) => monitorsMatch(p.monitors, currentMonitors)
      );
      setActiveProfile(matchingProfile?.name || null);

      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
      setProfiles([]);
      setActiveProfile(null);
    } finally {
      setIsLoading(false);
    }
  }, []);

  useEffect(() => {
    refresh();

    // Listen for profile-changed events (from tray menu or other sources)
    const unlisten = listen('profile-changed', () => {
      // Small delay to let Windows apply display changes
      setTimeout(() => refresh(), 500);
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, [refresh]);

  const saveProfile = useCallback(async (name: string) => {
    await invoke('save_profile', { name });
    await refresh();
  }, [refresh]);

  const loadProfile = useCallback(async (name: string) => {
    await invoke('load_profile', { name });
    // Small delay to let Windows apply display changes, then refresh to update active state
    setTimeout(() => refresh(), 500);
  }, [refresh]);

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
    activeProfile,
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

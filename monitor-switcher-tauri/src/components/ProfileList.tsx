import { ProfileItem } from './ProfileItem';
import type { ProfileDetails } from '../types';

interface ProfileListProps {
  profiles: ProfileDetails[];
  onLoad: (name: string) => Promise<void>;
  onDelete: (name: string) => Promise<void>;
  isLoading: boolean;
}

export function ProfileList({ profiles, onLoad, onDelete, isLoading }: ProfileListProps) {
  if (isLoading) {
    return (
      <div className="flex flex-col items-center justify-center py-8 text-slate-400">
        <div className="w-6 h-6 border-2 border-slate-600 border-t-blue-500 rounded-full animate-spin mb-2" />
        <span className="text-xs">Loading...</span>
      </div>
    );
  }

  if (profiles.length === 0) {
    return (
      <div className="flex flex-col items-center justify-center py-8 text-center">
        <div className="w-10 h-10 rounded-xl bg-slate-700/50 flex items-center justify-center mb-3">
          <svg className="w-5 h-5 text-slate-500" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={1.5}>
            <path strokeLinecap="round" strokeLinejoin="round" d="M19.5 14.25v-2.625a3.375 3.375 0 00-3.375-3.375h-1.5A1.125 1.125 0 0113.5 7.125v-1.5a3.375 3.375 0 00-3.375-3.375H8.25m3.75 9v6m3-3H9m1.5-12H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 00-9-9z" />
          </svg>
        </div>
        <p className="text-slate-400 text-xs mb-0.5">No profiles saved</p>
        <p className="text-slate-500 text-[10px]">Click "Save Current" to create one</p>
      </div>
    );
  }

  return (
    <div className="grid grid-cols-2 gap-2">
      {profiles.map((profile) => (
        <ProfileItem
          key={profile.name}
          profile={profile}
          onLoad={onLoad}
          onDelete={onDelete}
        />
      ))}
    </div>
  );
}

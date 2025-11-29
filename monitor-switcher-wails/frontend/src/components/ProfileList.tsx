import { ProfileItem } from './ProfileItem';

interface ProfileListProps {
  profiles: string[];
  onLoad: (name: string) => void;
  onDelete: (name: string) => void;
  isLoading: boolean;
}

export function ProfileList({ profiles, onLoad, onDelete, isLoading }: ProfileListProps) {
  if (isLoading) {
    return (
      <div className="flex items-center justify-center py-8 text-gray-400">
        Loading profiles...
      </div>
    );
  }

  if (profiles.length === 0) {
    return (
      <div className="flex items-center justify-center py-8 text-gray-400">
        No profiles saved yet. Save your current display configuration to get started.
      </div>
    );
  }

  return (
    <div className="flex flex-col gap-2">
      {profiles.map((name) => (
        <ProfileItem
          key={name}
          name={name}
          onLoad={onLoad}
          onDelete={onDelete}
        />
      ))}
    </div>
  );
}

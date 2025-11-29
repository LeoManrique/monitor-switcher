export interface MonitorDetails {
  name: string;
  width: number;
  height: number;
  refreshRate: number;
  positionX: number;
  positionY: number;
  rotation: number; // 1=0°, 2=90°, 3=180°, 4=270°
  isPrimary: boolean;
}

export interface ProfileDetails {
  name: string;
  monitors: MonitorDetails[];
}

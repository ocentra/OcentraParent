import type { CSSProperties } from 'react';
import type { DeviceChoiceGridConfig } from './DeviceChoiceGridConfig';

export type DeviceStatus = 'connected' | 'available' | 'offline' | 'unsupported' | 'empty';
export type ScopeValue = 'lan' | 'parent';
export type DeviceKind = 'mobile' | 'desktop' | 'laptop' | 'tablet' | 'router' | 'unknown';
export type DevicePlatformKind = 'windows' | 'macos' | 'linux' | 'android' | 'ios' | 'router' | 'unknown';
export type SelectableDeviceStatus = Exclude<DeviceStatus, 'empty'>;

export type LanDevice = {
  id: string;
  name: string;
  ip?: string;
  mac?: string;
  type?: DeviceKind;
  platform?: DevicePlatformKind;
  status?: SelectableDeviceStatus;
};

export type DeviceSlot = {
  value: string;
  label: string;
  status: DeviceStatus;
  device?: LanDevice;
  platform?: DevicePlatformKind;
  badge?: string;
  slotIndex: number;
};

export type DeepPartial<T> = T extends readonly (infer U)[]
  ? readonly U[]
  : T extends (...args: never[]) => unknown
    ? T
    : T extends object
      ? { [K in keyof T]?: DeepPartial<T[K]> }
      : T;

export type DeviceChoiceGridProps = {
  value?: string;
  defaultValue?: string;
  scope?: ScopeValue;
  defaultScope?: ScopeValue;
  portalDeviceIds?: string[];
  defaultPortalDeviceIds?: string[];
  devices?: LanDevice[];
  slots?: DeviceSlot[];
  options?: DeviceSlot[];
  rows?: number;
  columns?: number;
  parentRows?: number;
  parentColumns?: number;
  disabled?: boolean;
  deviceSelectionDisabled?: boolean;
  showScopeSelector?: boolean;
  className?: string;
  style?: CSSProperties;
  onChange?: (choice: DeviceSlot, index: number, row: number, column: number) => void;
  onScopeChange?: (scope: ScopeValue) => void;
  onAddToPortal?: (choice: DeviceSlot, portalIds: string[]) => void;
  config?: DeepPartial<DeviceChoiceGridConfig>;
};

export type DeviceChoiceGridIds = {
  cell: string;
  selected: string;
  shine: string;
  glow: string;
  gridClip: string;
  titleGlow: string;
  selectedGlow: string;
};

export type DeviceChoiceGridCellPosition = {
  row: number;
  col: number;
  x: number;
  y: number;
};

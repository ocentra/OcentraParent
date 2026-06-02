import type { CSSProperties, ReactElement } from 'react';
import type { DeviceChoiceGridConfig } from './DeviceChoiceGridConfig';

export type DeviceStatus = 'connected' | 'available' | 'offline' | 'unsupported' | 'empty';
export const DEVICE_CHOICE_SCOPE_VALUES = ['lan', 'parent', 'portal'] as const;
export type ScopeValue = (typeof DEVICE_CHOICE_SCOPE_VALUES)[number];
export const DEVICE_CHOICE_DEFAULT_SCOPE_VALUES: readonly ScopeValue[] = ['lan', 'parent'];
export type DeviceKind = 'mobile' | 'desktop' | 'laptop' | 'tablet' | 'router' | 'unknown';
export type DevicePlatformKind = 'windows' | 'macos' | 'linux' | 'android' | 'ios' | 'router' | 'unknown';
export type SelectableDeviceStatus = Exclude<DeviceStatus, 'empty'>;

export type DeviceChoiceGridScopeIconRenderProps = {
  x: number;
  y: number;
  width: number;
  height: number;
  scope: ScopeValue;
  selected: boolean;
};

export type DeviceChoiceGridScopeIconRenderer = (props: DeviceChoiceGridScopeIconRenderProps) => ReactElement;

export type DeviceChoiceGridScopeIcon =
  | { href: string }
  | { render: DeviceChoiceGridScopeIconRenderer; foreignObject?: boolean };

export type LanDevice = {
  id: string;
  name: string;
  ip?: string | undefined;
  mac?: string | undefined;
  hostname?: string | undefined;
  networkInterface?: string | undefined;
  agentStatus?: string | undefined;
  manufacturer?: string | undefined;
  model?: string | undefined;
  cpuModel?: string | undefined;
  cpuCores?: string | undefined;
  memoryTotal?: string | undefined;
  gpuModel?: string | undefined;
  gpuDriver?: string | undefined;
  gpuMemory?: string | undefined;
  nvidiaSmi?: string | undefined;
  routeId?: string | undefined;
  sourceState?: string | undefined;
  portalEligible?: boolean | undefined;
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
  scopeValues?: readonly ScopeValue[];
  className?: string;
  style?: CSSProperties;
  onChange?: (choice: DeviceSlot, index: number, row: number, column: number) => void;
  onScopeChange?: (scope: ScopeValue) => void;
  onAddToPortal?: (choice: DeviceSlot, portalIds: string[]) => void;
  scopeIcons?: Partial<Record<ScopeValue, DeviceChoiceGridScopeIcon>>;
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

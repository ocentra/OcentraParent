import type {
  DeviceKind,
  DevicePlatformKind,
  DeviceSlot,
  DeviceStatus,
  LanDevice,
  SelectableDeviceStatus,
} from './DeviceChoiceGridTypes';

export function emptyPortalSlot(slotIndex: number): DeviceSlot {
  return {
    value: `empty-${slotIndex + 1}`,
    label: 'Empty',
    status: 'empty',
    slotIndex,
  };
}

export function emptyLanSlot(slotIndex: number): DeviceSlot {
  return {
    value: `lan-empty-${slotIndex + 1}`,
    label: '',
    status: 'empty',
    slotIndex,
  };
}

export function unsupportedSlot(slotIndex: number): DeviceSlot {
  return {
    value: `unsupported-${slotIndex + 1}`,
    label: 'Unsupported',
    status: 'unsupported',
    slotIndex,
  };
}

export const emptySlot = emptyLanSlot;

export function toDeviceSlot(device: LanDevice, slotIndex: number, statusOverride?: DeviceStatus): DeviceSlot {
  return {
    value: device.id,
    label: device.name || device.ip || `Device ${slotIndex + 1}`,
    status: statusOverride ?? device.status ?? 'available',
    device,
    ...(device.platform ? { platform: device.platform } : {}),
    slotIndex,
  };
}

export function makeLanDeviceSlots(devices: LanDevice[] | undefined, totalSlots: number): DeviceSlot[] {
  const slots = (devices ?? []).slice(0, totalSlots).map((device, index) => toDeviceSlot(device, index));
  while (slots.length < totalSlots) {
    slots.push(emptyLanSlot(slots.length));
  }
  return slots;
}

export function makePortalSlots(lanSlots: DeviceSlot[], portalIds: string[], totalSlots: number): DeviceSlot[] {
  const portalIdSet = new Set(portalIds);
  const added: DeviceSlot[] = [];

  for (const slot of lanSlots) {
    if (added.length >= totalSlots) {
      break;
    }
    if (portalSlotEligible(slot) && (slot.status === 'connected' || portalIdSet.has(slot.value))) {
      const portalStatus = slot.status === 'offline' ? 'offline' : 'connected';
      added.push({ ...slot, status: portalStatus, slotIndex: added.length });
    }
  }

  while (added.length < totalSlots) {
    added.push(emptyPortalSlot(added.length));
  }
  return added;
}

function portalSlotEligible(slot: DeviceSlot): boolean {
  return !!slot.device && slot.device.portalEligible !== false;
}

export function makeDemoDeviceSlots(count: number): DeviceSlot[] {
  const kinds: DeviceKind[] = ['mobile', 'desktop', 'laptop', 'tablet', 'router', 'unknown'];
  const platforms: DevicePlatformKind[] = ['android', 'windows', 'macos', 'ios', 'router', 'linux'];
  const states: (SelectableDeviceStatus | 'unsupported')[] = ['connected', 'available', 'offline', 'unsupported'];

  return Array.from({ length: count }, (_, index) => {
    const status = states[(index * 7 + 3) % states.length] ?? 'available';
    return status === 'unsupported'
      ? unsupportedSlot(index)
      : toDeviceSlot(
          {
            id: `device-${index + 1}`,
            name: `Device ${index + 1}`,
            type: kinds[index % kinds.length] ?? 'unknown',
            platform: platforms[index % platforms.length] ?? 'unknown',
            status,
          },
          index
        );
  });
}

export function getLanSlots(
  slots: DeviceSlot[] | undefined,
  devices: LanDevice[] | undefined,
  options: DeviceSlot[] | undefined,
  count: number,
  fallback: DeviceSlot[]
): DeviceSlot[] {
  const source =
    slots !== undefined
      ? slots
      : options !== undefined
        ? options
        : devices !== undefined
          ? makeLanDeviceSlots(devices, count)
          : fallback;
  const items = source
    .filter((slot) => slot?.value && (slot.status === 'empty' || slot.label))
    .slice(0, count)
    .map((slot, index) => ({
      ...slot,
      label: slot.status === 'empty' ? '' : slot.label,
      slotIndex: slot.slotIndex ?? index,
      status: slot.status ?? 'available',
    }));

  while (items.length < count) {
    items.push(emptyLanSlot(items.length));
  }
  return items;
}

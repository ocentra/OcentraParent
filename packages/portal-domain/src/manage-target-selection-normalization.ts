import type { ManageTargetSelection } from './manage-target-selection';

type ManageTargetSelectionRecord = Partial<ManageTargetSelection> & {
  readonly deviceLabel?: unknown;
};

export function normalizeManageTargetSelectionValue(value: unknown): ManageTargetSelection | null {
  if (!isManageTargetSelectionRecord(value)) return null;
  if (value.scope !== 'global' && value.scope !== 'perDevice') return null;
  const device =
    typeof value.device === 'string' ? value.device : typeof value.deviceLabel === 'string' ? value.deviceLabel : '';
  const deviceId = typeof value.deviceId === 'string' ? value.deviceId : '';
  const browser = typeof value.browser === 'string' ? value.browser : '';
  return {
    scope: value.scope,
    // A family-scoped selection cannot carry a hidden device back into a later
    // per-device route. Keep persisted context canonical and presentation-only.
    device: value.scope === 'global' ? '' : normalizeSelectionText(device),
    deviceId: value.scope === 'global' ? '' : normalizeSelectionText(deviceId),
    browser: normalizeSelectionText(browser) || 'Chrome',
  };
}

function normalizeSelectionText(value: unknown): string {
  return typeof value === 'string' ? value.trim() : '';
}

function isManageTargetSelectionRecord(value: unknown): value is ManageTargetSelectionRecord {
  return typeof value === 'object' && value !== null;
}

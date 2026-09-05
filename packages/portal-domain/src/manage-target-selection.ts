import { normalizeManageTargetSelectionValue } from './manage-target-selection-normalization';

export type ManageScopeId = 'global' | 'perDevice';

export type ManageTargetSelection = {
  readonly scope: ManageScopeId;
  readonly device: string;
  readonly deviceId: string;
  readonly browser: string;
};

type ManageTargetSelectionStorage = {
  readonly getItem: (key: string) => string | null;
  readonly setItem: (key: string, value: string) => void;
};

export const PARENT_PORTAL_MANAGE_TARGET_SELECTION_STORAGE_KEY = 'ocentra.parent.portal.manage-target-selection.v1';

export function defaultManageTargetSelection(): ManageTargetSelection {
  return {
    scope: 'perDevice',
    device: '',
    deviceId: '',
    browser: 'Chrome',
  };
}

export function normalizeManageTargetSelection(value: unknown): ManageTargetSelection | null {
  return normalizeManageTargetSelectionValue(value);
}

export function readStoredManageTargetSelection(
  storage: ManageTargetSelectionStorage | null | undefined = browserSessionStorage()
): ManageTargetSelection | null {
  if (!storage) return null;
  try {
    const raw = storage.getItem(PARENT_PORTAL_MANAGE_TARGET_SELECTION_STORAGE_KEY);
    if (!raw) return null;
    return normalizeManageTargetSelection(JSON.parse(raw));
  } catch {
    return null;
  }
}

export function writeStoredManageTargetSelection(
  selection: ManageTargetSelection,
  storage: ManageTargetSelectionStorage | null | undefined = browserSessionStorage()
): void {
  if (!storage) return;
  const normalizedSelection = normalizeManageTargetSelection(selection);
  if (!normalizedSelection) return;
  try {
    storage.setItem(PARENT_PORTAL_MANAGE_TARGET_SELECTION_STORAGE_KEY, JSON.stringify(normalizedSelection));
  } catch {
    // Ignore storage write failures and keep the in-memory selection authoritative.
  }
}

export function withManageTargetSelectionDevice(
  selection: ManageTargetSelection,
  deviceId: string,
  deviceLabel: string
): ManageTargetSelection {
  return {
    ...selection,
    scope: 'perDevice',
    device: normalizeSelectionText(deviceLabel),
    deviceId: normalizeSelectionText(deviceId),
    browser: normalizeSelectionText(selection.browser) || 'Chrome',
  };
}

/**
 * Projects the persisted UI selection into optional route context.
 *
 * This value is not proof of ownership, pairing, reachability, or action
 * authority; those checks remain owned by the parent/LAN runtime boundary.
 */
export function selectedChildDeviceIdFromManageTargetSelection(
  selection: ManageTargetSelection | null | undefined
): string | null {
  if (!selection || selection.scope !== 'perDevice') return null;
  const deviceId = normalizeSelectionText(selection.deviceId);
  return deviceId.length > 0 ? deviceId : null;
}

function normalizeSelectionText(value: unknown): string {
  return typeof value === 'string' ? value.trim() : '';
}

function browserSessionStorage(): ManageTargetSelectionStorage | null {
  const windowLike = globalThis as typeof globalThis & {
    readonly window?: {
      readonly sessionStorage?: ManageTargetSelectionStorage;
    };
  };
  return windowLike.window?.sessionStorage ?? null;
}

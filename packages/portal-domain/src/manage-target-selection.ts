export type ManageScopeId = 'global' | 'perDevice';

export type ManageTargetSelection = {
  readonly scope: ManageScopeId;
  readonly device: string;
  readonly deviceId: string;
  readonly browser: string;
};

type ManageTargetSelectionRecord = Partial<ManageTargetSelection> & {
  readonly deviceLabel?: unknown;
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
  if (!isManageTargetSelectionRecord(value)) return null;
  if (value.scope !== 'global' && value.scope !== 'perDevice') return null;
  return {
    scope: value.scope,
    device:
      typeof value.device === 'string' ? value.device : typeof value.deviceLabel === 'string' ? value.deviceLabel : '',
    deviceId: typeof value.deviceId === 'string' ? value.deviceId : '',
    browser: typeof value.browser === 'string' ? value.browser : 'Chrome',
  };
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
  try {
    storage.setItem(PARENT_PORTAL_MANAGE_TARGET_SELECTION_STORAGE_KEY, JSON.stringify(selection));
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
    device: deviceLabel,
    deviceId,
  };
}

export function selectedChildDeviceIdFromManageTargetSelection(
  selection: ManageTargetSelection | null | undefined
): string | null {
  if (!selection || selection.scope !== 'perDevice') return null;
  const deviceId = selection.deviceId.trim();
  return deviceId.length > 0 ? deviceId : null;
}

function browserSessionStorage(): ManageTargetSelectionStorage | null {
  const windowLike = globalThis as typeof globalThis & {
    readonly window?: {
      readonly sessionStorage?: ManageTargetSelectionStorage;
    };
  };
  return windowLike.window?.sessionStorage ?? null;
}

function isManageTargetSelectionRecord(value: unknown): value is ManageTargetSelectionRecord {
  return typeof value === 'object' && value !== null;
}

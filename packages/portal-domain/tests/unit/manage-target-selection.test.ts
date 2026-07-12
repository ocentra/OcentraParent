import { describe, expect, it } from 'vitest';
import {
  defaultManageTargetSelection,
  normalizeManageTargetSelection,
  readStoredManageTargetSelection,
  selectedChildDeviceIdFromManageTargetSelection,
  withManageTargetSelectionDevice,
  writeStoredManageTargetSelection,
} from '../../src/manage-target-selection';

type MemoryStorageRecord = {
  value: string | null;
};

function createMemoryStorage(initialValue: string | null = null): {
  getItem: (key: string) => string | null;
  setItem: (key: string, value: string) => void;
} {
  const record: MemoryStorageRecord = { value: initialValue };
  return {
    getItem() {
      return record.value;
    },
    setItem(_key, value) {
      record.value = value;
    },
  };
}

describe('manage-target-selection', () => {
  it('normalizes legacy session payloads without inventing a stable child device id', () => {
    const selection = normalizeManageTargetSelection({
      scope: 'perDevice',
      device: 'Study Laptop',
      browser: 'Chrome',
    });

    expect(selection).toEqual({
      scope: 'perDevice',
      device: 'Study Laptop',
      deviceId: '',
      browser: 'Chrome',
    });
    expect(selectedChildDeviceIdFromManageTargetSelection(selection)).toBeNull();
  });

  it('stores and reloads stable selected child device ids for route context reuse', () => {
    const storage = createMemoryStorage();
    const selection = withManageTargetSelectionDevice(
      defaultManageTargetSelection(),
      'child-android-1',
      'Study Laptop'
    );

    writeStoredManageTargetSelection(selection, storage);

    expect(readStoredManageTargetSelection(storage)).toEqual(selection);
    expect(selectedChildDeviceIdFromManageTargetSelection(selection)).toBe('child-android-1');
  });
});

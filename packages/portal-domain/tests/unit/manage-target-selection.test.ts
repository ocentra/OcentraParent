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

  it('canonicalizes whitespace and clears stale device data from family scope', () => {
    expect(
      normalizeManageTargetSelection({
        scope: 'perDevice',
        device: '  Study Laptop  ',
        deviceId: '  child-android-1  ',
        browser: '  Chrome  ',
      })
    ).toEqual({
      scope: 'perDevice',
      device: 'Study Laptop',
      deviceId: 'child-android-1',
      browser: 'Chrome',
    });

    const familySelection = normalizeManageTargetSelection({
      scope: 'global',
      device: 'Study Laptop',
      deviceId: 'child-android-1',
      browser: 'Chrome',
    });

    expect(familySelection).toEqual({
      scope: 'global',
      device: '',
      deviceId: '',
      browser: 'Chrome',
    });
    expect(selectedChildDeviceIdFromManageTargetSelection(familySelection)).toBeNull();
  });

  it('fails closed for malformed or non-string stored context instead of emitting a child id', () => {
    const malformedValues: readonly unknown[] = [
      null,
      [],
      { scope: 'unknown', device: 'Study Laptop', deviceId: 'child-android-1' },
      { scope: 'perDevice', device: 'Study Laptop', deviceId: 42 },
      { scope: 'perDevice', device: 'Study Laptop', deviceId: '   ' },
    ];

    for (const value of malformedValues) {
      const selection = normalizeManageTargetSelection(value);
      expect(selectedChildDeviceIdFromManageTargetSelection(selection)).toBeNull();
    }

    expect(readStoredManageTargetSelection(createMemoryStorage('{"scope":'))).toBeNull();
  });

  it('persists the canonical presentation context rather than caller-shaped whitespace', () => {
    const storage = createMemoryStorage();

    writeStoredManageTargetSelection(
      {
        scope: 'perDevice',
        device: '  Study Laptop  ',
        deviceId: '  child-android-1  ',
        browser: '  Chrome  ',
      },
      storage
    );

    expect(readStoredManageTargetSelection(storage)).toEqual({
      scope: 'perDevice',
      device: 'Study Laptop',
      deviceId: 'child-android-1',
      browser: 'Chrome',
    });
  });
});

import { describe, expect, it } from 'vitest';
import {
  BrowserInventoryPlatformMatrix,
  BrowserInventoryPlatformMatrixEntrySchema,
  BrowserInventoryPlatformMatrixSchema,
} from '../src/browser';

describe('browser platform inventory matrix', () => {
  it('captures desktop and mobile platform states without upgrading claims', () => {
    const matrix = BrowserInventoryPlatformMatrixSchema.parse(BrowserInventoryPlatformMatrix);
    const platformCounts = countBy(matrix.entries.map((entry) => entry.platform));
    const proofCounts = countBy(matrix.entries.map((entry) => entry.proofState));

    expect(matrix.entries).toHaveLength(12);
    expect(platformCounts).toEqual({
      windows: 2,
      macos: 2,
      linux: 3,
      android: 3,
      ios: 2,
    });
    expect(proofCounts).toEqual({
      'fixture-backed': 2,
      'manual-required': 4,
      unsupported: 6,
    });
  });

  it('keeps non-Windows entries from claiming managed exact URL or active-tab support', () => {
    const matrix = BrowserInventoryPlatformMatrixSchema.parse(BrowserInventoryPlatformMatrix);
    const nonWindows = matrix.entries.filter((entry) => entry.platform !== 'windows');

    expect(nonWindows).not.toHaveLength(0);
    expect(
      nonWindows.every(
        (entry) =>
          entry.exactUrlCapability !== 'managed-exact-url-available' &&
          entry.activeTabCapability !== 'known-active-supported'
      )
    ).toBe(true);
  });

  it('marks mobile browser paths as owned-shell manual-required or unsupported only', () => {
    const matrix = BrowserInventoryPlatformMatrixSchema.parse(BrowserInventoryPlatformMatrix);
    const androidShell = matrix.entries.find(
      (entry) => entry.reasonCode === 'android-owned-browser-shell-manual-required'
    );
    const iosEntries = matrix.entries.filter((entry) => entry.platform === 'ios');

    expect(androidShell).toMatchObject({
      platform: 'android',
      managementTier: 'owned-shell',
      supportTier: 'candidate',
      exactUrlCapability: 'manual-required',
      proofState: 'manual-required',
    });
    expect(iosEntries.every((entry) => entry.managementTier === 'unsupported')).toBe(true);
  });

  it('rejects unsupported entries that try to keep exact URL available', () => {
    const safari = BrowserInventoryPlatformMatrix.entries.find((entry) => entry.productName === 'Safari');
    if (safari === undefined) {
      throw new Error('Missing Safari platform matrix entry');
    }

    expect(() =>
      BrowserInventoryPlatformMatrixEntrySchema.parse({
        ...safari,
        exactUrlCapability: 'managed-exact-url-available',
      })
    ).toThrow();
  });
});

function countBy(values: readonly string[]) {
  return values.reduce<Record<string, number>>((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}

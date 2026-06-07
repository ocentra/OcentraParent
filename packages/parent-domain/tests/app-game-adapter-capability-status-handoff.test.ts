import { describe, expect, it } from 'vitest';
import {
  AppGameAdapterCapabilityStatusReadModel,
  AppGameAdapterCapabilityStatusReadModelSchema,
  AppGameAdapterCapabilityStatusRowSchema,
} from '../src/app-game-adapter-capability-status-handoff';

describe('app game adapter capability status handoff', () => {
  registerProjectionTests();
  registerWindowsBoundaryTests();
  registerPlatformBoundaryTests();
  registerClaimUpgradeTests();
});

function registerProjectionTests() {
  it('projects native app and native game adapter status for every platform', () => {
    const readModel = AppGameAdapterCapabilityStatusReadModelSchema.parse(AppGameAdapterCapabilityStatusReadModel);

    expect(readModel.readModelId).toBe('app-game-adapter-capability-status-handoff');
    expect(readModel.rows).toHaveLength(10);
    expect(countBy(readModel.rows.map((row) => row.productTarget))).toEqual({
      'native-app': 5,
      'native-game': 5,
    });
    expect(countBy(readModel.rows.map((row) => row.platform))).toEqual({
      android: 2,
      ios: 2,
      linux: 2,
      macos: 2,
      windows: 2,
    });
  });
}

function registerWindowsBoundaryTests() {
  it('keeps Windows ready only for owned-process time-limit while broad blocking stays manual', () => {
    const nativeApp = rowFor('windows', 'native-app');
    const nativeGame = rowFor('windows', 'native-game');

    for (const row of [nativeApp, nativeGame]) {
      expect(row.adapterStatus).toBe('runtime-boundary-ready');
      expect(row.broadBlockingStatus).toBe('manual-required');
      expect(row.timeLimitProofRefs).toContain('windows-app-game-owned-process-time-limit');
      expect(row.platformProofRefs).toContain('windows-owned-process-terminate');
      expect(row.manualProofRequirements).toContain('host block apply artifact');
      expect(row.claimBoundary).toContain('not broad installed-app blocking');
    }
  });
}

function registerPlatformBoundaryTests() {
  it('keeps non Windows and mobile platforms manual scaffold or unavailable without dispatch claims', () => {
    expect(rowFor('macos', 'native-app')).toMatchObject({
      adapterStatus: 'scaffold-only',
      broadBlockingStatus: 'manual-required',
    });
    expect(rowFor('linux', 'native-game')).toMatchObject({
      adapterStatus: 'unavailable',
      broadBlockingStatus: 'unavailable',
    });
    expect(rowFor('android', 'native-app').manualProofRequirements).toContain(
      'device-owner or managed-profile artifact'
    );
    expect(rowFor('ios', 'native-game').manualProofRequirements).toContain('Family Controls entitlement artifact');
    expect(
      AppGameAdapterCapabilityStatusReadModel.rows.every(
        (row) =>
          !row.adapterDispatchClaimed &&
          !row.broadBlockingClaimed &&
          !row.platformEnforcementClaimed &&
          !row.childDeliveryClaimed
      )
    ).toBe(true);
  });
}

function registerClaimUpgradeTests() {
  it('rejects ready rows without time limit proof and rejects claim upgrades', () => {
    const windows = rowFor('windows', 'native-app');
    const android = rowFor('android', 'native-app');

    expect(() =>
      AppGameAdapterCapabilityStatusRowSchema.parse({
        ...windows,
        rowId: 'invalid-ready-without-time-limit-proof',
        timeLimitProofRefs: [],
      })
    ).toThrow();
    expect(() =>
      AppGameAdapterCapabilityStatusRowSchema.parse({
        ...windows,
        rowId: 'invalid-broad-blocking-claim',
        broadBlockingClaimed: true,
      })
    ).toThrow();
    expect(() =>
      AppGameAdapterCapabilityStatusRowSchema.parse({
        ...android,
        rowId: 'invalid-manual-row-dispatch-claim',
        adapterDispatchClaimed: true,
      })
    ).toThrow();
  });
}

function rowFor(platform: string, productTarget: string) {
  const row = AppGameAdapterCapabilityStatusReadModel.rows.find(
    (candidate) => candidate.platform === platform && candidate.productTarget === productTarget
  );
  if (row === undefined) {
    throw new Error(`Missing adapter capability status row for ${platform}/${productTarget}`);
  }
  return row;
}

function countBy(values: readonly string[]) {
  return values.reduce<Record<string, number>>((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}

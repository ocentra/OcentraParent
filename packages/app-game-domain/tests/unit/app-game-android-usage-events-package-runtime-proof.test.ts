import { describe, expect, it } from 'vitest';
import {
  AppGameAndroidUsageEventsPackageRuntimeProofSchema,
  createAppGameAndroidUsageEventsPackageRuntimeProof,
  summarizeAppGameAndroidUsageEventsPackageRuntimeProof,
} from '../../src/app-game-android-usage-events-package-runtime-proof';

describe('app-game Android UsageEvents package runtime proof', () => {
  it('accepts installed and launched package runtime evidence while grant is still required', () => {
    const readModel = createAppGameAndroidUsageEventsPackageRuntimeProof({
      permissionCheckState: 'settings-grant-required',
      sampleState: 'sample-permission-required',
      uiStateObserved: true,
      appOpsObserved: true,
      checkedAt: '2026-06-08T20:05:00.000Z',
    });
    const summary = summarizeAppGameAndroidUsageEventsPackageRuntimeProof(readModel);

    expect(summary.installedState).toBe('package-installed');
    expect(summary.launchState).toBe('package-launch-observed');
    expect(summary.permissionCheckState).toBe('settings-grant-required');
    expect(summary.sampleState).toBe('sample-permission-required');
    expect(readModel.openGaps).toEqual(
      expect.arrayContaining([
        'android-usage-stats-settings-grant-not-proved',
        'android-usage-events-live-package-sample-not-observed',
      ])
    );
  });

  it('accepts count-only live sample visibility without dispatch or enforcement claims', () => {
    const readModel = createAppGameAndroidUsageEventsPackageRuntimeProof({
      permissionCheckState: 'usage-stats-granted',
      sampleState: 'sample-observed',
      uiStateObserved: true,
      appOpsObserved: true,
      checkedAt: '2026-06-08T20:05:00.000Z',
    });

    expect(readModel.openGaps).not.toContain('android-usage-stats-settings-grant-not-proved');
    expect(readModel.openGaps).not.toContain('android-usage-events-live-package-sample-not-observed');
    expect(readModel.adapterDispatchClaimed).toBe(false);
    expect(readModel.platformEnforcementClaimed).toBe(false);
    expect(readModel.childDeviceDeliveryClaimed).toBe(false);
  });

  it('rejects missing UI evidence, raw data custody, dispatch, and child delivery claims', () => {
    const readModel = createAppGameAndroidUsageEventsPackageRuntimeProof({
      permissionCheckState: 'settings-grant-required',
      sampleState: 'sample-permission-required',
      uiStateObserved: true,
      appOpsObserved: true,
      checkedAt: '2026-06-08T20:05:00.000Z',
    });

    expect(
      AppGameAndroidUsageEventsPackageRuntimeProofSchema.safeParse({
        ...readModel,
        uiStateObserved: false,
      }).success
    ).toBe(false);
    expect(
      AppGameAndroidUsageEventsPackageRuntimeProofSchema.safeParse({
        ...readModel,
        rawUsageEventsStored: true,
      }).success
    ).toBe(false);
    expect(
      AppGameAndroidUsageEventsPackageRuntimeProofSchema.safeParse({
        ...readModel,
        adapterDispatchClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameAndroidUsageEventsPackageRuntimeProofSchema.safeParse({
        ...readModel,
        childDeviceDeliveryClaimed: true,
      }).success
    ).toBe(false);
  });
});

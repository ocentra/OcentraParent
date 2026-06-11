import { describe, expect, it } from 'vitest';
import {
  AppGameAndroidUsageEventsRuntimePreflightReadModelSchema,
  createAppGameAndroidUsageEventsRuntimePreflightReadModel,
  summarizeAppGameAndroidUsageEventsRuntimePreflightReadModel,
} from '../src/app-game-android-usage-events-runtime-preflight';

describe('app-game Android UsageEvents runtime preflight', () => {
  it('keeps runtime collection blocked when UsageStats settings grant is still required', () => {
    const readModel = createAppGameAndroidUsageEventsRuntimePreflightReadModel({
      permissionCheckState: 'settings-grant-required',
      usageStatsServiceState: 'service-visible',
      checkedAt: '2026-06-08T19:35:00.000Z',
    });
    const summary = summarizeAppGameAndroidUsageEventsRuntimePreflightReadModel(readModel);

    expect(summary.permissionCheckState).toBe('settings-grant-required');
    expect(summary.runtimeCollectionState).toBe('collection-blocked-before-runtime-proof');
    expect(summary.runtimeCollectionClaimed).toBe(false);
    expect(summary.adapterDispatchClaimed).toBe(false);
    expect(summary.platformEnforcementClaimed).toBe(false);
    expect(summary.childDeviceDeliveryClaimed).toBe(false);
    expectRuntimePreflightSignals(readModel);
  });

  it('allows granted UsageStats to become ready for proof without claiming samples', () => {
    const readModel = createAppGameAndroidUsageEventsRuntimePreflightReadModel({
      permissionCheckState: 'usage-stats-granted',
      usageStatsServiceState: 'service-visible',
      checkedAt: '2026-06-08T19:35:00.000Z',
    });

    expect(readModel.runtimeCollectionState).toBe('collection-ready-for-proof');
    expect(readModel.rawUsageEventsStored).toBe(false);
    expect(readModel.packageNamesStored).toBe(false);
    expect(readModel.runtimeCollectionClaimed).toBe(false);
    expect(readModel.openGaps).toContain('android-usage-events-runtime-sample-not-proved');
  });

  it('rejects runtime collection, raw storage, dispatch, and mismatched state overclaims', () => {
    const readModel = createAppGameAndroidUsageEventsRuntimePreflightReadModel({
      permissionCheckState: 'settings-grant-required',
      usageStatsServiceState: 'service-visible',
      checkedAt: '2026-06-08T19:35:00.000Z',
    });

    expect(
      AppGameAndroidUsageEventsRuntimePreflightReadModelSchema.safeParse({
        ...readModel,
        runtimeCollectionClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameAndroidUsageEventsRuntimePreflightReadModelSchema.safeParse({
        ...readModel,
        rawUsageEventsStored: true,
      }).success
    ).toBe(false);
    expect(
      AppGameAndroidUsageEventsRuntimePreflightReadModelSchema.safeParse({
        ...readModel,
        adapterDispatchClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameAndroidUsageEventsRuntimePreflightReadModelSchema.safeParse({
        ...readModel,
        runtimeCollectionState: 'collection-ready-for-proof',
      }).success
    ).toBe(false);
  });
});

function expectRuntimePreflightSignals(
  readModel: ReturnType<typeof createAppGameAndroidUsageEventsRuntimePreflightReadModel>
) {
  expect(readModel.commands).toEqual(['app-game.android.usage-events.runtime-preflight.get']);
  expect(readModel.events).toEqual(['app-game.android.usage-events.runtime-preflight.reported']);
  expect(readModel.proofRefs).toEqual(
    expect.arrayContaining(['android-usage-events-runtime-preflight-ref', 'android-usage-stats-appops-preflight-ref'])
  );
  expect(readModel.openGaps).toEqual(
    expect.arrayContaining([
      'android-usage-events-runtime-sample-not-proved',
      'android-child-runtime-delivery-not-proved',
      'android-platform-enforcement-not-proved',
    ])
  );
}

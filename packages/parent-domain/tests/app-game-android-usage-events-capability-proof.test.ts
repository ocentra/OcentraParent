import { describe, expect, it } from 'vitest';
import {
  AppGameAndroidUsageEventsCapabilityReadModelSchema,
  createAppGameAndroidUsageEventsCapabilityReadModel,
  summarizeAppGameAndroidUsageEventsCapabilityReadModel,
} from '../src/app-game-android-usage-events-capability-proof';

describe('app-game Android UsageEvents capability proof', () => {
  it('accepts the package-local Android UsageEvents bridge without promoting runtime claims', () => {
    const readModel = createAppGameAndroidUsageEventsCapabilityReadModel({
      checkedAt: '2026-06-08T19:20:00.000Z',
    });
    const summary = summarizeAppGameAndroidUsageEventsCapabilityReadModel(readModel);

    expect(summary.usageEventsBridgeState).toBe('package-local-scaffold');
    expect(summary.permissionState).toBe('settings-grant-required');
    expect(summary.eventCollectionState).toBe('runtime-grant-not-proved');
    expect(summary.replayConsumerState).toBe('parent-domain-boundary-only');
    expect(summary.adapterDispatchClaimed).toBe(false);
    expect(summary.platformEnforcementClaimed).toBe(false);
    expect(summary.childDeviceDeliveryClaimed).toBe(false);
    expectUsageEventsBridgeSignals(readModel);
  });

  it('rejects rows that claim raw UsageEvents storage or platform execution', () => {
    const readModel = createAppGameAndroidUsageEventsCapabilityReadModel({
      checkedAt: '2026-06-08T19:20:00.000Z',
    });

    expect(
      AppGameAndroidUsageEventsCapabilityReadModelSchema.safeParse({
        ...readModel,
        rawUsageEventsStored: true,
      }).success
    ).toBe(false);
    expect(
      AppGameAndroidUsageEventsCapabilityReadModelSchema.safeParse({
        ...readModel,
        packageNamesStored: true,
      }).success
    ).toBe(false);
    expect(
      AppGameAndroidUsageEventsCapabilityReadModelSchema.safeParse({
        ...readModel,
        adapterDispatchClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameAndroidUsageEventsCapabilityReadModelSchema.safeParse({
        ...readModel,
        platformEnforcementClaimed: true,
      }).success
    ).toBe(false);
  });

  it('rejects rows missing the package-local bridge proof refs and gaps', () => {
    const readModel = createAppGameAndroidUsageEventsCapabilityReadModel({
      checkedAt: '2026-06-08T19:20:00.000Z',
    });

    expect(
      AppGameAndroidUsageEventsCapabilityReadModelSchema.safeParse({
        ...readModel,
        proofRefs: ['android-usage-events-capability-bridge-ref'],
      }).success
    ).toBe(false);
    expect(
      AppGameAndroidUsageEventsCapabilityReadModelSchema.safeParse({
        ...readModel,
        openGaps: ['android-usage-events-runtime-collection-not-proved'],
      }).success
    ).toBe(false);
  });
});

function expectUsageEventsBridgeSignals(
  readModel: ReturnType<typeof createAppGameAndroidUsageEventsCapabilityReadModel>
) {
  expect(readModel.commands).toEqual(
    expect.arrayContaining([
      'app-game.android.usage-events.capability.get',
      'app-game.android.usage-events.replay-boundary.get',
    ])
  );
  expect(readModel.events).toEqual(
    expect.arrayContaining([
      'app-game.android.usage-events.capability.reported',
      'app-game.android.usage-events.replay-boundary.reported',
    ])
  );
  expect(readModel.proofRefs).toEqual(
    expect.arrayContaining([
      'android-usage-events-capability-bridge-ref',
      'android-package-local-usage-events-proof-ref',
    ])
  );
  expect(readModel.openGaps).toEqual(
    expect.arrayContaining([
      'android-usage-stats-settings-grant-not-proved',
      'android-usage-events-runtime-collection-not-proved',
      'android-child-runtime-delivery-not-proved',
      'android-platform-enforcement-not-proved',
    ])
  );
}

import { describe, expect, it } from 'vitest';
import { decodeAppGameAndroidPhysicalDeviceProof } from '@ocentra-parent/schema-domain/app-game-android-physical-device-proof';
import {
  AppGameAndroidUsageEventsReplayReadModelSchema,
  createAppGameAndroidUsageEventsReplayReadModel,
  summarizeAppGameAndroidUsageEventsReplayReadModel,
} from '@ocentra-parent/schema-domain/app-game-android-usage-events-replay';

describe('app-game Android UsageEvents replay readiness', () => {
  it('accepts redacted foreground UsageEvents counts as durable replay readiness', () => {
    const readModel = createAppGameAndroidUsageEventsReplayReadModel({
      androidProof: decodeAppGameAndroidPhysicalDeviceProof(validPhysicalProof()),
      generatedAt: '2026-06-08T17:35:00.000Z',
    });
    const summary = summarizeAppGameAndroidUsageEventsReplayReadModel(readModel);

    expect(summary.replayState).toBe('durable-replay-ready');
    expect(summary.runtimeVisibilityReady).toBe(true);
    expect(summary.durableReplayReady).toBe(true);
    expect(summary.usageEventsSampleCount).toBe(2331);
    expect(summary.foregroundActivityEventCount).toBe(577);
    expect(readModel.proofRefs).toEqual(
      expect.arrayContaining([
        'android-usage-events-replay-ref',
        'android-physical-usage-events-dump-ref',
        'android-runtime-visibility-read-model-ref',
      ])
    );
    expect(readModel.openGaps).toEqual(
      expect.arrayContaining([
        'android-child-runtime-replay-consumer-not-attached',
        'android-hide-suspend-not-proved',
        'android-platform-enforcement-not-proved',
        'android-child-device-delivery-not-proved',
      ])
    );
  });

  it('keeps unavailable UsageEvents dumps replay-not-ready without claiming raw rows', () => {
    const readModel = createAppGameAndroidUsageEventsReplayReadModel({
      androidProof: decodeAppGameAndroidPhysicalDeviceProof(unavailableUsageEventsProof()),
      generatedAt: '2026-06-08T17:35:00.000Z',
    });

    expect(readModel.replayState).toBe('replay-not-ready');
    expect(readModel.runtimeVisibilityReady).toBe(false);
    expect(readModel.durableReplayReady).toBe(false);
    expect(readModel.proofRefs).toEqual(['android-runtime-visibility-read-model-ref']);
  });

  it('rejects replay rows that claim private rows, enforcement, or drifted state', () => {
    const readModel = createAppGameAndroidUsageEventsReplayReadModel({
      androidProof: decodeAppGameAndroidPhysicalDeviceProof(validPhysicalProof()),
      generatedAt: '2026-06-08T17:35:00.000Z',
    });

    expect(
      AppGameAndroidUsageEventsReplayReadModelSchema.safeParse({
        ...readModel,
        rawActivityRowsClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameAndroidUsageEventsReplayReadModelSchema.safeParse({
        ...readModel,
        platformEnforcementClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameAndroidUsageEventsReplayReadModelSchema.safeParse({
        ...readModel,
        foregroundActivityEventCount: 0,
      }).success
    ).toBe(false);
  });
});

function validPhysicalProof() {
  return {
    schemaVersion: 'app-game-android-physical-device-proof',
    proofId: 'app-game-android-physical-device-proof-s9',
    targetKind: 'physical-device',
    connectionState: 'physical-device-connected',
    adbTargetRef: 'android-physical-adb-device-ref',
    product: 'star2qltecs',
    model: 'SM_G965W',
    device: 'star2qltecs',
    androidRelease: '10',
    sdkInt: 29,
    supportedAbiCount: 4,
    packageManagerVisibleCount: 278,
    usageStatsServiceState: 'service-visible',
    usageEventsDumpState: 'usage-events-dump-observed',
    usageEventsSampleCount: 2331,
    foregroundActivityEventCount: 577,
    deviceOwnerState: 'not-device-owner',
    profileOwnerState: 'not-profile-owner',
    proofRefs: [
      'android-physical-adb-device-ref',
      'android-physical-build-prop-ref',
      'android-physical-package-manager-ref',
      'android-physical-usage-stats-service-ref',
      'android-physical-usage-events-dump-ref',
      'android-physical-device-policy-ref',
    ],
    packageNamesRedacted: true,
    usageEventsPackageNamesRedacted: true,
    rawDeviceSerialRedacted: true,
    foregroundEvidenceObserved: true,
    hideSuspendClaimed: false,
    adapterDispatchClaimed: false,
    broadBlockingClaimed: false,
    platformEnforcementClaimed: false,
    parentVisibleSummary:
      'Physical Android 10 device is reachable for package and policy-state proof; normal-mode hide/suspend remains blocked until Device Owner or Profile Owner proof is attached.',
    checkedAt: '2026-06-08T15:55:00.000Z',
  };
}

function unavailableUsageEventsProof() {
  return {
    ...validPhysicalProof(),
    usageEventsDumpState: 'usage-events-dump-unavailable',
    usageEventsSampleCount: 0,
    foregroundActivityEventCount: 0,
    foregroundEvidenceObserved: false,
    proofRefs: [
      'android-physical-adb-device-ref',
      'android-physical-build-prop-ref',
      'android-physical-package-manager-ref',
      'android-physical-usage-stats-service-ref',
      'android-physical-device-policy-ref',
    ],
  };
}

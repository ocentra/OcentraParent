import { describe, expect, it } from 'vitest';
import { decodeAppGameAndroidPhysicalDeviceProof } from '../../src/app-game-android-physical-device-proof';
import { createAppGameAndroidUsageEventsReplayReadModel } from '../../src/app-game-android-usage-events-replay';
import {
  AppGameAndroidUsageEventsChildRuntimeReplayReadModelSchema,
  createAppGameAndroidUsageEventsChildRuntimeReplayReadModel,
  summarizeAppGameAndroidUsageEventsChildRuntimeReplayReadModel,
} from '../../src/app-game-android-usage-events-child-runtime-replay';

describe('app-game Android UsageEvents child runtime replay', () => {
  it('attaches redacted UsageEvents replay counts to a child runtime consumer boundary', () => {
    const readModel = createAppGameAndroidUsageEventsChildRuntimeReplayReadModel({
      replayReadModel: readyReplayReadModel(),
      generatedAt: '2026-06-08T17:46:00.000Z',
    });
    const summary = summarizeAppGameAndroidUsageEventsChildRuntimeReplayReadModel(readModel);

    expect(summary.replayState).toBe('consumer-attached-redacted-replay');
    expect(summary.childRuntimeReplayConsumerAttached).toBe(true);
    expect(summary.replayedForegroundEventCount).toBe(577);
    expect(summary.replayedUsageEventSampleCount).toBe(2331);
    expect(readModel.proofRefs).toEqual(
      expect.arrayContaining([
        'android-usage-events-child-runtime-replay-ref',
        'android-usage-events-replay-ref',
        'android-runtime-visibility-read-model-ref',
      ])
    );
    expect(readModel.openGaps).not.toContain('android-child-runtime-replay-consumer-not-attached');
    expect(readModel.openGaps).toEqual(
      expect.arrayContaining([
        'android-hide-suspend-not-proved',
        'android-platform-enforcement-not-proved',
        'android-child-device-delivery-not-proved',
      ])
    );
  });

  it('keeps unavailable replay consumer-not-ready without carrying foreground counts', () => {
    const readModel = createAppGameAndroidUsageEventsChildRuntimeReplayReadModel({
      replayReadModel: unavailableReplayReadModel(),
      generatedAt: '2026-06-08T17:46:00.000Z',
    });

    expect(readModel.replayState).toBe('consumer-not-ready');
    expect(readModel.childRuntimeReplayConsumerAttached).toBe(false);
    expect(readModel.replayedForegroundEventCount).toBe(0);
    expect(readModel.replayedUsageEventSampleCount).toBe(0);
    expect(readModel.proofRefs).toEqual(['android-runtime-visibility-read-model-ref']);
  });

  it('rejects raw rows delivery enforcement and drifted replay claims', () => {
    const readModel = createAppGameAndroidUsageEventsChildRuntimeReplayReadModel({
      replayReadModel: readyReplayReadModel(),
      generatedAt: '2026-06-08T17:46:00.000Z',
    });

    expect(
      AppGameAndroidUsageEventsChildRuntimeReplayReadModelSchema.safeParse({
        ...readModel,
        rawActivityRowsClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameAndroidUsageEventsChildRuntimeReplayReadModelSchema.safeParse({
        ...readModel,
        childDeviceDeliveryClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameAndroidUsageEventsChildRuntimeReplayReadModelSchema.safeParse({
        ...readModel,
        replayedForegroundEventCount: 0,
      }).success
    ).toBe(false);
  });
});

function readyReplayReadModel() {
  return createAppGameAndroidUsageEventsReplayReadModel({
    androidProof: decodeAppGameAndroidPhysicalDeviceProof(validPhysicalProof()),
    generatedAt: '2026-06-08T17:35:00.000Z',
  });
}

function unavailableReplayReadModel() {
  return createAppGameAndroidUsageEventsReplayReadModel({
    androidProof: decodeAppGameAndroidPhysicalDeviceProof(unavailableUsageEventsProof()),
    generatedAt: '2026-06-08T17:35:00.000Z',
  });
}

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

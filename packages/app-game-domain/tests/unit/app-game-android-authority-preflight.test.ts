import { describe, expect, it } from 'vitest';
import { decodeAppGameAndroidPhysicalDeviceProof } from '@ocentra-parent/schema-domain/app-game-android-physical-device-proof';
import {
  AppGameAndroidAuthorityPreflightReadModelSchema,
  createAppGameAndroidAuthorityPreflightReadModel,
  summarizeAppGameAndroidAuthorityPreflightReadModel,
} from '@ocentra-parent/schema-domain/app-game-android-authority-preflight';

describe('app-game Android authority preflight', () => {
  it('keeps package policy actions blocked when the physical device is not owner-enrolled', () => {
    const readModel = createAppGameAndroidAuthorityPreflightReadModel({
      androidProof: decodeAppGameAndroidPhysicalDeviceProof(validPhysicalProof()),
      generatedAt: '2026-06-08T17:55:00.000Z',
    });
    const summary = summarizeAppGameAndroidAuthorityPreflightReadModel(readModel);

    expect(summary.authorityState).toBe('authority-not-enrolled');
    expect(summary.deviceOwnerProofAttached).toBe(false);
    expect(summary.profileOwnerProofAttached).toBe(false);
    expect(summary.dispatchableActionCount).toBe(0);
    expect(summary.blockedActionCount).toBe(5);
    expect(readModel.rows.map((row) => row.action)).toEqual([
      'hide-package',
      'suspend-package',
      'uninstall-block',
      'lock-task',
      'managed-configuration',
    ]);
    expect(readModel.rows.every((row) => row.canDispatchAdapter === false)).toBe(true);
    expect(readModel.openBlockers).toEqual(
      expect.arrayContaining([
        'android-device-owner-not-proved',
        'android-profile-owner-not-proved',
        'android-adapter-dispatch-blocked-before-authority',
      ])
    );
  });

  it('does not treat not-proved policy states as owner proof', () => {
    const readModel = createAppGameAndroidAuthorityPreflightReadModel({
      androidProof: decodeAppGameAndroidPhysicalDeviceProof(notProvedOwnerStateProof()),
      generatedAt: '2026-06-08T17:55:00.000Z',
    });

    expect(readModel.authorityState).toBe('authority-not-enrolled');
    expect(readModel.deviceOwnerProofAttached).toBe(false);
    expect(readModel.profileOwnerProofAttached).toBe(false);
    expect(readModel.dispatchableActionCount).toBe(0);
    expect(readModel.blockedActionCount).toBe(5);
    expect(readModel.adapterDispatchClaimed).toBe(false);
    expect(readModel.platformEnforcementClaimed).toBe(false);
  });

  it('rejects dispatch enforcement and raw data overclaims', () => {
    const readModel = createAppGameAndroidAuthorityPreflightReadModel({
      androidProof: decodeAppGameAndroidPhysicalDeviceProof(validPhysicalProof()),
      generatedAt: '2026-06-08T17:55:00.000Z',
    });

    expect(
      AppGameAndroidAuthorityPreflightReadModelSchema.safeParse({
        ...readModel,
        rows: [{ ...readModel.rows[0], canDispatchAdapter: true }],
      }).success
    ).toBe(false);
    expect(
      AppGameAndroidAuthorityPreflightReadModelSchema.safeParse({
        ...readModel,
        platformEnforcementClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameAndroidAuthorityPreflightReadModelSchema.safeParse({
        ...readModel,
        rawPackageNamesClaimed: true,
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

function notProvedOwnerStateProof() {
  return {
    ...validPhysicalProof(),
    deviceOwnerState: 'not-proved',
  };
}

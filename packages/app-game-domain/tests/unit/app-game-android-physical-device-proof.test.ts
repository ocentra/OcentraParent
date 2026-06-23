import { describe, expect, it } from 'vitest';
import {
  AppGameAndroidPhysicalDeviceProofSchema,
  decodeAppGameAndroidPhysicalDeviceProof,
  summarizeAppGameAndroidPhysicalDeviceProof,
} from '@ocentra-parent/schema-domain/app-game-android-physical-device-proof';

describe('app-game Android physical device proof', () => {
  registerAcceptedProofTest();
  registerRedactionRejectionTest();
  registerUsageEventsRejectionTest();
  registerEnforcementClaimRejectionTest();
});

function registerAcceptedProofTest() {
  it('accepts a redacted physical Android device proof without upgrading enforcement claims', () => {
    const proof = decodeAppGameAndroidPhysicalDeviceProof(validPhysicalProof());
    const summary = summarizeAppGameAndroidPhysicalDeviceProof(proof);

    expect(proof.model).toBe('SM_G965W');
    expect(proof.product).toBe('star2qltecs');
    expect(proof.packageManagerVisibleCount).toBeGreaterThan(0);
    expect(proof.usageEventsSampleCount).toBeGreaterThan(0);
    expect(proof.foregroundActivityEventCount).toBeGreaterThan(0);
    expect(proof.packageNamesRedacted).toBe(true);
    expect(proof.usageEventsPackageNamesRedacted).toBe(true);
    expect(proof.rawDeviceSerialRedacted).toBe(true);
    expect(summary.foregroundEvidenceObserved).toBe(true);
    expect(summary.ownerProofAttached).toBe(false);
    expect(summary.adapterDispatchClaimed).toBe(false);
    expect(summary.platformEnforcementClaimed).toBe(false);
  });
}

function registerRedactionRejectionTest() {
  it('rejects physical Android evidence that exposes package names or raw serials', () => {
    expect(
      AppGameAndroidPhysicalDeviceProofSchema.safeParse({
        ...validPhysicalProof(),
        packageNamesRedacted: false,
      }).success
    ).toBe(false);
    expect(
      AppGameAndroidPhysicalDeviceProofSchema.safeParse({
        ...validPhysicalProof(),
        usageEventsPackageNamesRedacted: false,
      }).success
    ).toBe(false);
    expect(
      AppGameAndroidPhysicalDeviceProofSchema.safeParse({
        ...validPhysicalProof(),
        rawDeviceSerialRedacted: false,
      }).success
    ).toBe(false);
  });
}

function registerUsageEventsRejectionTest() {
  it('rejects foreground usage-event claims without observed redacted event samples', () => {
    expect(
      AppGameAndroidPhysicalDeviceProofSchema.safeParse({
        ...validPhysicalProof(),
        usageEventsSampleCount: 0,
      }).success
    ).toBe(false);
    expect(
      AppGameAndroidPhysicalDeviceProofSchema.safeParse({
        ...validPhysicalProof(),
        foregroundActivityEventCount: 0,
      }).success
    ).toBe(false);
    expect(
      AppGameAndroidPhysicalDeviceProofSchema.safeParse({
        ...validPhysicalProof(),
        foregroundEvidenceObserved: false,
      }).success
    ).toBe(false);
  });
}

function registerEnforcementClaimRejectionTest() {
  it('rejects attempts to claim app hide/suspend or platform enforcement from normal-mode proof', () => {
    expect(
      AppGameAndroidPhysicalDeviceProofSchema.safeParse({
        ...validPhysicalProof(),
        hideSuspendClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameAndroidPhysicalDeviceProofSchema.safeParse({
        ...validPhysicalProof(),
        adapterDispatchClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameAndroidPhysicalDeviceProofSchema.safeParse({
        ...validPhysicalProof(),
        platformEnforcementClaimed: true,
      }).success
    ).toBe(false);
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
    packageManagerVisibleCount: 200,
    usageStatsServiceState: 'service-visible',
    usageEventsDumpState: 'usage-events-dump-observed',
    usageEventsSampleCount: 80,
    foregroundActivityEventCount: 12,
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

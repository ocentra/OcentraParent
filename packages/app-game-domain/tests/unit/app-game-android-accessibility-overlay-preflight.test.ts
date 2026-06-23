import { describe, expect, it } from 'vitest';
import { decodeAppGameAndroidPhysicalDeviceProof } from '@ocentra-parent/schema-domain/app-game-android-physical-device-proof';
import {
  AppGameAndroidAccessibilityOverlayPreflightReadModelSchema,
  AppGameAndroidAccessibilitySettingsSampleSchema,
  createAppGameAndroidAccessibilityOverlayPreflightReadModel,
  summarizeAppGameAndroidAccessibilityOverlayPreflightReadModel,
} from '@ocentra-parent/schema-domain/app-game-android-accessibility-overlay-preflight';

describe('app-game Android Accessibility overlay preflight', () => {
  keepsOverlayActionsBlockedWithoutEnabledService();
  keepsOverlayActionsBlockedEvenWithEnabledServiceCount();
  rejectsRawServiceNamesAndOverlayExecutionClaims();
});

function keepsOverlayActionsBlockedWithoutEnabledService() {
  it('keeps overlay actions blocked when no enabled Accessibility service proof exists', () => {
    const readModel = createAppGameAndroidAccessibilityOverlayPreflightReadModel({
      androidProof: decodeAppGameAndroidPhysicalDeviceProof(validPhysicalProof()),
      accessibilitySettings: {
        accessibilityEnabled: false,
        enabledServiceCount: 0,
        serviceNamesRedacted: true,
        settingsReadable: true,
      },
      generatedAt: '2026-06-08T18:20:00.000Z',
    });
    const summary = summarizeAppGameAndroidAccessibilityOverlayPreflightReadModel(readModel);

    expect(summary.preflightState).toBe('accessibility-service-not-enabled');
    expect(summary.accessibilitySettingsReadable).toBe(true);
    expect(summary.enabledServiceCount).toBe(0);
    expect(summary.dispatchableActionCount).toBe(0);
    expect(summary.blockedActionCount).toBe(4);
    expect(readModel.rows.map((row) => row.action)).toEqual([
      'warning-overlay',
      'block-overlay',
      'request-overlay',
      'usage-context-overlay',
    ]);
    expect(readModel.openBlockers).toEqual(
      expect.arrayContaining([
        'android-accessibility-service-not-enabled',
        'android-accessibility-service-names-redacted',
        'android-overlay-runtime-not-proved',
        'android-adapter-dispatch-blocked-before-accessibility',
      ])
    );
  });
}

function keepsOverlayActionsBlockedEvenWithEnabledServiceCount() {
  it('keeps overlay actions blocked until overlay runtime proof exists', () => {
    const readModel = createAppGameAndroidAccessibilityOverlayPreflightReadModel({
      androidProof: decodeAppGameAndroidPhysicalDeviceProof(validPhysicalProof()),
      accessibilitySettings: {
        accessibilityEnabled: true,
        enabledServiceCount: 2,
        serviceNamesRedacted: true,
        settingsReadable: true,
      },
      generatedAt: '2026-06-08T18:20:00.000Z',
    });

    expect(readModel.preflightState).toBe('accessibility-service-enabled');
    expect(readModel.enabledServiceCount).toBe(2);
    expect(readModel.dispatchableActionCount).toBe(0);
    expect(readModel.blockedActionCount).toBe(4);
    expect(readModel.openBlockers).not.toContain('android-accessibility-service-not-enabled');
    expect(readModel.openBlockers).toContain('android-overlay-runtime-not-proved');
    expect(readModel.rows.every((row) => row.canDispatchAdapter === false)).toBe(true);
  });
}

function rejectsRawServiceNamesAndOverlayExecutionClaims() {
  it('rejects raw service names, overlay runtime, adapter dispatch, and platform enforcement claims', () => {
    expect(
      AppGameAndroidAccessibilitySettingsSampleSchema.safeParse({
        accessibilityEnabled: true,
        enabledServiceCount: 1,
        serviceNamesRedacted: false,
        settingsReadable: true,
      }).success
    ).toBe(false);

    const readModel = createAppGameAndroidAccessibilityOverlayPreflightReadModel({
      androidProof: decodeAppGameAndroidPhysicalDeviceProof(validPhysicalProof()),
      accessibilitySettings: {
        accessibilityEnabled: false,
        enabledServiceCount: 0,
        serviceNamesRedacted: true,
        settingsReadable: true,
      },
      generatedAt: '2026-06-08T18:20:00.000Z',
    });

    expect(
      AppGameAndroidAccessibilityOverlayPreflightReadModelSchema.safeParse({
        ...readModel,
        rows: [{ ...readModel.rows[0], overlayRuntimeClaimed: true }],
      }).success
    ).toBe(false);
    expect(
      AppGameAndroidAccessibilityOverlayPreflightReadModelSchema.safeParse({
        ...readModel,
        adapterDispatchClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameAndroidAccessibilityOverlayPreflightReadModelSchema.safeParse({
        ...readModel,
        rawAccessibilityServiceNamesClaimed: true,
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

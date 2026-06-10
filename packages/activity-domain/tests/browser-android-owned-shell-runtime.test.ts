import { describe, expect, it } from 'vitest';
import {
  BrowserAndroidOwnedShellRuntimeRowSchema,
  buildBrowserAndroidOwnedShellRuntimeReadModel,
} from '../src/browser-android-owned-shell-runtime';

describe('browser Android owned shell runtime', () => {
  it('projects physical visible owned-shell runtime without upgrading exact URL or enforcement', () => {
    const readModel = buildBrowserAndroidOwnedShellRuntimeReadModel(ownedShellProof());

    expect(readModel.productClaimed).toBe(false);
    expect(readModel.physicalVisibleRows).toBe(1);
    expect(readModel.manualRequiredRows).toBe(1);
    expect(readModel.rows.map((row) => row.runtimeState)).toEqual([
      'physical-visible-owned-shell',
      'emulator-browser-role-routing',
      'manual-required',
    ]);
    expect(readModel.rows[0]).toEqual({
      schemaVersion: 1,
      runtimeState: 'physical-visible-owned-shell',
      observedAt: '2026-06-08T21:38:28.673Z',
      sourceProofRef: 'browser-platform-android-owned-shell-proof',
      reasonCode: 'physical-android-owned-shell-visible-runtime-proof',
      physicalDeviceObserved: true,
      physicalInstallObserved: true,
      physicalExplicitLaunchObserved: true,
      physicalVisibleScreenshotObserved: true,
      physicalUiTreeObserved: true,
      emulatorDeviceOwnerOnly: true,
      emulatorBrowserRoleRoutingOnly: true,
      exactUrlPolicyClaimed: false,
      knownActiveTabProofClaimed: false,
      physicalDeviceOwnerClaimed: false,
      physicalBrowserRoleRoutingClaimed: false,
      vpnDnsBrowserProofClaimed: false,
      usageStatsRouteProofClaimed: false,
      accessibilityRouteProofClaimed: false,
      finalPolicyExecutionClaimed: false,
      enforcementClaimed: false,
    });
  });

  it('downgrades physical rows to manual-required when visible proof is missing', () => {
    const proof = ownedShellProof();
    proof.hostProofSummary.physicalDeviceScreenshotCaptured = false;
    proof.devices[1].screenshotCaptured = false;
    proof.devices[1].screenshotPersisted = false;
    const readModel = buildBrowserAndroidOwnedShellRuntimeReadModel(proof);

    expect(readModel.physicalVisibleRows).toBe(0);
    expect(readModel.manualRequiredRows).toBe(2);
    expect(readModel.rows.map((row) => row.runtimeState)).toEqual([
      'manual-required',
      'emulator-browser-role-routing',
      'manual-required',
    ]);
  });

  it('rejects dishonest physical Device Owner, Browser Role, exact URL, active-tab, and enforcement claims', () => {
    const accepted = buildBrowserAndroidOwnedShellRuntimeReadModel(ownedShellProof()).rows[0];

    for (const forbiddenClaim of [
      'exactUrlPolicyClaimed',
      'knownActiveTabProofClaimed',
      'physicalDeviceOwnerClaimed',
      'physicalBrowserRoleRoutingClaimed',
      'vpnDnsBrowserProofClaimed',
      'usageStatsRouteProofClaimed',
      'accessibilityRouteProofClaimed',
      'finalPolicyExecutionClaimed',
      'enforcementClaimed',
    ] as const) {
      expect(() => BrowserAndroidOwnedShellRuntimeRowSchema.parse({ ...accepted, [forbiddenClaim]: true })).toThrow();
    }
  });
});

function ownedShellProof() {
  return {
    schemaVersion: 1,
    proofId: 'browser-platform-android-owned-shell-proof',
    generatedAt: '2026-06-08T21:38:28.673Z',
    hostProofSummary: {
      physicalDeviceProofObserved: true,
      physicalDeviceInstallObserved: true,
      physicalDeviceActivityStartObserved: true,
      physicalDeviceExplicitLaunchObserved: true,
      physicalDeviceScreenshotCaptured: true,
      physicalDeviceUiTreeCaptured: true,
      deviceOwnerProofLimitedToProofLaunchedEmulator: true,
      deviceOwnerPolicyMutationLimitedToProofLaunchedEmulator: true,
      androidBrowserRoleAssignmentLimitedToProofLaunchedEmulator: true,
      exactUrlPolicyClaimed: false,
      knownActiveTabProofClaimed: false,
      deviceOwnerEnrollmentClaimed: true,
      deviceOwnerPolicyMutationClaimed: true,
      androidOwnedBrowserRoutingEnforcementClaimed: true,
      browserRoleAssignmentClaimed: false,
      vpnDnsBrowserProofClaimed: false,
      usageStatsRouteProofClaimed: false,
      accessibilityRouteProofClaimed: false,
      enforcementClaimed: false,
      physicalDeviceClaimBoundary:
        'physical-owned-shell-install-and-explicit-launch-only-no-device-owner-no-browser-role-no-enforcement',
      resultState: 'android-owned-browser-shell-browser-role-routing-proof',
    },
    devices: [
      {
        serialKind: 'emulator',
        proofLaunchedEmulator: true,
        packageInstalled: true,
        explicitActivityStartObserved: true,
        explicitActivityResumedObserved: true,
        explicitActivityFocusedObserved: true,
        explicitLaunchObserved: true,
        localProofPageObserved: true,
        implicitViewIntentLaunchObserved: true,
        uiTreeCaptured: true,
        uiTreeRawPersisted: false,
        screenshotCaptured: false,
        screenshotPersisted: false,
        exactUrlPolicyClaimed: false,
        knownActiveTabProofClaimed: false,
        deviceOwnerEnrollmentClaimed: true,
        deviceOwnerPolicyMutationClaimed: true,
        browserRoleAssignmentClaimed: false,
        androidOwnedBrowserRoutingEnforcementClaimed: true,
        enforcementClaimed: false,
      },
      {
        serialKind: 'physical-or-network-adb-device',
        proofLaunchedEmulator: false,
        packageInstalled: true,
        explicitActivityStartObserved: true,
        explicitActivityResumedObserved: true,
        explicitActivityFocusedObserved: true,
        explicitLaunchObserved: true,
        localProofPageObserved: true,
        implicitViewIntentLaunchObserved: false,
        uiTreeCaptured: true,
        uiTreeRawPersisted: false,
        screenshotCaptured: true,
        screenshotPersisted: true,
        exactUrlPolicyClaimed: false,
        knownActiveTabProofClaimed: false,
        deviceOwnerEnrollmentClaimed: false,
        deviceOwnerPolicyMutationClaimed: false,
        browserRoleAssignmentClaimed: false,
        androidOwnedBrowserRoutingEnforcementClaimed: false,
        enforcementClaimed: false,
      },
    ],
  };
}

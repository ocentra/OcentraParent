import { describe, expect, it } from 'vitest';
import {
  BrowserAndroidOwnedShellUrlCustodyReadModelSchema,
  BrowserAndroidOwnedShellUrlCustodyRowSchema,
  BrowserAndroidOwnedShellUrlCustodyState,
  buildBrowserAndroidOwnedShellUrlCustodyReadModel,
} from '../../src/browser-android-owned-shell-url-custody';

const Timestamp = '2026-06-08T23:34:00.000Z';

describe('browser Android owned shell URL custody', () => {
  it('projects a physical owned-shell requested URL ref without active-tab or enforcement claims', () => {
    const readModel = buildBrowserAndroidOwnedShellUrlCustodyReadModel(ownedShellProof());
    const urlRow = readModel.rows[0];
    const manualRow = readModel.rows[1];

    expect(readModel.physicalRequestedUrlRefRows).toBe(1);
    expect(readModel.manualRequiredRows).toBe(1);
    expect(readModel.exactActiveTabClaimed).toBe(false);
    expect(readModel.policyExecutionClaimed).toBe(false);
    expect(readModel.enforcementClaimed).toBe(false);
    expect(readModel.productClaimed).toBe(false);
    expect(urlRow).toEqual(expectedPhysicalUrlRow());
    expect(manualRow?.custodyState).toBe(BrowserAndroidOwnedShellUrlCustodyState.ManualRequired);
    expect(manualRow?.requestedUrlRef).toBeNull();
  });

  it('keeps URL custody manual-required when the physical screenshot proof is missing', () => {
    const proof = ownedShellProof({
      hostProofSummary: { physicalDeviceScreenshotCaptured: false },
      physicalDevice: { screenshotCaptured: false },
    });
    const readModel = buildBrowserAndroidOwnedShellUrlCustodyReadModel(proof);

    expect(readModel.physicalRequestedUrlRefRows).toBe(0);
    expect(readModel.manualRequiredRows).toBe(1);
    expect(readModel.rows).toEqual([expectedManualRow({ physicalVisibleScreenshotObserved: false })]);
  });

  it('rejects raw URL persistence, active-tab claims, physical routing claims, and forged counts', () => {
    const readModel = buildBrowserAndroidOwnedShellUrlCustodyReadModel(ownedShellProof());
    const urlRow = readModel.rows[0];

    expect(BrowserAndroidOwnedShellUrlCustodyRowSchema.safeParse({ ...urlRow, rawUrlPersisted: true }).success).toBe(
      false
    );
    expect(
      BrowserAndroidOwnedShellUrlCustodyRowSchema.safeParse({ ...urlRow, knownActiveTabProofClaimed: true }).success
    ).toBe(false);
    expect(
      BrowserAndroidOwnedShellUrlCustodyRowSchema.safeParse({ ...urlRow, physicalBrowserRoleRoutingClaimed: true })
        .success
    ).toBe(false);
    expect(
      BrowserAndroidOwnedShellUrlCustodyReadModelSchema.safeParse({
        ...readModel,
        physicalRequestedUrlRefRows: 2,
      }).success
    ).toBe(false);
  });
});

function expectedPhysicalUrlRow() {
  return {
    ...expectedManualRow({
      custodyState: BrowserAndroidOwnedShellUrlCustodyState.PhysicalOwnedShellRequestUrlRef,
      requestedUrlRef: 'redacted-android-owned-browser-proof-url-fbdf7f44a8a918d4',
      reasonCode: 'physical-owned-shell-view-intent-url-ref-custody',
      localProofPageObserved: true,
    }),
  };
}

function expectedManualRow(
  overrides: Partial<ReturnType<typeof baseExpectedRow>> = {}
): ReturnType<typeof baseExpectedRow> {
  return {
    ...baseExpectedRow(),
    ...overrides,
  };
}

function baseExpectedRow() {
  return {
    schemaVersion: 1,
    custodyState: BrowserAndroidOwnedShellUrlCustodyState.ManualRequired,
    observedAt: Timestamp,
    sourceProofRef: 'browser-platform-android-owned-shell-proof',
    requestedUrlRef: null,
    reasonCode: 'android-active-tab-policy-execution-and-enforcement-proof-required',
    physicalDeviceObserved: true,
    physicalInstallObserved: true,
    physicalExplicitLaunchObserved: true,
    physicalVisibleScreenshotObserved: true,
    physicalUiTreeObserved: true,
    ownedShellViewIntentDeclared: true,
    ownedShellWebViewDeclared: true,
    localProofPageObserved: false,
    rawUrlPersisted: false,
    exactUrlPolicyClaimed: false,
    knownActiveTabProofClaimed: false,
    physicalDeviceOwnerClaimed: false,
    physicalBrowserRoleRoutingClaimed: false,
    vpnDnsBrowserProofClaimed: false,
    usageStatsRouteProofClaimed: false,
    accessibilityRouteProofClaimed: false,
    finalPolicyExecutionClaimed: false,
    enforcementClaimed: false,
  };
}

type ProofOverrides = {
  readonly hostProofSummary?: Partial<ReturnType<typeof hostProofSummary>>;
  readonly physicalDevice?: Partial<ReturnType<typeof physicalDevice>>;
};

function ownedShellProof(overrides: ProofOverrides = {}) {
  return {
    schemaVersion: 1,
    proofId: 'browser-platform-android-owned-shell-proof',
    generatedAt: Timestamp,
    proofUrlRef: 'redacted-android-owned-browser-proof-url-fbdf7f44a8a918d4',
    proofUrlPersisted: false,
    hostProofSummary: {
      ...hostProofSummary(),
      ...overrides.hostProofSummary,
    },
    devices: [
      emulatorDevice(),
      {
        ...physicalDevice(),
        ...overrides.physicalDevice,
      },
    ],
    sourceBoundary: sourceBoundary(),
  };
}

function hostProofSummary() {
  return {
    physicalDeviceProofObserved: true,
    physicalDeviceInstallObserved: true,
    physicalDeviceExplicitLaunchObserved: true,
    physicalDeviceScreenshotCaptured: true,
    physicalDeviceUiTreeCaptured: true,
    deviceOwnerProofLimitedToProofLaunchedEmulator: true,
    deviceOwnerPolicyMutationLimitedToProofLaunchedEmulator: true,
    androidBrowserRoleAssignmentLimitedToProofLaunchedEmulator: true,
    exactUrlPolicyClaimed: false,
    knownActiveTabProofClaimed: false,
    vpnDnsBrowserProofClaimed: false,
    usageStatsRouteProofClaimed: false,
    accessibilityRouteProofClaimed: false,
    enforcementClaimed: false,
  };
}

function physicalDevice() {
  return {
    serialKind: 'physical-or-network-adb-device',
    proofLaunchedEmulator: false,
    packageInstalled: true,
    explicitLaunchObserved: true,
    localProofPageObserved: true,
    uiTreeCaptured: true,
    screenshotCaptured: true,
    rawUrlPersisted: false,
    exactUrlPolicyClaimed: false,
    knownActiveTabProofClaimed: false,
    deviceOwnerEnrollmentClaimed: false,
    deviceOwnerPolicyMutationClaimed: false,
    browserRoleAssignmentClaimed: false,
    androidOwnedBrowserRoutingEnforcementClaimed: false,
    enforcementClaimed: false,
  } as const;
}

function emulatorDevice() {
  return {
    ...physicalDevice(),
    serialKind: 'emulator',
    proofLaunchedEmulator: true,
    localProofPageObserved: true,
    screenshotCaptured: false,
    deviceOwnerEnrollmentClaimed: true,
    deviceOwnerPolicyMutationClaimed: true,
    browserRoleAssignmentClaimed: true,
    androidOwnedBrowserRoutingEnforcementClaimed: true,
  } as const;
}

function sourceBoundary() {
  return {
    webViewDeclared: true,
    browsableViewIntentDeclared: true,
    deviceOwnerPolicyMutationDeclared: true,
    accessibilityServiceDeclared: false,
    vpnServiceDeclared: false,
    usageStatsPermissionDeclared: false,
  };
}

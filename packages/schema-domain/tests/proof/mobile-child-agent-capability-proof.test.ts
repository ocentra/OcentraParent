import { describe, expect, it } from 'vitest';
import {
  type MobileChildAgentCapabilityReadModel,
  MobileChildAgentCapabilityReadModelSchema,
} from '@ocentra-parent/schema-domain/mobile-child-agent-capability-proof';

describe('mobile child-agent capability proof contracts', () => {
  acceptsHonestAndroidAndIosCapabilityMatrix();
  rejectsMissingAndroidOrIosRows();
  rejectsAndroidPrivilegedOverclaim();
  rejectsIosEntitlementOverclaim();
  rejectsIncompleteSourceProofs();
  rejectsPackageRuntimeHookOverclaim();
  rejectsMobileParityOrExternalTransportClaims();
});

function acceptsHonestAndroidAndIosCapabilityMatrix(): void {
  it('MobileChildAgentCapabilityReadModelSchema: accepts honest Android and iOS capability states', () => {
    const parsed = MobileChildAgentCapabilityReadModelSchema.parse(validReadModel());

    expect(parsed.schemaVersion).toBe('mobile-child-agent-capability-proof');
    expect(parsed.platforms).toHaveLength(2);
    expect(surfaceState(parsed, 'android-usage-stats')).toEqual({
      parentCapabilityStatus: 'manual-required',
      proofState: 'settings-grant-required',
    });
    expect(surfaceState(parsed, 'android-accessibility-service')).toEqual({
      parentCapabilityStatus: 'not-implemented',
      proofState: 'not-implemented',
    });
    expect(surfaceState(parsed, 'ios-family-controls')).toEqual({
      parentCapabilityStatus: 'manual-required',
      proofState: 'entitlement-required',
    });
    expect(hookState(parsed, 'ios-signing-profile')).toBe('signing-required');
  });
}

function rejectsMissingAndroidOrIosRows(): void {
  it('MobileChildAgentCapabilityReadModelSchema: rejects missing Android or iOS required rows', () => {
    const model = validReadModel();

    expect(
      MobileChildAgentCapabilityReadModelSchema.safeParse({
        ...model,
        capabilityRows: model.capabilityRows.filter((entry) => entry.surface !== 'android-device-owner'),
      }).success
    ).toBe(false);

    expect(
      MobileChildAgentCapabilityReadModelSchema.safeParse({
        ...model,
        capabilityRows: model.capabilityRows.filter((entry) => entry.surface !== 'ios-network-extension'),
      }).success
    ).toBe(false);
  });
}

function rejectsAndroidPrivilegedOverclaim(): void {
  it('MobileChildAgentCapabilityReadModelSchema: rejects Android privileged capability overclaims', () => {
    const model = validReadModel();

    expect(
      MobileChildAgentCapabilityReadModelSchema.safeParse({
        ...model,
        capabilityRows: replaceCapabilityRow(model, 'android-usage-stats', {
          parentCapabilityStatus: 'implemented',
          proofState: 'ci-mechanical-proof',
        }),
      }).success
    ).toBe(false);

    expect(
      MobileChildAgentCapabilityReadModelSchema.safeParse({
        ...model,
        capabilityRows: replaceCapabilityRow(model, 'android-device-owner', {
          parentCapabilityStatus: 'implemented',
          proofState: 'ci-mechanical-proof',
        }),
      }).success
    ).toBe(false);
  });
}

function rejectsIosEntitlementOverclaim(): void {
  it('MobileChildAgentCapabilityReadModelSchema: rejects iOS entitlement and distribution overclaims', () => {
    const model = validReadModel();

    expect(
      MobileChildAgentCapabilityReadModelSchema.safeParse({
        ...model,
        capabilityRows: replaceCapabilityRow(model, 'ios-family-controls', {
          parentCapabilityStatus: 'implemented',
          proofState: 'ci-mechanical-proof',
        }),
      }).success
    ).toBe(false);

    expect(
      MobileChildAgentCapabilityReadModelSchema.safeParse({
        ...model,
        capabilityRows: replaceCapabilityRow(model, 'ios-testflight', {
          parentCapabilityStatus: 'implemented',
          proofState: 'ci-mechanical-proof',
        }),
      }).success
    ).toBe(false);
  });
}

function rejectsIncompleteSourceProofs(): void {
  it('MobileChildAgentCapabilityReadModelSchema: rejects missing or unwired source proofs', () => {
    const model = validReadModel();

    expect(
      MobileChildAgentCapabilityReadModelSchema.safeParse({
        ...model,
        sourceProofs: model.sourceProofs.filter((entry) => entry.source !== 'child-android-device-proof-artifact-gate'),
      }).success
    ).toBe(false);

    expect(
      MobileChildAgentCapabilityReadModelSchema.safeParse({
        ...model,
        sourceProofs: model.sourceProofs.map((entry) =>
          entry.source === 'child-ios-entitlement-capability-proof'
            ? { ...entry, command: 'npm run wrong-ios-proof' }
            : entry
        ),
      }).success
    ).toBe(false);
  });
}

function rejectsPackageRuntimeHookOverclaim(): void {
  it('MobileChildAgentCapabilityReadModelSchema: rejects device, signing, and store hook upgrades', () => {
    const model = validReadModel();

    expect(
      MobileChildAgentCapabilityReadModelSchema.safeParse({
        ...model,
        packageRuntimeHooks: replaceRuntimeHook(model, 'android-device-install', {
          hookState: 'ci-mechanical-proof',
          evidencePath: 'test-results/mobile-child-agent-capability-proof/android-device-install.json',
        }),
      }).success
    ).toBe(false);

    expect(
      MobileChildAgentCapabilityReadModelSchema.safeParse({
        ...model,
        packageRuntimeHooks: replaceRuntimeHook(model, 'ios-signing-profile', {
          hookState: 'ci-mechanical-proof',
          evidencePath: 'test-results/mobile-child-agent-capability-proof/ios-signing-profile.json',
        }),
      }).success
    ).toBe(false);
  });
}

function rejectsMobileParityOrExternalTransportClaims(): void {
  it('MobileChildAgentCapabilityReadModelSchema: rejects mobile parity and external transport claims', () => {
    const model = validReadModel();

    expect(
      MobileChildAgentCapabilityReadModelSchema.safeParse({
        ...model,
        claimBoundaries: {
          ...model.claimBoundaries,
          childIosParity: 'claimed',
        },
      }).success
    ).toBe(false);

    expect(
      MobileChildAgentCapabilityReadModelSchema.safeParse({
        ...model,
        platforms: model.platforms.map((entry) =>
          entry.platform === 'android-child-agent' ? { ...entry, externalTransportState: 'ci-mechanical-proof' } : entry
        ),
      }).success
    ).toBe(false);
  });
}

function validReadModel(): MobileChildAgentCapabilityReadModel {
  return MobileChildAgentCapabilityReadModelSchema.parse({
    schemaVersion: 'mobile-child-agent-capability-proof',
    checkedAt: '2026-06-02T00:00:00.000Z',
    platforms: [
      {
        platform: 'android-child-agent',
        childAgentReadiness: 'manual-device-proof-required',
        packageRuntimeState: 'package-local-scaffold',
        privilegedOsState: 'blocked',
        externalTransportState: 'not-implemented',
        reviewerSummary: 'Android child-agent capability remains package-local until device proof artifacts exist',
      },
      {
        platform: 'ios-child-agent',
        childAgentReadiness: 'entitlement-review-required',
        packageRuntimeState: 'simulator-scaffold',
        privilegedOsState: 'entitlement-required',
        externalTransportState: 'not-implemented',
        reviewerSummary: 'iOS child-agent capability remains simulator and entitlement-review scoped',
      },
    ],
    sourceProofs: sourceProofs(),
    capabilityRows: capabilityRows(),
    packageRuntimeHooks: packageRuntimeHooks(),
    claimBoundaries: {
      parentMobileScope: 'separate-parent-mobile-workstream',
      childAndroidParity: 'not-claimed',
      childIosParity: 'not-claimed',
      privilegedOsBehavior: 'not-claimed',
      externalChildAgentTransport: 'not-claimed',
      storeDistribution: 'not-claimed',
      reviewerSummary:
        'Mobile child-agent parity requires real device, entitlement, signing, and store proof artifacts',
    },
    knownManualGaps: knownManualGaps(),
  });
}

function sourceProofs() {
  return [
    sourceProof('child-android-protocol-package-lifecycle-proof'),
    sourceProof('child-android-storage-protocol-capability-proof'),
    sourceProof('child-android-service-protocol-capability-proof'),
    sourceProof('child-android-permission-capability-proof'),
    sourceProof('child-android-privileged-capability-proof'),
    sourceProof('child-android-device-proof-artifact-gate'),
    sourceProof('child-ios-entitlement-capability-proof'),
  ];
}

function capabilityRows() {
  return [...androidCapabilityRows(), ...iosCapabilityRows()];
}

function androidCapabilityRows() {
  return [...androidPackageProtocolRows(), ...androidPrivilegedDeviceRows()];
}

function androidPackageProtocolRows() {
  return [
    androidRow(
      'android-foreground-service',
      'foreground-mobile-service',
      'manual-required',
      'device-proof-required',
      'child-android-service-protocol-capability-proof'
    ),
    androidRow(
      'android-storage-protocol-bridge',
      'local-storage',
      'scaffold',
      'package-local-scaffold',
      'child-android-storage-protocol-capability-proof'
    ),
    androidRow(
      'android-typed-protocol-bridge',
      'typed-protocol-bridge',
      'scaffold',
      'package-local-scaffold',
      'child-android-storage-protocol-capability-proof'
    ),
    androidRow(
      'android-notifications',
      'notifications',
      'manual-required',
      'manual-required',
      'child-android-permission-capability-proof'
    ),
  ];
}

function androidPrivilegedDeviceRows() {
  return [
    androidRow(
      'android-usage-stats',
      'usage-stats',
      'manual-required',
      'settings-grant-required',
      'child-android-privileged-capability-proof'
    ),
    androidRow(
      'android-accessibility-service',
      'accessibility-service',
      'not-implemented',
      'not-implemented',
      'child-android-privileged-capability-proof'
    ),
    androidRow(
      'android-vpn-dns',
      'vpn-dns-filtering',
      'not-implemented',
      'not-implemented',
      'child-android-privileged-capability-proof'
    ),
    androidRow(
      'android-device-owner',
      'device-owner-policy',
      'manual-required',
      'blocked',
      'child-android-privileged-capability-proof'
    ),
    androidRow(
      'android-managed-profile',
      'managed-profile',
      'manual-required',
      'blocked',
      'child-android-privileged-capability-proof'
    ),
    androidRow(
      'android-device-proof',
      'package-lifecycle',
      'manual-required',
      'device-proof-required',
      'child-android-device-proof-artifact-gate'
    ),
    androidRow(
      'android-play-signing',
      'store-distribution',
      'planned',
      'planned',
      'child-android-device-proof-artifact-gate'
    ),
    androidRow(
      'android-external-transport',
      'typed-protocol-bridge',
      'not-implemented',
      'not-implemented',
      'child-android-device-proof-artifact-gate'
    ),
  ];
}

function iosCapabilityRows() {
  return [
    iosRow('ios-simulator-status-surface', 'typed-protocol-bridge', 'scaffold', 'simulator-scaffold'),
    iosRow('ios-family-controls', 'family-controls-entitlement', 'manual-required', 'entitlement-required'),
    iosRow('ios-device-activity', 'device-activity', 'manual-required', 'entitlement-required'),
    iosRow('ios-screen-time', 'screen-time-api', 'manual-required', 'entitlement-required'),
    iosRow('ios-network-extension', 'network-extension', 'manual-required', 'entitlement-required'),
    iosRow('ios-notifications', 'notifications', 'manual-required', 'manual-required'),
    iosRow('ios-background-execution', 'background-execution', 'manual-required', 'manual-required'),
    iosRow('ios-signing', 'signing-entitlements', 'manual-required', 'signing-required'),
    iosRow('ios-testflight', 'testflight-distribution', 'manual-required', 'device-proof-required'),
    iosRow('ios-device-proof', 'package-lifecycle', 'manual-required', 'device-proof-required'),
    iosRow('ios-app-store', 'store-distribution', 'planned', 'planned'),
    iosRow('ios-external-transport', 'typed-protocol-bridge', 'not-implemented', 'not-implemented'),
  ];
}

function packageRuntimeHooks() {
  return [
    runtimeHook(
      'android-debug-apk-checksum',
      'android-child-agent',
      'ci-mechanical-proof',
      'target/release-packages/android/ocentra-parent-agent-android-debug-latest.apk.sha256',
      'child-android-device-proof-artifact-gate'
    ),
    runtimeHook(
      'android-package-local-status',
      'android-child-agent',
      'package-local-scaffold',
      'platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/OcentraParentAgentService.java',
      'child-android-device-proof-artifact-gate'
    ),
    runtimeHook(
      'android-device-install',
      'android-child-agent',
      'device-proof-required',
      null,
      'child-android-device-proof-artifact-gate'
    ),
    runtimeHook(
      'android-play-signing',
      'android-child-agent',
      'planned',
      null,
      'child-android-device-proof-artifact-gate'
    ),
    runtimeHook(
      'ios-xcode-target',
      'ios-child-agent',
      'ci-mechanical-proof',
      'platforms/ios/OcentraParentAgent.xcodeproj/project.pbxproj',
      'child-ios-entitlement-capability-proof'
    ),
    runtimeHook(
      'ios-simulator-status',
      'ios-child-agent',
      'simulator-scaffold',
      'platforms/ios/OcentraParentAgent/AgentStatusViewController.swift',
      'child-ios-entitlement-capability-proof'
    ),
    runtimeHook(
      'ios-signing-profile',
      'ios-child-agent',
      'signing-required',
      null,
      'child-ios-entitlement-capability-proof'
    ),
    runtimeHook(
      'ios-testflight-device',
      'ios-child-agent',
      'device-proof-required',
      null,
      'child-ios-entitlement-capability-proof'
    ),
  ];
}

function knownManualGaps() {
  return [
    'Android emulator install and launch evidence',
    'Android physical-device install and foreground service evidence',
    'Android POST_NOTIFICATIONS grant and delivery evidence',
    'Android UsageStats settings grant and observed event evidence',
    'Android AccessibilityService declaration, grant, and behavior',
    'Android VPN service and DNS filtering behavior',
    'Android Device Owner enrollment and policy action',
    'Android managed profile enrollment and behavior',
    'Android Play signing or release-track evidence',
    'iOS Family Controls entitlement approval and behavior',
    'iOS DeviceActivity schedule and event behavior',
    'iOS Network Extension entitlement and filtering behavior',
    'iOS notification and background execution behavior',
    'iOS signing, TestFlight, App Store, and physical-device evidence',
  ];
}

function sourceProof(source: string) {
  return {
    source,
    status: 'ci-mechanical-proof',
    command: `npm run test:${source}`,
    outputPath: `test-results/${source}/proof.json`,
  };
}

function androidRow(
  surface: string,
  parentCapability: string,
  parentCapabilityStatus: string,
  proofState: string,
  source: string
) {
  return capabilityRow(surface, 'android-child-agent', parentCapability, parentCapabilityStatus, proofState, source);
}

function iosRow(surface: string, parentCapability: string, parentCapabilityStatus: string, proofState: string) {
  return capabilityRow(
    surface,
    'ios-child-agent',
    parentCapability,
    parentCapabilityStatus,
    proofState,
    'child-ios-entitlement-capability-proof'
  );
}

function capabilityRow(
  surface: string,
  platform: string,
  parentCapability: string,
  parentCapabilityStatus: string,
  proofState: string,
  source: string
) {
  const proofRequirement = `${surface} remains ${proofState} until required platform artifacts change it`;
  return {
    surface,
    platform,
    parentCapability,
    parentCapabilityStatus,
    proofState,
    source,
    proofRequirement,
    claimBoundary: proofRequirement,
  };
}

function runtimeHook(hook: string, platform: string, hookState: string, evidencePath: string | null, source: string) {
  return { hook, platform, hookState, evidencePath, source };
}

function replaceCapabilityRow(
  model: MobileChildAgentCapabilityReadModel,
  surface: MobileChildAgentCapabilityReadModel['capabilityRows'][number]['surface'],
  patch: Record<string, unknown>
) {
  return model.capabilityRows.map((entry) => (entry.surface === surface ? { ...entry, ...patch } : entry));
}

function replaceRuntimeHook(
  model: MobileChildAgentCapabilityReadModel,
  hook: MobileChildAgentCapabilityReadModel['packageRuntimeHooks'][number]['hook'],
  patch: Record<string, unknown>
) {
  return model.packageRuntimeHooks.map((entry) => (entry.hook === hook ? { ...entry, ...patch } : entry));
}

function surfaceState(
  model: MobileChildAgentCapabilityReadModel,
  surface: MobileChildAgentCapabilityReadModel['capabilityRows'][number]['surface']
) {
  const entry = model.capabilityRows.find((row) => row.surface === surface);
  return {
    parentCapabilityStatus: entry?.parentCapabilityStatus,
    proofState: entry?.proofState,
  };
}

function hookState(
  model: MobileChildAgentCapabilityReadModel,
  hook: MobileChildAgentCapabilityReadModel['packageRuntimeHooks'][number]['hook']
) {
  return model.packageRuntimeHooks.find((entry) => entry.hook === hook)?.hookState;
}

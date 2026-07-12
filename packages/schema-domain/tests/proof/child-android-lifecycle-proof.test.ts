import { describe, expect, it } from 'vitest';
import {
  type ChildAndroidLifecycleReadModel,
  ChildAndroidLifecycleReadModelSchema,
} from '@ocentra-parent/schema-domain/child-android-lifecycle-proof';

describe('child android lifecycle proof contracts', () => {
  it('ChildAndroidLifecycleReadModelSchema: accepts package-local protocol bridge and split Android capability states', () => {
    const parsed = ChildAndroidLifecycleReadModelSchema.parse(validReadModel());

    expect(parsed.schemaVersion).toBe('child-android-protocol-package-lifecycle-proof');
    expect(parsed.packageProof.launchActivity).toBe('ca.ocentra.parent.agent/.MainActivity');
    expect(parsed.protocolBridgeProof.commands).toEqual([
      'child.android.lifecycle.snapshot.get',
      'child.android.capabilities.snapshot.get',
      'child.android.package.lifecycle.proof.get',
    ]);
    expect(capabilityState(parsed, 'typed-protocol-bridge')).toEqual({
      parentCapabilityStatus: 'scaffold',
      proofState: 'ci-mechanical-proof',
    });
    expect(capabilityState(parsed, 'device-owner-policy')).toEqual({
      parentCapabilityStatus: 'manual-required',
      proofState: 'manual-required',
    });
    expect(parsed.installAuthorityProof).toEqual(installAuthorityProof());
  });

  it('ChildAndroidLifecycleReadModelSchema: rejects missing Android capability rows', () => {
    const model = validReadModel();

    expect(
      ChildAndroidLifecycleReadModelSchema.safeParse({
        ...model,
        capabilityProofs: model.capabilityProofs.filter((entry) => entry.capability !== 'vpn-dns-filtering'),
      }).success
    ).toBe(false);
  });

  it('ChildAndroidLifecycleReadModelSchema: rejects package lifecycle upgraded beyond install/update proof', () => {
    const model = validReadModel();

    expect(
      ChildAndroidLifecycleReadModelSchema.safeParse({
        ...model,
        packageLifecycleAssertions: model.packageLifecycleAssertions.map((entry) =>
          entry.phase === 'install' ? { ...entry, proofState: 'ci-mechanical-proof' } : entry
        ),
      }).success
    ).toBe(false);
  });

  it('ChildAndroidLifecycleReadModelSchema: rejects external protocol transport claims from the package scaffold', () => {
    const model = validReadModel();

    expect(
      ChildAndroidLifecycleReadModelSchema.safeParse({
        ...model,
        protocolBridgeProof: {
          ...model.protocolBridgeProof,
          externalTransportState: 'package-local-scaffold',
        },
      }).success
    ).toBe(false);
  });
});

describe('child android lifecycle proof install-authority claims', () => {
  it('ChildAndroidLifecycleReadModelSchema: rejects install or launch states upgraded beyond debug APK sideload truth', () => {
    const model = validReadModel();

    expect(
      ChildAndroidLifecycleReadModelSchema.safeParse({
        ...model,
        installAuthorityProof: {
          ...model.installAuthorityProof,
          installState: 'manual-launch-proof-required' as never,
        },
      }).success
    ).toBe(false);

    expect(
      ChildAndroidLifecycleReadModelSchema.safeParse({
        ...model,
        installAuthorityProof: {
          ...model.installAuthorityProof,
          launchState: 'manual-removal-proof-required' as never,
        },
      }).success
    ).toBe(false);
  });
});

describe('child android lifecycle proof unsupported claims', () => {
  it('ChildAndroidLifecycleReadModelSchema: rejects implemented device-owner support without real enrollment proof', () => {
    const model = validReadModel();

    expect(
      ChildAndroidLifecycleReadModelSchema.safeParse({
        ...model,
        capabilityProofs: model.capabilityProofs.map((entry) =>
          entry.capability === 'device-owner-policy'
            ? { ...entry, parentCapabilityStatus: 'implemented', proofState: 'ci-mechanical-proof' }
            : entry
        ),
      }).success
    ).toBe(false);
  });

  it('ChildAndroidLifecycleReadModelSchema: rejects Android notification permission without a manual runtime grant boundary', () => {
    const model = validReadModel();

    expect(
      ChildAndroidLifecycleReadModelSchema.safeParse({
        ...model,
        permissionProofs: model.permissionProofs.map((entry) =>
          entry.permission === 'android.permission.POST_NOTIFICATIONS'
            ? { ...entry, runtimeGrantState: 'not-applicable' }
            : entry
        ),
      }).success
    ).toBe(false);
  });

  it('ChildAndroidLifecycleReadModelSchema: rejects managed-profile or device-owner authority claims without platform evidence', () => {
    const model = validReadModel();

    expect(
      ChildAndroidLifecycleReadModelSchema.safeParse({
        ...model,
        installAuthorityProof: {
          ...model.installAuthorityProof,
          deviceOwnerBoundary: 'device-owner claim is proved by CI package output',
        },
      }).success
    ).toBe(false);

    expect(
      ChildAndroidLifecycleReadModelSchema.safeParse({
        ...model,
        installAuthorityProof: {
          ...model.installAuthorityProof,
          managedProfileBoundary: 'managed-profile claim is proved by CI package output',
        },
      }).success
    ).toBe(false);
  });
});

function capabilityState(
  model: ChildAndroidLifecycleReadModel,
  capability: ChildAndroidLifecycleReadModel['capabilityProofs'][number]['capability']
) {
  const entry = model.capabilityProofs.find((proof) => proof.capability === capability);
  return {
    parentCapabilityStatus: entry?.parentCapabilityStatus,
    proofState: entry?.proofState,
  };
}

function validReadModel(): ChildAndroidLifecycleReadModel {
  return ChildAndroidLifecycleReadModelSchema.parse({
    schemaVersion: 'child-android-protocol-package-lifecycle-proof',
    packageProof: {
      packageId: 'ca.ocentra.parent.agent',
      applicationId: 'ca.ocentra.parent.agent',
      launchActivity: 'ca.ocentra.parent.agent/.MainActivity',
      foregroundService: 'ca.ocentra.parent.agent/.OcentraParentAgentService',
      nativeBridgeClass: 'ca.ocentra.parent.agent.ChildAndroidLifecycleProof',
      minSdk: 26,
      targetSdk: 35,
      versionName: '0.1.1',
      debugApkPath: 'target/release-packages/android/ocentra-parent-agent-android-debug-v0.1.1.apk',
      latestApkPath: 'target/release-packages/android/ocentra-parent-agent-android-debug-latest.apk',
      checksumState: 'ci-mechanical-proof',
      releaseCommand: 'cmd /c npm run release:package:android',
    },
    protocolBridgeProof: {
      bridgeState: 'package-local-scaffold',
      externalTransportState: 'not-implemented',
      commands: [
        'child.android.lifecycle.snapshot.get',
        'child.android.capabilities.snapshot.get',
        'child.android.package.lifecycle.proof.get',
      ],
      events: [
        'child.android.lifecycle.snapshot.reported',
        'child.android.capability.snapshot.reported',
        'child.android.package.lifecycle.proof.reported',
      ],
      nativeBridgeClass: 'ca.ocentra.parent.agent.ChildAndroidLifecycleProof',
      runtimeOwner: 'android-native-wrapper',
      proofRequirement: 'native wrapper exposes lifecycle and capability snapshot constants in the compiled package',
      claimBoundary: 'package-local scaffold is not a LAN/WebSocket child-agent protocol transport',
    },
    capabilityProofs: capabilityProofs(),
    packageLifecycleAssertions: lifecycleAssertions(),
    permissionProofs: [
      permissionProof('android.permission.FOREGROUND_SERVICE', 'declared-in-manifest', 'not-applicable'),
      permissionProof('android.permission.FOREGROUND_SERVICE_DATA_SYNC', 'declared-in-manifest', 'not-applicable'),
      permissionProof('android.permission.POST_NOTIFICATIONS', 'declared-in-manifest', 'manual-required'),
    ],
    installAuthorityProof: installAuthorityProof(),
    claimBoundaries: {
      childAndroidEnforcementParity: 'not claimed; this proof only covers package-local bridge mechanics',
      foregroundServiceRuntime:
        'foreground service declaration and Java start path compile, runtime behavior needs device proof',
      notificationRuntime: 'notification permission is declared but grant and delivery remain manual-required',
      accessibility: 'manual-required; no AccessibilityService behavior is claimed',
      vpnDns: 'manual-required; no VPN or DNS filtering adapter is claimed',
      deviceOwner: 'manual-required; no device-owner enrollment or policy action is claimed',
      managedProfile: 'manual-required; no managed-profile enrollment proof is present',
      usageStats: 'manual-required; no UsageStats permission grant or observation is claimed',
      packageLifecycle:
        'debug APK build and checksum are proved; install update background reboot uninstall remain manual',
      physicalDevice: 'manual-required; no emulator or physical-device run is claimed by CI',
      storeDistribution: 'planned; Google Play signing and release tracks are not wired',
    },
    updatedAt: '2026-05-31T00:00:00.000Z',
  });
}

function installAuthorityProof() {
  return {
    childAgentArtifactState: 'debug-apk-built',
    installMode: 'debug-apk-sideload',
    installState: 'manual-install-proof-required',
    launchState: 'manual-launch-proof-required',
    removalState: 'manual-removal-proof-required',
    deviceOwnerAuthorityState: 'manual-required',
    managedProfileAuthorityState: 'manual-required',
    childAgentArtifactBoundary:
      'debug APK is the Android child-agent artifact proved by CI package output and checksum only',
    installModeBoundary:
      'proof is limited to debug APK sideload mode and does not claim managed-profile or device-owner packaging',
    installStateBoundary: 'Android install remains manual-required until emulator or physical-device proof exists',
    launchStateBoundary:
      'Android launcher runtime remains manual-required until emulator or physical-device proof exists',
    removalStateBoundary: 'Android uninstall and removal behavior remain manual-required until device proof exists',
    deviceOwnerBoundary: 'manual-required; no device-owner claim is made without enrollment evidence',
    managedProfileBoundary: 'manual-required; no managed-profile claim is made without enrollment evidence',
  };
}

function capabilityProofs() {
  return [
    ...ciMechanicalCapabilityProofs(),
    ...manualCapabilityProofs(),
    capabilityProof(
      'store-distribution',
      'planned',
      'planned',
      'store-distribution',
      'Google Play signing and release tracks are planned, not wired'
    ),
  ];
}

function ciMechanicalCapabilityProofs() {
  return [
    capabilityProof(
      'foreground-mobile-service',
      'manual-required',
      'ci-mechanical-proof',
      'android-native-wrapper',
      'foreground service is declared and started by package code, but device runtime proof is still required'
    ),
    capabilityProof(
      'notifications',
      'manual-required',
      'manual-required',
      'android-os-permission',
      'notification permission grant and delivery require emulator or physical-device evidence'
    ),
    capabilityProof(
      'local-storage',
      'scaffold',
      'scaffold',
      'android-native-wrapper',
      'local storage remains scaffold until device persistence proof exists'
    ),
    capabilityProof(
      'typed-protocol-bridge',
      'scaffold',
      'ci-mechanical-proof',
      'android-native-wrapper',
      'typed package-local lifecycle bridge compiles but no LAN/WebSocket transport is claimed'
    ),
    capabilityProof(
      'package-lifecycle',
      'manual-required',
      'ci-mechanical-proof',
      'android-package-build',
      'debug APK build and checksum are CI proof, while install/update/background/reboot/uninstall remain manual'
    ),
  ];
}

function manualCapabilityProofs() {
  return [
    capabilityProof(
      'usage-stats',
      'manual-required',
      'manual-required',
      'manual-device-proof',
      'UsageStats needs a real permission grant and observation artifact'
    ),
    capabilityProof(
      'accessibility-service',
      'manual-required',
      'manual-required',
      'manual-device-proof',
      'Accessibility requires explicit service grant and device behavior proof'
    ),
    capabilityProof(
      'vpn-dns-filtering',
      'manual-required',
      'manual-required',
      'manual-device-proof',
      'VPN or DNS filtering needs approved adapter and device proof'
    ),
    capabilityProof(
      'device-owner-policy',
      'manual-required',
      'manual-required',
      'manual-device-proof',
      'device-owner policy requires enrollment and policy action proof'
    ),
    capabilityProof(
      'managed-profile',
      'manual-required',
      'manual-required',
      'manual-device-proof',
      'managed profile behavior requires enrollment proof'
    ),
  ];
}

function lifecycleAssertions() {
  return [
    lifecycleAssertion('debug-apk-build', 'ci-mechanical-proof', 'android-package-build'),
    lifecycleAssertion('checksum', 'ci-mechanical-proof', 'android-package-build'),
    lifecycleAssertion('launcher-activity', 'ci-mechanical-proof', 'android-manifest'),
    lifecycleAssertion('foreground-service-registration', 'ci-mechanical-proof', 'android-manifest'),
    lifecycleAssertion('notification-permission-declared', 'ci-mechanical-proof', 'android-manifest'),
    lifecycleAssertion('install', 'manual-required', 'manual-device-proof'),
    lifecycleAssertion('update', 'manual-required', 'manual-device-proof'),
    lifecycleAssertion('background-execution', 'manual-required', 'manual-device-proof'),
    lifecycleAssertion('reboot-recovery', 'manual-required', 'manual-device-proof'),
    lifecycleAssertion('uninstall', 'manual-required', 'manual-device-proof'),
  ];
}

function capabilityProof(
  capability: string,
  parentCapabilityStatus: string,
  proofState: string,
  runtimeOwner: string,
  proofRequirement: string
) {
  return {
    capability,
    parentCapability: capability,
    parentCapabilityStatus,
    proofState,
    runtimeOwner,
    proofRequirement,
    claimBoundary: proofRequirement,
  };
}

function lifecycleAssertion(phase: string, proofState: string, runtimeOwner: string) {
  return {
    phase,
    proofState,
    runtimeOwner,
    proofRequirement: `${phase} proof state remains ${proofState}`,
    claimBoundary: `${phase} does not upgrade unproven Android runtime behavior`,
  };
}

function permissionProof(permission: string, declarationState: string, runtimeGrantState: string) {
  return {
    permission,
    declarationState,
    runtimeGrantState,
    proofRequirement: `${permission} declaration is parsed while runtime grant remains ${runtimeGrantState}`,
  };
}

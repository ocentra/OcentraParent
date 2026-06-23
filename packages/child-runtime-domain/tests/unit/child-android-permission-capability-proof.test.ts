import { describe, expect, it } from 'vitest';
import {
  type ChildAndroidPermissionCapabilityReadModel,
  ChildAndroidPermissionCapabilityReadModelSchema,
} from '@ocentra-parent/schema-domain/child-android-permission-capability-proof';

describe('child android permission capability proof contracts', () => {
  acceptsHonestPackageAndPermissionStates();
  rejectsMissingPermissionRows();
  rejectsAutomaticNotificationGrant();
  rejectsUsageStatsUpgrade();
  rejectsAccessibilityOrVpnDeclarations();
  rejectsDeviceOwnerAndInstallUpgrades();
});

function acceptsHonestPackageAndPermissionStates(): void {
  it('ChildAndroidPermissionCapabilityReadModelSchema: accepts honest package and permission states', () => {
    const parsed = ChildAndroidPermissionCapabilityReadModelSchema.parse(validReadModel());

    expect(parsed.schemaVersion).toBe('child-android-permission-capability-proof');
    expect(parsed.nativeBridgeClass).toBe('ca.ocentra.parent.agent.ChildAndroidPermissionCapabilityProof');
    expect(parsed.protocolBridgeProof.commands).toEqual([
      'child.android.permission.capability.snapshot.get',
      'child.android.permission.package.proof.get',
      'child.android.permission.runtime.manual-proof.get',
    ]);
    expect(permissionState(parsed, 'android.permission.POST_NOTIFICATIONS')).toEqual({
      declarationState: 'declared-in-manifest',
      runtimeGrantState: 'manual-runtime-required',
      proofState: 'manual-required',
    });
    expect(permissionState(parsed, 'android.permission.PACKAGE_USAGE_STATS')).toEqual({
      declarationState: 'not-declared-by-design',
      runtimeGrantState: 'manual-settings-required',
      proofState: 'settings-grant-required',
    });
    expect(adapterState(parsed, 'device-owner-policy')).toEqual({
      parentCapabilityStatus: 'manual-required',
      adapterState: 'blocked-without-enrollment',
      proofState: 'blocked',
    });
  });
}

function rejectsMissingPermissionRows(): void {
  it('ChildAndroidPermissionCapabilityReadModelSchema: rejects missing permission rows', () => {
    const model = validReadModel();

    expect(
      ChildAndroidPermissionCapabilityReadModelSchema.safeParse({
        ...model,
        permissionProofs: model.permissionProofs.filter(
          (entry) => entry.permission !== 'android.permission.PACKAGE_USAGE_STATS'
        ),
      }).success
    ).toBe(false);
  });
}

function rejectsAutomaticNotificationGrant(): void {
  it('ChildAndroidPermissionCapabilityReadModelSchema: rejects notification permission as automatically granted', () => {
    const model = validReadModel();

    expect(
      ChildAndroidPermissionCapabilityReadModelSchema.safeParse({
        ...model,
        permissionProofs: model.permissionProofs.map((entry) =>
          entry.permission === 'android.permission.POST_NOTIFICATIONS'
            ? { ...entry, runtimeGrantState: 'not-applicable' }
            : entry
        ),
      }).success
    ).toBe(false);
  });
}

function rejectsUsageStatsUpgrade(): void {
  it('ChildAndroidPermissionCapabilityReadModelSchema: rejects UsageStats as declared or implemented without device proof', () => {
    const model = validReadModel();

    expect(
      ChildAndroidPermissionCapabilityReadModelSchema.safeParse({
        ...model,
        permissionProofs: model.permissionProofs.map((entry) =>
          entry.permission === 'android.permission.PACKAGE_USAGE_STATS'
            ? {
                ...entry,
                declarationState: 'declared-in-manifest',
                runtimeGrantState: 'not-applicable',
                proofState: 'ci-mechanical-proof',
              }
            : entry
        ),
      }).success
    ).toBe(false);
  });
}

function rejectsAccessibilityOrVpnDeclarations(): void {
  it('ChildAndroidPermissionCapabilityReadModelSchema: rejects accessibility or VPN/DNS declarations without adapters', () => {
    const model = validReadModel();

    expect(
      ChildAndroidPermissionCapabilityReadModelSchema.safeParse({
        ...model,
        adapterProofs: model.adapterProofs.map((entry) =>
          entry.surface === 'accessibility-service'
            ? {
                ...entry,
                parentCapabilityStatus: 'implemented',
                adapterState: 'package-local-scaffold',
                proofState: 'ci-mechanical-proof',
              }
            : entry
        ),
      }).success
    ).toBe(false);

    expect(
      ChildAndroidPermissionCapabilityReadModelSchema.safeParse({
        ...model,
        adapterProofs: model.adapterProofs.map((entry) =>
          entry.surface === 'vpn-dns-service'
            ? {
                ...entry,
                parentCapabilityStatus: 'implemented',
                adapterState: 'package-local-scaffold',
                proofState: 'ci-mechanical-proof',
              }
            : entry
        ),
      }).success
    ).toBe(false);
  });
}

function rejectsDeviceOwnerAndInstallUpgrades(): void {
  it('ChildAndroidPermissionCapabilityReadModelSchema: rejects device-owner and install lifecycle upgrades', () => {
    const model = validReadModel();

    expect(
      ChildAndroidPermissionCapabilityReadModelSchema.safeParse({
        ...model,
        adapterProofs: model.adapterProofs.map((entry) =>
          entry.surface === 'device-owner-policy'
            ? { ...entry, parentCapabilityStatus: 'implemented', proofState: 'ci-mechanical-proof' }
            : entry
        ),
      }).success
    ).toBe(false);

    expect(
      ChildAndroidPermissionCapabilityReadModelSchema.safeParse({
        ...model,
        packageLifecycleProofs: model.packageLifecycleProofs.map((entry) =>
          entry.phase === 'install' ? { ...entry, proofState: 'ci-mechanical-proof' } : entry
        ),
      }).success
    ).toBe(false);
  });
}

function validReadModel(): ChildAndroidPermissionCapabilityReadModel {
  return ChildAndroidPermissionCapabilityReadModelSchema.parse({
    schemaVersion: 'child-android-permission-capability-proof',
    packageId: 'ca.ocentra.parent.agent',
    nativeBridgeClass: 'ca.ocentra.parent.agent.ChildAndroidPermissionCapabilityProof',
    protocolBridgeProof: {
      packageId: 'ca.ocentra.parent.agent',
      nativeBridgeClass: 'ca.ocentra.parent.agent.ChildAndroidPermissionCapabilityProof',
      bridgeState: 'package-local-scaffold',
      externalTransportState: 'not-implemented',
      commands: [
        'child.android.permission.capability.snapshot.get',
        'child.android.permission.package.proof.get',
        'child.android.permission.runtime.manual-proof.get',
      ],
      events: [
        'child.android.permission.capability.snapshot.reported',
        'child.android.permission.package.proof.reported',
        'child.android.permission.runtime.manual-proof.reported',
      ],
      runtimeOwner: 'android-native-wrapper',
      proofRequirement: 'permission bridge constants compile into the Android debug package',
      claimBoundary: 'permission bridge is package-local and not external child-agent transport',
    },
    permissionProofs: permissionProofs(),
    adapterProofs: adapterProofs(),
    packageLifecycleProofs: packageLifecycleProofs(),
    claimBoundaries: {
      packageLifecycle: 'debug APK build and checksum are proved; install update reboot uninstall remain manual',
      foregroundService:
        'foreground service permissions are declared, but runtime foreground behavior needs device proof',
      notifications: 'POST_NOTIFICATIONS is declared, but runtime grant and delivery are manual-required',
      usageStats: 'UsageStats needs settings grant and observation artifact before it is available',
      accessibility: 'no AccessibilityService declaration, grant, or behavior is claimed',
      vpnDns: 'no VPN service, DNS adapter, or filtering behavior is claimed',
      deviceOwner: 'device-owner remains blocked without enrollment and policy action proof',
      managedProfile: 'managed-profile remains blocked without enrollment proof',
      appPrivateStorage: 'app-private storage path is package-local scaffold only',
      backgroundLifecycle: 'background, reboot, and uninstall behavior require device proof',
      externalTransport: 'no LAN/WebSocket Android child-agent permission transport is claimed',
    },
    updatedAt: '2026-05-31T00:00:00.000Z',
  });
}

function permissionProofs() {
  return [
    permissionProof(
      'android.permission.FOREGROUND_SERVICE',
      'foreground-mobile-service',
      'declared-in-manifest',
      'not-applicable',
      'declared-in-manifest',
      'android-manifest'
    ),
    permissionProof(
      'android.permission.FOREGROUND_SERVICE_DATA_SYNC',
      'foreground-mobile-service',
      'declared-in-manifest',
      'not-applicable',
      'declared-in-manifest',
      'android-manifest'
    ),
    permissionProof(
      'android.permission.POST_NOTIFICATIONS',
      'notifications',
      'declared-in-manifest',
      'manual-runtime-required',
      'manual-required',
      'android-os-permission'
    ),
    permissionProof(
      'android.permission.PACKAGE_USAGE_STATS',
      'usage-stats',
      'not-declared-by-design',
      'manual-settings-required',
      'settings-grant-required',
      'manual-device-proof'
    ),
  ];
}

function adapterProofs() {
  return [...packageAdapterProofs(), ...permissionAdapterProofs(), ...policyAdapterProofs()];
}

function packageAdapterProofs() {
  return [
    adapterProof(
      'package-debug-apk',
      'package-lifecycle',
      'manual-required',
      'package-local-scaffold',
      'ci-mechanical-proof',
      'android-package-build'
    ),
    adapterProof(
      'foreground-service-permission',
      'foreground-mobile-service',
      'manual-required',
      'package-local-scaffold',
      'ci-mechanical-proof',
      'android-manifest'
    ),
    adapterProof(
      'app-private-storage',
      'local-storage',
      'scaffold',
      'package-local-scaffold',
      'package-local-scaffold',
      'android-app-private-storage'
    ),
    adapterProof(
      'background-service-lifecycle',
      'background-execution',
      'manual-required',
      'not-implemented',
      'manual-required',
      'manual-device-proof'
    ),
  ];
}

function permissionAdapterProofs() {
  return [
    adapterProof(
      'post-notifications-permission',
      'notifications',
      'manual-required',
      'declared-in-manifest',
      'manual-required',
      'android-os-permission'
    ),
    adapterProof(
      'usage-stats-permission',
      'usage-stats',
      'manual-required',
      'not-declared',
      'settings-grant-required',
      'manual-device-proof'
    ),
    adapterProof(
      'accessibility-service',
      'accessibility-service',
      'not-implemented',
      'not-declared',
      'not-implemented',
      'android-accessibility-service'
    ),
    adapterProof(
      'vpn-dns-service',
      'vpn-dns-filtering',
      'not-implemented',
      'not-declared',
      'not-implemented',
      'android-vpn-service'
    ),
  ];
}

function policyAdapterProofs() {
  return [
    adapterProof(
      'device-owner-policy',
      'device-owner-policy',
      'manual-required',
      'blocked-without-enrollment',
      'blocked',
      'android-policy-provider'
    ),
    adapterProof(
      'managed-profile',
      'managed-profile',
      'manual-required',
      'blocked-without-enrollment',
      'blocked',
      'android-policy-provider'
    ),
  ];
}

function packageLifecycleProofs() {
  return [
    lifecycleProof('debug-apk-build', 'ci-mechanical-proof', 'android-package-build'),
    lifecycleProof('checksum', 'ci-mechanical-proof', 'android-package-build'),
    lifecycleProof('launcher-activity', 'ci-mechanical-proof', 'android-manifest'),
    lifecycleProof('foreground-service-registration', 'ci-mechanical-proof', 'android-manifest'),
    lifecycleProof('notification-permission-declared', 'ci-mechanical-proof', 'android-manifest'),
    lifecycleProof('app-private-storage-path', 'ci-mechanical-proof', 'android-app-private-storage'),
    lifecycleProof('background-service-start', 'manual-required', 'manual-device-proof'),
    lifecycleProof('install', 'manual-required', 'manual-device-proof'),
    lifecycleProof('update', 'manual-required', 'manual-device-proof'),
    lifecycleProof('reboot-recovery', 'manual-required', 'manual-device-proof'),
    lifecycleProof('uninstall', 'manual-required', 'manual-device-proof'),
  ];
}

function permissionProof(
  permission: string,
  parentCapability: string,
  declarationState: string,
  runtimeGrantState: string,
  proofState: string,
  runtimeOwner: string
) {
  const proofRequirement = `${permission} remains ${runtimeGrantState} until device proof changes it`;
  return {
    permission,
    parentCapability,
    declarationState,
    runtimeGrantState,
    proofState,
    runtimeOwner,
    proofRequirement,
    claimBoundary: proofRequirement,
  };
}

function adapterProof(
  surface: string,
  parentCapability: string,
  parentCapabilityStatus: string,
  adapterState: string,
  proofState: string,
  runtimeOwner: string
) {
  const proofRequirement = `${surface} remains ${adapterState} with ${proofState}`;
  return {
    surface,
    parentCapability,
    parentCapabilityStatus,
    adapterState,
    proofState,
    runtimeOwner,
    proofRequirement,
    claimBoundary: proofRequirement,
  };
}

function lifecycleProof(phase: string, proofState: string, runtimeOwner: string) {
  return {
    phase,
    proofState,
    runtimeOwner,
    proofRequirement: `${phase} proof state is ${proofState}`,
    claimBoundary: `${phase} does not upgrade Android runtime behavior without device evidence`,
  };
}

function permissionState(
  model: ChildAndroidPermissionCapabilityReadModel,
  permission: ChildAndroidPermissionCapabilityReadModel['permissionProofs'][number]['permission']
) {
  const entry = model.permissionProofs.find((proof) => proof.permission === permission);
  return {
    declarationState: entry?.declarationState,
    runtimeGrantState: entry?.runtimeGrantState,
    proofState: entry?.proofState,
  };
}

function adapterState(
  model: ChildAndroidPermissionCapabilityReadModel,
  surface: ChildAndroidPermissionCapabilityReadModel['adapterProofs'][number]['surface']
) {
  const entry = model.adapterProofs.find((proof) => proof.surface === surface);
  return {
    parentCapabilityStatus: entry?.parentCapabilityStatus,
    adapterState: entry?.adapterState,
    proofState: entry?.proofState,
  };
}

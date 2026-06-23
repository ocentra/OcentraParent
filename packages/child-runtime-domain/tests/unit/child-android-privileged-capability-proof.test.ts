import { describe, expect, it } from 'vitest';
import {
  type ChildAndroidPrivilegedCapabilityReadModel,
  ChildAndroidPrivilegedCapabilityReadModelSchema,
} from '@ocentra-parent/schema-domain/child-android-privileged-capability-proof';

describe('child Android privileged capability proof contracts', () => {
  acceptsHonestPrivilegedCapabilityStates();
  rejectsMissingPrivilegedRows();
  rejectsUsageStatsRuntimeUpgrade();
  rejectsAccessibilityOrVpnImplementationClaims();
  rejectsDeviceOwnerOrManagedProfileEnrollmentClaims();
  rejectsDeviceProofOrTransportUpgrade();
});

function acceptsHonestPrivilegedCapabilityStates(): void {
  it('ChildAndroidPrivilegedCapabilityReadModelSchema: accepts honest privileged capability states', () => {
    const parsed = ChildAndroidPrivilegedCapabilityReadModelSchema.parse(validReadModel());

    expect(parsed.schemaVersion).toBe('child-android-privileged-capability-proof');
    expect(parsed.nativeBridgeClass).toBe('ca.ocentra.parent.agent.ChildAndroidPrivilegedCapabilityProof');
    expect(parsed.protocolBridgeProof.commands).toEqual([
      'child.android.privileged.capability.snapshot.get',
      'child.android.privileged.settings-proof.get',
      'child.android.privileged.enrollment-proof.get',
    ]);
    expect(surfaceState(parsed, 'usage-stats-settings-access')).toEqual({
      parentCapabilityStatus: 'manual-required',
      declarationState: 'not-declared-by-design',
      runtimeGrantState: 'manual-settings-required',
      proofState: 'settings-grant-required',
    });
    expect(surfaceState(parsed, 'device-owner-enrollment')).toEqual({
      parentCapabilityStatus: 'manual-required',
      declarationState: 'not-declared',
      runtimeGrantState: 'blocked',
      proofState: 'blocked',
    });
  });
}

function rejectsMissingPrivilegedRows(): void {
  it('ChildAndroidPrivilegedCapabilityReadModelSchema: rejects missing privileged capability rows', () => {
    const model = validReadModel();

    expect(
      ChildAndroidPrivilegedCapabilityReadModelSchema.safeParse({
        ...model,
        privilegedSurfaceProofs: model.privilegedSurfaceProofs.filter(
          (entry) => entry.surface !== 'accessibility-service-adapter'
        ),
      }).success
    ).toBe(false);
  });
}

function rejectsUsageStatsRuntimeUpgrade(): void {
  it('ChildAndroidPrivilegedCapabilityReadModelSchema: rejects UsageStats as granted or observed without device proof', () => {
    const model = validReadModel();

    expect(
      ChildAndroidPrivilegedCapabilityReadModelSchema.safeParse({
        ...model,
        privilegedSurfaceProofs: model.privilegedSurfaceProofs.map((entry) =>
          entry.surface === 'usage-stats-settings-access'
            ? {
                ...entry,
                parentCapabilityStatus: 'implemented',
                runtimeGrantState: 'not-applicable',
                proofState: 'ci-mechanical-proof',
              }
            : entry
        ),
      }).success
    ).toBe(false);

    expect(
      ChildAndroidPrivilegedCapabilityReadModelSchema.safeParse({
        ...model,
        privilegedSurfaceProofs: model.privilegedSurfaceProofs.map((entry) =>
          entry.surface === 'usage-stats-observation'
            ? {
                ...entry,
                parentCapabilityStatus: 'implemented',
                runtimeGrantState: 'not-applicable',
                proofState: 'ci-mechanical-proof',
              }
            : entry
        ),
      }).success
    ).toBe(false);
  });
}

function rejectsAccessibilityOrVpnImplementationClaims(): void {
  it('ChildAndroidPrivilegedCapabilityReadModelSchema: rejects Accessibility or VPN/DNS implementation claims', () => {
    const model = validReadModel();

    expect(
      ChildAndroidPrivilegedCapabilityReadModelSchema.safeParse({
        ...model,
        privilegedSurfaceProofs: model.privilegedSurfaceProofs.map((entry) =>
          entry.surface === 'accessibility-service-adapter'
            ? {
                ...entry,
                parentCapabilityStatus: 'implemented',
                declarationState: 'status-bundle-label',
                runtimeGrantState: 'not-applicable',
                proofState: 'ci-mechanical-proof',
              }
            : entry
        ),
      }).success
    ).toBe(false);

    expect(
      ChildAndroidPrivilegedCapabilityReadModelSchema.safeParse({
        ...model,
        privilegedSurfaceProofs: model.privilegedSurfaceProofs.map((entry) =>
          entry.surface === 'vpn-service-adapter'
            ? {
                ...entry,
                parentCapabilityStatus: 'implemented',
                declarationState: 'status-bundle-label',
                proofState: 'ci-mechanical-proof',
              }
            : entry
        ),
      }).success
    ).toBe(false);
  });
}

function rejectsDeviceOwnerOrManagedProfileEnrollmentClaims(): void {
  it('ChildAndroidPrivilegedCapabilityReadModelSchema: rejects device-owner or managed-profile enrollment claims', () => {
    const model = validReadModel();

    expect(
      ChildAndroidPrivilegedCapabilityReadModelSchema.safeParse({
        ...model,
        privilegedSurfaceProofs: model.privilegedSurfaceProofs.map((entry) =>
          entry.surface === 'device-owner-enrollment'
            ? {
                ...entry,
                parentCapabilityStatus: 'implemented',
                runtimeGrantState: 'not-applicable',
                proofState: 'ci-mechanical-proof',
              }
            : entry
        ),
      }).success
    ).toBe(false);

    expect(
      ChildAndroidPrivilegedCapabilityReadModelSchema.safeParse({
        ...model,
        privilegedSurfaceProofs: model.privilegedSurfaceProofs.map((entry) =>
          entry.surface === 'managed-profile-enrollment'
            ? {
                ...entry,
                parentCapabilityStatus: 'implemented',
                runtimeGrantState: 'not-applicable',
                proofState: 'ci-mechanical-proof',
              }
            : entry
        ),
      }).success
    ).toBe(false);
  });
}

function rejectsDeviceProofOrTransportUpgrade(): void {
  it('ChildAndroidPrivilegedCapabilityReadModelSchema: rejects device proof or external transport upgrades', () => {
    const model = validReadModel();

    expect(
      ChildAndroidPrivilegedCapabilityReadModelSchema.safeParse({
        ...model,
        privilegedSurfaceProofs: model.privilegedSurfaceProofs.map((entry) =>
          entry.surface === 'physical-device-proof'
            ? { ...entry, proofState: 'ci-mechanical-proof', runtimeGrantState: 'not-applicable' }
            : entry
        ),
      }).success
    ).toBe(false);

    expect(
      ChildAndroidPrivilegedCapabilityReadModelSchema.safeParse({
        ...model,
        protocolBridgeProof: {
          ...model.protocolBridgeProof,
          externalTransportState: 'package-local-scaffold',
        },
      }).success
    ).toBe(false);
  });
}

function validReadModel(): ChildAndroidPrivilegedCapabilityReadModel {
  return ChildAndroidPrivilegedCapabilityReadModelSchema.parse({
    schemaVersion: 'child-android-privileged-capability-proof',
    packageId: 'ca.ocentra.parent.agent',
    nativeBridgeClass: 'ca.ocentra.parent.agent.ChildAndroidPrivilegedCapabilityProof',
    protocolBridgeProof: {
      packageId: 'ca.ocentra.parent.agent',
      nativeBridgeClass: 'ca.ocentra.parent.agent.ChildAndroidPrivilegedCapabilityProof',
      bridgeState: 'package-local-scaffold',
      externalTransportState: 'not-implemented',
      commands: [
        'child.android.privileged.capability.snapshot.get',
        'child.android.privileged.settings-proof.get',
        'child.android.privileged.enrollment-proof.get',
      ],
      events: [
        'child.android.privileged.capability.snapshot.reported',
        'child.android.privileged.settings-proof.reported',
        'child.android.privileged.enrollment-proof.reported',
      ],
      runtimeOwner: 'android-native-wrapper',
      proofRequirement: 'privileged capability status labels compile into the Android debug package',
      claimBoundary: 'privileged proof is package-local and not external child-agent transport',
    },
    privilegedSurfaceProofs: privilegedSurfaceProofs(),
    claimBoundaries: {
      usageStats: 'UsageStats requires user settings grant and observation artifact before support is claimed',
      accessibility: 'AccessibilityService remains not declared and not implemented',
      vpnDns: 'VPN service and DNS filtering remain not declared and not implemented',
      deviceOwner: 'device-owner policy remains blocked without enrollment and policy action proof',
      managedProfile: 'managed profile remains blocked without enrollment proof',
      statusBundle: 'native status Bundle labels are package-local scaffold proof only',
      physicalDevice: 'physical-device install and runtime behavior remain device-proof-required',
      externalTransport: 'no LAN or WebSocket child-agent privileged transport is claimed',
    },
    updatedAt: '2026-05-31T00:00:00.000Z',
  });
}

function privilegedSurfaceProofs() {
  return [...usageStatsSurfaceProofs(), ...privilegedAdapterSurfaceProofs(), ...enrollmentSurfaceProofs()];
}

function usageStatsSurfaceProofs() {
  return [
    surfaceProof(
      'usage-stats-settings-access',
      'usage-stats',
      'manual-required',
      'not-declared-by-design',
      'manual-settings-required',
      'settings-grant-required',
      'android-settings-panel'
    ),
    surfaceProof(
      'usage-stats-observation',
      'usage-stats',
      'manual-required',
      'not-applicable',
      'manual-device-required',
      'manual-device-proof',
      'android-usage-stats-manager'
    ),
  ];
}

function privilegedAdapterSurfaceProofs() {
  return [
    surfaceProof(
      'accessibility-service-adapter',
      'accessibility-service',
      'not-implemented',
      'not-declared',
      'unavailable',
      'not-implemented',
      'android-accessibility-service'
    ),
    surfaceProof(
      'vpn-service-adapter',
      'vpn-dns-filtering',
      'not-implemented',
      'not-declared',
      'unavailable',
      'not-implemented',
      'android-vpn-service'
    ),
    surfaceProof(
      'dns-filtering-adapter',
      'vpn-dns-filtering',
      'not-implemented',
      'not-declared',
      'not-implemented',
      'not-implemented',
      'android-dns-filtering'
    ),
  ];
}

function enrollmentSurfaceProofs() {
  return [
    surfaceProof(
      'device-owner-enrollment',
      'device-owner-policy',
      'manual-required',
      'not-declared',
      'blocked',
      'blocked',
      'android-device-policy-manager'
    ),
    surfaceProof(
      'managed-profile-enrollment',
      'managed-profile',
      'manual-required',
      'not-declared',
      'blocked',
      'blocked',
      'android-managed-profile-owner'
    ),
    surfaceProof(
      'privileged-status-bundle',
      'typed-protocol-bridge',
      'scaffold',
      'status-bundle-label',
      'not-applicable',
      'package-local-scaffold',
      'android-native-wrapper'
    ),
    surfaceProof(
      'physical-device-proof',
      'package-lifecycle',
      'manual-required',
      'not-applicable',
      'manual-device-required',
      'device-proof-required',
      'manual-device-proof'
    ),
    surfaceProof(
      'external-child-agent-transport',
      'typed-protocol-bridge',
      'not-implemented',
      'not-applicable',
      'not-implemented',
      'not-implemented',
      'external-child-agent-transport'
    ),
  ];
}

function surfaceProof(
  surface: string,
  parentCapability: string,
  parentCapabilityStatus: string,
  declarationState: string,
  runtimeGrantState: string,
  proofState: string,
  runtimeOwner: string
) {
  const proofRequirement = `${surface} remains ${proofState} until device artifacts change it`;
  return {
    surface,
    parentCapability,
    parentCapabilityStatus,
    declarationState,
    runtimeGrantState,
    proofState,
    runtimeOwner,
    proofRequirement,
    claimBoundary: proofRequirement,
  };
}

function surfaceState(
  model: ChildAndroidPrivilegedCapabilityReadModel,
  surface: ChildAndroidPrivilegedCapabilityReadModel['privilegedSurfaceProofs'][number]['surface']
) {
  const entry = model.privilegedSurfaceProofs.find((proof) => proof.surface === surface);
  return {
    parentCapabilityStatus: entry?.parentCapabilityStatus,
    declarationState: entry?.declarationState,
    runtimeGrantState: entry?.runtimeGrantState,
    proofState: entry?.proofState,
  };
}

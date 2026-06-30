import { describe, expect, it } from 'vitest';
import {
  type ChildAndroidServiceProtocolReadModel,
  ChildAndroidServiceProtocolReadModelSchema,
} from '@ocentra-parent/schema-domain/child-android-service-protocol-proof';

describe('child android service protocol proof contracts', () => {
  acceptsPackageLocalServiceProof();
  rejectsMissingServiceSurfaces();
  rejectsExternalServiceTransport();
  rejectsExternalStatusExport();
  rejectsUsageStatsUpgrade();
  rejectsDeviceOwnerAvailability();
});

function acceptsPackageLocalServiceProof(): void {
  it('ChildAndroidServiceProtocolReadModelSchema: accepts package-local service and capability labels', () => {
    const parsed = ChildAndroidServiceProtocolReadModelSchema.parse(validReadModel());

    expect(parsed.schemaVersion).toBe('child-android-service-protocol-capability-proof');
    expect(parsed.foregroundService).toEqual({
      packageId: 'ca.ocentra.parent.agent',
      serviceClass: 'ca.ocentra.parent.agent/.OcentraParentAgentService',
      notificationChannelId: 'ocentra_parent_agent',
      notificationId: 4477,
      foregroundServiceType: 'dataSync',
      serviceStatus: 'declared-started-by-package',
      runtimeOwner: 'android-foreground-service',
      proofRequirement: 'foreground service declaration and Java start path compile into the debug package',
      claimBoundary: 'foreground service runtime still requires emulator or physical-device evidence',
    });
    expect(parsed.protocolBridgeProof.commands).toEqual([
      'child.android.service.status.get',
      'child.android.service.capability.labels.get',
      'child.android.service.status.export.get',
      'child.android.service.protocol.proof.get',
    ]);
    expect(surfaceState(parsed, 'usage-stats-capability-label')).toEqual({
      parentCapabilityStatus: 'manual-required',
      capabilityLabel: 'permission-required',
      proofState: 'permission-required',
    });
    expect(surfaceState(parsed, 'device-owner-capability-label')).toEqual({
      parentCapabilityStatus: 'manual-required',
      capabilityLabel: 'blocked',
      proofState: 'blocked',
    });
  });
}

function rejectsMissingServiceSurfaces(): void {
  it('ChildAndroidServiceProtocolReadModelSchema: rejects missing service surfaces', () => {
    const model = validReadModel();

    expect(
      ChildAndroidServiceProtocolReadModelSchema.safeParse({
        ...model,
        serviceSurfaces: model.serviceSurfaces.filter((entry) => entry.surface !== 'managed-profile-capability-label'),
      }).success
    ).toBe(false);
  });
}

function rejectsExternalServiceTransport(): void {
  it('ChildAndroidServiceProtocolReadModelSchema: rejects external service transport claims', () => {
    const model = validReadModel();

    expect(
      ChildAndroidServiceProtocolReadModelSchema.safeParse({
        ...model,
        protocolBridgeProof: {
          ...model.protocolBridgeProof,
          externalTransportState: 'package-local-scaffold',
        },
      }).success
    ).toBe(false);
  });
}

function rejectsExternalStatusExport(): void {
  it('ChildAndroidServiceProtocolReadModelSchema: rejects service status export as external transport', () => {
    const model = validReadModel();

    expect(
      ChildAndroidServiceProtocolReadModelSchema.safeParse({
        ...model,
        statusExportProof: {
          ...model.statusExportProof,
          exportState: 'not-external-transport',
        },
      }).success
    ).toBe(false);
  });
}

function rejectsUsageStatsUpgrade(): void {
  it('ChildAndroidServiceProtocolReadModelSchema: rejects UsageStats upgraded without permission evidence', () => {
    const model = validReadModel();

    expect(
      ChildAndroidServiceProtocolReadModelSchema.safeParse({
        ...model,
        serviceSurfaces: model.serviceSurfaces.map((entry) =>
          entry.surface === 'usage-stats-capability-label'
            ? { ...entry, parentCapabilityStatus: 'implemented', capabilityLabel: 'implemented' }
            : entry
        ),
      }).success
    ).toBe(false);
  });
}

function rejectsDeviceOwnerAvailability(): void {
  it('ChildAndroidServiceProtocolReadModelSchema: rejects device-owner presented as available', () => {
    const model = validReadModel();

    expect(
      ChildAndroidServiceProtocolReadModelSchema.safeParse({
        ...model,
        serviceSurfaces: model.serviceSurfaces.map((entry) =>
          entry.surface === 'device-owner-capability-label'
            ? { ...entry, capabilityLabel: 'manual-required', proofState: 'ci-mechanical-proof' }
            : entry
        ),
      }).success
    ).toBe(false);
  });
}

function surfaceState(
  model: ChildAndroidServiceProtocolReadModel,
  surface: ChildAndroidServiceProtocolReadModel['serviceSurfaces'][number]['surface']
) {
  const entry = model.serviceSurfaces.find((proof) => proof.surface === surface);
  return {
    parentCapabilityStatus: entry?.parentCapabilityStatus,
    capabilityLabel: entry?.capabilityLabel,
    proofState: entry?.proofState,
  };
}

function validReadModel(): ChildAndroidServiceProtocolReadModel {
  return ChildAndroidServiceProtocolReadModelSchema.parse({
    schemaVersion: 'child-android-service-protocol-capability-proof',
    foregroundService: {
      packageId: 'ca.ocentra.parent.agent',
      serviceClass: 'ca.ocentra.parent.agent/.OcentraParentAgentService',
      notificationChannelId: 'ocentra_parent_agent',
      notificationId: 4477,
      foregroundServiceType: 'dataSync',
      serviceStatus: 'declared-started-by-package',
      runtimeOwner: 'android-foreground-service',
      proofRequirement: 'foreground service declaration and Java start path compile into the debug package',
      claimBoundary: 'foreground service runtime still requires emulator or physical-device evidence',
    },
    protocolBridgeProof: {
      packageId: 'ca.ocentra.parent.agent',
      nativeBridgeClass: 'ca.ocentra.parent.agent.ChildAndroidServiceProtocolProof',
      storageBridgeClass: 'ca.ocentra.parent.agent.ChildAndroidStorageProtocolProof',
      bridgeState: 'package-local-scaffold',
      storageBridgeState: 'package-local-scaffold',
      externalTransportState: 'not-implemented',
      commands: [
        'child.android.service.status.get',
        'child.android.service.capability.labels.get',
        'child.android.service.status.export.get',
        'child.android.service.protocol.proof.get',
      ],
      events: [
        'child.android.service.status.reported',
        'child.android.service.capability.labels.reported',
        'child.android.service.status.export.reported',
        'child.android.service.protocol.proof.reported',
      ],
      runtimeOwner: 'android-native-wrapper',
      proofRequirement: 'service protocol bridge constants compile into the debug package',
      claimBoundary: 'service protocol bridge is package-local and not LAN/WebSocket child-agent transport',
    },
    statusExportProof: {
      exportState: 'package-local-bundle',
      fields: [
        'schemaVersion',
        'packageId',
        'nativeBridgeClass',
        'foregroundServiceStatus',
        'storageBridgeState',
        'capabilityLabels',
        'commands',
        'events',
      ],
      runtimeOwner: 'status-export-bundle',
      proofRequirement: 'service status is exported only through a package-local Android Bundle',
      claimBoundary: 'status export is not parent-owned export storage or remote transport',
    },
    serviceSurfaces: serviceSurfaces(),
    claimBoundaries: {
      foregroundService: 'foreground service declaration and start path compile; runtime still needs device proof',
      storageProtocolBridge: 'storage protocol bridge is referenced as package-local scaffold only',
      statusExport: 'service status export is a local Bundle surface only',
      usageStats: 'UsageStats requires a user/device permission grant and observation artifact',
      accessibility: 'AccessibilityService is not implemented or declared by this proof',
      vpnDns: 'VPN/DNS filtering adapter is not implemented or declared by this proof',
      deviceOwner: 'device-owner policy remains blocked until enrollment and policy proof exist',
      managedProfile: 'managed-profile behavior remains blocked until enrollment proof exists',
      externalTransport: 'no LAN/WebSocket Android child-agent transport is claimed',
      childAndroidServiceRuntime: 'no emulator or physical-device foreground runtime behavior is claimed',
    },
    updatedAt: '2026-05-31T00:00:00.000Z',
  });
}

function serviceSurfaces() {
  return [
    serviceSurface(
      'foreground-service-status',
      'foreground-mobile-service',
      'manual-required',
      'scaffold-only',
      'ci-mechanical-proof',
      'android-foreground-service'
    ),
    serviceSurface(
      'storage-protocol-bridge',
      'typed-protocol-bridge',
      'scaffold',
      'scaffold-only',
      'ci-mechanical-proof',
      'agent-protocol'
    ),
    serviceSurface(
      'status-export-surface',
      'typed-protocol-bridge',
      'scaffold',
      'scaffold-only',
      'package-local-scaffold',
      'status-export-bundle'
    ),
    serviceSurface(
      'usage-stats-capability-label',
      'usage-stats',
      'manual-required',
      'permission-required',
      'permission-required',
      'android-os-permission'
    ),
    serviceSurface(
      'accessibility-capability-label',
      'accessibility-service',
      'not-implemented',
      'unavailable',
      'not-implemented',
      'android-accessibility-service'
    ),
    serviceSurface(
      'vpn-dns-capability-label',
      'vpn-dns-filtering',
      'not-implemented',
      'unavailable',
      'not-implemented',
      'android-vpn-service'
    ),
    serviceSurface(
      'device-owner-capability-label',
      'device-owner-policy',
      'manual-required',
      'blocked',
      'blocked',
      'android-policy-provider'
    ),
    serviceSurface(
      'managed-profile-capability-label',
      'managed-profile',
      'manual-required',
      'blocked',
      'blocked',
      'android-policy-provider'
    ),
  ];
}

function serviceSurface(
  surface: string,
  parentCapability: string,
  parentCapabilityStatus: string,
  capabilityLabel: string,
  proofState: string,
  runtimeOwner: string
) {
  const proofRequirement = `${surface} remains ${capabilityLabel} until Android device proof changes it`;
  return {
    surface,
    parentCapability,
    parentCapabilityStatus,
    capabilityLabel,
    proofState,
    runtimeOwner,
    proofRequirement,
    claimBoundary: proofRequirement,
  };
}

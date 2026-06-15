import { describe, expect, it } from 'vitest';
import {
  type ChildIosEntitlementCapabilityReadModel,
  ChildIosEntitlementCapabilityReadModelSchema,
} from '../../src/child-ios-entitlement-capability-proof';

describe('child iOS entitlement capability proof contracts', () => {
  acceptsHonestSimulatorAndManualEntitlementStates();
  rejectsMissingCapabilityRows();
  rejectsFamilyControlsAsImplemented();
  rejectsNetworkExtensionOrScreenTimeAsDeclared();
  rejectsSigningOrTestFlightUpgrade();
  rejectsDeviceInstallUpgrade();
});

function acceptsHonestSimulatorAndManualEntitlementStates(): void {
  it('ChildIosEntitlementCapabilityReadModelSchema: accepts honest simulator and manual entitlement states', () => {
    const parsed = ChildIosEntitlementCapabilityReadModelSchema.parse(validReadModel());

    expect(parsed.schemaVersion).toBe('child-ios-entitlement-capability-proof');
    expect(parsed.bundleId).toBe('ca.ocentra.parent.agent');
    expect(parsed.protocolBridgeProof.commands).toEqual([
      'child.ios.entitlement.capability.snapshot.get',
      'child.ios.entitlement.package.proof.get',
      'child.ios.entitlement.manual-proof.get',
    ]);
    expect(surfaceState(parsed, 'family-controls-entitlement')).toEqual({
      parentCapabilityStatus: 'manual-required',
      declarationState: 'not-declared',
      proofState: 'entitlement-required',
    });
    expect(surfaceState(parsed, 'signing-entitlements')).toEqual({
      parentCapabilityStatus: 'manual-required',
      declarationState: 'not-applicable',
      proofState: 'signing-required',
    });
  });
}

function rejectsMissingCapabilityRows(): void {
  it('ChildIosEntitlementCapabilityReadModelSchema: rejects missing iOS capability rows', () => {
    const model = validReadModel();

    expect(
      ChildIosEntitlementCapabilityReadModelSchema.safeParse({
        ...model,
        surfaceProofs: model.surfaceProofs.filter((entry) => entry.surface !== 'device-activity-framework'),
      }).success
    ).toBe(false);
  });
}

function rejectsFamilyControlsAsImplemented(): void {
  it('ChildIosEntitlementCapabilityReadModelSchema: rejects Family Controls as implemented without entitlement proof', () => {
    const model = validReadModel();

    expect(
      ChildIosEntitlementCapabilityReadModelSchema.safeParse({
        ...model,
        surfaceProofs: model.surfaceProofs.map((entry) =>
          entry.surface === 'family-controls-entitlement'
            ? { ...entry, parentCapabilityStatus: 'implemented', proofState: 'ci-mechanical-proof' }
            : entry
        ),
      }).success
    ).toBe(false);
  });
}

function rejectsNetworkExtensionOrScreenTimeAsDeclared(): void {
  it('ChildIosEntitlementCapabilityReadModelSchema: rejects Network Extension or Screen Time declarations without artifacts', () => {
    const model = validReadModel();

    expect(
      ChildIosEntitlementCapabilityReadModelSchema.safeParse({
        ...model,
        surfaceProofs: model.surfaceProofs.map((entry) =>
          entry.surface === 'network-extension'
            ? { ...entry, declarationState: 'declared-in-project', proofState: 'ci-mechanical-proof' }
            : entry
        ),
      }).success
    ).toBe(false);

    expect(
      ChildIosEntitlementCapabilityReadModelSchema.safeParse({
        ...model,
        surfaceProofs: model.surfaceProofs.map((entry) =>
          entry.surface === 'screen-time-api'
            ? { ...entry, declarationState: 'declared-in-project', proofState: 'ci-mechanical-proof' }
            : entry
        ),
      }).success
    ).toBe(false);
  });
}

function rejectsSigningOrTestFlightUpgrade(): void {
  it('ChildIosEntitlementCapabilityReadModelSchema: rejects signing or TestFlight as proved without Apple artifacts', () => {
    const model = validReadModel();

    expect(
      ChildIosEntitlementCapabilityReadModelSchema.safeParse({
        ...model,
        surfaceProofs: model.surfaceProofs.map((entry) =>
          entry.surface === 'signing-entitlements'
            ? { ...entry, proofState: 'ci-mechanical-proof', runtimeOwner: 'ios-xcode-project' }
            : entry
        ),
      }).success
    ).toBe(false);

    expect(
      ChildIosEntitlementCapabilityReadModelSchema.safeParse({
        ...model,
        surfaceProofs: model.surfaceProofs.map((entry) =>
          entry.surface === 'testflight-distribution'
            ? { ...entry, parentCapabilityStatus: 'implemented', proofState: 'ci-mechanical-proof' }
            : entry
        ),
      }).success
    ).toBe(false);
  });
}

function rejectsDeviceInstallUpgrade(): void {
  it('ChildIosEntitlementCapabilityReadModelSchema: rejects device install lifecycle upgrade', () => {
    const model = validReadModel();

    expect(
      ChildIosEntitlementCapabilityReadModelSchema.safeParse({
        ...model,
        packageLifecycleProofs: model.packageLifecycleProofs.map((entry) =>
          entry.phase === 'device-install' ? { ...entry, proofState: 'ci-mechanical-proof' } : entry
        ),
      }).success
    ).toBe(false);
  });
}

function validReadModel(): ChildIosEntitlementCapabilityReadModel {
  return {
    schemaVersion: 'child-ios-entitlement-capability-proof',
    bundleId: 'ca.ocentra.parent.agent',
    statusSurfaceClass: 'AgentStatusViewController',
    protocolBridgeProof: {
      bundleId: 'ca.ocentra.parent.agent',
      statusSurfaceClass: 'AgentStatusViewController',
      bridgeState: 'simulator-scaffold',
      externalTransportState: 'not-implemented',
      commands: [
        'child.ios.entitlement.capability.snapshot.get',
        'child.ios.entitlement.package.proof.get',
        'child.ios.entitlement.manual-proof.get',
      ],
      events: [
        'child.ios.entitlement.capability.snapshot.reported',
        'child.ios.entitlement.package.proof.reported',
        'child.ios.entitlement.manual-proof.reported',
      ],
      runtimeOwner: 'ios-swift-scaffold',
      proofRequirement: 'iOS scaffold labels compile in the simulator target source',
      claimBoundary: 'status labels are not Apple entitlement or device proof',
    },
    surfaceProofs: surfaceProofs(),
    packageLifecycleProofs: packageLifecycleProofs(),
    claimBoundaries: {
      simulatorPackage: 'simulator target and bundle id are source proof only',
      familyControls: 'Family Controls requires Apple entitlement approval and device proof',
      deviceActivity: 'DeviceActivity requires entitlement and schedule artifact proof',
      screenTime: 'Screen Time APIs require Apple-approved entitlement and apply artifacts',
      networkExtension: 'Network Extension requires entitlement signing and device artifacts',
      notifications: 'notification authorization and delivery remain manual-required',
      backgroundExecution: 'background execution requires mode entitlement and device behavior proof',
      signingEntitlements: 'signing and entitlements require Apple credential artifacts',
      testflight: 'TestFlight distribution requires App Store Connect and device install evidence',
      deviceProof: 'physical iOS device behavior is not claimed by simulator scaffold',
      externalTransport: 'no iOS child-agent LAN or WebSocket transport is claimed',
    },
    updatedAt: '2026-05-31T00:00:00.000Z',
  };
}

function surfaceProofs(): ChildIosEntitlementCapabilityReadModel['surfaceProofs'] {
  return [...packageSurfaceProofs(), ...entitlementSurfaceProofs(), ...distributionSurfaceProofs()];
}

function packageSurfaceProofs(): ChildIosEntitlementCapabilityReadModel['surfaceProofs'] {
  return [
    surfaceProof(
      'simulator-app-target',
      'package-lifecycle',
      'manual-required',
      'declared-in-project',
      'ci-mechanical-proof',
      'ios-xcode-project'
    ),
    surfaceProof(
      'bundle-identifier',
      'package-lifecycle',
      'manual-required',
      'declared-in-project',
      'ci-mechanical-proof',
      'ios-xcode-project'
    ),
    surfaceProof(
      'status-surface',
      'typed-protocol-bridge',
      'scaffold',
      'scaffold-status-label',
      'simulator-scaffold',
      'ios-swift-scaffold'
    ),
  ];
}

function entitlementSurfaceProofs(): ChildIosEntitlementCapabilityReadModel['surfaceProofs'] {
  return [
    surfaceProof(
      'family-controls-entitlement',
      'family-controls-entitlement',
      'manual-required',
      'not-declared',
      'entitlement-required',
      'apple-entitlement'
    ),
    surfaceProof(
      'device-activity-framework',
      'device-activity',
      'manual-required',
      'not-declared',
      'entitlement-required',
      'apple-device-framework'
    ),
    surfaceProof(
      'screen-time-api',
      'screen-time-api',
      'manual-required',
      'not-declared',
      'entitlement-required',
      'apple-device-framework'
    ),
    surfaceProof(
      'network-extension',
      'network-extension',
      'manual-required',
      'not-declared',
      'entitlement-required',
      'apple-network-extension'
    ),
    surfaceProof(
      'notifications-permission',
      'notifications',
      'manual-required',
      'not-declared',
      'manual-required',
      'apple-notification-permission'
    ),
    surfaceProof(
      'background-execution',
      'background-execution',
      'manual-required',
      'not-declared',
      'manual-required',
      'apple-background-mode'
    ),
  ];
}

function distributionSurfaceProofs(): ChildIosEntitlementCapabilityReadModel['surfaceProofs'] {
  return [
    surfaceProof(
      'signing-entitlements',
      'signing-entitlements',
      'manual-required',
      'not-applicable',
      'signing-required',
      'apple-signing'
    ),
    surfaceProof(
      'testflight-distribution',
      'testflight-distribution',
      'manual-required',
      'not-applicable',
      'device-proof-required',
      'apple-testflight'
    ),
    surfaceProof(
      'physical-device-proof',
      'package-lifecycle',
      'manual-required',
      'not-applicable',
      'device-proof-required',
      'apple-device-proof'
    ),
    surfaceProof(
      'app-store-distribution',
      'store-distribution',
      'planned',
      'not-applicable',
      'planned',
      'app-store-connect'
    ),
  ];
}

function packageLifecycleProofs(): ChildIosEntitlementCapabilityReadModel['packageLifecycleProofs'] {
  return [
    lifecycleProof('xcode-project-target', 'ci-mechanical-proof', 'ios-xcode-project'),
    lifecycleProof('bundle-identifier', 'ci-mechanical-proof', 'ios-xcode-project'),
    lifecycleProof('simulator-build-script', 'ci-mechanical-proof', 'ios-simulator-build-script'),
    lifecycleProof('status-view', 'simulator-scaffold', 'ios-swift-scaffold'),
    lifecycleProof('info-plist', 'ci-mechanical-proof', 'ios-info-plist'),
    lifecycleProof('simulator-build', 'manual-required', 'ios-simulator-build-script'),
    lifecycleProof('device-install', 'device-proof-required', 'apple-device-proof'),
    lifecycleProof('testflight-install', 'device-proof-required', 'apple-testflight'),
    lifecycleProof('signing-profile', 'signing-required', 'apple-signing'),
    lifecycleProof('entitlement-review', 'entitlement-required', 'apple-entitlement'),
  ];
}

function surfaceProof(
  surface: ChildIosEntitlementCapabilityReadModel['surfaceProofs'][number]['surface'],
  parentCapability: ChildIosEntitlementCapabilityReadModel['surfaceProofs'][number]['parentCapability'],
  parentCapabilityStatus: ChildIosEntitlementCapabilityReadModel['surfaceProofs'][number]['parentCapabilityStatus'],
  declarationState: ChildIosEntitlementCapabilityReadModel['surfaceProofs'][number]['declarationState'],
  proofState: ChildIosEntitlementCapabilityReadModel['surfaceProofs'][number]['proofState'],
  runtimeOwner: ChildIosEntitlementCapabilityReadModel['surfaceProofs'][number]['runtimeOwner']
): ChildIosEntitlementCapabilityReadModel['surfaceProofs'][number] {
  const proofRequirement = `${surface} remains ${proofState} until Apple artifact proof changes it`;
  return {
    surface,
    parentCapability,
    parentCapabilityStatus,
    declarationState,
    proofState,
    runtimeOwner,
    proofRequirement,
    claimBoundary: proofRequirement,
  };
}

function lifecycleProof(
  phase: ChildIosEntitlementCapabilityReadModel['packageLifecycleProofs'][number]['phase'],
  proofState: ChildIosEntitlementCapabilityReadModel['packageLifecycleProofs'][number]['proofState'],
  runtimeOwner: ChildIosEntitlementCapabilityReadModel['packageLifecycleProofs'][number]['runtimeOwner']
): ChildIosEntitlementCapabilityReadModel['packageLifecycleProofs'][number] {
  return {
    phase,
    proofState,
    runtimeOwner,
    proofRequirement: `${phase} proof state is ${proofState}`,
    claimBoundary: `${phase} does not upgrade iOS child capability without entitlement signing or device evidence`,
  };
}

function surfaceState(
  model: ChildIosEntitlementCapabilityReadModel,
  surface: ChildIosEntitlementCapabilityReadModel['surfaceProofs'][number]['surface']
) {
  const entry = model.surfaceProofs.find((proof) => proof.surface === surface);
  return {
    parentCapabilityStatus: entry?.parentCapabilityStatus,
    declarationState: entry?.declarationState,
    proofState: entry?.proofState,
  };
}

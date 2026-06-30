import { describe, expect, it } from 'vitest';
import {
  type ChildIosEntitlementCapabilityReadModel,
  ChildIosEntitlementCapabilityReadModelProof,
  ChildIosEntitlementCapabilityReadModelSchema,
} from '@ocentra-parent/schema-domain/child-ios-entitlement-capability-proof';

describe('child iOS entitlement capability proof contracts', () => {
  acceptsHonestSimulatorAndManualEntitlementStates();
  rejectsMissingCapabilityRows();
  rejectsFamilyControlsAsImplemented();
  rejectsNetworkExtensionOrScreenTimeAsDeclared();
  rejectsProvisioningOrSupervisionUpgrade();
  rejectsSigningOrTestFlightUpgrade();
  rejectsDeviceInstallUpgrade();
  rejectsLaunchOrRecoveryUpgrade();
  rejectsCapabilityOnlyBoundaryRemoval();
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
    expect(surfaceState(parsed, 'provisioning-profile')).toEqual({
      parentCapabilityStatus: 'manual-required',
      declarationState: 'not-applicable',
      proofState: 'signing-required',
    });
    expect(surfaceState(parsed, 'supervision-state')).toEqual({
      parentCapabilityStatus: 'manual-required',
      declarationState: 'not-applicable',
      proofState: 'device-proof-required',
    });
    expect(lifecycleState(parsed, 'simulator-launch')).toEqual({
      proofState: 'manual-required',
      runtimeOwner: 'apple-simulator-host',
    });
    expect(lifecycleState(parsed, 'device-launch')).toEqual({
      proofState: 'device-proof-required',
      runtimeOwner: 'apple-device-proof',
    });
    expect(lifecycleState(parsed, 'recovery-behavior')).toEqual({
      proofState: 'not-implemented',
      runtimeOwner: 'apple-background-mode',
    });
    expect(parsed.claimBoundaries.launchAvailability).toContain('launch availability remain manual-required');
    expect(parsed.claimBoundaries.recoveryBehavior).toContain('launch recovery remains not-implemented');
    expect(parsed.claimBoundaries.provisioningProfile).toContain('provisioning remains manual-required');
    expect(parsed.claimBoundaries.supervision).toContain('supervision remains manual-required');
    expect(parsed.claimBoundaries.capabilityOnlyState).toContain('capability-only');
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

function rejectsProvisioningOrSupervisionUpgrade(): void {
  it('ChildIosEntitlementCapabilityReadModelSchema: rejects provisioning or supervision as proved without Apple artifacts', () => {
    const model = validReadModel();

    expect(
      ChildIosEntitlementCapabilityReadModelSchema.safeParse({
        ...model,
        surfaceProofs: model.surfaceProofs.map((entry) =>
          entry.surface === 'provisioning-profile'
            ? { ...entry, parentCapabilityStatus: 'implemented', proofState: 'ci-mechanical-proof' }
            : entry
        ),
      }).success
    ).toBe(false);

    expect(
      ChildIosEntitlementCapabilityReadModelSchema.safeParse({
        ...model,
        surfaceProofs: model.surfaceProofs.map((entry) =>
          entry.surface === 'supervision-state'
            ? { ...entry, parentCapabilityStatus: 'implemented', proofState: 'ci-mechanical-proof' }
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

function rejectsCapabilityOnlyBoundaryRemoval(): void {
  it('ChildIosEntitlementCapabilityReadModelSchema: rejects removing the capability-only and no-daemon boundary', () => {
    const model = validReadModel();

    expect(
      ChildIosEntitlementCapabilityReadModelSchema.safeParse({
        ...model,
        protocolBridgeProof: {
          ...model.protocolBridgeProof,
          claimBoundary: 'status surface proves a hidden background daemon on iOS',
        },
      }).success
    ).toBe(false);

    expect(
      ChildIosEntitlementCapabilityReadModelSchema.safeParse({
        ...model,
        claimBoundaries: {
          ...model.claimBoundaries,
          capabilityOnlyState: 'iOS child runtime is a hidden persistent daemon',
        },
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

function rejectsLaunchOrRecoveryUpgrade(): void {
  it('ChildIosEntitlementCapabilityReadModelSchema: rejects launch availability or recovery upgrades', () => {
    const model = validReadModel();

    expect(
      ChildIosEntitlementCapabilityReadModelSchema.safeParse({
        ...model,
        packageLifecycleProofs: model.packageLifecycleProofs.map((entry) =>
          entry.phase === 'simulator-launch' ? { ...entry, proofState: 'ci-mechanical-proof' } : entry
        ),
      }).success
    ).toBe(false);

    expect(
      ChildIosEntitlementCapabilityReadModelSchema.safeParse({
        ...model,
        packageLifecycleProofs: model.packageLifecycleProofs.map((entry) =>
          entry.phase === 'recovery-behavior' ? { ...entry, proofState: 'manual-required' } : entry
        ),
      }).success
    ).toBe(false);
  });
}

function validReadModel(): ChildIosEntitlementCapabilityReadModel {
  return ChildIosEntitlementCapabilityReadModelSchema.parse(
    structuredClone(ChildIosEntitlementCapabilityReadModelProof)
  );
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

function lifecycleState(
  model: ChildIosEntitlementCapabilityReadModel,
  phase: ChildIosEntitlementCapabilityReadModel['packageLifecycleProofs'][number]['phase']
) {
  const entry = model.packageLifecycleProofs.find((proof) => proof.phase === phase);
  return {
    proofState: entry?.proofState,
    runtimeOwner: entry?.runtimeOwner,
  };
}

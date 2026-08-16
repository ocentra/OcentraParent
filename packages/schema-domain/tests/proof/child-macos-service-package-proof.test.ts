import { describe, expect, it } from 'vitest';
import {
  type ChildMacosServicePackageProofReadModel,
  ChildMacosServicePackageProofReadModelSchema,
} from '../../src/child-macos-service-package-proof';

describe('child macOS service package proof contracts', () => {
  acceptsHonestLaunchdPackageBoundary();
  rejectsMissingLaunchdRows();
  rejectsRestartUpgradeFromKeepAliveDeclaration();
  rejectsSignedOrNotarizedUpgradeWithoutArtifacts();
  rejectsEntitlementOrUninstallUpgradeWithoutArtifacts();
  rejectsParentParityOrHiddenServiceBoundaryClaims();
});

function acceptsHonestLaunchdPackageBoundary(): void {
  it('ChildMacosServicePackageProofReadModelSchema: accepts honest launchd package and manual-required states', () => {
    const parsed = ChildMacosServicePackageProofReadModelSchema.parse(validReadModel());

    expect(parsed.distributionMode).toBe('launchd-pkg-script');
    expect(parsed.artifactState).toBe('pkg-script-defined');
    expect(parsed.launchdBoundaryState).toBe('launchd-boundary-scripted');
    expect(parsed.restartState).toBe('keepalive-declared-manual-recovery-proof');
    expect(parsed.signingState).toBe('unsigned');
    expect(parsed.notarizationState).toBe('manual-required');
    expect(parsed.uninstallState).toBe('manual-uninstall-proof-required');
    expect(surfaceState(parsed, 'launchctl-bootstrap')).toEqual({
      parentCapabilityStatus: 'manual-required',
      proofState: 'ci-mechanical-proof',
      runtimeOwner: 'macos-launchctl-script',
    });
    expect(surfaceState(parsed, 'signing-review')).toEqual({
      parentCapabilityStatus: 'manual-required',
      proofState: 'unsigned',
      runtimeOwner: 'apple-codesign',
    });
    expect(parsed.claimBoundaries.restartBoundary).toContain('KeepAlive declaration is not runtime restart');
    expect(parsed.claimBoundaries.notarizationBoundary).toContain('manual-required');
    expect(parsed.claimBoundaries.parentParityBoundary).toContain('does not imply parent-client parity');
  });
}

function rejectsMissingLaunchdRows(): void {
  it('ChildMacosServicePackageProofReadModelSchema: rejects missing launchd proof rows', () => {
    const model = validReadModel();

    expect(
      ChildMacosServicePackageProofReadModelSchema.safeParse({
        ...model,
        surfaceProofs: model.surfaceProofs.filter((entry) => entry.surface !== 'launchd-plist'),
      }).success
    ).toBe(false);
  });
}

function rejectsRestartUpgradeFromKeepAliveDeclaration(): void {
  it('ChildMacosServicePackageProofReadModelSchema: rejects treating KeepAlive declaration as proved restart behavior', () => {
    const model = validReadModel();

    expect(
      ChildMacosServicePackageProofReadModelSchema.safeParse({
        ...model,
        restartState: 'restart-proved',
      }).success
    ).toBe(false);

    expect(
      ChildMacosServicePackageProofReadModelSchema.safeParse({
        ...model,
        claimBoundaries: {
          ...model.claimBoundaries,
          restartBoundary: 'KeepAlive proves persistent restart and recovery behavior',
        },
      }).success
    ).toBe(false);
  });
}

function rejectsSignedOrNotarizedUpgradeWithoutArtifacts(): void {
  it('ChildMacosServicePackageProofReadModelSchema: rejects signed or notarized claims without Apple artifacts', () => {
    const model = validReadModel();

    expect(
      ChildMacosServicePackageProofReadModelSchema.safeParse({
        ...model,
        signingState: 'signed',
      }).success
    ).toBe(false);

    expect(
      ChildMacosServicePackageProofReadModelSchema.safeParse({
        ...model,
        notarizationState: 'notarized',
      }).success
    ).toBe(false);
  });
}

function rejectsEntitlementOrUninstallUpgradeWithoutArtifacts(): void {
  it('ChildMacosServicePackageProofReadModelSchema: rejects entitlement, uninstall, or removal upgrades without audited artifacts', () => {
    const model = validReadModel();

    expect(
      ChildMacosServicePackageProofReadModelSchema.safeParse({
        ...model,
        entitlementState: 'signed-entitlements-proved',
      }).success
    ).toBe(false);

    expect(
      ChildMacosServicePackageProofReadModelSchema.safeParse({
        ...model,
        uninstallState: 'uninstall-proved',
      }).success
    ).toBe(false);

    expect(
      ChildMacosServicePackageProofReadModelSchema.safeParse({
        ...model,
        removalState: 'removal-proved',
      }).success
    ).toBe(false);
  });
}

function rejectsParentParityOrHiddenServiceBoundaryClaims(): void {
  it('ChildMacosServicePackageProofReadModelSchema: rejects parent parity or hidden background-service claims', () => {
    const model = validReadModel();

    expect(
      ChildMacosServicePackageProofReadModelSchema.safeParse({
        ...model,
        protocolBridgeProof: {
          ...model.protocolBridgeProof,
          claimBoundary: 'launchd package proves hidden persistent background-service parity with the parent client',
        },
      }).success
    ).toBe(false);

    expect(
      ChildMacosServicePackageProofReadModelSchema.safeParse({
        ...model,
        claimBoundaries: {
          ...model.claimBoundaries,
          parentParityBoundary: 'child macOS launchd proof matches parent-client parity and hidden persistence',
        },
      }).success
    ).toBe(false);
  });
}

function validReadModel(): ChildMacosServicePackageProofReadModel {
  return ChildMacosServicePackageProofReadModelSchema.parse({
    schemaVersion: 'child-macos-service-package-proof',
    serviceLabel: 'ca.ocentra.parent.agent',
    plistPath: '/Library/LaunchDaemons/ca.ocentra.parent.agent.plist',
    binaryPath: '/Library/Ocentra/Ocentra Parent Agent/bin/ocentra-parent-agent-service',
    distributionMode: 'launchd-pkg-script',
    artifactState: 'pkg-script-defined',
    launchdBoundaryState: 'launchd-boundary-scripted',
    installState: 'manual-install-proof-required',
    runtimeState: 'manual-service-proof-required',
    restartState: 'keepalive-declared-manual-recovery-proof',
    signingState: 'unsigned',
    notarizationState: 'manual-required',
    entitlementState: 'manual-required',
    uninstallState: 'manual-uninstall-proof-required',
    removalState: 'manual-removal-proof-required',
    protocolBridgeProof: {
      serviceLabel: 'ca.ocentra.parent.agent',
      plistPath: '/Library/LaunchDaemons/ca.ocentra.parent.agent.plist',
      binaryPath: '/Library/Ocentra/Ocentra Parent Agent/bin/ocentra-parent-agent-service',
      commands: [
        'child.macos.service.package.proof.get',
        'child.macos.service.lifecycle.proof.get',
        'child.macos.service.manual-proof.get',
      ],
      events: [
        'child.macos.service.package.proof.reported',
        'child.macos.service.lifecycle.proof.reported',
        'child.macos.service.manual-proof.reported',
      ],
      runtimeOwner: 'macos-launchctl-script',
      proofRequirement:
        'macOS child package proof names the launchd service boundary, install script bootstrap path, and manual-required runtime gaps',
      claimBoundary:
        'launchd plist and install scripts prove only the macOS service-manager boundary; they do not prove installed runtime health, restart recovery, notarization, or parent-client parity',
    },
    surfaceProofs: surfaceProofs(),
    lifecycleProofs: lifecycleProofs(),
    claimBoundaries: {
      packageArtifact:
        'pkgbuild script and staged payload prove only the child macOS artifact layout and install script boundary',
      launchdBoundary: 'launchd plist plus bootstrap/enable commands prove only the macOS service-manager boundary',
      runtimeBoundary:
        'launchd source proof does not prove installed service health, launch success, or crash-free runtime behavior',
      restartBoundary: 'KeepAlive declaration is not runtime restart or recovery proof without macOS service artifacts',
      signingBoundary:
        'the child macOS package is unsigned in this proof surface because no codesign or productsign artifact is attached',
      notarizationBoundary:
        'notarization remains manual-required because no notarytool or stapled ticket artifact is attached',
      entitlementBoundary:
        'entitlement or hardened-runtime claims remain manual-required without signed entitlement artifacts',
      uninstallBoundary:
        'disable and uninstall remain manual-required because no uninstall script or launchctl disable artifact is attached',
      removalBoundary:
        'removal and cleanup remain manual-required because no package removal, plist cleanup, or post-remove heartbeat artifact is attached',
      parentParityBoundary:
        'child macOS launchd proof does not imply parent-client parity or hidden background-service authority',
    },
    updatedAt: '2026-06-28T00:00:00.000Z',
  });
}

function surfaceProofs() {
  return [
    surfaceProof(
      'pkgbuild-script',
      'package-lifecycle',
      'manual-required',
      'ci-mechanical-proof',
      'macos-pkgbuild-script'
    ),
    surfaceProof(
      'service-binary-path',
      'headless-agent-service',
      'manual-required',
      'ci-mechanical-proof',
      'macos-release-binary'
    ),
    surfaceProof(
      'launchd-plist',
      'headless-agent-service',
      'manual-required',
      'ci-mechanical-proof',
      'macos-launchd-plist'
    ),
    surfaceProof(
      'launchctl-bootstrap',
      'headless-agent-service',
      'manual-required',
      'ci-mechanical-proof',
      'macos-launchctl-script'
    ),
    surfaceProof(
      'launchctl-enable',
      'headless-agent-service',
      'manual-required',
      'ci-mechanical-proof',
      'macos-launchctl-script'
    ),
    surfaceProof(
      'run-at-load',
      'headless-agent-service',
      'manual-required',
      'ci-mechanical-proof',
      'macos-launchd-plist'
    ),
    surfaceProof(
      'keepalive-declaration',
      'headless-agent-service',
      'manual-required',
      'ci-mechanical-proof',
      'macos-launchd-plist'
    ),
    surfaceProof('signing-review', 'signing-entitlements', 'manual-required', 'unsigned', 'apple-codesign'),
    surfaceProof('notarization-review', 'store-distribution', 'manual-required', 'manual-required', 'apple-notarytool'),
    surfaceProof('entitlement-review', 'signing-entitlements', 'manual-required', 'manual-required', 'apple-codesign'),
    surfaceProof(
      'uninstall-disable-review',
      'package-lifecycle',
      'manual-required',
      'manual-required',
      'macos-manual-proof'
    ),
    surfaceProof('removal-review', 'package-lifecycle', 'manual-required', 'manual-required', 'macos-manual-proof'),
  ];
}

function lifecycleProofs() {
  return [
    lifecycleProof('release-script', 'ci-mechanical-proof', 'macos-pkgbuild-script'),
    lifecycleProof('binary-stage', 'ci-mechanical-proof', 'macos-release-binary'),
    lifecycleProof('launchd-plist', 'ci-mechanical-proof', 'macos-launchd-plist'),
    lifecycleProof('package-build', 'ci-mechanical-proof', 'macos-pkgbuild-script'),
    lifecycleProof('install-bootstrap', 'ci-mechanical-proof', 'macos-launchctl-script'),
    lifecycleProof('install-enable', 'ci-mechanical-proof', 'macos-launchctl-script'),
    lifecycleProof('service-start', 'manual-required', 'macos-manual-proof'),
    lifecycleProof('restart-recovery', 'manual-required', 'macos-manual-proof'),
    lifecycleProof('signing-review', 'unsigned', 'apple-codesign'),
    lifecycleProof('notarization-review', 'manual-required', 'apple-notarytool'),
    lifecycleProof('uninstall-disable', 'manual-required', 'macos-manual-proof'),
    lifecycleProof('removal-cleanup', 'manual-required', 'macos-manual-proof'),
  ];
}

function surfaceProof(
  surface: string,
  parentCapability: string,
  parentCapabilityStatus: string,
  proofState: string,
  runtimeOwner: string
) {
  const proofRequirement = `${surface} remains ${proofState} until real macOS artifacts change it`;
  return {
    surface,
    parentCapability,
    parentCapabilityStatus,
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
    claimBoundary: `${phase} does not upgrade macOS service claims without platform artifacts`,
  };
}

function surfaceState(
  model: ChildMacosServicePackageProofReadModel,
  surface: ChildMacosServicePackageProofReadModel['surfaceProofs'][number]['surface']
) {
  const entry = model.surfaceProofs.find((proof) => proof.surface === surface);
  return {
    parentCapabilityStatus: entry?.parentCapabilityStatus,
    proofState: entry?.proofState,
    runtimeOwner: entry?.runtimeOwner,
  };
}

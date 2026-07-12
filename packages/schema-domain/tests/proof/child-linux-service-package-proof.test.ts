import { describe, expect, it } from 'vitest';
import {
  type ChildLinuxServicePackageProofReadModel,
  ChildLinuxServicePackageProofReadModelSchema,
} from '../../src/child-linux-service-package-proof';

describe('child Linux service package proof contracts', () => {
  acceptsHonestDebianSystemdBoundary();
  rejectsMissingSystemdRows();
  rejectsRestartOrRuntimeUpgradeWithoutLinuxHostArtifacts();
  rejectsSignedRepoOrGenericLinuxUpgradeWithoutArtifacts();
  rejectsChecksumUninstallOrCleanupUpgradeWithoutArtifacts();
  rejectsParentParityOrNonSystemdBoundaryClaims();
});

function acceptsHonestDebianSystemdBoundary(): void {
  it('ChildLinuxServicePackageProofReadModelSchema: accepts honest Debian package, checksum, and systemd manual-host states', () => {
    const parsed = ChildLinuxServicePackageProofReadModelSchema.parse(validReadModel());

    expect(parsed.distributionMode).toBe('direct-deb-package');
    expect(parsed.artifactState).toBe('deb-script-defined');
    expect(parsed.serviceManagerBoundaryState).toBe('systemd-boundary-scripted');
    expect(parsed.restartState).toBe('restart-policy-scripted-manual-host-proof');
    expect(parsed.checksumState).toBe('sha256-sidecar-scripted');
    expect(parsed.packageSigningState).toBe('unsigned');
    expect(parsed.repositoryState).toBe('direct-deb-only');
    expect(parsed.distroSupportState).toBe('ubuntu-22.04-amd64-glibc-2.35');
    expect(surfaceState(parsed, 'systemctl-stop')).toEqual({
      parentCapabilityStatus: 'manual-required',
      proofState: 'ci-mechanical-proof',
      runtimeOwner: 'linux-dpkg-maintainer-scripts',
    });
    expect(surfaceState(parsed, 'signing-review')).toEqual({
      parentCapabilityStatus: 'manual-required',
      proofState: 'unsigned',
      runtimeOwner: 'linux-package-signing',
    });
    expect(parsed.claimBoundaries.distributionBoundary).toContain('direct .deb artifact');
    expect(parsed.claimBoundaries.distroBoundary).toContain('does not imply generic distro-wide');
    expect(parsed.claimBoundaries.cleanupBoundary).toContain('daemon cleanup expectations explicit');
  });
}

function rejectsMissingSystemdRows(): void {
  it('ChildLinuxServicePackageProofReadModelSchema: rejects missing systemd or uninstall proof rows', () => {
    const model = validReadModel();

    expect(
      ChildLinuxServicePackageProofReadModelSchema.safeParse({
        ...model,
        surfaceProofs: model.surfaceProofs.filter((entry) => entry.surface !== 'systemd-unit'),
      }).success
    ).toBe(false);

    expect(
      ChildLinuxServicePackageProofReadModelSchema.safeParse({
        ...model,
        lifecycleProofs: model.lifecycleProofs.filter((entry) => entry.phase !== 'daemon-reload'),
      }).success
    ).toBe(false);
  });
}

function rejectsRestartOrRuntimeUpgradeWithoutLinuxHostArtifacts(): void {
  it('ChildLinuxServicePackageProofReadModelSchema: rejects runtime or restart upgrades without Linux host artifacts', () => {
    const model = validReadModel();

    expect(
      ChildLinuxServicePackageProofReadModelSchema.safeParse({
        ...model,
        runtimeState: 'running',
      }).success
    ).toBe(false);

    expect(
      ChildLinuxServicePackageProofReadModelSchema.safeParse({
        ...model,
        restartState: 'restart-proved',
      }).success
    ).toBe(false);

    expect(
      ChildLinuxServicePackageProofReadModelSchema.safeParse({
        ...model,
        claimBoundaries: {
          ...model.claimBoundaries,
          restartBoundary: 'Restart=always proves crash recovery and restart survival on every supported Linux host',
        },
      }).success
    ).toBe(false);
  });
}

function rejectsSignedRepoOrGenericLinuxUpgradeWithoutArtifacts(): void {
  it('ChildLinuxServicePackageProofReadModelSchema: rejects signed repository or generic Linux upgrades without artifacts', () => {
    const model = validReadModel();

    expect(
      ChildLinuxServicePackageProofReadModelSchema.safeParse({
        ...model,
        distributionMode: 'signed-repo-distribution',
      }).success
    ).toBe(false);

    expect(
      ChildLinuxServicePackageProofReadModelSchema.safeParse({
        ...model,
        packageSigningState: 'signed',
      }).success
    ).toBe(false);

    expect(
      ChildLinuxServicePackageProofReadModelSchema.safeParse({
        ...model,
        repositoryState: 'apt-repository-proved',
      }).success
    ).toBe(false);

    expect(
      ChildLinuxServicePackageProofReadModelSchema.safeParse({
        ...model,
        claimBoundaries: {
          ...model.claimBoundaries,
          distroBoundary: 'Linux package proof covers generic distro support and non-systemd hosts by default',
        },
      }).success
    ).toBe(false);
  });
}

function rejectsChecksumUninstallOrCleanupUpgradeWithoutArtifacts(): void {
  it('ChildLinuxServicePackageProofReadModelSchema: rejects checksum, uninstall, or cleanup upgrades without host artifacts', () => {
    const model = validReadModel();

    expect(
      ChildLinuxServicePackageProofReadModelSchema.safeParse({
        ...model,
        checksumState: 'verified',
      }).success
    ).toBe(false);

    expect(
      ChildLinuxServicePackageProofReadModelSchema.safeParse({
        ...model,
        uninstallState: 'uninstall-proved',
      }).success
    ).toBe(false);

    expect(
      ChildLinuxServicePackageProofReadModelSchema.safeParse({
        ...model,
        cleanupState: 'cleanup-proved',
      }).success
    ).toBe(false);
  });
}

function rejectsParentParityOrNonSystemdBoundaryClaims(): void {
  it('ChildLinuxServicePackageProofReadModelSchema: rejects parent parity or non-systemd boundary claims', () => {
    const model = validReadModel();

    expect(
      ChildLinuxServicePackageProofReadModelSchema.safeParse({
        ...model,
        serviceManagerProof: {
          ...model.serviceManagerProof,
          claimBoundary: 'systemd package proof proves runtime health, parent parity, and non-systemd Linux support',
        },
      }).success
    ).toBe(false);

    expect(
      ChildLinuxServicePackageProofReadModelSchema.safeParse({
        ...model,
        claimBoundaries: {
          ...model.claimBoundaries,
          parentParityBoundary:
            'child Linux package proof implies parent-client parity and generic cross-platform readiness',
        },
      }).success
    ).toBe(false);
  });
}

function validReadModel(): ChildLinuxServicePackageProofReadModel {
  return ChildLinuxServicePackageProofReadModelSchema.parse({
    schemaVersion: 'child-linux-service-package-proof',
    packageName: 'ocentra-parent-agent',
    serviceName: 'ocentra-parent-agent.service',
    unitPath: '/lib/systemd/system/ocentra-parent-agent.service',
    binaryPath: '/opt/ocentra/ocentra-parent-agent/bin/ocentra-parent-agent-service',
    distributionMode: 'direct-deb-package',
    artifactState: 'deb-script-defined',
    serviceManagerBoundaryState: 'systemd-boundary-scripted',
    installState: 'dpkg-install-scripted-manual-host-proof',
    runtimeState: 'systemd-start-scripted-manual-host-proof',
    restartState: 'restart-policy-scripted-manual-host-proof',
    checksumState: 'sha256-sidecar-scripted',
    packageSigningState: 'unsigned',
    repositoryState: 'direct-deb-only',
    distroSupportState: 'ubuntu-22.04-amd64-glibc-2.35',
    uninstallState: 'dpkg-remove-scripted-manual-host-proof',
    cleanupState: 'daemon-reload-scripted-manual-host-proof',
    serviceManagerProof: {
      packageName: 'ocentra-parent-agent',
      serviceName: 'ocentra-parent-agent.service',
      unitPath: '/lib/systemd/system/ocentra-parent-agent.service',
      binaryPath: '/opt/ocentra/ocentra-parent-agent/bin/ocentra-parent-agent-service',
      commands: [
        'child.linux.service.package.proof.get',
        'child.linux.service.lifecycle.proof.get',
        'child.linux.service.manual-proof.get',
      ],
      events: [
        'child.linux.service.package.proof.reported',
        'child.linux.service.lifecycle.proof.reported',
        'child.linux.service.manual-proof.reported',
      ],
      runtimeOwner: 'linux-dpkg-maintainer-scripts',
      proofRequirement:
        'Linux child package proof names the direct Debian package path, systemd service boundary, checksum sidecars, and manual-host-only runtime gaps',
      claimBoundary:
        'systemd unit, maintainer scripts, and smoke script prove only the Linux Debian package and service-manager boundary; they do not prove signed distribution, non-systemd hosts, or live runtime health',
    },
    surfaceProofs: surfaceProofs(),
    lifecycleProofs: lifecycleProofs(),
    claimBoundaries: {
      packageArtifact:
        'Debian package script and staged payload prove only the child Linux amd64 artifact layout and maintainer-script boundary',
      distributionBoundary:
        'the child Linux distribution path is a direct .deb artifact with sha256 sidecars; no apt repository, package feed, or production release channel is attached',
      distroBoundary:
        'Linux package proof is limited to Ubuntu 22.04 amd64 with glibc 2.35 baseline metadata and does not imply generic distro-wide or non-systemd support',
      serviceManagerBoundary:
        'systemd unit and maintainer scripts prove only the Linux systemd service-manager boundary for the child agent package',
      runtimeBoundary:
        'source and smoke scripts do not prove installed runtime health, crash-free behavior, or non-systemd host behavior in this proof surface',
      restartBoundary:
        'Restart=always and post-install restart wiring do not prove crash recovery on a real Linux host without service-manager artifacts',
      checksumBoundary:
        'sha256 sidecars are scripted and smoke-verified, but checksum proof does not imply package signing or repository promotion',
      signingBoundary:
        'the child Linux package is unsigned in this proof surface because no debsig, dpkg-sig, GPG, or repository signature artifact is attached',
      uninstallBoundary:
        'prerm stop/disable hooks and smoke remove checks make uninstall expectations explicit, but host uninstall proof remains manual-required without Linux package-manager artifacts',
      cleanupBoundary:
        'postrm daemon-reload and smoke purge checks make daemon cleanup expectations explicit, but live host cleanup proof remains manual-required without Linux artifacts',
      parentParityBoundary:
        'child Linux package proof does not imply parent-client distribution, Windows or macOS readiness, or hidden cross-platform parity claims',
    },
    updatedAt: '2026-06-28T00:00:00.000Z',
  });
}

function surfaceProofs() {
  return [...surfaceProofsPackageAndService(), ...surfaceProofsServiceControl(), ...surfaceProofsClosure()];
}

function surfaceProofsPackageAndService() {
  return [
    surfaceProof(
      'deb-build-script',
      'package-lifecycle',
      'manual-required',
      'ci-mechanical-proof',
      'linux-deb-build-script'
    ),
    surfaceProof(
      'direct-deb-distribution',
      'package-lifecycle',
      'manual-required',
      'ci-mechanical-proof',
      'linux-deb-build-script'
    ),
    surfaceProof(
      'service-binary-path',
      'headless-agent-service',
      'manual-required',
      'ci-mechanical-proof',
      'linux-release-binary'
    ),
    surfaceProof(
      'systemd-unit',
      'headless-agent-service',
      'manual-required',
      'ci-mechanical-proof',
      'linux-systemd-unit'
    ),
    surfaceProof(
      'dpkg-install-path',
      'package-lifecycle',
      'manual-required',
      'ci-mechanical-proof',
      'linux-smoke-script'
    ),
  ];
}

function surfaceProofsServiceControl() {
  return [
    surfaceProof(
      'systemctl-enable',
      'headless-agent-service',
      'manual-required',
      'ci-mechanical-proof',
      'linux-dpkg-maintainer-scripts'
    ),
    surfaceProof(
      'systemctl-restart',
      'headless-agent-service',
      'manual-required',
      'ci-mechanical-proof',
      'linux-dpkg-maintainer-scripts'
    ),
    surfaceProof(
      'systemctl-stop',
      'headless-agent-service',
      'manual-required',
      'ci-mechanical-proof',
      'linux-dpkg-maintainer-scripts'
    ),
    surfaceProof(
      'systemctl-disable',
      'headless-agent-service',
      'manual-required',
      'ci-mechanical-proof',
      'linux-dpkg-maintainer-scripts'
    ),
    surfaceProof(
      'daemon-reload-hook',
      'headless-agent-service',
      'manual-required',
      'ci-mechanical-proof',
      'linux-dpkg-maintainer-scripts'
    ),
  ];
}

function surfaceProofsClosure() {
  return [
    surfaceProof(
      'checksum-sidecar',
      'package-lifecycle',
      'manual-required',
      'ci-mechanical-proof',
      'linux-sha256-sidecar'
    ),
    surfaceProof('signing-review', 'store-distribution', 'manual-required', 'unsigned', 'linux-package-signing'),
    surfaceProof(
      'distro-baseline-review',
      'package-lifecycle',
      'manual-required',
      'ci-mechanical-proof',
      'linux-deb-build-script'
    ),
    surfaceProof(
      'uninstall-cleanup-review',
      'package-lifecycle',
      'manual-required',
      'ci-mechanical-proof',
      'linux-smoke-script'
    ),
  ];
}

function lifecycleProofs() {
  return [
    lifecycleProof('release-script', 'ci-mechanical-proof', 'linux-deb-build-script'),
    lifecycleProof('binary-stage', 'ci-mechanical-proof', 'linux-release-binary'),
    lifecycleProof('systemd-unit', 'ci-mechanical-proof', 'linux-systemd-unit'),
    lifecycleProof('package-build', 'ci-mechanical-proof', 'linux-deb-build-script'),
    lifecycleProof('checksum-write', 'ci-mechanical-proof', 'linux-sha256-sidecar'),
    lifecycleProof('install-path', 'ci-mechanical-proof', 'linux-smoke-script'),
    lifecycleProof('service-enable', 'ci-mechanical-proof', 'linux-dpkg-maintainer-scripts'),
    lifecycleProof('service-restart', 'ci-mechanical-proof', 'linux-dpkg-maintainer-scripts'),
    lifecycleProof('service-stop', 'ci-mechanical-proof', 'linux-dpkg-maintainer-scripts'),
    lifecycleProof('service-disable', 'ci-mechanical-proof', 'linux-dpkg-maintainer-scripts'),
    lifecycleProof('daemon-reload', 'ci-mechanical-proof', 'linux-dpkg-maintainer-scripts'),
    lifecycleProof('signing-review', 'unsigned', 'linux-package-signing'),
    lifecycleProof('repository-review', 'manual-required', 'linux-manual-proof'),
    lifecycleProof('cleanup-review', 'ci-mechanical-proof', 'linux-smoke-script'),
  ];
}

function surfaceProof(
  surface: string,
  parentCapability: string,
  parentCapabilityStatus: string,
  proofState: string,
  runtimeOwner: string
) {
  const proofRequirement = `${surface} remains ${proofState} until real Linux package artifacts change it`;
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
    claimBoundary: `${phase} does not upgrade Linux package claims without platform artifacts`,
  };
}

function surfaceState(
  model: ChildLinuxServicePackageProofReadModel,
  surface: ChildLinuxServicePackageProofReadModel['surfaceProofs'][number]['surface']
) {
  const entry = model.surfaceProofs.find((proof) => proof.surface === surface);
  return {
    parentCapabilityStatus: entry?.parentCapabilityStatus,
    proofState: entry?.proofState,
    runtimeOwner: entry?.runtimeOwner,
  };
}

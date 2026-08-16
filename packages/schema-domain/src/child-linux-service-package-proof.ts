/* generated from crates/schema/src/child_linux_service_package_proof_ts.rs */

import { type Infer, brandedNonEmptyStringSchema, Schema, withParser } from './effect';
import { ParentControlCapabilityNameSchema, ParentControlCapabilityStatusSchema } from './capabilities';
import { ParentTimestampSchema } from './family-reference-primitives';

export const ChildLinuxServicePackageProofSchemaVersionSchema = withParser(
  Schema.Literal('child-linux-service-package-proof')
);
export const ChildLinuxServiceSurfaceNameSchema = withParser(
  Schema.Literal(
    'deb-build-script',
    'direct-deb-distribution',
    'service-binary-path',
    'systemd-unit',
    'dpkg-install-path',
    'systemctl-enable',
    'systemctl-restart',
    'systemctl-stop',
    'systemctl-disable',
    'daemon-reload-hook',
    'checksum-sidecar',
    'signing-review',
    'distro-baseline-review',
    'uninstall-cleanup-review'
  )
);
export const ChildLinuxServiceProofStateSchema = withParser(
  Schema.Literal('ci-mechanical-proof', 'manual-required', 'unsigned', 'planned')
);
export const ChildLinuxServiceRuntimeOwnerSchema = withParser(
  Schema.Literal(
    'linux-deb-build-script',
    'linux-release-binary',
    'linux-systemd-unit',
    'linux-dpkg-maintainer-scripts',
    'linux-sha256-sidecar',
    'linux-smoke-script',
    'linux-package-signing',
    'linux-manual-proof'
  )
);
export const ChildLinuxServiceArtifactStateSchema = withParser(
  Schema.Literal('deb-script-defined', 'deb-built', 'manual-required')
);
export const ChildLinuxServiceDistributionModeSchema = withParser(
  Schema.Literal('direct-deb-package', 'signed-repo-distribution', 'manual-required')
);
export const ChildLinuxServiceManagerBoundaryStateSchema = withParser(
  Schema.Literal('systemd-boundary-scripted', 'systemd-installed', 'manual-required')
);
export const ChildLinuxServiceInstallStateSchema = withParser(
  Schema.Literal('dpkg-install-scripted-manual-host-proof', 'installed', 'failed')
);
export const ChildLinuxServiceRuntimeStateSchema = withParser(
  Schema.Literal('systemd-start-scripted-manual-host-proof', 'running', 'failed')
);
export const ChildLinuxServiceRestartStateSchema = withParser(
  Schema.Literal('restart-policy-scripted-manual-host-proof', 'restart-proved', 'failed')
);
export const ChildLinuxServiceChecksumStateSchema = withParser(
  Schema.Literal('sha256-sidecar-scripted', 'verified', 'manual-required')
);
export const ChildLinuxServicePackageSigningStateSchema = withParser(
  Schema.Literal('unsigned', 'signed', 'manual-required')
);
export const ChildLinuxServiceRepositoryStateSchema = withParser(
  Schema.Literal('direct-deb-only', 'apt-repository-proved', 'manual-required')
);
export const ChildLinuxServiceDistroSupportStateSchema = withParser(
  Schema.Literal('ubuntu-22.04-amd64-glibc-2.35', 'manual-required')
);
export const ChildLinuxServiceUninstallStateSchema = withParser(
  Schema.Literal('dpkg-remove-scripted-manual-host-proof', 'uninstall-proved', 'failed')
);
export const ChildLinuxServiceCleanupStateSchema = withParser(
  Schema.Literal('daemon-reload-scripted-manual-host-proof', 'cleanup-proved', 'failed')
);
export const ChildLinuxServiceLifecyclePhaseSchema = withParser(
  Schema.Literal(
    'release-script',
    'binary-stage',
    'systemd-unit',
    'package-build',
    'checksum-write',
    'install-path',
    'service-enable',
    'service-restart',
    'service-stop',
    'service-disable',
    'daemon-reload',
    'signing-review',
    'repository-review',
    'cleanup-review'
  )
);
export const ChildLinuxServiceProtocolCommandSchema = withParser(
  Schema.Literal(
    'child.linux.service.package.proof.get',
    'child.linux.service.lifecycle.proof.get',
    'child.linux.service.manual-proof.get'
  )
);
export const ChildLinuxServiceProtocolEventSchema = withParser(
  Schema.Literal(
    'child.linux.service.package.proof.reported',
    'child.linux.service.lifecycle.proof.reported',
    'child.linux.service.manual-proof.reported'
  )
);

const ChildLinuxServiceLabelSchema = brandedNonEmptyStringSchema('ChildLinuxServiceLabel');
const ChildLinuxServicePathSchema = brandedNonEmptyStringSchema('ChildLinuxServicePath');
const ChildLinuxServiceRequirementSchema = brandedNonEmptyStringSchema('ChildLinuxServiceRequirement');
const ChildLinuxServiceBoundarySchema = brandedNonEmptyStringSchema('ChildLinuxServiceBoundary');

export const ChildLinuxServiceSurfaceProofSchema = withParser(
  Schema.Struct({
    surface: ChildLinuxServiceSurfaceNameSchema,
    parentCapability: ParentControlCapabilityNameSchema,
    parentCapabilityStatus: ParentControlCapabilityStatusSchema,
    proofState: ChildLinuxServiceProofStateSchema,
    runtimeOwner: ChildLinuxServiceRuntimeOwnerSchema,
    proofRequirement: ChildLinuxServiceRequirementSchema,
    claimBoundary: ChildLinuxServiceBoundarySchema,
  })
);

export const ChildLinuxServiceLifecycleProofSchema = withParser(
  Schema.Struct({
    phase: ChildLinuxServiceLifecyclePhaseSchema,
    proofState: ChildLinuxServiceProofStateSchema,
    runtimeOwner: ChildLinuxServiceRuntimeOwnerSchema,
    proofRequirement: ChildLinuxServiceRequirementSchema,
    claimBoundary: ChildLinuxServiceBoundarySchema,
  })
);

export const ChildLinuxServiceManagerProofSchema = withParser(
  Schema.Struct({
    packageName: ChildLinuxServiceLabelSchema,
    serviceName: ChildLinuxServiceLabelSchema,
    unitPath: ChildLinuxServicePathSchema,
    binaryPath: ChildLinuxServicePathSchema,
    commands: Schema.Array(ChildLinuxServiceProtocolCommandSchema),
    events: Schema.Array(ChildLinuxServiceProtocolEventSchema),
    runtimeOwner: ChildLinuxServiceRuntimeOwnerSchema,
    proofRequirement: ChildLinuxServiceRequirementSchema,
    claimBoundary: ChildLinuxServiceBoundarySchema,
  })
);

export const ChildLinuxServiceClaimBoundariesSchema = withParser(
  Schema.Struct({
    packageArtifact: ChildLinuxServiceBoundarySchema,
    distributionBoundary: ChildLinuxServiceBoundarySchema,
    distroBoundary: ChildLinuxServiceBoundarySchema,
    serviceManagerBoundary: ChildLinuxServiceBoundarySchema,
    runtimeBoundary: ChildLinuxServiceBoundarySchema,
    restartBoundary: ChildLinuxServiceBoundarySchema,
    checksumBoundary: ChildLinuxServiceBoundarySchema,
    signingBoundary: ChildLinuxServiceBoundarySchema,
    uninstallBoundary: ChildLinuxServiceBoundarySchema,
    cleanupBoundary: ChildLinuxServiceBoundarySchema,
    parentParityBoundary: ChildLinuxServiceBoundarySchema,
  })
);

const ChildLinuxServiceReadModelBaseSchema = Schema.Struct({
  schemaVersion: ChildLinuxServicePackageProofSchemaVersionSchema,
  packageName: ChildLinuxServiceLabelSchema,
  serviceName: ChildLinuxServiceLabelSchema,
  unitPath: ChildLinuxServicePathSchema,
  binaryPath: ChildLinuxServicePathSchema,
  distributionMode: ChildLinuxServiceDistributionModeSchema,
  artifactState: ChildLinuxServiceArtifactStateSchema,
  serviceManagerBoundaryState: ChildLinuxServiceManagerBoundaryStateSchema,
  installState: ChildLinuxServiceInstallStateSchema,
  runtimeState: ChildLinuxServiceRuntimeStateSchema,
  restartState: ChildLinuxServiceRestartStateSchema,
  checksumState: ChildLinuxServiceChecksumStateSchema,
  packageSigningState: ChildLinuxServicePackageSigningStateSchema,
  repositoryState: ChildLinuxServiceRepositoryStateSchema,
  distroSupportState: ChildLinuxServiceDistroSupportStateSchema,
  uninstallState: ChildLinuxServiceUninstallStateSchema,
  cleanupState: ChildLinuxServiceCleanupStateSchema,
  serviceManagerProof: ChildLinuxServiceManagerProofSchema,
  surfaceProofs: Schema.Array(ChildLinuxServiceSurfaceProofSchema),
  lifecycleProofs: Schema.Array(ChildLinuxServiceLifecycleProofSchema),
  claimBoundaries: ChildLinuxServiceClaimBoundariesSchema,
  updatedAt: ParentTimestampSchema,
});

type ChildLinuxServiceReadModelCandidate = Infer<typeof ChildLinuxServiceReadModelBaseSchema>;

export const ChildLinuxServicePackageProofReadModelSchema = withParser(
  ChildLinuxServiceReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        childLinuxServicePackageProofReadModelIsHonest(readModel) ||
        'Expected child Linux service package proof to stay a direct unsigned Debian package with Ubuntu 22.04 amd64 / glibc 2.35 limits, keep install/runtime/restart/uninstall/cleanup as script-defined manual-host proof, expose checksum state, and avoid signed repo, non-systemd, or parent parity claims without Linux host artifacts'
    )
  )
);

const ExpectedServiceManagerProofRequirement =
  'Linux child package proof names the direct Debian package path, systemd service boundary, checksum sidecars, and manual-host-only runtime gaps';
const ExpectedServiceManagerClaimBoundary =
  'systemd unit, maintainer scripts, and smoke script prove only the Linux Debian package and service-manager boundary; they do not prove signed distribution, non-systemd hosts, or live runtime health';

const RequiredSurfaces = [
  'deb-build-script',
  'direct-deb-distribution',
  'service-binary-path',
  'systemd-unit',
  'dpkg-install-path',
  'systemctl-enable',
  'systemctl-restart',
  'systemctl-stop',
  'systemctl-disable',
  'daemon-reload-hook',
  'checksum-sidecar',
  'signing-review',
  'distro-baseline-review',
  'uninstall-cleanup-review',
] as const satisfies ReadonlyArray<ChildLinuxServiceSurfaceName>;

const RequiredLifecyclePhases = [
  'release-script',
  'binary-stage',
  'systemd-unit',
  'package-build',
  'checksum-write',
  'install-path',
  'service-enable',
  'service-restart',
  'service-stop',
  'service-disable',
  'daemon-reload',
  'signing-review',
  'repository-review',
  'cleanup-review',
] as const satisfies ReadonlyArray<ChildLinuxServiceLifecyclePhase>;

const RequiredCommands = [
  'child.linux.service.package.proof.get',
  'child.linux.service.lifecycle.proof.get',
  'child.linux.service.manual-proof.get',
] as const satisfies ReadonlyArray<ChildLinuxServiceProtocolCommand>;

const RequiredEvents = [
  'child.linux.service.package.proof.reported',
  'child.linux.service.lifecycle.proof.reported',
  'child.linux.service.manual-proof.reported',
] as const satisfies ReadonlyArray<ChildLinuxServiceProtocolEvent>;

const SurfaceExpectations = {
  'deb-build-script': {
    parentCapability: 'package-lifecycle',
    parentCapabilityStatus: 'manual-required',
    proofState: 'ci-mechanical-proof',
    runtimeOwner: 'linux-deb-build-script',
  },
  'direct-deb-distribution': {
    parentCapability: 'package-lifecycle',
    parentCapabilityStatus: 'manual-required',
    proofState: 'ci-mechanical-proof',
    runtimeOwner: 'linux-deb-build-script',
  },
  'service-binary-path': {
    parentCapability: 'headless-agent-service',
    parentCapabilityStatus: 'manual-required',
    proofState: 'ci-mechanical-proof',
    runtimeOwner: 'linux-release-binary',
  },
  'systemd-unit': {
    parentCapability: 'headless-agent-service',
    parentCapabilityStatus: 'manual-required',
    proofState: 'ci-mechanical-proof',
    runtimeOwner: 'linux-systemd-unit',
  },
  'dpkg-install-path': {
    parentCapability: 'package-lifecycle',
    parentCapabilityStatus: 'manual-required',
    proofState: 'ci-mechanical-proof',
    runtimeOwner: 'linux-smoke-script',
  },
  'systemctl-enable': {
    parentCapability: 'headless-agent-service',
    parentCapabilityStatus: 'manual-required',
    proofState: 'ci-mechanical-proof',
    runtimeOwner: 'linux-dpkg-maintainer-scripts',
  },
  'systemctl-restart': {
    parentCapability: 'headless-agent-service',
    parentCapabilityStatus: 'manual-required',
    proofState: 'ci-mechanical-proof',
    runtimeOwner: 'linux-dpkg-maintainer-scripts',
  },
  'systemctl-stop': {
    parentCapability: 'headless-agent-service',
    parentCapabilityStatus: 'manual-required',
    proofState: 'ci-mechanical-proof',
    runtimeOwner: 'linux-dpkg-maintainer-scripts',
  },
  'systemctl-disable': {
    parentCapability: 'headless-agent-service',
    parentCapabilityStatus: 'manual-required',
    proofState: 'ci-mechanical-proof',
    runtimeOwner: 'linux-dpkg-maintainer-scripts',
  },
  'daemon-reload-hook': {
    parentCapability: 'headless-agent-service',
    parentCapabilityStatus: 'manual-required',
    proofState: 'ci-mechanical-proof',
    runtimeOwner: 'linux-dpkg-maintainer-scripts',
  },
  'checksum-sidecar': {
    parentCapability: 'package-lifecycle',
    parentCapabilityStatus: 'manual-required',
    proofState: 'ci-mechanical-proof',
    runtimeOwner: 'linux-sha256-sidecar',
  },
  'signing-review': {
    parentCapability: 'store-distribution',
    parentCapabilityStatus: 'manual-required',
    proofState: 'unsigned',
    runtimeOwner: 'linux-package-signing',
  },
  'distro-baseline-review': {
    parentCapability: 'package-lifecycle',
    parentCapabilityStatus: 'manual-required',
    proofState: 'ci-mechanical-proof',
    runtimeOwner: 'linux-deb-build-script',
  },
  'uninstall-cleanup-review': {
    parentCapability: 'package-lifecycle',
    parentCapabilityStatus: 'manual-required',
    proofState: 'ci-mechanical-proof',
    runtimeOwner: 'linux-smoke-script',
  },
} as const satisfies Record<
  ChildLinuxServiceSurfaceName,
  Pick<ChildLinuxServiceSurfaceProof, 'parentCapability' | 'parentCapabilityStatus' | 'proofState' | 'runtimeOwner'>
>;

const ExpectedClaimBoundaries = {
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
} as const satisfies Record<keyof ChildLinuxServiceClaimBoundaries, string>;

function childLinuxServicePackageProofReadModelIsHonest(readModel: ChildLinuxServiceReadModelCandidate): boolean {
  return (
    childLinuxServicePackageIdentityIsHonest(readModel) &&
    childLinuxServicePackageLifecycleIsHonest(readModel) &&
    childLinuxServicePackageHostProofsAreHonest(readModel) &&
    serviceManagerProofIsHonest(readModel.serviceManagerProof) &&
    surfaceProofsAreHonest(readModel.surfaceProofs) &&
    lifecycleProofsAreHonest(readModel.lifecycleProofs) &&
    claimBoundariesAreHonest(readModel.claimBoundaries)
  );
}

function childLinuxServicePackageIdentityIsHonest(readModel: ChildLinuxServiceReadModelCandidate): boolean {
  return (
    readModel.packageName === 'ocentra-parent-agent' &&
    readModel.serviceName === 'ocentra-parent-agent.service' &&
    readModel.unitPath === '/lib/systemd/system/ocentra-parent-agent.service' &&
    readModel.binaryPath === '/opt/ocentra/ocentra-parent-agent/bin/ocentra-parent-agent-service'
  );
}

function childLinuxServicePackageLifecycleIsHonest(readModel: ChildLinuxServiceReadModelCandidate): boolean {
  return (
    readModel.distributionMode === 'direct-deb-package' &&
    readModel.artifactState === 'deb-script-defined' &&
    readModel.serviceManagerBoundaryState === 'systemd-boundary-scripted' &&
    readModel.installState === 'dpkg-install-scripted-manual-host-proof' &&
    readModel.runtimeState === 'systemd-start-scripted-manual-host-proof' &&
    readModel.restartState === 'restart-policy-scripted-manual-host-proof'
  );
}

function childLinuxServicePackageHostProofsAreHonest(readModel: ChildLinuxServiceReadModelCandidate): boolean {
  return (
    readModel.checksumState === 'sha256-sidecar-scripted' &&
    readModel.packageSigningState === 'unsigned' &&
    readModel.repositoryState === 'direct-deb-only' &&
    readModel.distroSupportState === 'ubuntu-22.04-amd64-glibc-2.35' &&
    readModel.uninstallState === 'dpkg-remove-scripted-manual-host-proof' &&
    readModel.cleanupState === 'daemon-reload-scripted-manual-host-proof'
  );
}

function serviceManagerProofIsHonest(proof: ChildLinuxServiceManagerProof): boolean {
  return (
    proof.packageName === 'ocentra-parent-agent' &&
    proof.serviceName === 'ocentra-parent-agent.service' &&
    proof.unitPath === '/lib/systemd/system/ocentra-parent-agent.service' &&
    proof.binaryPath === '/opt/ocentra/ocentra-parent-agent/bin/ocentra-parent-agent-service' &&
    proof.runtimeOwner === 'linux-dpkg-maintainer-scripts' &&
    proof.proofRequirement === ExpectedServiceManagerProofRequirement &&
    proof.claimBoundary === ExpectedServiceManagerClaimBoundary &&
    requiredValuesArePresent(proof.commands, RequiredCommands) &&
    requiredValuesArePresent(proof.events, RequiredEvents)
  );
}

function surfaceProofsAreHonest(proofs: ReadonlyArray<ChildLinuxServiceSurfaceProof>): boolean {
  const bySurface = new Map(proofs.map((entry) => [entry.surface, entry] as const));
  return (
    bySurface.size === proofs.length &&
    RequiredSurfaces.every((surface) => surfaceProofIsHonest(bySurface.get(surface), surface))
  );
}

function surfaceProofIsHonest(
  proof: ChildLinuxServiceSurfaceProof | undefined,
  surface: ChildLinuxServiceSurfaceName
): boolean {
  const expected = SurfaceExpectations[surface];
  return Boolean(
    proof &&
    proof.surface === surface &&
    proof.parentCapability === expected.parentCapability &&
    proof.parentCapabilityStatus === expected.parentCapabilityStatus &&
    proof.proofState === expected.proofState &&
    proof.runtimeOwner === expected.runtimeOwner
  );
}

function lifecycleProofsAreHonest(proofs: ReadonlyArray<ChildLinuxServiceLifecycleProof>): boolean {
  const byPhase = new Map(proofs.map((entry) => [entry.phase, entry] as const));
  return (
    byPhase.size === proofs.length &&
    RequiredLifecyclePhases.every((phase) => lifecyclePhaseIsHonest(byPhase.get(phase), phase))
  );
}

function lifecyclePhaseIsHonest(
  proof: ChildLinuxServiceLifecycleProof | undefined,
  phase: ChildLinuxServiceLifecyclePhase
): boolean {
  if (!proof || proof.phase !== phase) {
    return false;
  }

  if (phase === 'signing-review') {
    return proof.proofState === 'unsigned';
  }

  if (phase === 'repository-review') {
    return proof.proofState === 'manual-required';
  }

  return proof.proofState === 'ci-mechanical-proof';
}

function claimBoundariesAreHonest(boundaries: ChildLinuxServiceClaimBoundaries): boolean {
  return Object.entries(ExpectedClaimBoundaries).every(
    ([key, value]) => boundaries[key as keyof ChildLinuxServiceClaimBoundaries] === value
  );
}

function requiredValuesArePresent<Value extends string>(
  values: ReadonlyArray<Value>,
  required: ReadonlyArray<Value>
): boolean {
  const valueSet = new Set(values);
  return valueSet.size === values.length && required.every((value) => valueSet.has(value));
}

export type ChildLinuxServiceSurfaceName = Infer<typeof ChildLinuxServiceSurfaceNameSchema>;
export type ChildLinuxServiceProofState = Infer<typeof ChildLinuxServiceProofStateSchema>;
export type ChildLinuxServiceRuntimeOwner = Infer<typeof ChildLinuxServiceRuntimeOwnerSchema>;
export type ChildLinuxServiceArtifactState = Infer<typeof ChildLinuxServiceArtifactStateSchema>;
export type ChildLinuxServiceDistributionMode = Infer<typeof ChildLinuxServiceDistributionModeSchema>;
export type ChildLinuxServiceManagerBoundaryState = Infer<typeof ChildLinuxServiceManagerBoundaryStateSchema>;
export type ChildLinuxServiceInstallState = Infer<typeof ChildLinuxServiceInstallStateSchema>;
export type ChildLinuxServiceRuntimeState = Infer<typeof ChildLinuxServiceRuntimeStateSchema>;
export type ChildLinuxServiceRestartState = Infer<typeof ChildLinuxServiceRestartStateSchema>;
export type ChildLinuxServiceChecksumState = Infer<typeof ChildLinuxServiceChecksumStateSchema>;
export type ChildLinuxServicePackageSigningState = Infer<typeof ChildLinuxServicePackageSigningStateSchema>;
export type ChildLinuxServiceRepositoryState = Infer<typeof ChildLinuxServiceRepositoryStateSchema>;
export type ChildLinuxServiceDistroSupportState = Infer<typeof ChildLinuxServiceDistroSupportStateSchema>;
export type ChildLinuxServiceUninstallState = Infer<typeof ChildLinuxServiceUninstallStateSchema>;
export type ChildLinuxServiceCleanupState = Infer<typeof ChildLinuxServiceCleanupStateSchema>;
export type ChildLinuxServiceLifecyclePhase = Infer<typeof ChildLinuxServiceLifecyclePhaseSchema>;
export type ChildLinuxServiceProtocolCommand = Infer<typeof ChildLinuxServiceProtocolCommandSchema>;
export type ChildLinuxServiceProtocolEvent = Infer<typeof ChildLinuxServiceProtocolEventSchema>;
export type ChildLinuxServiceSurfaceProof = Infer<typeof ChildLinuxServiceSurfaceProofSchema>;
export type ChildLinuxServiceLifecycleProof = Infer<typeof ChildLinuxServiceLifecycleProofSchema>;
export type ChildLinuxServiceManagerProof = Infer<typeof ChildLinuxServiceManagerProofSchema>;
export type ChildLinuxServiceClaimBoundaries = Infer<typeof ChildLinuxServiceClaimBoundariesSchema>;
export type ChildLinuxServicePackageProofReadModel = Infer<typeof ChildLinuxServicePackageProofReadModelSchema>;

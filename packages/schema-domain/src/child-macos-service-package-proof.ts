/* generated from crates/schema/src/child_macos_service_package_proof_ts.rs */

import { type Infer, brandedNonEmptyStringSchema, Schema, withParser } from './effect';
import { ParentControlCapabilityNameSchema, ParentControlCapabilityStatusSchema } from './capabilities';
import { ParentTimestampSchema } from './family-reference-primitives';

export const ChildMacosServicePackageProofSchemaVersionSchema = withParser(
  Schema.Literal('child-macos-service-package-proof')
);
export const ChildMacosServiceSurfaceNameSchema = withParser(
  Schema.Literal(
    'pkgbuild-script',
    'service-binary-path',
    'launchd-plist',
    'launchctl-bootstrap',
    'launchctl-enable',
    'run-at-load',
    'keepalive-declaration',
    'signing-review',
    'notarization-review',
    'entitlement-review',
    'uninstall-disable-review',
    'removal-review'
  )
);
export const ChildMacosServiceProofStateSchema = withParser(
  Schema.Literal('ci-mechanical-proof', 'manual-required', 'unsigned', 'notarization-required', 'planned')
);
export const ChildMacosServiceRuntimeOwnerSchema = withParser(
  Schema.Literal(
    'macos-pkgbuild-script',
    'macos-release-binary',
    'macos-launchd-plist',
    'macos-launchctl-script',
    'apple-codesign',
    'apple-notarytool',
    'macos-manual-proof'
  )
);
export const ChildMacosServiceArtifactStateSchema = withParser(
  Schema.Literal('pkg-script-defined', 'pkg-built', 'manual-required')
);
export const ChildMacosServiceDistributionModeSchema = withParser(
  Schema.Literal('launchd-pkg-script', 'signed-pkg-distribution', 'manual-required')
);
export const ChildMacosServiceLaunchdBoundaryStateSchema = withParser(
  Schema.Literal('launchd-boundary-scripted', 'launchd-installed', 'manual-required')
);
export const ChildMacosServiceInstallStateSchema = withParser(
  Schema.Literal('manual-install-proof-required', 'installed', 'failed')
);
export const ChildMacosServiceRuntimeStateSchema = withParser(
  Schema.Literal('manual-service-proof-required', 'running', 'failed')
);
export const ChildMacosServiceRestartStateSchema = withParser(
  Schema.Literal('keepalive-declared-manual-recovery-proof', 'restart-proved', 'failed')
);
export const ChildMacosServiceSigningStateSchema = withParser(Schema.Literal('unsigned', 'signed', 'manual-required'));
export const ChildMacosServiceNotarizationStateSchema = withParser(
  Schema.Literal('manual-required', 'notarized', 'failed')
);
export const ChildMacosServiceEntitlementStateSchema = withParser(
  Schema.Literal('manual-required', 'signed-entitlements-proved', 'failed')
);
export const ChildMacosServiceUninstallStateSchema = withParser(
  Schema.Literal('manual-uninstall-proof-required', 'uninstall-proved', 'failed')
);
export const ChildMacosServiceRemovalStateSchema = withParser(
  Schema.Literal('manual-removal-proof-required', 'removal-proved', 'failed')
);
export const ChildMacosServiceLifecyclePhaseSchema = withParser(
  Schema.Literal(
    'release-script',
    'binary-stage',
    'launchd-plist',
    'package-build',
    'install-bootstrap',
    'install-enable',
    'service-start',
    'restart-recovery',
    'signing-review',
    'notarization-review',
    'uninstall-disable',
    'removal-cleanup'
  )
);
export const ChildMacosServiceProtocolCommandSchema = withParser(
  Schema.Literal(
    'child.macos.service.package.proof.get',
    'child.macos.service.lifecycle.proof.get',
    'child.macos.service.manual-proof.get'
  )
);
export const ChildMacosServiceProtocolEventSchema = withParser(
  Schema.Literal(
    'child.macos.service.package.proof.reported',
    'child.macos.service.lifecycle.proof.reported',
    'child.macos.service.manual-proof.reported'
  )
);

const ChildMacosServiceLabelSchema = brandedNonEmptyStringSchema('ChildMacosServiceLabel');
const ChildMacosServicePathSchema = brandedNonEmptyStringSchema('ChildMacosServicePath');
const ChildMacosServiceRequirementSchema = brandedNonEmptyStringSchema('ChildMacosServiceRequirement');
const ChildMacosServiceBoundarySchema = brandedNonEmptyStringSchema('ChildMacosServiceBoundary');

export const ChildMacosServiceSurfaceProofSchema = withParser(
  Schema.Struct({
    surface: ChildMacosServiceSurfaceNameSchema,
    parentCapability: ParentControlCapabilityNameSchema,
    parentCapabilityStatus: ParentControlCapabilityStatusSchema,
    proofState: ChildMacosServiceProofStateSchema,
    runtimeOwner: ChildMacosServiceRuntimeOwnerSchema,
    proofRequirement: ChildMacosServiceRequirementSchema,
    claimBoundary: ChildMacosServiceBoundarySchema,
  })
);

export const ChildMacosServiceLifecycleProofSchema = withParser(
  Schema.Struct({
    phase: ChildMacosServiceLifecyclePhaseSchema,
    proofState: ChildMacosServiceProofStateSchema,
    runtimeOwner: ChildMacosServiceRuntimeOwnerSchema,
    proofRequirement: ChildMacosServiceRequirementSchema,
    claimBoundary: ChildMacosServiceBoundarySchema,
  })
);

export const ChildMacosServiceProtocolBridgeProofSchema = withParser(
  Schema.Struct({
    serviceLabel: ChildMacosServiceLabelSchema,
    plistPath: ChildMacosServicePathSchema,
    binaryPath: ChildMacosServicePathSchema,
    commands: Schema.Array(ChildMacosServiceProtocolCommandSchema),
    events: Schema.Array(ChildMacosServiceProtocolEventSchema),
    runtimeOwner: ChildMacosServiceRuntimeOwnerSchema,
    proofRequirement: ChildMacosServiceRequirementSchema,
    claimBoundary: ChildMacosServiceBoundarySchema,
  })
);

export const ChildMacosServiceClaimBoundariesSchema = withParser(
  Schema.Struct({
    packageArtifact: ChildMacosServiceBoundarySchema,
    launchdBoundary: ChildMacosServiceBoundarySchema,
    runtimeBoundary: ChildMacosServiceBoundarySchema,
    restartBoundary: ChildMacosServiceBoundarySchema,
    signingBoundary: ChildMacosServiceBoundarySchema,
    notarizationBoundary: ChildMacosServiceBoundarySchema,
    entitlementBoundary: ChildMacosServiceBoundarySchema,
    uninstallBoundary: ChildMacosServiceBoundarySchema,
    removalBoundary: ChildMacosServiceBoundarySchema,
    parentParityBoundary: ChildMacosServiceBoundarySchema,
  })
);

const ChildMacosServiceReadModelBaseSchema = Schema.Struct({
  schemaVersion: ChildMacosServicePackageProofSchemaVersionSchema,
  serviceLabel: ChildMacosServiceLabelSchema,
  plistPath: ChildMacosServicePathSchema,
  binaryPath: ChildMacosServicePathSchema,
  distributionMode: ChildMacosServiceDistributionModeSchema,
  artifactState: ChildMacosServiceArtifactStateSchema,
  launchdBoundaryState: ChildMacosServiceLaunchdBoundaryStateSchema,
  installState: ChildMacosServiceInstallStateSchema,
  runtimeState: ChildMacosServiceRuntimeStateSchema,
  restartState: ChildMacosServiceRestartStateSchema,
  signingState: ChildMacosServiceSigningStateSchema,
  notarizationState: ChildMacosServiceNotarizationStateSchema,
  entitlementState: ChildMacosServiceEntitlementStateSchema,
  uninstallState: ChildMacosServiceUninstallStateSchema,
  removalState: ChildMacosServiceRemovalStateSchema,
  protocolBridgeProof: ChildMacosServiceProtocolBridgeProofSchema,
  surfaceProofs: Schema.Array(ChildMacosServiceSurfaceProofSchema),
  lifecycleProofs: Schema.Array(ChildMacosServiceLifecycleProofSchema),
  claimBoundaries: ChildMacosServiceClaimBoundariesSchema,
  updatedAt: ParentTimestampSchema,
});

type ChildMacosServiceReadModelCandidate = Infer<typeof ChildMacosServiceReadModelBaseSchema>;

export const ChildMacosServicePackageProofReadModelSchema = withParser(
  ChildMacosServiceReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        childMacosServicePackageProofReadModelIsHonest(readModel) ||
        'Expected child macOS service package proof to keep the package as a launchd pkg script boundary, declare KeepAlive without upgrading restart proof, keep signing unsigned, keep notarization/entitlements/uninstall/removal manual-required, and avoid parent-client or hidden background-service parity claims without macOS artifacts'
    )
  )
);

const ExpectedProtocolBridgeProofRequirement =
  'macOS child package proof names the launchd service boundary, install script bootstrap path, and manual-required runtime gaps';
const ExpectedProtocolBridgeClaimBoundary =
  'launchd plist and install scripts prove only the macOS service-manager boundary; they do not prove installed runtime health, restart recovery, notarization, or parent-client parity';

const RequiredSurfaces = [
  'pkgbuild-script',
  'service-binary-path',
  'launchd-plist',
  'launchctl-bootstrap',
  'launchctl-enable',
  'run-at-load',
  'keepalive-declaration',
  'signing-review',
  'notarization-review',
  'entitlement-review',
  'uninstall-disable-review',
  'removal-review',
] as const satisfies ReadonlyArray<ChildMacosServiceSurfaceName>;

const RequiredLifecyclePhases = [
  'release-script',
  'binary-stage',
  'launchd-plist',
  'package-build',
  'install-bootstrap',
  'install-enable',
  'service-start',
  'restart-recovery',
  'signing-review',
  'notarization-review',
  'uninstall-disable',
  'removal-cleanup',
] as const satisfies ReadonlyArray<ChildMacosServiceLifecyclePhase>;

const RequiredCommands = [
  'child.macos.service.package.proof.get',
  'child.macos.service.lifecycle.proof.get',
  'child.macos.service.manual-proof.get',
] as const satisfies ReadonlyArray<ChildMacosServiceProtocolCommand>;

const RequiredEvents = [
  'child.macos.service.package.proof.reported',
  'child.macos.service.lifecycle.proof.reported',
  'child.macos.service.manual-proof.reported',
] as const satisfies ReadonlyArray<ChildMacosServiceProtocolEvent>;

const SurfaceExpectations = {
  'pkgbuild-script': {
    parentCapability: 'package-lifecycle',
    parentCapabilityStatus: 'manual-required',
    proofState: 'ci-mechanical-proof',
    runtimeOwner: 'macos-pkgbuild-script',
  },
  'service-binary-path': {
    parentCapability: 'headless-agent-service',
    parentCapabilityStatus: 'manual-required',
    proofState: 'ci-mechanical-proof',
    runtimeOwner: 'macos-release-binary',
  },
  'launchd-plist': {
    parentCapability: 'headless-agent-service',
    parentCapabilityStatus: 'manual-required',
    proofState: 'ci-mechanical-proof',
    runtimeOwner: 'macos-launchd-plist',
  },
  'launchctl-bootstrap': {
    parentCapability: 'headless-agent-service',
    parentCapabilityStatus: 'manual-required',
    proofState: 'ci-mechanical-proof',
    runtimeOwner: 'macos-launchctl-script',
  },
  'launchctl-enable': {
    parentCapability: 'headless-agent-service',
    parentCapabilityStatus: 'manual-required',
    proofState: 'ci-mechanical-proof',
    runtimeOwner: 'macos-launchctl-script',
  },
  'run-at-load': {
    parentCapability: 'headless-agent-service',
    parentCapabilityStatus: 'manual-required',
    proofState: 'ci-mechanical-proof',
    runtimeOwner: 'macos-launchd-plist',
  },
  'keepalive-declaration': {
    parentCapability: 'headless-agent-service',
    parentCapabilityStatus: 'manual-required',
    proofState: 'ci-mechanical-proof',
    runtimeOwner: 'macos-launchd-plist',
  },
  'signing-review': {
    parentCapability: 'signing-entitlements',
    parentCapabilityStatus: 'manual-required',
    proofState: 'unsigned',
    runtimeOwner: 'apple-codesign',
  },
  'notarization-review': {
    parentCapability: 'store-distribution',
    parentCapabilityStatus: 'manual-required',
    proofState: 'manual-required',
    runtimeOwner: 'apple-notarytool',
  },
  'entitlement-review': {
    parentCapability: 'signing-entitlements',
    parentCapabilityStatus: 'manual-required',
    proofState: 'manual-required',
    runtimeOwner: 'apple-codesign',
  },
  'uninstall-disable-review': {
    parentCapability: 'package-lifecycle',
    parentCapabilityStatus: 'manual-required',
    proofState: 'manual-required',
    runtimeOwner: 'macos-manual-proof',
  },
  'removal-review': {
    parentCapability: 'package-lifecycle',
    parentCapabilityStatus: 'manual-required',
    proofState: 'manual-required',
    runtimeOwner: 'macos-manual-proof',
  },
} as const satisfies Record<
  ChildMacosServiceSurfaceName,
  Pick<ChildMacosServiceSurfaceProof, 'parentCapability' | 'parentCapabilityStatus' | 'proofState' | 'runtimeOwner'>
>;

const ExpectedClaimBoundaries = {
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
} as const satisfies Record<keyof ChildMacosServiceClaimBoundaries, string>;

function childMacosServicePackageProofReadModelIsHonest(readModel: ChildMacosServiceReadModelCandidate): boolean {
  return (
    serviceIdentityAndStatesAreHonest(readModel) &&
    protocolBridgeProofIsHonest(readModel.protocolBridgeProof) &&
    surfaceProofsAreHonest(readModel.surfaceProofs) &&
    lifecycleProofsAreHonest(readModel.lifecycleProofs) &&
    claimBoundariesAreHonest(readModel.claimBoundaries)
  );
}

function serviceIdentityAndStatesAreHonest(readModel: ChildMacosServiceReadModelCandidate): boolean {
  return [
    readModel.serviceLabel === 'ca.ocentra.parent.agent',
    readModel.plistPath === '/Library/LaunchDaemons/ca.ocentra.parent.agent.plist',
    readModel.binaryPath === '/Library/Ocentra/Ocentra Parent Agent/bin/ocentra-parent-agent-service',
    readModel.distributionMode === 'launchd-pkg-script',
    readModel.artifactState === 'pkg-script-defined',
    readModel.launchdBoundaryState === 'launchd-boundary-scripted',
    readModel.installState === 'manual-install-proof-required',
    readModel.runtimeState === 'manual-service-proof-required',
    readModel.restartState === 'keepalive-declared-manual-recovery-proof',
    readModel.signingState === 'unsigned',
    readModel.notarizationState === 'manual-required',
    readModel.entitlementState === 'manual-required',
    readModel.uninstallState === 'manual-uninstall-proof-required',
    readModel.removalState === 'manual-removal-proof-required',
  ].every(Boolean);
}

function protocolBridgeProofIsHonest(proof: ChildMacosServiceProtocolBridgeProof): boolean {
  return (
    proof.serviceLabel === 'ca.ocentra.parent.agent' &&
    proof.plistPath === '/Library/LaunchDaemons/ca.ocentra.parent.agent.plist' &&
    proof.binaryPath === '/Library/Ocentra/Ocentra Parent Agent/bin/ocentra-parent-agent-service' &&
    proof.runtimeOwner === 'macos-launchctl-script' &&
    proof.proofRequirement === ExpectedProtocolBridgeProofRequirement &&
    proof.claimBoundary === ExpectedProtocolBridgeClaimBoundary &&
    requiredValuesArePresent(proof.commands, RequiredCommands) &&
    requiredValuesArePresent(proof.events, RequiredEvents)
  );
}

function surfaceProofsAreHonest(proofs: ReadonlyArray<ChildMacosServiceSurfaceProof>): boolean {
  const bySurface = new Map(proofs.map((entry) => [entry.surface, entry] as const));
  return (
    bySurface.size === proofs.length &&
    RequiredSurfaces.every((surface) => surfaceProofIsHonest(bySurface.get(surface), surface))
  );
}

function surfaceProofIsHonest(
  proof: ChildMacosServiceSurfaceProof | undefined,
  surface: ChildMacosServiceSurfaceName
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

function lifecycleProofsAreHonest(proofs: ReadonlyArray<ChildMacosServiceLifecycleProof>): boolean {
  const byPhase = new Map(proofs.map((entry) => [entry.phase, entry] as const));
  return (
    byPhase.size === proofs.length &&
    RequiredLifecyclePhases.every((phase) => lifecyclePhaseIsHonest(byPhase.get(phase), phase))
  );
}

function lifecyclePhaseIsHonest(
  proof: ChildMacosServiceLifecycleProof | undefined,
  phase: ChildMacosServiceLifecyclePhase
): boolean {
  if (!proof || proof.phase !== phase) {
    return false;
  }

  if (
    phase === 'release-script' ||
    phase === 'binary-stage' ||
    phase === 'launchd-plist' ||
    phase === 'package-build' ||
    phase === 'install-bootstrap' ||
    phase === 'install-enable'
  ) {
    return proof.proofState === 'ci-mechanical-proof';
  }

  if (phase === 'signing-review') {
    return proof.proofState === 'unsigned';
  }

  return proof.proofState === 'manual-required';
}

function claimBoundariesAreHonest(boundaries: ChildMacosServiceClaimBoundaries): boolean {
  return Object.entries(ExpectedClaimBoundaries).every(
    ([key, value]) => boundaries[key as keyof ChildMacosServiceClaimBoundaries] === value
  );
}

function requiredValuesArePresent<Value extends string>(
  values: ReadonlyArray<Value>,
  required: ReadonlyArray<Value>
): boolean {
  const valueSet = new Set(values);
  return valueSet.size === values.length && required.every((value) => valueSet.has(value));
}

export type ChildMacosServiceSurfaceName = Infer<typeof ChildMacosServiceSurfaceNameSchema>;
export type ChildMacosServiceProofState = Infer<typeof ChildMacosServiceProofStateSchema>;
export type ChildMacosServiceRuntimeOwner = Infer<typeof ChildMacosServiceRuntimeOwnerSchema>;
export type ChildMacosServiceArtifactState = Infer<typeof ChildMacosServiceArtifactStateSchema>;
export type ChildMacosServiceDistributionMode = Infer<typeof ChildMacosServiceDistributionModeSchema>;
export type ChildMacosServiceLaunchdBoundaryState = Infer<typeof ChildMacosServiceLaunchdBoundaryStateSchema>;
export type ChildMacosServiceInstallState = Infer<typeof ChildMacosServiceInstallStateSchema>;
export type ChildMacosServiceRuntimeState = Infer<typeof ChildMacosServiceRuntimeStateSchema>;
export type ChildMacosServiceRestartState = Infer<typeof ChildMacosServiceRestartStateSchema>;
export type ChildMacosServiceSigningState = Infer<typeof ChildMacosServiceSigningStateSchema>;
export type ChildMacosServiceNotarizationState = Infer<typeof ChildMacosServiceNotarizationStateSchema>;
export type ChildMacosServiceEntitlementState = Infer<typeof ChildMacosServiceEntitlementStateSchema>;
export type ChildMacosServiceUninstallState = Infer<typeof ChildMacosServiceUninstallStateSchema>;
export type ChildMacosServiceRemovalState = Infer<typeof ChildMacosServiceRemovalStateSchema>;
export type ChildMacosServiceLifecyclePhase = Infer<typeof ChildMacosServiceLifecyclePhaseSchema>;
export type ChildMacosServiceProtocolCommand = Infer<typeof ChildMacosServiceProtocolCommandSchema>;
export type ChildMacosServiceProtocolEvent = Infer<typeof ChildMacosServiceProtocolEventSchema>;
export type ChildMacosServiceSurfaceProof = Infer<typeof ChildMacosServiceSurfaceProofSchema>;
export type ChildMacosServiceLifecycleProof = Infer<typeof ChildMacosServiceLifecycleProofSchema>;
export type ChildMacosServiceProtocolBridgeProof = Infer<typeof ChildMacosServiceProtocolBridgeProofSchema>;
export type ChildMacosServiceClaimBoundaries = Infer<typeof ChildMacosServiceClaimBoundariesSchema>;
export type ChildMacosServicePackageProofReadModel = Infer<typeof ChildMacosServicePackageProofReadModelSchema>;

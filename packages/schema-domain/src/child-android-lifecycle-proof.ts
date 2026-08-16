/* generated from crates/schema/src/child_android_lifecycle_proof_ts.rs */

import { type Infer, brandedNonEmptyStringSchema, Schema, withParser } from './effect';
import { ParentControlCapabilityNameSchema, ParentControlCapabilityStatusSchema } from './capabilities';
import { ParentTimestampSchema } from './family-reference-primitives';

export const ChildAndroidLifecycleProofSchemaVersionSchema = withParser(
  Schema.Literal('child-android-protocol-package-lifecycle-proof')
);
export const ChildAndroidLifecycleCapabilityNameSchema = withParser(
  Schema.Literal(
    'foreground-mobile-service',
    'notifications',
    'local-storage',
    'typed-protocol-bridge',
    'usage-stats',
    'accessibility-service',
    'vpn-dns-filtering',
    'device-owner-policy',
    'managed-profile',
    'package-lifecycle',
    'store-distribution'
  )
);
export const ChildAndroidLifecycleProofStateSchema = withParser(
  Schema.Literal('ci-mechanical-proof', 'scaffold', 'manual-required', 'planned', 'not-implemented')
);
export const ChildAndroidLifecycleRuntimeOwnerSchema = withParser(
  Schema.Literal(
    'android-native-wrapper',
    'android-manifest',
    'android-package-build',
    'android-os-permission',
    'manual-device-proof',
    'store-distribution'
  )
);
export const ChildAndroidLifecyclePackagePhaseSchema = withParser(
  Schema.Literal(
    'debug-apk-build',
    'checksum',
    'launcher-activity',
    'foreground-service-registration',
    'notification-permission-declared',
    'install',
    'update',
    'background-execution',
    'reboot-recovery',
    'uninstall'
  )
);
export const ChildAndroidProtocolCommandSchema = withParser(
  Schema.Literal(
    'child.android.lifecycle.snapshot.get',
    'child.android.capabilities.snapshot.get',
    'child.android.package.lifecycle.proof.get'
  )
);
export const ChildAndroidProtocolEventSchema = withParser(
  Schema.Literal(
    'child.android.lifecycle.snapshot.reported',
    'child.android.capability.snapshot.reported',
    'child.android.package.lifecycle.proof.reported'
  )
);
export const ChildAndroidProtocolBridgeStateSchema = withParser(
  Schema.Literal('package-local-scaffold', 'not-implemented')
);
export const ChildAndroidPermissionDeclarationStateSchema = withParser(
  Schema.Literal('declared-in-manifest', 'manual-required')
);
export const ChildAndroidRuntimeGrantStateSchema = withParser(Schema.Literal('not-applicable', 'manual-required'));
export const ChildAndroidInstallModeSchema = withParser(Schema.Literal('debug-apk-sideload'));
export const ChildAndroidChildAgentArtifactStateSchema = withParser(Schema.Literal('debug-apk-built'));
export const ChildAndroidInstallStateSchema = withParser(Schema.Literal('manual-install-proof-required'));
export const ChildAndroidLaunchStateSchema = withParser(Schema.Literal('manual-launch-proof-required'));
export const ChildAndroidRemovalStateSchema = withParser(Schema.Literal('manual-removal-proof-required'));
export const ChildAndroidPlatformAuthorityStateSchema = withParser(Schema.Literal('manual-required'));

const ChildAndroidPackageIdSchema = brandedNonEmptyStringSchema('ChildAndroidPackageId');
const ChildAndroidClassNameSchema = brandedNonEmptyStringSchema('ChildAndroidClassName');
const ChildAndroidPathSchema = brandedNonEmptyStringSchema('ChildAndroidProofPath');
const ChildAndroidCommandTextSchema = brandedNonEmptyStringSchema('ChildAndroidCommandText');
const ChildAndroidProofRequirementSchema = brandedNonEmptyStringSchema('ChildAndroidProofRequirement');
const ChildAndroidClaimBoundarySchema = brandedNonEmptyStringSchema('ChildAndroidClaimBoundary');
const ChildAndroidPermissionNameSchema = brandedNonEmptyStringSchema('ChildAndroidPermissionName');

export const ChildAndroidCapabilityProofSchema = withParser(
  Schema.Struct({
    capability: ChildAndroidLifecycleCapabilityNameSchema,
    parentCapability: ParentControlCapabilityNameSchema,
    parentCapabilityStatus: ParentControlCapabilityStatusSchema,
    proofState: ChildAndroidLifecycleProofStateSchema,
    runtimeOwner: ChildAndroidLifecycleRuntimeOwnerSchema,
    proofRequirement: ChildAndroidProofRequirementSchema,
    claimBoundary: ChildAndroidClaimBoundarySchema,
  })
);

export const ChildAndroidPackageProofSchema = withParser(
  Schema.Struct({
    packageId: ChildAndroidPackageIdSchema,
    applicationId: ChildAndroidPackageIdSchema,
    launchActivity: ChildAndroidClassNameSchema,
    foregroundService: ChildAndroidClassNameSchema,
    nativeBridgeClass: ChildAndroidClassNameSchema,
    minSdk: Schema.Number,
    targetSdk: Schema.Number,
    versionName: ChildAndroidCommandTextSchema,
    debugApkPath: ChildAndroidPathSchema,
    latestApkPath: ChildAndroidPathSchema,
    checksumState: ChildAndroidLifecycleProofStateSchema,
    releaseCommand: ChildAndroidCommandTextSchema,
  })
);

export const ChildAndroidPackageLifecycleAssertionSchema = withParser(
  Schema.Struct({
    phase: ChildAndroidLifecyclePackagePhaseSchema,
    proofState: ChildAndroidLifecycleProofStateSchema,
    runtimeOwner: ChildAndroidLifecycleRuntimeOwnerSchema,
    proofRequirement: ChildAndroidProofRequirementSchema,
    claimBoundary: ChildAndroidClaimBoundarySchema,
  })
);

export const ChildAndroidProtocolBridgeProofSchema = withParser(
  Schema.Struct({
    bridgeState: ChildAndroidProtocolBridgeStateSchema,
    externalTransportState: ChildAndroidProtocolBridgeStateSchema,
    commands: Schema.Array(ChildAndroidProtocolCommandSchema),
    events: Schema.Array(ChildAndroidProtocolEventSchema),
    nativeBridgeClass: ChildAndroidClassNameSchema,
    runtimeOwner: ChildAndroidLifecycleRuntimeOwnerSchema,
    proofRequirement: ChildAndroidProofRequirementSchema,
    claimBoundary: ChildAndroidClaimBoundarySchema,
  })
);

export const ChildAndroidPermissionProofSchema = withParser(
  Schema.Struct({
    permission: ChildAndroidPermissionNameSchema,
    declarationState: ChildAndroidPermissionDeclarationStateSchema,
    runtimeGrantState: ChildAndroidRuntimeGrantStateSchema,
    proofRequirement: ChildAndroidProofRequirementSchema,
  })
);

export const ChildAndroidInstallAuthorityProofSchema = withParser(
  Schema.Struct({
    childAgentArtifactState: ChildAndroidChildAgentArtifactStateSchema,
    installMode: ChildAndroidInstallModeSchema,
    installState: ChildAndroidInstallStateSchema,
    launchState: ChildAndroidLaunchStateSchema,
    removalState: ChildAndroidRemovalStateSchema,
    deviceOwnerAuthorityState: ChildAndroidPlatformAuthorityStateSchema,
    managedProfileAuthorityState: ChildAndroidPlatformAuthorityStateSchema,
    childAgentArtifactBoundary: ChildAndroidClaimBoundarySchema,
    installModeBoundary: ChildAndroidClaimBoundarySchema,
    installStateBoundary: ChildAndroidClaimBoundarySchema,
    launchStateBoundary: ChildAndroidClaimBoundarySchema,
    removalStateBoundary: ChildAndroidClaimBoundarySchema,
    deviceOwnerBoundary: ChildAndroidClaimBoundarySchema,
    managedProfileBoundary: ChildAndroidClaimBoundarySchema,
  })
);

export const ChildAndroidLifecycleClaimBoundariesSchema = withParser(
  Schema.Struct({
    childAndroidEnforcementParity: ChildAndroidClaimBoundarySchema,
    foregroundServiceRuntime: ChildAndroidClaimBoundarySchema,
    notificationRuntime: ChildAndroidClaimBoundarySchema,
    accessibility: ChildAndroidClaimBoundarySchema,
    vpnDns: ChildAndroidClaimBoundarySchema,
    deviceOwner: ChildAndroidClaimBoundarySchema,
    managedProfile: ChildAndroidClaimBoundarySchema,
    usageStats: ChildAndroidClaimBoundarySchema,
    packageLifecycle: ChildAndroidClaimBoundarySchema,
    physicalDevice: ChildAndroidClaimBoundarySchema,
    storeDistribution: ChildAndroidClaimBoundarySchema,
  })
);

const ChildAndroidLifecycleReadModelBaseSchema = Schema.Struct({
  schemaVersion: ChildAndroidLifecycleProofSchemaVersionSchema,
  packageProof: ChildAndroidPackageProofSchema,
  protocolBridgeProof: ChildAndroidProtocolBridgeProofSchema,
  capabilityProofs: Schema.Array(ChildAndroidCapabilityProofSchema),
  packageLifecycleAssertions: Schema.Array(ChildAndroidPackageLifecycleAssertionSchema),
  permissionProofs: Schema.Array(ChildAndroidPermissionProofSchema),
  installAuthorityProof: ChildAndroidInstallAuthorityProofSchema,
  claimBoundaries: ChildAndroidLifecycleClaimBoundariesSchema,
  updatedAt: ParentTimestampSchema,
});

type ChildAndroidLifecycleReadModelCandidate = Infer<typeof ChildAndroidLifecycleReadModelBaseSchema>;

export const ChildAndroidLifecycleReadModelSchema = withParser(
  ChildAndroidLifecycleReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        childAndroidLifecycleReadModelIsHonest(readModel) ||
        'Expected Child Android lifecycle proof to keep package build and bridge proof CI-mechanical while Android permissions, device-owner, accessibility, VPN/DNS, managed profile, install/update/background/reboot/uninstall, store distribution, and enforcement parity remain manual-required, planned, or unimplemented'
    )
  )
);

const RequiredCapabilities = [
  'foreground-mobile-service',
  'notifications',
  'local-storage',
  'typed-protocol-bridge',
  'usage-stats',
  'accessibility-service',
  'vpn-dns-filtering',
  'device-owner-policy',
  'managed-profile',
  'package-lifecycle',
  'store-distribution',
] as const satisfies ReadonlyArray<ChildAndroidLifecycleCapabilityName>;

const RequiredCiPackagePhases = [
  'debug-apk-build',
  'checksum',
  'launcher-activity',
  'foreground-service-registration',
  'notification-permission-declared',
] as const satisfies ReadonlyArray<ChildAndroidLifecyclePackagePhase>;

const RequiredManualPackagePhases = [
  'install',
  'update',
  'background-execution',
  'reboot-recovery',
  'uninstall',
] as const satisfies ReadonlyArray<ChildAndroidLifecyclePackagePhase>;

const RequiredCommands = [
  'child.android.lifecycle.snapshot.get',
  'child.android.capabilities.snapshot.get',
  'child.android.package.lifecycle.proof.get',
] as const satisfies ReadonlyArray<ChildAndroidProtocolCommand>;

const RequiredEvents = [
  'child.android.lifecycle.snapshot.reported',
  'child.android.capability.snapshot.reported',
  'child.android.package.lifecycle.proof.reported',
] as const satisfies ReadonlyArray<ChildAndroidProtocolEvent>;

function childAndroidLifecycleReadModelIsHonest(readModel: ChildAndroidLifecycleReadModelCandidate): boolean {
  return (
    packageProofIsHonest(readModel.packageProof) &&
    protocolBridgeProofIsHonest(readModel.protocolBridgeProof, readModel.packageProof.nativeBridgeClass) &&
    capabilityProofsAreHonest(readModel.capabilityProofs) &&
    lifecycleAssertionsAreHonest(readModel.packageLifecycleAssertions) &&
    permissionProofsAreHonest(readModel.permissionProofs) &&
    installAuthorityProofIsHonest(readModel.installAuthorityProof)
  );
}

function packageProofIsHonest(packageProof: ChildAndroidPackageProof): boolean {
  return (
    packageProof.packageId === 'ca.ocentra.parent.agent' &&
    packageProof.applicationId === packageProof.packageId &&
    packageProof.launchActivity === 'ca.ocentra.parent.agent/.MainActivity' &&
    packageProof.foregroundService === 'ca.ocentra.parent.agent/.OcentraParentAgentService' &&
    packageProof.nativeBridgeClass === 'ca.ocentra.parent.agent.ChildAndroidLifecycleProof' &&
    packageProof.minSdk >= 26 &&
    packageProof.targetSdk >= 35 &&
    packageProof.debugApkPath.endsWith('.apk') &&
    packageProof.latestApkPath.endsWith('.apk') &&
    packageProof.checksumState === 'ci-mechanical-proof' &&
    packageProof.releaseCommand === 'cmd /c npm run release:package:android'
  );
}

function protocolBridgeProofIsHonest(
  bridgeProof: ChildAndroidProtocolBridgeProof,
  nativeBridgeClass: ChildAndroidPackageProof['nativeBridgeClass']
): boolean {
  return (
    bridgeProof.bridgeState === 'package-local-scaffold' &&
    bridgeProof.externalTransportState === 'not-implemented' &&
    bridgeProof.nativeBridgeClass === nativeBridgeClass &&
    bridgeProof.runtimeOwner === 'android-native-wrapper' &&
    requiredValuesArePresent(bridgeProof.commands, RequiredCommands) &&
    requiredValuesArePresent(bridgeProof.events, RequiredEvents)
  );
}

function capabilityProofsAreHonest(proofs: ReadonlyArray<ChildAndroidCapabilityProof>): boolean {
  const byCapability = new Map(proofs.map((entry) => [entry.capability, entry] as const));
  if (byCapability.size !== proofs.length || byCapability.size !== RequiredCapabilities.length) {
    return false;
  }

  return RequiredCapabilities.every((capability) => {
    const proof = byCapability.get(capability);
    if (!proof || proof.parentCapability !== capability) {
      return false;
    }
    return capabilityProofStateIsHonest(proof);
  });
}

function capabilityProofStateIsHonest(proof: ChildAndroidCapabilityProof): boolean {
  if (proof.capability === 'foreground-mobile-service' || proof.capability === 'package-lifecycle') {
    return proof.parentCapabilityStatus === 'manual-required' && proof.proofState === 'ci-mechanical-proof';
  }

  if (proof.capability === 'typed-protocol-bridge') {
    return proof.parentCapabilityStatus === 'scaffold' && proof.proofState === 'ci-mechanical-proof';
  }

  if (proof.capability === 'local-storage') {
    return proof.parentCapabilityStatus === 'scaffold' && proof.proofState === 'scaffold';
  }

  if (proof.capability === 'store-distribution') {
    return proof.parentCapabilityStatus === 'planned' && proof.proofState === 'planned';
  }

  return proof.parentCapabilityStatus === 'manual-required' && proof.proofState === 'manual-required';
}

function lifecycleAssertionsAreHonest(assertions: ReadonlyArray<ChildAndroidPackageLifecycleAssertion>): boolean {
  const byPhase = new Map(assertions.map((entry) => [entry.phase, entry] as const));
  return (
    byPhase.size === assertions.length &&
    RequiredCiPackagePhases.every((phase) => byPhase.get(phase)?.proofState === 'ci-mechanical-proof') &&
    RequiredManualPackagePhases.every((phase) => byPhase.get(phase)?.proofState === 'manual-required')
  );
}

function permissionProofsAreHonest(proofs: ReadonlyArray<ChildAndroidPermissionProof>): boolean {
  const permissions = new Map(proofs.map((entry) => [String(entry.permission), entry] as const));
  const foreground = permissions.get('android.permission.FOREGROUND_SERVICE');
  const dataSync = permissions.get('android.permission.FOREGROUND_SERVICE_DATA_SYNC');
  const notifications = permissions.get('android.permission.POST_NOTIFICATIONS');

  return (
    foreground?.declarationState === 'declared-in-manifest' &&
    foreground.runtimeGrantState === 'not-applicable' &&
    dataSync?.declarationState === 'declared-in-manifest' &&
    dataSync.runtimeGrantState === 'not-applicable' &&
    notifications?.declarationState === 'declared-in-manifest' &&
    notifications.runtimeGrantState === 'manual-required'
  );
}

function installAuthorityProofIsHonest(proof: ChildAndroidInstallAuthorityProof): boolean {
  return [
    proof.childAgentArtifactState === 'debug-apk-built',
    proof.installMode === 'debug-apk-sideload',
    proof.installState === 'manual-install-proof-required',
    proof.launchState === 'manual-launch-proof-required',
    proof.removalState === 'manual-removal-proof-required',
    proof.deviceOwnerAuthorityState === 'manual-required',
    proof.managedProfileAuthorityState === 'manual-required',
    proof.childAgentArtifactBoundary.includes('child-agent artifact'),
    proof.installModeBoundary.includes('debug APK sideload mode'),
    proof.installStateBoundary.includes('manual-required'),
    proof.launchStateBoundary.includes('manual-required'),
    proof.removalStateBoundary.includes('manual-required'),
    proof.deviceOwnerBoundary.includes('no device-owner claim'),
    proof.managedProfileBoundary.includes('no managed-profile claim'),
  ].every(Boolean);
}

function requiredValuesArePresent<Value extends string>(
  values: ReadonlyArray<Value>,
  required: ReadonlyArray<Value>
): boolean {
  const valueSet = new Set(values);
  return valueSet.size === values.length && required.every((value) => valueSet.has(value));
}

export type ChildAndroidLifecycleCapabilityName = Infer<typeof ChildAndroidLifecycleCapabilityNameSchema>;
export type ChildAndroidLifecycleProofState = Infer<typeof ChildAndroidLifecycleProofStateSchema>;
export type ChildAndroidLifecycleRuntimeOwner = Infer<typeof ChildAndroidLifecycleRuntimeOwnerSchema>;
export type ChildAndroidLifecyclePackagePhase = Infer<typeof ChildAndroidLifecyclePackagePhaseSchema>;
export type ChildAndroidProtocolCommand = Infer<typeof ChildAndroidProtocolCommandSchema>;
export type ChildAndroidProtocolEvent = Infer<typeof ChildAndroidProtocolEventSchema>;
export type ChildAndroidProtocolBridgeState = Infer<typeof ChildAndroidProtocolBridgeStateSchema>;
export type ChildAndroidPermissionDeclarationState = Infer<typeof ChildAndroidPermissionDeclarationStateSchema>;
export type ChildAndroidRuntimeGrantState = Infer<typeof ChildAndroidRuntimeGrantStateSchema>;
export type ChildAndroidInstallMode = Infer<typeof ChildAndroidInstallModeSchema>;
export type ChildAndroidChildAgentArtifactState = Infer<typeof ChildAndroidChildAgentArtifactStateSchema>;
export type ChildAndroidInstallState = Infer<typeof ChildAndroidInstallStateSchema>;
export type ChildAndroidLaunchState = Infer<typeof ChildAndroidLaunchStateSchema>;
export type ChildAndroidRemovalState = Infer<typeof ChildAndroidRemovalStateSchema>;
export type ChildAndroidPlatformAuthorityState = Infer<typeof ChildAndroidPlatformAuthorityStateSchema>;
export type ChildAndroidCapabilityProof = Infer<typeof ChildAndroidCapabilityProofSchema>;
export type ChildAndroidPackageProof = Infer<typeof ChildAndroidPackageProofSchema>;
export type ChildAndroidPackageLifecycleAssertion = Infer<typeof ChildAndroidPackageLifecycleAssertionSchema>;
export type ChildAndroidProtocolBridgeProof = Infer<typeof ChildAndroidProtocolBridgeProofSchema>;
export type ChildAndroidPermissionProof = Infer<typeof ChildAndroidPermissionProofSchema>;
export type ChildAndroidInstallAuthorityProof = Infer<typeof ChildAndroidInstallAuthorityProofSchema>;
export type ChildAndroidLifecycleClaimBoundaries = Infer<typeof ChildAndroidLifecycleClaimBoundariesSchema>;
export type ChildAndroidLifecycleReadModel = Infer<typeof ChildAndroidLifecycleReadModelSchema>;

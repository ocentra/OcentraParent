/* generated from crates/schema/src/child_android_permission_capability_proof_ts.rs */

import { type Infer, brandedNonEmptyStringSchema, Schema, withParser } from './effect';
import { ParentControlCapabilityNameSchema, ParentControlCapabilityStatusSchema } from './capabilities';
import { ParentTimestampSchema } from './family-reference-primitives';

export const ChildAndroidPermissionCapabilityProofSchemaVersionSchema = withParser(
  Schema.Literal('child-android-permission-capability-proof')
);
export const ChildAndroidPermissionSurfaceNameSchema = withParser(
  Schema.Literal(
    'package-debug-apk',
    'foreground-service-permission',
    'post-notifications-permission',
    'usage-stats-permission',
    'accessibility-service',
    'vpn-dns-service',
    'device-owner-policy',
    'managed-profile',
    'app-private-storage',
    'background-service-lifecycle'
  )
);
export const ChildAndroidPermissionProofStateSchema = withParser(
  Schema.Literal(
    'ci-mechanical-proof',
    'declared-in-manifest',
    'package-local-scaffold',
    'manual-required',
    'permission-required',
    'settings-grant-required',
    'not-implemented',
    'blocked'
  )
);
export const ChildAndroidPermissionRuntimeOwnerSchema = withParser(
  Schema.Literal(
    'android-manifest',
    'android-package-build',
    'android-native-wrapper',
    'android-os-permission',
    'android-accessibility-service',
    'android-vpn-service',
    'android-policy-provider',
    'android-app-private-storage',
    'manual-device-proof'
  )
);
export const ChildAndroidPermissionNameSchema = withParser(
  Schema.Literal(
    'android.permission.FOREGROUND_SERVICE',
    'android.permission.FOREGROUND_SERVICE_DATA_SYNC',
    'android.permission.POST_NOTIFICATIONS',
    'android.permission.PACKAGE_USAGE_STATS'
  )
);
export const ChildAndroidPermissionDeclarationStateSchema = withParser(
  Schema.Literal('declared-in-manifest', 'not-declared-by-design', 'not-applicable')
);
export const ChildAndroidPermissionRuntimeGrantStateSchema = withParser(
  Schema.Literal('not-applicable', 'manual-runtime-required', 'manual-settings-required', 'unavailable', 'blocked')
);
export const ChildAndroidPermissionAdapterStateSchema = withParser(
  Schema.Literal(
    'declared-in-manifest',
    'not-declared',
    'not-implemented',
    'blocked-without-enrollment',
    'package-local-scaffold'
  )
);
export const ChildAndroidPermissionPackagePhaseSchema = withParser(
  Schema.Literal(
    'debug-apk-build',
    'checksum',
    'launcher-activity',
    'foreground-service-registration',
    'notification-permission-declared',
    'app-private-storage-path',
    'background-service-start',
    'install',
    'update',
    'reboot-recovery',
    'uninstall'
  )
);
export const ChildAndroidPermissionProtocolCommandSchema = withParser(
  Schema.Literal(
    'child.android.permission.capability.snapshot.get',
    'child.android.permission.package.proof.get',
    'child.android.permission.runtime.manual-proof.get'
  )
);
export const ChildAndroidPermissionProtocolEventSchema = withParser(
  Schema.Literal(
    'child.android.permission.capability.snapshot.reported',
    'child.android.permission.package.proof.reported',
    'child.android.permission.runtime.manual-proof.reported'
  )
);
export const ChildAndroidPermissionBridgeStateSchema = withParser(
  Schema.Literal('package-local-scaffold', 'not-implemented')
);

const ChildAndroidPermissionPackageIdSchema = brandedNonEmptyStringSchema('ChildAndroidPermissionPackageId');
const ChildAndroidPermissionClassNameSchema = brandedNonEmptyStringSchema('ChildAndroidPermissionClassName');
const ChildAndroidPermissionRequirementSchema = brandedNonEmptyStringSchema('ChildAndroidPermissionRequirement');
const ChildAndroidPermissionBoundarySchema = brandedNonEmptyStringSchema('ChildAndroidPermissionBoundary');

export const ChildAndroidPermissionProofSchema = withParser(
  Schema.Struct({
    permission: ChildAndroidPermissionNameSchema,
    parentCapability: ParentControlCapabilityNameSchema,
    declarationState: ChildAndroidPermissionDeclarationStateSchema,
    runtimeGrantState: ChildAndroidPermissionRuntimeGrantStateSchema,
    proofState: ChildAndroidPermissionProofStateSchema,
    runtimeOwner: ChildAndroidPermissionRuntimeOwnerSchema,
    proofRequirement: ChildAndroidPermissionRequirementSchema,
    claimBoundary: ChildAndroidPermissionBoundarySchema,
  })
);

export const ChildAndroidPermissionAdapterProofSchema = withParser(
  Schema.Struct({
    surface: ChildAndroidPermissionSurfaceNameSchema,
    parentCapability: ParentControlCapabilityNameSchema,
    parentCapabilityStatus: ParentControlCapabilityStatusSchema,
    adapterState: ChildAndroidPermissionAdapterStateSchema,
    proofState: ChildAndroidPermissionProofStateSchema,
    runtimeOwner: ChildAndroidPermissionRuntimeOwnerSchema,
    proofRequirement: ChildAndroidPermissionRequirementSchema,
    claimBoundary: ChildAndroidPermissionBoundarySchema,
  })
);

export const ChildAndroidPermissionPackageLifecycleProofSchema = withParser(
  Schema.Struct({
    phase: ChildAndroidPermissionPackagePhaseSchema,
    proofState: ChildAndroidPermissionProofStateSchema,
    runtimeOwner: ChildAndroidPermissionRuntimeOwnerSchema,
    proofRequirement: ChildAndroidPermissionRequirementSchema,
    claimBoundary: ChildAndroidPermissionBoundarySchema,
  })
);

export const ChildAndroidPermissionProtocolBridgeProofSchema = withParser(
  Schema.Struct({
    packageId: ChildAndroidPermissionPackageIdSchema,
    nativeBridgeClass: ChildAndroidPermissionClassNameSchema,
    bridgeState: ChildAndroidPermissionBridgeStateSchema,
    externalTransportState: ChildAndroidPermissionBridgeStateSchema,
    commands: Schema.Array(ChildAndroidPermissionProtocolCommandSchema),
    events: Schema.Array(ChildAndroidPermissionProtocolEventSchema),
    runtimeOwner: ChildAndroidPermissionRuntimeOwnerSchema,
    proofRequirement: ChildAndroidPermissionRequirementSchema,
    claimBoundary: ChildAndroidPermissionBoundarySchema,
  })
);

export const ChildAndroidPermissionClaimBoundariesSchema = withParser(
  Schema.Struct({
    packageLifecycle: ChildAndroidPermissionBoundarySchema,
    foregroundService: ChildAndroidPermissionBoundarySchema,
    notifications: ChildAndroidPermissionBoundarySchema,
    usageStats: ChildAndroidPermissionBoundarySchema,
    accessibility: ChildAndroidPermissionBoundarySchema,
    vpnDns: ChildAndroidPermissionBoundarySchema,
    deviceOwner: ChildAndroidPermissionBoundarySchema,
    managedProfile: ChildAndroidPermissionBoundarySchema,
    appPrivateStorage: ChildAndroidPermissionBoundarySchema,
    backgroundLifecycle: ChildAndroidPermissionBoundarySchema,
    externalTransport: ChildAndroidPermissionBoundarySchema,
  })
);

const ChildAndroidPermissionReadModelBaseSchema = Schema.Struct({
  schemaVersion: ChildAndroidPermissionCapabilityProofSchemaVersionSchema,
  packageId: ChildAndroidPermissionPackageIdSchema,
  nativeBridgeClass: ChildAndroidPermissionClassNameSchema,
  protocolBridgeProof: ChildAndroidPermissionProtocolBridgeProofSchema,
  permissionProofs: Schema.Array(ChildAndroidPermissionProofSchema),
  adapterProofs: Schema.Array(ChildAndroidPermissionAdapterProofSchema),
  packageLifecycleProofs: Schema.Array(ChildAndroidPermissionPackageLifecycleProofSchema),
  claimBoundaries: ChildAndroidPermissionClaimBoundariesSchema,
  updatedAt: ParentTimestampSchema,
});

type ChildAndroidPermissionReadModelCandidate = Infer<typeof ChildAndroidPermissionReadModelBaseSchema>;

export const ChildAndroidPermissionCapabilityReadModelSchema = withParser(
  ChildAndroidPermissionReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        childAndroidPermissionCapabilityReadModelIsHonest(readModel) ||
        'Expected Child Android permission/package proof to keep package, foreground service, notification declaration, and app-private storage as CI/package-local proof while UsageStats, accessibility, VPN/DNS, device-owner, managed profile, install/update/reboot/uninstall, and background lifecycle remain manual-required, unavailable, blocked, or not implemented without device artifacts'
    )
  )
);

const RequiredPermissions = [
  'android.permission.FOREGROUND_SERVICE',
  'android.permission.FOREGROUND_SERVICE_DATA_SYNC',
  'android.permission.POST_NOTIFICATIONS',
  'android.permission.PACKAGE_USAGE_STATS',
] as const satisfies ReadonlyArray<ChildAndroidPermissionName>;

const RequiredAdapterSurfaces = [
  'package-debug-apk',
  'foreground-service-permission',
  'post-notifications-permission',
  'usage-stats-permission',
  'accessibility-service',
  'vpn-dns-service',
  'device-owner-policy',
  'managed-profile',
  'app-private-storage',
  'background-service-lifecycle',
] as const satisfies ReadonlyArray<ChildAndroidPermissionSurfaceName>;

const RequiredLifecyclePhases = [
  'debug-apk-build',
  'checksum',
  'launcher-activity',
  'foreground-service-registration',
  'notification-permission-declared',
  'app-private-storage-path',
  'background-service-start',
  'install',
  'update',
  'reboot-recovery',
  'uninstall',
] as const satisfies ReadonlyArray<ChildAndroidPermissionPackagePhase>;

const RequiredCommands = [
  'child.android.permission.capability.snapshot.get',
  'child.android.permission.package.proof.get',
  'child.android.permission.runtime.manual-proof.get',
] as const satisfies ReadonlyArray<ChildAndroidPermissionProtocolCommand>;

const RequiredEvents = [
  'child.android.permission.capability.snapshot.reported',
  'child.android.permission.package.proof.reported',
  'child.android.permission.runtime.manual-proof.reported',
] as const satisfies ReadonlyArray<ChildAndroidPermissionProtocolEvent>;

const PermissionExpectations = {
  'android.permission.FOREGROUND_SERVICE': {
    declarationState: 'declared-in-manifest',
    runtimeGrantState: 'not-applicable',
    proofState: 'declared-in-manifest',
    runtimeOwner: 'android-manifest',
  },
  'android.permission.FOREGROUND_SERVICE_DATA_SYNC': {
    declarationState: 'declared-in-manifest',
    runtimeGrantState: 'not-applicable',
    proofState: 'declared-in-manifest',
    runtimeOwner: 'android-manifest',
  },
  'android.permission.POST_NOTIFICATIONS': {
    declarationState: 'declared-in-manifest',
    runtimeGrantState: 'manual-runtime-required',
    proofState: 'manual-required',
    runtimeOwner: 'android-os-permission',
  },
  'android.permission.PACKAGE_USAGE_STATS': {
    declarationState: 'not-declared-by-design',
    runtimeGrantState: 'manual-settings-required',
    proofState: 'settings-grant-required',
    runtimeOwner: 'manual-device-proof',
  },
} as const satisfies Record<
  ChildAndroidPermissionName,
  Pick<ChildAndroidPermissionProof, 'declarationState' | 'runtimeGrantState' | 'proofState' | 'runtimeOwner'>
>;

const AdapterExpectations = {
  'package-debug-apk': {
    parentCapability: 'package-lifecycle',
    parentCapabilityStatus: 'manual-required',
    adapterState: 'package-local-scaffold',
    proofState: 'ci-mechanical-proof',
  },
  'foreground-service-permission': {
    parentCapability: 'foreground-mobile-service',
    parentCapabilityStatus: 'manual-required',
    adapterState: 'package-local-scaffold',
    proofState: 'ci-mechanical-proof',
  },
  'post-notifications-permission': {
    parentCapability: 'notifications',
    parentCapabilityStatus: 'manual-required',
    adapterState: 'declared-in-manifest',
    proofState: 'manual-required',
  },
  'usage-stats-permission': {
    parentCapability: 'usage-stats',
    parentCapabilityStatus: 'manual-required',
    adapterState: 'not-declared',
    proofState: 'settings-grant-required',
  },
  'accessibility-service': {
    parentCapability: 'accessibility-service',
    parentCapabilityStatus: 'not-implemented',
    adapterState: 'not-declared',
    proofState: 'not-implemented',
  },
  'vpn-dns-service': {
    parentCapability: 'vpn-dns-filtering',
    parentCapabilityStatus: 'not-implemented',
    adapterState: 'not-declared',
    proofState: 'not-implemented',
  },
  'device-owner-policy': {
    parentCapability: 'device-owner-policy',
    parentCapabilityStatus: 'manual-required',
    adapterState: 'blocked-without-enrollment',
    proofState: 'blocked',
  },
  'managed-profile': {
    parentCapability: 'managed-profile',
    parentCapabilityStatus: 'manual-required',
    adapterState: 'blocked-without-enrollment',
    proofState: 'blocked',
  },
  'app-private-storage': {
    parentCapability: 'local-storage',
    parentCapabilityStatus: 'scaffold',
    adapterState: 'package-local-scaffold',
    proofState: 'package-local-scaffold',
  },
  'background-service-lifecycle': {
    parentCapability: 'background-execution',
    parentCapabilityStatus: 'manual-required',
    adapterState: 'not-implemented',
    proofState: 'manual-required',
  },
} as const satisfies Record<
  ChildAndroidPermissionSurfaceName,
  Pick<
    ChildAndroidPermissionAdapterProof,
    'parentCapability' | 'parentCapabilityStatus' | 'adapterState' | 'proofState'
  >
>;

function childAndroidPermissionCapabilityReadModelIsHonest(
  readModel: ChildAndroidPermissionReadModelCandidate
): boolean {
  return (
    readModel.packageId === 'ca.ocentra.parent.agent' &&
    readModel.nativeBridgeClass === 'ca.ocentra.parent.agent.ChildAndroidPermissionCapabilityProof' &&
    permissionProtocolBridgeProofIsHonest(readModel.protocolBridgeProof) &&
    permissionProofsAreHonest(readModel.permissionProofs) &&
    adapterProofsAreHonest(readModel.adapterProofs) &&
    packageLifecycleProofsAreHonest(readModel.packageLifecycleProofs)
  );
}

function permissionProtocolBridgeProofIsHonest(proof: ChildAndroidPermissionProtocolBridgeProof): boolean {
  return (
    proof.packageId === 'ca.ocentra.parent.agent' &&
    proof.nativeBridgeClass === 'ca.ocentra.parent.agent.ChildAndroidPermissionCapabilityProof' &&
    proof.bridgeState === 'package-local-scaffold' &&
    proof.externalTransportState === 'not-implemented' &&
    proof.runtimeOwner === 'android-native-wrapper' &&
    requiredValuesArePresent(proof.commands, RequiredCommands) &&
    requiredValuesArePresent(proof.events, RequiredEvents)
  );
}

function permissionProofsAreHonest(proofs: ReadonlyArray<ChildAndroidPermissionProof>): boolean {
  const byPermission = new Map(proofs.map((entry) => [entry.permission, entry] as const));
  return (
    byPermission.size === proofs.length &&
    RequiredPermissions.every((permission) => permissionProofIsHonest(byPermission.get(permission), permission))
  );
}

function permissionProofIsHonest(
  proof: ChildAndroidPermissionProof | undefined,
  permission: ChildAndroidPermissionName
): boolean {
  const expected = PermissionExpectations[permission];
  return Boolean(
    proof &&
    proof.permission === permission &&
    proof.declarationState === expected.declarationState &&
    proof.runtimeGrantState === expected.runtimeGrantState &&
    proof.proofState === expected.proofState &&
    proof.runtimeOwner === expected.runtimeOwner
  );
}

function adapterProofsAreHonest(proofs: ReadonlyArray<ChildAndroidPermissionAdapterProof>): boolean {
  const bySurface = new Map(proofs.map((entry) => [entry.surface, entry] as const));
  return (
    bySurface.size === proofs.length &&
    RequiredAdapterSurfaces.every((surface) => adapterSurfaceProofIsHonest(bySurface.get(surface), surface))
  );
}

function adapterSurfaceProofIsHonest(
  proof: ChildAndroidPermissionAdapterProof | undefined,
  surface: ChildAndroidPermissionSurfaceName
): boolean {
  const expected = AdapterExpectations[surface];
  return Boolean(
    proof &&
    proof.surface === surface &&
    proof.parentCapability === expected.parentCapability &&
    proof.parentCapabilityStatus === expected.parentCapabilityStatus &&
    proof.adapterState === expected.adapterState &&
    proof.proofState === expected.proofState
  );
}

function packageLifecycleProofsAreHonest(proofs: ReadonlyArray<ChildAndroidPermissionPackageLifecycleProof>): boolean {
  const byPhase = new Map(proofs.map((entry) => [entry.phase, entry] as const));
  return (
    byPhase.size === proofs.length &&
    RequiredLifecyclePhases.every((phase) => packageLifecyclePhaseIsHonest(byPhase.get(phase), phase))
  );
}

function packageLifecyclePhaseIsHonest(
  proof: ChildAndroidPermissionPackageLifecycleProof | undefined,
  phase: ChildAndroidPermissionPackagePhase
): boolean {
  if (!proof || proof.phase !== phase) {
    return false;
  }

  if (
    phase === 'debug-apk-build' ||
    phase === 'checksum' ||
    phase === 'launcher-activity' ||
    phase === 'foreground-service-registration' ||
    phase === 'notification-permission-declared' ||
    phase === 'app-private-storage-path'
  ) {
    return proof.proofState === 'ci-mechanical-proof';
  }

  return proof.proofState === 'manual-required';
}

function requiredValuesArePresent<Value extends string>(
  values: ReadonlyArray<Value>,
  required: ReadonlyArray<Value>
): boolean {
  const valueSet = new Set(values);
  return valueSet.size === values.length && required.every((value) => valueSet.has(value));
}

export type ChildAndroidPermissionSurfaceName = Infer<typeof ChildAndroidPermissionSurfaceNameSchema>;
export type ChildAndroidPermissionProofState = Infer<typeof ChildAndroidPermissionProofStateSchema>;
export type ChildAndroidPermissionRuntimeOwner = Infer<typeof ChildAndroidPermissionRuntimeOwnerSchema>;
export type ChildAndroidPermissionName = Infer<typeof ChildAndroidPermissionNameSchema>;
export type ChildAndroidPermissionDeclarationState = Infer<typeof ChildAndroidPermissionDeclarationStateSchema>;
export type ChildAndroidPermissionRuntimeGrantState = Infer<typeof ChildAndroidPermissionRuntimeGrantStateSchema>;
export type ChildAndroidPermissionAdapterState = Infer<typeof ChildAndroidPermissionAdapterStateSchema>;
export type ChildAndroidPermissionPackagePhase = Infer<typeof ChildAndroidPermissionPackagePhaseSchema>;
export type ChildAndroidPermissionProtocolCommand = Infer<typeof ChildAndroidPermissionProtocolCommandSchema>;
export type ChildAndroidPermissionProtocolEvent = Infer<typeof ChildAndroidPermissionProtocolEventSchema>;
export type ChildAndroidPermissionBridgeState = Infer<typeof ChildAndroidPermissionBridgeStateSchema>;
export type ChildAndroidPermissionProof = Infer<typeof ChildAndroidPermissionProofSchema>;
export type ChildAndroidPermissionAdapterProof = Infer<typeof ChildAndroidPermissionAdapterProofSchema>;
export type ChildAndroidPermissionPackageLifecycleProof = Infer<
  typeof ChildAndroidPermissionPackageLifecycleProofSchema
>;
export type ChildAndroidPermissionProtocolBridgeProof = Infer<typeof ChildAndroidPermissionProtocolBridgeProofSchema>;
export type ChildAndroidPermissionClaimBoundaries = Infer<typeof ChildAndroidPermissionClaimBoundariesSchema>;
export type ChildAndroidPermissionCapabilityReadModel = Infer<typeof ChildAndroidPermissionCapabilityReadModelSchema>;

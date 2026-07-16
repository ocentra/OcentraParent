/* generated from crates/schema/src/child_android_privileged_capability_proof_ts.rs */

import { type Infer, brandedNonEmptyStringSchema, Schema, withParser } from './effect';
import { ParentControlCapabilityNameSchema, ParentControlCapabilityStatusSchema } from './capabilities';
import { ParentTimestampSchema } from './family-reference-primitives';

export const ChildAndroidPrivilegedCapabilityProofSchemaVersionSchema = withParser(
  Schema.Literal('child-android-privileged-capability-proof')
);
export const ChildAndroidPrivilegedSurfaceNameSchema = withParser(
  Schema.Literal(
    'usage-stats-settings-access',
    'usage-stats-observation',
    'accessibility-service-adapter',
    'vpn-service-adapter',
    'dns-filtering-adapter',
    'device-owner-enrollment',
    'managed-profile-enrollment',
    'privileged-status-bundle',
    'physical-device-proof',
    'external-child-agent-transport'
  )
);
export const ChildAndroidPrivilegedProofStateSchema = withParser(
  Schema.Literal(
    'ci-mechanical-proof',
    'package-local-scaffold',
    'settings-grant-required',
    'manual-device-proof',
    'device-proof-required',
    'not-declared',
    'not-implemented',
    'blocked',
    'unavailable'
  )
);
export const ChildAndroidPrivilegedRuntimeOwnerSchema = withParser(
  Schema.Literal(
    'android-native-wrapper',
    'android-settings-panel',
    'android-usage-stats-manager',
    'android-accessibility-service',
    'android-vpn-service',
    'android-dns-filtering',
    'android-device-policy-manager',
    'android-managed-profile-owner',
    'manual-device-proof',
    'external-child-agent-transport'
  )
);
export const ChildAndroidPrivilegedDeclarationStateSchema = withParser(
  Schema.Literal('status-bundle-label', 'not-declared-by-design', 'not-declared', 'not-applicable')
);
export const ChildAndroidPrivilegedRuntimeGrantStateSchema = withParser(
  Schema.Literal(
    'not-applicable',
    'manual-settings-required',
    'manual-device-required',
    'unavailable',
    'blocked',
    'not-implemented'
  )
);
export const ChildAndroidPrivilegedProtocolCommandSchema = withParser(
  Schema.Literal(
    'child.android.privileged.capability.snapshot.get',
    'child.android.privileged.settings-proof.get',
    'child.android.privileged.enrollment-proof.get'
  )
);
export const ChildAndroidPrivilegedProtocolEventSchema = withParser(
  Schema.Literal(
    'child.android.privileged.capability.snapshot.reported',
    'child.android.privileged.settings-proof.reported',
    'child.android.privileged.enrollment-proof.reported'
  )
);
export const ChildAndroidPrivilegedBridgeStateSchema = withParser(
  Schema.Literal('package-local-scaffold', 'not-implemented')
);

const ChildAndroidPrivilegedPackageIdSchema = brandedNonEmptyStringSchema('ChildAndroidPrivilegedPackageId');
const ChildAndroidPrivilegedClassNameSchema = brandedNonEmptyStringSchema('ChildAndroidPrivilegedClassName');
const ChildAndroidPrivilegedRequirementSchema = brandedNonEmptyStringSchema('ChildAndroidPrivilegedRequirement');
const ChildAndroidPrivilegedBoundarySchema = brandedNonEmptyStringSchema('ChildAndroidPrivilegedBoundary');

export const ChildAndroidPrivilegedSurfaceProofSchema = withParser(
  Schema.Struct({
    surface: ChildAndroidPrivilegedSurfaceNameSchema,
    parentCapability: ParentControlCapabilityNameSchema,
    parentCapabilityStatus: ParentControlCapabilityStatusSchema,
    declarationState: ChildAndroidPrivilegedDeclarationStateSchema,
    runtimeGrantState: ChildAndroidPrivilegedRuntimeGrantStateSchema,
    proofState: ChildAndroidPrivilegedProofStateSchema,
    runtimeOwner: ChildAndroidPrivilegedRuntimeOwnerSchema,
    proofRequirement: ChildAndroidPrivilegedRequirementSchema,
    claimBoundary: ChildAndroidPrivilegedBoundarySchema,
  })
);

export const ChildAndroidPrivilegedProtocolBridgeProofSchema = withParser(
  Schema.Struct({
    packageId: ChildAndroidPrivilegedPackageIdSchema,
    nativeBridgeClass: ChildAndroidPrivilegedClassNameSchema,
    bridgeState: ChildAndroidPrivilegedBridgeStateSchema,
    externalTransportState: ChildAndroidPrivilegedBridgeStateSchema,
    commands: Schema.Array(ChildAndroidPrivilegedProtocolCommandSchema),
    events: Schema.Array(ChildAndroidPrivilegedProtocolEventSchema),
    runtimeOwner: ChildAndroidPrivilegedRuntimeOwnerSchema,
    proofRequirement: ChildAndroidPrivilegedRequirementSchema,
    claimBoundary: ChildAndroidPrivilegedBoundarySchema,
  })
);

export const ChildAndroidPrivilegedClaimBoundariesSchema = withParser(
  Schema.Struct({
    usageStats: ChildAndroidPrivilegedBoundarySchema,
    accessibility: ChildAndroidPrivilegedBoundarySchema,
    vpnDns: ChildAndroidPrivilegedBoundarySchema,
    deviceOwner: ChildAndroidPrivilegedBoundarySchema,
    managedProfile: ChildAndroidPrivilegedBoundarySchema,
    statusBundle: ChildAndroidPrivilegedBoundarySchema,
    physicalDevice: ChildAndroidPrivilegedBoundarySchema,
    externalTransport: ChildAndroidPrivilegedBoundarySchema,
  })
);

const ChildAndroidPrivilegedReadModelBaseSchema = Schema.Struct({
  schemaVersion: ChildAndroidPrivilegedCapabilityProofSchemaVersionSchema,
  packageId: ChildAndroidPrivilegedPackageIdSchema,
  nativeBridgeClass: ChildAndroidPrivilegedClassNameSchema,
  protocolBridgeProof: ChildAndroidPrivilegedProtocolBridgeProofSchema,
  privilegedSurfaceProofs: Schema.Array(ChildAndroidPrivilegedSurfaceProofSchema),
  claimBoundaries: ChildAndroidPrivilegedClaimBoundariesSchema,
  updatedAt: ParentTimestampSchema,
});

type ChildAndroidPrivilegedReadModelCandidate = Infer<typeof ChildAndroidPrivilegedReadModelBaseSchema>;

export const ChildAndroidPrivilegedCapabilityReadModelSchema = withParser(
  ChildAndroidPrivilegedReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        childAndroidPrivilegedCapabilityReadModelIsHonest(readModel) ||
        'Expected Child Android privileged capability proof to keep UsageStats as settings/manual-device proof, Accessibility and VPN/DNS as not declared or not implemented, device-owner and managed-profile as blocked without enrollment, physical-device behavior as device-proof-required, and external child-agent transport as not implemented'
    )
  )
);

const RequiredSurfaces = [
  'usage-stats-settings-access',
  'usage-stats-observation',
  'accessibility-service-adapter',
  'vpn-service-adapter',
  'dns-filtering-adapter',
  'device-owner-enrollment',
  'managed-profile-enrollment',
  'privileged-status-bundle',
  'physical-device-proof',
  'external-child-agent-transport',
] as const satisfies ReadonlyArray<ChildAndroidPrivilegedSurfaceName>;

const RequiredCommands = [
  'child.android.privileged.capability.snapshot.get',
  'child.android.privileged.settings-proof.get',
  'child.android.privileged.enrollment-proof.get',
] as const satisfies ReadonlyArray<ChildAndroidPrivilegedProtocolCommand>;

const RequiredEvents = [
  'child.android.privileged.capability.snapshot.reported',
  'child.android.privileged.settings-proof.reported',
  'child.android.privileged.enrollment-proof.reported',
] as const satisfies ReadonlyArray<ChildAndroidPrivilegedProtocolEvent>;

const SurfaceExpectations = {
  'usage-stats-settings-access': {
    parentCapability: 'usage-stats',
    parentCapabilityStatus: 'manual-required',
    declarationState: 'not-declared-by-design',
    runtimeGrantState: 'manual-settings-required',
    proofState: 'settings-grant-required',
    runtimeOwner: 'android-settings-panel',
  },
  'usage-stats-observation': {
    parentCapability: 'usage-stats',
    parentCapabilityStatus: 'manual-required',
    declarationState: 'not-applicable',
    runtimeGrantState: 'manual-device-required',
    proofState: 'manual-device-proof',
    runtimeOwner: 'android-usage-stats-manager',
  },
  'accessibility-service-adapter': {
    parentCapability: 'accessibility-service',
    parentCapabilityStatus: 'not-implemented',
    declarationState: 'not-declared',
    runtimeGrantState: 'unavailable',
    proofState: 'not-implemented',
    runtimeOwner: 'android-accessibility-service',
  },
  'vpn-service-adapter': {
    parentCapability: 'vpn-dns-filtering',
    parentCapabilityStatus: 'not-implemented',
    declarationState: 'not-declared',
    runtimeGrantState: 'unavailable',
    proofState: 'not-implemented',
    runtimeOwner: 'android-vpn-service',
  },
  'dns-filtering-adapter': {
    parentCapability: 'vpn-dns-filtering',
    parentCapabilityStatus: 'not-implemented',
    declarationState: 'not-declared',
    runtimeGrantState: 'not-implemented',
    proofState: 'not-implemented',
    runtimeOwner: 'android-dns-filtering',
  },
  'device-owner-enrollment': {
    parentCapability: 'device-owner-policy',
    parentCapabilityStatus: 'manual-required',
    declarationState: 'not-declared',
    runtimeGrantState: 'blocked',
    proofState: 'blocked',
    runtimeOwner: 'android-device-policy-manager',
  },
  'managed-profile-enrollment': {
    parentCapability: 'managed-profile',
    parentCapabilityStatus: 'manual-required',
    declarationState: 'not-declared',
    runtimeGrantState: 'blocked',
    proofState: 'blocked',
    runtimeOwner: 'android-managed-profile-owner',
  },
  'privileged-status-bundle': {
    parentCapability: 'typed-protocol-bridge',
    parentCapabilityStatus: 'scaffold',
    declarationState: 'status-bundle-label',
    runtimeGrantState: 'not-applicable',
    proofState: 'package-local-scaffold',
    runtimeOwner: 'android-native-wrapper',
  },
  'physical-device-proof': {
    parentCapability: 'package-lifecycle',
    parentCapabilityStatus: 'manual-required',
    declarationState: 'not-applicable',
    runtimeGrantState: 'manual-device-required',
    proofState: 'device-proof-required',
    runtimeOwner: 'manual-device-proof',
  },
  'external-child-agent-transport': {
    parentCapability: 'typed-protocol-bridge',
    parentCapabilityStatus: 'not-implemented',
    declarationState: 'not-applicable',
    runtimeGrantState: 'not-implemented',
    proofState: 'not-implemented',
    runtimeOwner: 'external-child-agent-transport',
  },
} as const satisfies Record<
  ChildAndroidPrivilegedSurfaceName,
  Pick<
    ChildAndroidPrivilegedSurfaceProof,
    | 'parentCapability'
    | 'parentCapabilityStatus'
    | 'declarationState'
    | 'runtimeGrantState'
    | 'proofState'
    | 'runtimeOwner'
  >
>;

function childAndroidPrivilegedCapabilityReadModelIsHonest(
  readModel: ChildAndroidPrivilegedReadModelCandidate
): boolean {
  return (
    readModel.packageId === 'ca.ocentra.parent.agent' &&
    readModel.nativeBridgeClass === 'ca.ocentra.parent.agent.ChildAndroidPrivilegedCapabilityProof' &&
    privilegedProtocolBridgeProofIsHonest(readModel.protocolBridgeProof) &&
    privilegedSurfaceProofsAreHonest(readModel.privilegedSurfaceProofs)
  );
}

function privilegedProtocolBridgeProofIsHonest(proof: ChildAndroidPrivilegedProtocolBridgeProof): boolean {
  return (
    proof.packageId === 'ca.ocentra.parent.agent' &&
    proof.nativeBridgeClass === 'ca.ocentra.parent.agent.ChildAndroidPrivilegedCapabilityProof' &&
    proof.bridgeState === 'package-local-scaffold' &&
    proof.externalTransportState === 'not-implemented' &&
    proof.runtimeOwner === 'android-native-wrapper' &&
    requiredValuesArePresent(proof.commands, RequiredCommands) &&
    requiredValuesArePresent(proof.events, RequiredEvents)
  );
}

function privilegedSurfaceProofsAreHonest(proofs: ReadonlyArray<ChildAndroidPrivilegedSurfaceProof>): boolean {
  const bySurface = new Map(proofs.map((entry) => [entry.surface, entry] as const));
  return (
    bySurface.size === proofs.length &&
    RequiredSurfaces.every((surface) => privilegedSurfaceProofIsHonest(bySurface.get(surface), surface))
  );
}

function privilegedSurfaceProofIsHonest(
  proof: ChildAndroidPrivilegedSurfaceProof | undefined,
  surface: ChildAndroidPrivilegedSurfaceName
): boolean {
  const expected = SurfaceExpectations[surface];
  return Boolean(
    proof &&
    proof.surface === surface &&
    proof.parentCapability === expected.parentCapability &&
    proof.parentCapabilityStatus === expected.parentCapabilityStatus &&
    proof.declarationState === expected.declarationState &&
    proof.runtimeGrantState === expected.runtimeGrantState &&
    proof.proofState === expected.proofState &&
    proof.runtimeOwner === expected.runtimeOwner
  );
}

function requiredValuesArePresent<Value extends string>(
  values: ReadonlyArray<Value>,
  required: ReadonlyArray<Value>
): boolean {
  const valueSet = new Set(values);
  return valueSet.size === values.length && required.every((value) => valueSet.has(value));
}

export type ChildAndroidPrivilegedSurfaceName = Infer<typeof ChildAndroidPrivilegedSurfaceNameSchema>;
export type ChildAndroidPrivilegedProofState = Infer<typeof ChildAndroidPrivilegedProofStateSchema>;
export type ChildAndroidPrivilegedRuntimeOwner = Infer<typeof ChildAndroidPrivilegedRuntimeOwnerSchema>;
export type ChildAndroidPrivilegedDeclarationState = Infer<typeof ChildAndroidPrivilegedDeclarationStateSchema>;
export type ChildAndroidPrivilegedRuntimeGrantState = Infer<typeof ChildAndroidPrivilegedRuntimeGrantStateSchema>;
export type ChildAndroidPrivilegedProtocolCommand = Infer<typeof ChildAndroidPrivilegedProtocolCommandSchema>;
export type ChildAndroidPrivilegedProtocolEvent = Infer<typeof ChildAndroidPrivilegedProtocolEventSchema>;
export type ChildAndroidPrivilegedBridgeState = Infer<typeof ChildAndroidPrivilegedBridgeStateSchema>;
export type ChildAndroidPrivilegedSurfaceProof = Infer<typeof ChildAndroidPrivilegedSurfaceProofSchema>;
export type ChildAndroidPrivilegedProtocolBridgeProof = Infer<typeof ChildAndroidPrivilegedProtocolBridgeProofSchema>;
export type ChildAndroidPrivilegedClaimBoundaries = Infer<typeof ChildAndroidPrivilegedClaimBoundariesSchema>;
export type ChildAndroidPrivilegedCapabilityReadModel = Infer<typeof ChildAndroidPrivilegedCapabilityReadModelSchema>;

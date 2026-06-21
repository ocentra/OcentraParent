import { type Infer, brandedNonEmptyStringSchema, Schema, withParser } from './effect';
import { ParentControlCapabilityNameSchema, ParentControlCapabilityStatusSchema } from './capabilities';
import { ParentTimestampSchema } from './family-reference-primitives';

export const ChildIosEntitlementCapabilityProofSchemaVersionSchema = withParser(
  Schema.Literal('child-ios-entitlement-capability-proof')
);
export const ChildIosEntitlementSurfaceNameSchema = withParser(
  Schema.Literal(
    'simulator-app-target',
    'bundle-identifier',
    'status-surface',
    'family-controls-entitlement',
    'device-activity-framework',
    'screen-time-api',
    'network-extension',
    'notifications-permission',
    'background-execution',
    'signing-entitlements',
    'testflight-distribution',
    'physical-device-proof',
    'app-store-distribution'
  )
);
export const ChildIosEntitlementProofStateSchema = withParser(
  Schema.Literal(
    'ci-mechanical-proof',
    'simulator-scaffold',
    'manual-required',
    'entitlement-required',
    'signing-required',
    'device-proof-required',
    'not-declared',
    'not-implemented',
    'planned'
  )
);
export const ChildIosEntitlementRuntimeOwnerSchema = withParser(
  Schema.Literal(
    'ios-xcode-project',
    'ios-info-plist',
    'ios-swift-scaffold',
    'ios-simulator-build-script',
    'apple-entitlement',
    'apple-device-framework',
    'apple-network-extension',
    'apple-notification-permission',
    'apple-background-mode',
    'apple-signing',
    'apple-testflight',
    'apple-device-proof',
    'app-store-connect'
  )
);
export const ChildIosEntitlementDeclarationStateSchema = withParser(
  Schema.Literal('declared-in-project', 'declared-in-plist', 'scaffold-status-label', 'not-declared', 'not-applicable')
);
export const ChildIosEntitlementPackagePhaseSchema = withParser(
  Schema.Literal(
    'xcode-project-target',
    'bundle-identifier',
    'simulator-build-script',
    'status-view',
    'info-plist',
    'simulator-build',
    'device-install',
    'testflight-install',
    'signing-profile',
    'entitlement-review'
  )
);
export const ChildIosEntitlementProtocolCommandSchema = withParser(
  Schema.Literal(
    'child.ios.entitlement.capability.snapshot.get',
    'child.ios.entitlement.package.proof.get',
    'child.ios.entitlement.manual-proof.get'
  )
);
export const ChildIosEntitlementProtocolEventSchema = withParser(
  Schema.Literal(
    'child.ios.entitlement.capability.snapshot.reported',
    'child.ios.entitlement.package.proof.reported',
    'child.ios.entitlement.manual-proof.reported'
  )
);
export const ChildIosEntitlementBridgeStateSchema = withParser(Schema.Literal('simulator-scaffold', 'not-implemented'));

const ChildIosEntitlementBundleIdSchema = brandedNonEmptyStringSchema('ChildIosEntitlementBundleId');
const ChildIosEntitlementClassNameSchema = brandedNonEmptyStringSchema('ChildIosEntitlementClassName');
const ChildIosEntitlementRequirementSchema = brandedNonEmptyStringSchema('ChildIosEntitlementRequirement');
const ChildIosEntitlementBoundarySchema = brandedNonEmptyStringSchema('ChildIosEntitlementBoundary');

export const ChildIosEntitlementSurfaceProofSchema = withParser(
  Schema.Struct({
    surface: ChildIosEntitlementSurfaceNameSchema,
    parentCapability: ParentControlCapabilityNameSchema,
    parentCapabilityStatus: ParentControlCapabilityStatusSchema,
    declarationState: ChildIosEntitlementDeclarationStateSchema,
    proofState: ChildIosEntitlementProofStateSchema,
    runtimeOwner: ChildIosEntitlementRuntimeOwnerSchema,
    proofRequirement: ChildIosEntitlementRequirementSchema,
    claimBoundary: ChildIosEntitlementBoundarySchema,
  })
);

export const ChildIosEntitlementPackageLifecycleProofSchema = withParser(
  Schema.Struct({
    phase: ChildIosEntitlementPackagePhaseSchema,
    proofState: ChildIosEntitlementProofStateSchema,
    runtimeOwner: ChildIosEntitlementRuntimeOwnerSchema,
    proofRequirement: ChildIosEntitlementRequirementSchema,
    claimBoundary: ChildIosEntitlementBoundarySchema,
  })
);

export const ChildIosEntitlementProtocolBridgeProofSchema = withParser(
  Schema.Struct({
    bundleId: ChildIosEntitlementBundleIdSchema,
    statusSurfaceClass: ChildIosEntitlementClassNameSchema,
    bridgeState: ChildIosEntitlementBridgeStateSchema,
    externalTransportState: ChildIosEntitlementBridgeStateSchema,
    commands: Schema.Array(ChildIosEntitlementProtocolCommandSchema),
    events: Schema.Array(ChildIosEntitlementProtocolEventSchema),
    runtimeOwner: ChildIosEntitlementRuntimeOwnerSchema,
    proofRequirement: ChildIosEntitlementRequirementSchema,
    claimBoundary: ChildIosEntitlementBoundarySchema,
  })
);

export const ChildIosEntitlementClaimBoundariesSchema = withParser(
  Schema.Struct({
    simulatorPackage: ChildIosEntitlementBoundarySchema,
    familyControls: ChildIosEntitlementBoundarySchema,
    deviceActivity: ChildIosEntitlementBoundarySchema,
    screenTime: ChildIosEntitlementBoundarySchema,
    networkExtension: ChildIosEntitlementBoundarySchema,
    notifications: ChildIosEntitlementBoundarySchema,
    backgroundExecution: ChildIosEntitlementBoundarySchema,
    signingEntitlements: ChildIosEntitlementBoundarySchema,
    testflight: ChildIosEntitlementBoundarySchema,
    deviceProof: ChildIosEntitlementBoundarySchema,
    externalTransport: ChildIosEntitlementBoundarySchema,
  })
);

const ChildIosEntitlementReadModelBaseSchema = Schema.Struct({
  schemaVersion: ChildIosEntitlementCapabilityProofSchemaVersionSchema,
  bundleId: ChildIosEntitlementBundleIdSchema,
  statusSurfaceClass: ChildIosEntitlementClassNameSchema,
  protocolBridgeProof: ChildIosEntitlementProtocolBridgeProofSchema,
  surfaceProofs: Schema.Array(ChildIosEntitlementSurfaceProofSchema),
  packageLifecycleProofs: Schema.Array(ChildIosEntitlementPackageLifecycleProofSchema),
  claimBoundaries: ChildIosEntitlementClaimBoundariesSchema,
  updatedAt: ParentTimestampSchema,
});

type ChildIosEntitlementReadModelCandidate = Infer<typeof ChildIosEntitlementReadModelBaseSchema>;

export const ChildIosEntitlementCapabilityReadModelSchema = withParser(
  ChildIosEntitlementReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        childIosEntitlementCapabilityReadModelIsHonest(readModel) ||
        'Expected Child iOS entitlement/package proof to keep simulator app target, bundle id, status surface, and simulator build script as scaffold/CI proof while Family Controls, DeviceActivity, Screen Time, Network Extension, notifications, background execution, signing, TestFlight, App Store, and device behavior remain manual-required, entitlement-required, signing-required, device-proof-required, planned, or not implemented without Apple artifacts'
    )
  )
);

const RequiredSurfaces = [
  'simulator-app-target',
  'bundle-identifier',
  'status-surface',
  'family-controls-entitlement',
  'device-activity-framework',
  'screen-time-api',
  'network-extension',
  'notifications-permission',
  'background-execution',
  'signing-entitlements',
  'testflight-distribution',
  'physical-device-proof',
  'app-store-distribution',
] as const satisfies ReadonlyArray<ChildIosEntitlementSurfaceName>;

const RequiredLifecyclePhases = [
  'xcode-project-target',
  'bundle-identifier',
  'simulator-build-script',
  'status-view',
  'info-plist',
  'simulator-build',
  'device-install',
  'testflight-install',
  'signing-profile',
  'entitlement-review',
] as const satisfies ReadonlyArray<ChildIosEntitlementPackagePhase>;

const RequiredCommands = [
  'child.ios.entitlement.capability.snapshot.get',
  'child.ios.entitlement.package.proof.get',
  'child.ios.entitlement.manual-proof.get',
] as const satisfies ReadonlyArray<ChildIosEntitlementProtocolCommand>;

const RequiredEvents = [
  'child.ios.entitlement.capability.snapshot.reported',
  'child.ios.entitlement.package.proof.reported',
  'child.ios.entitlement.manual-proof.reported',
] as const satisfies ReadonlyArray<ChildIosEntitlementProtocolEvent>;

const SurfaceExpectations = {
  'simulator-app-target': {
    parentCapability: 'package-lifecycle',
    parentCapabilityStatus: 'manual-required',
    declarationState: 'declared-in-project',
    proofState: 'ci-mechanical-proof',
    runtimeOwner: 'ios-xcode-project',
  },
  'bundle-identifier': {
    parentCapability: 'package-lifecycle',
    parentCapabilityStatus: 'manual-required',
    declarationState: 'declared-in-project',
    proofState: 'ci-mechanical-proof',
    runtimeOwner: 'ios-xcode-project',
  },
  'status-surface': {
    parentCapability: 'typed-protocol-bridge',
    parentCapabilityStatus: 'scaffold',
    declarationState: 'scaffold-status-label',
    proofState: 'simulator-scaffold',
    runtimeOwner: 'ios-swift-scaffold',
  },
  'family-controls-entitlement': {
    parentCapability: 'family-controls-entitlement',
    parentCapabilityStatus: 'manual-required',
    declarationState: 'not-declared',
    proofState: 'entitlement-required',
    runtimeOwner: 'apple-entitlement',
  },
  'device-activity-framework': {
    parentCapability: 'device-activity',
    parentCapabilityStatus: 'manual-required',
    declarationState: 'not-declared',
    proofState: 'entitlement-required',
    runtimeOwner: 'apple-device-framework',
  },
  'screen-time-api': {
    parentCapability: 'screen-time-api',
    parentCapabilityStatus: 'manual-required',
    declarationState: 'not-declared',
    proofState: 'entitlement-required',
    runtimeOwner: 'apple-device-framework',
  },
  'network-extension': {
    parentCapability: 'network-extension',
    parentCapabilityStatus: 'manual-required',
    declarationState: 'not-declared',
    proofState: 'entitlement-required',
    runtimeOwner: 'apple-network-extension',
  },
  'notifications-permission': {
    parentCapability: 'notifications',
    parentCapabilityStatus: 'manual-required',
    declarationState: 'not-declared',
    proofState: 'manual-required',
    runtimeOwner: 'apple-notification-permission',
  },
  'background-execution': {
    parentCapability: 'background-execution',
    parentCapabilityStatus: 'manual-required',
    declarationState: 'not-declared',
    proofState: 'manual-required',
    runtimeOwner: 'apple-background-mode',
  },
  'signing-entitlements': {
    parentCapability: 'signing-entitlements',
    parentCapabilityStatus: 'manual-required',
    declarationState: 'not-applicable',
    proofState: 'signing-required',
    runtimeOwner: 'apple-signing',
  },
  'testflight-distribution': {
    parentCapability: 'testflight-distribution',
    parentCapabilityStatus: 'manual-required',
    declarationState: 'not-applicable',
    proofState: 'device-proof-required',
    runtimeOwner: 'apple-testflight',
  },
  'physical-device-proof': {
    parentCapability: 'package-lifecycle',
    parentCapabilityStatus: 'manual-required',
    declarationState: 'not-applicable',
    proofState: 'device-proof-required',
    runtimeOwner: 'apple-device-proof',
  },
  'app-store-distribution': {
    parentCapability: 'store-distribution',
    parentCapabilityStatus: 'planned',
    declarationState: 'not-applicable',
    proofState: 'planned',
    runtimeOwner: 'app-store-connect',
  },
} as const satisfies Record<
  ChildIosEntitlementSurfaceName,
  Pick<
    ChildIosEntitlementSurfaceProof,
    'parentCapability' | 'parentCapabilityStatus' | 'declarationState' | 'proofState' | 'runtimeOwner'
  >
>;

function childIosEntitlementCapabilityReadModelIsHonest(readModel: ChildIosEntitlementReadModelCandidate): boolean {
  return (
    readModel.bundleId === 'ca.ocentra.parent.agent' &&
    readModel.statusSurfaceClass === 'AgentStatusViewController' &&
    entitlementProtocolBridgeProofIsHonest(readModel.protocolBridgeProof) &&
    entitlementSurfaceProofsAreHonest(readModel.surfaceProofs) &&
    entitlementPackageLifecycleProofsAreHonest(readModel.packageLifecycleProofs)
  );
}

function entitlementProtocolBridgeProofIsHonest(proof: ChildIosEntitlementProtocolBridgeProof): boolean {
  return (
    proof.bundleId === 'ca.ocentra.parent.agent' &&
    proof.statusSurfaceClass === 'AgentStatusViewController' &&
    proof.bridgeState === 'simulator-scaffold' &&
    proof.externalTransportState === 'not-implemented' &&
    proof.runtimeOwner === 'ios-swift-scaffold' &&
    requiredValuesArePresent(proof.commands, RequiredCommands) &&
    requiredValuesArePresent(proof.events, RequiredEvents)
  );
}

function entitlementSurfaceProofsAreHonest(proofs: ReadonlyArray<ChildIosEntitlementSurfaceProof>): boolean {
  const bySurface = new Map(proofs.map((entry) => [entry.surface, entry] as const));
  return (
    bySurface.size === proofs.length &&
    RequiredSurfaces.every((surface) => entitlementSurfaceProofIsHonest(bySurface.get(surface), surface))
  );
}

function entitlementSurfaceProofIsHonest(
  proof: ChildIosEntitlementSurfaceProof | undefined,
  surface: ChildIosEntitlementSurfaceName
): boolean {
  const expected = SurfaceExpectations[surface];
  return Boolean(
    proof &&
    proof.surface === surface &&
    proof.parentCapability === expected.parentCapability &&
    proof.parentCapabilityStatus === expected.parentCapabilityStatus &&
    proof.declarationState === expected.declarationState &&
    proof.proofState === expected.proofState &&
    proof.runtimeOwner === expected.runtimeOwner
  );
}

function entitlementPackageLifecycleProofsAreHonest(
  proofs: ReadonlyArray<ChildIosEntitlementPackageLifecycleProof>
): boolean {
  const byPhase = new Map(proofs.map((entry) => [entry.phase, entry] as const));
  return (
    byPhase.size === proofs.length &&
    RequiredLifecyclePhases.every((phase) => entitlementPackageLifecyclePhaseIsHonest(byPhase.get(phase), phase))
  );
}

function entitlementPackageLifecyclePhaseIsHonest(
  proof: ChildIosEntitlementPackageLifecycleProof | undefined,
  phase: ChildIosEntitlementPackagePhase
): boolean {
  if (!proof || proof.phase !== phase) {
    return false;
  }

  if (
    phase === 'xcode-project-target' ||
    phase === 'bundle-identifier' ||
    phase === 'simulator-build-script' ||
    phase === 'status-view' ||
    phase === 'info-plist'
  ) {
    return proof.proofState === 'ci-mechanical-proof' || proof.proofState === 'simulator-scaffold';
  }

  return (
    proof.proofState === 'manual-required' ||
    proof.proofState === 'signing-required' ||
    proof.proofState === 'device-proof-required' ||
    proof.proofState === 'entitlement-required'
  );
}

function requiredValuesArePresent<Value extends string>(
  values: ReadonlyArray<Value>,
  required: ReadonlyArray<Value>
): boolean {
  const valueSet = new Set(values);
  return valueSet.size === values.length && required.every((value) => valueSet.has(value));
}

export type ChildIosEntitlementSurfaceName = Infer<typeof ChildIosEntitlementSurfaceNameSchema>;
export type ChildIosEntitlementProofState = Infer<typeof ChildIosEntitlementProofStateSchema>;
export type ChildIosEntitlementRuntimeOwner = Infer<typeof ChildIosEntitlementRuntimeOwnerSchema>;
export type ChildIosEntitlementDeclarationState = Infer<typeof ChildIosEntitlementDeclarationStateSchema>;
export type ChildIosEntitlementPackagePhase = Infer<typeof ChildIosEntitlementPackagePhaseSchema>;
export type ChildIosEntitlementProtocolCommand = Infer<typeof ChildIosEntitlementProtocolCommandSchema>;
export type ChildIosEntitlementProtocolEvent = Infer<typeof ChildIosEntitlementProtocolEventSchema>;
export type ChildIosEntitlementBridgeState = Infer<typeof ChildIosEntitlementBridgeStateSchema>;
export type ChildIosEntitlementSurfaceProof = Infer<typeof ChildIosEntitlementSurfaceProofSchema>;
export type ChildIosEntitlementPackageLifecycleProof = Infer<typeof ChildIosEntitlementPackageLifecycleProofSchema>;
export type ChildIosEntitlementProtocolBridgeProof = Infer<typeof ChildIosEntitlementProtocolBridgeProofSchema>;
export type ChildIosEntitlementClaimBoundaries = Infer<typeof ChildIosEntitlementClaimBoundariesSchema>;
export type ChildIosEntitlementCapabilityReadModel = Infer<typeof ChildIosEntitlementCapabilityReadModelSchema>;

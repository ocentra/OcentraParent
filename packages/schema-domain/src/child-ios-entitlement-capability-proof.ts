/* thin adapter over Rust-generated child iOS entitlement capability contracts */

import { type Infer, Schema, brandedNonEmptyStringSchema, withParser } from './effect';
import { ParentTimestampSchema } from './family-reference-primitives';
import {
  ChildIosEntitlementCapabilityContractRuntime,
  GeneratedChildIosEntitlementBridgeStates,
  GeneratedChildIosEntitlementCapabilityReadModel,
  GeneratedChildIosEntitlementDeclarationStates,
  GeneratedChildIosEntitlementPackagePhases,
  GeneratedChildIosEntitlementParentCapabilities,
  GeneratedChildIosEntitlementParentCapabilityStatuses,
  GeneratedChildIosEntitlementProofStates,
  GeneratedChildIosEntitlementProtocolCommands,
  GeneratedChildIosEntitlementProtocolEvents,
  GeneratedChildIosEntitlementRuntimeOwners,
  GeneratedChildIosEntitlementSurfaceNames,
  type GeneratedChildIosEntitlementPackagePhase,
  type GeneratedChildIosEntitlementProtocolCommand,
  type GeneratedChildIosEntitlementProtocolEvent,
  type GeneratedChildIosEntitlementSurfaceName,
} from './generated/child-ios-entitlement-capability-proof-contracts';

export const ChildIosEntitlementCapabilityProofSchemaVersionSchema = withParser(
  Schema.Literal(ChildIosEntitlementCapabilityContractRuntime.SchemaVersion)
);
export const ChildIosEntitlementSurfaceNameSchema = withParser(
  Schema.Literal(...GeneratedChildIosEntitlementSurfaceNames)
);
export const ChildIosEntitlementProofStateSchema = withParser(
  Schema.Literal(...GeneratedChildIosEntitlementProofStates)
);
export const ChildIosEntitlementRuntimeOwnerSchema = withParser(
  Schema.Literal(...GeneratedChildIosEntitlementRuntimeOwners)
);
export const ChildIosEntitlementDeclarationStateSchema = withParser(
  Schema.Literal(...GeneratedChildIosEntitlementDeclarationStates)
);
export const ChildIosEntitlementPackagePhaseSchema = withParser(
  Schema.Literal(...GeneratedChildIosEntitlementPackagePhases)
);
export const ChildIosEntitlementProtocolCommandSchema = withParser(
  Schema.Literal(...GeneratedChildIosEntitlementProtocolCommands)
);
export const ChildIosEntitlementProtocolEventSchema = withParser(
  Schema.Literal(...GeneratedChildIosEntitlementProtocolEvents)
);
export const ChildIosEntitlementBridgeStateSchema = withParser(
  Schema.Literal(...GeneratedChildIosEntitlementBridgeStates)
);

const ChildIosEntitlementParentCapabilitySchema = withParser(
  Schema.Literal(...GeneratedChildIosEntitlementParentCapabilities)
);
const ChildIosEntitlementParentCapabilityStatusSchema = withParser(
  Schema.Literal(...GeneratedChildIosEntitlementParentCapabilityStatuses)
);
const ChildIosEntitlementBundleIdSchema = brandedNonEmptyStringSchema('ChildIosEntitlementBundleId');
const ChildIosEntitlementClassNameSchema = brandedNonEmptyStringSchema('ChildIosEntitlementClassName');
const ChildIosEntitlementRequirementSchema = brandedNonEmptyStringSchema('ChildIosEntitlementRequirement');
const ChildIosEntitlementBoundarySchema = brandedNonEmptyStringSchema('ChildIosEntitlementBoundary');

export type ChildIosEntitlementBundleId = typeof ChildIosEntitlementBundleIdSchema.Type;
export type ChildIosEntitlementClassName = typeof ChildIosEntitlementClassNameSchema.Type;
export type ChildIosEntitlementRequirement = typeof ChildIosEntitlementRequirementSchema.Type;
export type ChildIosEntitlementBoundary = typeof ChildIosEntitlementBoundarySchema.Type;
export type ChildIosEntitlementTimestamp = typeof ParentTimestampSchema.Type;

export const ChildIosEntitlementSurfaceProofSchema = withParser(
  Schema.Struct({
    surface: ChildIosEntitlementSurfaceNameSchema,
    parentCapability: ChildIosEntitlementParentCapabilitySchema,
    parentCapabilityStatus: ChildIosEntitlementParentCapabilityStatusSchema,
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
    launchAvailability: ChildIosEntitlementBoundarySchema,
    familyControls: ChildIosEntitlementBoundarySchema,
    deviceActivity: ChildIosEntitlementBoundarySchema,
    screenTime: ChildIosEntitlementBoundarySchema,
    networkExtension: ChildIosEntitlementBoundarySchema,
    notifications: ChildIosEntitlementBoundarySchema,
    backgroundExecution: ChildIosEntitlementBoundarySchema,
    recoveryBehavior: ChildIosEntitlementBoundarySchema,
    provisioningProfile: ChildIosEntitlementBoundarySchema,
    supervision: ChildIosEntitlementBoundarySchema,
    signingEntitlements: ChildIosEntitlementBoundarySchema,
    testflight: ChildIosEntitlementBoundarySchema,
    deviceProof: ChildIosEntitlementBoundarySchema,
    capabilityOnlyState: ChildIosEntitlementBoundarySchema,
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

const ExpectedProof = GeneratedChildIosEntitlementCapabilityReadModel;
const RequiredSurfaces = ExpectedProof.surfaceProofs.map((proof) => proof.surface) as ReadonlyArray<GeneratedChildIosEntitlementSurfaceName>;
const RequiredLifecyclePhases = ExpectedProof.packageLifecycleProofs.map((proof) => proof.phase) as ReadonlyArray<GeneratedChildIosEntitlementPackagePhase>;
const RequiredCommands = [...ExpectedProof.protocolBridgeProof.commands] as ReadonlyArray<GeneratedChildIosEntitlementProtocolCommand>;
const RequiredEvents = [...ExpectedProof.protocolBridgeProof.events] as ReadonlyArray<GeneratedChildIosEntitlementProtocolEvent>;
const ExpectedSurfaceProofs = new Map(ExpectedProof.surfaceProofs.map((proof) => [proof.surface, proof] as const));
const ExpectedLifecycleProofs = new Map(
  ExpectedProof.packageLifecycleProofs.map((proof) => [proof.phase, proof] as const)
);

export const ChildIosEntitlementCapabilityReadModelSchema = withParser(
  ChildIosEntitlementReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        childIosEntitlementCapabilityReadModelIsHonest(readModel) ||
        'Expected Child iOS entitlement/package proof to stay aligned to the Rust-owned capability contract and keep manual-required, entitlement-required, signing-required, device-proof-required, planned, and not-implemented boundaries explicit until Apple artifacts change them'
    )
  )
);

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

export const ChildIosEntitlementCapabilityReadModelProof =
  ChildIosEntitlementCapabilityReadModelSchema.parse(GeneratedChildIosEntitlementCapabilityReadModel);

function childIosEntitlementCapabilityReadModelIsHonest(readModel: ChildIosEntitlementReadModelCandidate): boolean {
  return (
    protocolBridgeProofIsHonest(readModel.protocolBridgeProof) &&
    surfaceProofsAreHonest(readModel.surfaceProofs) &&
    packageLifecycleProofsAreHonest(readModel.packageLifecycleProofs) &&
    claimBoundariesAreHonest(readModel.claimBoundaries)
  );
}

function protocolBridgeProofIsHonest(proof: ChildIosEntitlementProtocolBridgeProof): boolean {
  return (
    proof.bundleId === ExpectedProof.protocolBridgeProof.bundleId &&
    proof.statusSurfaceClass === ExpectedProof.protocolBridgeProof.statusSurfaceClass &&
    proof.bridgeState === ExpectedProof.protocolBridgeProof.bridgeState &&
    proof.externalTransportState === ExpectedProof.protocolBridgeProof.externalTransportState &&
    proof.runtimeOwner === ExpectedProof.protocolBridgeProof.runtimeOwner &&
    proof.proofRequirement === ExpectedProof.protocolBridgeProof.proofRequirement &&
    proof.claimBoundary === ExpectedProof.protocolBridgeProof.claimBoundary &&
    hasExactStringMembership(proof.commands, RequiredCommands) &&
    hasExactStringMembership(proof.events, RequiredEvents)
  );
}

function surfaceProofsAreHonest(proofs: ReadonlyArray<ChildIosEntitlementSurfaceProof>): boolean {
  const bySurface = new Map(proofs.map((proof) => [proof.surface, proof] as const));
  return (
    bySurface.size === RequiredSurfaces.length &&
    RequiredSurfaces.every((surface) => surfaceProofIsHonest(bySurface.get(surface), surface))
  );
}

function surfaceProofIsHonest(
  proof: ChildIosEntitlementSurfaceProof | undefined,
  surface: GeneratedChildIosEntitlementSurfaceName
): boolean {
  const expected = ExpectedSurfaceProofs.get(surface);
  return Boolean(
    proof &&
      expected &&
      proof.parentCapability === expected.parentCapability &&
      proof.parentCapabilityStatus === expected.parentCapabilityStatus &&
      proof.declarationState === expected.declarationState &&
      proof.proofState === expected.proofState &&
      proof.runtimeOwner === expected.runtimeOwner &&
      proof.proofRequirement.length > 0 &&
      proof.claimBoundary.length > 0
  );
}

function packageLifecycleProofsAreHonest(
  proofs: ReadonlyArray<ChildIosEntitlementPackageLifecycleProof>
): boolean {
  const byPhase = new Map(proofs.map((proof) => [proof.phase, proof] as const));
  return (
    byPhase.size === RequiredLifecyclePhases.length &&
    RequiredLifecyclePhases.every((phase) => packageLifecycleProofIsHonest(byPhase.get(phase), phase))
  );
}

function packageLifecycleProofIsHonest(
  proof: ChildIosEntitlementPackageLifecycleProof | undefined,
  phase: GeneratedChildIosEntitlementPackagePhase
): boolean {
  const expected = ExpectedLifecycleProofs.get(phase);
  return Boolean(
    proof &&
      expected &&
      proof.proofState === expected.proofState &&
      proof.runtimeOwner === expected.runtimeOwner &&
      proof.proofRequirement.length > 0 &&
      proof.claimBoundary.length > 0
  );
}

function claimBoundariesAreHonest(boundaries: ChildIosEntitlementClaimBoundaries): boolean {
  return Object.entries(ExpectedProof.claimBoundaries).every(
    ([key, value]) => boundaries[key as keyof ChildIosEntitlementClaimBoundaries] === value
  );
}

function hasExactStringMembership(
  values: ReadonlyArray<string>,
  expected: ReadonlyArray<string>
): boolean {
  return values.length === expected.length && expected.every((value) => values.includes(value));
}

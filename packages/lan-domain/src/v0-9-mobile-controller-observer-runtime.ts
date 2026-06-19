import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import {
  LanPairingIntentKindSchema,
  LanPairingRejectionReasonSchema,
  LanPairingResponseStateSchema,
} from '@ocentra-parent/lan-domain/lan-pairing-values';
import {
  ParentMobileCommandAuthorityStateSchema,
  ParentMobileControllerStateSchema,
  ParentMobilePackageStateSchema,
  ParentMobilePlatformSchema,
  ParentMobileServiceAvailabilityStateSchema,
  ParentMobileSigningStateSchema,
  ParentMobileStoreDistributionStateSchema,
} from './parent-mobile-runtime';
import { ParentControlCapabilityNameSchema, ParentControlCapabilityStatusSchema } from '@ocentra-parent/capability-domain/capabilities';
import { ParentDeviceIdSchema, ParentEvidenceReferenceIdSchema, ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';

export const V09MobileControllerObserverRuntimeSchemaVersionSchema = withParser(
  Schema.Literal('v0.9-mobile-controller-observer-runtime')
);
export const V09MobileControllerObserverRoleSchema = withParser(
  Schema.Literal('observer', 'controller-candidate', 'degraded-observer')
);
export const V09MobileControllerObserverRouteKindSchema = withParser(
  Schema.Literal('local-service', 'lan-service', 'cloud-relay', 'parent-cache', 'parent-owned-storage')
);
export const V09MobileControllerObserverReadinessStateSchema = withParser(
  Schema.Literal('ci-mechanical-proof', 'manual-required', 'degraded', 'unavailable', 'not-implemented')
);
export const V09MobileControllerObserverOperationSchema = withParser(
  Schema.Literal(
    'observe-status',
    'preview-policy-draft',
    'refresh-capabilities',
    'request-controller-takeover',
    'release-controller-lease',
    'submit-lan-ai-job',
    'write-policy',
    'approve-override',
    'pair-device',
    'revoke-device'
  )
);
export const V09MobileControllerObserverOperationStateSchema = withParser(
  Schema.Literal(
    'allowed-read-only',
    'rejected-observer-read-only',
    'manual-required-mobile-package',
    'proved-local-service',
    'degraded-provider',
    'unavailable'
  )
);
export const V09MobileControllerObserverRuntimeOwnerSchema = withParser(
  Schema.Literal('parent-mobile-shell', 'agent-service', 'lan-ai-provider', 'manual-proof')
);
export const V09MobileControllerObserverProofSourceSchema = withParser(
  Schema.Literal(
    'parent-mobile-shell-runtime-proof',
    'v0-9-production-lan-mobile-controller-proof',
    'v0-9-mobile-controller-discovery-runtime-proof'
  )
);
export const V09MobileControllerObserverControllerLeaseStateSchema = withParser(
  Schema.Literal('visible-read-only', 'manual-required', 'unavailable')
);

export const V09MobileControllerObserverProofLabelSchema = brandedNonEmptyStringSchema('V09MobileControllerObserverProofLabel');
export const V09MobileControllerObserverProofPathSchema = brandedNonEmptyStringSchema('V09MobileControllerObserverProofPath');
export const V09MobileControllerObserverProofCommandSchema = brandedNonEmptyStringSchema('V09MobileControllerObserverProofCommand');
export const V09MobileControllerObserverProofRequirementSchema = brandedNonEmptyStringSchema('V09MobileControllerObserverProofRequirement');
export const V09MobileControllerObserverClaimBoundarySchema = brandedNonEmptyStringSchema('V09MobileControllerObserverClaimBoundary');
const V09MobileControllerObserverRouteIdSchema = brandedNonEmptyStringSchema('V09MobileControllerObserverRouteId');
const V09MobileControllerObserverControllerLeaseIdSchema = brandedNonEmptyStringSchema('V09MobileControllerObserverControllerLeaseId');

export const V09MobileControllerObserverPackageReadinessSchema = withParser(
  Schema.Struct({
    packageState: ParentMobilePackageStateSchema,
    runtimeState: V09MobileControllerObserverReadinessStateSchema,
    signingState: ParentMobileSigningStateSchema,
    storeDistributionState: ParentMobileStoreDistributionStateSchema,
    foregroundOrBackgroundState: V09MobileControllerObserverReadinessStateSchema,
    notificationState: V09MobileControllerObserverReadinessStateSchema,
    missingCapabilityProofs: Schema.Array(ParentControlCapabilityNameSchema),
  })
);

export const V09MobileControllerObserverCapabilityStateSchema = withParser(
  Schema.Struct({
    capability: ParentControlCapabilityNameSchema,
    status: ParentControlCapabilityStatusSchema,
    proofRequirement: V09MobileControllerObserverProofRequirementSchema,
    claimBoundary: V09MobileControllerObserverClaimBoundarySchema,
  })
);

export const V09MobileControllerObserverRouteStatusSchema = withParser(
  Schema.Struct({
    routeKind: V09MobileControllerObserverRouteKindSchema,
    state: ParentMobileServiceAvailabilityStateSchema,
    selectedRouteId: Schema.Union(V09MobileControllerObserverRouteIdSchema, Schema.Null),
    proofRequirement: V09MobileControllerObserverProofRequirementSchema,
  })
);

export const V09MobileControllerObserverControllerLeaseProofSchema = withParser(
  Schema.Struct({
    leaseState: V09MobileControllerObserverControllerLeaseStateSchema,
    controllerLeaseVisible: Schema.Boolean,
    controllerLeaseId: Schema.Union(V09MobileControllerObserverControllerLeaseIdSchema, Schema.Null),
    proofRequirement: V09MobileControllerObserverProofRequirementSchema,
  })
);

export const V09MobileControllerObserverOperationProofSchema = withParser(
  Schema.Struct({
    operation: V09MobileControllerObserverOperationSchema,
    intentKind: LanPairingIntentKindSchema,
    responseState: LanPairingResponseStateSchema,
    operationState: V09MobileControllerObserverOperationStateSchema,
    runtimeOwner: V09MobileControllerObserverRuntimeOwnerSchema,
    rejectionReason: Schema.Union(LanPairingRejectionReasonSchema, Schema.Null),
    proofLabel: V09MobileControllerObserverProofLabelSchema,
    proofRequirement: V09MobileControllerObserverProofRequirementSchema,
    evidenceReferenceIds: Schema.Array(ParentEvidenceReferenceIdSchema),
  })
);

export const V09MobileControllerObserverReadModelSchema = withParser(
  Schema.Struct({
    platform: ParentMobilePlatformSchema,
    parentDeviceId: ParentDeviceIdSchema,
    role: V09MobileControllerObserverRoleSchema,
    controllerState: ParentMobileControllerStateSchema,
    commandAuthorityState: ParentMobileCommandAuthorityStateSchema,
    controllerLeaseProof: V09MobileControllerObserverControllerLeaseProofSchema,
    serviceState: ParentMobileServiceAvailabilityStateSchema,
    routeStatuses: Schema.Array(V09MobileControllerObserverRouteStatusSchema),
    packageReadiness: V09MobileControllerObserverPackageReadinessSchema,
    capabilities: Schema.Array(V09MobileControllerObserverCapabilityStateSchema),
    operationProofs: Schema.Array(V09MobileControllerObserverOperationProofSchema),
  })
);

export const V09MobileControllerObserverProofInputSchema = withParser(
  Schema.Struct({
    source: V09MobileControllerObserverProofSourceSchema,
    path: V09MobileControllerObserverProofPathSchema,
    command: V09MobileControllerObserverProofCommandSchema,
  })
);

export const V09MobileControllerObserverProofHarnessSchema = withParser(
  Schema.Struct({
    sourceProofs: Schema.Array(V09MobileControllerObserverProofInputSchema),
    outputProofPath: V09MobileControllerObserverProofPathSchema,
    checkpointPath: V09MobileControllerObserverProofPathSchema,
  })
);

export const V09MobileControllerObserverClaimBoundariesSchema = withParser(
  Schema.Struct({
    parentMobileWriteAuthority: V09MobileControllerObserverClaimBoundarySchema,
    physicalHouseholdLan: V09MobileControllerObserverClaimBoundarySchema,
    cloudRelay: V09MobileControllerObserverClaimBoundarySchema,
    childAgentBehavior: V09MobileControllerObserverClaimBoundarySchema,
    mobileChildAgentParity: V09MobileControllerObserverClaimBoundarySchema,
    signingStoresEntitlements: V09MobileControllerObserverClaimBoundarySchema,
    cUiOwnership: V09MobileControllerObserverClaimBoundarySchema,
  })
);

const V09MobileControllerObserverRuntimeReadModelBaseSchema = Schema.Struct({
  schemaVersion: V09MobileControllerObserverRuntimeSchemaVersionSchema,
  cloudRelayState: V09MobileControllerObserverReadinessStateSchema,
  mobileReadModels: Schema.Array(V09MobileControllerObserverReadModelSchema),
  proofHarness: V09MobileControllerObserverProofHarnessSchema,
  claimBoundaries: V09MobileControllerObserverClaimBoundariesSchema,
  updatedAt: ParentTimestampSchema,
});

type V09MobileControllerObserverRuntimeReadModelCandidate = Infer<
  typeof V09MobileControllerObserverRuntimeReadModelBaseSchema
>;

export const V09MobileControllerObserverRuntimeReadModelSchema = withParser(
  V09MobileControllerObserverRuntimeReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        v09MobileControllerObserverRuntimeIsHonest(readModel) ||
        'Expected V0.9 mobile controller observer runtime proof to keep parent mobile observer/manual-required until real device authority exists'
    )
  )
);

const RequiredObserverOperations = [
  'observe-status',
  'preview-policy-draft',
  'refresh-capabilities',
  'request-controller-takeover',
  'release-controller-lease',
  'submit-lan-ai-job',
  'write-policy',
  'approve-override',
  'pair-device',
  'revoke-device',
] as const satisfies ReadonlyArray<V09MobileControllerObserverOperation>;

const RequiredObserverRouteKinds = [
  'local-service',
  'lan-service',
  'cloud-relay',
  'parent-cache',
  'parent-owned-storage',
] as const satisfies ReadonlyArray<V09MobileControllerObserverRouteKind>;

const ReadOnlyOperations = [
  'observe-status',
  'preview-policy-draft',
  'refresh-capabilities',
] as const satisfies ReadonlyArray<V09MobileControllerObserverOperation>;

const ObserverRejectedOperations = [
  'write-policy',
  'approve-override',
  'pair-device',
  'revoke-device',
] as const satisfies ReadonlyArray<V09MobileControllerObserverOperation>;

type V09MobileControllerObserverOperationExpectation = Readonly<{
  operationState: V09MobileControllerObserverOperationState;
  responseState: Infer<typeof LanPairingResponseStateSchema>;
  runtimeOwner: V09MobileControllerObserverRuntimeOwner;
  rejectionReason: Infer<typeof LanPairingRejectionReasonSchema> | null;
}>;

const OperationExpectations: ReadonlyMap<
  V09MobileControllerObserverOperation,
  V09MobileControllerObserverOperationExpectation
> = new Map([
  ...ReadOnlyOperations.map((operation) =>
    operationExpectation(operation, {
      operationState: 'allowed-read-only',
      responseState: 'completed',
      runtimeOwner: 'parent-mobile-shell',
      rejectionReason: null,
    })
  ),
  ...ObserverRejectedOperations.map((operation) =>
    operationExpectation(operation, {
      operationState: 'rejected-observer-read-only',
      responseState: 'rejected',
      runtimeOwner: 'agent-service',
      rejectionReason: 'observer-read-only',
    })
  ),
  operationExpectation('request-controller-takeover', {
    operationState: 'manual-required-mobile-package',
    responseState: 'rejected',
    runtimeOwner: 'manual-proof',
    rejectionReason: 'takeover-denied',
  }),
  operationExpectation('release-controller-lease', {
    operationState: 'proved-local-service',
    responseState: 'completed',
    runtimeOwner: 'agent-service',
    rejectionReason: null,
  }),
]);

function operationExpectation(
  operation: V09MobileControllerObserverOperation,
  expectation: V09MobileControllerObserverOperationExpectation
): readonly [V09MobileControllerObserverOperation, V09MobileControllerObserverOperationExpectation] {
  return [operation, expectation];
}

function v09MobileControllerObserverRuntimeIsHonest(
  readModel: V09MobileControllerObserverRuntimeReadModelCandidate
): boolean {
  return (
    readModel.cloudRelayState === 'not-implemented' &&
    proofHarnessIsComplete(readModel.proofHarness) &&
    mobileReadModelsAreComplete(readModel.mobileReadModels) &&
    claimBoundariesAreHonest(readModel.claimBoundaries)
  );
}

function proofHarnessIsComplete(proofHarness: V09MobileControllerObserverProofHarness): boolean {
  const sources = new Set(proofHarness.sourceProofs.map((proof) => proof.source));
  return (
    sources.has('parent-mobile-shell-runtime-proof') &&
    sources.has('v0-9-production-lan-mobile-controller-proof') &&
    sources.has('v0-9-mobile-controller-discovery-runtime-proof') &&
    proofHarness.outputProofPath.includes('v0-9-mobile-controller-observer-runtime-proof/proof.json') &&
    proofHarness.checkpointPath.includes('v0-9-mobile-controller-observer-runtime-proof-2026-05-29.md')
  );
}

function mobileReadModelsAreComplete(readModels: ReadonlyArray<V09MobileControllerObserverReadModel>): boolean {
  const platforms = new Set(readModels.map((readModel) => readModel.platform));
  return (
    readModels.length === 2 &&
    platforms.has('android') &&
    platforms.has('ios') &&
    lanAiProviderStatesAreCovered(readModels) &&
    readModels.every((readModel) => mobileReadModelIsHonest(readModel))
  );
}

function mobileReadModelIsHonest(readModel: V09MobileControllerObserverReadModel): boolean {
  if (
    readModel.controllerState === 'active-controller' ||
    readModel.commandAuthorityState === 'active-controller-backend-proof'
  ) {
    return false;
  }

  if (readModel.packageReadiness.signingState !== 'manual-required') {
    return false;
  }

  if (readModel.packageReadiness.missingCapabilityProofs.length === 0) {
    return false;
  }

  return (
    controllerLeaseProofIsHonest(readModel) &&
    routeStatusesAreHonest(readModel) &&
    operationProofsAreHonest(readModel.operationProofs)
  );
}

function controllerLeaseProofIsHonest(readModel: V09MobileControllerObserverReadModel): boolean {
  switch (readModel.controllerState) {
    case 'observer':
      return observerLeaseProofIsHonest(readModel);
    case 'manual-required':
      return manualRequiredLeaseProofIsHonest(readModel);
    case 'unavailable':
      return unavailableLeaseProofIsHonest(readModel);
    case 'active-controller':
      return false;
  }
}

function observerLeaseProofIsHonest(readModel: V09MobileControllerObserverReadModel): boolean {
  const leaseProof = readModel.controllerLeaseProof;
  return (
    leaseProof.leaseState === 'visible-read-only' &&
    leaseProof.controllerLeaseVisible === true &&
    leaseProof.controllerLeaseId !== null &&
    readModel.commandAuthorityState === 'observer-read-only'
  );
}

function manualRequiredLeaseProofIsHonest(readModel: V09MobileControllerObserverReadModel): boolean {
  const leaseProof = readModel.controllerLeaseProof;
  return (
    leaseProof.leaseState === 'manual-required' &&
    leaseProof.controllerLeaseVisible === false &&
    leaseProof.controllerLeaseId === null &&
    readModel.commandAuthorityState === 'controller-takeover-manual-required'
  );
}

function unavailableLeaseProofIsHonest(readModel: V09MobileControllerObserverReadModel): boolean {
  const leaseProof = readModel.controllerLeaseProof;
  return (
    leaseProof.leaseState === 'unavailable' &&
    leaseProof.controllerLeaseVisible === false &&
    leaseProof.controllerLeaseId === null &&
    readModel.commandAuthorityState === 'unavailable'
  );
}

function routeStatusesAreHonest(readModel: V09MobileControllerObserverReadModel): boolean {
  const byKind = new Map(readModel.routeStatuses.map((route) => [route.routeKind, route] as const));
  if (byKind.size !== readModel.routeStatuses.length || !RequiredObserverRouteKinds.every((kind) => byKind.has(kind))) {
    return false;
  }

  return (
    byKind.get('cloud-relay')?.state === 'not-implemented' &&
    byKind.get('parent-cache')?.state === 'stale' &&
    byKind.get('parent-owned-storage')?.state === 'offline' &&
    byKind.get('lan-service')?.state === readModel.serviceState &&
    byKind.get('local-service')?.state !== 'available'
  );
}

function operationProofsAreHonest(proofs: ReadonlyArray<V09MobileControllerObserverOperationProof>): boolean {
  const proofByOperation = new Map(proofs.map((proof) => [proof.operation, proof] as const));
  return (
    proofByOperation.size === proofs.length &&
    RequiredObserverOperations.every((operation) => proofByOperation.has(operation)) &&
    proofs.every((proof) => operationProofIsHonest(proof))
  );
}

function operationProofIsHonest(proof: V09MobileControllerObserverOperationProof): boolean {
  if (proof.operation === 'submit-lan-ai-job') {
    return lanAiProviderOperationProofIsHonest(proof);
  }

  const expected = OperationExpectations.get(proof.operation);
  return (
    expected !== undefined &&
    proof.operationState === expected.operationState &&
    proof.responseState === expected.responseState &&
    proof.runtimeOwner === expected.runtimeOwner &&
    proof.rejectionReason === expected.rejectionReason
  );
}

function lanAiProviderOperationProofIsHonest(proof: V09MobileControllerObserverOperationProof): boolean {
  if (proof.runtimeOwner !== 'lan-ai-provider' || proof.rejectionReason !== 'lan-ai-provider-unavailable') {
    return false;
  }

  return (
    proof.responseState === 'degraded' &&
    (proof.operationState === 'degraded-provider' || proof.operationState === 'unavailable')
  );
}

function lanAiProviderStatesAreCovered(readModels: ReadonlyArray<V09MobileControllerObserverReadModel>): boolean {
  const states = new Set(
    readModels
      .map((readModel) => readModel.operationProofs.find((proof) => proof.operation === 'submit-lan-ai-job'))
      .filter((proof): proof is V09MobileControllerObserverOperationProof => proof !== undefined)
      .map((proof) => proof.operationState)
  );
  return states.has('degraded-provider') && states.has('unavailable');
}

function claimBoundariesAreHonest(boundaries: V09MobileControllerObserverClaimBoundaries): boolean {
  return (
    boundaries.parentMobileWriteAuthority.includes('manual-required') &&
    boundaries.childAgentBehavior.includes('not claimed') &&
    boundaries.mobileChildAgentParity.includes('not claimed') &&
    boundaries.cloudRelay.includes('not implemented')
  );
}

export type V09MobileControllerObserverRole = Infer<typeof V09MobileControllerObserverRoleSchema>;
export type V09MobileControllerObserverRouteKind = Infer<typeof V09MobileControllerObserverRouteKindSchema>;
export type V09MobileControllerObserverReadinessState = Infer<typeof V09MobileControllerObserverReadinessStateSchema>;
export type V09MobileControllerObserverOperation = Infer<typeof V09MobileControllerObserverOperationSchema>;
export type V09MobileControllerObserverOperationState = Infer<typeof V09MobileControllerObserverOperationStateSchema>;
export type V09MobileControllerObserverRuntimeOwner = Infer<typeof V09MobileControllerObserverRuntimeOwnerSchema>;
export type V09MobileControllerObserverProofSource = Infer<typeof V09MobileControllerObserverProofSourceSchema>;
export type V09MobileControllerObserverControllerLeaseState = Infer<
  typeof V09MobileControllerObserverControllerLeaseStateSchema
>;
export type V09MobileControllerObserverPackageReadiness = Infer<
  typeof V09MobileControllerObserverPackageReadinessSchema
>;
export type V09MobileControllerObserverCapabilityState = Infer<typeof V09MobileControllerObserverCapabilityStateSchema>;
export type V09MobileControllerObserverRouteStatus = Infer<typeof V09MobileControllerObserverRouteStatusSchema>;
export type V09MobileControllerObserverControllerLeaseProof = Infer<
  typeof V09MobileControllerObserverControllerLeaseProofSchema
>;
export type V09MobileControllerObserverOperationProof = Infer<typeof V09MobileControllerObserverOperationProofSchema>;
export type V09MobileControllerObserverReadModel = Infer<typeof V09MobileControllerObserverReadModelSchema>;
export type V09MobileControllerObserverProofInput = Infer<typeof V09MobileControllerObserverProofInputSchema>;
export type V09MobileControllerObserverProofHarness = Infer<typeof V09MobileControllerObserverProofHarnessSchema>;
export type V09MobileControllerObserverClaimBoundaries = Infer<typeof V09MobileControllerObserverClaimBoundariesSchema>;
export type V09MobileControllerObserverRuntimeReadModel = Infer<
  typeof V09MobileControllerObserverRuntimeReadModelSchema
>;


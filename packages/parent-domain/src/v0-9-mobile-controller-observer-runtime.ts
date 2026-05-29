import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  LanPairingIntentKindSchema,
  LanPairingRejectionReasonSchema,
  LanPairingResponseStateSchema,
} from './lan-pairing-values';
import {
  ParentMobileCommandAuthorityStateSchema,
  ParentMobileControllerStateSchema,
  ParentMobilePackageStateSchema,
  ParentMobilePlatformSchema,
  ParentMobileServiceAvailabilityStateSchema,
  ParentMobileSigningStateSchema,
  ParentMobileStoreDistributionStateSchema,
} from './parent-mobile-runtime';
import { ParentControlCapabilityNameSchema, ParentControlCapabilityStatusSchema } from './capabilities';
import { ParentDeviceIdSchema, ParentEvidenceReferenceIdSchema, ParentTimestampSchema } from './reference-primitives';

const NonEmptyV09ObserverRuntimeText = Schema.String.pipe(Schema.minLength(1));

export const V09MobileControllerObserverRuntimeSchemaVersionSchema = withParser(
  Schema.Literal('v0.9-mobile-controller-observer-runtime')
);
export const V09MobileControllerObserverRoleSchema = withParser(
  Schema.Literal('observer', 'controller-candidate', 'degraded-observer')
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

export const V09MobileControllerObserverProofLabelSchema = NonEmptyV09ObserverRuntimeText.pipe(
  Schema.brand('V09MobileControllerObserverProofLabel')
);
export const V09MobileControllerObserverProofPathSchema = NonEmptyV09ObserverRuntimeText.pipe(
  Schema.brand('V09MobileControllerObserverProofPath')
);
export const V09MobileControllerObserverProofCommandSchema = NonEmptyV09ObserverRuntimeText.pipe(
  Schema.brand('V09MobileControllerObserverProofCommand')
);
export const V09MobileControllerObserverProofRequirementSchema = NonEmptyV09ObserverRuntimeText.pipe(
  Schema.brand('V09MobileControllerObserverProofRequirement')
);
export const V09MobileControllerObserverClaimBoundarySchema = NonEmptyV09ObserverRuntimeText.pipe(
  Schema.brand('V09MobileControllerObserverClaimBoundary')
);

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
    serviceState: ParentMobileServiceAvailabilityStateSchema,
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
  operationExpectation('submit-lan-ai-job', {
    operationState: 'degraded-provider',
    responseState: 'degraded',
    runtimeOwner: 'lan-ai-provider',
    rejectionReason: 'lan-ai-provider-unavailable',
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
    mobileReadModelsAreComplete(readModel.mobileReadModels)
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

  return operationProofsAreHonest(readModel.operationProofs);
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
  const expected = OperationExpectations.get(proof.operation);
  return (
    expected !== undefined &&
    proof.operationState === expected.operationState &&
    proof.responseState === expected.responseState &&
    proof.runtimeOwner === expected.runtimeOwner &&
    proof.rejectionReason === expected.rejectionReason
  );
}

export type V09MobileControllerObserverRole = Infer<typeof V09MobileControllerObserverRoleSchema>;
export type V09MobileControllerObserverReadinessState = Infer<typeof V09MobileControllerObserverReadinessStateSchema>;
export type V09MobileControllerObserverOperation = Infer<typeof V09MobileControllerObserverOperationSchema>;
export type V09MobileControllerObserverOperationState = Infer<typeof V09MobileControllerObserverOperationStateSchema>;
export type V09MobileControllerObserverRuntimeOwner = Infer<typeof V09MobileControllerObserverRuntimeOwnerSchema>;
export type V09MobileControllerObserverProofSource = Infer<typeof V09MobileControllerObserverProofSourceSchema>;
export type V09MobileControllerObserverPackageReadiness = Infer<
  typeof V09MobileControllerObserverPackageReadinessSchema
>;
export type V09MobileControllerObserverCapabilityState = Infer<typeof V09MobileControllerObserverCapabilityStateSchema>;
export type V09MobileControllerObserverOperationProof = Infer<typeof V09MobileControllerObserverOperationProofSchema>;
export type V09MobileControllerObserverReadModel = Infer<typeof V09MobileControllerObserverReadModelSchema>;
export type V09MobileControllerObserverProofInput = Infer<typeof V09MobileControllerObserverProofInputSchema>;
export type V09MobileControllerObserverProofHarness = Infer<typeof V09MobileControllerObserverProofHarnessSchema>;
export type V09MobileControllerObserverClaimBoundaries = Infer<typeof V09MobileControllerObserverClaimBoundariesSchema>;
export type V09MobileControllerObserverRuntimeReadModel = Infer<
  typeof V09MobileControllerObserverRuntimeReadModelSchema
>;

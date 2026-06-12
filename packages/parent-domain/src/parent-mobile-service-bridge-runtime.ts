import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { LanPairingRejectionReasonSchema, type LanPairingRejectionReason } from '@ocentra-parent/lan-domain/lan-pairing-values';
import {
  AndroidParentMobileCapabilityStatuses,
  IosParentMobileCapabilityStatuses,
} from './parent-mobile-runtime-capability-statuses';
import {
  ParentMobileCommandAuthorityStateSchema,
  ParentMobileControllerStateSchema,
  ParentMobileLocalModelExecutionStateSchema,
  ParentMobilePackageStateSchema,
  ParentMobilePlatformSchema,
  ParentMobileSigningStateSchema,
  ParentMobileStoreDistributionStateSchema,
  type ParentMobileCommandAuthorityState,
  type ParentMobilePlatform,
} from './parent-mobile-runtime';
import { ParentControlCapabilityNameSchema, ParentControlCapabilityStatusSchema } from './capabilities';
import { ParentDeviceIdSchema, ParentEvidenceReferenceIdSchema, ParentTimestampSchema } from './reference-primitives';

const NonEmptyParentMobileServiceBridgeText = Schema.String.pipe(Schema.minLength(1));

export const ParentMobileServiceBridgeRuntimeSchemaVersionSchema = withParser(
  Schema.Literal('parent-mobile-service-bridge-proof')
);
export const ParentMobileServiceBridgeConnectionKindSchema = withParser(
  Schema.Literal(
    'local-service',
    'lan-service',
    'cloud-relay',
    'parent-cache',
    'parent-owned-storage',
    'mobile-package'
  )
);
export const ParentMobileServiceBridgeProofStateSchema = withParser(
  Schema.Literal(
    'available',
    'degraded',
    'unavailable',
    'manual-required',
    'not-implemented',
    'ci-mechanical-proof',
    'stale',
    'offline'
  )
);
export const ParentMobileServiceBridgeRoleSchema = withParser(
  Schema.Literal('observer', 'controller-candidate', 'degraded-observer')
);
export const ParentMobileServiceBridgeOperationSchema = withParser(
  Schema.Literal(
    'service-status-read',
    'lan-route-status-read',
    'parent-cache-status-read',
    'parent-owned-storage-status-read',
    'capability-refresh',
    'package-service-launch',
    'controller-takeover-request',
    'controller-release',
    'write-policy',
    'approval-decision',
    'submit-lan-ai-job',
    'submit-cloud-relay-job',
    'submit-phone-local-model-job'
  )
);
export const ParentMobileServiceBridgeOperationStateSchema = withParser(
  Schema.Literal(
    'allowed-read-only',
    'proved-local-service',
    'rejected-observer-read-only',
    'manual-required-mobile-package',
    'degraded-provider',
    'unavailable',
    'not-implemented',
    'rejected-no-phone-local-model'
  )
);
export const ParentMobileServiceBridgeOperationResponseStateSchema = withParser(
  Schema.Literal('completed', 'rejected', 'degraded', 'unavailable', 'not-implemented')
);
export const ParentMobileServiceBridgeRuntimeOwnerSchema = withParser(
  Schema.Literal(
    'parent-mobile-shell',
    'agent-service',
    'lan-ai-provider',
    'manual-proof',
    'cloud-relay-not-implemented',
    'parent-cache',
    'parent-owned-storage'
  )
);
export const ParentMobileServiceBridgeAssistantRouteSchema = withParser(
  Schema.Literal('lan-ai-provider', 'unavailable', 'phone-local-model-disabled')
);
export const ParentMobileServiceBridgeAssistantJobStateSchema = withParser(
  Schema.Literal('degraded', 'unavailable', 'rejected')
);
export const ParentMobileServiceBridgeProofSourceSchema = withParser(
  Schema.Literal(
    'parent-mobile-shell-runtime-proof',
    'v0-9-production-lan-mobile-controller-proof',
    'v0-9-mobile-controller-observer-runtime-proof'
  )
);

const ParentMobileServiceBridgeProofLabelSchema = NonEmptyParentMobileServiceBridgeText.pipe(
  Schema.brand('ParentMobileServiceBridgeProofLabel')
);
const ParentMobileServiceBridgeProofPathSchema = NonEmptyParentMobileServiceBridgeText.pipe(
  Schema.brand('ParentMobileServiceBridgeProofPath')
);
const ParentMobileServiceBridgeProofCommandSchema = NonEmptyParentMobileServiceBridgeText.pipe(
  Schema.brand('ParentMobileServiceBridgeProofCommand')
);
const ParentMobileServiceBridgeProofRequirementSchema = NonEmptyParentMobileServiceBridgeText.pipe(
  Schema.brand('ParentMobileServiceBridgeProofRequirement')
);
const ParentMobileServiceBridgeClaimBoundarySchema = NonEmptyParentMobileServiceBridgeText.pipe(
  Schema.brand('ParentMobileServiceBridgeClaimBoundary')
);
const ParentMobileServiceBridgeRouteIdSchema = NonEmptyParentMobileServiceBridgeText.pipe(
  Schema.brand('ParentMobileServiceBridgeRouteId')
);
const ParentMobileServiceBridgeProviderIdSchema = NonEmptyParentMobileServiceBridgeText.pipe(
  Schema.brand('ParentMobileServiceBridgeProviderId')
);
const ParentMobileServiceBridgeAiCapabilitySchema = NonEmptyParentMobileServiceBridgeText.pipe(
  Schema.brand('ParentMobileServiceBridgeAiCapability')
);
const ParentMobileServiceBridgeUnavailableReasonSchema = NonEmptyParentMobileServiceBridgeText.pipe(
  Schema.brand('ParentMobileServiceBridgeUnavailableReason')
);
const ParentMobileServiceBridgeLaunchTargetSchema = NonEmptyParentMobileServiceBridgeText.pipe(
  Schema.brand('ParentMobileServiceBridgeLaunchTarget')
);

export const ParentMobileServiceBridgeProofInputSchema = withParser(
  Schema.Struct({
    source: ParentMobileServiceBridgeProofSourceSchema,
    path: ParentMobileServiceBridgeProofPathSchema,
    command: ParentMobileServiceBridgeProofCommandSchema,
  })
);

export const ParentMobileServiceBridgeConnectionSchema = withParser(
  Schema.Struct({
    connectionKind: ParentMobileServiceBridgeConnectionKindSchema,
    state: ParentMobileServiceBridgeProofStateSchema,
    runtimeOwner: ParentMobileServiceBridgeRuntimeOwnerSchema,
    selectedRouteId: Schema.Union(ParentMobileServiceBridgeRouteIdSchema, Schema.Null),
    proofLabel: ParentMobileServiceBridgeProofLabelSchema,
    proofRequirement: ParentMobileServiceBridgeProofRequirementSchema,
  })
);

export const ParentMobileServiceBridgePackageReadinessSchema = withParser(
  Schema.Struct({
    platform: ParentMobilePlatformSchema,
    packageState: ParentMobilePackageStateSchema,
    serviceLaunchState: ParentMobileServiceBridgeProofStateSchema,
    launchTarget: ParentMobileServiceBridgeLaunchTargetSchema,
    signingState: ParentMobileSigningStateSchema,
    storeDistributionState: ParentMobileStoreDistributionStateSchema,
    missingCapabilityProofs: Schema.Array(ParentControlCapabilityNameSchema),
  })
);

export const ParentMobileServiceBridgeCapabilityStateSchema = withParser(
  Schema.Struct({
    capability: ParentControlCapabilityNameSchema,
    status: ParentControlCapabilityStatusSchema,
    proofRequirement: ParentMobileServiceBridgeProofRequirementSchema,
    claimBoundary: ParentMobileServiceBridgeClaimBoundarySchema,
  })
);

export const ParentMobileServiceBridgeAssistantSubmissionSchema = withParser(
  Schema.Struct({
    route: ParentMobileServiceBridgeAssistantRouteSchema,
    jobState: ParentMobileServiceBridgeAssistantJobStateSchema,
    providerId: Schema.Union(ParentMobileServiceBridgeProviderIdSchema, Schema.Null),
    requiredCapabilities: Schema.Array(ParentMobileServiceBridgeAiCapabilitySchema),
    evidenceReferenceIds: Schema.Array(ParentEvidenceReferenceIdSchema),
    unavailableReason: Schema.Union(ParentMobileServiceBridgeUnavailableReasonSchema, Schema.Null),
    localModelExecutionState: ParentMobileLocalModelExecutionStateSchema,
    localModelExecutionAllowed: Schema.Literal(false),
  })
);

export const ParentMobileServiceBridgeOperationProofSchema = withParser(
  Schema.Struct({
    operation: ParentMobileServiceBridgeOperationSchema,
    responseState: ParentMobileServiceBridgeOperationResponseStateSchema,
    operationState: ParentMobileServiceBridgeOperationStateSchema,
    runtimeOwner: ParentMobileServiceBridgeRuntimeOwnerSchema,
    commandAuthorityState: ParentMobileCommandAuthorityStateSchema,
    rejectionReason: Schema.Union(LanPairingRejectionReasonSchema, Schema.Null),
    unavailableReason: Schema.Union(ParentMobileServiceBridgeUnavailableReasonSchema, Schema.Null),
    proofLabel: ParentMobileServiceBridgeProofLabelSchema,
    proofRequirement: ParentMobileServiceBridgeProofRequirementSchema,
    evidenceReferenceIds: Schema.Array(ParentEvidenceReferenceIdSchema),
  })
);

export const ParentMobileServiceBridgeReadModelSchema = withParser(
  Schema.Struct({
    platform: ParentMobilePlatformSchema,
    parentDeviceId: ParentDeviceIdSchema,
    role: ParentMobileServiceBridgeRoleSchema,
    controllerState: ParentMobileControllerStateSchema,
    commandAuthorityState: ParentMobileCommandAuthorityStateSchema,
    connections: Schema.Array(ParentMobileServiceBridgeConnectionSchema),
    packageReadiness: ParentMobileServiceBridgePackageReadinessSchema,
    aiSubmission: ParentMobileServiceBridgeAssistantSubmissionSchema,
    capabilities: Schema.Array(ParentMobileServiceBridgeCapabilityStateSchema),
    operationProofs: Schema.Array(ParentMobileServiceBridgeOperationProofSchema),
  })
);

export const ParentMobileServiceBridgeClaimBoundariesSchema = withParser(
  Schema.Struct({
    parentMobileWriteAuthority: ParentMobileServiceBridgeClaimBoundarySchema,
    physicalHouseholdLan: ParentMobileServiceBridgeClaimBoundarySchema,
    cloudRelay: ParentMobileServiceBridgeClaimBoundarySchema,
    parentOwnedStorage: ParentMobileServiceBridgeClaimBoundarySchema,
    phoneLocalModel: ParentMobileServiceBridgeClaimBoundarySchema,
    packageServiceLaunch: ParentMobileServiceBridgeClaimBoundarySchema,
    androidParentMobile: ParentMobileServiceBridgeClaimBoundarySchema,
    iosParentMobile: ParentMobileServiceBridgeClaimBoundarySchema,
    androidChildAgent: ParentMobileServiceBridgeClaimBoundarySchema,
    iosChildAgent: ParentMobileServiceBridgeClaimBoundarySchema,
    cUiOwnership: ParentMobileServiceBridgeClaimBoundarySchema,
  })
);

export const ParentMobileServiceBridgeProofHarnessSchema = withParser(
  Schema.Struct({
    sourceProofs: Schema.Array(ParentMobileServiceBridgeProofInputSchema),
    outputProofPath: ParentMobileServiceBridgeProofPathSchema,
    checkpointPath: ParentMobileServiceBridgeProofPathSchema,
  })
);

const ParentMobileServiceBridgeRuntimeReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentMobileServiceBridgeRuntimeSchemaVersionSchema,
  proofHarness: ParentMobileServiceBridgeProofHarnessSchema,
  mobileBridgeReadModels: Schema.Array(ParentMobileServiceBridgeReadModelSchema),
  claimBoundaries: ParentMobileServiceBridgeClaimBoundariesSchema,
  updatedAt: ParentTimestampSchema,
});

type ParentMobileServiceBridgeRuntimeReadModelCandidate = Infer<
  typeof ParentMobileServiceBridgeRuntimeReadModelBaseSchema
>;

export const ParentMobileServiceBridgeRuntimeReadModelSchema = withParser(
  ParentMobileServiceBridgeRuntimeReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        parentMobileServiceBridgeRuntimeIsHonest(readModel) ||
        'Expected parent mobile service bridge proof to keep mobile authority manual, cloud relay not implemented, LAN AI degraded/unavailable, and phone-local model execution disabled'
    )
  )
);

const RequiredConnectionKinds = [
  'local-service',
  'lan-service',
  'cloud-relay',
  'parent-cache',
  'parent-owned-storage',
  'mobile-package',
] as const satisfies ReadonlyArray<ParentMobileServiceBridgeConnectionKind>;

const RequiredOperations = [
  'service-status-read',
  'lan-route-status-read',
  'parent-cache-status-read',
  'parent-owned-storage-status-read',
  'capability-refresh',
  'package-service-launch',
  'controller-takeover-request',
  'controller-release',
  'write-policy',
  'approval-decision',
  'submit-lan-ai-job',
  'submit-cloud-relay-job',
  'submit-phone-local-model-job',
] as const satisfies ReadonlyArray<ParentMobileServiceBridgeOperation>;

type StaticOperationExpectation = Readonly<{
  responseState: ParentMobileServiceBridgeOperationResponseState;
  operationState: ParentMobileServiceBridgeOperationState;
  runtimeOwner: ParentMobileServiceBridgeRuntimeOwner;
  commandAuthorityState: ParentMobileCommandAuthorityState;
  rejectionReason: LanPairingRejectionReason | null;
}>;

const StaticOperationExpectations: ReadonlyMap<ParentMobileServiceBridgeOperation, StaticOperationExpectation> =
  new Map([
    staticOperation(
      'service-status-read',
      'completed',
      'allowed-read-only',
      'parent-mobile-shell',
      'observer-read-only'
    ),
    staticOperation(
      'lan-route-status-read',
      'completed',
      'allowed-read-only',
      'parent-mobile-shell',
      'observer-read-only'
    ),
    staticOperation(
      'parent-cache-status-read',
      'completed',
      'allowed-read-only',
      'parent-mobile-shell',
      'observer-read-only'
    ),
    staticOperation(
      'parent-owned-storage-status-read',
      'completed',
      'allowed-read-only',
      'parent-mobile-shell',
      'observer-read-only'
    ),
    staticOperation(
      'capability-refresh',
      'completed',
      'allowed-read-only',
      'parent-mobile-shell',
      'observer-read-only'
    ),
    staticOperation(
      'package-service-launch',
      'rejected',
      'manual-required-mobile-package',
      'manual-proof',
      'controller-takeover-manual-required',
      'takeover-denied'
    ),
    staticOperation(
      'controller-takeover-request',
      'rejected',
      'manual-required-mobile-package',
      'manual-proof',
      'controller-takeover-manual-required',
      'takeover-denied'
    ),
    staticOperation('controller-release', 'completed', 'proved-local-service', 'agent-service', 'observer-read-only'),
    staticOperation(
      'write-policy',
      'rejected',
      'rejected-observer-read-only',
      'agent-service',
      'observer-read-only',
      'observer-read-only'
    ),
    staticOperation(
      'approval-decision',
      'rejected',
      'rejected-observer-read-only',
      'agent-service',
      'observer-read-only',
      'observer-read-only'
    ),
    staticOperation(
      'submit-cloud-relay-job',
      'not-implemented',
      'not-implemented',
      'cloud-relay-not-implemented',
      'observer-read-only'
    ),
    staticOperation(
      'submit-phone-local-model-job',
      'rejected',
      'rejected-no-phone-local-model',
      'parent-mobile-shell',
      'observer-read-only'
    ),
  ]);

function staticOperation(
  operation: ParentMobileServiceBridgeOperation,
  responseState: ParentMobileServiceBridgeOperationResponseState,
  operationState: ParentMobileServiceBridgeOperationState,
  runtimeOwner: ParentMobileServiceBridgeRuntimeOwner,
  commandAuthorityState: ParentMobileCommandAuthorityState,
  rejectionReason: LanPairingRejectionReason | null = null
): readonly [ParentMobileServiceBridgeOperation, StaticOperationExpectation] {
  return [operation, { responseState, operationState, runtimeOwner, commandAuthorityState, rejectionReason }];
}

function parentMobileServiceBridgeRuntimeIsHonest(
  readModel: ParentMobileServiceBridgeRuntimeReadModelCandidate
): boolean {
  return proofHarnessIsComplete(readModel.proofHarness) && mobileBridgeReadModelsAreComplete(readModel);
}

function proofHarnessIsComplete(proofHarness: ParentMobileServiceBridgeProofHarness): boolean {
  const sources = new Set(proofHarness.sourceProofs.map((proof) => proof.source));
  return (
    sources.has('parent-mobile-shell-runtime-proof') &&
    sources.has('v0-9-production-lan-mobile-controller-proof') &&
    sources.has('v0-9-mobile-controller-observer-runtime-proof') &&
    proofHarness.outputProofPath.includes('parent-mobile-service-bridge-proof/proof.json') &&
    proofHarness.checkpointPath.includes('parent-mobile-service-bridge-proof-2026-05-29.md')
  );
}

function mobileBridgeReadModelsAreComplete(readModel: ParentMobileServiceBridgeRuntimeReadModelCandidate): boolean {
  const platforms = new Set(readModel.mobileBridgeReadModels.map((entry) => entry.platform));
  return (
    readModel.mobileBridgeReadModels.length === 2 &&
    platforms.has('android') &&
    platforms.has('ios') &&
    readModel.mobileBridgeReadModels.every((entry) => mobileBridgeReadModelIsHonest(entry))
  );
}

function mobileBridgeReadModelIsHonest(readModel: ParentMobileServiceBridgeReadModel): boolean {
  if (
    readModel.controllerState === 'active-controller' ||
    readModel.commandAuthorityState === 'active-controller-backend-proof'
  ) {
    return false;
  }

  return (
    serviceConnectionsAreHonest(readModel.connections) &&
    packageReadinessIsHonest(readModel.packageReadiness, readModel.platform) &&
    aiSubmissionIsHonest(readModel.aiSubmission) &&
    capabilityStatesAreHonest(readModel) &&
    operationProofsAreHonest(readModel)
  );
}

function serviceConnectionsAreHonest(connections: ReadonlyArray<ParentMobileServiceBridgeConnection>): boolean {
  const byKind = new Map(connections.map((connection) => [connection.connectionKind, connection] as const));
  if (byKind.size !== connections.length || !RequiredConnectionKinds.every((kind) => byKind.has(kind))) {
    return false;
  }

  return (
    byKind.get('cloud-relay')?.state === 'not-implemented' &&
    byKind.get('parent-cache')?.state === 'stale' &&
    byKind.get('parent-owned-storage')?.state === 'offline' &&
    byKind.get('mobile-package')?.state === 'ci-mechanical-proof' &&
    byKind.get('local-service')?.state !== 'available'
  );
}

function packageReadinessIsHonest(
  packageReadiness: ParentMobileServiceBridgePackageReadiness,
  platform: ParentMobilePlatform
): boolean {
  return (
    packageReadiness.platform === platform &&
    packageReadiness.signingState === 'manual-required' &&
    packageReadiness.serviceLaunchState === 'manual-required' &&
    packageReadiness.missingCapabilityProofs.length > 0
  );
}

function aiSubmissionIsHonest(submission: ParentMobileServiceBridgeAssistantSubmission): boolean {
  if (
    submission.localModelExecutionAllowed !== false ||
    submission.localModelExecutionState !== 'disabled-by-default' ||
    submission.requiredCapabilities.length === 0
  ) {
    return false;
  }

  if (submission.jobState === 'degraded' || submission.jobState === 'unavailable') {
    return (
      submission.route !== 'phone-local-model-disabled' &&
      submission.providerId === null &&
      submission.unavailableReason !== null
    );
  }

  return (
    submission.route === 'phone-local-model-disabled' &&
    submission.providerId === null &&
    submission.unavailableReason !== null
  );
}

function capabilityStatesAreHonest(readModel: ParentMobileServiceBridgeReadModel): boolean {
  const expected =
    readModel.platform === 'android' ? AndroidParentMobileCapabilityStatuses : IosParentMobileCapabilityStatuses;
  const byCapability = new Map(readModel.capabilities.map((entry) => [entry.capability, entry.status] as const));
  return (
    byCapability.size === readModel.capabilities.length &&
    byCapability.size === expected.length &&
    readModel.capabilities.every((entry) => entry.status !== 'implemented' && entry.status !== 'supported') &&
    expected.every(([capability, status]) => byCapability.get(capability) === status)
  );
}

function operationProofsAreHonest(readModel: ParentMobileServiceBridgeReadModel): boolean {
  const byOperation = new Map(readModel.operationProofs.map((proof) => [proof.operation, proof] as const));
  return (
    byOperation.size === readModel.operationProofs.length &&
    RequiredOperations.every((operation) => byOperation.has(operation)) &&
    readModel.operationProofs.every((proof) => operationProofIsHonest(proof, readModel.aiSubmission.jobState))
  );
}

function operationProofIsHonest(
  proof: ParentMobileServiceBridgeOperationProof,
  aiSubmissionState: ParentMobileServiceBridgeAssistantJobState
): boolean {
  if (proof.operation === 'submit-lan-ai-job') {
    return lanAiOperationProofIsHonest(proof, aiSubmissionState);
  }

  const expected = StaticOperationExpectations.get(proof.operation);
  return (
    expected !== undefined &&
    proof.responseState === expected.responseState &&
    proof.operationState === expected.operationState &&
    proof.runtimeOwner === expected.runtimeOwner &&
    proof.commandAuthorityState === expected.commandAuthorityState &&
    proof.rejectionReason === expected.rejectionReason
  );
}

function lanAiOperationProofIsHonest(
  proof: ParentMobileServiceBridgeOperationProof,
  aiSubmissionState: ParentMobileServiceBridgeAssistantJobState
): boolean {
  if (aiSubmissionState === 'degraded') {
    return (
      proof.responseState === 'degraded' &&
      proof.operationState === 'degraded-provider' &&
      proof.runtimeOwner === 'lan-ai-provider' &&
      proof.rejectionReason === 'lan-ai-provider-unavailable' &&
      proof.unavailableReason !== null
    );
  }

  return (
    aiSubmissionState === 'unavailable' &&
    proof.responseState === 'unavailable' &&
    proof.operationState === 'unavailable' &&
    proof.runtimeOwner === 'lan-ai-provider' &&
    proof.rejectionReason === 'lan-ai-provider-unavailable' &&
    proof.unavailableReason !== null
  );
}

export type ParentMobileServiceBridgeConnectionKind = Infer<typeof ParentMobileServiceBridgeConnectionKindSchema>;
export type ParentMobileServiceBridgeProofState = Infer<typeof ParentMobileServiceBridgeProofStateSchema>;
export type ParentMobileServiceBridgeRole = Infer<typeof ParentMobileServiceBridgeRoleSchema>;
export type ParentMobileServiceBridgeOperation = Infer<typeof ParentMobileServiceBridgeOperationSchema>;
export type ParentMobileServiceBridgeOperationState = Infer<typeof ParentMobileServiceBridgeOperationStateSchema>;
export type ParentMobileServiceBridgeOperationResponseState = Infer<
  typeof ParentMobileServiceBridgeOperationResponseStateSchema
>;
export type ParentMobileServiceBridgeRuntimeOwner = Infer<typeof ParentMobileServiceBridgeRuntimeOwnerSchema>;
export type ParentMobileServiceBridgeAssistantRoute = Infer<typeof ParentMobileServiceBridgeAssistantRouteSchema>;
export type ParentMobileServiceBridgeAssistantJobState = Infer<typeof ParentMobileServiceBridgeAssistantJobStateSchema>;
export type ParentMobileServiceBridgeProofInput = Infer<typeof ParentMobileServiceBridgeProofInputSchema>;
export type ParentMobileServiceBridgeConnection = Infer<typeof ParentMobileServiceBridgeConnectionSchema>;
export type ParentMobileServiceBridgePackageReadiness = Infer<typeof ParentMobileServiceBridgePackageReadinessSchema>;
export type ParentMobileServiceBridgeCapabilityState = Infer<typeof ParentMobileServiceBridgeCapabilityStateSchema>;
export type ParentMobileServiceBridgeAssistantSubmission = Infer<
  typeof ParentMobileServiceBridgeAssistantSubmissionSchema
>;
export type ParentMobileServiceBridgeOperationProof = Infer<typeof ParentMobileServiceBridgeOperationProofSchema>;
export type ParentMobileServiceBridgeReadModel = Infer<typeof ParentMobileServiceBridgeReadModelSchema>;
export type ParentMobileServiceBridgeClaimBoundaries = Infer<typeof ParentMobileServiceBridgeClaimBoundariesSchema>;
export type ParentMobileServiceBridgeProofHarness = Infer<typeof ParentMobileServiceBridgeProofHarnessSchema>;
export type ParentMobileServiceBridgeRuntimeReadModel = Infer<typeof ParentMobileServiceBridgeRuntimeReadModelSchema>;

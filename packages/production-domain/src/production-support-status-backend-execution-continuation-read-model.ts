import { Schema } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
import {
  ProductionSupportStatusBackendExecutionContinuationProofSchema,
  type ProductionSupportStatusBackendExecutionContinuationRow,
  type ProductionSupportStatusBackendExecutionContinuationState,
  type ProductionSupportStatusBackendExecutionContinuationTarget,
} from './production-support-status-backend-execution-continuation-proof';
import {
  ExecutionContinuationManualRequirementSchema,
  ExecutionContinuationReferenceSchema,
  ExecutionContinuationSupportSafeDataClasses,
  ForbiddenExecutionContinuationDataClasses,
  RequiredExecutionContinuationNonClaims,
  RequiredExecutionContinuationSourceProofs,
  RequiredExecutionContinuationStates,
  RequiredExecutionContinuationTargets,
} from './production-support-status-backend-execution-continuation-values';

export const ProductionSupportStatusBackendExecutionContinuationReadModel =
  ProductionSupportStatusBackendExecutionContinuationProofSchema.parse({
    schemaVersion: 'production-support-status-backend-execution-continuation-proof',
    sourceContractRefs: RequiredExecutionContinuationSourceProofs,
    rows: RequiredExecutionContinuationTargets.flatMap((target) =>
      RequiredExecutionContinuationStates.map((continuationState) =>
        executionContinuationReadiness(target, continuationState)
      )
    ),
    nonClaims: RequiredExecutionContinuationNonClaims,
    statusBackendExecutionClaim: 'manual-required',
    durableQueueStorageClaim: 'manual-required',
    retryWorkerExecutionClaim: 'manual-required',
    auditPersistenceClaim: 'manual-required',
    deadLetterPayloadCustodyClaim: 'manual-required',
    statusBackendPayloadCustodyClaim: 'manual-required',
    redactionManifestExecutionClaim: 'manual-required',
    publicRuntimeExecutionClaim: 'not-implemented',
    providerExecutionClaim: 'not-implemented',
    supportBackendUploadExecutionClaim: 'manual-required',
    accountLookupExecutionClaim: 'manual-required',
    billingProviderContactClaim: 'manual-required',
    legalDisclosureExecutionClaim: 'manual-required',
    remoteSupportSessionClaim: 'not-implemented',
    productionSlaClaim: 'not-implemented',
    providerSecretCustodyClaim: 'not-implemented',
    defaultHostedFamilyDataClaim: 'not-implemented',
    childActivityCustodyClaim: 'not-implemented',
    updatedAt: Schema.decodeUnknownSync(ParentTimestampSchema)('2026-06-07T21:21:51.734Z'),
  });

export const ProductionSupportStatusBackendExecutionContinuationKnownGaps = [
  'Execution continuation composes durable queue runtime, runtime closure, status backend payload-custody, and redaction-manifest proof refs without claiming live status backend execution.',
  'Real status backend execution, durable queue storage, retry-worker execution, audit persistence, dead-letter payload custody, status backend payload custody, and redaction-manifest execution remain manual-required until a real status backend worker and storage boundary exist.',
  'Public runtime execution, provider execution, support backend upload execution, account lookup, billing provider contact, legal disclosure execution, remote support sessions, production SLA, provider-secret custody, default hosted family data, and child activity custody remain unclaimed.',
] as const;

function executionContinuationReadiness(
  target: ProductionSupportStatusBackendExecutionContinuationTarget,
  continuationState: ProductionSupportStatusBackendExecutionContinuationState
): ProductionSupportStatusBackendExecutionContinuationRow {
  return {
    schemaVersion: 'production-support-status-backend-execution-continuation-proof',
    target,
    continuationState,
    sourceProofRefs: RequiredExecutionContinuationSourceProofs,
    durableQueueRuntimeRef: executionContinuationReference(target, continuationState, 'durable-queue-runtime'),
    payloadCustodyBoundaryRef: executionContinuationReference(target, continuationState, 'payload-custody'),
    redactionManifestRef: executionContinuationReference(target, continuationState, 'redaction-manifest'),
    manualProofRef: executionContinuationReference(target, continuationState, 'manual-proof'),
    supportSafeDataClasses: ExecutionContinuationSupportSafeDataClasses,
    forbiddenDataClasses: ForbiddenExecutionContinuationDataClasses,
    statusBackendExecutionState:
      continuationState === 'backend-unavailable' ? 'backend-unavailable' : 'manual-required',
    durableQueueStorageState: continuationState === 'durable-storage-required' ? 'manual-required' : 'not-implemented',
    retryWorkerExecutionState: continuationState === 'runtime-worker-required' ? 'manual-required' : 'not-implemented',
    auditPersistenceState: continuationState === 'execution-preflight-ready' ? 'manual-required' : 'not-implemented',
    deadLetterPayloadCustodyState:
      continuationState === 'payload-custody-required' ? 'manual-required' : 'not-implemented',
    statusBackendPayloadCustodyState:
      continuationState === 'payload-custody-required' ? 'manual-required' : 'not-implemented',
    redactionManifestExecutionState:
      continuationState === 'redaction-manifest-required' ? 'manual-required' : 'not-implemented',
    publicRuntimeExecutionState: 'not-implemented',
    providerExecutionState: 'not-implemented',
    supportBackendUploadExecutionState:
      target === 'support-upload-status-backend-execution-continuation' ? 'manual-required' : 'not-implemented',
    legalDisclosureExecutionState:
      target === 'privacy-legal-status-backend-execution-continuation' ? 'manual-required' : 'not-implemented',
    defaultHostedFamilyDataState: 'not-implemented',
    childActivityCustodyState: 'not-implemented',
    manualRequirement: Schema.decodeUnknownSync(ExecutionContinuationManualRequirementSchema)(
      `${target}-${continuationState}-requires-real-status-backend-worker-durable-storage-retry-audit-payload-custody-redaction-manifest-and-operator-proof-before-product-claim`
    ),
  };
}

function executionContinuationReference(
  target: ProductionSupportStatusBackendExecutionContinuationTarget,
  continuationState: ProductionSupportStatusBackendExecutionContinuationState,
  referenceKind: 'durable-queue-runtime' | 'payload-custody' | 'redaction-manifest' | 'manual-proof'
): ProductionSupportStatusBackendExecutionContinuationRow['durableQueueRuntimeRef'] {
  return Schema.decodeUnknownSync(ExecutionContinuationReferenceSchema)(
    `production-support-status-backend-execution-continuation-${referenceKind}-${target}-${continuationState}`
  );
}

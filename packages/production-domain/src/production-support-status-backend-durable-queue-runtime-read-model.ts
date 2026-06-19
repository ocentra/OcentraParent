import { Schema } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  ProductionSupportStatusBackendDurableQueueRuntimeProofSchema,
  type ProductionSupportStatusBackendDurableQueueRuntimeRow,
  type ProductionSupportStatusBackendDurableQueueRuntimeState,
  type ProductionSupportStatusBackendDurableQueueRuntimeTarget,
} from './production-support-status-backend-durable-queue-runtime-proof';
import {
  DurableQueueRuntimeManualRequirementSchema,
  DurableQueueRuntimeReferenceSchema,
  DurableQueueRuntimeSupportSafeDataClasses,
  ForbiddenDurableQueueRuntimeDataClasses,
  RequiredDurableQueueRuntimeNonClaims,
  RequiredDurableQueueRuntimeSourceProofs,
  RequiredDurableQueueRuntimeStates,
  RequiredDurableQueueRuntimeTargets,
} from './production-support-status-backend-durable-queue-runtime-values';

export const ProductionSupportStatusBackendDurableQueueRuntimeReadModel =
  ProductionSupportStatusBackendDurableQueueRuntimeProofSchema.parse({
    schemaVersion: 'production-support-status-backend-durable-queue-runtime-proof',
    sourceContractRefs: RequiredDurableQueueRuntimeSourceProofs,
    rows: RequiredDurableQueueRuntimeTargets.flatMap((target) =>
      RequiredDurableQueueRuntimeStates.map((runtimeBoundaryState) =>
        durableQueueRuntimeReadiness(target, runtimeBoundaryState)
      )
    ),
    nonClaims: RequiredDurableQueueRuntimeNonClaims,
    statusBackendExecutionClaim: 'manual-required',
    durableQueueStorageClaim: 'manual-required',
    retryWorkerExecutionClaim: 'manual-required',
    auditPersistenceClaim: 'manual-required',
    deadLetterPayloadCustodyClaim: 'manual-required',
    publicRuntimeExecutionClaim: 'not-implemented',
    providerExecutionClaim: 'not-implemented',
    supportBackendUploadExecutionClaim: 'manual-required',
    accountLookupExecutionClaim: 'manual-required',
    billingProviderContactClaim: 'manual-required',
    legalDisclosureExecutionClaim: 'manual-required',
    remoteSupportSessionClaim: 'not-implemented',
    productionSlaClaim: 'not-implemented',
    providerSecretCustodyClaim: 'not-implemented',
    childActivityCustodyClaim: 'not-implemented',
    updatedAt: Schema.decodeUnknownSync(ParentTimestampSchema)('2026-06-07T20:21:22.308Z'),
  });

export const ProductionSupportStatusBackendDurableQueueRuntimeKnownGaps = [
  'Durable queue runtime readiness proves source-backed queue storage, retry-worker, audit-persistence, dead-letter, runtime-execution, and runtime-closure refs can be composed without claiming live status backend execution.',
  'Durable queue storage, retry-worker execution, audit persistence, and dead-letter payload custody remain manual-required until a real status backend worker and durable store exist.',
  'Public runtime execution, provider execution, support backend upload execution, account lookup, billing provider contact, legal disclosure execution, remote support sessions, production SLA, provider-secret custody, and child activity custody remain unclaimed.',
] as const;

function durableQueueRuntimeReadiness(
  target: ProductionSupportStatusBackendDurableQueueRuntimeTarget,
  runtimeBoundaryState: ProductionSupportStatusBackendDurableQueueRuntimeState
): ProductionSupportStatusBackendDurableQueueRuntimeRow {
  return {
    schemaVersion: 'production-support-status-backend-durable-queue-runtime-proof',
    target,
    runtimeBoundaryState,
    sourceProofRefs: RequiredDurableQueueRuntimeSourceProofs,
    queueBoundaryRef: durableQueueRuntimeReference(target, runtimeBoundaryState, 'queue-boundary'),
    retryWorkerRef: durableQueueRuntimeReference(target, runtimeBoundaryState, 'retry-worker'),
    auditPersistenceRef: durableQueueRuntimeReference(target, runtimeBoundaryState, 'audit-persistence'),
    deadLetterRef: durableQueueRuntimeReference(target, runtimeBoundaryState, 'dead-letter'),
    runtimeExecutionRef: durableQueueRuntimeReference(target, runtimeBoundaryState, 'runtime-execution'),
    supportSafeDataClasses: DurableQueueRuntimeSupportSafeDataClasses,
    forbiddenDataClasses: ForbiddenDurableQueueRuntimeDataClasses,
    durableQueueStorageState:
      runtimeBoundaryState === 'backend-unavailable' ? 'backend-unavailable' : 'manual-required',
    retryWorkerState: runtimeBoundaryState === 'retry-worker-boundary-ready' ? 'manual-required' : 'not-implemented',
    auditPersistenceState:
      runtimeBoundaryState === 'audit-persistence-boundary-ready' ? 'manual-required' : 'not-implemented',
    deadLetterPayloadCustodyState:
      runtimeBoundaryState === 'dead-letter-runtime-boundary-ready' ? 'manual-required' : 'not-implemented',
    statusBackendExecutionState: 'manual-required',
    publicRuntimeExecutionState: 'not-implemented',
    providerExecutionState: 'not-implemented',
    supportBackendUploadState:
      target === 'support-upload-status-backend-durable-queue-runtime' ? 'manual-required' : 'not-implemented',
    legalDisclosureExecutionState:
      target === 'privacy-legal-status-backend-durable-queue-runtime' ? 'manual-required' : 'not-implemented',
    childActivityCustodyState: 'not-implemented',
    manualRequirement: Schema.decodeUnknownSync(DurableQueueRuntimeManualRequirementSchema)(
      `${target}-${runtimeBoundaryState}-requires-real-status-backend-worker-durable-store-retry-worker-audit-persistence-and-dead-letter-custody-proof-before-product-claim`
    ),
  };
}

function durableQueueRuntimeReference(
  target: ProductionSupportStatusBackendDurableQueueRuntimeTarget,
  runtimeBoundaryState: ProductionSupportStatusBackendDurableQueueRuntimeState,
  referenceKind: 'queue-boundary' | 'retry-worker' | 'audit-persistence' | 'dead-letter' | 'runtime-execution'
): ProductionSupportStatusBackendDurableQueueRuntimeRow['queueBoundaryRef'] {
  return Schema.decodeUnknownSync(DurableQueueRuntimeReferenceSchema)(
    `production-support-status-backend-durable-queue-runtime-${referenceKind}-${target}-${runtimeBoundaryState}`
  );
}

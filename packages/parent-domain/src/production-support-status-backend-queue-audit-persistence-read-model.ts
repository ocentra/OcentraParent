import { Schema } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';
import {
  ProductionSupportStatusBackendQueueAuditPersistenceProofSchema,
  type ProductionSupportStatusBackendQueueAuditPersistenceRow,
  type ProductionSupportStatusBackendQueueAuditPersistenceState,
  type ProductionSupportStatusBackendQueueAuditPersistenceTarget,
} from './production-support-status-backend-queue-audit-persistence-proof';
import {
  ForbiddenQueueAuditPersistenceDataClasses,
  QueueAuditPersistenceManualRequirementSchema,
  QueueAuditPersistenceReferenceSchema,
  RequiredQueueAuditPersistenceNonClaims,
  RequiredQueueAuditPersistenceStates,
  RequiredQueueAuditPersistenceTargets,
} from './production-support-status-backend-queue-audit-persistence-values';

const SafeDataClassesByTarget: Record<
  ProductionSupportStatusBackendQueueAuditPersistenceTarget,
  ProductionSupportStatusBackendQueueAuditPersistenceRow['supportSafeDataClasses']
> = {
  'support-runbook-status-queue-audit-persistence': [
    'publication-status-label',
    'support-runbook-status',
    'status-backend-queue-reference',
    'retry-policy-reference',
    'audit-reference',
    'manual-proof-reference',
  ],
  'incident-status-queue-audit-persistence': [
    'publication-status-label',
    'incident-status',
    'status-backend-queue-reference',
    'retry-policy-reference',
    'audit-reference',
    'manual-proof-reference',
  ],
  'public-support-contact-status-queue-audit-persistence': [
    'publication-status-label',
    'public-support-contact-status',
    'status-backend-queue-reference',
    'retry-policy-reference',
    'audit-reference',
    'manual-proof-reference',
  ],
  'support-upload-status-queue-audit-persistence': [
    'publication-status-label',
    'support-upload-status-summary',
    'status-backend-queue-reference',
    'retry-policy-reference',
    'audit-reference',
    'manual-proof-reference',
  ],
  'privacy-legal-status-queue-audit-persistence': [
    'publication-status-label',
    'privacy-policy-status',
    'legal-review-status',
    'status-backend-queue-reference',
    'retry-policy-reference',
    'audit-reference',
    'manual-proof-reference',
  ],
  'account-billing-status-queue-audit-persistence': [
    'publication-status-label',
    'account-status-summary',
    'billing-support-status',
    'status-backend-queue-reference',
    'retry-policy-reference',
    'audit-reference',
    'manual-proof-reference',
  ],
};

export const ProductionSupportStatusBackendQueueAuditPersistenceReadModel =
  ProductionSupportStatusBackendQueueAuditPersistenceProofSchema.parse({
    schemaVersion: 'production-support-status-backend-queue-audit-persistence-proof',
    rows: RequiredQueueAuditPersistenceTargets.flatMap((target) =>
      RequiredQueueAuditPersistenceStates.map((readinessState) =>
        queueAuditPersistenceReadiness(target, readinessState)
      )
    ),
    nonClaims: RequiredQueueAuditPersistenceNonClaims,
    statusBackendExecutionClaim: 'manual-required',
    durableQueueStorageClaim: 'manual-required',
    retryWorkerExecutionClaim: 'manual-required',
    auditPersistenceClaim: 'manual-required',
    publicRuntimeExecutionClaim: 'not-implemented',
    providerExecutionClaim: 'not-implemented',
    supportBackendUploadExecutionClaim: 'manual-required',
    accountLookupExecutionClaim: 'manual-required',
    billingProviderContactClaim: 'manual-required',
    productionSlaClaim: 'not-implemented',
    legalDisclosureExecutionClaim: 'manual-required',
    childActivityCustodyClaim: 'not-implemented',
    updatedAt: Schema.decodeUnknownSync(ParentTimestampSchema)('2026-06-06T12:06:00.000Z'),
  });

export const ProductionSupportStatusBackendQueueAuditPersistenceKnownGaps = [
  'Status backend queue audit persistence remains a deterministic contract proof; no durable queue storage is implemented.',
  'Retry worker execution and audit persistence remain manual-required until a real status backend worker, durable store, and audit sink exist.',
  'Public runtime execution, provider execution, support upload execution, account lookup, billing provider contact, legal disclosure execution, remote support sessions, production SLA, provider-secret custody, status backend payload custody, and child activity custody remain unclaimed.',
] as const;

function queueAuditPersistenceReadiness(
  target: ProductionSupportStatusBackendQueueAuditPersistenceTarget,
  readinessState: ProductionSupportStatusBackendQueueAuditPersistenceState
): ProductionSupportStatusBackendQueueAuditPersistenceRow {
  return {
    schemaVersion: 'production-support-status-backend-queue-audit-persistence-proof',
    target,
    sourceProof: sourceProofForTarget(target),
    readinessState,
    sourceContractState: 'source-contract-ready',
    statusContractState: 'status-contract-ready',
    durableQueueStorageState: storageStateForReadiness(readinessState),
    retryWorkerState: retryStateForReadiness(readinessState),
    auditPersistenceState: auditStateForReadiness(readinessState),
    backendExecutionState: backendExecutionStateForReadiness(readinessState),
    publicRuntimeExecutionState: 'not-implemented',
    providerExecutionState: 'not-implemented',
    supportBackendUploadState:
      target === 'support-upload-status-queue-audit-persistence' ? 'manual-required' : 'not-implemented',
    supportSafeDataClasses: SafeDataClassesByTarget[target],
    forbiddenDataClasses: ForbiddenQueueAuditPersistenceDataClasses,
    queueReference: Schema.decodeUnknownSync(QueueAuditPersistenceReferenceSchema)(
      `production-support-status-backend-queue-audit-persistence-${target}-${readinessState}`
    ),
    retryReference: Schema.decodeUnknownSync(QueueAuditPersistenceReferenceSchema)(
      `production-support-status-backend-queue-audit-persistence-retry-${target}-${readinessState}`
    ),
    auditReference: Schema.decodeUnknownSync(QueueAuditPersistenceReferenceSchema)(
      `production-support-status-backend-queue-audit-persistence-audit-${target}-${readinessState}`
    ),
    manualRequirement: Schema.decodeUnknownSync(QueueAuditPersistenceManualRequirementSchema)(
      `${target}-${readinessState}-requires-durable-queue-retry-worker-and-audit-persistence-proof-before-product-claim`
    ),
  };
}

function sourceProofForTarget(
  target: ProductionSupportStatusBackendQueueAuditPersistenceTarget
): ProductionSupportStatusBackendQueueAuditPersistenceRow['sourceProof'] {
  if (target === 'privacy-legal-status-queue-audit-persistence') {
    return 'production-support-privacy-legal-disclosure-status-proof';
  }
  if (target === 'public-support-contact-status-queue-audit-persistence') {
    return 'public-support-contact-status-proof';
  }
  if (target === 'support-upload-status-queue-audit-persistence') {
    return 'production-support-publication-runtime-readiness-proof';
  }
  return 'production-support-status-backend-execution-queue-proof';
}

function storageStateForReadiness(
  readinessState: ProductionSupportStatusBackendQueueAuditPersistenceState
): ProductionSupportStatusBackendQueueAuditPersistenceRow['durableQueueStorageState'] {
  return readinessState === 'backend-unavailable' ? 'backend-unavailable' : 'manual-required';
}

function retryStateForReadiness(
  readinessState: ProductionSupportStatusBackendQueueAuditPersistenceState
): ProductionSupportStatusBackendQueueAuditPersistenceRow['retryWorkerState'] {
  return readinessState === 'retry-scheduled' || readinessState === 'failed' ? 'manual-required' : 'not-implemented';
}

function auditStateForReadiness(
  readinessState: ProductionSupportStatusBackendQueueAuditPersistenceState
): ProductionSupportStatusBackendQueueAuditPersistenceRow['auditPersistenceState'] {
  return readinessState === 'audit-ready' ? 'manual-required' : 'not-implemented';
}

function backendExecutionStateForReadiness(
  readinessState: ProductionSupportStatusBackendQueueAuditPersistenceState
): ProductionSupportStatusBackendQueueAuditPersistenceRow['backendExecutionState'] {
  return readinessState === 'backend-unavailable' ? 'backend-unavailable' : 'manual-required';
}

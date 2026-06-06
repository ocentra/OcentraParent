import { Schema } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';
import {
  ProductionSupportStatusBackendDeadLetterProofSchema,
  type ProductionSupportStatusBackendDeadLetterRow,
  type ProductionSupportStatusBackendDeadLetterState,
  type ProductionSupportStatusBackendDeadLetterTarget,
} from './production-support-status-backend-dead-letter-proof';
import {
  ForbiddenStatusBackendDeadLetterDataClasses,
  RequiredStatusBackendDeadLetterNonClaims,
  RequiredStatusBackendDeadLetterStates,
  RequiredStatusBackendDeadLetterTargets,
  StatusBackendDeadLetterManualRequirementSchema,
  StatusBackendDeadLetterReferenceSchema,
} from './production-support-status-backend-dead-letter-values';

const SafeDataClassesByTarget: Record<
  ProductionSupportStatusBackendDeadLetterTarget,
  ProductionSupportStatusBackendDeadLetterRow['supportSafeDataClasses']
> = {
  'support-runbook-status-dead-letter': [
    'publication-status-label',
    'support-runbook-status',
    'status-backend-queue-reference',
    'dead-letter-reference',
    'retry-policy-reference',
    'audit-reference',
    'manual-proof-reference',
  ],
  'incident-status-dead-letter': [
    'publication-status-label',
    'incident-status',
    'status-backend-queue-reference',
    'dead-letter-reference',
    'retry-policy-reference',
    'audit-reference',
    'manual-proof-reference',
  ],
  'public-support-contact-status-dead-letter': [
    'publication-status-label',
    'public-support-contact-status',
    'status-backend-queue-reference',
    'dead-letter-reference',
    'retry-policy-reference',
    'audit-reference',
    'manual-proof-reference',
  ],
  'support-upload-status-dead-letter': [
    'publication-status-label',
    'support-upload-status-summary',
    'status-backend-queue-reference',
    'dead-letter-reference',
    'retry-policy-reference',
    'audit-reference',
    'manual-proof-reference',
  ],
  'privacy-legal-status-dead-letter': [
    'publication-status-label',
    'privacy-policy-status',
    'legal-review-status',
    'status-backend-queue-reference',
    'dead-letter-reference',
    'retry-policy-reference',
    'audit-reference',
    'manual-proof-reference',
  ],
  'account-billing-status-dead-letter': [
    'publication-status-label',
    'account-status-summary',
    'billing-support-status',
    'status-backend-queue-reference',
    'dead-letter-reference',
    'retry-policy-reference',
    'audit-reference',
    'manual-proof-reference',
  ],
};

export const ProductionSupportStatusBackendDeadLetterReadModel =
  ProductionSupportStatusBackendDeadLetterProofSchema.parse({
    schemaVersion: 'production-support-status-backend-dead-letter-proof',
    rows: RequiredStatusBackendDeadLetterTargets.flatMap((target) =>
      RequiredStatusBackendDeadLetterStates.map((deadLetterState) =>
        statusBackendDeadLetterReadiness(target, deadLetterState)
      )
    ),
    nonClaims: RequiredStatusBackendDeadLetterNonClaims,
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
    productionSlaClaim: 'not-implemented',
    legalDisclosureExecutionClaim: 'manual-required',
    childActivityCustodyClaim: 'not-implemented',
    updatedAt: Schema.decodeUnknownSync(ParentTimestampSchema)('2026-06-06T14:34:00.000Z'),
  });

export const ProductionSupportStatusBackendDeadLetterKnownGaps = [
  'Status backend dead-letter handling remains a deterministic contract proof; no durable dead-letter store or worker exists.',
  'Dead-letter payload custody, retry worker execution, audit persistence, and backend execution remain manual-required until a real status backend worker and durable store exist.',
  'Public runtime execution, provider execution, support upload execution, account lookup, billing provider contact, legal disclosure execution, remote support sessions, production SLA, provider-secret custody, and child activity custody remain unclaimed.',
] as const;

function statusBackendDeadLetterReadiness(
  target: ProductionSupportStatusBackendDeadLetterTarget,
  deadLetterState: ProductionSupportStatusBackendDeadLetterState
): ProductionSupportStatusBackendDeadLetterRow {
  return {
    schemaVersion: 'production-support-status-backend-dead-letter-proof',
    target,
    sourceProof: sourceProofForTarget(target),
    deadLetterState,
    sourceContractState: 'source-contract-ready',
    statusContractState: 'status-contract-ready',
    durableQueueStorageState: storageStateForDeadLetter(deadLetterState),
    retryWorkerState: retryStateForDeadLetter(deadLetterState),
    auditPersistenceState: auditStateForDeadLetter(deadLetterState),
    backendExecutionState: backendExecutionStateForDeadLetter(deadLetterState),
    deadLetterPayloadCustodyState: payloadCustodyStateForDeadLetter(deadLetterState),
    publicRuntimeExecutionState: 'not-implemented',
    providerExecutionState: 'not-implemented',
    supportBackendUploadState: target === 'support-upload-status-dead-letter' ? 'manual-required' : 'not-implemented',
    supportSafeDataClasses: SafeDataClassesByTarget[target],
    forbiddenDataClasses: ForbiddenStatusBackendDeadLetterDataClasses,
    queueReference: Schema.decodeUnknownSync(StatusBackendDeadLetterReferenceSchema)(
      `production-support-status-backend-dead-letter-queue-${target}-${deadLetterState}`
    ),
    deadLetterReference: Schema.decodeUnknownSync(StatusBackendDeadLetterReferenceSchema)(
      `production-support-status-backend-dead-letter-${target}-${deadLetterState}`
    ),
    retryReference: Schema.decodeUnknownSync(StatusBackendDeadLetterReferenceSchema)(
      `production-support-status-backend-dead-letter-retry-${target}-${deadLetterState}`
    ),
    auditReference: Schema.decodeUnknownSync(StatusBackendDeadLetterReferenceSchema)(
      `production-support-status-backend-dead-letter-audit-${target}-${deadLetterState}`
    ),
    manualRequirement: Schema.decodeUnknownSync(StatusBackendDeadLetterManualRequirementSchema)(
      `${target}-${deadLetterState}-requires-dead-letter-store-worker-audit-and-manual-triage-proof-before-product-claim`
    ),
  };
}

function sourceProofForTarget(
  target: ProductionSupportStatusBackendDeadLetterTarget
): ProductionSupportStatusBackendDeadLetterRow['sourceProof'] {
  if (target === 'privacy-legal-status-dead-letter') {
    return 'production-support-privacy-legal-disclosure-status-proof';
  }
  if (target === 'public-support-contact-status-dead-letter') {
    return 'public-support-contact-status-proof';
  }
  if (target === 'support-upload-status-dead-letter') {
    return 'production-support-publication-runtime-readiness-proof';
  }
  return 'production-support-status-backend-queue-audit-persistence-proof';
}

function storageStateForDeadLetter(
  deadLetterState: ProductionSupportStatusBackendDeadLetterState
): ProductionSupportStatusBackendDeadLetterRow['durableQueueStorageState'] {
  return deadLetterState === 'backend-unavailable' ? 'backend-unavailable' : 'manual-required';
}

function retryStateForDeadLetter(
  deadLetterState: ProductionSupportStatusBackendDeadLetterState
): ProductionSupportStatusBackendDeadLetterRow['retryWorkerState'] {
  return deadLetterState === 'retry-blocked' || deadLetterState === 'failed' ? 'manual-required' : 'not-implemented';
}

function auditStateForDeadLetter(
  deadLetterState: ProductionSupportStatusBackendDeadLetterState
): ProductionSupportStatusBackendDeadLetterRow['auditPersistenceState'] {
  return deadLetterState === 'triage-ready' || deadLetterState === 'dead-lettered'
    ? 'manual-required'
    : 'not-implemented';
}

function backendExecutionStateForDeadLetter(
  deadLetterState: ProductionSupportStatusBackendDeadLetterState
): ProductionSupportStatusBackendDeadLetterRow['backendExecutionState'] {
  return deadLetterState === 'backend-unavailable' ? 'backend-unavailable' : 'manual-required';
}

function payloadCustodyStateForDeadLetter(
  deadLetterState: ProductionSupportStatusBackendDeadLetterState
): ProductionSupportStatusBackendDeadLetterRow['deadLetterPayloadCustodyState'] {
  return deadLetterState === 'backend-unavailable' ? 'backend-unavailable' : 'manual-required';
}

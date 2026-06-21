import { Schema } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  ProductionSupportStatusBackendExecutionQueueProofSchema,
  type ProductionSupportStatusBackendExecutionQueueRow,
  type ProductionSupportStatusBackendExecutionQueueState,
  type ProductionSupportStatusBackendExecutionQueueTarget,
} from './production-support-status-backend-execution-queue-proof';
import {
  ForbiddenStatusBackendExecutionQueueDataClasses,
  RequiredStatusBackendExecutionQueueNonClaims,
  RequiredStatusBackendExecutionQueueStates,
  RequiredStatusBackendExecutionQueueTargets,
  StatusBackendExecutionQueueManualRequirementSchema,
  StatusBackendExecutionQueueReferenceSchema,
} from './production-support-status-backend-execution-queue-values';

const SafeDataClassesByTarget: Record<
  ProductionSupportStatusBackendExecutionQueueTarget,
  ProductionSupportStatusBackendExecutionQueueRow['supportSafeDataClasses']
> = {
  'support-runbook-status-backend-queue': [
    'publication-status-label',
    'support-runbook-status',
    'status-backend-queue-reference',
    'retry-policy-reference',
    'audit-reference',
    'manual-proof-reference',
  ],
  'incident-status-backend-queue': [
    'publication-status-label',
    'incident-status',
    'status-backend-queue-reference',
    'retry-policy-reference',
    'audit-reference',
    'manual-proof-reference',
  ],
  'public-support-contact-status-backend-queue': [
    'publication-status-label',
    'public-support-contact-status',
    'status-backend-queue-reference',
    'retry-policy-reference',
    'audit-reference',
    'manual-proof-reference',
  ],
  'support-upload-status-backend-queue': [
    'publication-status-label',
    'support-upload-status-summary',
    'status-backend-queue-reference',
    'retry-policy-reference',
    'audit-reference',
    'manual-proof-reference',
  ],
  'privacy-legal-status-backend-queue': [
    'publication-status-label',
    'privacy-policy-status',
    'legal-review-status',
    'status-backend-queue-reference',
    'retry-policy-reference',
    'audit-reference',
    'manual-proof-reference',
  ],
  'account-billing-status-backend-queue': [
    'publication-status-label',
    'account-status-summary',
    'billing-support-status',
    'status-backend-queue-reference',
    'retry-policy-reference',
    'audit-reference',
    'manual-proof-reference',
  ],
};

export const ProductionSupportStatusBackendExecutionQueueReadModel =
  ProductionSupportStatusBackendExecutionQueueProofSchema.parse({
    schemaVersion: 'production-support-status-backend-execution-queue-proof',
    rows: RequiredStatusBackendExecutionQueueTargets.flatMap((target) =>
      RequiredStatusBackendExecutionQueueStates.map((queueState) => statusBackendExecutionQueue(target, queueState))
    ),
    nonClaims: RequiredStatusBackendExecutionQueueNonClaims,
    statusBackendExecutionClaim: 'manual-required',
    publicRuntimeExecutionClaim: 'not-implemented',
    providerExecutionClaim: 'not-implemented',
    supportBackendUploadExecutionClaim: 'manual-required',
    accountLookupExecutionClaim: 'manual-required',
    billingProviderContactClaim: 'manual-required',
    productionSlaClaim: 'not-implemented',
    legalDisclosureExecutionClaim: 'manual-required',
    childActivityCustodyClaim: 'not-implemented',
    updatedAt: Schema.decodeUnknownSync(ParentTimestampSchema)('2026-06-06T10:58:00.000Z'),
  });

export const ProductionSupportStatusBackendExecutionQueueKnownGaps = [
  'Status backend execution queue remains a deterministic contract proof; no status backend worker executes these rows.',
  'Durable queue storage, retry workers, audit persistence, public runtime execution, provider execution, support upload execution, and production SLA remain manual-required or unimplemented.',
  'Account lookup, billing provider contact, legal disclosure execution, remote support sessions, provider-secret custody, status backend payload custody, and child activity custody remain unclaimed.',
] as const;

function statusBackendExecutionQueue(
  target: ProductionSupportStatusBackendExecutionQueueTarget,
  queueState: ProductionSupportStatusBackendExecutionQueueState
): ProductionSupportStatusBackendExecutionQueueRow {
  return {
    schemaVersion: 'production-support-status-backend-execution-queue-proof',
    target,
    sourceProof: sourceProofForTarget(target),
    queueState,
    sourceContractState: 'source-contract-ready',
    statusContractState: 'status-contract-ready',
    authorizationState: authorizationStateForQueueState(queueState),
    queueAdapterState: queueAdapterStateForQueueState(queueState),
    backendExecutionState: backendExecutionStateForQueueState(queueState),
    retryState: retryStateForQueueState(queueState),
    auditState: auditStateForQueueState(queueState),
    publicRuntimeExecutionState: 'not-implemented',
    providerExecutionState: 'not-implemented',
    supportBackendUploadState: target === 'support-upload-status-backend-queue' ? 'manual-required' : 'not-implemented',
    supportSafeDataClasses: SafeDataClassesByTarget[target],
    forbiddenDataClasses: ForbiddenStatusBackendExecutionQueueDataClasses,
    queueReference: Schema.decodeUnknownSync(StatusBackendExecutionQueueReferenceSchema)(
      `production-support-status-backend-execution-queue-${target}-${queueState}`
    ),
    retryReference: Schema.decodeUnknownSync(StatusBackendExecutionQueueReferenceSchema)(
      `production-support-status-backend-execution-queue-retry-${target}-${queueState}`
    ),
    auditReference: Schema.decodeUnknownSync(StatusBackendExecutionQueueReferenceSchema)(
      `production-support-status-backend-execution-queue-audit-${target}-${queueState}`
    ),
    manualRequirement: Schema.decodeUnknownSync(StatusBackendExecutionQueueManualRequirementSchema)(
      `${target}-${queueState}-requires-real-status-backend-queue-worker-audit-and-retry-proof-before-product-claim`
    ),
  };
}

function sourceProofForTarget(
  target: ProductionSupportStatusBackendExecutionQueueTarget
): ProductionSupportStatusBackendExecutionQueueRow['sourceProof'] {
  if (target === 'privacy-legal-status-backend-queue') {
    return 'production-support-privacy-legal-disclosure-status-proof';
  }
  if (target === 'public-support-contact-status-backend-queue') {
    return 'public-support-contact-status-proof';
  }
  if (target === 'support-upload-status-backend-queue') {
    return 'production-support-publication-runtime-readiness-proof';
  }
  return 'production-support-status-backend-public-runtime-followthrough-proof';
}

function authorizationStateForQueueState(
  queueState: ProductionSupportStatusBackendExecutionQueueState
): ProductionSupportStatusBackendExecutionQueueRow['authorizationState'] {
  return queueState === 'requested' ? 'requested' : 'authorized';
}

function queueAdapterStateForQueueState(
  queueState: ProductionSupportStatusBackendExecutionQueueState
): ProductionSupportStatusBackendExecutionQueueRow['queueAdapterState'] {
  return queueState === 'backend-unavailable' ? 'backend-unavailable' : queueState;
}

function backendExecutionStateForQueueState(
  queueState: ProductionSupportStatusBackendExecutionQueueState
): ProductionSupportStatusBackendExecutionQueueRow['backendExecutionState'] {
  return queueState === 'backend-unavailable' ? 'backend-unavailable' : 'manual-required';
}

function retryStateForQueueState(
  queueState: ProductionSupportStatusBackendExecutionQueueState
): ProductionSupportStatusBackendExecutionQueueRow['retryState'] {
  return queueState === 'failed' || queueState === 'backend-unavailable' ? 'retry-required' : 'manual-required';
}

function auditStateForQueueState(
  queueState: ProductionSupportStatusBackendExecutionQueueState
): ProductionSupportStatusBackendExecutionQueueRow['auditState'] {
  return queueState === 'requested' ? 'manual-required' : 'audit-ready';
}

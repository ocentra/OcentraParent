import { Schema } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
import {
  ProductionSupportStatusBackendRuntimeExecutionProofSchema,
  type ProductionSupportStatusBackendRuntimeExecutionRow,
  type ProductionSupportStatusBackendRuntimeExecutionState,
  type ProductionSupportStatusBackendRuntimeExecutionTarget,
} from './production-support-status-backend-runtime-execution-proof';
import {
  ForbiddenRuntimeExecutionDataClasses,
  RequiredRuntimeExecutionNonClaims,
  RequiredRuntimeExecutionStates,
  RequiredRuntimeExecutionTargets,
  RuntimeExecutionManualRequirementSchema,
  RuntimeExecutionReferenceSchema,
} from './production-support-status-backend-runtime-execution-values';

const SafeDataClassesByTarget: Record<
  ProductionSupportStatusBackendRuntimeExecutionTarget,
  ProductionSupportStatusBackendRuntimeExecutionRow['supportSafeDataClasses']
> = {
  'support-runbook-status-runtime-execution': [
    'publication-status-label',
    'support-runbook-status',
    'status-backend-queue-reference',
    'retry-policy-reference',
    'audit-reference',
    'dead-letter-reference',
    'runtime-evidence-reference',
    'manual-proof-reference',
  ],
  'incident-status-runtime-execution': [
    'publication-status-label',
    'incident-status',
    'status-backend-queue-reference',
    'retry-policy-reference',
    'audit-reference',
    'dead-letter-reference',
    'runtime-evidence-reference',
    'manual-proof-reference',
  ],
  'public-support-contact-status-runtime-execution': [
    'publication-status-label',
    'public-support-contact-status',
    'status-backend-queue-reference',
    'retry-policy-reference',
    'audit-reference',
    'dead-letter-reference',
    'runtime-evidence-reference',
    'manual-proof-reference',
  ],
  'support-upload-status-runtime-execution': [
    'publication-status-label',
    'support-upload-status-summary',
    'status-backend-queue-reference',
    'retry-policy-reference',
    'audit-reference',
    'dead-letter-reference',
    'runtime-evidence-reference',
    'manual-proof-reference',
  ],
  'privacy-legal-status-runtime-execution': [
    'publication-status-label',
    'privacy-policy-status',
    'legal-review-status',
    'status-backend-queue-reference',
    'retry-policy-reference',
    'audit-reference',
    'dead-letter-reference',
    'runtime-evidence-reference',
    'manual-proof-reference',
  ],
  'account-billing-status-runtime-execution': [
    'publication-status-label',
    'account-status-summary',
    'billing-support-status',
    'status-backend-queue-reference',
    'retry-policy-reference',
    'audit-reference',
    'dead-letter-reference',
    'runtime-evidence-reference',
    'manual-proof-reference',
  ],
};

export const ProductionSupportStatusBackendRuntimeExecutionReadModel =
  ProductionSupportStatusBackendRuntimeExecutionProofSchema.parse({
    schemaVersion: 'production-support-status-backend-runtime-execution-proof',
    rows: RequiredRuntimeExecutionTargets.flatMap((target) =>
      RequiredRuntimeExecutionStates.map((readinessState) => runtimeExecutionReadiness(target, readinessState))
    ),
    nonClaims: RequiredRuntimeExecutionNonClaims,
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
    updatedAt: Schema.decodeUnknownSync(ParentTimestampSchema)('2026-06-07T06:40:00.000Z'),
  });

export const ProductionSupportStatusBackendRuntimeExecutionKnownGaps = [
  'Status backend runtime execution remains a deterministic contract proof; no real status backend execution is implemented.',
  'Durable queue storage, retry worker execution, audit persistence, and dead-letter payload custody remain manual-required until a real status backend worker, durable store, and audit sink exist.',
  'Public runtime execution, provider execution, support upload execution, account lookup, billing provider contact, legal disclosure execution, remote support sessions, production SLA, provider-secret custody, status backend payload custody, and child activity custody remain unclaimed.',
  'Package exports expose the contract, read model, and values modules; runtime execution remains manual-required until real backend workers and persistence exist.',
] as const;

function runtimeExecutionReadiness(
  target: ProductionSupportStatusBackendRuntimeExecutionTarget,
  readinessState: ProductionSupportStatusBackendRuntimeExecutionState
): ProductionSupportStatusBackendRuntimeExecutionRow {
  return {
    schemaVersion: 'production-support-status-backend-runtime-execution-proof',
    target,
    sourceProof: sourceProofForTarget(target),
    readinessState,
    sourceContractState: 'source-contract-ready',
    statusContractState: 'status-contract-ready',
    durableQueueStorageState: durableQueueStateForReadiness(readinessState),
    retryWorkerState: retryStateForReadiness(readinessState),
    auditPersistenceState: auditStateForReadiness(readinessState),
    deadLetterState: deadLetterStateForReadiness(readinessState),
    runtimeEvidenceState: runtimeEvidenceStateForReadiness(readinessState),
    backendExecutionState: backendExecutionStateForReadiness(readinessState),
    publicRuntimeExecutionState: 'not-implemented',
    providerExecutionState: 'not-implemented',
    supportBackendUploadState:
      target === 'support-upload-status-runtime-execution' ? 'manual-required' : 'not-implemented',
    supportSafeDataClasses: SafeDataClassesByTarget[target],
    forbiddenDataClasses: ForbiddenRuntimeExecutionDataClasses,
    queueReference: runtimeExecutionReference(target, readinessState, 'queue'),
    retryReference: runtimeExecutionReference(target, readinessState, 'retry'),
    auditReference: runtimeExecutionReference(target, readinessState, 'audit'),
    deadLetterReference: runtimeExecutionReference(target, readinessState, 'dead-letter'),
    runtimeEvidenceReference: runtimeExecutionReference(target, readinessState, 'runtime-evidence'),
    manualRequirement: Schema.decodeUnknownSync(RuntimeExecutionManualRequirementSchema)(
      `${target}-${readinessState}-requires-real-status-backend-runtime-worker-durable-store-and-audit-proof-before-product-claim`
    ),
  };
}

function runtimeExecutionReference(
  target: ProductionSupportStatusBackendRuntimeExecutionTarget,
  readinessState: ProductionSupportStatusBackendRuntimeExecutionState,
  referenceKind: 'queue' | 'retry' | 'audit' | 'dead-letter' | 'runtime-evidence'
): ProductionSupportStatusBackendRuntimeExecutionRow['queueReference'] {
  return Schema.decodeUnknownSync(RuntimeExecutionReferenceSchema)(
    `production-support-status-backend-runtime-execution-${referenceKind}-${target}-${readinessState}`
  );
}

function sourceProofForTarget(
  target: ProductionSupportStatusBackendRuntimeExecutionTarget
): ProductionSupportStatusBackendRuntimeExecutionRow['sourceProof'] {
  if (target === 'privacy-legal-status-runtime-execution') {
    return 'production-support-privacy-legal-disclosure-status-proof';
  }
  if (target === 'public-support-contact-status-runtime-execution') {
    return 'public-support-contact-status-proof';
  }
  if (target === 'support-upload-status-runtime-execution') {
    return 'production-support-publication-runtime-readiness-proof';
  }
  if (target === 'account-billing-status-runtime-execution') {
    return 'production-support-status-backend-queue-audit-persistence-proof';
  }
  return 'production-support-status-backend-dead-letter-proof';
}

function durableQueueStateForReadiness(
  readinessState: ProductionSupportStatusBackendRuntimeExecutionState
): ProductionSupportStatusBackendRuntimeExecutionRow['durableQueueStorageState'] {
  return readinessState === 'backend-unavailable' ? 'backend-unavailable' : 'manual-required';
}

function retryStateForReadiness(
  readinessState: ProductionSupportStatusBackendRuntimeExecutionState
): ProductionSupportStatusBackendRuntimeExecutionRow['retryWorkerState'] {
  return readinessState === 'running' || readinessState === 'failed' ? 'manual-required' : 'not-implemented';
}

function auditStateForReadiness(
  readinessState: ProductionSupportStatusBackendRuntimeExecutionState
): ProductionSupportStatusBackendRuntimeExecutionRow['auditPersistenceState'] {
  return readinessState === 'audit-ready' ? 'manual-required' : 'not-implemented';
}

function deadLetterStateForReadiness(
  readinessState: ProductionSupportStatusBackendRuntimeExecutionState
): ProductionSupportStatusBackendRuntimeExecutionRow['deadLetterState'] {
  return readinessState === 'failed' || readinessState === 'manual-required' ? 'manual-required' : 'not-implemented';
}

function runtimeEvidenceStateForReadiness(
  readinessState: ProductionSupportStatusBackendRuntimeExecutionState
): ProductionSupportStatusBackendRuntimeExecutionRow['runtimeEvidenceState'] {
  return readinessState === 'runtime-evidence-ready' ? 'runtime-evidence-ready' : 'source-contract-ready';
}

function backendExecutionStateForReadiness(
  readinessState: ProductionSupportStatusBackendRuntimeExecutionState
): ProductionSupportStatusBackendRuntimeExecutionRow['backendExecutionState'] {
  return readinessState === 'backend-unavailable' ? 'backend-unavailable' : 'manual-required';
}

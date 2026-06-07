import { describe, expect, it } from 'vitest';
import {
  ProductionSupportStatusBackendDurableQueueRuntimeProofSchema,
  ProductionSupportStatusBackendDurableQueueRuntimeRowSchema,
  summarizeProductionSupportStatusBackendDurableQueueRuntimeRows,
} from '../src/production-support-status-backend-durable-queue-runtime-proof';
import { ProductionSupportStatusBackendDurableQueueRuntimeReadModel } from '../src/production-support-status-backend-durable-queue-runtime-read-model';

describe('production support status backend durable queue runtime proof', () => {
  acceptsDurableQueueRuntimeRows();
  rejectsDurableQueueRuntimeOverclaims();
  rejectsSensitiveDurableQueueRuntimeData();
  rejectsIncompleteDurableQueueRuntimeCoverage();
});

function acceptsDurableQueueRuntimeRows(): void {
  it('accepts each status backend target with required durable queue runtime states', () => {
    const proof = ProductionSupportStatusBackendDurableQueueRuntimeProofSchema.parse(
      ProductionSupportStatusBackendDurableQueueRuntimeReadModel
    );

    for (const targetSummary of Object.values(
      summarizeProductionSupportStatusBackendDurableQueueRuntimeRows(proof.rows)
    )) {
      expect(targetSummary).toEqual({
        'queue-storage-boundary-ready': 1,
        'retry-worker-boundary-ready': 1,
        'audit-persistence-boundary-ready': 1,
        'dead-letter-runtime-boundary-ready': 1,
        'runtime-boundary-manual-required': 1,
        'backend-unavailable': 1,
      });
    }
    expect(proof.statusBackendExecutionClaim).toBe('manual-required');
    expect(proof.durableQueueStorageClaim).toBe('manual-required');
    expect(proof.retryWorkerExecutionClaim).toBe('manual-required');
    expect(proof.auditPersistenceClaim).toBe('manual-required');
    expect(proof.deadLetterPayloadCustodyClaim).toBe('manual-required');
    expect(proof.publicRuntimeExecutionClaim).toBe('not-implemented');
    expect(proof.providerExecutionClaim).toBe('not-implemented');
    expect(proof.supportBackendUploadExecutionClaim).toBe('manual-required');
    expect(proof.accountLookupExecutionClaim).toBe('manual-required');
    expect(proof.billingProviderContactClaim).toBe('manual-required');
    expect(proof.legalDisclosureExecutionClaim).toBe('manual-required');
    expect(proof.remoteSupportSessionClaim).toBe('not-implemented');
    expect(proof.productionSlaClaim).toBe('not-implemented');
    expect(proof.providerSecretCustodyClaim).toBe('not-implemented');
    expect(proof.childActivityCustodyClaim).toBe('not-implemented');
  });
}

function rejectsDurableQueueRuntimeOverclaims(): void {
  it('rejects durable storage retry audit dead-letter and external runtime execution claims', () => {
    const row = requiredRuntime('support-runbook-status-backend-durable-queue-runtime', 'queue-storage-boundary-ready');

    expect(
      ProductionSupportStatusBackendDurableQueueRuntimeRowSchema.safeParse({
        ...row,
        durableQueueStorageState: 'persisted',
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendDurableQueueRuntimeRowSchema.safeParse({
        ...row,
        retryWorkerState: 'executed',
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendDurableQueueRuntimeRowSchema.safeParse({
        ...row,
        auditPersistenceState: 'persisted',
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendDurableQueueRuntimeRowSchema.safeParse({
        ...row,
        deadLetterPayloadCustodyState: 'persisted',
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendDurableQueueRuntimeRowSchema.safeParse({
        ...row,
        publicRuntimeExecutionState: 'executed',
      }).success
    ).toBe(false);
  });
}

function rejectsSensitiveDurableQueueRuntimeData(): void {
  it('rejects durable queue payload data or omitted child-custody exclusions', () => {
    const row = requiredRuntime('support-upload-status-backend-durable-queue-runtime', 'retry-worker-boundary-ready');

    expect(
      ProductionSupportStatusBackendDurableQueueRuntimeRowSchema.safeParse({
        ...row,
        supportSafeDataClasses: [...row.supportSafeDataClasses, 'durable-queue-payload'],
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendDurableQueueRuntimeRowSchema.safeParse({
        ...row,
        forbiddenDataClasses: row.forbiddenDataClasses.filter((dataClass) => dataClass !== 'child-activity-evidence'),
      }).success
    ).toBe(false);
  });
}

function rejectsIncompleteDurableQueueRuntimeCoverage(): void {
  it('rejects proof missing target coverage source refs or non-claims', () => {
    expect(
      ProductionSupportStatusBackendDurableQueueRuntimeProofSchema.safeParse({
        ...ProductionSupportStatusBackendDurableQueueRuntimeReadModel,
        rows: ProductionSupportStatusBackendDurableQueueRuntimeReadModel.rows.filter(
          (row) =>
            row.target !== 'privacy-legal-status-backend-durable-queue-runtime' ||
            row.runtimeBoundaryState !== 'backend-unavailable'
        ),
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendDurableQueueRuntimeProofSchema.safeParse({
        ...ProductionSupportStatusBackendDurableQueueRuntimeReadModel,
        sourceContractRefs: ProductionSupportStatusBackendDurableQueueRuntimeReadModel.sourceContractRefs.filter(
          (sourceProof) => sourceProof !== 'production-support-status-backend-dead-letter-proof'
        ),
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendDurableQueueRuntimeProofSchema.safeParse({
        ...ProductionSupportStatusBackendDurableQueueRuntimeReadModel,
        nonClaims: ProductionSupportStatusBackendDurableQueueRuntimeReadModel.nonClaims.filter(
          (nonClaim) => nonClaim !== 'no-retry-worker-execution'
        ),
      }).success
    ).toBe(false);
  });
}

function requiredRuntime(
  target:
    | 'support-runbook-status-backend-durable-queue-runtime'
    | 'support-upload-status-backend-durable-queue-runtime',
  runtimeBoundaryState: 'queue-storage-boundary-ready' | 'retry-worker-boundary-ready'
): (typeof ProductionSupportStatusBackendDurableQueueRuntimeReadModel.rows)[number] {
  const row = ProductionSupportStatusBackendDurableQueueRuntimeReadModel.rows.find(
    (entry) => entry.target === target && entry.runtimeBoundaryState === runtimeBoundaryState
  );
  if (row === undefined) {
    throw new Error(`missing durable queue runtime row: ${target} ${runtimeBoundaryState}`);
  }
  return row;
}

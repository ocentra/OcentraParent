import { describe, expect, it } from 'vitest';
import {
  ProductionSupportStatusBackendQueueAuditPersistenceProofSchema,
  ProductionSupportStatusBackendQueueAuditPersistenceRowSchema,
  summarizeProductionSupportStatusBackendQueueAuditPersistenceRows,
} from '@ocentra-parent/schema-domain/production-support-status-backend-queue-audit-persistence-proof';
import { ProductionSupportStatusBackendQueueAuditPersistenceReadModel } from '@ocentra-parent/schema-domain/production-support-status-backend-queue-audit-persistence-read-model';

describe('production support status backend queue audit persistence proof', () => {
  acceptsQueueAuditPersistenceRows();
  rejectsDurableRetryAuditAndBackendOverclaims();
  rejectsSensitiveQueueAuditPersistenceData();
  rejectsIncompleteQueueAuditPersistenceCoverage();
});

function acceptsQueueAuditPersistenceRows(): void {
  it('accepts each queue audit persistence target with required readiness states', () => {
    const proof = ProductionSupportStatusBackendQueueAuditPersistenceProofSchema.parse(
      ProductionSupportStatusBackendQueueAuditPersistenceReadModel
    );

    for (const targetSummary of Object.values(
      summarizeProductionSupportStatusBackendQueueAuditPersistenceRows(proof.rows)
    )) {
      expect(targetSummary).toEqual({
        requested: 1,
        authorized: 1,
        queued: 1,
        'retry-scheduled': 1,
        'audit-ready': 1,
        failed: 1,
        'manual-required': 1,
        'backend-unavailable': 1,
      });
    }
    expect(proof.statusBackendExecutionClaim).toBe('manual-required');
    expect(proof.durableQueueStorageClaim).toBe('manual-required');
    expect(proof.retryWorkerExecutionClaim).toBe('manual-required');
    expect(proof.auditPersistenceClaim).toBe('manual-required');
    expect(proof.publicRuntimeExecutionClaim).toBe('not-implemented');
    expect(proof.providerExecutionClaim).toBe('not-implemented');
    expect(proof.supportBackendUploadExecutionClaim).toBe('manual-required');
    expect(proof.accountLookupExecutionClaim).toBe('manual-required');
    expect(proof.billingProviderContactClaim).toBe('manual-required');
    expect(proof.productionSlaClaim).toBe('not-implemented');
    expect(proof.legalDisclosureExecutionClaim).toBe('manual-required');
    expect(proof.childActivityCustodyClaim).toBe('not-implemented');
  });
}

function rejectsDurableRetryAuditAndBackendOverclaims(): void {
  it('rejects implemented executed or persisted storage retry audit and backend states', () => {
    const runbookRow = requiredReadiness('support-runbook-status-queue-audit-persistence', 'queued');
    const incidentRow = requiredReadiness('incident-status-queue-audit-persistence', 'retry-scheduled');
    const uploadRow = requiredReadiness('support-upload-status-queue-audit-persistence', 'audit-ready');

    expect(
      ProductionSupportStatusBackendQueueAuditPersistenceRowSchema.safeParse({
        ...runbookRow,
        durableQueueStorageState: 'persisted',
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendQueueAuditPersistenceRowSchema.safeParse({
        ...incidentRow,
        retryWorkerState: 'executed',
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendQueueAuditPersistenceRowSchema.safeParse({
        ...incidentRow,
        auditPersistenceState: 'implemented',
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendQueueAuditPersistenceRowSchema.safeParse({
        ...uploadRow,
        backendExecutionState: 'executed',
      }).success
    ).toBe(false);
  });
}

function rejectsSensitiveQueueAuditPersistenceData(): void {
  it('rejects persisted audit payloads or omitted provider-secret exclusions', () => {
    const row = requiredReadiness('support-upload-status-queue-audit-persistence', 'failed');

    expect(
      ProductionSupportStatusBackendQueueAuditPersistenceRowSchema.safeParse({
        ...row,
        supportSafeDataClasses: [...row.supportSafeDataClasses, 'audit-persistence-payload'],
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendQueueAuditPersistenceRowSchema.safeParse({
        ...row,
        forbiddenDataClasses: row.forbiddenDataClasses.filter((dataClass) => dataClass !== 'provider-secrets'),
      }).success
    ).toBe(false);
  });
}

function rejectsIncompleteQueueAuditPersistenceCoverage(): void {
  it('rejects proof missing readiness coverage or durable queue storage non-claim', () => {
    expect(
      ProductionSupportStatusBackendQueueAuditPersistenceProofSchema.safeParse({
        ...ProductionSupportStatusBackendQueueAuditPersistenceReadModel,
        rows: ProductionSupportStatusBackendQueueAuditPersistenceReadModel.rows.filter(
          (row) =>
            row.target !== 'privacy-legal-status-queue-audit-persistence' ||
            row.readinessState !== 'backend-unavailable'
        ),
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendQueueAuditPersistenceProofSchema.safeParse({
        ...ProductionSupportStatusBackendQueueAuditPersistenceReadModel,
        nonClaims: ProductionSupportStatusBackendQueueAuditPersistenceReadModel.nonClaims.filter(
          (nonClaim) => nonClaim !== 'no-durable-queue-storage'
        ),
      }).success
    ).toBe(false);
  });
}

function requiredReadiness(
  target:
    | 'support-runbook-status-queue-audit-persistence'
    | 'incident-status-queue-audit-persistence'
    | 'support-upload-status-queue-audit-persistence',
  readinessState: 'queued' | 'retry-scheduled' | 'audit-ready' | 'failed'
): (typeof ProductionSupportStatusBackendQueueAuditPersistenceReadModel.rows)[number] {
  const row = ProductionSupportStatusBackendQueueAuditPersistenceReadModel.rows.find(
    (entry) => entry.target === target && entry.readinessState === readinessState
  );
  if (row === undefined) {
    throw new Error(`missing queue audit persistence row: ${target} ${readinessState}`);
  }
  return row;
}

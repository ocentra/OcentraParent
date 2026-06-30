import { describe, expect, it } from 'vitest';
import {
  ProductionSupportStatusBackendDeadLetterProofSchema,
  ProductionSupportStatusBackendDeadLetterRowSchema,
  summarizeProductionSupportStatusBackendDeadLetterRows,
} from '@ocentra-parent/schema-domain/production-support-status-backend-dead-letter-proof';
import { ProductionSupportStatusBackendDeadLetterReadModel } from '@ocentra-parent/schema-domain/production-support-status-backend-dead-letter-read-model';

describe('production support status backend dead-letter proof', () => {
  acceptsDeadLetterRows();
  rejectsDurableRetryAuditPayloadAndBackendOverclaims();
  rejectsSensitiveDeadLetterData();
  rejectsIncompleteDeadLetterCoverage();
});

function acceptsDeadLetterRows(): void {
  it('accepts each status backend target with required dead-letter states', () => {
    const proof = ProductionSupportStatusBackendDeadLetterProofSchema.parse(
      ProductionSupportStatusBackendDeadLetterReadModel
    );

    for (const targetSummary of Object.values(summarizeProductionSupportStatusBackendDeadLetterRows(proof.rows))) {
      expect(targetSummary).toEqual({
        requested: 1,
        authorized: 1,
        'dead-lettered': 1,
        'triage-ready': 1,
        'retry-blocked': 1,
        failed: 1,
        'manual-required': 1,
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
    expect(proof.productionSlaClaim).toBe('not-implemented');
    expect(proof.legalDisclosureExecutionClaim).toBe('manual-required');
    expect(proof.childActivityCustodyClaim).toBe('not-implemented');
  });
}

function rejectsDurableRetryAuditPayloadAndBackendOverclaims(): void {
  it('rejects implemented executed or persisted dead-letter storage retry audit payload and backend states', () => {
    const runbookRow = requiredDeadLetter('support-runbook-status-dead-letter', 'dead-lettered');
    const incidentRow = requiredDeadLetter('incident-status-dead-letter', 'retry-blocked');
    const uploadRow = requiredDeadLetter('support-upload-status-dead-letter', 'triage-ready');

    expect(
      ProductionSupportStatusBackendDeadLetterRowSchema.safeParse({
        ...runbookRow,
        durableQueueStorageState: 'persisted',
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendDeadLetterRowSchema.safeParse({
        ...incidentRow,
        retryWorkerState: 'executed',
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendDeadLetterRowSchema.safeParse({
        ...incidentRow,
        auditPersistenceState: 'implemented',
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendDeadLetterRowSchema.safeParse({
        ...uploadRow,
        deadLetterPayloadCustodyState: 'persisted',
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendDeadLetterRowSchema.safeParse({
        ...uploadRow,
        backendExecutionState: 'executed',
      }).success
    ).toBe(false);
  });
}

function rejectsSensitiveDeadLetterData(): void {
  it('rejects dead-letter payloads or omitted provider-secret exclusions', () => {
    const row = requiredDeadLetter('support-upload-status-dead-letter', 'failed');

    expect(
      ProductionSupportStatusBackendDeadLetterRowSchema.safeParse({
        ...row,
        supportSafeDataClasses: [...row.supportSafeDataClasses, 'dead-letter-payload'],
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendDeadLetterRowSchema.safeParse({
        ...row,
        forbiddenDataClasses: row.forbiddenDataClasses.filter((dataClass) => dataClass !== 'provider-secrets'),
      }).success
    ).toBe(false);
  });
}

function rejectsIncompleteDeadLetterCoverage(): void {
  it('rejects proof missing dead-letter coverage or payload custody non-claim', () => {
    expect(
      ProductionSupportStatusBackendDeadLetterProofSchema.safeParse({
        ...ProductionSupportStatusBackendDeadLetterReadModel,
        rows: ProductionSupportStatusBackendDeadLetterReadModel.rows.filter(
          (row) => row.target !== 'privacy-legal-status-dead-letter' || row.deadLetterState !== 'backend-unavailable'
        ),
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendDeadLetterProofSchema.safeParse({
        ...ProductionSupportStatusBackendDeadLetterReadModel,
        nonClaims: ProductionSupportStatusBackendDeadLetterReadModel.nonClaims.filter(
          (nonClaim) => nonClaim !== 'no-dead-letter-payload-custody'
        ),
      }).success
    ).toBe(false);
  });
}

function requiredDeadLetter(
  target: 'support-runbook-status-dead-letter' | 'incident-status-dead-letter' | 'support-upload-status-dead-letter',
  deadLetterState: 'dead-lettered' | 'triage-ready' | 'retry-blocked' | 'failed'
): (typeof ProductionSupportStatusBackendDeadLetterReadModel.rows)[number] {
  const row = ProductionSupportStatusBackendDeadLetterReadModel.rows.find(
    (entry) => entry.target === target && entry.deadLetterState === deadLetterState
  );
  if (row === undefined) {
    throw new Error(`missing status backend dead-letter row: ${target} ${deadLetterState}`);
  }
  return row;
}

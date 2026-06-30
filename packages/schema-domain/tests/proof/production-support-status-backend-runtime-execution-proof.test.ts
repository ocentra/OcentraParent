import { describe, expect, it } from 'vitest';
import {
  ProductionSupportStatusBackendRuntimeExecutionProofSchema,
  ProductionSupportStatusBackendRuntimeExecutionRowSchema,
  summarizeProductionSupportStatusBackendRuntimeExecutionRows,
} from '@ocentra-parent/schema-domain/production-support-status-backend-runtime-execution-proof';
import { ProductionSupportStatusBackendRuntimeExecutionReadModel } from '@ocentra-parent/schema-domain/production-support-status-backend-runtime-execution-read-model';

describe('production support status backend runtime execution proof', () => {
  acceptsRuntimeExecutionRows();
  rejectsRuntimeExecutionInfrastructureOverclaims();
  rejectsSensitiveRuntimeExecutionData();
  rejectsIncompleteRuntimeExecutionCoverage();
});

function acceptsRuntimeExecutionRows(): void {
  it('accepts each runtime execution target with required readiness states', () => {
    const proof = ProductionSupportStatusBackendRuntimeExecutionProofSchema.parse(
      ProductionSupportStatusBackendRuntimeExecutionReadModel
    );

    for (const targetSummary of Object.values(
      summarizeProductionSupportStatusBackendRuntimeExecutionRows(proof.rows)
    )) {
      expect(targetSummary).toEqual({
        requested: 1,
        authorized: 1,
        queued: 1,
        running: 1,
        'runtime-evidence-ready': 1,
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

function rejectsRuntimeExecutionInfrastructureOverclaims(): void {
  it('rejects implemented executed or persisted runtime infrastructure states', () => {
    const runbookRow = requiredReadiness('support-runbook-status-runtime-execution', 'queued');
    const incidentRow = requiredReadiness('incident-status-runtime-execution', 'running');
    const uploadRow = requiredReadiness('support-upload-status-runtime-execution', 'audit-ready');

    expect(
      ProductionSupportStatusBackendRuntimeExecutionRowSchema.safeParse({
        ...runbookRow,
        durableQueueStorageState: 'persisted',
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendRuntimeExecutionRowSchema.safeParse({
        ...incidentRow,
        retryWorkerState: 'executed',
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendRuntimeExecutionRowSchema.safeParse({
        ...uploadRow,
        auditPersistenceState: 'implemented',
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendRuntimeExecutionRowSchema.safeParse({
        ...uploadRow,
        backendExecutionState: 'executed',
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendRuntimeExecutionRowSchema.safeParse({
        ...uploadRow,
        publicRuntimeExecutionState: 'executed',
      }).success
    ).toBe(false);
  });
}

function rejectsSensitiveRuntimeExecutionData(): void {
  it('rejects runtime execution payloads or omitted provider-secret exclusions', () => {
    const row = requiredReadiness('support-upload-status-runtime-execution', 'failed');

    expect(
      ProductionSupportStatusBackendRuntimeExecutionRowSchema.safeParse({
        ...row,
        supportSafeDataClasses: [...row.supportSafeDataClasses, 'status-backend-execution-payload'],
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendRuntimeExecutionRowSchema.safeParse({
        ...row,
        forbiddenDataClasses: row.forbiddenDataClasses.filter((dataClass) => dataClass !== 'provider-secrets'),
      }).success
    ).toBe(false);
  });
}

function rejectsIncompleteRuntimeExecutionCoverage(): void {
  it('rejects proof missing readiness coverage or runtime execution non-claim', () => {
    expect(
      ProductionSupportStatusBackendRuntimeExecutionProofSchema.safeParse({
        ...ProductionSupportStatusBackendRuntimeExecutionReadModel,
        rows: ProductionSupportStatusBackendRuntimeExecutionReadModel.rows.filter(
          (row) =>
            row.target !== 'privacy-legal-status-runtime-execution' || row.readinessState !== 'backend-unavailable'
        ),
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendRuntimeExecutionProofSchema.safeParse({
        ...ProductionSupportStatusBackendRuntimeExecutionReadModel,
        nonClaims: ProductionSupportStatusBackendRuntimeExecutionReadModel.nonClaims.filter(
          (nonClaim) => nonClaim !== 'no-real-status-backend-execution'
        ),
      }).success
    ).toBe(false);
  });
}

function requiredReadiness(
  target:
    | 'support-runbook-status-runtime-execution'
    | 'incident-status-runtime-execution'
    | 'support-upload-status-runtime-execution',
  readinessState: 'queued' | 'running' | 'audit-ready' | 'failed'
): (typeof ProductionSupportStatusBackendRuntimeExecutionReadModel.rows)[number] {
  const row = ProductionSupportStatusBackendRuntimeExecutionReadModel.rows.find(
    (entry) => entry.target === target && entry.readinessState === readinessState
  );
  if (row === undefined) {
    throw new Error(`missing runtime execution row: ${target} ${readinessState}`);
  }
  return row;
}

import { describe, expect, it } from 'vitest';
import {
  ProductionSupportStatusBackendExecutionQueueProofSchema,
  ProductionSupportStatusBackendExecutionQueueRowSchema,
  summarizeProductionSupportStatusBackendExecutionQueueRows,
} from '@ocentra-parent/schema-domain/production-support-status-backend-execution-queue-proof';
import { ProductionSupportStatusBackendExecutionQueueReadModel } from '@ocentra-parent/schema-domain/production-support-status-backend-execution-queue-read-model';

describe('production support status backend execution queue proof', () => {
  acceptsQueueRows();
  rejectsBackendPublicRuntimeAndProviderOverclaims();
  rejectsSensitiveQueueData();
  rejectsIncompleteQueueCoverage();
});

function acceptsQueueRows(): void {
  it('accepts each status backend target with requested authorized queued running succeeded failed manual and unavailable states', () => {
    const proof = ProductionSupportStatusBackendExecutionQueueProofSchema.parse(
      ProductionSupportStatusBackendExecutionQueueReadModel
    );

    for (const targetSummary of Object.values(summarizeProductionSupportStatusBackendExecutionQueueRows(proof.rows))) {
      expect(targetSummary).toEqual({
        requested: 1,
        authorized: 1,
        queued: 1,
        running: 1,
        succeeded: 1,
        failed: 1,
        'manual-required': 1,
        'backend-unavailable': 1,
      });
    }
    expect(proof.statusBackendExecutionClaim).toBe('manual-required');
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

function rejectsBackendPublicRuntimeAndProviderOverclaims(): void {
  it('rejects implemented or executed backend public runtime provider and upload states', () => {
    const runbookRow = requiredQueue('support-runbook-status-backend-queue', 'queued');
    const incidentRow = requiredQueue('incident-status-backend-queue', 'running');
    const uploadRow = requiredQueue('support-upload-status-backend-queue', 'succeeded');

    expect(
      ProductionSupportStatusBackendExecutionQueueRowSchema.safeParse({
        ...runbookRow,
        backendExecutionState: 'executed',
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendExecutionQueueRowSchema.safeParse({
        ...incidentRow,
        publicRuntimeExecutionState: 'implemented',
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendExecutionQueueRowSchema.safeParse({
        ...incidentRow,
        providerExecutionState: 'executed',
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendExecutionQueueRowSchema.safeParse({
        ...uploadRow,
        supportBackendUploadState: 'executed',
      }).success
    ).toBe(false);
  });
}

function rejectsSensitiveQueueData(): void {
  it('rejects execution payloads or omitted provider-secret exclusions', () => {
    const row = requiredQueue('support-upload-status-backend-queue', 'failed');

    expect(
      ProductionSupportStatusBackendExecutionQueueRowSchema.safeParse({
        ...row,
        supportSafeDataClasses: [...row.supportSafeDataClasses, 'status-backend-execution-payload'],
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendExecutionQueueRowSchema.safeParse({
        ...row,
        forbiddenDataClasses: row.forbiddenDataClasses.filter((dataClass) => dataClass !== 'provider-secrets'),
      }).success
    ).toBe(false);
  });
}

function rejectsIncompleteQueueCoverage(): void {
  it('rejects proof missing queue state coverage or status backend execution non-claim', () => {
    expect(
      ProductionSupportStatusBackendExecutionQueueProofSchema.safeParse({
        ...ProductionSupportStatusBackendExecutionQueueReadModel,
        rows: ProductionSupportStatusBackendExecutionQueueReadModel.rows.filter(
          (row) => row.target !== 'privacy-legal-status-backend-queue' || row.queueState !== 'backend-unavailable'
        ),
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendExecutionQueueProofSchema.safeParse({
        ...ProductionSupportStatusBackendExecutionQueueReadModel,
        nonClaims: ProductionSupportStatusBackendExecutionQueueReadModel.nonClaims.filter(
          (nonClaim) => nonClaim !== 'no-real-status-backend-execution'
        ),
      }).success
    ).toBe(false);
  });
}

function requiredQueue(
  target:
    | 'support-runbook-status-backend-queue'
    | 'incident-status-backend-queue'
    | 'support-upload-status-backend-queue',
  queueState: 'queued' | 'running' | 'succeeded' | 'failed'
): (typeof ProductionSupportStatusBackendExecutionQueueReadModel.rows)[number] {
  const row = ProductionSupportStatusBackendExecutionQueueReadModel.rows.find(
    (entry) => entry.target === target && entry.queueState === queueState
  );
  if (row === undefined) {
    throw new Error(`missing status backend execution queue row: ${target} ${queueState}`);
  }
  return row;
}

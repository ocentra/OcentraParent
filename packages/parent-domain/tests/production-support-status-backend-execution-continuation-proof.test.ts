import { describe, expect, it } from 'vitest';
import {
  ProductionSupportStatusBackendExecutionContinuationProofSchema,
  ProductionSupportStatusBackendExecutionContinuationRowSchema,
  summarizeProductionSupportStatusBackendExecutionContinuationRows,
} from '../src/production-support-status-backend-execution-continuation-proof';
import { ProductionSupportStatusBackendExecutionContinuationReadModel } from '../src/production-support-status-backend-execution-continuation-read-model';

describe('production support status backend execution continuation proof', () => {
  acceptsExecutionContinuationRows();
  rejectsExecutionContinuationOverclaims();
  rejectsSensitiveExecutionContinuationData();
  rejectsIncompleteExecutionContinuationCoverage();
});

function acceptsExecutionContinuationRows(): void {
  it('accepts each status backend target with required execution continuation states', () => {
    const proof = ProductionSupportStatusBackendExecutionContinuationProofSchema.parse(
      ProductionSupportStatusBackendExecutionContinuationReadModel
    );

    for (const targetSummary of Object.values(
      summarizeProductionSupportStatusBackendExecutionContinuationRows(proof.rows)
    )) {
      expect(targetSummary).toEqual({
        'execution-preflight-ready': 1,
        'runtime-worker-required': 1,
        'durable-storage-required': 1,
        'payload-custody-required': 1,
        'redaction-manifest-required': 1,
        'manual-required': 1,
        'backend-unavailable': 1,
      });
    }
    expect(proof.statusBackendExecutionClaim).toBe('manual-required');
    expect(proof.durableQueueStorageClaim).toBe('manual-required');
    expect(proof.retryWorkerExecutionClaim).toBe('manual-required');
    expect(proof.auditPersistenceClaim).toBe('manual-required');
    expect(proof.deadLetterPayloadCustodyClaim).toBe('manual-required');
    expect(proof.statusBackendPayloadCustodyClaim).toBe('manual-required');
    expect(proof.redactionManifestExecutionClaim).toBe('manual-required');
    expect(proof.publicRuntimeExecutionClaim).toBe('not-implemented');
    expect(proof.providerExecutionClaim).toBe('not-implemented');
    expect(proof.supportBackendUploadExecutionClaim).toBe('manual-required');
    expect(proof.accountLookupExecutionClaim).toBe('manual-required');
    expect(proof.billingProviderContactClaim).toBe('manual-required');
    expect(proof.legalDisclosureExecutionClaim).toBe('manual-required');
    expect(proof.remoteSupportSessionClaim).toBe('not-implemented');
    expect(proof.productionSlaClaim).toBe('not-implemented');
    expect(proof.providerSecretCustodyClaim).toBe('not-implemented');
    expect(proof.defaultHostedFamilyDataClaim).toBe('not-implemented');
    expect(proof.childActivityCustodyClaim).toBe('not-implemented');
  });
}

function rejectsExecutionContinuationOverclaims(): void {
  it('rejects status backend execution storage custody redaction and public runtime overclaims', () => {
    const row = requiredContinuation(
      'support-runbook-status-backend-execution-continuation',
      'execution-preflight-ready'
    );

    expect(
      ProductionSupportStatusBackendExecutionContinuationRowSchema.safeParse({
        ...row,
        statusBackendExecutionState: 'executed',
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendExecutionContinuationRowSchema.safeParse({
        ...row,
        durableQueueStorageState: 'persisted',
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendExecutionContinuationRowSchema.safeParse({
        ...row,
        statusBackendPayloadCustodyState: 'persisted',
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendExecutionContinuationRowSchema.safeParse({
        ...row,
        redactionManifestExecutionState: 'executed',
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendExecutionContinuationRowSchema.safeParse({
        ...row,
        publicRuntimeExecutionState: 'executed',
      }).success
    ).toBe(false);
  });
}

function rejectsSensitiveExecutionContinuationData(): void {
  it('rejects status backend payload data or omitted child-custody exclusions', () => {
    const row = requiredContinuation(
      'support-upload-status-backend-execution-continuation',
      'payload-custody-required'
    );

    expect(
      ProductionSupportStatusBackendExecutionContinuationRowSchema.safeParse({
        ...row,
        supportSafeDataClasses: [...row.supportSafeDataClasses, 'status-backend-payload'],
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendExecutionContinuationRowSchema.safeParse({
        ...row,
        forbiddenDataClasses: row.forbiddenDataClasses.filter((dataClass) => dataClass !== 'child-activity-evidence'),
      }).success
    ).toBe(false);
  });
}

function rejectsIncompleteExecutionContinuationCoverage(): void {
  it('rejects proof missing target coverage source refs or non-claims', () => {
    expect(
      ProductionSupportStatusBackendExecutionContinuationProofSchema.safeParse({
        ...ProductionSupportStatusBackendExecutionContinuationReadModel,
        rows: ProductionSupportStatusBackendExecutionContinuationReadModel.rows.filter(
          (row) =>
            row.target !== 'privacy-legal-status-backend-execution-continuation' ||
            row.continuationState !== 'backend-unavailable'
        ),
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendExecutionContinuationProofSchema.safeParse({
        ...ProductionSupportStatusBackendExecutionContinuationReadModel,
        sourceContractRefs: ProductionSupportStatusBackendExecutionContinuationReadModel.sourceContractRefs.filter(
          (sourceProof) => sourceProof !== 'production-support-status-backend-redaction-manifest-proof'
        ),
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendExecutionContinuationProofSchema.safeParse({
        ...ProductionSupportStatusBackendExecutionContinuationReadModel,
        nonClaims: ProductionSupportStatusBackendExecutionContinuationReadModel.nonClaims.filter(
          (nonClaim) => nonClaim !== 'no-status-backend-payload-custody'
        ),
      }).success
    ).toBe(false);
  });
}

function requiredContinuation(
  target:
    | 'support-runbook-status-backend-execution-continuation'
    | 'support-upload-status-backend-execution-continuation',
  continuationState: 'execution-preflight-ready' | 'payload-custody-required'
): (typeof ProductionSupportStatusBackendExecutionContinuationReadModel.rows)[number] {
  const row = ProductionSupportStatusBackendExecutionContinuationReadModel.rows.find(
    (entry) => entry.target === target && entry.continuationState === continuationState
  );
  if (row === undefined) {
    throw new Error(`missing execution continuation row: ${target} ${continuationState}`);
  }
  return row;
}

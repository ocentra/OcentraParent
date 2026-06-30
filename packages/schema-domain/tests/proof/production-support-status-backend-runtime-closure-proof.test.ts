import { describe, expect, it } from 'vitest';
import {
  ProductionSupportStatusBackendRuntimeClosureProofSchema,
  ProductionSupportStatusBackendRuntimeClosureRowSchema,
  summarizeProductionSupportStatusBackendRuntimeClosureRows,
} from '@ocentra-parent/schema-domain/production-support-status-backend-runtime-closure-proof';
import { ProductionSupportStatusBackendRuntimeClosureReadModel } from '@ocentra-parent/schema-domain/production-support-status-backend-runtime-closure-read-model';

describe('production support status backend runtime closure proof', () => {
  acceptsRuntimeClosureRows();
  rejectsRuntimeClosureInfrastructureOverclaims();
  rejectsSensitiveRuntimeClosureData();
  rejectsIncompleteRuntimeClosureCoverage();
});

function acceptsRuntimeClosureRows(): void {
  it('accepts each status backend closure target with required closure states', () => {
    const proof = ProductionSupportStatusBackendRuntimeClosureProofSchema.parse(
      ProductionSupportStatusBackendRuntimeClosureReadModel
    );

    for (const targetSummary of Object.values(summarizeProductionSupportStatusBackendRuntimeClosureRows(proof.rows))) {
      expect(targetSummary).toEqual({
        'runtime-row-validated': 1,
        'queue-audit-linked': 1,
        'payload-custody-linked': 1,
        'redaction-manifest-linked': 1,
        'closure-manual-required': 1,
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
    expect(proof.childActivityCustodyClaim).toBe('not-implemented');
  });
}

function rejectsRuntimeClosureInfrastructureOverclaims(): void {
  it('rejects implemented executed or persisted closure infrastructure states', () => {
    const runbookRow = requiredClosure('support-runbook-status-backend-closure', 'queue-audit-linked');
    const incidentRow = requiredClosure('incident-status-backend-closure', 'payload-custody-linked');
    const privacyLegalRow = requiredClosure('privacy-legal-status-backend-closure', 'redaction-manifest-linked');

    expect(
      ProductionSupportStatusBackendRuntimeClosureRowSchema.safeParse({
        ...runbookRow,
        durableQueueStorageState: 'persisted',
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendRuntimeClosureRowSchema.safeParse({
        ...runbookRow,
        auditPersistenceState: 'executed',
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendRuntimeClosureRowSchema.safeParse({
        ...incidentRow,
        statusBackendPayloadCustodyState: 'persisted',
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendRuntimeClosureRowSchema.safeParse({
        ...privacyLegalRow,
        redactionManifestExecutionState: 'executed',
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendRuntimeClosureRowSchema.safeParse({
        ...privacyLegalRow,
        publicRuntimeExecutionState: 'executed',
      }).success
    ).toBe(false);
  });
}

function rejectsSensitiveRuntimeClosureData(): void {
  it('rejects closure payload classes or omitted child-custody exclusions', () => {
    const row = requiredClosure('support-upload-status-backend-closure', 'payload-custody-linked');

    expect(
      ProductionSupportStatusBackendRuntimeClosureRowSchema.safeParse({
        ...row,
        supportSafeDataClasses: [...row.supportSafeDataClasses, 'status-backend-payload'],
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendRuntimeClosureRowSchema.safeParse({
        ...row,
        forbiddenDataClasses: row.forbiddenDataClasses.filter((dataClass) => dataClass !== 'child-activity-evidence'),
      }).success
    ).toBe(false);
  });
}

function rejectsIncompleteRuntimeClosureCoverage(): void {
  it('rejects proof missing closure coverage source refs or non-claims', () => {
    expect(
      ProductionSupportStatusBackendRuntimeClosureProofSchema.safeParse({
        ...ProductionSupportStatusBackendRuntimeClosureReadModel,
        rows: ProductionSupportStatusBackendRuntimeClosureReadModel.rows.filter(
          (row) => row.target !== 'privacy-legal-status-backend-closure' || row.closureState !== 'backend-unavailable'
        ),
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendRuntimeClosureProofSchema.safeParse({
        ...ProductionSupportStatusBackendRuntimeClosureReadModel,
        sourceContractRefs: ProductionSupportStatusBackendRuntimeClosureReadModel.sourceContractRefs.filter(
          (sourceProof) => sourceProof !== 'production-support-status-backend-redaction-manifest-proof'
        ),
      }).success
    ).toBe(false);
    expect(
      ProductionSupportStatusBackendRuntimeClosureProofSchema.safeParse({
        ...ProductionSupportStatusBackendRuntimeClosureReadModel,
        nonClaims: ProductionSupportStatusBackendRuntimeClosureReadModel.nonClaims.filter(
          (nonClaim) => nonClaim !== 'no-status-backend-payload-custody'
        ),
      }).success
    ).toBe(false);
  });
}

function requiredClosure(
  target:
    | 'support-runbook-status-backend-closure'
    | 'incident-status-backend-closure'
    | 'privacy-legal-status-backend-closure'
    | 'support-upload-status-backend-closure',
  closureState: 'queue-audit-linked' | 'payload-custody-linked' | 'redaction-manifest-linked'
): (typeof ProductionSupportStatusBackendRuntimeClosureReadModel.rows)[number] {
  const row = ProductionSupportStatusBackendRuntimeClosureReadModel.rows.find(
    (entry) => entry.target === target && entry.closureState === closureState
  );
  if (row === undefined) {
    throw new Error(`missing runtime closure row: ${target} ${closureState}`);
  }
  return row;
}

import { describe, expect, it } from 'vitest';
import {
  ProductionSupportProofStatusMatrixClosureProofSchema,
  ProductionSupportProofStatusMatrixClosureRowSchema,
  summarizeProductionSupportProofStatusMatrixClosureRows,
} from '../src/production-support-proof-status-matrix-closure-proof';
import { ProductionSupportProofStatusMatrixClosureReadModel } from '../src/production-support-proof-status-matrix-closure-read-model';

describe('production support proof status matrix closure proof', () => {
  it('accepts a closure row for every backend public legal provider export and release area', () => {
    const proof = ProductionSupportProofStatusMatrixClosureProofSchema.parse(
      ProductionSupportProofStatusMatrixClosureReadModel
    );

    expect(summarizeProductionSupportProofStatusMatrixClosureRows(proof.rows)).toEqual({
      'status-backend-runtime': 1,
      'public-runtime-publication': 1,
      'privacy-legal-disclosure': 1,
      'provider-secret-custody': 1,
      'export-delete-lifecycle': 1,
      'release-installer-support': 1,
    });
    expect(proof.publicRuntimeClaim).toBe('not-implemented');
    expect(proof.statusBackendExecutionClaim).toBe('manual-required');
    expect(proof.signingStoreClaim).toBe('manual-required');
    expect(proof.updaterExecutionClaim).toBe('manual-required');
    expect(proof.supportBackendUploadExecutionClaim).toBe('manual-required');
    expect(proof.accountBillingProviderExecutionClaim).toBe('manual-required');
    expect(proof.legalDisclosureExecutionClaim).toBe('manual-required');
    expect(proof.productionSlaClaim).toBe('not-implemented');
    expect(proof.providerSecretCustodyClaim).toBe('not-implemented');
    expect(proof.childActivityCustodyClaim).toBe('not-implemented');
  });

  it('rejects runtime backend public legal provider and child custody overclaims', () => {
    const row = requiredRow('status-backend-runtime');

    expect(
      ProductionSupportProofStatusMatrixClosureRowSchema.safeParse({ ...row, runtimeState: 'source-proof-present' })
        .success
    ).toBe(false);
    expect(
      ProductionSupportProofStatusMatrixClosureRowSchema.safeParse({
        ...row,
        backendExecutionState: 'source-proof-present',
      }).success
    ).toBe(false);
    expect(
      ProductionSupportProofStatusMatrixClosureRowSchema.safeParse({
        ...row,
        publicRuntimeState: 'source-proof-present',
      }).success
    ).toBe(false);
    expect(
      ProductionSupportProofStatusMatrixClosureRowSchema.safeParse({
        ...row,
        childActivityCustodyState: 'source-proof-present',
      }).success
    ).toBe(false);
  });

  it('rejects proof missing required areas source proofs or non-claims', () => {
    expect(
      ProductionSupportProofStatusMatrixClosureProofSchema.safeParse({
        ...ProductionSupportProofStatusMatrixClosureReadModel,
        rows: ProductionSupportProofStatusMatrixClosureReadModel.rows.filter(
          (row) => row.area !== 'release-installer-support'
        ),
      }).success
    ).toBe(false);
    expect(
      ProductionSupportProofStatusMatrixClosureProofSchema.safeParse({
        ...ProductionSupportProofStatusMatrixClosureReadModel,
        sourceProofRefs: ProductionSupportProofStatusMatrixClosureReadModel.sourceProofRefs.filter(
          (sourceProof) => sourceProof !== 'production-support-delete-executor-proof'
        ),
      }).success
    ).toBe(false);
    expect(
      ProductionSupportProofStatusMatrixClosureProofSchema.safeParse({
        ...ProductionSupportProofStatusMatrixClosureReadModel,
        nonClaims: ProductionSupportProofStatusMatrixClosureReadModel.nonClaims.filter(
          (nonClaim) => nonClaim !== 'no-child-activity-custody'
        ),
      }).success
    ).toBe(false);
  });
});

function requiredRow(
  area: 'status-backend-runtime'
): (typeof ProductionSupportProofStatusMatrixClosureReadModel.rows)[number] {
  const row = ProductionSupportProofStatusMatrixClosureReadModel.rows.find((entry) => entry.area === area);
  if (row === undefined) {
    throw new Error(`missing matrix closure row: ${area}`);
  }
  return row;
}

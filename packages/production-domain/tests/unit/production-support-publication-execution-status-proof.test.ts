import { describe, expect, it } from 'vitest';
import {
  ProductionSupportPublicationExecutionStatusProofSchema,
  ProductionSupportPublicationExecutionStatusRowSchema,
  summarizeProductionSupportPublicationExecutionStatusRows,
} from '../../src/production-support-publication-execution-status-proof';
import { ProductionSupportPublicationExecutionStatusReadModel } from '../../src/production-support-publication-execution-status-read-model';

describe('production support publication execution status proof', () => {
  acceptsPublicationExecutionStatusRows();
  rejectsRuntimeExecutionOverclaims();
  rejectsSensitivePublicationExecutionData();
  rejectsIncompleteStatusCoverage();
});

function acceptsPublicationExecutionStatusRows(): void {
  it('accepts every publication target with requested queued running succeeded failed and manual states', () => {
    const proof = ProductionSupportPublicationExecutionStatusProofSchema.parse(
      ProductionSupportPublicationExecutionStatusReadModel
    );

    const summary = summarizeProductionSupportPublicationExecutionStatusRows(proof.rows);
    for (const targetSummary of Object.values(summary)) {
      expect(targetSummary).toEqual({
        requested: 1,
        queued: 1,
        running: 1,
        succeeded: 1,
        failed: 1,
        'manual-required': 1,
      });
    }
    expect(proof.publicRuntimeExecutionClaim).toBe('not-implemented');
    expect(proof.publicationRunnerExecutionClaim).toBe('manual-required');
    expect(proof.statusBackendExecutionClaim).toBe('manual-required');
    expect(proof.supportBackendUploadExecutionClaim).toBe('manual-required');
    expect(proof.legalDisclosureExecutionClaim).toBe('manual-required');
    expect(proof.childActivityCustodyClaim).toBe('not-implemented');
  });
}

function rejectsRuntimeExecutionOverclaims(): void {
  it('rejects implemented public runtime runner status backend upload and legal execution states', () => {
    const runbookRow = requiredExecutionStatus('support-runbook-publication-execution', 'requested');
    const uploadRow = requiredExecutionStatus('support-backend-upload-publication-execution', 'running');
    const legalRow = requiredExecutionStatus('privacy-legal-publication-execution', 'succeeded');

    expect(
      ProductionSupportPublicationExecutionStatusRowSchema.safeParse({ ...runbookRow, publicRuntimeState: 'executed' })
        .success
    ).toBe(false);
    expect(
      ProductionSupportPublicationExecutionStatusRowSchema.safeParse({
        ...runbookRow,
        publicationRunnerState: 'implemented',
      }).success
    ).toBe(false);
    expect(
      ProductionSupportPublicationExecutionStatusRowSchema.safeParse({ ...runbookRow, statusBackendState: 'executed' })
        .success
    ).toBe(false);
    expect(
      ProductionSupportPublicationExecutionStatusRowSchema.safeParse({
        ...uploadRow,
        supportBackendUploadState: 'executed',
      }).success
    ).toBe(false);
    expect(
      ProductionSupportPublicationExecutionStatusRowSchema.safeParse({ ...legalRow, legalExecutionState: 'executed' })
        .success
    ).toBe(false);
  });
}

function rejectsSensitivePublicationExecutionData(): void {
  it('rejects rows that expose support bundles or omit provider secrets from exclusions', () => {
    const row = requiredExecutionStatus('support-backend-upload-publication-execution', 'queued');

    expect(
      ProductionSupportPublicationExecutionStatusRowSchema.safeParse({
        ...row,
        supportSafeDataClasses: [...row.supportSafeDataClasses, 'raw-support-bundle'],
      }).success
    ).toBe(false);
    expect(
      ProductionSupportPublicationExecutionStatusRowSchema.safeParse({
        ...row,
        forbiddenDataClasses: row.forbiddenDataClasses.filter((dataClass) => dataClass !== 'provider-secrets'),
      }).success
    ).toBe(false);
  });
}

function rejectsIncompleteStatusCoverage(): void {
  it('rejects proof missing an execution status row or status backend non-claim', () => {
    expect(
      ProductionSupportPublicationExecutionStatusProofSchema.safeParse({
        ...ProductionSupportPublicationExecutionStatusReadModel,
        rows: ProductionSupportPublicationExecutionStatusReadModel.rows.filter(
          (row) =>
            row.target !== 'public-support-contact-publication-execution' || row.lifecycleStatus !== 'manual-required'
        ),
      }).success
    ).toBe(false);
    expect(
      ProductionSupportPublicationExecutionStatusProofSchema.safeParse({
        ...ProductionSupportPublicationExecutionStatusReadModel,
        nonClaims: ProductionSupportPublicationExecutionStatusReadModel.nonClaims.filter(
          (nonClaim) => nonClaim !== 'no-status-backend-execution'
        ),
      }).success
    ).toBe(false);
  });
}

function requiredExecutionStatus(
  target:
    | 'support-runbook-publication-execution'
    | 'support-backend-upload-publication-execution'
    | 'privacy-legal-publication-execution',
  lifecycleStatus: 'requested' | 'queued' | 'running' | 'succeeded'
): (typeof ProductionSupportPublicationExecutionStatusReadModel.rows)[number] {
  const row = ProductionSupportPublicationExecutionStatusReadModel.rows.find(
    (entry) => entry.target === target && entry.lifecycleStatus === lifecycleStatus
  );
  if (row === undefined) {
    throw new Error(`missing publication execution status row: ${target} ${lifecycleStatus}`);
  }
  return row;
}

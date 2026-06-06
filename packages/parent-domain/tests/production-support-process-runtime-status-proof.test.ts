import { describe, expect, it } from 'vitest';
import {
  ProductionSupportProcessRuntimeStatusProofSchema,
  ProductionSupportProcessRuntimeStatusRowSchema,
  summarizeProductionSupportProcessRuntimeStatusRows,
} from '../src/production-support-process-runtime-status-proof';
import { ProductionSupportProcessRuntimeStatusReadModel } from '../src/production-support-process-runtime-status-read-model';

describe('production support process runtime status proof', () => {
  acceptsSupportProcessRuntimeRows();
  rejectsRuntimeAndCustodyOverclaims();
  rejectsIncompleteRuntimeCoverage();
});

function acceptsSupportProcessRuntimeRows(): void {
  it('accepts requested authorized queued running succeeded failed and manual-required support process rows', () => {
    const proof = ProductionSupportProcessRuntimeStatusProofSchema.parse(
      ProductionSupportProcessRuntimeStatusReadModel
    );

    expect(summarizeProductionSupportProcessRuntimeStatusRows(proof.rows)).toEqual({
      'support-process-requested': 1,
      'parent-consent-authorized': 1,
      'privacy-legal-queued': 1,
      'redaction-review-running': 1,
      'backend-upload-failed': 1,
      'case-resolution-succeeded': 1,
      'support-process-manual-required': 1,
    });
    expect(proof.backendUploadExecutionState).toBe('manual-required');
    expect(proof.publicRuntimeExecutionState).toBe('not-implemented');
    expect(proof.providerExecutionState).toBe('not-implemented');
    expect(proof.productionSlaState).toBe('not-implemented');
    expect(proof.remoteSupportSessionState).toBe('not-implemented');
    expect(proof.childActivityCustodyState).toBe('not-implemented');
    expect(proof.defaultOcentraHostedFamilyDataState).toBe('not-implemented');
  });
}

function rejectsRuntimeAndCustodyOverclaims(): void {
  it('rejects backend upload execution and sensitive support data classes', () => {
    const failed = requiredRuntimeStatus('backend-upload-failed');

    expect(
      ProductionSupportProcessRuntimeStatusRowSchema.safeParse({
        ...failed,
        backendUploadState: 'executed',
      }).success
    ).toBe(false);
    expect(
      ProductionSupportProcessRuntimeStatusRowSchema.safeParse({
        ...failed,
        supportSafeDataClasses: [...failed.supportSafeDataClasses, 'raw-support-bundle'],
      }).success
    ).toBe(false);
    expect(
      ProductionSupportProcessRuntimeStatusRowSchema.safeParse({
        ...failed,
        forbiddenDataClasses: failed.forbiddenDataClasses.filter((dataClass) => dataClass !== 'provider-secret'),
      }).success
    ).toBe(false);
  });
}

function rejectsIncompleteRuntimeCoverage(): void {
  it('rejects missing runtime rows or missing non-claims', () => {
    expect(
      ProductionSupportProcessRuntimeStatusProofSchema.safeParse({
        ...ProductionSupportProcessRuntimeStatusReadModel,
        rows: ProductionSupportProcessRuntimeStatusReadModel.rows.filter(
          (row) => row.surface !== 'support-process-manual-required'
        ),
      }).success
    ).toBe(false);
    expect(
      ProductionSupportProcessRuntimeStatusProofSchema.safeParse({
        ...ProductionSupportProcessRuntimeStatusReadModel,
        nonClaims: ProductionSupportProcessRuntimeStatusReadModel.nonClaims.filter(
          (nonClaim) => nonClaim !== 'no-provider-secrets'
        ),
      }).success
    ).toBe(false);
  });
}

function requiredRuntimeStatus(surface: 'backend-upload-failed') {
  const row = ProductionSupportProcessRuntimeStatusReadModel.rows.find((entry) => entry.surface === surface);
  if (row === undefined) {
    throw new Error(`missing production support process runtime status row: ${surface}`);
  }
  return row;
}

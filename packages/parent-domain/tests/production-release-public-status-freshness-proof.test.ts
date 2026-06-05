import { describe, expect, it } from 'vitest';
import {
  ProductionReleasePublicStatusFreshnessProofSchema,
  ProductionReleasePublicStatusFreshnessReadModel,
  ProductionReleasePublicStatusFreshnessRowSchema,
  summarizeProductionReleasePublicStatusFreshnessRows,
} from '../src/production-release-public-status-freshness-proof';

describe('production release public status freshness proof', () => {
  acceptsPublicStatusFreshnessRows();
  rejectsRuntimeAndBackendOverclaims();
  rejectsIncompletePublicStatusFreshnessCoverage();
});

function acceptsPublicStatusFreshnessRows(): void {
  it('accepts public status freshness rows without live public runtime claims', () => {
    const proof = ProductionReleasePublicStatusFreshnessProofSchema.parse(
      ProductionReleasePublicStatusFreshnessReadModel
    );

    expect(summarizeProductionReleasePublicStatusFreshnessRows(proof.rows)).toEqual({
      'public-download': 1,
      'release-status': 1,
      'update-status': 1,
      'account-status': 1,
      'subscription-status': 1,
      'support-status': 1,
    });
    expect(proof.publicRuntimeExecutionClaim).toBe('not-implemented');
    expect(proof.accountBackendRuntimeClaim).toBe('backend-required');
    expect(proof.billingProviderRuntimeClaim).toBe('not-implemented');
    expect(proof.productionPublishingState).toBe('publication-required');
    expect(proof.signingStoreProofState).toBe('manual-required');
    expect(proof.updaterExecutionState).toBe('manual-required');
    expect(proof.supportBackendUploadState).toBe('manual-required');
    expect(proof.productionSlaClaim).toBe('not-implemented');
    expect(proof.childActivityCustodyClaim).toBe('not-implemented');
  });
}

function rejectsRuntimeAndBackendOverclaims(): void {
  it('rejects public runtime and backend runtime readiness overclaims', () => {
    const accountStatus = requiredFreshnessRow('account-status');

    expect(
      ProductionReleasePublicStatusFreshnessRowSchema.safeParse({
        ...accountStatus,
        publicRuntimeState: 'source-contract-ready',
      }).success
    ).toBe(false);
    expect(
      ProductionReleasePublicStatusFreshnessRowSchema.safeParse({
        ...accountStatus,
        backendState: 'source-contract-ready',
      }).success
    ).toBe(false);
    expect(
      ProductionReleasePublicStatusFreshnessRowSchema.safeParse({
        ...accountStatus,
        freshnessPolicyState: 'manual-required',
      }).success
    ).toBe(false);
  });
}

function rejectsIncompletePublicStatusFreshnessCoverage(): void {
  it('rejects proof missing support status or production SLA non-claim', () => {
    expect(
      ProductionReleasePublicStatusFreshnessProofSchema.safeParse({
        ...ProductionReleasePublicStatusFreshnessReadModel,
        rows: ProductionReleasePublicStatusFreshnessReadModel.rows.filter((row) => row.surface !== 'support-status'),
      }).success
    ).toBe(false);
    expect(
      ProductionReleasePublicStatusFreshnessProofSchema.safeParse({
        ...ProductionReleasePublicStatusFreshnessReadModel,
        nonClaims: ProductionReleasePublicStatusFreshnessReadModel.nonClaims.filter(
          (nonClaim) => nonClaim !== 'no-production-sla'
        ),
      }).success
    ).toBe(false);
  });
}

function requiredFreshnessRow(
  surface: 'account-status'
): (typeof ProductionReleasePublicStatusFreshnessReadModel.rows)[number] {
  const row = ProductionReleasePublicStatusFreshnessReadModel.rows.find((entry) => entry.surface === surface);
  if (row === undefined) {
    throw new Error(`missing public status freshness row: ${surface}`);
  }
  return row;
}

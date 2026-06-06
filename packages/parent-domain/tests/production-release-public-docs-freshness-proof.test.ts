import { describe, expect, it } from 'vitest';
import {
  ProductionReleasePublicDocsFreshnessProofSchema,
  ProductionReleasePublicDocsFreshnessReadModel,
  ProductionReleasePublicDocsFreshnessRowSchema,
  summarizeProductionReleasePublicDocsFreshnessRows,
} from '../src/production-release-public-docs-freshness-proof';

describe('production release public docs freshness proof', () => {
  acceptsPublicDocsFreshnessRows();
  rejectsPublicationAndRouteOverclaims();
  rejectsIncompletePublicDocsFreshnessCoverage();
});

function acceptsPublicDocsFreshnessRows(): void {
  it('accepts public docs freshness rows without live publication claims', () => {
    const proof = ProductionReleasePublicDocsFreshnessProofSchema.parse(ProductionReleasePublicDocsFreshnessReadModel);

    expect(summarizeProductionReleasePublicDocsFreshnessRows(proof.rows)).toEqual({
      'privacy-policy': 1,
      'retention-policy': 1,
      'export-delete-process': 1,
      'support-runbook': 1,
      'incident-status-disclosure': 1,
      'legal-disclosure': 1,
    });
    expect(proof.publicPublicationClaim).toBe('manual-required');
    expect(proof.legalDisclosureExecutionClaim).toBe('manual-required');
    expect(proof.supportBackendUploadClaim).toBe('manual-required');
    expect(proof.accountLookupExecutionClaim).toBe('manual-required');
    expect(proof.billingProviderContactClaim).toBe('manual-required');
    expect(proof.remoteSupportSessionClaim).toBe('not-implemented');
    expect(proof.productionSlaClaim).toBe('not-implemented');
    expect(proof.childActivityCustodyClaim).toBe('not-implemented');
  });
}

function rejectsPublicationAndRouteOverclaims(): void {
  it('rejects public docs publication and route execution overclaims', () => {
    const privacyPolicy = requiredFreshnessRow('privacy-policy');

    expect(
      ProductionReleasePublicDocsFreshnessRowSchema.safeParse({
        ...privacyPolicy,
        publicPublicationState: 'source-contract-ready',
      }).success
    ).toBe(false);
    expect(
      ProductionReleasePublicDocsFreshnessRowSchema.safeParse({
        ...privacyPolicy,
        publicRouteState: 'source-contract-ready',
      }).success
    ).toBe(false);
    expect(
      ProductionReleasePublicDocsFreshnessRowSchema.safeParse({
        ...privacyPolicy,
        freshnessPolicyState: 'manual-required',
      }).success
    ).toBe(false);
  });
}

function rejectsIncompletePublicDocsFreshnessCoverage(): void {
  it('rejects proof missing legal disclosure or production SLA non-claim', () => {
    expect(
      ProductionReleasePublicDocsFreshnessProofSchema.safeParse({
        ...ProductionReleasePublicDocsFreshnessReadModel,
        rows: ProductionReleasePublicDocsFreshnessReadModel.rows.filter((row) => row.document !== 'legal-disclosure'),
      }).success
    ).toBe(false);
    expect(
      ProductionReleasePublicDocsFreshnessProofSchema.safeParse({
        ...ProductionReleasePublicDocsFreshnessReadModel,
        nonClaims: ProductionReleasePublicDocsFreshnessReadModel.nonClaims.filter(
          (nonClaim) => nonClaim !== 'no-production-sla'
        ),
      }).success
    ).toBe(false);
  });
}

function requiredFreshnessRow(
  document: 'privacy-policy'
): (typeof ProductionReleasePublicDocsFreshnessReadModel.rows)[number] {
  const row = ProductionReleasePublicDocsFreshnessReadModel.rows.find((entry) => entry.document === document);
  if (row === undefined) {
    throw new Error(`missing public docs freshness row: ${document}`);
  }
  return row;
}

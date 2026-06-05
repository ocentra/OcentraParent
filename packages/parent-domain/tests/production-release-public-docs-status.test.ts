import { describe, expect, it } from 'vitest';
import {
  ProductionReleasePublicDocsStatusProofSchema,
  ProductionReleasePublicDocsStatusRowSchema,
  summarizeProductionReleasePublicDocsStatusRows,
} from '../src/production-release-public-docs-status';
import { ProductionReleasePublicDocsStatusReadModel } from '../src/production-release-public-docs-status-read-model';

describe('production release public docs status', () => {
  acceptsPublicDocsStatusRows();
  rejectsPublicationAndRuntimeOverclaims();
  rejectsSensitiveDataCustody();
  rejectsIncompleteDocsStatusCoverage();
});

function acceptsPublicDocsStatusRows(): void {
  it('accepts privacy retention export support incident and legal docs as manual publication status proof', () => {
    const proof = ProductionReleasePublicDocsStatusProofSchema.parse(ProductionReleasePublicDocsStatusReadModel);

    expect(summarizeProductionReleasePublicDocsStatusRows(proof.rows)).toEqual({
      'privacy-policy': 1,
      'retention-policy': 1,
      'export-delete-process': 1,
      'support-runbook': 1,
      'incident-status-disclosure': 1,
      'legal-disclosure': 1,
    });
    expect(proof.publicWebsitePublicationClaim).toBe('manual-required');
    expect(proof.supportBackendUploadClaim).toBe('manual-required');
    expect(proof.accountLookupExecutionClaim).toBe('manual-required');
    expect(proof.billingProviderContactClaim).toBe('manual-required');
    expect(proof.remoteSupportSessionClaim).toBe('not-implemented');
    expect(proof.productionSlaClaim).toBe('not-implemented');
    expect(proof.childActivityCustodyClaim).toBe('not-implemented');
    expect(proof.nonClaims).toEqual([
      'no-public-website-publication',
      'no-support-backend-upload',
      'no-account-lookup-execution',
      'no-billing-provider-contact',
      'no-remote-support-session',
      'no-production-sla',
      'no-child-activity-custody',
      'no-legal-disclosure-execution',
    ]);
  });
}

function rejectsPublicationAndRuntimeOverclaims(): void {
  it('rejects published public docs, implemented public routes, and executed support actions', () => {
    const privacyPolicy = requiredDocument('privacy-policy');

    expect(
      ProductionReleasePublicDocsStatusRowSchema.safeParse({
        ...privacyPolicy,
        publicPublicationState: 'published',
      }).success
    ).toBe(false);
    expect(
      ProductionReleasePublicDocsStatusRowSchema.safeParse({
        ...privacyPolicy,
        publicPublicationState: 'implemented',
      }).success
    ).toBe(false);
    expect(
      ProductionReleasePublicDocsStatusRowSchema.safeParse({
        ...privacyPolicy,
        publicRouteState: 'implemented',
      }).success
    ).toBe(false);
    expect(
      ProductionReleasePublicDocsStatusProofSchema.safeParse({
        ...ProductionReleasePublicDocsStatusReadModel,
        accountLookupExecutionClaim: 'executed',
      }).success
    ).toBe(false);
  });
}

function rejectsSensitiveDataCustody(): void {
  it('rejects docs rows that allow support bundles or omit remote support transcripts from exclusions', () => {
    const supportRunbook = requiredDocument('support-runbook');

    expect(
      ProductionReleasePublicDocsStatusRowSchema.safeParse({
        ...supportRunbook,
        supportSafeDataClasses: [...supportRunbook.supportSafeDataClasses, 'raw-support-bundle'],
      }).success
    ).toBe(false);
    expect(
      ProductionReleasePublicDocsStatusRowSchema.safeParse({
        ...supportRunbook,
        forbiddenDataClasses: supportRunbook.forbiddenDataClasses.filter(
          (dataClass) => dataClass !== 'remote-support-session-transcript'
        ),
      }).success
    ).toBe(false);
  });
}

function rejectsIncompleteDocsStatusCoverage(): void {
  it('rejects proof that omits legal disclosure or remote support non-claims', () => {
    expect(
      ProductionReleasePublicDocsStatusProofSchema.safeParse({
        ...ProductionReleasePublicDocsStatusReadModel,
        rows: ProductionReleasePublicDocsStatusReadModel.rows.filter((row) => row.document !== 'legal-disclosure'),
      }).success
    ).toBe(false);
    expect(
      ProductionReleasePublicDocsStatusProofSchema.safeParse({
        ...ProductionReleasePublicDocsStatusReadModel,
        nonClaims: ProductionReleasePublicDocsStatusReadModel.nonClaims.filter(
          (nonClaim) => nonClaim !== 'no-remote-support-session'
        ),
      }).success
    ).toBe(false);
  });
}

function requiredDocument(
  documentName: 'privacy-policy' | 'support-runbook'
): (typeof ProductionReleasePublicDocsStatusReadModel.rows)[number] {
  const row = ProductionReleasePublicDocsStatusReadModel.rows.find((entry) => entry.document === documentName);
  if (row === undefined) {
    throw new Error(`missing public docs status row: ${documentName}`);
  }
  return row;
}

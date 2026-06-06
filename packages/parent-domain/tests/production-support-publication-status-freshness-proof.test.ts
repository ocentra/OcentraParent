import { describe, expect, it } from 'vitest';
import {
  ProductionSupportPublicationStatusFreshnessProofSchema,
  ProductionSupportPublicationStatusFreshnessRowSchema,
  summarizeProductionSupportPublicationStatusFreshnessRows,
} from '../src/production-support-publication-status-freshness-proof';
import { ProductionSupportPublicationStatusFreshnessReadModel } from '../src/production-support-publication-status-freshness-read-model';

describe('production support publication status freshness proof', () => {
  acceptsPublicationStatusFreshnessRows();
  rejectsPublicRuntimeAndSupportUploadOverclaims();
  rejectsSensitivePublicationStatusData();
  rejectsIncompleteFreshnessCoverage();
});

function acceptsPublicationStatusFreshnessRows(): void {
  it('accepts support publication freshness rows without live publication claims', () => {
    const proof = ProductionSupportPublicationStatusFreshnessProofSchema.parse(
      ProductionSupportPublicationStatusFreshnessReadModel
    );

    expect(summarizeProductionSupportPublicationStatusFreshnessRows(proof.rows)).toEqual({
      'support-runbook-publication-freshness': 1,
      'incident-status-publication-freshness': 1,
      'public-support-contact-publication-freshness': 1,
      'support-backend-upload-publication-freshness': 1,
      'privacy-legal-publication-freshness': 1,
      'account-billing-support-publication-freshness': 1,
    });
    expect(proof.publicRuntimeClaim).toBe('not-implemented');
    expect(proof.supportPublicationExecutionClaim).toBe('manual-required');
    expect(proof.supportBackendUploadExecutionClaim).toBe('manual-required');
    expect(proof.accountLookupExecutionClaim).toBe('manual-required');
    expect(proof.billingProviderContactClaim).toBe('manual-required');
    expect(proof.productionSlaClaim).toBe('not-implemented');
    expect(proof.legalDisclosureExecutionClaim).toBe('manual-required');
    expect(proof.childActivityCustodyClaim).toBe('not-implemented');
  });
}

function rejectsPublicRuntimeAndSupportUploadOverclaims(): void {
  it('rejects implemented public runtime publication and executed support upload or legal disclosure', () => {
    const runbookRow = requiredFreshnessRow('support-runbook-publication-freshness');
    const uploadRow = requiredFreshnessRow('support-backend-upload-publication-freshness');
    const legalRow = requiredFreshnessRow('privacy-legal-publication-freshness');

    expect(
      ProductionSupportPublicationStatusFreshnessRowSchema.safeParse({
        ...runbookRow,
        publicRuntimeState: 'implemented',
      }).success
    ).toBe(false);
    expect(
      ProductionSupportPublicationStatusFreshnessRowSchema.safeParse({
        ...uploadRow,
        supportBackendUploadState: 'executed',
      }).success
    ).toBe(false);
    expect(
      ProductionSupportPublicationStatusFreshnessRowSchema.safeParse({
        ...legalRow,
        legalExecutionState: 'executed',
      }).success
    ).toBe(false);
  });
}

function rejectsSensitivePublicationStatusData(): void {
  it('rejects rows that expose account lookup results or omit raw support bundle exclusions', () => {
    const accountRow = requiredFreshnessRow('account-billing-support-publication-freshness');

    expect(
      ProductionSupportPublicationStatusFreshnessRowSchema.safeParse({
        ...accountRow,
        supportSafeDataClasses: [...accountRow.supportSafeDataClasses, 'account-lookup-result'],
      }).success
    ).toBe(false);
    expect(
      ProductionSupportPublicationStatusFreshnessRowSchema.safeParse({
        ...accountRow,
        forbiddenDataClasses: accountRow.forbiddenDataClasses.filter((dataClass) => dataClass !== 'raw-support-bundle'),
      }).success
    ).toBe(false);
  });
}

function rejectsIncompleteFreshnessCoverage(): void {
  it('rejects proof missing support contact freshness or support publication non-claim', () => {
    expect(
      ProductionSupportPublicationStatusFreshnessProofSchema.safeParse({
        ...ProductionSupportPublicationStatusFreshnessReadModel,
        rows: ProductionSupportPublicationStatusFreshnessReadModel.rows.filter(
          (row) => row.surface !== 'public-support-contact-publication-freshness'
        ),
      }).success
    ).toBe(false);
    expect(
      ProductionSupportPublicationStatusFreshnessProofSchema.safeParse({
        ...ProductionSupportPublicationStatusFreshnessReadModel,
        nonClaims: ProductionSupportPublicationStatusFreshnessReadModel.nonClaims.filter(
          (nonClaim) => nonClaim !== 'no-support-publication-execution'
        ),
      }).success
    ).toBe(false);
  });
}

function requiredFreshnessRow(
  surface:
    | 'support-runbook-publication-freshness'
    | 'support-backend-upload-publication-freshness'
    | 'privacy-legal-publication-freshness'
    | 'account-billing-support-publication-freshness'
): (typeof ProductionSupportPublicationStatusFreshnessReadModel.rows)[number] {
  const row = ProductionSupportPublicationStatusFreshnessReadModel.rows.find((entry) => entry.surface === surface);
  if (row === undefined) {
    throw new Error(`missing publication status freshness row: ${surface}`);
  }
  return row;
}

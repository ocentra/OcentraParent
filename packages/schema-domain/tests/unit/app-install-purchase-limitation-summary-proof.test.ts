import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseLimitationSummaryProofReadModel,
  AppInstallPurchaseLimitationSummaryProofSchema,
  AppInstallPurchaseLimitationSummaryRowSchema,
  summarizeAppInstallPurchaseLimitationSummaryProof,
} from '@ocentra-parent/schema-domain/app-install-purchase-limitation-summary-proof';

describe('app install and purchase limitation summary proof', () => {
  acceptsLimitationSummaryRowsWithoutRuntimeOrProviderClaims();
  rejectsMissingLimitationSummaryCoverageOrRefs();
  rejectsPortalProviderDeliveryCustodyAndBlockingOverclaims();
  rejectsMissingLimitationSummaryNonClaims();
});

function acceptsLimitationSummaryRowsWithoutRuntimeOrProviderClaims(): void {
  it('accepts limitation summary buckets linked to provider store and report status rows', () => {
    const proof = AppInstallPurchaseLimitationSummaryProofSchema.parse(
      AppInstallPurchaseLimitationSummaryProofReadModel
    );

    expect(summarizeAppInstallPurchaseLimitationSummaryProof(proof)).toEqual({
      limitationSummaryRows: 3,
      readyRows: 1,
      manualRequiredRows: 1,
      unavailableRows: 1,
      sourceProviderStoreRows: 5,
      sourceReportStatusRows: 4,
      providerExecutedRows: 0,
      externallyDeliveredRows: 0,
    });
    expect(
      proof.limitationSummaryRows.map(
        (row) =>
          `${row.limitationSummaryState}:${row.sourceProviderStoreReportStatusRowIds.length}:${row.sourceReportStatusReadModelRowIds.length}`
      )
    ).toEqual(['ready:1:3', 'manual-required:3:1', 'unavailable:1:0']);
    for (const row of proof.limitationSummaryRows) {
      expect(row.parentVisibleSummaryRef).toContain(row.limitationSummaryState);
      expect(row.sourceAuditEventRefs.length).toBeGreaterThan(0);
      expect(row.portalApprovalUiClaim).toBe('not-implemented');
      expect(row.portalReportUiClaim).toBe('not-implemented');
      expect(row.runtimeReportDeliveryClaim).toBe('not-delivered');
      expect(row.providerApiExecutionClaim).toBe('not-executed');
      expect(row.storeIntegrationClaim).toBe('not-claimed');
      expect(row.billingProviderContactClaim).toBe('not-executed');
      expect(row.platformAdapterClaim).toBe('not-implemented');
      expect(row.childDeviceDeliveryClaim).toBe('not-delivered');
      expect(row.appBlockingClaim).toBe('not-claimed');
      expect(row.childDataCustody).toBe('no-child-activity-data');
      expect(row.ocentraHostedFamilyDataCustodyClaim).toBe('not-claimed');
      expect(row.claimBoundary).toContain('parent-visible ready manual-required and unavailable buckets');
      expect(row.claimBoundary).toContain('no provider API execution');
    }
  });
}

function rejectsMissingLimitationSummaryCoverageOrRefs(): void {
  it('rejects proofs that omit limitation summary buckets or required refs', () => {
    const proof = AppInstallPurchaseLimitationSummaryProofReadModel;
    const row = proof.limitationSummaryRows[0];

    expect(
      AppInstallPurchaseLimitationSummaryProofSchema.safeParse({
        ...proof,
        limitationSummaryRows: proof.limitationSummaryRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseLimitationSummaryRowSchema.safeParse({
        ...row,
        sourceProviderStoreReportStatusStates: ['manual-required'],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseLimitationSummaryRowSchema.safeParse({
        ...row,
        sourceAuditEventRefs: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseLimitationSummaryRowSchema.safeParse({
        ...row,
        parentVisibleSummaryRef: '',
      }).success
    ).toBe(false);
  });
}

function rejectsPortalProviderDeliveryCustodyAndBlockingOverclaims(): void {
  it('rejects rows that claim portal provider delivery custody or blocking behavior', () => {
    const row = AppInstallPurchaseLimitationSummaryProofReadModel.limitationSummaryRows[0];

    for (const invalidRow of [
      { ...row, portalApprovalUiClaim: 'implemented' },
      { ...row, portalReportUiClaim: 'implemented' },
      { ...row, runtimeReportDeliveryClaim: 'delivered' },
      { ...row, providerApiExecutionClaim: 'executed' },
      { ...row, storeIntegrationClaim: 'claimed' },
      { ...row, billingProviderContactClaim: 'contacted' },
      { ...row, platformAdapterClaim: 'implemented' },
      { ...row, childDeviceDeliveryClaim: 'delivered' },
      { ...row, appBlockingClaim: 'claimed' },
      { ...row, childDataCustody: 'child-activity-data-included' },
      { ...row, ocentraHostedFamilyDataCustodyClaim: 'claimed' },
      { ...row, claimBoundary: 'limitation summary delivered through portal and provider execution' },
    ]) {
      expect(AppInstallPurchaseLimitationSummaryRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingLimitationSummaryNonClaims(): void {
  it('rejects limitation summary proof when required non-claims are removed', () => {
    const proof = AppInstallPurchaseLimitationSummaryProofReadModel;

    for (const claim of [
      'no-portal-approval-ui',
      'no-portal-report-ui',
      'no-external-runtime-report-delivery',
      'no-provider-api-execution',
      'no-store-integration',
      'no-billing-provider-contact',
      'no-platform-adapter-implementation',
      'no-child-device-delivery',
      'no-app-blocking',
      'no-child-activity-data',
      'no-ocentra-hosted-family-data-custody',
    ] as const) {
      expect(
        AppInstallPurchaseLimitationSummaryProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

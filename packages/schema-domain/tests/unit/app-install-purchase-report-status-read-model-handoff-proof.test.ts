import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseReportStatusReadModelHandoffProofReadModel,
  AppInstallPurchaseReportStatusReadModelHandoffProofSchema,
  AppInstallPurchaseReportStatusReadModelHandoffRowSchema,
  summarizeAppInstallPurchaseReportStatusReadModelHandoffProof,
} from '@ocentra-parent/schema-domain/app-install-purchase-report-status-read-model-handoff-proof';

describe('app install and purchase report status read-model handoff proof', () => {
  acceptsReportStatusReadModelRows();
  rejectsMissingRefsOverclaimsAndMissingNonClaims();
});

function acceptsReportStatusReadModelRows(): void {
  it('accepts report status read-model rows that remain parent-visible and nondelivered', () => {
    const proof = AppInstallPurchaseReportStatusReadModelHandoffProofSchema.parse(
      AppInstallPurchaseReportStatusReadModelHandoffProofReadModel
    );

    expect(summarizeAppInstallPurchaseReportStatusReadModelHandoffProof(proof)).toEqual({
      reportStatusReadModelRows: 4,
      readyRows: 3,
      manualRequiredRows: 1,
      portalReportUiRows: 0,
      externallyDeliveredRows: 0,
    });
    expect(
      proof.reportStatusReadModelRows.map(
        (row) =>
          `${row.sourceDecisionAction}:${row.sourceApprovalReportDomainState}:${row.sourceRuntimeReportWriterDeliveryState}:${row.parentVisibleReportStatusState}`
      )
    ).toEqual([
      'approve:approval-report-ready:report-delivery-ready:parent-report-status-ready',
      'deny:approval-report-ready:report-delivery-ready:parent-report-status-ready',
      'time-box:approval-report-ready:report-delivery-ready:parent-report-status-ready',
      'review-needed:approval-report-manual-review:manual-required:manual-required',
    ]);

    const reviewNeededRow = proof.reportStatusReadModelRows.find((row) => row.sourceDecisionAction === 'review-needed');
    if (reviewNeededRow === undefined) {
      throw new Error('missing report status read-model row for review-needed');
    }

    expect(reviewNeededRow.parentVisibleReportStatusRef.length).toBeGreaterThan(0);
    expect(reviewNeededRow.parentVisibleReportReceiptRef.length).toBeGreaterThan(0);
    expect(reviewNeededRow.reportAuditEventRefs.length).toBeGreaterThan(0);

    for (const row of proof.reportStatusReadModelRows) {
      expect(row.portalReportUiClaim).toBe('not-implemented');
      expect(row.runtimeReportDeliveryClaim).toBe('not-delivered');
      expect(row.providerApiExecutionClaim).toBe('not-executed');
      expect(row.storeIntegrationClaim).toBe('not-claimed');
      expect(row.platformAdapterClaim).toBe('not-implemented');
      expect(row.childDeviceDeliveryClaim).toBe('not-delivered');
      expect(row.appBlockingClaim).toBe('not-claimed');
      expect(row.childDataCustody).toBe('no-child-activity-data');
      expect(row.ocentraHostedFamilyDataCustodyClaim).toBe('not-claimed');
      expect(row.claimBoundary).toContain('parent-visible report status rows');
      expect(row.claimBoundary).toContain('no portal report UI');
    }
  });
}

function rejectsMissingRefsOverclaimsAndMissingNonClaims(): void {
  it('rejects read-model rows that omit linked refs or invent portal, delivery, provider, custody, or blocking claims', () => {
    const proof = AppInstallPurchaseReportStatusReadModelHandoffProofReadModel;
    const row = proof.reportStatusReadModelRows[0];

    expect(
      AppInstallPurchaseReportStatusReadModelHandoffProofSchema.safeParse({
        ...proof,
        reportStatusReadModelRows: proof.reportStatusReadModelRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseReportStatusReadModelHandoffRowSchema.safeParse({
        ...row,
        sourceRuntimeReportWriterDeliveryRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseReportStatusReadModelHandoffRowSchema.safeParse({
        ...row,
        sourceApprovalReportDomainRowId: '',
      }).success
    ).toBe(false);

    for (const invalidRow of [
      { ...row, parentVisibleReportStatusRef: '' },
      { ...row, parentVisibleReportStatusState: 'manual-required' },
      { ...row, portalReportUiClaim: 'claimed' },
      { ...row, runtimeReportDeliveryClaim: 'delivered' },
      { ...row, providerApiExecutionClaim: 'executed' },
      { ...row, storeIntegrationClaim: 'claimed' },
      { ...row, platformAdapterClaim: 'implemented' },
      { ...row, childDeviceDeliveryClaim: 'delivered' },
      { ...row, appBlockingClaim: 'claimed' },
      { ...row, childDataCustody: 'child-activity-data-included' },
      { ...row, ocentraHostedFamilyDataCustodyClaim: 'claimed' },
      { ...row, claimBoundary: 'report status UI is implemented' },
    ]) {
      expect(AppInstallPurchaseReportStatusReadModelHandoffRowSchema.safeParse(invalidRow).success).toBe(false);
    }

    expect(
      AppInstallPurchaseReportStatusReadModelHandoffProofSchema.safeParse({
        ...proof,
        nonClaims: proof.nonClaims.filter((claim) => claim !== 'no-portal-report-ui'),
      }).success
    ).toBe(false);
  });
}

import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseReportStatusReadModelHandoffProofReadModel,
  AppInstallPurchaseReportStatusReadModelHandoffProofSchema,
  AppInstallPurchaseReportStatusReadModelHandoffRowSchema,
  summarizeAppInstallPurchaseReportStatusReadModelHandoffProof,
} from '@ocentra-parent/schema-domain/app-install-purchase-report-status-read-model-handoff-proof';

describe('app install and purchase report status read model handoff proof', () => {
  acceptsParentVisibleReportStatusRowsWithoutDeliveryClaims();
  rejectsMissingReportStatusReadModelCoverageOrRefs();
  rejectsPortalDeliveryProviderAdapterCustodyAndBlockingOverclaims();
  rejectsMissingReportStatusReadModelNonClaims();
});

function acceptsParentVisibleReportStatusRowsWithoutDeliveryClaims(): void {
  it('accepts report status read-model rows linked to approval and runtime report writer refs', () => {
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
    for (const row of proof.reportStatusReadModelRows) {
      expect(row.sourceRuntimeReportWriterDeliveryRowId).toContain(row.sourceDecisionAction);
      expect(row.sourceApprovalReportDomainRowId).toContain(row.sourceDecisionAction);
      expect(row.parentVisibleReportStatusRef).toContain(row.sourceDecisionAction);
      expect(row.reportAuditEventRefs.length).toBeGreaterThan(0);
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
      expect(row.claimBoundary).toContain('no external runtime report delivery');
    }
  });
}

function rejectsMissingReportStatusReadModelCoverageOrRefs(): void {
  it('rejects proofs that omit report status rows or required refs', () => {
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
    expect(
      AppInstallPurchaseReportStatusReadModelHandoffRowSchema.safeParse({
        ...row,
        parentVisibleReportStatusRef: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseReportStatusReadModelHandoffRowSchema.safeParse({
        ...row,
        reportAuditEventRefs: [],
      }).success
    ).toBe(false);
  });
}

function rejectsPortalDeliveryProviderAdapterCustodyAndBlockingOverclaims(): void {
  it('rejects rows that claim portal delivery provider adapter custody or blocking behavior', () => {
    const row = AppInstallPurchaseReportStatusReadModelHandoffProofReadModel.reportStatusReadModelRows[0];

    for (const invalidRow of [
      { ...row, parentVisibleReportStatusState: 'delivered' },
      { ...row, portalReportUiClaim: 'implemented' },
      { ...row, runtimeReportDeliveryClaim: 'delivered' },
      { ...row, providerApiExecutionClaim: 'executed' },
      { ...row, storeIntegrationClaim: 'claimed' },
      { ...row, platformAdapterClaim: 'implemented' },
      { ...row, childDeviceDeliveryClaim: 'delivered' },
      { ...row, appBlockingClaim: 'claimed' },
      { ...row, childDataCustody: 'child-activity-data-included' },
      { ...row, ocentraHostedFamilyDataCustodyClaim: 'claimed' },
      { ...row, claimBoundary: 'parent report delivered through portal with provider and adapter execution' },
    ]) {
      expect(AppInstallPurchaseReportStatusReadModelHandoffRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingReportStatusReadModelNonClaims(): void {
  it('rejects report status read-model proof when required non-claims are removed', () => {
    const proof = AppInstallPurchaseReportStatusReadModelHandoffProofReadModel;

    for (const claim of [
      'no-portal-report-ui',
      'no-external-runtime-report-delivery',
      'no-provider-api-execution',
      'no-store-integration',
      'no-platform-adapter-implementation',
      'no-child-device-delivery',
      'no-app-blocking',
      'no-child-activity-data',
      'no-ocentra-hosted-family-data-custody',
    ] as const) {
      expect(
        AppInstallPurchaseReportStatusReadModelHandoffProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

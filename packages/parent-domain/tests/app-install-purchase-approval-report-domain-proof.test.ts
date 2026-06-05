import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseApprovalReportDomainProofReadModel,
  AppInstallPurchaseApprovalReportDomainProofSchema,
  AppInstallPurchaseApprovalReportDomainRowSchema,
  summarizeAppInstallPurchaseApprovalReportDomainProof,
} from '../src/app-install-purchase-approval-report-domain-proof';

describe('app install and purchase approval report domain proof', () => {
  acceptsApprovalReportDomainRowsWithoutPortalOrDeliveryClaims();
  rejectsMissingApprovalReportDomainCoverageOrRefs();
  rejectsPortalDeliveryProviderCustodyAndBlockingOverclaims();
  rejectsMissingApprovalReportDomainNonClaims();
});

function acceptsApprovalReportDomainRowsWithoutPortalOrDeliveryClaims(): void {
  it('accepts parent approval action rows linked to report runtime refs as a domain read model', () => {
    const proof = AppInstallPurchaseApprovalReportDomainProofSchema.parse(
      AppInstallPurchaseApprovalReportDomainProofReadModel
    );

    expect(summarizeAppInstallPurchaseApprovalReportDomainProof(proof)).toEqual({
      approvalReportDomainRows: 4,
      readyRows: 3,
      manualReviewRows: 1,
      unavailableRows: 0,
      reportLinkedRows: 4,
      portalApprovalUiRows: 0,
      portalReportUiRows: 0,
    });
    expect(
      proof.approvalReportDomainRows.map(
        (row) =>
          `${row.sourceDecisionAction}:${row.sourceParentReviewActionState}:${row.approvalReportDomainState}:${row.parentActionRecorded}`
      )
    ).toEqual([
      'approve:approved:approval-report-ready:true',
      'deny:denied:approval-report-ready:true',
      'time-box:time-box-active:approval-report-ready:true',
      'review-needed:review-needed:approval-report-manual-review:false',
    ]);
    for (const row of proof.approvalReportDomainRows) {
      expect(row.sourceParentReviewActionRowId).toContain(row.sourceDecisionAction);
      expect(row.sourceReportRuntimeRefs).toHaveLength(4);
      expect(row.sourceReportSurfaces).toEqual([
        'request-audit-history',
        'parent-decision-audit-history',
        'child-facing-state-report',
        'platform-limitation-report',
      ]);
      expect(row.sourceAuditEventRefs.length).toBeGreaterThan(0);
      expect(row.domainReadModelClaim).toBe('domain-read-model-only');
      expect(row.portalApprovalUiClaim).toBe('not-implemented');
      expect(row.portalReportUiClaim).toBe('not-implemented');
      expect(row.runtimeReportDeliveryClaim).toBe('not-delivered');
      expect(row.providerApiExecutionClaim).toBe('not-executed');
      expect(row.storeIntegrationClaim).toBe('not-claimed');
      expect(row.platformAdapterClaim).toBe('not-implemented');
      expect(row.childDeviceDeliveryClaim).toBe('not-delivered');
      expect(row.interceptionClaim).toBe('not-claimed');
      expect(row.appBlockingClaim).toBe('not-claimed');
      expect(row.childDataCustody).toBe('no-child-activity-data');
      expect(row.ocentraHostedFamilyDataCustodyClaim).toBe('not-claimed');
      expect(row.claimBoundary).toContain('no portal approval UI');
      expect(row.claimBoundary).toContain('no portal report UI');
      expect(row.claimBoundary).toContain('no runtime report delivery');
    }
  });
}

function rejectsMissingApprovalReportDomainCoverageOrRefs(): void {
  it('rejects proofs that omit approval report rows or required refs', () => {
    const proof = AppInstallPurchaseApprovalReportDomainProofReadModel;
    const row = proof.approvalReportDomainRows[0];

    expect(
      AppInstallPurchaseApprovalReportDomainProofSchema.safeParse({
        ...proof,
        approvalReportDomainRows: proof.approvalReportDomainRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseApprovalReportDomainRowSchema.safeParse({
        ...row,
        sourceParentReviewActionRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseApprovalReportDomainRowSchema.safeParse({
        ...row,
        sourceReportRuntimeRefs: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseApprovalReportDomainRowSchema.safeParse({
        ...row,
        sourceAuditEventRefs: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseApprovalReportDomainRowSchema.safeParse({
        ...row,
        reportRuntimeLinked: false,
      }).success
    ).toBe(false);
  });
}

function rejectsPortalDeliveryProviderCustodyAndBlockingOverclaims(): void {
  it('rejects rows that claim portal UI delivery provider custody interception or blocking behavior', () => {
    const row = AppInstallPurchaseApprovalReportDomainProofReadModel.approvalReportDomainRows[0];

    for (const invalidRow of [
      { ...row, portalApprovalUiClaim: 'implemented' },
      { ...row, portalReportUiClaim: 'implemented' },
      { ...row, runtimeReportDeliveryClaim: 'delivered' },
      { ...row, providerApiExecutionClaim: 'executed' },
      { ...row, storeIntegrationClaim: 'claimed' },
      { ...row, platformAdapterClaim: 'implemented' },
      { ...row, childDeviceDeliveryClaim: 'delivered' },
      { ...row, interceptionClaim: 'claimed' },
      { ...row, appBlockingClaim: 'claimed' },
      { ...row, childDataCustody: 'child-activity-data-included' },
      { ...row, ocentraHostedFamilyDataCustodyClaim: 'claimed' },
      { ...row, claimBoundary: 'portal approval and report UI delivered provider action to child device' },
    ]) {
      expect(AppInstallPurchaseApprovalReportDomainRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingApprovalReportDomainNonClaims(): void {
  it('rejects approval report domain proof when required non-claims are removed', () => {
    const proof = AppInstallPurchaseApprovalReportDomainProofReadModel;

    for (const claim of [
      'no-portal-approval-ui',
      'no-portal-report-ui',
      'no-runtime-report-delivery',
      'no-provider-api-execution',
      'no-store-integration',
      'no-platform-adapter',
      'no-child-device-delivery',
      'no-real-install-or-purchase-interception',
      'no-app-blocking',
      'no-child-activity-data',
      'no-ocentra-hosted-family-data-custody',
    ] as const) {
      expect(
        AppInstallPurchaseApprovalReportDomainProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

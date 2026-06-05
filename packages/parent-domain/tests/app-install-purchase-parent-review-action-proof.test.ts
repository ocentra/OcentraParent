import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseParentReviewActionProofReadModel,
  AppInstallPurchaseParentReviewActionProofSchema,
  AppInstallPurchaseParentReviewActionRowSchema,
  summarizeAppInstallPurchaseParentReviewActionProof,
} from '../src/app-install-purchase-parent-review-action-proof';

describe('app install and purchase parent review action proof', () => {
  acceptsDecisionLinkedParentReviewRowsWithoutRuntimeClaims();
  rejectsMissingActionEvidenceOrReportCoverage();
  rejectsPortalProviderDeliveryCustodyInterceptionAndBlockingOverclaims();
  rejectsMissingParentReviewActionNonClaims();
});

function acceptsDecisionLinkedParentReviewRowsWithoutRuntimeClaims(): void {
  it('accepts parent review action rows linked to decisions without portal runtime provider delivery or custody claims', () => {
    const proof = AppInstallPurchaseParentReviewActionProofSchema.parse(
      AppInstallPurchaseParentReviewActionProofReadModel
    );

    expect(summarizeAppInstallPurchaseParentReviewActionProof(proof)).toEqual({
      parentReviewActionRows: 4,
      parentActionRecordedRows: 3,
      manualReviewStateRows: 1,
      reportRuntimeLinkedRows: 4,
      portalApprovalUiRows: 0,
      runtimeDeliveredRows: 0,
    });
    expect(
      proof.parentReviewActionRows.map((row) => `${row.sourceDecisionAction}:${row.parentReviewActionState}`)
    ).toEqual(['approve:approved', 'deny:denied', 'time-box:time-box-active', 'review-needed:review-needed']);
    for (const row of proof.parentReviewActionRows) {
      expect(row.auditEventRefs).toHaveLength(1);
      expect(row.sourceApprovedApiEntitlementEvidenceRefs).toHaveLength(5);
      expect(row.sourceReportRuntimeRefs).toHaveLength(4);
      expect(row.runtimeActionDeliveryClaim).toBe('not-delivered');
      expect(row.portalApprovalUiClaim).toBe('not-implemented');
      expect(row.providerApiExecutionClaim).toBe('not-executed');
      expect(row.storeIntegrationClaim).toBe('not-claimed');
      expect(row.platformAdapterClaim).toBe('not-implemented');
      expect(row.childDeliveryClaim).toBe('not-delivered');
      expect(row.runtimeReportDeliveryClaim).toBe('not-delivered');
      expect(row.interceptionClaim).toBe('not-claimed');
      expect(row.appBlockingClaim).toBe('not-claimed');
      expect(row.childDataCustody).toBe('no-child-activity-data');
      expect(row.ocentraHostedFamilyDataCustodyClaim).toBe('not-claimed');
      expect(row.claimBoundary).toContain('no portal approval UI');
      expect(row.claimBoundary).toContain('no parent action runtime delivery');
    }
    expect(
      proof.parentReviewActionRows.filter((row) => row.parentActionRuntimeClaim === 'contract-action-recorded')
    ).toHaveLength(3);
    expect(
      proof.parentReviewActionRows.filter((row) => row.parentActionRuntimeClaim === 'manual-review-state-only')
    ).toHaveLength(1);
  });
}

function rejectsMissingActionEvidenceOrReportCoverage(): void {
  it('rejects parent review proof rows that omit actions, approved API evidence, or report runtime refs', () => {
    const proof = AppInstallPurchaseParentReviewActionProofReadModel;
    const row = proof.parentReviewActionRows[0];

    expect(
      AppInstallPurchaseParentReviewActionProofSchema.safeParse({
        ...proof,
        parentReviewActionRows: proof.parentReviewActionRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseParentReviewActionRowSchema.safeParse({
        ...row,
        sourceApprovedApiEntitlementEvidenceRefs: row.sourceApprovedApiEntitlementEvidenceRefs.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseParentReviewActionRowSchema.safeParse({
        ...row,
        sourceReportRuntimeRefs: row.sourceReportRuntimeRefs.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseParentReviewActionRowSchema.safeParse({
        ...row,
        auditEventRefs: [],
      }).success
    ).toBe(false);
  });
}

function rejectsPortalProviderDeliveryCustodyInterceptionAndBlockingOverclaims(): void {
  it('rejects rows that claim portal delivery provider store adapter custody interception or blocking behavior', () => {
    const row = AppInstallPurchaseParentReviewActionProofReadModel.parentReviewActionRows[0];

    for (const invalidRow of [
      { ...row, runtimeActionDeliveryClaim: 'delivered' },
      { ...row, portalApprovalUiClaim: 'implemented' },
      { ...row, providerApiExecutionClaim: 'executed' },
      { ...row, storeIntegrationClaim: 'claimed' },
      { ...row, platformAdapterClaim: 'implemented' },
      { ...row, childDeliveryClaim: 'delivered' },
      { ...row, runtimeReportDeliveryClaim: 'delivered' },
      { ...row, interceptionClaim: 'claimed' },
      { ...row, appBlockingClaim: 'claimed' },
      { ...row, childDataCustody: 'child-activity-data-included' },
      { ...row, ocentraHostedFamilyDataCustodyClaim: 'claimed' },
      { ...row, claimBoundary: 'parent review action delivered to portal and child device' },
    ]) {
      expect(AppInstallPurchaseParentReviewActionRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingParentReviewActionNonClaims(): void {
  it('rejects parent review action proof when portal delivery custody or blocking non-claims are removed', () => {
    const proof = AppInstallPurchaseParentReviewActionProofReadModel;

    for (const claim of [
      'no-portal-approval-ui',
      'no-parent-action-runtime-delivery',
      'no-provider-api-execution',
      'no-child-device-delivery',
      'no-runtime-report-delivery',
      'no-child-activity-data',
      'not-generic-app-blocking',
      'no-ocentra-hosted-family-data-custody',
    ] as const) {
      expect(
        AppInstallPurchaseParentReviewActionProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseProductClaimSafeParentWorkflowProofReadModel,
  AppInstallPurchaseProductClaimSafeParentWorkflowProofSchema,
  AppInstallPurchaseProductClaimSafeParentWorkflowRowSchema,
  summarizeAppInstallPurchaseProductClaimSafeParentWorkflowProof,
} from '@ocentra-parent/schema-domain/app-install-purchase-product-claim-safe-parent-workflow-proof';

describe('app install purchase product claim safe parent workflow proof', () => {
  acceptsSafeParentWorkflowRows();
  rejectsMissingGateRefsAndWorkflowCoverage();
  rejectsPortalProviderDeliveryBlockingAndCustodyClaims();
  rejectsMissingNonClaims();
});

function acceptsSafeParentWorkflowRows(): void {
  it('routes gated product claims into safe parent workflow states without approving product claims', () => {
    const proof = AppInstallPurchaseProductClaimSafeParentWorkflowProofSchema.parse(
      AppInstallPurchaseProductClaimSafeParentWorkflowProofReadModel
    );

    expect(summarizeAppInstallPurchaseProductClaimSafeParentWorkflowProof(proof)).toEqual({
      safeParentWorkflowRows: 5,
      safeParentReviewReadyRows: 1,
      manualParentReviewRequiredRows: 1,
      unsupportedStoreWorkflowBlockedRows: 3,
      providerExecutedRows: 0,
      portalUiClaimedRows: 0,
      productClaimApprovedRows: 0,
    });
    expect(
      proof.safeParentWorkflowRows.map(
        (row) => `${row.platform}:${row.storeSurface}:${row.sourceProductClaimGateState}:${row.safeParentWorkflowState}`
      )
    ).toEqual([
      'windows:microsoft-store:product-claim-denied:safe-parent-review-ready',
      'macos:mac-app-store:manual-required:manual-parent-review-required',
      'linux:linux-package-manager:blocked:unsupported-store-workflow-blocked',
      'android:google-play:blocked:unsupported-store-workflow-blocked',
      'ios:apple-app-store:blocked:unsupported-store-workflow-blocked',
    ]);
    for (const row of proof.safeParentWorkflowRows) {
      expect(row.parentWorkflowRefs.length).toBe(2);
      expect(row.requiredPortalTestRefs.length).toBe(1);
      expect(row.requiredChildDeliveryRefs.length).toBe(1);
      expect(row.requiredProviderStoreExecutionRefs.length).toBe(1);
      expect(row.requiredPlatformAdapterRefs.length).toBe(1);
      expect(row.portalApprovalUiClaim).toBe('not-claimed');
      expect(row.portalReportUiClaim).toBe('not-claimed');
      expect(row.providerApiExecutionClaim).toBe('not-executed');
      expect(row.childDeviceDeliveryClaim).toBe('not-delivered');
      expect(row.claimBoundary).toContain('safe parent workflow proof only');
    }
  });
}

function rejectsMissingGateRefsAndWorkflowCoverage(): void {
  it('rejects rows that drop source gate linkage or required follow-up refs', () => {
    const proof = AppInstallPurchaseProductClaimSafeParentWorkflowProofReadModel;
    const row = proof.safeParentWorkflowRows[0];

    expect(
      AppInstallPurchaseProductClaimSafeParentWorkflowProofSchema.safeParse({
        ...proof,
        safeParentWorkflowRows: proof.safeParentWorkflowRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProductClaimSafeParentWorkflowRowSchema.safeParse({
        ...row,
        sourceProductClaimGateRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProductClaimSafeParentWorkflowRowSchema.safeParse({
        ...row,
        parentWorkflowRefs: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProductClaimSafeParentWorkflowRowSchema.safeParse({
        ...row,
        requiredPlatformAdapterRefs: [],
      }).success
    ).toBe(false);
  });
}

function rejectsPortalProviderDeliveryBlockingAndCustodyClaims(): void {
  it('rejects rows that claim portal UI provider execution delivery blocking or custody', () => {
    const row = AppInstallPurchaseProductClaimSafeParentWorkflowProofReadModel.safeParentWorkflowRows[0];

    for (const invalidRow of [
      { ...row, safeParentWorkflowState: 'approved-product-claim' },
      { ...row, portalApprovalUiClaim: 'claimed' },
      { ...row, portalReportUiClaim: 'claimed' },
      { ...row, providerApiExecutionClaim: 'executed' },
      { ...row, storeIntegrationClaim: 'claimed' },
      { ...row, platformAdapterClaim: 'implemented' },
      { ...row, childDeviceDeliveryClaim: 'delivered' },
      { ...row, runtimeWriterDeliveryClaim: 'delivered' },
      { ...row, runtimeReportDeliveryClaim: 'delivered' },
      { ...row, appBlockingClaim: 'claimed' },
      { ...row, childDataCustody: 'child-activity-data-included' },
      { ...row, claimBoundary: 'safe workflow with provider execution and portal approval UI' },
    ]) {
      expect(AppInstallPurchaseProductClaimSafeParentWorkflowRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingNonClaims(): void {
  it('rejects proof when required non-claims are removed', () => {
    const proof = AppInstallPurchaseProductClaimSafeParentWorkflowProofReadModel;

    for (const claim of [
      'no-portal-approval-ui',
      'no-portal-report-ui',
      'no-provider-api-execution',
      'no-store-integration',
      'no-platform-adapter-implementation',
      'no-child-device-delivery',
      'no-runtime-writer-delivery',
      'no-runtime-report-delivery',
      'no-app-blocking',
      'no-child-activity-data',
      'no-ocentra-hosted-family-data-custody',
    ] as const) {
      expect(
        AppInstallPurchaseProductClaimSafeParentWorkflowProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

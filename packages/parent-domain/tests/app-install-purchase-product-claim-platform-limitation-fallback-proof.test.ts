import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseProductClaimPlatformLimitationFallbackProofReadModel,
  AppInstallPurchaseProductClaimPlatformLimitationFallbackProofSchema,
  AppInstallPurchaseProductClaimPlatformLimitationFallbackRowSchema,
  summarizeAppInstallPurchaseProductClaimPlatformLimitationFallbackProof,
} from '../src/app-install-purchase-product-claim-platform-limitation-fallback-proof';

describe('app install purchase product claim platform limitation fallback proof', () => {
  acceptsPlatformLimitationFallbackRows();
  rejectsMissingFallbackSourceRefs();
  rejectsProductClaimExecutionDeliveryAndCustodyOverclaims();
  rejectsMissingFallbackNonClaims();
});

function acceptsPlatformLimitationFallbackRows(): void {
  it('links platform preclaim safe parent workflow and limitation actions without approving product claims', () => {
    const proof = AppInstallPurchaseProductClaimPlatformLimitationFallbackProofSchema.parse(
      AppInstallPurchaseProductClaimPlatformLimitationFallbackProofReadModel
    );

    expect(summarizeAppInstallPurchaseProductClaimPlatformLimitationFallbackProof(proof)).toEqual({
      platformLimitationFallbackRows: 5,
      fallbackParentWorkflowReadyRows: 1,
      manualPlatformLimitationFallbackRequiredRows: 1,
      unsupportedPlatformLimitationFallbackBlockedRows: 3,
      productClaimApprovedRows: 0,
      providerExecutedRows: 0,
      platformAdapterImplementedRows: 0,
    });
    expect(
      proof.platformLimitationFallbackRows.map(
        (row) =>
          `${row.platform}:${row.storeSurface}:${row.sourcePlatformPreclaimState}:${row.sourceSafeParentWorkflowState}:${row.sourcePlatformLimitationActionState}:${row.fallbackState}`
      )
    ).toEqual([
      'windows:microsoft-store:manual-platform-preclaim-required:safe-parent-review-ready:parent-action-ready:fallback-parent-workflow-ready',
      'macos:mac-app-store:manual-platform-preclaim-required:manual-parent-review-required:manual-required:manual-platform-limitation-fallback-required',
      'linux:linux-package-manager:unsupported-platform-preclaim-blocked:unsupported-store-workflow-blocked:unavailable:unsupported-platform-limitation-fallback-blocked',
      'android:google-play:unsupported-platform-preclaim-blocked:unsupported-store-workflow-blocked:manual-required:unsupported-platform-limitation-fallback-blocked',
      'ios:apple-app-store:unsupported-platform-preclaim-blocked:unsupported-store-workflow-blocked:manual-required:unsupported-platform-limitation-fallback-blocked',
    ]);

    for (const row of proof.platformLimitationFallbackRows) {
      expect(row.fallbackParentWorkflowRefs.length).toBe(2);
      expect(row.parentLimitationActionRef.length).toBeGreaterThan(0);
      expect(row.requiredPortalTestRefs.length).toBe(2);
      expect(row.requiredManualPlatformEvidenceRefs.length).toBeGreaterThan(0);
      expect(row.requiredChildDeliveryRefs.length).toBeGreaterThan(0);
      expect(row.requiredProviderStoreExecutionRefs.length).toBeGreaterThan(0);
      expect(row.requiredPlatformAdapterRefs.length).toBeGreaterThan(0);
      expect(row.productClaimApprovalClaim).toBe('not-claimed');
      expect(row.providerApiExecutionClaim).toBe('not-executed');
      expect(row.platformAdapterClaim).toBe('not-implemented');
      expect(row.childDeviceDeliveryClaim).toBe('not-delivered');
      expect(row.claimBoundary).toContain('parent-visible fallback workflow refs');
    }
  });
}

function rejectsMissingFallbackSourceRefs(): void {
  it('rejects rows that drop preclaim workflow limitation or follow-up refs', () => {
    const proof = AppInstallPurchaseProductClaimPlatformLimitationFallbackProofReadModel;
    const row = proof.platformLimitationFallbackRows[0];

    expect(
      AppInstallPurchaseProductClaimPlatformLimitationFallbackProofSchema.safeParse({
        ...proof,
        platformLimitationFallbackRows: proof.platformLimitationFallbackRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProductClaimPlatformLimitationFallbackRowSchema.safeParse({
        ...row,
        sourcePlatformPreclaimRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProductClaimPlatformLimitationFallbackRowSchema.safeParse({
        ...row,
        sourceSafeParentWorkflowRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProductClaimPlatformLimitationFallbackRowSchema.safeParse({
        ...row,
        sourcePlatformLimitationActionRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProductClaimPlatformLimitationFallbackRowSchema.safeParse({
        ...row,
        requiredManualPlatformEvidenceRefs: [],
      }).success
    ).toBe(false);
  });
}

function rejectsProductClaimExecutionDeliveryAndCustodyOverclaims(): void {
  it('rejects rows that claim product approval provider execution platform delivery blocking or custody', () => {
    const row =
      AppInstallPurchaseProductClaimPlatformLimitationFallbackProofReadModel.platformLimitationFallbackRows[0];

    for (const invalidRow of [
      { ...row, fallbackState: 'product-claim-approved' },
      { ...row, productClaimApprovalClaim: 'claimed' },
      { ...row, portalApprovalUiClaim: 'claimed' },
      { ...row, portalReportUiClaim: 'claimed' },
      { ...row, googlePlayExecutionClaim: 'executed' },
      { ...row, appleAppStoreExecutionClaim: 'executed' },
      { ...row, microsoftStoreExecutionClaim: 'executed' },
      { ...row, billingProviderContactClaim: 'executed' },
      { ...row, providerApiExecutionClaim: 'executed' },
      { ...row, storeIntegrationClaim: 'claimed' },
      { ...row, platformInterceptionClaim: 'claimed' },
      { ...row, platformAdapterClaim: 'implemented' },
      { ...row, childDeviceDeliveryClaim: 'delivered' },
      { ...row, runtimeWriterDeliveryClaim: 'delivered' },
      { ...row, runtimeReportDeliveryClaim: 'delivered' },
      { ...row, appBlockingClaim: 'claimed' },
      { ...row, childDataCustody: 'child-activity-data-included' },
      { ...row, claimBoundary: 'fallback approves app install product claims' },
    ]) {
      expect(AppInstallPurchaseProductClaimPlatformLimitationFallbackRowSchema.safeParse(invalidRow).success).toBe(
        false
      );
    }
  });
}

function rejectsMissingFallbackNonClaims(): void {
  it('rejects proof when required fallback non-claims are removed', () => {
    const proof = AppInstallPurchaseProductClaimPlatformLimitationFallbackProofReadModel;

    for (const claim of [
      'no-product-claim-approval',
      'no-portal-approval-ui',
      'no-portal-report-ui',
      'no-google-play-execution',
      'no-apple-app-store-execution',
      'no-microsoft-store-execution',
      'no-billing-provider-contact',
      'no-provider-api-execution',
      'no-store-integration',
      'no-platform-interception',
      'no-platform-adapter-implementation',
      'no-child-device-delivery',
      'no-runtime-writer-delivery',
      'no-runtime-report-delivery',
      'no-app-blocking',
      'no-child-activity-data',
      'no-ocentra-hosted-family-data-custody',
    ] as const) {
      expect(
        AppInstallPurchaseProductClaimPlatformLimitationFallbackProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseProductClaimPlatformPreclaimProofReadModel,
  AppInstallPurchaseProductClaimPlatformPreclaimProofSchema,
  AppInstallPurchaseProductClaimPlatformPreclaimRowSchema,
  summarizeAppInstallPurchaseProductClaimPlatformPreclaimProof,
} from '../src/app-install-purchase-product-claim-platform-preclaim-proof';

describe('app install purchase product claim platform preclaim proof', () => {
  acceptsPlatformPreclaimRowsWithoutProductClaimApproval();
  rejectsMissingPlatformPreclaimCoverageOrRefs();
  rejectsPlatformPreclaimOverclaims();
  rejectsMissingPlatformPreclaimNonClaims();
});

function acceptsPlatformPreclaimRowsWithoutProductClaimApproval(): void {
  it('links portal test readiness with platform proof readiness before product claims', () => {
    const proof = AppInstallPurchaseProductClaimPlatformPreclaimProofSchema.parse(
      AppInstallPurchaseProductClaimPlatformPreclaimProofReadModel
    );

    expect(summarizeAppInstallPurchaseProductClaimPlatformPreclaimProof(proof)).toEqual({
      platformPreclaimRows: 5,
      manualPlatformPreclaimRequiredRows: 2,
      unsupportedPlatformPreclaimBlockedRows: 3,
      portalUiClaimedRows: 0,
      platformAdapterImplementedRows: 0,
      productClaimApprovedRows: 0,
    });
    expect(
      proof.platformPreclaimRows.map(
        (row) =>
          `${row.platform}:${row.storeSurface}:${row.sourcePortalTestReadinessState}:${row.sourcePlatformProofReadinessState}:${row.platformPreclaimState}`
      )
    ).toEqual([
      'windows:microsoft-store:portal-test-ready:manual-proof-required:manual-platform-preclaim-required',
      'macos:mac-app-store:manual-portal-test-required:manual-proof-required:manual-platform-preclaim-required',
      'linux:linux-package-manager:unsupported-portal-test-blocked:unavailable:unsupported-platform-preclaim-blocked',
      'android:google-play:unsupported-portal-test-blocked:policy-blocked:unsupported-platform-preclaim-blocked',
      'ios:apple-app-store:unsupported-portal-test-blocked:policy-blocked:unsupported-platform-preclaim-blocked',
    ]);

    for (const row of proof.platformPreclaimRows) {
      expect(row.portalApprovalTestRef.length).toBeGreaterThan(0);
      expect(row.portalReportTestRef.length).toBeGreaterThan(0);
      expect(row.requiredManualPlatformEvidenceRefs.length).toBeGreaterThan(0);
      expect(row.requiredChildDeliveryRefs.length).toBeGreaterThan(0);
      expect(row.requiredProviderStoreExecutionRefs.length).toBeGreaterThan(0);
      expect(row.requiredPlatformAdapterRefs.length).toBeGreaterThan(0);
      expect(row.productClaimApprovalClaim).toBe('not-claimed');
      expect(row.portalApprovalUiClaim).toBe('not-claimed');
      expect(row.portalReportUiClaim).toBe('not-claimed');
      expect(row.providerApiExecutionClaim).toBe('not-executed');
      expect(row.storeIntegrationClaim).toBe('not-claimed');
      expect(row.platformAdapterClaim).toBe('not-implemented');
      expect(row.childDeviceDeliveryClaim).toBe('not-delivered');
      expect(row.claimBoundary).toContain('platform proof readiness before product claims');
    }
  });
}

function rejectsMissingPlatformPreclaimCoverageOrRefs(): void {
  it('rejects missing store coverage and missing portal or platform refs', () => {
    const proof = AppInstallPurchaseProductClaimPlatformPreclaimProofReadModel;
    const row = proof.platformPreclaimRows[0];

    expect(
      AppInstallPurchaseProductClaimPlatformPreclaimProofSchema.safeParse({
        ...proof,
        platformPreclaimRows: proof.platformPreclaimRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProductClaimPlatformPreclaimRowSchema.safeParse({
        ...row,
        sourcePortalTestReadinessRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProductClaimPlatformPreclaimRowSchema.safeParse({
        ...row,
        sourcePlatformProofReadinessRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProductClaimPlatformPreclaimRowSchema.safeParse({
        ...row,
        portalApprovalTestRef: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProductClaimPlatformPreclaimRowSchema.safeParse({
        ...row,
        requiredManualPlatformEvidenceRefs: [],
      }).success
    ).toBe(false);
  });
}

function rejectsPlatformPreclaimOverclaims(): void {
  it('rejects rows that approve product claims or claim portal platform delivery blocking or custody', () => {
    const row = AppInstallPurchaseProductClaimPlatformPreclaimProofReadModel.platformPreclaimRows[0];

    for (const invalidRow of [
      { ...row, platformPreclaimState: 'product-claim-approved' },
      { ...row, productClaimApprovalClaim: 'claimed' },
      { ...row, portalApprovalUiClaim: 'claimed' },
      { ...row, portalReportUiClaim: 'claimed' },
      { ...row, googlePlayExecutionClaim: 'executed' },
      { ...row, appleAppStoreExecutionClaim: 'executed' },
      { ...row, microsoftStoreExecutionClaim: 'executed' },
      { ...row, providerApiExecutionClaim: 'executed' },
      { ...row, storeIntegrationClaim: 'claimed' },
      { ...row, platformAdapterClaim: 'implemented' },
      { ...row, childDeviceDeliveryClaim: 'delivered' },
      { ...row, runtimeWriterDeliveryClaim: 'delivered' },
      { ...row, runtimeReportDeliveryClaim: 'delivered' },
      { ...row, appBlockingClaim: 'claimed' },
      { ...row, childDataCustody: 'child-activity-data-included' },
      { ...row, claimBoundary: 'platform preclaim approves product claims' },
    ]) {
      expect(AppInstallPurchaseProductClaimPlatformPreclaimRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingPlatformPreclaimNonClaims(): void {
  it('rejects proof when required platform preclaim non-claims are removed', () => {
    const proof = AppInstallPurchaseProductClaimPlatformPreclaimProofReadModel;

    for (const claim of [
      'no-product-claim-approval',
      'no-portal-approval-ui',
      'no-portal-report-ui',
      'no-google-play-execution',
      'no-apple-app-store-execution',
      'no-microsoft-store-execution',
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
        AppInstallPurchaseProductClaimPlatformPreclaimProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

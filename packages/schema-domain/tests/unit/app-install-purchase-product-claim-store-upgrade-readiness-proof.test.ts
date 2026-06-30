import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseProductClaimStoreUpgradeReadinessProofReadModel,
  AppInstallPurchaseProductClaimStoreUpgradeReadinessProofSchema,
  AppInstallPurchaseProductClaimStoreUpgradeReadinessRowSchema,
  summarizeAppInstallPurchaseProductClaimStoreUpgradeReadinessProof,
} from '@ocentra-parent/schema-domain/app-install-purchase-product-claim-store-upgrade-readiness-proof';

describe('app install purchase product claim store upgrade readiness proof', () => {
  acceptsStoreUpgradeReadinessRowsWithoutProductClaimApproval();
  rejectsMissingStoreUpgradeCoverageOrRefs();
  rejectsStoreUpgradeOverclaims();
  rejectsMissingStoreUpgradeNonClaims();
});

function acceptsStoreUpgradeReadinessRowsWithoutProductClaimApproval(): void {
  it('links gate portal and provider store rows while keeping product claim upgrades blocked', () => {
    const proof = AppInstallPurchaseProductClaimStoreUpgradeReadinessProofSchema.parse(
      AppInstallPurchaseProductClaimStoreUpgradeReadinessProofReadModel
    );

    expect(summarizeAppInstallPurchaseProductClaimStoreUpgradeReadinessProof(proof)).toEqual({
      storeUpgradeReadinessRows: 5,
      productClaimStoreUpgradeBlockedRows: 1,
      manualStoreUpgradeRequiredRows: 1,
      unsupportedStoreUpgradeBlockedRows: 3,
      providerExecutedRows: 0,
      portalUiClaimedRows: 0,
      productClaimApprovedRows: 0,
    });
    expect(
      proof.storeUpgradeReadinessRows.map(
        (row) =>
          `${row.platform}:${row.storeSurface}:${row.sourceProductClaimGateState}:${row.sourcePortalTestReadinessState}:${row.sourceProviderStoreProductClaimState}:${row.storeUpgradeReadinessState}`
      )
    ).toEqual([
      'windows:microsoft-store:product-claim-denied:portal-test-ready:provider-store-proof-required:product-claim-store-upgrade-blocked',
      'macos:mac-app-store:manual-required:manual-portal-test-required:manual-provider-store-proof-required:manual-store-upgrade-required',
      'linux:linux-package-manager:blocked:unsupported-portal-test-blocked:unsupported-store-proof-blocked:unsupported-store-upgrade-blocked',
      'android:google-play:blocked:unsupported-portal-test-blocked:unsupported-store-proof-blocked:unsupported-store-upgrade-blocked',
      'ios:apple-app-store:blocked:unsupported-portal-test-blocked:unsupported-store-proof-blocked:unsupported-store-upgrade-blocked',
    ]);

    for (const row of proof.storeUpgradeReadinessRows) {
      expect(row.requiredPortalTestRefs.length).toBeGreaterThan(0);
      expect(row.requiredProviderStoreExecutionRefs.length).toBeGreaterThan(0);
      expect(row.requiredProviderEvidenceRefs.length).toBeGreaterThan(0);
      expect(row.requiredChildDeliveryRefs.length).toBeGreaterThan(0);
      expect(row.requiredPlatformAdapterRefs.length).toBeGreaterThan(0);
      expect(row.runtimeWriterReceiptRefs.length).toBeGreaterThan(0);
      expect(row.runtimeReportRefs.length).toBeGreaterThan(0);
      expect(row.productClaimApprovalClaim).toBe('not-claimed');
      expect(row.providerApiExecutionClaim).toBe('not-executed');
      expect(row.storeIntegrationClaim).toBe('not-claimed');
      expect(row.platformAdapterClaim).toBe('not-implemented');
      expect(row.childDeviceDeliveryClaim).toBe('not-delivered');
      expect(row.portalApprovalUiClaim).toBe('not-claimed');
      expect(row.portalReportUiClaim).toBe('not-claimed');
      expect(row.claimBoundary).toContain('product-claim upgrades remain blocked');
    }
  });
}

function rejectsMissingStoreUpgradeCoverageOrRefs(): void {
  it('rejects missing store coverage and missing source refs', () => {
    const proof = AppInstallPurchaseProductClaimStoreUpgradeReadinessProofReadModel;
    const row = proof.storeUpgradeReadinessRows[0];

    expect(
      AppInstallPurchaseProductClaimStoreUpgradeReadinessProofSchema.safeParse({
        ...proof,
        storeUpgradeReadinessRows: proof.storeUpgradeReadinessRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProductClaimStoreUpgradeReadinessRowSchema.safeParse({
        ...row,
        sourceProductClaimGateRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProductClaimStoreUpgradeReadinessRowSchema.safeParse({
        ...row,
        sourcePortalTestReadinessRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProductClaimStoreUpgradeReadinessRowSchema.safeParse({
        ...row,
        sourceProviderStoreProductClaimRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProductClaimStoreUpgradeReadinessRowSchema.safeParse({
        ...row,
        requiredPortalTestRefs: [],
      }).success
    ).toBe(false);
  });
}

function rejectsStoreUpgradeOverclaims(): void {
  it('rejects rows that approve product claims or claim execution delivery portal UI blocking or custody', () => {
    const row = AppInstallPurchaseProductClaimStoreUpgradeReadinessProofReadModel.storeUpgradeReadinessRows[0];

    for (const invalidRow of [
      { ...row, storeUpgradeReadinessState: 'product-claim-approved' },
      { ...row, productClaimApprovalClaim: 'claimed' },
      { ...row, googlePlayExecutionClaim: 'executed' },
      { ...row, appleAppStoreExecutionClaim: 'executed' },
      { ...row, microsoftStoreExecutionClaim: 'executed' },
      { ...row, billingProviderContactClaim: 'contacted' },
      { ...row, providerApiExecutionClaim: 'executed' },
      { ...row, storeIntegrationClaim: 'claimed' },
      { ...row, platformInterceptionClaim: 'claimed' },
      { ...row, platformAdapterClaim: 'implemented' },
      { ...row, runtimeDeviceDeliveryClaim: 'delivered' },
      { ...row, runtimeWriterDeliveryClaim: 'delivered' },
      { ...row, runtimeReportDeliveryClaim: 'delivered' },
      { ...row, childDeviceDeliveryClaim: 'delivered' },
      { ...row, portalApprovalUiClaim: 'claimed' },
      { ...row, portalReportUiClaim: 'claimed' },
      { ...row, appBlockingClaim: 'claimed' },
      { ...row, childDataCustody: 'child-activity-data-included' },
      { ...row, claimBoundary: 'store upgrade readiness approves product claims' },
    ]) {
      expect(AppInstallPurchaseProductClaimStoreUpgradeReadinessRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingStoreUpgradeNonClaims(): void {
  it('rejects proof when required upgrade non-claims are removed', () => {
    const proof = AppInstallPurchaseProductClaimStoreUpgradeReadinessProofReadModel;

    for (const claim of [
      'no-product-claim-approval',
      'no-google-play-execution',
      'no-apple-app-store-execution',
      'no-microsoft-store-execution',
      'no-billing-provider-contact',
      'no-provider-api-execution',
      'no-store-integration',
      'no-platform-interception',
      'no-platform-adapter-implementation',
      'no-runtime-device-delivery',
      'no-runtime-writer-delivery',
      'no-runtime-report-delivery',
      'no-child-device-delivery',
      'no-portal-approval-ui',
      'no-portal-report-ui',
      'no-app-blocking',
      'no-child-activity-data',
      'no-ocentra-hosted-family-data-custody',
    ] as const) {
      expect(
        AppInstallPurchaseProductClaimStoreUpgradeReadinessProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

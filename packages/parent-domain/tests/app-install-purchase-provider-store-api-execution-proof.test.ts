import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseProviderStoreApiExecutionProofReadModel,
  AppInstallPurchaseProviderStoreApiExecutionProofSchema,
  AppInstallPurchaseProviderStoreApiExecutionRowSchema,
  summarizeAppInstallPurchaseProviderStoreApiExecutionProof,
} from '../src/app-install-purchase-provider-store-api-execution-proof';

describe('app install purchase provider store api execution proof', () => {
  acceptsProviderStoreApiExecutionRows();
  rejectsMissingExecutionSourceRefs();
  rejectsProviderStorePlatformDeliveryAndCustodyOverclaims();
  rejectsMissingExecutionNonClaims();
});

function acceptsProviderStoreApiExecutionRows(): void {
  it('links provider store product claim rows with platform fallback rows without executing providers', () => {
    const proof = AppInstallPurchaseProviderStoreApiExecutionProofSchema.parse(
      AppInstallPurchaseProviderStoreApiExecutionProofReadModel
    );

    expect(summarizeAppInstallPurchaseProviderStoreApiExecutionProof(proof)).toEqual({
      providerStoreApiExecutionRows: 5,
      executionReadyRows: 1,
      manualRequiredRows: 1,
      unavailableRows: 1,
      blockedBeforeClaimRows: 2,
      providerExecutedRows: 0,
      productClaimApprovedRows: 0,
    });
    expect(
      proof.providerStoreApiExecutionRows.map(
        (row) =>
          `${row.platform}:${row.storeSurface}:${row.sourceProviderStoreProductClaimState}:${row.sourcePlatformLimitationFallbackState}:${row.providerStoreApiExecutionState}`
      )
    ).toEqual([
      'windows:microsoft-store:provider-store-proof-required:fallback-parent-workflow-ready:execution-ready',
      'macos:mac-app-store:manual-provider-store-proof-required:manual-platform-limitation-fallback-required:manual-required',
      'linux:linux-package-manager:unsupported-store-proof-blocked:unsupported-platform-limitation-fallback-blocked:unavailable',
      'android:google-play:unsupported-store-proof-blocked:unsupported-platform-limitation-fallback-blocked:blocked-before-claim',
      'ios:apple-app-store:unsupported-store-proof-blocked:unsupported-platform-limitation-fallback-blocked:blocked-before-claim',
    ]);

    for (const row of proof.providerStoreApiExecutionRows) {
      expect(row.providerApiExecutionEvidenceRefs.length).toBeGreaterThan(0);
      expect(row.providerCredentialRequirementRefs.length).toBeGreaterThan(0);
      expect(row.fallbackParentWorkflowRefs.length).toBeGreaterThan(0);
      expect(row.manualPlatformEvidenceRefs.length).toBeGreaterThan(0);
      expect(row.requiredPortalTestRefs.length).toBeGreaterThan(0);
      expect(row.requiredChildDeliveryRefs.length).toBeGreaterThan(0);
      expect(row.requiredPlatformAdapterRefs.length).toBeGreaterThan(0);
      expect(row.blockerRefs.length).toBeGreaterThan(0);
      expect(row.productClaimApprovalClaim).toBe('not-claimed');
      expect(row.googlePlayExecutionClaim).toBe('not-executed');
      expect(row.appleAppStoreExecutionClaim).toBe('not-executed');
      expect(row.microsoftStoreExecutionClaim).toBe('not-executed');
      expect(row.billingProviderContactClaim).toBe('not-executed');
      expect(row.providerApiExecutionClaim).toBe('not-executed');
      expect(row.storeIntegrationClaim).toBe('not-claimed');
      expect(row.platformAdapterClaim).toBe('not-implemented');
      expect(row.childDeviceDeliveryClaim).toBe('not-delivered');
      expect(row.claimBoundary).toContain('provider store API execution proof boundary only');
    }
  });
}

function rejectsMissingExecutionSourceRefs(): void {
  it('rejects rows that drop provider store or platform fallback evidence refs', () => {
    const proof = AppInstallPurchaseProviderStoreApiExecutionProofReadModel;
    const row = proof.providerStoreApiExecutionRows[0];

    expect(
      AppInstallPurchaseProviderStoreApiExecutionProofSchema.safeParse({
        ...proof,
        providerStoreApiExecutionRows: proof.providerStoreApiExecutionRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProviderStoreApiExecutionRowSchema.safeParse({
        ...row,
        sourceProviderStoreRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProviderStoreApiExecutionRowSchema.safeParse({
        ...row,
        sourcePlatformLimitationFallbackRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProviderStoreApiExecutionRowSchema.safeParse({
        ...row,
        providerApiExecutionEvidenceRefs: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProviderStoreApiExecutionRowSchema.safeParse({
        ...row,
        fallbackParentWorkflowRefs: [],
      }).success
    ).toBe(false);
  });
}

function rejectsProviderStorePlatformDeliveryAndCustodyOverclaims(): void {
  it('rejects rows that claim product approval provider execution delivery blocking or custody', () => {
    const row = AppInstallPurchaseProviderStoreApiExecutionProofReadModel.providerStoreApiExecutionRows[0];

    for (const invalidRow of [
      { ...row, providerStoreApiExecutionState: 'product-claim-approved' },
      { ...row, productClaimApprovalClaim: 'claimed' },
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
      { ...row, portalApprovalUiClaim: 'claimed' },
      { ...row, portalReportUiClaim: 'claimed' },
      { ...row, appBlockingClaim: 'claimed' },
      { ...row, childDataCustody: 'child-activity-data-included' },
      { ...row, ocentraHostedFamilyDataCustodyClaim: 'claimed' },
      { ...row, claimBoundary: 'provider store API execution proof approves product claims' },
    ]) {
      expect(AppInstallPurchaseProviderStoreApiExecutionRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingExecutionNonClaims(): void {
  it('rejects proof when required provider store API execution non-claims are removed', () => {
    const proof = AppInstallPurchaseProviderStoreApiExecutionProofReadModel;

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
      'no-child-device-delivery',
      'no-runtime-writer-delivery',
      'no-runtime-report-delivery',
      'no-portal-approval-ui',
      'no-portal-report-ui',
      'no-app-blocking',
      'no-child-activity-data',
      'no-ocentra-hosted-family-data-custody',
    ] as const) {
      expect(
        AppInstallPurchaseProviderStoreApiExecutionProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

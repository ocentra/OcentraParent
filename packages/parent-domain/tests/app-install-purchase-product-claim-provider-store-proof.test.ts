import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseProductClaimProviderStoreProofReadModel,
  AppInstallPurchaseProductClaimProviderStoreProofSchema,
  AppInstallPurchaseProductClaimProviderStoreRowSchema,
  summarizeAppInstallPurchaseProductClaimProviderStoreProof,
} from '../src/app-install-purchase-product-claim-provider-store-proof';

describe('app install purchase product claim provider store proof', () => {
  acceptsProviderStoreProductClaimRowsWithoutExecutionClaims();
  rejectsMissingProviderStoreProductClaimCoverageOrRefs();
  rejectsProviderStoreProductClaimOverclaims();
  rejectsMissingProviderStoreProductClaimNonClaims();
});

function acceptsProviderStoreProductClaimRowsWithoutExecutionClaims(): void {
  it('links product claim gates to provider store preflight rows while keeping product claims blocked', () => {
    const proof = AppInstallPurchaseProductClaimProviderStoreProofSchema.parse(
      AppInstallPurchaseProductClaimProviderStoreProofReadModel
    );

    expect(summarizeAppInstallPurchaseProductClaimProviderStoreProof(proof)).toEqual({
      providerStoreProductClaimRows: 5,
      providerStoreProofRequiredRows: 1,
      manualProviderStoreProofRequiredRows: 1,
      unsupportedStoreProofBlockedRows: 3,
      providerExecutedRows: 0,
      productClaimAllowedRows: 0,
    });
    expect(
      proof.providerStoreProductClaimRows.map(
        (row) =>
          `${row.platform}:${row.storeSurface}:${row.sourceProductClaimGateState}:${row.sourceProviderStorePreflightState}:${row.providerStoreProductClaimState}`
      )
    ).toEqual([
      'windows:microsoft-store:product-claim-denied:preflight-ready:provider-store-proof-required',
      'macos:mac-app-store:manual-required:manual-provider-proof-required:manual-provider-store-proof-required',
      'linux:linux-package-manager:blocked:provider-unavailable:unsupported-store-proof-blocked',
      'android:google-play:blocked:manual-provider-proof-required:unsupported-store-proof-blocked',
      'ios:apple-app-store:blocked:manual-provider-proof-required:unsupported-store-proof-blocked',
    ]);
    for (const row of proof.providerStoreProductClaimRows) {
      expect(row.requiredProviderStoreExecutionRefs.length).toBeGreaterThan(0);
      expect(row.requiredProviderEvidenceRefs.length).toBeGreaterThan(0);
      expect(row.runtimeWriterReceiptRefs.length).toBeGreaterThan(0);
      expect(row.requiredPortalTestRefs.length).toBeGreaterThan(0);
      expect(row.requiredChildDeliveryRefs.length).toBeGreaterThan(0);
      expect(row.requiredPlatformAdapterRefs.length).toBeGreaterThan(0);
      expect(row.googlePlayExecutionClaim).toBe('not-executed');
      expect(row.appleAppStoreExecutionClaim).toBe('not-executed');
      expect(row.microsoftStoreExecutionClaim).toBe('not-executed');
      expect(row.billingProviderContactClaim).toBe('not-executed');
      expect(row.providerApiExecutionClaim).toBe('not-executed');
      expect(row.storeIntegrationClaim).toBe('not-claimed');
      expect(row.platformAdapterClaim).toBe('not-implemented');
      expect(row.runtimeDeviceDeliveryClaim).toBe('not-delivered');
      expect(row.childDeviceDeliveryClaim).toBe('not-delivered');
      expect(row.portalApprovalUiClaim).toBe('not-claimed');
      expect(row.portalReportUiClaim).toBe('not-claimed');
      expect(row.appBlockingClaim).toBe('not-claimed');
      expect(row.childDataCustody).toBe('no-child-activity-data');
      expect(row.claimBoundary).toContain('product claims blocked');
    }
  });
}

function rejectsMissingProviderStoreProductClaimCoverageOrRefs(): void {
  it('rejects missing store coverage and missing source refs', () => {
    const proof = AppInstallPurchaseProductClaimProviderStoreProofReadModel;
    const row = proof.providerStoreProductClaimRows[0];

    expect(
      AppInstallPurchaseProductClaimProviderStoreProofSchema.safeParse({
        ...proof,
        providerStoreProductClaimRows: proof.providerStoreProductClaimRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProductClaimProviderStoreRowSchema.safeParse({
        ...row,
        sourceProductClaimGateRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProductClaimProviderStoreRowSchema.safeParse({
        ...row,
        sourceProviderStorePreflightRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProductClaimProviderStoreRowSchema.safeParse({
        ...row,
        requiredProviderEvidenceRefs: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProductClaimProviderStoreRowSchema.safeParse({
        ...row,
        requiredProviderStoreExecutionRefs: [],
      }).success
    ).toBe(false);
  });
}

function rejectsProviderStoreProductClaimOverclaims(): void {
  it('rejects rows that claim provider execution delivery portal UI blocking or custody', () => {
    const row = AppInstallPurchaseProductClaimProviderStoreProofReadModel.providerStoreProductClaimRows[0];

    for (const invalidRow of [
      { ...row, providerStoreProductClaimState: 'product-claim-allowed' },
      { ...row, googlePlayExecutionClaim: 'executed' },
      { ...row, appleAppStoreExecutionClaim: 'executed' },
      { ...row, microsoftStoreExecutionClaim: 'executed' },
      { ...row, billingProviderContactClaim: 'contacted' },
      { ...row, providerApiExecutionClaim: 'executed' },
      { ...row, storeIntegrationClaim: 'claimed' },
      { ...row, platformInterceptionClaim: 'claimed' },
      { ...row, platformAdapterClaim: 'implemented' },
      { ...row, runtimeDeviceDeliveryClaim: 'delivered' },
      { ...row, childDeviceDeliveryClaim: 'delivered' },
      { ...row, portalApprovalUiClaim: 'claimed' },
      { ...row, portalReportUiClaim: 'claimed' },
      { ...row, appBlockingClaim: 'claimed' },
      { ...row, childDataCustody: 'child-activity-data-included' },
      { ...row, claimBoundary: 'provider store execution proof allows product claims' },
    ]) {
      expect(AppInstallPurchaseProductClaimProviderStoreRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingProviderStoreProductClaimNonClaims(): void {
  it('rejects proof when required provider store non-claims are removed', () => {
    const proof = AppInstallPurchaseProductClaimProviderStoreProofReadModel;

    for (const claim of [
      'no-google-play-execution',
      'no-apple-app-store-execution',
      'no-microsoft-store-execution',
      'no-billing-provider-contact',
      'no-provider-api-execution',
      'no-store-integration',
      'no-platform-interception',
      'no-platform-adapter-implementation',
      'no-runtime-device-delivery',
      'no-child-device-delivery',
      'no-portal-approval-ui',
      'no-portal-report-ui',
      'no-app-blocking',
      'no-child-activity-data',
      'no-ocentra-hosted-family-data-custody',
    ] as const) {
      expect(
        AppInstallPurchaseProductClaimProviderStoreProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

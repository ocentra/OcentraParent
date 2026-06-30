import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseProductClaimProviderStoreProofReadModel,
  AppInstallPurchaseProductClaimProviderStoreProofSchema,
  AppInstallPurchaseProductClaimProviderStoreRowSchema,
  summarizeAppInstallPurchaseProductClaimProviderStoreProof,
} from '@ocentra-parent/schema-domain/app-install-purchase-product-claim-provider-store-proof';

describe('app install and purchase product claim provider store proof', () => {
  acceptsBlockedProviderStoreProductClaimRows();
  rejectsMissingRefsOverclaimsAndMissingNonClaims();
});

function acceptsBlockedProviderStoreProductClaimRows(): void {
  it('accepts provider store product claim rows that keep claims blocked until provider execution proof exists', () => {
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
      expect(row.providerApiExecutionClaim).toBe('not-executed');
      expect(row.storeIntegrationClaim).toBe('not-claimed');
      expect(row.platformInterceptionClaim).toBe('not-claimed');
      expect(row.platformAdapterClaim).toBe('not-implemented');
      expect(row.runtimeDeviceDeliveryClaim).toBe('not-delivered');
      expect(row.childDeviceDeliveryClaim).toBe('not-delivered');
      expect(row.portalApprovalUiClaim).toBe('not-claimed');
      expect(row.portalReportUiClaim).toBe('not-claimed');
      expect(row.childDataCustody).toBe('no-child-activity-data');
      expect(row.claimBoundary).toContain('product claims blocked');
      expect(row.claimBoundary).toContain('no provider API execution');
    }
  });
}

function rejectsMissingRefsOverclaimsAndMissingNonClaims(): void {
  it('rejects provider store product claim rows that omit refs or invent execution, portal, delivery, or custody claims', () => {
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

    for (const invalidRow of [
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
      { ...row, claimBoundary: 'provider store product claims are approved' },
    ]) {
      expect(AppInstallPurchaseProductClaimProviderStoreRowSchema.safeParse(invalidRow).success).toBe(false);
    }

    expect(
      AppInstallPurchaseProductClaimProviderStoreProofSchema.safeParse({
        ...proof,
        nonClaims: proof.nonClaims.filter((claim) => claim !== 'no-provider-api-execution'),
      }).success
    ).toBe(false);
  });
}

import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseProductClaimGateProofReadModel,
  AppInstallPurchaseProductClaimGateProofSchema,
  AppInstallPurchaseProductClaimGateRowSchema,
  summarizeAppInstallPurchaseProductClaimGateProof,
} from '../../src/app-install-purchase-product-claim-gate-proof';

describe('app install purchase product claim gate proof', () => {
  acceptsProductClaimGateRows();
  rejectsMissingGateCoverageAndProofRefs();
  rejectsProductClaimAndRuntimeOverclaims();
  rejectsMissingNonClaims();
});

function acceptsProductClaimGateRows(): void {
  it('denies product claims until portal child provider and platform proof are present', () => {
    const proof = AppInstallPurchaseProductClaimGateProofSchema.parse(AppInstallPurchaseProductClaimGateProofReadModel);

    expect(summarizeAppInstallPurchaseProductClaimGateProof(proof)).toEqual({
      productClaimGateRows: 5,
      productClaimDeniedRows: 1,
      manualRequiredRows: 1,
      blockedRows: 3,
      missingPortalTestRows: 5,
      missingChildDeliveryRows: 5,
      missingProviderStoreApiRows: 5,
      missingPlatformAdapterRows: 5,
      unsupportedLimitationRows: 3,
      productClaimAllowedRows: 0,
    });
    expect(
      proof.productClaimGateRows.map(
        (row) =>
          `${row.platform}:${row.storeSurface}:${row.sourceManualEvidencePacketState}:${row.productClaimGateState}:${row.unsupportedOsStoreLimitationState}`
      )
    ).toEqual([
      'windows:microsoft-store:manual-evidence-packet-ready:product-claim-denied:not-limited',
      'macos:mac-app-store:manual-review-required:manual-required:not-limited',
      'linux:linux-package-manager:provider-unavailable:blocked:unsupported-os-store-limitation',
      'android:google-play:manual-review-required:blocked:unsupported-os-store-limitation',
      'ios:apple-app-store:manual-review-required:blocked:unsupported-os-store-limitation',
    ]);
    for (const row of proof.productClaimGateRows) {
      expect(row.portalApprovalReportTestState).toBe('missing');
      expect(row.childDeviceDeliveryProofState).toBe('missing');
      expect(row.providerStoreApiExecutionProofState).toBe('missing');
      expect(row.platformAdapterProofState).toBe('missing');
      expect(row.portalApprovalUiClaim).toBe('not-claimed');
      expect(row.portalReportUiClaim).toBe('not-claimed');
      expect(row.providerApiExecutionClaim).toBe('not-executed');
      expect(row.platformAdapterClaim).toBe('not-implemented');
      expect(row.childDeviceDeliveryClaim).toBe('not-delivered');
      expect(row.claimBoundary).toContain('denies app install product claim');
    }
  });
}

function rejectsMissingGateCoverageAndProofRefs(): void {
  it('rejects proofs that omit store coverage or required gate proof refs', () => {
    const proof = AppInstallPurchaseProductClaimGateProofReadModel;
    const row = proof.productClaimGateRows[0];

    expect(
      AppInstallPurchaseProductClaimGateProofSchema.safeParse({
        ...proof,
        productClaimGateRows: proof.productClaimGateRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProductClaimGateRowSchema.safeParse({
        ...row,
        sourceManualEvidencePacketRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProductClaimGateRowSchema.safeParse({
        ...row,
        requiredPortalTestRefs: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProductClaimGateRowSchema.safeParse({
        ...row,
        requiredProviderStoreExecutionRefs: [],
      }).success
    ).toBe(false);
  });
}

function rejectsProductClaimAndRuntimeOverclaims(): void {
  it('rejects rows that claim product proof runtime delivery provider execution or custody', () => {
    const row = AppInstallPurchaseProductClaimGateProofReadModel.productClaimGateRows[0];

    for (const invalidRow of [
      { ...row, productClaimGateState: 'product-claim-allowed' },
      { ...row, portalApprovalReportTestState: 'present' },
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
      { ...row, claimBoundary: 'product claim allowed with provider execution and child delivery' },
    ]) {
      expect(AppInstallPurchaseProductClaimGateRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingNonClaims(): void {
  it('rejects proof when required non-claims are removed', () => {
    const proof = AppInstallPurchaseProductClaimGateProofReadModel;

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
        AppInstallPurchaseProductClaimGateProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

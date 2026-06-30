import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseApprovedApiEntitlementEvidenceRowSchema,
  AppInstallPurchaseApprovedApiEntitlementProofReadModel,
  AppInstallPurchaseApprovedApiEntitlementProofSchema,
  summarizeAppInstallPurchaseApprovedApiEntitlementProof,
} from '@ocentra-parent/schema-domain/app-install-purchase-approved-api-entitlement-proof';

describe('app install and purchase approved API entitlement proof', () => {
  acceptsApprovedApiEntitlementEvidenceRows();
  rejectsMissingPlatformAndStatusCoverage();
  rejectsProviderAdapterDeliveryReportAndCustodyOverclaims();
  rejectsMissingRequiredNonClaims();
});

function acceptsApprovedApiEntitlementEvidenceRows(): void {
  it('accepts approved API entitlement evidence refs without provider execution or delivery claims', () => {
    const proof = AppInstallPurchaseApprovedApiEntitlementProofSchema.parse(
      AppInstallPurchaseApprovedApiEntitlementProofReadModel
    );

    expect(summarizeAppInstallPurchaseApprovedApiEntitlementProof(proof)).toEqual({
      evidenceRows: 5,
      approvedApiRequiredRows: 1,
      entitlementRequiredRows: 2,
      manualReviewRows: 1,
      unavailableRows: 1,
    });
    expect(proof.evidenceRows.map((row) => [row.platform, row.storeSurface, row.evidenceStatus])).toEqual([
      ['windows', 'microsoft-store', 'approved-api-evidence-required'],
      ['macos', 'mac-app-store', 'manual-platform-review-required'],
      ['linux', 'linux-package-manager', 'platform-unavailable'],
      ['android', 'google-play', 'store-entitlement-evidence-required'],
      ['ios', 'apple-app-store', 'store-entitlement-evidence-required'],
    ]);
    for (const row of proof.evidenceRows) {
      expect(row.providerApiExecutionClaim).toBe('not-executed');
      expect(row.storeIntegrationClaim).toBe('not-claimed');
      expect(row.platformAdapterClaim).toBe('not-implemented');
      expect(row.childDeliveryClaim).toBe('not-delivered');
      expect(row.runtimeReportDeliveryClaim).toBe('not-delivered');
      expect(row.interceptionClaim).toBe('not-claimed');
      expect(row.appBlockingClaim).toBe('not-claimed');
      expect(row.childDataCustody).toBe('no-child-activity-data');
      expect(row.claimBoundary).toContain('no provider API execution');
    }
  });
}

function rejectsMissingPlatformAndStatusCoverage(): void {
  it('rejects proof rows that omit a platform source or required evidence state', () => {
    const proof = AppInstallPurchaseApprovedApiEntitlementProofReadModel;

    expect(
      AppInstallPurchaseApprovedApiEntitlementProofSchema.safeParse({
        ...proof,
        evidenceRows: proof.evidenceRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseApprovedApiEntitlementProofSchema.safeParse({
        ...proof,
        evidenceRows: proof.evidenceRows.map((row) =>
          row.evidenceStatus === 'manual-platform-review-required'
            ? { ...row, evidenceStatus: 'approved-api-evidence-required', evidenceSource: 'approved-store-api' }
            : row
        ),
      }).success
    ).toBe(false);
  });
}

function rejectsProviderAdapterDeliveryReportAndCustodyOverclaims(): void {
  it('rejects rows that claim provider execution adapter delivery report custody interception or blocking behavior', () => {
    const row = AppInstallPurchaseApprovedApiEntitlementProofReadModel.evidenceRows[0];

    for (const invalidRow of [
      { ...row, providerApiExecutionClaim: 'executed' },
      { ...row, storeIntegrationClaim: 'claimed' },
      { ...row, platformAdapterClaim: 'implemented' },
      { ...row, childDeliveryClaim: 'delivered' },
      { ...row, runtimeReportDeliveryClaim: 'delivered' },
      { ...row, interceptionClaim: 'claimed' },
      { ...row, appBlockingClaim: 'claimed' },
      { ...row, childDataCustody: 'child-activity-data-included' },
      { ...row, claimBoundary: 'approved store API integration is implemented' },
    ]) {
      expect(AppInstallPurchaseApprovedApiEntitlementEvidenceRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingRequiredNonClaims(): void {
  it('rejects approved API entitlement proof when provider delivery custody or blocking non-claims are removed', () => {
    const proof = AppInstallPurchaseApprovedApiEntitlementProofReadModel;

    for (const claim of [
      'no-provider-api-execution',
      'no-child-device-delivery',
      'no-runtime-report-delivery',
      'no-child-activity-data',
      'not-generic-app-blocking',
    ] as const) {
      expect(
        AppInstallPurchaseApprovedApiEntitlementProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

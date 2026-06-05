import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchasePlatformAdapterBoundaryProofReadModel,
  AppInstallPurchasePlatformAdapterBoundaryProofSchema,
  AppInstallPurchasePlatformAdapterBoundaryRowSchema,
  summarizeAppInstallPurchasePlatformAdapterBoundaryProof,
} from '../src/app-install-purchase-platform-adapter-boundary-proof';

describe('app install and purchase platform adapter boundary proof', () => {
  acceptsPlatformAdapterBoundaryRowsWithoutRuntimeClaims();
  rejectsMissingPlatformOrAdapterEvidenceCoverage();
  rejectsProviderStoreDeliveryCustodyInterceptionAndBlockingOverclaims();
  rejectsMissingPlatformAdapterBoundaryNonClaims();
});

function acceptsPlatformAdapterBoundaryRowsWithoutRuntimeClaims(): void {
  it('accepts adapter readiness rows without provider store delivery custody interception or blocking claims', () => {
    const proof = AppInstallPurchasePlatformAdapterBoundaryProofSchema.parse(
      AppInstallPurchasePlatformAdapterBoundaryProofReadModel
    );

    expect(summarizeAppInstallPurchasePlatformAdapterBoundaryProof(proof)).toEqual({
      adapterBoundaryRows: 5,
      notImplementedRows: 3,
      manualRequiredRows: 1,
      unavailableRows: 1,
      reportRuntimeLinkedRows: 5,
    });
    expect(proof.adapterBoundaryRows.map((row) => [row.platform, row.storeSurface, row.adapterEvidenceState])).toEqual([
      ['windows', 'microsoft-store', 'approved-api-adapter-evidence-required'],
      ['macos', 'mac-app-store', 'manual-platform-review-required'],
      ['linux', 'linux-package-manager', 'platform-unavailable'],
      ['android', 'google-play', 'entitlement-adapter-evidence-required'],
      ['ios', 'apple-app-store', 'entitlement-adapter-evidence-required'],
    ]);
    for (const row of proof.adapterBoundaryRows) {
      expect(row.providerApiExecutionClaim).toBe('not-executed');
      expect(row.storeIntegrationClaim).toBe('not-claimed');
      expect(row.childDeliveryClaim).toBe('not-delivered');
      expect(row.runtimeReportDeliveryClaim).toBe('not-delivered');
      expect(row.interceptionClaim).toBe('not-claimed');
      expect(row.appBlockingClaim).toBe('not-claimed');
      expect(row.childDataCustody).toBe('no-child-activity-data');
      expect(row.ocentraHostedFamilyDataCustodyClaim).toBe('not-claimed');
      expect(row.reportRuntimeRefs.length).toBe(4);
      expect(row.claimBoundary).toContain('no platform adapter implementation');
    }
  });
}

function rejectsMissingPlatformOrAdapterEvidenceCoverage(): void {
  it('rejects proofs that omit platform source rows or required adapter evidence states', () => {
    const proof = AppInstallPurchasePlatformAdapterBoundaryProofReadModel;

    expect(
      AppInstallPurchasePlatformAdapterBoundaryProofSchema.safeParse({
        ...proof,
        adapterBoundaryRows: proof.adapterBoundaryRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchasePlatformAdapterBoundaryProofSchema.safeParse({
        ...proof,
        adapterBoundaryRows: proof.adapterBoundaryRows.map((row) =>
          row.adapterEvidenceState === 'manual-platform-review-required'
            ? {
                ...row,
                adapterEvidenceState: 'approved-api-adapter-evidence-required',
                adapterRuntimeState: 'not-implemented',
              }
            : row
        ),
      }).success
    ).toBe(false);
  });
}

function rejectsProviderStoreDeliveryCustodyInterceptionAndBlockingOverclaims(): void {
  it('rejects rows that claim provider execution store integration delivery custody interception or blocking behavior', () => {
    const row = AppInstallPurchasePlatformAdapterBoundaryProofReadModel.adapterBoundaryRows[0];

    for (const invalidRow of [
      { ...row, providerApiExecutionClaim: 'executed' },
      { ...row, storeIntegrationClaim: 'claimed' },
      { ...row, childDeliveryClaim: 'delivered' },
      { ...row, runtimeReportDeliveryClaim: 'delivered' },
      { ...row, interceptionClaim: 'claimed' },
      { ...row, appBlockingClaim: 'claimed' },
      { ...row, childDataCustody: 'child-activity-data-included' },
      { ...row, ocentraHostedFamilyDataCustodyClaim: 'claimed' },
      { ...row, claimBoundary: 'platform adapter implemented' },
    ]) {
      expect(AppInstallPurchasePlatformAdapterBoundaryRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingPlatformAdapterBoundaryNonClaims(): void {
  it('rejects platform adapter boundary proof when provider delivery custody or blocking non-claims are removed', () => {
    const proof = AppInstallPurchasePlatformAdapterBoundaryProofReadModel;

    for (const claim of [
      'no-platform-adapter-implementation',
      'no-provider-api-execution',
      'no-child-device-delivery',
      'no-runtime-report-delivery',
      'no-child-activity-data',
      'no-app-blocking',
      'no-ocentra-hosted-family-data-custody',
    ] as const) {
      expect(
        AppInstallPurchasePlatformAdapterBoundaryProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

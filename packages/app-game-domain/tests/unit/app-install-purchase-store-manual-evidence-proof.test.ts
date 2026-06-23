import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseStoreManualEvidenceProofReadModel,
  AppInstallPurchaseStoreManualEvidenceProofSchema,
  AppInstallPurchaseStoreManualEvidenceRowSchema,
  summarizeAppInstallPurchaseStoreManualEvidence,
} from '@ocentra-parent/schema-domain/app-install-purchase-store-manual-evidence-proof';

describe('app install and purchase store manual evidence proof', () => {
  acceptsStoreManualEvidenceRowsWithoutStoreExecutionClaims();
  rejectsMissingStoreCoverageOrEvidenceRefs();
  rejectsProviderStoreAdapterDeliveryCustodyAndBlockingOverclaims();
  rejectsMissingStoreManualEvidenceNonClaims();
});

function acceptsStoreManualEvidenceRowsWithoutStoreExecutionClaims(): void {
  it('accepts store manual evidence rows that map platform proof readiness to store evidence states', () => {
    const proof = AppInstallPurchaseStoreManualEvidenceProofSchema.parse(
      AppInstallPurchaseStoreManualEvidenceProofReadModel
    );

    expect(summarizeAppInstallPurchaseStoreManualEvidence(proof)).toEqual({
      storeRows: 5,
      manualEvidenceRequiredRows: 2,
      policyReviewRequiredRows: 2,
      unavailableRows: 1,
      providerExecutedRows: 0,
      storeIntegratedRows: 0,
    });
    expect(
      proof.storeManualEvidenceRows.map((row) => `${row.platform}:${row.storeSurface}:${row.storeManualEvidenceState}`)
    ).toEqual([
      'windows:microsoft-store:manual-evidence-required',
      'macos:mac-app-store:manual-evidence-required',
      'linux:linux-package-manager:store-unavailable',
      'android:google-play:store-policy-review-required',
      'ios:apple-app-store:store-policy-review-required',
    ]);
    expect(proof.storeManualEvidenceRows.map((row) => row.sourceManualEvidenceRefs)).toEqual([
      ['windows-host-package-source-proof', 'windows-guarded-adapter-proof'],
      ['macos-signing-receipt-proof', 'macos-store-source-manual-proof'],
      ['linux-package-manager-source-path-proof'],
      ['android-device-owner-or-managed-profile-proof', 'google-play-policy-review-proof'],
      ['ios-family-controls-entitlement-proof', 'apple-review-proof'],
    ]);
    for (const row of proof.storeManualEvidenceRows) {
      expect(row.sourcePlatformProofReadinessProofVersion).toBe('app-install-purchase-platform-proof-readiness');
      expect(row.sourceManualEvidenceRefs.length).toBeGreaterThan(0);
      expect(row.providerApiExecutionClaim).toBe('not-executed');
      expect(row.storeIntegrationClaim).toBe('not-claimed');
      expect(row.platformAdapterClaim).toBe('not-implemented');
      expect(row.childDeviceDeliveryClaim).toBe('not-delivered');
      expect(row.runtimeWriterDeliveryClaim).toBe('not-delivered');
      expect(row.runtimeReportDeliveryClaim).toBe('not-delivered');
      expect(row.appBlockingClaim).toBe('not-claimed');
      expect(row.childDataCustody).toBe('no-child-activity-data');
      expect(row.ocentraHostedFamilyDataCustodyClaim).toBe('not-claimed');
    }
  });
}

function rejectsMissingStoreCoverageOrEvidenceRefs(): void {
  it('rejects proofs that omit a store row or required manual evidence', () => {
    const proof = AppInstallPurchaseStoreManualEvidenceProofReadModel;
    const row = proof.storeManualEvidenceRows[0];

    expect(
      AppInstallPurchaseStoreManualEvidenceProofSchema.safeParse({
        ...proof,
        storeManualEvidenceRows: proof.storeManualEvidenceRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseStoreManualEvidenceRowSchema.safeParse({
        ...row,
        sourcePlatformProofReadinessProofVersion: 'different-source-proof',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseStoreManualEvidenceRowSchema.safeParse({
        ...row,
        sourceManualEvidenceRefs: [],
      }).success
    ).toBe(false);
  });
}

function rejectsProviderStoreAdapterDeliveryCustodyAndBlockingOverclaims(): void {
  it('rejects rows that claim store execution, adapters, delivery, custody, or blocking', () => {
    const row = AppInstallPurchaseStoreManualEvidenceProofReadModel.storeManualEvidenceRows[0];

    for (const invalidRow of [
      { ...row, providerApiExecutionClaim: 'executed' },
      { ...row, storeIntegrationClaim: 'claimed' },
      { ...row, platformAdapterClaim: 'implemented' },
      { ...row, childDeviceDeliveryClaim: 'delivered' },
      { ...row, runtimeWriterDeliveryClaim: 'delivered' },
      { ...row, runtimeReportDeliveryClaim: 'delivered' },
      { ...row, appBlockingClaim: 'claimed' },
      { ...row, childDataCustody: 'child-activity-data-included' },
      { ...row, ocentraHostedFamilyDataCustodyClaim: 'claimed' },
      { ...row, claimBoundary: 'store execution implemented provider execution and runtime delivery' },
    ]) {
      expect(AppInstallPurchaseStoreManualEvidenceRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingStoreManualEvidenceNonClaims(): void {
  it('rejects proofs that remove a required store manual evidence non-claim', () => {
    const proof = AppInstallPurchaseStoreManualEvidenceProofReadModel;

    for (const claim of proof.nonClaims) {
      expect(
        AppInstallPurchaseStoreManualEvidenceProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

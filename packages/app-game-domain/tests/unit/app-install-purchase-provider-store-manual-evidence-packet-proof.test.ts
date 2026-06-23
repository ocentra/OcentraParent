import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseProviderStoreManualEvidencePacketProofReadModel,
  AppInstallPurchaseProviderStoreManualEvidencePacketProofSchema,
  AppInstallPurchaseProviderStoreManualEvidencePacketRowSchema,
  summarizeAppInstallPurchaseProviderStoreManualEvidencePacketProof,
} from '@ocentra-parent/schema-domain/app-install-purchase-provider-store-manual-evidence-packet-proof';

describe('app install purchase provider store manual evidence packet proof', () => {
  acceptsManualEvidencePacketRows();
  rejectsMissingCoverageOrEvidenceRefs();
  rejectsExecutionDeliveryAndCustodyOverclaims();
  rejectsMissingNonClaims();
});

function acceptsManualEvidencePacketRows(): void {
  it('links platform proof readiness and provider store preflight rows into manual evidence packets', () => {
    const proof = AppInstallPurchaseProviderStoreManualEvidencePacketProofSchema.parse(
      AppInstallPurchaseProviderStoreManualEvidencePacketProofReadModel
    );

    expect(summarizeAppInstallPurchaseProviderStoreManualEvidencePacketProof(proof)).toEqual({
      manualEvidencePacketRows: 5,
      packetReadyRows: 1,
      manualReviewRequiredRows: 3,
      providerUnavailableRows: 1,
      providerExecutedRows: 0,
      childDeliveredRows: 0,
    });
    expect(
      proof.manualEvidencePacketRows.map(
        (row) =>
          `${row.platform}:${row.storeSurface}:${row.sourcePlatformProofReadinessState}:${row.sourceProviderStorePreflightState}:${row.manualEvidencePacketState}`
      )
    ).toEqual([
      'windows:microsoft-store:manual-proof-required:preflight-ready:manual-evidence-packet-ready',
      'macos:mac-app-store:manual-proof-required:manual-provider-proof-required:manual-review-required',
      'linux:linux-package-manager:unavailable:provider-unavailable:provider-unavailable',
      'android:google-play:policy-blocked:manual-provider-proof-required:manual-review-required',
      'ios:apple-app-store:policy-blocked:manual-provider-proof-required:manual-review-required',
    ]);
    for (const row of proof.manualEvidencePacketRows) {
      expect(row.requiredManualEvidenceRefs.length).toBeGreaterThan(0);
      expect(row.requiredProviderEvidenceRefs.length).toBeGreaterThan(0);
      expect(row.runtimeWriterReceiptRefs.length).toBeGreaterThan(0);
      expect(row.auditEventRefs.length).toBeGreaterThan(0);
      expect(row.reportRuntimeRefs.length).toBeGreaterThan(0);
      expect(row.providerApiExecutionClaim).toBe('not-executed');
      expect(row.storeIntegrationClaim).toBe('not-claimed');
      expect(row.platformAdapterClaim).toBe('not-implemented');
      expect(row.runtimeWriterDeliveryClaim).toBe('not-delivered');
      expect(row.runtimeReportDeliveryClaim).toBe('not-delivered');
      expect(row.childDeviceDeliveryClaim).toBe('not-delivered');
      expect(row.appBlockingClaim).toBe('not-claimed');
      expect(row.childDataCustody).toBe('no-child-activity-data');
      expect(row.claimBoundary).toContain('parent-owned packet');
    }
  });
}

function rejectsMissingCoverageOrEvidenceRefs(): void {
  it('rejects proofs that omit store coverage or manual evidence refs', () => {
    const proof = AppInstallPurchaseProviderStoreManualEvidencePacketProofReadModel;
    const row = proof.manualEvidencePacketRows[0];

    expect(
      AppInstallPurchaseProviderStoreManualEvidencePacketProofSchema.safeParse({
        ...proof,
        manualEvidencePacketRows: proof.manualEvidencePacketRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProviderStoreManualEvidencePacketRowSchema.safeParse({
        ...row,
        sourceProviderStorePreflightRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProviderStoreManualEvidencePacketRowSchema.safeParse({
        ...row,
        requiredManualEvidenceRefs: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProviderStoreManualEvidencePacketRowSchema.safeParse({
        ...row,
        requiredProviderEvidenceRefs: [],
      }).success
    ).toBe(false);
  });
}

function rejectsExecutionDeliveryAndCustodyOverclaims(): void {
  it('rejects rows that claim provider execution delivery custody or blocking', () => {
    const row = AppInstallPurchaseProviderStoreManualEvidencePacketProofReadModel.manualEvidencePacketRows[0];

    for (const invalidRow of [
      { ...row, manualEvidencePacketState: 'executed' },
      { ...row, providerApiExecutionClaim: 'executed' },
      { ...row, googlePlayExecutionClaim: 'executed' },
      { ...row, appleAppStoreExecutionClaim: 'executed' },
      { ...row, microsoftStoreExecutionClaim: 'executed' },
      { ...row, storeIntegrationClaim: 'claimed' },
      { ...row, platformAdapterClaim: 'implemented' },
      { ...row, runtimeWriterDeliveryClaim: 'delivered' },
      { ...row, runtimeReportDeliveryClaim: 'delivered' },
      { ...row, childDeviceDeliveryClaim: 'delivered' },
      { ...row, appBlockingClaim: 'claimed' },
      { ...row, childDataCustody: 'child-activity-data-included' },
      { ...row, claimBoundary: 'provider executed Google Play and delivered child-device approval' },
    ]) {
      expect(AppInstallPurchaseProviderStoreManualEvidencePacketRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingNonClaims(): void {
  it('rejects proof when required non-claims are removed', () => {
    const proof = AppInstallPurchaseProviderStoreManualEvidencePacketProofReadModel;

    for (const claim of [
      'no-google-play-execution',
      'no-apple-app-store-execution',
      'no-microsoft-store-execution',
      'no-provider-api-execution',
      'no-store-integration',
      'no-platform-adapter-implementation',
      'no-runtime-writer-delivery',
      'no-runtime-report-delivery',
      'no-child-device-delivery',
      'no-app-blocking',
      'no-child-activity-data',
      'no-ocentra-hosted-family-data-custody',
    ] as const) {
      expect(
        AppInstallPurchaseProviderStoreManualEvidencePacketProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

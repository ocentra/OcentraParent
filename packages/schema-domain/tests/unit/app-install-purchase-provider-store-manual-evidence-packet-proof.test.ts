import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseProviderStoreManualEvidencePacketProofReadModel,
  AppInstallPurchaseProviderStoreManualEvidencePacketProofSchema,
  AppInstallPurchaseProviderStoreManualEvidencePacketRowSchema,
  summarizeAppInstallPurchaseProviderStoreManualEvidencePacketProof,
} from '@ocentra-parent/schema-domain/app-install-purchase-provider-store-manual-evidence-packet-proof';

describe('app install and purchase provider store manual evidence packet proof', () => {
  acceptsManualEvidencePacketRows();
  rejectsMissingRefsOverclaimsAndMissingNonClaims();
});

function acceptsManualEvidencePacketRows(): void {
  it('accepts provider store manual evidence packet rows that stay packet-only and execution-free', () => {
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

    const windowsRow = proof.manualEvidencePacketRows.find((row) => row.platform === 'windows');
    if (windowsRow === undefined) {
      throw new Error('missing manual evidence packet row for windows');
    }

    expect(windowsRow.requiredManualEvidenceRefs.length).toBeGreaterThan(0);
    expect(windowsRow.requiredProviderEvidenceRefs.length).toBeGreaterThan(0);
    expect(windowsRow.runtimeWriterReceiptRefs.length).toBeGreaterThan(0);
    expect(windowsRow.auditEventRefs.length).toBeGreaterThan(0);
    expect(windowsRow.reportRuntimeRefs.length).toBeGreaterThan(0);

    for (const row of proof.manualEvidencePacketRows) {
      expect(row.providerApiExecutionClaim).toBe('not-executed');
      expect(row.googlePlayExecutionClaim).toBe('not-executed');
      expect(row.appleAppStoreExecutionClaim).toBe('not-executed');
      expect(row.microsoftStoreExecutionClaim).toBe('not-executed');
      expect(row.storeIntegrationClaim).toBe('not-claimed');
      expect(row.platformAdapterClaim).toBe('not-implemented');
      expect(row.runtimeWriterDeliveryClaim).toBe('not-delivered');
      expect(row.runtimeReportDeliveryClaim).toBe('not-delivered');
      expect(row.childDeviceDeliveryClaim).toBe('not-delivered');
      expect(row.appBlockingClaim).toBe('not-claimed');
      expect(row.childDataCustody).toBe('no-child-activity-data');
      expect(row.ocentraHostedFamilyDataCustodyClaim).toBe('not-claimed');
      expect(row.claimBoundary).toContain('platform manual evidence refs');
      expect(row.claimBoundary).toContain('no provider API execution');
    }
  });
}

function rejectsMissingRefsOverclaimsAndMissingNonClaims(): void {
  it('rejects packet rows that omit required refs or invent execution, delivery, custody, or blocking claims', () => {
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
        requiredProviderEvidenceRefs: [],
      }).success
    ).toBe(false);

    for (const invalidRow of [
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
      { ...row, ocentraHostedFamilyDataCustodyClaim: 'claimed' },
      { ...row, claimBoundary: 'manual evidence packet is complete' },
    ]) {
      expect(AppInstallPurchaseProviderStoreManualEvidencePacketRowSchema.safeParse(invalidRow).success).toBe(false);
    }

    expect(
      AppInstallPurchaseProviderStoreManualEvidencePacketProofSchema.safeParse({
        ...proof,
        nonClaims: proof.nonClaims.filter((claim) => claim !== 'no-provider-api-execution'),
      }).success
    ).toBe(false);
  });
}

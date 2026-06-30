import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseProviderStorePlatformEvidenceProofReadModel,
  AppInstallPurchaseProviderStorePlatformEvidenceProofSchema,
  AppInstallPurchaseProviderStorePlatformEvidenceRowSchema,
  summarizeAppInstallPurchaseProviderStorePlatformEvidenceProof,
} from '@ocentra-parent/schema-domain/app-install-purchase-provider-store-platform-evidence-proof';

describe('app install and purchase provider store platform evidence proof', () => {
  acceptsProviderStorePlatformEvidenceRows();
  rejectsMissingRefsOverclaimsAndMissingNonClaims();
});

function acceptsProviderStorePlatformEvidenceRows(): void {
  it('accepts provider store platform evidence rows that keep provider, adapter, delivery, and custody claims unimplemented', () => {
    const proof = AppInstallPurchaseProviderStorePlatformEvidenceProofSchema.parse(
      AppInstallPurchaseProviderStorePlatformEvidenceProofReadModel
    );

    expect(summarizeAppInstallPurchaseProviderStorePlatformEvidenceProof(proof)).toEqual({
      providerStorePlatformEvidenceRows: 5,
      manualRequiredRows: 2,
      platformUnavailableRows: 1,
      blockedBeforeClaimRows: 2,
      providerExecutedRows: 0,
      platformAdapterImplementedRows: 0,
      childDeliveredRows: 0,
    });
    expect(
      proof.providerStorePlatformEvidenceRows.map(
        (row) => `${row.platform}:${row.storeSurface}:${row.providerStorePlatformEvidenceState}`
      )
    ).toEqual([
      'windows:microsoft-store:manual-provider-store-platform-evidence-required',
      'macos:mac-app-store:manual-provider-store-platform-evidence-required',
      'linux:linux-package-manager:platform-unavailable',
      'android:google-play:blocked-before-claim',
      'ios:apple-app-store:blocked-before-claim',
    ]);

    const windowsRow = proof.providerStorePlatformEvidenceRows.find((row) => row.platform === 'windows');
    if (windowsRow === undefined) {
      throw new Error('missing provider store platform evidence row for windows');
    }

    expect(windowsRow.missingProviderStoreArtifactRefs).toEqual([
      'missing-microsoft-store-provider-credential-proof',
      'missing-microsoft-store-provider-api-response-proof',
      'missing-billing-provider-contact-proof',
    ]);
    expect(windowsRow.missingPlatformArtifactRefs).toEqual([
      'missing-windows-production-platform-adapter-execution-proof',
      'missing-windows-platform-interception-policy-proof',
    ]);
    expect(windowsRow.missingChildDeviceArtifactRefs).toEqual([
      'missing-windows-child-device-delivery-receipt-proof',
    ]);

    for (const row of proof.providerStorePlatformEvidenceRows) {
      expect(row.productClaimApprovalClaim).toBe('not-claimed');
      expect(row.providerApiExecutionClaim).toBe('not-executed');
      expect(row.storeIntegrationClaim).toBe('not-claimed');
      expect(row.platformInterceptionClaim).toBe('not-claimed');
      expect(row.productionPlatformAdapterClaim).toBe('not-implemented');
      expect(row.runtimeWriterExecutionClaim).toBe('not-executed');
      expect(row.runtimeWriterDeliveryClaim).toBe('not-delivered');
      expect(row.childDeviceDeliveryClaim).toBe('not-delivered');
      expect(row.runtimeReportDeliveryClaim).toBe('not-delivered');
      expect(row.childDataCustody).toBe('no-child-activity-data');
      expect(row.claimBoundary).toContain('no provider API execution');
      expect(row.claimBoundary).toContain('no child-device delivery');
    }
  });
}

function rejectsMissingRefsOverclaimsAndMissingNonClaims(): void {
  it('rejects provider store platform evidence rows that omit refs or invent approval, execution, adapter, delivery, or custody claims', () => {
    const proof = AppInstallPurchaseProviderStorePlatformEvidenceProofReadModel;
    const row = proof.providerStorePlatformEvidenceRows[0];

    expect(
      AppInstallPurchaseProviderStorePlatformEvidenceProofSchema.safeParse({
        ...proof,
        providerStorePlatformEvidenceRows: proof.providerStorePlatformEvidenceRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProviderStorePlatformEvidenceRowSchema.safeParse({
        ...row,
        sourceProviderStoreExecutionPreflightRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProviderStorePlatformEvidenceRowSchema.safeParse({
        ...row,
        sourceWindowsPackageSourceRuntimeHandoffRowId: '',
      }).success
    ).toBe(false);

    for (const invalidRow of [
      { ...row, productClaimApprovalClaim: 'approved' },
      { ...row, providerApiExecutionClaim: 'executed' },
      { ...row, storeIntegrationClaim: 'claimed' },
      { ...row, platformInterceptionClaim: 'claimed' },
      { ...row, productionPlatformAdapterClaim: 'implemented' },
      { ...row, runtimeWriterExecutionClaim: 'executed' },
      { ...row, runtimeWriterDeliveryClaim: 'delivered' },
      { ...row, childDeviceDeliveryClaim: 'delivered' },
      { ...row, runtimeReportDeliveryClaim: 'delivered' },
      { ...row, portalApprovalUiClaim: 'claimed' },
      { ...row, portalReportUiClaim: 'claimed' },
      { ...row, childDataCustody: 'child-activity-data-included' },
      { ...row, claimBoundary: 'provider store platform evidence is complete' },
    ]) {
      expect(AppInstallPurchaseProviderStorePlatformEvidenceRowSchema.safeParse(invalidRow).success).toBe(false);
    }

    expect(
      AppInstallPurchaseProviderStorePlatformEvidenceProofSchema.safeParse({
        ...proof,
        nonClaims: proof.nonClaims.filter((claim) => claim !== 'no-provider-api-execution'),
      }).success
    ).toBe(false);
  });
}

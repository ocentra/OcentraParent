import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchasePlatformAdapterEvidenceGapProofReadModel,
  AppInstallPurchasePlatformAdapterEvidenceGapProofSchema,
  AppInstallPurchasePlatformAdapterEvidenceGapRowSchema,
  summarizeAppInstallPurchasePlatformAdapterEvidenceGapProof,
} from '../../src/app-install-purchase-platform-adapter-evidence-gap-proof';

describe('schema-domain app install purchase platform adapter evidence gap proof', () => {
  acceptsPlatformAdapterEvidenceGapRows();
  rejectsMissingAdapterEvidenceRefs();
  rejectsProviderStorePlatformDeliveryAndCustodyOverclaims();
  rejectsMissingAdapterEvidenceNonClaims();
});

function acceptsPlatformAdapterEvidenceGapRows(): void {
  it('links provider store api execution rows to platform adapter evidence requirements without implementing adapters', () => {
    const proof = AppInstallPurchasePlatformAdapterEvidenceGapProofSchema.parse(
      AppInstallPurchasePlatformAdapterEvidenceGapProofReadModel
    );

    expect(summarizeAppInstallPurchasePlatformAdapterEvidenceGapProof(proof)).toEqual({
      platformAdapterEvidenceGapRows: 5,
      adapterEvidenceGapRows: 1,
      manualAdapterEvidenceRequiredRows: 1,
      platformUnavailableRows: 1,
      blockedBeforeClaimRows: 2,
      realAdapterEvidenceRows: 0,
      adapterImplementedRows: 0,
      productClaimApprovedRows: 0,
    });
    expect(
      proof.platformAdapterEvidenceGapRows.map(
        (row) =>
          `${row.platform}:${row.storeSurface}:${row.sourceProviderStoreApiExecutionState}:${row.sourcePlatformProofReadinessState}:${row.platformAdapterEvidenceGapState}:${row.realPlatformAdapterEvidenceState}`
      )
    ).toEqual([
      'windows:microsoft-store:execution-ready:manual-proof-required:adapter-evidence-gap:no-real-adapter-evidence-attached',
      'macos:mac-app-store:manual-required:manual-proof-required:manual-adapter-evidence-required:no-real-adapter-evidence-attached',
      'linux:linux-package-manager:unavailable:unavailable:platform-unavailable:no-real-adapter-evidence-attached',
      'android:google-play:blocked-before-claim:policy-blocked:blocked-before-claim:no-real-adapter-evidence-attached',
      'ios:apple-app-store:blocked-before-claim:policy-blocked:blocked-before-claim:no-real-adapter-evidence-attached',
    ]);

    for (const row of proof.platformAdapterEvidenceGapRows) {
      expect(row.providerStoreApiExecutionEvidenceRefs.length).toBeGreaterThan(0);
      expect(row.sourcePlatformProofReadinessProofVersion).toBe('app-install-purchase-platform-proof-readiness');
      expect(row.requiredPlatformAdapterEvidenceRefs.length).toBeGreaterThan(0);
      expect(row.requiredManualPlatformEvidenceRefs.length).toBeGreaterThan(0);
      expect(row.requiredProviderCredentialRefs.length).toBeGreaterThan(0);
      expect(row.requiredPortalTestRefs.length).toBeGreaterThan(0);
      expect(row.requiredChildDeliveryRefs.length).toBeGreaterThan(0);
      expect(row.realPlatformAdapterEvidenceState).toBe('no-real-adapter-evidence-attached');
      expect(row.platformAdapterClaim).toBe('not-implemented');
      expect(row.providerApiExecutionClaim).toBe('not-executed');
      expect(row.productClaimApprovalClaim).toBe('not-claimed');
      expect(row.childDeviceDeliveryClaim).toBe('not-delivered');
      expect(row.claimBoundary).toContain('platform adapter evidence gap proof boundary only');
    }
  });
}

function rejectsMissingAdapterEvidenceRefs(): void {
  it('rejects rows that drop provider store source or platform adapter evidence refs', () => {
    const proof = AppInstallPurchasePlatformAdapterEvidenceGapProofReadModel;
    const row = proof.platformAdapterEvidenceGapRows[0];

    expect(
      AppInstallPurchasePlatformAdapterEvidenceGapProofSchema.safeParse({
        ...proof,
        platformAdapterEvidenceGapRows: proof.platformAdapterEvidenceGapRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchasePlatformAdapterEvidenceGapRowSchema.safeParse({
        ...row,
        sourceProviderStoreApiExecutionRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchasePlatformAdapterEvidenceGapRowSchema.safeParse({
        ...row,
        providerStoreApiExecutionEvidenceRefs: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchasePlatformAdapterEvidenceGapRowSchema.safeParse({
        ...row,
        sourcePlatformProofReadinessState: 'policy-blocked',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchasePlatformAdapterEvidenceGapRowSchema.safeParse({
        ...row,
        requiredPlatformAdapterEvidenceRefs: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchasePlatformAdapterEvidenceGapRowSchema.safeParse({
        ...row,
        requiredChildDeliveryRefs: [],
      }).success
    ).toBe(false);
  });
}

function rejectsProviderStorePlatformDeliveryAndCustodyOverclaims(): void {
  it('rejects rows that claim product approval provider execution adapters delivery blocking or custody', () => {
    const row = AppInstallPurchasePlatformAdapterEvidenceGapProofReadModel.platformAdapterEvidenceGapRows[0];

    for (const invalidRow of [
      { ...row, platformAdapterEvidenceGapState: 'adapter-implemented' },
      { ...row, realPlatformAdapterEvidenceState: 'real-adapter-evidence-attached' },
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
      { ...row, claimBoundary: 'platform adapter proof implements real adapters' },
    ]) {
      expect(AppInstallPurchasePlatformAdapterEvidenceGapRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingAdapterEvidenceNonClaims(): void {
  it('rejects proof when required platform adapter evidence gap non-claims are removed', () => {
    const proof = AppInstallPurchasePlatformAdapterEvidenceGapProofReadModel;

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
      'no-real-platform-adapter-evidence-attached',
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
        AppInstallPurchasePlatformAdapterEvidenceGapProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

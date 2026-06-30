import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseProductClaimStoreHandoffProofReadModel,
  AppInstallPurchaseProductClaimStoreHandoffProofSchema,
  AppInstallPurchaseProductClaimStoreHandoffRowSchema,
  summarizeAppInstallPurchaseProductClaimStoreHandoffProof,
} from '@ocentra-parent/schema-domain/app-install-purchase-product-claim-store-handoff-proof';

describe('app install purchase product claim store handoff proof', () => {
  acceptsStoreHandoffRows();
  rejectsDroppedSourceAndEvidenceRefs();
  rejectsProductClaimProviderPortalDeliveryBlockingAndCustodyClaims();
  rejectsMissingNonClaims();
});

function acceptsStoreHandoffRows(): void {
  it('links safe parent workflow rows and manual evidence packets without approving product claims', () => {
    const proof = AppInstallPurchaseProductClaimStoreHandoffProofSchema.parse(
      AppInstallPurchaseProductClaimStoreHandoffProofReadModel
    );

    expect(summarizeAppInstallPurchaseProductClaimStoreHandoffProof(proof)).toEqual({
      productClaimStoreHandoffRows: 5,
      reviewReadyRows: 1,
      manualRequiredRows: 1,
      unavailableRows: 3,
      productClaimApprovedRows: 0,
      providerExecutedRows: 0,
    });
    expect(
      proof.productClaimStoreHandoffRows.map(
        (row) =>
          `${row.platform}:${row.storeSurface}:${row.sourceSafeParentWorkflowState}:${row.sourceManualEvidencePacketState}:${row.storeHandoffState}`
      )
    ).toEqual([
      'windows:microsoft-store:safe-parent-review-ready:manual-evidence-packet-ready:store-handoff-review-ready',
      'macos:mac-app-store:manual-parent-review-required:manual-review-required:store-handoff-manual-required',
      'linux:linux-package-manager:unsupported-store-workflow-blocked:provider-unavailable:store-handoff-unavailable',
      'android:google-play:unsupported-store-workflow-blocked:manual-review-required:store-handoff-unavailable',
      'ios:apple-app-store:unsupported-store-workflow-blocked:manual-review-required:store-handoff-unavailable',
    ]);
    for (const row of proof.productClaimStoreHandoffRows) {
      expect(row.parentWorkflowRefs.length).toBe(2);
      expect(row.requiredManualEvidenceRefs.length).toBeGreaterThan(0);
      expect(row.requiredProviderEvidenceRefs.length).toBeGreaterThan(0);
      expect(row.requiredPortalTestRefs.length).toBe(1);
      expect(row.requiredChildDeliveryRefs.length).toBe(1);
      expect(row.requiredProviderStoreExecutionRefs.length).toBe(1);
      expect(row.requiredPlatformAdapterRefs.length).toBe(1);
      expect(row.productClaimApprovedClaim).toBe('not-claimed');
      expect(row.providerApiExecutionClaim).toBe('not-executed');
      expect(row.claimBoundary).toContain('product claim store handoff proof only');
    }
  });
}

function rejectsDroppedSourceAndEvidenceRefs(): void {
  it('rejects rows that drop safe workflow manual packet or follow-up evidence refs', () => {
    const proof = AppInstallPurchaseProductClaimStoreHandoffProofReadModel;
    const row = proof.productClaimStoreHandoffRows[0];

    expect(
      AppInstallPurchaseProductClaimStoreHandoffProofSchema.safeParse({
        ...proof,
        productClaimStoreHandoffRows: proof.productClaimStoreHandoffRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProductClaimStoreHandoffRowSchema.safeParse({
        ...row,
        sourceSafeParentWorkflowRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProductClaimStoreHandoffRowSchema.safeParse({
        ...row,
        sourceManualEvidencePacketRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProductClaimStoreHandoffRowSchema.safeParse({
        ...row,
        requiredProviderStoreExecutionRefs: [],
      }).success
    ).toBe(false);
  });
}

function rejectsProductClaimProviderPortalDeliveryBlockingAndCustodyClaims(): void {
  it('rejects rows that claim product approval portal UI provider execution delivery blocking or custody', () => {
    const row = AppInstallPurchaseProductClaimStoreHandoffProofReadModel.productClaimStoreHandoffRows[0];

    for (const invalidRow of [
      { ...row, storeHandoffState: 'store-handoff-approved' },
      { ...row, productClaimApprovedClaim: 'claimed' },
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
      { ...row, claimBoundary: 'store handoff with product claim approval and provider execution' },
    ]) {
      expect(AppInstallPurchaseProductClaimStoreHandoffRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingNonClaims(): void {
  it('rejects proof when required non-claims are removed', () => {
    const proof = AppInstallPurchaseProductClaimStoreHandoffProofReadModel;

    for (const claim of [
      'no-product-claim-approved',
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
        AppInstallPurchaseProductClaimStoreHandoffProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseProductClaimPortalTestReadinessProofReadModel,
  AppInstallPurchaseProductClaimPortalTestReadinessProofSchema,
  AppInstallPurchaseProductClaimPortalTestReadinessRowSchema,
  summarizeAppInstallPurchaseProductClaimPortalTestReadinessProof,
} from '@ocentra-parent/schema-domain/app-install-purchase-product-claim-portal-test-readiness-proof';

describe('app install purchase product claim portal test readiness proof', () => {
  acceptsPortalTestReadinessRows();
  rejectsMissingRefsAndCoverage();
  rejectsPortalProviderDeliveryBlockingAndCustodyClaims();
  rejectsMissingNonClaims();
});

function acceptsPortalTestReadinessRows(): void {
  it('names portal approval and report test refs without approving product claims', () => {
    const proof = AppInstallPurchaseProductClaimPortalTestReadinessProofSchema.parse(
      AppInstallPurchaseProductClaimPortalTestReadinessProofReadModel
    );

    expect(summarizeAppInstallPurchaseProductClaimPortalTestReadinessProof(proof)).toEqual({
      portalTestReadinessRows: 5,
      portalTestReadyRows: 1,
      manualPortalTestRequiredRows: 1,
      unsupportedPortalTestBlockedRows: 3,
      portalUiClaimedRows: 0,
      productClaimApprovedRows: 0,
    });
    expect(
      proof.portalTestReadinessRows.map(
        (row) =>
          `${row.platform}:${row.storeSurface}:${row.sourceProductClaimGateState}:${row.portalTestReadinessState}`
      )
    ).toEqual([
      'windows:microsoft-store:product-claim-denied:portal-test-ready',
      'macos:mac-app-store:manual-required:manual-portal-test-required',
      'linux:linux-package-manager:blocked:unsupported-portal-test-blocked',
      'android:google-play:blocked:unsupported-portal-test-blocked',
      'ios:apple-app-store:blocked:unsupported-portal-test-blocked',
    ]);
    for (const row of proof.portalTestReadinessRows) {
      expect(row.portalApprovalTestRef).toContain('portal-approval-report-test');
      expect(row.portalReportTestRef).toContain('portal-report-test');
      expect(row.requiredChildDeliveryRefs.length).toBe(1);
      expect(row.requiredProviderStoreExecutionRefs.length).toBe(1);
      expect(row.requiredPlatformAdapterRefs.length).toBe(1);
      expect(row.portalApprovalUiClaim).toBe('not-claimed');
      expect(row.portalReportUiClaim).toBe('not-claimed');
      expect(row.providerApiExecutionClaim).toBe('not-executed');
      expect(row.claimBoundary).toContain('portal test readiness proof only');
    }
  });
}

function rejectsMissingRefsAndCoverage(): void {
  it('rejects missing source gate linkage portal refs and store coverage', () => {
    const proof = AppInstallPurchaseProductClaimPortalTestReadinessProofReadModel;
    const row = proof.portalTestReadinessRows[0];

    expect(
      AppInstallPurchaseProductClaimPortalTestReadinessProofSchema.safeParse({
        ...proof,
        portalTestReadinessRows: proof.portalTestReadinessRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProductClaimPortalTestReadinessRowSchema.safeParse({
        ...row,
        sourceProductClaimGateRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProductClaimPortalTestReadinessRowSchema.safeParse({
        ...row,
        portalReportTestRef: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProductClaimPortalTestReadinessRowSchema.safeParse({
        ...row,
        requiredProviderStoreExecutionRefs: [],
      }).success
    ).toBe(false);
  });
}

function rejectsPortalProviderDeliveryBlockingAndCustodyClaims(): void {
  it('rejects rows that claim portal UI provider execution delivery blocking or custody', () => {
    const row = AppInstallPurchaseProductClaimPortalTestReadinessProofReadModel.portalTestReadinessRows[0];

    for (const invalidRow of [
      { ...row, portalTestReadinessState: 'approved-product-claim' },
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
      { ...row, claimBoundary: 'portal UI executes provider and approves product claim' },
    ]) {
      expect(AppInstallPurchaseProductClaimPortalTestReadinessRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingNonClaims(): void {
  it('rejects proof when required non-claims are removed', () => {
    const proof = AppInstallPurchaseProductClaimPortalTestReadinessProofReadModel;

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
        AppInstallPurchaseProductClaimPortalTestReadinessProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

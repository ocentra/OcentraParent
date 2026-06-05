import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseRuntimeReportDeliveryProofReadModel,
  AppInstallPurchaseRuntimeReportDeliveryProofSchema,
  AppInstallPurchaseRuntimeReportDeliveryRowSchema,
  summarizeAppInstallPurchaseRuntimeReportDeliveryProof,
} from '../src/app-install-purchase-runtime-report-delivery-proof';

describe('app install and purchase runtime report delivery proof', () => {
  acceptsRuntimeReportDeliveryRowsWithoutPortalOrChildDeliveryClaims();
  rejectsMissingRuntimeReportDeliveryCoverageOrReceipts();
  rejectsPortalProviderChildCustodyAndBlockingOverclaims();
  rejectsMissingRuntimeReportDeliveryNonClaims();
});

function acceptsRuntimeReportDeliveryRowsWithoutPortalOrChildDeliveryClaims(): void {
  it('accepts parent-owned runtime report delivery rows with receipts and no portal UI claim', () => {
    const proof = AppInstallPurchaseRuntimeReportDeliveryProofSchema.parse(
      AppInstallPurchaseRuntimeReportDeliveryProofReadModel
    );

    expect(summarizeAppInstallPurchaseRuntimeReportDeliveryProof(proof)).toEqual({
      runtimeReportDeliveryRows: 4,
      deliveredRows: 4,
      receiptRows: 4,
      portalReportUiRows: 0,
      childDeviceDeliveryRows: 0,
    });
    expect(proof.runtimeReportDeliveryRows.map((row) => row.reportSurface)).toEqual([
      'request-audit-history',
      'parent-decision-audit-history',
      'child-facing-state-report',
      'platform-limitation-report',
    ]);
    for (const row of proof.runtimeReportDeliveryRows) {
      expect(row.deliveryState).toBe('runtime-report-delivered');
      expect(row.runtimeReportReceiptRef).toContain(row.reportSurface);
      expect(row.compilerOutputReportRef).toBe('parent-owned-storage-report-output-ref');
      expect(row.sourceChildArtifactRefs.length).toBeGreaterThan(0);
      expect(row.parentAuthorized).toBe(true);
      expect(row.rawEvidenceExcludedFromOutput).toBe(true);
      expect(row.childDetailMinimized).toBe(true);
      expect(row.tempDeletionConfirmed).toBe(true);
      expect(row.localEvidenceMutated).toBe(false);
      expect(row.ocentraHostedReportRetained).toBe(false);
      expect(row.runtimeReportDeliveryClaim).toBe('parent-runtime-delivered');
      expect(row.portalReportUiClaim).toBe('not-claimed');
      expect(row.providerApiExecutionClaim).toBe('not-claimed');
      expect(row.storeIntegrationClaim).toBe('not-claimed');
      expect(row.platformAdapterClaim).toBe('not-implemented');
      expect(row.childDeviceDeliveryClaim).toBe('not-delivered');
      expect(row.childDataCustody).toBe('no-child-activity-data');
      expect(row.appBlockingClaim).toBe('not-claimed');
      expect(row.ocentraHostedFamilyDataCustodyClaim).toBe('not-claimed');
      expect(row.claimBoundary).toContain('no portal report UI');
      expect(row.claimBoundary).toContain('no child-device delivery');
    }
  });
}

function rejectsMissingRuntimeReportDeliveryCoverageOrReceipts(): void {
  it('rejects proofs that omit delivery rows receipts or compiler outputs', () => {
    const proof = AppInstallPurchaseRuntimeReportDeliveryProofReadModel;
    const row = proof.runtimeReportDeliveryRows[0];

    expect(
      AppInstallPurchaseRuntimeReportDeliveryProofSchema.safeParse({
        ...proof,
        runtimeReportDeliveryRows: proof.runtimeReportDeliveryRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseRuntimeReportDeliveryRowSchema.safeParse({
        ...row,
        runtimeReportReceiptRef: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseRuntimeReportDeliveryRowSchema.safeParse({
        ...row,
        compilerOutputReportRef: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseRuntimeReportDeliveryRowSchema.safeParse({
        ...row,
        sourceChildArtifactRefs: [],
      }).success
    ).toBe(false);
  });
}

function rejectsPortalProviderChildCustodyAndBlockingOverclaims(): void {
  it('rejects rows that claim portal provider store adapter child delivery custody or blocking behavior', () => {
    const row = AppInstallPurchaseRuntimeReportDeliveryProofReadModel.runtimeReportDeliveryRows[0];

    for (const invalidRow of [
      { ...row, portalReportUiClaim: 'claimed' },
      { ...row, providerApiExecutionClaim: 'claimed' },
      { ...row, storeIntegrationClaim: 'claimed' },
      { ...row, platformAdapterClaim: 'implemented' },
      { ...row, childDeviceDeliveryClaim: 'delivered' },
      { ...row, childDataCustody: 'child-activity-data-included' },
      { ...row, appBlockingClaim: 'claimed' },
      { ...row, ocentraHostedFamilyDataCustodyClaim: 'claimed' },
      { ...row, localEvidenceMutated: true },
      { ...row, ocentraHostedReportRetained: true },
      { ...row, claimBoundary: 'portal report UI delivered provider data to child device' },
    ]) {
      expect(AppInstallPurchaseRuntimeReportDeliveryRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingRuntimeReportDeliveryNonClaims(): void {
  it('rejects runtime report delivery proofs when required non-claims are removed', () => {
    const proof = AppInstallPurchaseRuntimeReportDeliveryProofReadModel;

    for (const claim of [
      'no-portal-report-ui',
      'no-provider-api-execution',
      'no-store-integration',
      'no-platform-adapter',
      'no-child-device-delivery',
      'no-child-activity-data',
      'no-app-blocking',
      'no-ocentra-hosted-family-data-custody',
    ] as const) {
      expect(
        AppInstallPurchaseRuntimeReportDeliveryProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

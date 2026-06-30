import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseProviderStoreReportStatusRuntimeProofReadModel,
  AppInstallPurchaseProviderStoreReportStatusRuntimeProofSchema,
  AppInstallPurchaseProviderStoreReportStatusRuntimeRowSchema,
  summarizeAppInstallPurchaseProviderStoreReportStatusRuntimeProof,
} from '@ocentra-parent/schema-domain/app-install-purchase-provider-store-report-status-runtime-proof';

describe('app install and purchase provider store report status runtime proof', () => {
  acceptsProviderStoreReportStatusRuntimeRows();
  rejectsMissingRefsOverclaimsAndMissingNonClaims();
});

function acceptsProviderStoreReportStatusRuntimeRows(): void {
  it('accepts provider store report status runtime rows that stay parent-owned and non-delivered', () => {
    const proof = AppInstallPurchaseProviderStoreReportStatusRuntimeProofSchema.parse(
      AppInstallPurchaseProviderStoreReportStatusRuntimeProofReadModel
    );

    expect(summarizeAppInstallPurchaseProviderStoreReportStatusRuntimeProof(proof)).toEqual({
      providerStoreReportStatusRuntimeRows: 5,
      readyRows: 1,
      manualRequiredRows: 3,
      unavailableRows: 1,
      runtimeReportWriterLinkedRows: 5,
      providerExecutedRows: 0,
      externallyDeliveredRows: 0,
    });
    expect(
      proof.providerStoreReportStatusRuntimeRows.map(
        (row) =>
          `${row.platform}:${row.storeSurface}:${row.sourceProviderStoreExecutionReadinessState}:${row.providerStoreReportStatusRuntimeState}`
      )
    ).toEqual([
      'windows:microsoft-store:provider-store-execution-ready:provider-store-report-status-ready',
      'macos:mac-app-store:manual-required:manual-required',
      'linux:linux-package-manager:unavailable:unavailable',
      'android:google-play:manual-required:manual-required',
      'ios:apple-app-store:manual-required:manual-required',
    ]);

    const windowsRow = proof.providerStoreReportStatusRuntimeRows.find((row) => row.platform === 'windows');
    if (windowsRow === undefined) {
      throw new Error('missing provider store report status runtime row for windows');
    }

    expect(windowsRow.sourceRuntimeReportWriterDeliveryRowIds.length).toBeGreaterThan(0);
    expect(windowsRow.sourceRuntimeReportWriterReceiptRefs.length).toBeGreaterThan(0);
    expect(windowsRow.reportCompilerOutputRefs.length).toBeGreaterThan(0);
    expect(windowsRow.reportReceiptRefs.length).toBeGreaterThan(0);
    expect(windowsRow.requiredProofRefs.length).toBeGreaterThan(0);

    for (const row of proof.providerStoreReportStatusRuntimeRows) {
      expect(row.providerApiExecutionClaim).toBe('not-executed');
      expect(row.storeIntegrationClaim).toBe('not-claimed');
      expect(row.portalReportUiClaim).toBe('not-claimed');
      expect(row.runtimeReportDeliveryClaim).toBe('not-delivered');
      expect(row.platformAdapterClaim).toBe('not-implemented');
      expect(row.childDeviceDeliveryClaim).toBe('not-delivered');
      expect(row.appBlockingClaim).toBe('not-claimed');
      expect(row.childDataCustody).toBe('no-child-activity-data');
      expect(row.ocentraHostedFamilyDataCustodyClaim).toBe('not-claimed');
      expect(row.claimBoundary).toContain('runtime report writer receipts');
      expect(row.claimBoundary).toContain('no external runtime report delivery');
    }
  });
}

function rejectsMissingRefsOverclaimsAndMissingNonClaims(): void {
  it('rejects runtime rows that omit writer receipts or invent execution, delivery, portal, custody, or blocking claims', () => {
    const proof = AppInstallPurchaseProviderStoreReportStatusRuntimeProofReadModel;
    const row = proof.providerStoreReportStatusRuntimeRows[0];

    expect(
      AppInstallPurchaseProviderStoreReportStatusRuntimeProofSchema.safeParse({
        ...proof,
        providerStoreReportStatusRuntimeRows: proof.providerStoreReportStatusRuntimeRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProviderStoreReportStatusRuntimeRowSchema.safeParse({
        ...row,
        sourceProviderStoreExecutionReadinessRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProviderStoreReportStatusRuntimeRowSchema.safeParse({
        ...row,
        sourceRuntimeReportWriterDeliveryRowIds: [],
      }).success
    ).toBe(false);

    for (const invalidRow of [
      { ...row, providerApiExecutionClaim: 'executed' },
      { ...row, storeIntegrationClaim: 'claimed' },
      { ...row, portalReportUiClaim: 'claimed' },
      { ...row, runtimeReportDeliveryClaim: 'delivered' },
      { ...row, platformAdapterClaim: 'implemented' },
      { ...row, childDeviceDeliveryClaim: 'delivered' },
      { ...row, appBlockingClaim: 'claimed' },
      { ...row, childDataCustody: 'child-activity-data-included' },
      { ...row, ocentraHostedFamilyDataCustodyClaim: 'claimed' },
      { ...row, claimBoundary: 'runtime proof is externally delivered' },
    ]) {
      expect(AppInstallPurchaseProviderStoreReportStatusRuntimeRowSchema.safeParse(invalidRow).success).toBe(false);
    }

    expect(
      AppInstallPurchaseProviderStoreReportStatusRuntimeProofSchema.safeParse({
        ...proof,
        nonClaims: proof.nonClaims.filter((claim) => claim !== 'no-provider-api-execution'),
      }).success
    ).toBe(false);
  });
}

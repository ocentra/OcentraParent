import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseProviderStoreReportStatusRuntimeProofReadModel,
  AppInstallPurchaseProviderStoreReportStatusRuntimeProofSchema,
  AppInstallPurchaseProviderStoreReportStatusRuntimeRowSchema,
  summarizeAppInstallPurchaseProviderStoreReportStatusRuntimeProof,
} from '../../src/app-install-purchase-provider-store-report-status-runtime-proof';

describe('app install and purchase provider store report status runtime proof', () => {
  acceptsProviderStoreReportStatusRuntimeRowsWithoutDeliveryClaims();
  rejectsMissingProviderStoreReportStatusRuntimeCoverage();
  rejectsProviderStoreReportStatusRuntimeOverclaims();
  rejectsMissingProviderStoreReportStatusRuntimeNonClaims();
});

function acceptsProviderStoreReportStatusRuntimeRowsWithoutDeliveryClaims(): void {
  it('accepts provider store report status runtime rows without execution or delivery claims', () => {
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
    for (const row of proof.providerStoreReportStatusRuntimeRows) {
      expect(row.sourceProviderStoreExecutionReadinessRowId).toContain(row.platform);
      expect(row.sourceRuntimeReportWriterDeliveryRowIds).toHaveLength(4);
      expect(row.sourceRuntimeReportWriterReceiptRefs).toHaveLength(4);
      expect(row.reportCompilerOutputRefs.length).toBeGreaterThan(0);
      expect(row.reportReceiptRefs).toHaveLength(4);
      expect(row.requiredProofRefs.length).toBeGreaterThan(0);
      expect(row.providerApiExecutionClaim).toBe('not-executed');
      expect(row.storeIntegrationClaim).toBe('not-claimed');
      expect(row.portalReportUiClaim).toBe('not-claimed');
      expect(row.runtimeReportDeliveryClaim).toBe('not-delivered');
      expect(row.platformAdapterClaim).toBe('not-implemented');
      expect(row.childDeviceDeliveryClaim).toBe('not-delivered');
      expect(row.appBlockingClaim).toBe('not-claimed');
      expect(row.childDataCustody).toBe('no-child-activity-data');
      expect(row.ocentraHostedFamilyDataCustodyClaim).toBe('not-claimed');
    }
  });
}

function rejectsMissingProviderStoreReportStatusRuntimeCoverage(): void {
  it('rejects rows missing provider store or runtime report writer coverage', () => {
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
    expect(
      AppInstallPurchaseProviderStoreReportStatusRuntimeRowSchema.safeParse({
        ...row,
        reportReceiptRefs: [],
      }).success
    ).toBe(false);
  });
}

function rejectsProviderStoreReportStatusRuntimeOverclaims(): void {
  it('rejects provider store report status overclaims', () => {
    const row =
      AppInstallPurchaseProviderStoreReportStatusRuntimeProofReadModel.providerStoreReportStatusRuntimeRows[0];

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
      { ...row, claimBoundary: 'provider store report status executed through Google Play' },
    ]) {
      expect(AppInstallPurchaseProviderStoreReportStatusRuntimeRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingProviderStoreReportStatusRuntimeNonClaims(): void {
  it('rejects missing provider store report status runtime non-claims', () => {
    const proof = AppInstallPurchaseProviderStoreReportStatusRuntimeProofReadModel;

    for (const claim of [
      'no-provider-api-execution',
      'no-store-integration',
      'no-portal-report-ui',
      'no-external-runtime-report-delivery',
      'no-platform-adapter-implementation',
      'no-child-device-delivery',
      'no-app-blocking',
      'no-child-activity-data',
      'no-ocentra-hosted-family-data-custody',
    ] as const) {
      expect(
        AppInstallPurchaseProviderStoreReportStatusRuntimeProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

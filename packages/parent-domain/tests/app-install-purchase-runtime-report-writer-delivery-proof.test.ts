import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseRuntimeReportWriterDeliveryProofReadModel,
  AppInstallPurchaseRuntimeReportWriterDeliveryProofSchema,
  AppInstallPurchaseRuntimeReportWriterDeliveryRowSchema,
  summarizeAppInstallPurchaseRuntimeReportWriterDeliveryProof,
} from '../src/app-install-purchase-runtime-report-writer-delivery-proof';

describe('app install and purchase runtime report writer delivery proof', () => {
  acceptsParentOwnedReportDeliveryRowsAndReceipts();
  rejectsMissingRuntimeWriterReportOrAuditCoverage();
  rejectsProviderStorePlatformChildPortalCustodyReportAndBlockingOverclaims();
  rejectsMissingRuntimeReportWriterDeliveryNonClaims();
});

function acceptsParentOwnedReportDeliveryRowsAndReceipts(): void {
  it('accepts parent-owned report delivery rows linked to runtime writer receipts without external delivery claims', () => {
    const proof = AppInstallPurchaseRuntimeReportWriterDeliveryProofSchema.parse(
      AppInstallPurchaseRuntimeReportWriterDeliveryProofReadModel
    );

    expect(summarizeAppInstallPurchaseRuntimeReportWriterDeliveryProof(proof)).toEqual({
      runtimeReportWriterDeliveryRows: 4,
      reportDeliveryReadyRows: 3,
      reportReceiptRows: 3,
      manualRequiredRows: 1,
      externallyDeliveredRows: 0,
      portalUiRows: 0,
    });
    expect(
      proof.runtimeReportWriterDeliveryRows.map(
        (row) =>
          `${row.sourceDecisionAction}:${row.runtimeReportWriterDeliveryState}:${row.runtimeReportWriterReceiptState}`
      )
    ).toEqual([
      'approve:report-delivery-ready:parent-owned-report-receipt-recorded',
      'deny:report-delivery-ready:parent-owned-report-receipt-recorded',
      'time-box:report-delivery-ready:parent-owned-report-receipt-recorded',
      'review-needed:manual-required:manual-required',
    ]);

    for (const row of proof.runtimeReportWriterDeliveryRows) {
      expect(row.sourceRuntimeWriterExecutionDeliveryRowId).toContain(row.sourceDecisionAction);
      expect(row.sourceReportRuntimeRowIds).toHaveLength(4);
      expect(row.runtimeReportWriterOutputRef).toContain(row.sourceDecisionAction);
      expect(row.runtimeReportWriterReceiptRef).toContain(row.sourceDecisionAction);
      expect(row.reportCompilerOutputRefs).toEqual(['parent-owned-storage-report-output-ref']);
      expect(row.runtimeWriterReceiptRef).toContain(row.sourceDecisionAction);
      expect(row.runtimeWriterAuditEventRefs).toHaveLength(1);
      expect(row.parentActionAuditEventRefs).toHaveLength(1);
      expect(row.reportAuditEventRefs).toHaveLength(4);
      expect(row.providerApiExecutionClaim).toBe('not-executed');
      expect(row.storeIntegrationClaim).toBe('not-claimed');
      expect(row.platformInterceptionClaim).toBe('not-claimed');
      expect(row.platformAdapterClaim).toBe('not-implemented');
      expect(row.childDeviceDeliveryClaim).toBe('not-delivered');
      expect(row.runtimeReportDeliveryClaim).toBe('not-delivered');
      expect(row.portalReportUiClaim).toBe('not-claimed');
      expect(row.appBlockingClaim).toBe('not-claimed');
      expect(row.childDataCustody).toBe('no-child-activity-data');
      expect(row.ocentraHostedFamilyDataCustodyClaim).toBe('not-claimed');
      expect(row.claimBoundary).toContain('parent-owned report delivery rows');
      expect(row.claimBoundary).toContain('runtime writer receipts');
      expect(row.claimBoundary).toContain('no external runtime report delivery');
    }
  });
}

function rejectsMissingRuntimeWriterReportOrAuditCoverage(): void {
  it('rejects rows that omit source writer rows, report rows, output refs, receipt refs, or audit refs', () => {
    const proof = AppInstallPurchaseRuntimeReportWriterDeliveryProofReadModel;
    const row = proof.runtimeReportWriterDeliveryRows[0];

    expect(
      AppInstallPurchaseRuntimeReportWriterDeliveryProofSchema.safeParse({
        ...proof,
        runtimeReportWriterDeliveryRows: proof.runtimeReportWriterDeliveryRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseRuntimeReportWriterDeliveryRowSchema.safeParse({
        ...row,
        sourceReportRuntimeRowIds: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseRuntimeReportWriterDeliveryRowSchema.safeParse({
        ...row,
        runtimeReportWriterOutputRef: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseRuntimeReportWriterDeliveryRowSchema.safeParse({
        ...row,
        runtimeReportWriterReceiptRef: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseRuntimeReportWriterDeliveryRowSchema.safeParse({
        ...row,
        runtimeWriterAuditEventRefs: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseRuntimeReportWriterDeliveryRowSchema.safeParse({ ...row, reportAuditEventRefs: [] }).success
    ).toBe(false);
  });
}

function rejectsProviderStorePlatformChildPortalCustodyReportAndBlockingOverclaims(): void {
  it('rejects rows that claim provider store platform child portal report custody or blocking behavior', () => {
    const row = AppInstallPurchaseRuntimeReportWriterDeliveryProofReadModel.runtimeReportWriterDeliveryRows[0];

    for (const invalidRow of [
      { ...row, providerApiExecutionClaim: 'executed' },
      { ...row, storeIntegrationClaim: 'claimed' },
      { ...row, platformInterceptionClaim: 'claimed' },
      { ...row, platformAdapterClaim: 'implemented' },
      { ...row, childDeviceDeliveryClaim: 'delivered' },
      { ...row, runtimeReportDeliveryClaim: 'delivered' },
      { ...row, portalReportUiClaim: 'claimed' },
      { ...row, appBlockingClaim: 'claimed' },
      { ...row, childDataCustody: 'child-activity-data-included' },
      { ...row, ocentraHostedFamilyDataCustodyClaim: 'claimed' },
      { ...row, claimBoundary: 'runtime report delivered to portal and child device' },
    ]) {
      expect(AppInstallPurchaseRuntimeReportWriterDeliveryRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingRuntimeReportWriterDeliveryNonClaims(): void {
  it('rejects runtime report writer delivery proof when external delivery non-claims are removed', () => {
    const proof = AppInstallPurchaseRuntimeReportWriterDeliveryProofReadModel;

    for (const claim of [
      'no-portal-report-ui',
      'no-external-runtime-report-delivery',
      'no-provider-api-execution',
      'no-store-integration',
      'no-platform-interception',
      'no-platform-adapter-implementation',
      'no-child-device-delivery',
      'no-real-install-or-purchase-interception',
      'no-app-blocking',
      'no-child-activity-data',
      'no-ocentra-hosted-family-data-custody',
    ] as const) {
      expect(
        AppInstallPurchaseRuntimeReportWriterDeliveryProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

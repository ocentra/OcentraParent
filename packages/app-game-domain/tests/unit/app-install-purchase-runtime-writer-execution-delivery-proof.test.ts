import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseRuntimeWriterExecutionDeliveryProofReadModel,
  AppInstallPurchaseRuntimeWriterExecutionDeliveryProofSchema,
  AppInstallPurchaseRuntimeWriterExecutionDeliveryRowSchema,
  summarizeAppInstallPurchaseRuntimeWriterExecutionDeliveryProof,
} from '../../src/app-install-purchase-runtime-writer-execution-delivery-proof';

describe('app install and purchase runtime writer execution delivery proof', () => {
  acceptsParentOwnedRuntimeWriterEnvelopesAndReceipts();
  rejectsMissingEnvelopeReceiptAuditOrReportCoverage();
  rejectsProviderStorePlatformChildCustodyReportAndBlockingOverclaims();
  rejectsMissingRuntimeWriterExecutionDeliveryNonClaims();
});

function acceptsParentOwnedRuntimeWriterEnvelopesAndReceipts(): void {
  it('accepts deterministic parent-owned writer envelopes and result receipts without external delivery claims', () => {
    const proof = AppInstallPurchaseRuntimeWriterExecutionDeliveryProofSchema.parse(
      AppInstallPurchaseRuntimeWriterExecutionDeliveryProofReadModel
    );

    expect(summarizeAppInstallPurchaseRuntimeWriterExecutionDeliveryProof(proof)).toEqual({
      runtimeWriterExecutionDeliveryRows: 4,
      parentOwnedEnvelopeRows: 3,
      deliveryResultReceiptRows: 3,
      manualRequiredRows: 1,
      providerExecutedRows: 0,
      childDeliveredRows: 0,
    });
    expect(
      proof.runtimeWriterExecutionDeliveryRows.map(
        (row) =>
          `${row.sourceDecisionAction}:${row.runtimeWriterEnvelopeState}:${row.runtimeWriterExecutionDeliveryState}`
      )
    ).toEqual([
      'approve:parent-owned-envelope-written:delivery-result-recorded',
      'deny:parent-owned-envelope-written:delivery-result-recorded',
      'time-box:parent-owned-envelope-written:delivery-result-recorded',
      'review-needed:manual-required:manual-required',
    ]);

    for (const row of proof.runtimeWriterExecutionDeliveryRows) {
      expect(row.sourceRuntimeWriterDeliveryRowId).toContain(row.sourceDecisionAction);
      expect(row.sourceParentActionDeliveryReadinessRowId).toContain(row.sourceDecisionAction);
      expect(row.runtimeWriterEnvelopeRef).toContain(row.sourceDecisionAction);
      expect(row.deliveryResultReceiptRef).toContain(row.sourceDecisionAction);
      expect(row.deliveryResultAuditEventRefs).toHaveLength(1);
      expect(row.parentActionAuditEventRefs).toHaveLength(1);
      expect(row.reportRuntimeRefs.length).toBeGreaterThan(0);
      expect(row.providerApiExecutionClaim).toBe('not-executed');
      expect(row.storeIntegrationClaim).toBe('not-claimed');
      expect(row.platformInterceptionClaim).toBe('not-claimed');
      expect(row.platformAdapterClaim).toBe('not-implemented');
      expect(row.childDeviceDeliveryClaim).toBe('not-delivered');
      expect(row.runtimeReportDeliveryClaim).toBe('not-delivered');
      expect(row.appBlockingClaim).toBe('not-claimed');
      expect(row.childDataCustody).toBe('no-child-activity-data');
      expect(row.ocentraHostedFamilyDataCustodyClaim).toBe('not-claimed');
      expect(row.claimBoundary).toContain('parent-owned runtime writer envelope');
      expect(row.claimBoundary).toContain('delivery result receipt');
      expect(row.claimBoundary).toContain('no child-device delivery');
    }
  });
}

function rejectsMissingEnvelopeReceiptAuditOrReportCoverage(): void {
  it('rejects rows that omit source rows, envelope refs, receipt refs, audit refs, or report refs', () => {
    const proof = AppInstallPurchaseRuntimeWriterExecutionDeliveryProofReadModel;
    const row = proof.runtimeWriterExecutionDeliveryRows[0];

    expect(
      AppInstallPurchaseRuntimeWriterExecutionDeliveryProofSchema.safeParse({
        ...proof,
        runtimeWriterExecutionDeliveryRows: proof.runtimeWriterExecutionDeliveryRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseRuntimeWriterExecutionDeliveryRowSchema.safeParse({
        ...row,
        runtimeWriterEnvelopeRef: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseRuntimeWriterExecutionDeliveryRowSchema.safeParse({
        ...row,
        deliveryResultReceiptRef: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseRuntimeWriterExecutionDeliveryRowSchema.safeParse({
        ...row,
        deliveryResultAuditEventRefs: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseRuntimeWriterExecutionDeliveryRowSchema.safeParse({
        ...row,
        parentActionAuditEventRefs: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseRuntimeWriterExecutionDeliveryRowSchema.safeParse({ ...row, reportRuntimeRefs: [] }).success
    ).toBe(false);
  });
}

function rejectsProviderStorePlatformChildCustodyReportAndBlockingOverclaims(): void {
  it('rejects rows that claim provider store platform child delivery custody report delivery or blocking behavior', () => {
    const row = AppInstallPurchaseRuntimeWriterExecutionDeliveryProofReadModel.runtimeWriterExecutionDeliveryRows[0];

    for (const invalidRow of [
      { ...row, providerApiExecutionClaim: 'executed' },
      { ...row, storeIntegrationClaim: 'claimed' },
      { ...row, platformInterceptionClaim: 'claimed' },
      { ...row, platformAdapterClaim: 'implemented' },
      { ...row, childDeviceDeliveryClaim: 'delivered' },
      { ...row, runtimeReportDeliveryClaim: 'delivered' },
      { ...row, appBlockingClaim: 'claimed' },
      { ...row, childDataCustody: 'child-activity-data-included' },
      { ...row, ocentraHostedFamilyDataCustodyClaim: 'claimed' },
      { ...row, claimBoundary: 'runtime writer delivered the action to the child device' },
    ]) {
      expect(AppInstallPurchaseRuntimeWriterExecutionDeliveryRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingRuntimeWriterExecutionDeliveryNonClaims(): void {
  it('rejects runtime writer execution delivery proof when external delivery non-claims are removed', () => {
    const proof = AppInstallPurchaseRuntimeWriterExecutionDeliveryProofReadModel;

    for (const claim of [
      'no-provider-api-execution',
      'no-store-integration',
      'no-platform-interception',
      'no-platform-adapter-implementation',
      'no-child-device-delivery',
      'no-runtime-report-delivery',
      'no-real-install-or-purchase-interception',
      'no-app-blocking',
      'no-child-activity-data',
      'no-ocentra-hosted-family-data-custody',
    ] as const) {
      expect(
        AppInstallPurchaseRuntimeWriterExecutionDeliveryProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

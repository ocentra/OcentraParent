import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseRuntimeDeliveryReceiptBoundaryProofReadModel,
  AppInstallPurchaseRuntimeDeliveryReceiptBoundaryProofSchema,
  AppInstallPurchaseRuntimeDeliveryReceiptBoundaryRowSchema,
  summarizeAppInstallPurchaseRuntimeDeliveryReceiptBoundaryProof,
} from '../src/app-install-purchase-runtime-delivery-receipt-boundary-proof';

describe('app install and purchase runtime delivery receipt boundary proof', () => {
  acceptsReceiptBoundaryRowsWithoutChildDelivery();
  rejectsMissingReceiptBoundaryRefsOrBlockers();
  rejectsReceiptProviderPlatformChildReportCustodyAndBlockingOverclaims();
  rejectsMissingReceiptBoundaryNonClaims();
});

function acceptsReceiptBoundaryRowsWithoutChildDelivery(): void {
  it('accepts deterministic receipt boundary rows without delivery claims', () => {
    const proof = AppInstallPurchaseRuntimeDeliveryReceiptBoundaryProofSchema.parse(
      AppInstallPurchaseRuntimeDeliveryReceiptBoundaryProofReadModel
    );

    expect(summarizeAppInstallPurchaseRuntimeDeliveryReceiptBoundaryProof(proof)).toEqual({
      runtimeDeliveryReceiptBoundaryRows: 4,
      blockedReceiptRows: 3,
      manualRequiredRows: 1,
      receiptMissingRows: 3,
      readyReceiptRows: 0,
      childDeviceDeliveredRows: 0,
    });
    expect(
      proof.runtimeDeliveryReceiptBoundaryRows.map(
        (row) =>
          `${row.sourceDecisionAction}:${row.runtimeDeliveryReceiptBoundaryState}:${row.childDeviceTransportReceiptState}`
      )
    ).toEqual([
      'approve:receipt-blocked-waiting-runtime-artifacts:receipt-missing',
      'deny:receipt-blocked-waiting-runtime-artifacts:receipt-missing',
      'time-box:receipt-blocked-waiting-runtime-artifacts:receipt-missing',
      'review-needed:manual-required:manual-required',
    ]);

    for (const row of proof.runtimeDeliveryReceiptBoundaryRows) {
      expect(row.sourceDispatchPreflightRowId).toContain(row.sourceDecisionAction);
      expect(row.sourceParentOwnedDispatchPacketRef).toContain(row.sourceDecisionAction);
      expect(row.parentOwnedReceiptBoundaryRef).toContain(row.sourceDecisionAction);
      expect(row.requiredReceiptArtifactBlockers).toEqual([
        'external-writer-dispatch-execution-missing',
        'provider-store-execution-receipt-missing',
        'platform-adapter-execution-receipt-missing',
        'child-device-transport-receipt-missing',
      ]);
      expect(row.externalWriterDispatchExecutionProofRefs.length).toBeGreaterThan(0);
      expect(row.providerStoreExecutionReceiptProofRefs.length).toBeGreaterThan(0);
      expect(row.platformAdapterExecutionReceiptProofRefs.length).toBeGreaterThan(0);
      expect(row.childDeviceTransportReceiptProofRefs.length).toBeGreaterThan(0);
      expect(row.externalRuntimeWriterExecutionClaim).toBe('not-executed');
      expect(row.externalRuntimeWriterDeliveryClaim).toBe('not-delivered');
      expect(row.childDeviceDeliveryClaim).toBe('not-delivered');
      expect(row.claimBoundary).toContain('receipt rows consume parent-owned withheld dispatch packets');
      expect(row.claimBoundary).toContain('before any delivery receipt claim');
      expect(row.claimBoundary).toContain('no child-device delivery');
    }
  });
}

function rejectsMissingReceiptBoundaryRefsOrBlockers(): void {
  it('rejects receipt boundary rows that omit source refs, receipt refs, or blockers', () => {
    const proof = AppInstallPurchaseRuntimeDeliveryReceiptBoundaryProofReadModel;
    const row = proof.runtimeDeliveryReceiptBoundaryRows[0];

    expect(
      AppInstallPurchaseRuntimeDeliveryReceiptBoundaryProofSchema.safeParse({
        ...proof,
        runtimeDeliveryReceiptBoundaryRows: proof.runtimeDeliveryReceiptBoundaryRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseRuntimeDeliveryReceiptBoundaryRowSchema.safeParse({
        ...row,
        sourceDispatchPreflightRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseRuntimeDeliveryReceiptBoundaryRowSchema.safeParse({
        ...row,
        parentOwnedReceiptBoundaryRef: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseRuntimeDeliveryReceiptBoundaryRowSchema.safeParse({
        ...row,
        requiredReceiptArtifactBlockers: row.requiredReceiptArtifactBlockers.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseRuntimeDeliveryReceiptBoundaryRowSchema.safeParse({
        ...row,
        receiptBlockedReasonRefs: [],
      }).success
    ).toBe(false);
  });
}

function rejectsReceiptProviderPlatformChildReportCustodyAndBlockingOverclaims(): void {
  it('rejects receipt boundary rows that claim delivery or execution exists', () => {
    const row = AppInstallPurchaseRuntimeDeliveryReceiptBoundaryProofReadModel.runtimeDeliveryReceiptBoundaryRows[0];

    for (const invalidRow of [
      { ...row, runtimeDeliveryReceiptBoundaryState: 'receipt-ready' },
      { ...row, childDeviceTransportReceiptState: 'receipt-attached' },
      { ...row, runtimeDeliveryReceiptReadinessState: 'ready' },
      { ...row, externalRuntimeWriterExecutionClaim: 'executed' },
      { ...row, externalRuntimeWriterDeliveryClaim: 'delivered' },
      { ...row, parentActionRuntimeDeliveryClaim: 'delivered' },
      { ...row, providerApiExecutionClaim: 'executed' },
      { ...row, storeIntegrationClaim: 'claimed' },
      { ...row, platformInterceptionClaim: 'claimed' },
      { ...row, platformAdapterClaim: 'implemented' },
      { ...row, childDeviceDeliveryClaim: 'delivered' },
      { ...row, runtimeReportDeliveryClaim: 'delivered' },
      { ...row, appBlockingClaim: 'claimed' },
      { ...row, childDataCustody: 'child-activity-data-included' },
      { ...row, ocentraHostedFamilyDataCustodyClaim: 'claimed' },
      { ...row, claimBoundary: 'runtime delivery receipt is ready' },
    ]) {
      expect(AppInstallPurchaseRuntimeDeliveryReceiptBoundaryRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingReceiptBoundaryNonClaims(): void {
  it('rejects receipt boundary proof when required non-claims are removed', () => {
    const proof = AppInstallPurchaseRuntimeDeliveryReceiptBoundaryProofReadModel;

    for (const claim of [
      'no-external-runtime-writer-execution',
      'no-external-runtime-writer-delivery',
      'no-parent-action-runtime-delivery',
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
        AppInstallPurchaseRuntimeDeliveryReceiptBoundaryProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

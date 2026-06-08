import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseRuntimeTransportDeliveryExecutionProofReadModel,
  AppInstallPurchaseRuntimeTransportDeliveryExecutionProofSchema,
  AppInstallPurchaseRuntimeTransportDeliveryExecutionRowSchema,
  summarizeAppInstallPurchaseRuntimeTransportDeliveryExecutionProof,
} from '../src/app-install-purchase-runtime-transport-delivery-execution-proof';

describe('app install and purchase runtime transport delivery execution proof', () => {
  acceptsTransportExecutionRowsWithoutDeliveryClaims();
  rejectsMissingExecutionRefsOrBlockers();
  rejectsExecutionProviderPlatformChildReportCustodyAndBlockingOverclaims();
  rejectsMissingExecutionNonClaims();
});

function acceptsTransportExecutionRowsWithoutDeliveryClaims(): void {
  it('accepts deterministic transport execution rows while execution is withheld', () => {
    const proof = AppInstallPurchaseRuntimeTransportDeliveryExecutionProofSchema.parse(
      AppInstallPurchaseRuntimeTransportDeliveryExecutionProofReadModel
    );

    expect(summarizeAppInstallPurchaseRuntimeTransportDeliveryExecutionProof(proof)).toEqual({
      runtimeTransportDeliveryExecutionRows: 4,
      withheldExecutionRows: 3,
      manualRequiredRows: 1,
      transportAttemptsStartedRows: 0,
      deliveryResultRecordedRows: 0,
      childDeviceReceiptHandoffReadyRows: 0,
      externalRuntimeWriterDeliveredRows: 0,
    });
    expect(
      proof.runtimeTransportDeliveryExecutionRows.map(
        (row) =>
          `${row.sourceDecisionAction}:${row.runtimeTransportExecutionState}:${row.childDeviceReceiptHandoffState}`
      )
    ).toEqual([
      'approve:execution-withheld-missing-artifacts:receipt-handoff-missing',
      'deny:execution-withheld-missing-artifacts:receipt-handoff-missing',
      'time-box:execution-withheld-missing-artifacts:receipt-handoff-missing',
      'review-needed:manual-required:manual-required',
    ]);

    for (const row of proof.runtimeTransportDeliveryExecutionRows) {
      expect(row.sourceReceiptBoundaryRowId).toContain(row.sourceDecisionAction);
      expect(row.parentOwnedTransportExecutionAttemptRef).toContain(row.sourceDecisionAction);
      expect(row.parentOwnedDeliveryResultReceiptRef).toContain(row.sourceDecisionAction);
      expect(row.childDeviceReceiptHandoffRef).toContain(row.sourceDecisionAction);
      expect(row.requiredRuntimeExecutionBlockers).toEqual([
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
      expect(row.claimBoundary).toContain('rows consume runtime delivery receipt boundary rows');
      expect(row.claimBoundary).toContain('parent-owned transport execution attempts withheld');
      expect(row.claimBoundary).toContain('no child-device delivery');
    }
  });
}

function rejectsMissingExecutionRefsOrBlockers(): void {
  it('rejects transport execution rows that omit refs or required blockers', () => {
    const proof = AppInstallPurchaseRuntimeTransportDeliveryExecutionProofReadModel;
    const row = proof.runtimeTransportDeliveryExecutionRows[0];

    expect(
      AppInstallPurchaseRuntimeTransportDeliveryExecutionProofSchema.safeParse({
        ...proof,
        runtimeTransportDeliveryExecutionRows: proof.runtimeTransportDeliveryExecutionRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseRuntimeTransportDeliveryExecutionRowSchema.safeParse({
        ...row,
        sourceReceiptBoundaryRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseRuntimeTransportDeliveryExecutionRowSchema.safeParse({
        ...row,
        parentOwnedTransportExecutionAttemptRef: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseRuntimeTransportDeliveryExecutionRowSchema.safeParse({
        ...row,
        childDeviceReceiptHandoffRef: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseRuntimeTransportDeliveryExecutionRowSchema.safeParse({
        ...row,
        requiredRuntimeExecutionBlockers: row.requiredRuntimeExecutionBlockers.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseRuntimeTransportDeliveryExecutionRowSchema.safeParse({
        ...row,
        executionWithheldReasonRefs: [],
      }).success
    ).toBe(false);
  });
}

function rejectsExecutionProviderPlatformChildReportCustodyAndBlockingOverclaims(): void {
  it('rejects transport execution rows that claim execution, delivery, provider, platform, or custody behavior', () => {
    const row =
      AppInstallPurchaseRuntimeTransportDeliveryExecutionProofReadModel.runtimeTransportDeliveryExecutionRows[0];

    for (const invalidRow of [
      { ...row, runtimeTransportExecutionState: 'execution-ready' },
      { ...row, runtimeTransportAttemptState: 'started' },
      { ...row, runtimeDeliveryResultState: 'result-recorded' },
      { ...row, childDeviceReceiptHandoffState: 'receipt-handoff-ready' },
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
      { ...row, claimBoundary: 'runtime transport delivery execution is complete' },
    ]) {
      expect(AppInstallPurchaseRuntimeTransportDeliveryExecutionRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingExecutionNonClaims(): void {
  it('rejects runtime transport delivery execution proof when required non-claims are removed', () => {
    const proof = AppInstallPurchaseRuntimeTransportDeliveryExecutionProofReadModel;

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
        AppInstallPurchaseRuntimeTransportDeliveryExecutionProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

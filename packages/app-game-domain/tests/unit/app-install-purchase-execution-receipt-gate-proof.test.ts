import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseExecutionReceiptGateProofReadModel,
  AppInstallPurchaseExecutionReceiptGateProofSchema,
  AppInstallPurchaseExecutionReceiptGateRowSchema,
  summarizeAppInstallPurchaseExecutionReceiptGateProof,
} from '../../src/app-install-purchase-execution-receipt-gate-proof';

describe('app install and purchase execution receipt gate proof', () => {
  acceptsExecutionReceiptGateRowsWithoutDeliveryClaims();
  rejectsMissingReceiptGateRefsOrFamilies();
  rejectsWriterProviderPlatformChildReportCustodyAndBlockingOverclaims();
  rejectsMissingExecutionReceiptGateNonClaims();
});

function acceptsExecutionReceiptGateRowsWithoutDeliveryClaims(): void {
  it('accepts receipt gate rows while every execution receipt family is missing or manual', () => {
    const proof = AppInstallPurchaseExecutionReceiptGateProofSchema.parse(
      AppInstallPurchaseExecutionReceiptGateProofReadModel
    );

    expect(summarizeAppInstallPurchaseExecutionReceiptGateProof(proof)).toEqual({
      executionReceiptGateRows: 4,
      blockedReceiptGateRows: 3,
      manualRequiredRows: 1,
      acceptedExecutionReceiptFamilies: 0,
      childDeviceDeliveredRows: 0,
    });
    expect(
      proof.executionReceiptGateRows.map(
        (row) => `${row.sourceDecisionAction}:${row.executionReceiptGateState}:${row.childDeviceTransportReceiptState}`
      )
    ).toEqual([
      'approve:blocked-missing-execution-receipts:receipt-missing',
      'deny:blocked-missing-execution-receipts:receipt-missing',
      'time-box:blocked-missing-execution-receipts:receipt-missing',
      'review-needed:manual-required:manual-required',
    ]);

    for (const row of proof.executionReceiptGateRows) {
      expect(row.sourceWriterTransportExecutionRowId).toContain(row.sourceDecisionAction);
      expect(row.sourceExternalWriterTransportPacketRef).toContain(row.sourceDecisionAction);
      expect(row.sourceExternalWriterTransportExecutionStatusRef).toContain(row.sourceDecisionAction);
      expect(row.sourceExternalWriterTransportAckRef).toContain(row.sourceDecisionAction);
      expect(row.requiredExecutionReceiptFamilies).toEqual([
        'external-writer-dispatch-executor-receipt',
        'provider-store-execution-receipt',
        'platform-adapter-execution-receipt',
        'child-device-transport-receipt',
      ]);
      expect(row.externalWriterDispatchExecutorReceiptProofRefs.length).toBeGreaterThan(0);
      expect(row.providerStoreExecutionReceiptProofRefs.length).toBeGreaterThan(0);
      expect(row.platformAdapterExecutionReceiptProofRefs.length).toBeGreaterThan(0);
      expect(row.childDeviceTransportReceiptProofRefs.length).toBeGreaterThan(0);
      expect(row.externalRuntimeWriterExecutionClaim).toBe('not-executed');
      expect(row.externalRuntimeWriterDeliveryClaim).toBe('not-delivered');
      expect(row.providerApiExecutionClaim).toBe('not-executed');
      expect(row.childDeviceDeliveryClaim).toBe('not-delivered');
      expect(row.claimBoundary).toContain('rows consume external runtime writer transport execution rows');
      expect(row.claimBoundary).toContain('external writer dispatch executor receipt');
      expect(row.claimBoundary).toContain('no child-device delivery');
    }
  });
}

function rejectsMissingReceiptGateRefsOrFamilies(): void {
  it('rejects receipt gate rows that omit source refs, receipt families, or blocker refs', () => {
    const proof = AppInstallPurchaseExecutionReceiptGateProofReadModel;
    const row = proof.executionReceiptGateRows[0];

    expect(
      AppInstallPurchaseExecutionReceiptGateProofSchema.safeParse({
        ...proof,
        executionReceiptGateRows: proof.executionReceiptGateRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseExecutionReceiptGateRowSchema.safeParse({
        ...row,
        sourceWriterTransportExecutionRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseExecutionReceiptGateRowSchema.safeParse({
        ...row,
        sourceExternalWriterTransportAckRef: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseExecutionReceiptGateRowSchema.safeParse({
        ...row,
        requiredExecutionReceiptFamilies: row.requiredExecutionReceiptFamilies.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseExecutionReceiptGateRowSchema.safeParse({
        ...row,
        executionReceiptGateBlockedReasonRefs: [],
      }).success
    ).toBe(false);
  });
}

function rejectsWriterProviderPlatformChildReportCustodyAndBlockingOverclaims(): void {
  it('rejects rows that claim receipt acceptance, execution, delivery, provider, platform, report, custody, or blocking behavior', () => {
    const row = AppInstallPurchaseExecutionReceiptGateProofReadModel.executionReceiptGateRows[0];

    for (const invalidRow of [
      { ...row, executionReceiptGateState: 'receipts-accepted' },
      { ...row, externalWriterDispatchExecutorReceiptState: 'receipt-accepted' },
      { ...row, providerStoreExecutionReceiptState: 'receipt-accepted' },
      { ...row, platformAdapterExecutionReceiptState: 'receipt-accepted' },
      { ...row, childDeviceTransportReceiptState: 'receipt-accepted' },
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
      { ...row, claimBoundary: 'execution receipt gate accepted every receipt' },
    ]) {
      expect(AppInstallPurchaseExecutionReceiptGateRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingExecutionReceiptGateNonClaims(): void {
  it('rejects proof when any required execution receipt gate non-claim is removed', () => {
    const proof = AppInstallPurchaseExecutionReceiptGateProofReadModel;

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
        AppInstallPurchaseExecutionReceiptGateProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

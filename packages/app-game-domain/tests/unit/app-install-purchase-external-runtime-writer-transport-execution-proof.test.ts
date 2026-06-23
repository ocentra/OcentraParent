import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseExternalRuntimeWriterTransportExecutionProofReadModel,
  AppInstallPurchaseExternalRuntimeWriterTransportExecutionProofSchema,
  AppInstallPurchaseExternalRuntimeWriterTransportExecutionRowSchema,
  summarizeAppInstallPurchaseExternalRuntimeWriterTransportExecutionProof,
} from '@ocentra-parent/schema-domain/app-install-purchase-external-runtime-writer-transport-execution-proof';

describe('app install and purchase external runtime writer transport execution proof', () => {
  acceptsExternalWriterTransportExecutionRowsWithoutExecutionClaims();
  rejectsMissingTransportExecutionRefsOrBlockers();
  rejectsWriterProviderPlatformChildReportCustodyAndBlockingOverclaims();
  rejectsMissingExternalWriterTransportExecutionNonClaims();
});

function acceptsExternalWriterTransportExecutionRowsWithoutExecutionClaims(): void {
  it('accepts deterministic external writer transport rows while packets are blocked', () => {
    const proof = AppInstallPurchaseExternalRuntimeWriterTransportExecutionProofSchema.parse(
      AppInstallPurchaseExternalRuntimeWriterTransportExecutionProofReadModel
    );

    expect(summarizeAppInstallPurchaseExternalRuntimeWriterTransportExecutionProof(proof)).toEqual({
      externalRuntimeWriterTransportExecutionRows: 4,
      blockedTransportExecutionRows: 3,
      manualRequiredRows: 1,
      withheldTransportPackets: 3,
      recordedTransportAcks: 0,
      externalRuntimeWriterExecutedRows: 0,
    });
    expect(
      proof.externalRuntimeWriterTransportExecutionRows.map(
        (row) =>
          `${row.sourceDecisionAction}:${row.externalWriterTransportExecutionState}:${row.externalWriterTransportAckState}`
      )
    ).toEqual([
      'approve:transport-execution-blocked:ack-not-recorded',
      'deny:transport-execution-blocked:ack-not-recorded',
      'time-box:transport-execution-blocked:ack-not-recorded',
      'review-needed:manual-required:manual-required',
    ]);

    for (const row of proof.externalRuntimeWriterTransportExecutionRows) {
      expect(row.sourceRuntimeTransportDeliveryExecutionRowId).toContain(row.sourceDecisionAction);
      expect(row.parentOwnedExternalWriterTransportPacketRef).toContain(row.sourceDecisionAction);
      expect(row.parentOwnedExternalWriterTransportExecutionStatusRef).toContain(row.sourceDecisionAction);
      expect(row.parentOwnedExternalWriterTransportAckRef).toContain(row.sourceDecisionAction);
      expect(row.requiredExternalWriterTransportExecutionBlockers).toEqual([
        'external-writer-dispatch-executor-missing',
        'provider-store-execution-receipt-missing',
        'platform-adapter-execution-receipt-missing',
        'child-device-transport-receipt-missing',
      ]);
      expect(row.externalWriterDispatchExecutorProofRefs.length).toBeGreaterThan(0);
      expect(row.providerStoreExecutionReceiptProofRefs.length).toBeGreaterThan(0);
      expect(row.platformAdapterExecutionReceiptProofRefs.length).toBeGreaterThan(0);
      expect(row.childDeviceTransportReceiptProofRefs.length).toBeGreaterThan(0);
      expect(row.externalRuntimeWriterExecutionClaim).toBe('not-executed');
      expect(row.externalRuntimeWriterDeliveryClaim).toBe('not-delivered');
      expect(row.childDeviceDeliveryClaim).toBe('not-delivered');
      expect(row.claimBoundary).toContain('rows consume runtime transport delivery execution rows');
      expect(row.claimBoundary).toContain('parent-owned external writer transport packet execution status refs');
      expect(row.claimBoundary).toContain('no child-device delivery');
    }
  });
}

function rejectsMissingTransportExecutionRefsOrBlockers(): void {
  it('rejects external writer transport execution rows that omit refs or required blockers', () => {
    const proof = AppInstallPurchaseExternalRuntimeWriterTransportExecutionProofReadModel;
    const row = proof.externalRuntimeWriterTransportExecutionRows[0];

    expect(
      AppInstallPurchaseExternalRuntimeWriterTransportExecutionProofSchema.safeParse({
        ...proof,
        externalRuntimeWriterTransportExecutionRows: proof.externalRuntimeWriterTransportExecutionRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseExternalRuntimeWriterTransportExecutionRowSchema.safeParse({
        ...row,
        sourceRuntimeTransportDeliveryExecutionRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseExternalRuntimeWriterTransportExecutionRowSchema.safeParse({
        ...row,
        parentOwnedExternalWriterTransportPacketRef: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseExternalRuntimeWriterTransportExecutionRowSchema.safeParse({
        ...row,
        parentOwnedExternalWriterTransportAckRef: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseExternalRuntimeWriterTransportExecutionRowSchema.safeParse({
        ...row,
        requiredExternalWriterTransportExecutionBlockers: row.requiredExternalWriterTransportExecutionBlockers.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseExternalRuntimeWriterTransportExecutionRowSchema.safeParse({
        ...row,
        transportExecutionBlockedReasonRefs: [],
      }).success
    ).toBe(false);
  });
}

function rejectsWriterProviderPlatformChildReportCustodyAndBlockingOverclaims(): void {
  it('rejects rows that claim writer, provider, platform, child, report, custody, or blocking behavior', () => {
    const row =
      AppInstallPurchaseExternalRuntimeWriterTransportExecutionProofReadModel
        .externalRuntimeWriterTransportExecutionRows[0];

    for (const invalidRow of [
      { ...row, externalWriterTransportExecutionState: 'transport-executed' },
      { ...row, externalWriterTransportPacketState: 'packet-sent' },
      { ...row, externalWriterTransportAckState: 'ack-recorded' },
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
      { ...row, claimBoundary: 'external writer transport execution is complete' },
    ]) {
      expect(AppInstallPurchaseExternalRuntimeWriterTransportExecutionRowSchema.safeParse(invalidRow).success).toBe(
        false
      );
    }
  });
}

function rejectsMissingExternalWriterTransportExecutionNonClaims(): void {
  it('rejects proof when any required external writer transport non-claim is removed', () => {
    const proof = AppInstallPurchaseExternalRuntimeWriterTransportExecutionProofReadModel;

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
        AppInstallPurchaseExternalRuntimeWriterTransportExecutionProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

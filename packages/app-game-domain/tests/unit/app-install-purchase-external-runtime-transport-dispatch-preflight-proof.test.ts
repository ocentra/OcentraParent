import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseExternalRuntimeTransportDispatchPreflightProofReadModel,
  AppInstallPurchaseExternalRuntimeTransportDispatchPreflightProofSchema,
  AppInstallPurchaseExternalRuntimeTransportDispatchPreflightRowSchema,
  summarizeAppInstallPurchaseExternalRuntimeTransportDispatchPreflightProof,
} from '../../src/app-install-purchase-external-runtime-transport-dispatch-preflight-proof';

describe('app install and purchase external runtime transport dispatch preflight proof', () => {
  acceptsWithheldDispatchPreflightRows();
  rejectsMissingDispatchPreflightRefsOrBlockers();
  rejectsDispatchDeliveryProviderPlatformChildReportCustodyAndBlockingOverclaims();
  rejectsMissingDispatchPreflightNonClaims();
});

function acceptsWithheldDispatchPreflightRows(): void {
  it('accepts deterministic parent-owned dispatch packets without runtime delivery', () => {
    const proof = AppInstallPurchaseExternalRuntimeTransportDispatchPreflightProofSchema.parse(
      AppInstallPurchaseExternalRuntimeTransportDispatchPreflightProofReadModel
    );

    expect(summarizeAppInstallPurchaseExternalRuntimeTransportDispatchPreflightProof(proof)).toEqual({
      externalRuntimeTransportDispatchPreflightRows: 4,
      blockedPreflightRows: 3,
      manualRequiredRows: 1,
      withheldDispatchPackets: 3,
      readyDispatchRows: 0,
      externalRuntimeWriterDeliveredRows: 0,
    });
    expect(
      proof.externalRuntimeTransportDispatchPreflightRows.map(
        (row) => `${row.sourceDecisionAction}:${row.dispatchPreflightState}:${row.dispatchPacketState}`
      )
    ).toEqual([
      'approve:blocked-waiting-runtime-artifacts:withheld',
      'deny:blocked-waiting-runtime-artifacts:withheld',
      'time-box:blocked-waiting-runtime-artifacts:withheld',
      'review-needed:manual-required:manual-required',
    ]);

    for (const row of proof.externalRuntimeTransportDispatchPreflightRows) {
      expect(row.sourceExternalRuntimeTransportQueueRowId).toContain(row.sourceDecisionAction);
      expect(row.parentOwnedTransportQueueRef).toContain(row.sourceDecisionAction);
      expect(row.parentOwnedDispatchPacketRef).toContain(row.sourceDecisionAction);
      expect(row.requiredDispatchArtifactBlockers).toEqual([
        'external-writer-transport-handler-missing',
        'provider-store-execution-handler-missing',
        'platform-adapter-execution-handler-missing',
        'child-device-transport-receipt-missing',
      ]);
      expect(row.externalWriterTransportHandlerProofRefs.length).toBeGreaterThan(0);
      expect(row.providerStoreExecutionHandlerProofRefs.length).toBeGreaterThan(0);
      expect(row.platformAdapterExecutionHandlerProofRefs.length).toBeGreaterThan(0);
      expect(row.childDeviceTransportReceiptProofRefs.length).toBeGreaterThan(0);
      expect(row.externalRuntimeWriterExecutionClaim).toBe('not-executed');
      expect(row.externalRuntimeWriterDeliveryClaim).toBe('not-delivered');
      expect(row.childDeviceDeliveryClaim).toBe('not-delivered');
      expect(row.claimBoundary).toContain('must not leave the parent queue');
      expect(row.claimBoundary).toContain('no external runtime writer delivery');
      expect(row.claimBoundary).toContain('no child-device delivery');
    }
  });
}

function rejectsMissingDispatchPreflightRefsOrBlockers(): void {
  it('rejects dispatch preflight rows that omit source refs, packet refs, or blocker refs', () => {
    const proof = AppInstallPurchaseExternalRuntimeTransportDispatchPreflightProofReadModel;
    const row = proof.externalRuntimeTransportDispatchPreflightRows[0];

    expect(
      AppInstallPurchaseExternalRuntimeTransportDispatchPreflightProofSchema.safeParse({
        ...proof,
        externalRuntimeTransportDispatchPreflightRows: proof.externalRuntimeTransportDispatchPreflightRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseExternalRuntimeTransportDispatchPreflightRowSchema.safeParse({
        ...row,
        sourceExternalRuntimeTransportQueueRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseExternalRuntimeTransportDispatchPreflightRowSchema.safeParse({
        ...row,
        parentOwnedDispatchPacketRef: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseExternalRuntimeTransportDispatchPreflightRowSchema.safeParse({
        ...row,
        requiredDispatchArtifactBlockers: row.requiredDispatchArtifactBlockers.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseExternalRuntimeTransportDispatchPreflightRowSchema.safeParse({
        ...row,
        dispatchBlockedReasonRefs: [],
      }).success
    ).toBe(false);
  });
}

function rejectsDispatchDeliveryProviderPlatformChildReportCustodyAndBlockingOverclaims(): void {
  it('rejects dispatch preflight rows that send packets or claim runtime delivery exists', () => {
    const row =
      AppInstallPurchaseExternalRuntimeTransportDispatchPreflightProofReadModel
        .externalRuntimeTransportDispatchPreflightRows[0];

    for (const invalidRow of [
      { ...row, dispatchPreflightState: 'ready' },
      { ...row, dispatchPacketState: 'sent' },
      { ...row, dispatchReadinessState: 'ready' },
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
      { ...row, claimBoundary: 'external runtime transport dispatch is ready' },
    ]) {
      expect(AppInstallPurchaseExternalRuntimeTransportDispatchPreflightRowSchema.safeParse(invalidRow).success).toBe(
        false
      );
    }
  });
}

function rejectsMissingDispatchPreflightNonClaims(): void {
  it('rejects dispatch preflight proof when required non-claims are removed', () => {
    const proof = AppInstallPurchaseExternalRuntimeTransportDispatchPreflightProofReadModel;

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
        AppInstallPurchaseExternalRuntimeTransportDispatchPreflightProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

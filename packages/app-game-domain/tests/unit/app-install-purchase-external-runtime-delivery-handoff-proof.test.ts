import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseExternalRuntimeDeliveryHandoffProofReadModel,
  AppInstallPurchaseExternalRuntimeDeliveryHandoffProofSchema,
  AppInstallPurchaseExternalRuntimeDeliveryHandoffRowSchema,
  summarizeAppInstallPurchaseExternalRuntimeDeliveryHandoffProof,
} from '../../src/app-install-purchase-external-runtime-delivery-handoff-proof';

describe('app install and purchase external runtime delivery handoff proof', () => {
  acceptsParentOwnedExternalRuntimeHandoffPacketsWithoutDeliveryClaims();
  rejectsMissingSourcePacketQueueAuditOrReportCoverage();
  rejectsExternalRuntimeProviderStorePlatformChildReportCustodyAndBlockingOverclaims();
  rejectsMissingExternalRuntimeDeliveryHandoffNonClaims();
});

function acceptsParentOwnedExternalRuntimeHandoffPacketsWithoutDeliveryClaims(): void {
  it('accepts deterministic external runtime handoff rows without external or child delivery claims', () => {
    const proof = AppInstallPurchaseExternalRuntimeDeliveryHandoffProofSchema.parse(
      AppInstallPurchaseExternalRuntimeDeliveryHandoffProofReadModel
    );

    expect(summarizeAppInstallPurchaseExternalRuntimeDeliveryHandoffProof(proof)).toEqual({
      externalRuntimeDeliveryHandoffRows: 4,
      handoffPacketReadyRows: 3,
      manualRequiredRows: 1,
      externalRuntimeWriterDeliveredRows: 0,
      childDeviceDeliveredRows: 0,
    });
    expect(
      proof.externalRuntimeDeliveryHandoffRows.map(
        (row) => `${row.sourceDecisionAction}:${row.externalRuntimeDeliveryHandoffState}`
      )
    ).toEqual([
      'approve:handoff-packet-ready',
      'deny:handoff-packet-ready',
      'time-box:handoff-packet-ready',
      'review-needed:manual-required',
    ]);

    for (const row of proof.externalRuntimeDeliveryHandoffRows) {
      expect(row.sourceExternalRuntimeDeviceDeliveryRowId).toContain(row.sourceDecisionAction);
      expect(row.sourceRuntimeWriterEnvelopeRef).toContain(row.sourceDecisionAction);
      expect(row.sourceDeliveryResultReceiptRef).toContain(row.sourceDecisionAction);
      expect(row.sourceExternalRuntimeWriterTargetRefs.length).toBeGreaterThan(0);
      expect(row.externalRuntimeHandoffPacketRef).toContain(row.sourceDecisionAction);
      expect(row.externalRuntimeWriterQueueRef).toContain(row.sourceDecisionAction);
      expect(row.externalRuntimeWriterDispatchAuditEventRefs.length).toBeGreaterThan(0);
      expect(row.reportRuntimeRefs.length).toBeGreaterThan(0);
      expect(row.externalRuntimeWriterExecutionClaim).toBe('not-executed');
      expect(row.externalRuntimeWriterDeliveryClaim).toBe('not-delivered');
      expect(row.parentActionRuntimeDeliveryClaim).toBe('not-delivered');
      expect(row.providerApiExecutionClaim).toBe('not-executed');
      expect(row.storeIntegrationClaim).toBe('not-claimed');
      expect(row.platformInterceptionClaim).toBe('not-claimed');
      expect(row.platformAdapterClaim).toBe('not-implemented');
      expect(row.childDeviceDeliveryClaim).toBe('not-delivered');
      expect(row.runtimeReportDeliveryClaim).toBe('not-delivered');
      expect(row.appBlockingClaim).toBe('not-claimed');
      expect(row.childDataCustody).toBe('no-child-activity-data');
      expect(row.ocentraHostedFamilyDataCustodyClaim).toBe('not-claimed');
      expect(row.claimBoundary).toContain('parent-owned handoff packet');
      expect(row.claimBoundary).toContain('no external runtime writer delivery');
      expect(row.claimBoundary).toContain('no child-device delivery');
    }
  });
}

function rejectsMissingSourcePacketQueueAuditOrReportCoverage(): void {
  it('rejects rows that omit source rows, packet refs, queue refs, audit refs, or report refs', () => {
    const proof = AppInstallPurchaseExternalRuntimeDeliveryHandoffProofReadModel;
    const row = proof.externalRuntimeDeliveryHandoffRows[0];

    expect(
      AppInstallPurchaseExternalRuntimeDeliveryHandoffProofSchema.safeParse({
        ...proof,
        externalRuntimeDeliveryHandoffRows: proof.externalRuntimeDeliveryHandoffRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseExternalRuntimeDeliveryHandoffRowSchema.safeParse({
        ...row,
        sourceExternalRuntimeDeviceDeliveryRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseExternalRuntimeDeliveryHandoffRowSchema.safeParse({
        ...row,
        externalRuntimeHandoffPacketRef: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseExternalRuntimeDeliveryHandoffRowSchema.safeParse({
        ...row,
        externalRuntimeWriterQueueRef: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseExternalRuntimeDeliveryHandoffRowSchema.safeParse({
        ...row,
        externalRuntimeWriterDispatchAuditEventRefs: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseExternalRuntimeDeliveryHandoffRowSchema.safeParse({ ...row, reportRuntimeRefs: [] }).success
    ).toBe(false);
  });
}

function rejectsExternalRuntimeProviderStorePlatformChildReportCustodyAndBlockingOverclaims(): void {
  it('rejects rows that claim external runtime delivery provider store platform child report custody or blocking behavior', () => {
    const row = AppInstallPurchaseExternalRuntimeDeliveryHandoffProofReadModel.externalRuntimeDeliveryHandoffRows[0];

    for (const invalidRow of [
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
      { ...row, claimBoundary: 'external runtime writer delivered to the child device' },
    ]) {
      expect(AppInstallPurchaseExternalRuntimeDeliveryHandoffRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingExternalRuntimeDeliveryHandoffNonClaims(): void {
  it('rejects external runtime delivery handoff proof when required non-claims are removed', () => {
    const proof = AppInstallPurchaseExternalRuntimeDeliveryHandoffProofReadModel;

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
        AppInstallPurchaseExternalRuntimeDeliveryHandoffProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

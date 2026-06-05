import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseChildDeviceDeliveryRuntimeWriterProofReadModel,
  AppInstallPurchaseChildDeviceDeliveryRuntimeWriterProofSchema,
  AppInstallPurchaseChildDeviceDeliveryRuntimeWriterRowSchema,
  summarizeAppInstallPurchaseChildDeviceDeliveryRuntimeWriterProof,
} from '../src/app-install-purchase-child-device-delivery-runtime-writer-proof';

describe('app install and purchase child-device delivery runtime writer proof', () => {
  acceptsChildDeliveryRuntimeWriterRowsWithoutRuntimeClaims();
  rejectsMissingWriterPackageAuditOrReportCoverage();
  rejectsWriterProviderStoreAdapterDeliveryCustodyInterceptionAndBlockingOverclaims();
  rejectsMissingChildDeliveryRuntimeWriterNonClaims();
});

function acceptsChildDeliveryRuntimeWriterRowsWithoutRuntimeClaims(): void {
  it('accepts child delivery runtime writer rows linked to writer and package-source status refs', () => {
    const proof = AppInstallPurchaseChildDeviceDeliveryRuntimeWriterProofSchema.parse(
      AppInstallPurchaseChildDeviceDeliveryRuntimeWriterProofReadModel
    );

    expect(summarizeAppInstallPurchaseChildDeviceDeliveryRuntimeWriterProof(proof)).toEqual({
      childDeviceDeliveryRuntimeWriterRows: 4,
      childDeliveryEnvelopeReadyRows: 3,
      manualReviewRequiredRows: 1,
      packageSourceCaptureLinkedRows: 4,
      runtimeWriterExecutedRows: 0,
      childDeviceDeliveredRows: 0,
    });
    expect(
      proof.childDeviceDeliveryRuntimeWriterRows.map(
        (row) => `${row.sourceDecisionAction}:${row.sourceRuntimeWriterDeliveryState}:${row.childDeliveryEnvelopeState}`
      )
    ).toEqual([
      'approve:writer-envelope-ready:child-delivery-envelope-ready',
      'deny:writer-envelope-ready:child-delivery-envelope-ready',
      'time-box:writer-envelope-ready:child-delivery-envelope-ready',
      'review-needed:manual-review-required:manual-review-required',
    ]);
    for (const row of proof.childDeviceDeliveryRuntimeWriterRows) {
      expect(row.sourcePackageSourceCaptureRefs).toHaveLength(5);
      expect(row.sourcePackageSourceCaptureStatuses).toEqual([
        'captured',
        'manual-required',
        'unavailable',
        'blocked',
        'blocked',
      ]);
      expect(row.childDeliveryTargetRefs.length).toBeGreaterThanOrEqual(5);
      expect(row.runtimeWriterAuditEventRefs).toHaveLength(1);
      expect(row.packageSourceAuditEventRefs).toHaveLength(5);
      expect(row.reportRuntimeRefs).toHaveLength(5);
      expect(row.runtimeWriterExecutionClaim).toBe('not-executed');
      expect(row.runtimeWriterDeliveryClaim).toBe('not-delivered');
      expect(row.parentActionRuntimeDeliveryClaim).toBe('not-delivered');
      expect(row.providerApiExecutionClaim).toBe('not-executed');
      expect(row.storeIntegrationClaim).toBe('not-claimed');
      expect(row.platformAdapterClaim).toBe('not-implemented');
      expect(row.childDeviceDeliveryClaim).toBe('not-delivered');
      expect(row.runtimeReportDeliveryClaim).toBe('not-delivered');
      expect(row.interceptionClaim).toBe('not-claimed');
      expect(row.appBlockingClaim).toBe('not-claimed');
      expect(row.childDataCustody).toBe('no-child-activity-data');
      expect(row.ocentraHostedFamilyDataCustodyClaim).toBe('not-claimed');
      expect(row.claimBoundary).toContain('no runtime writer execution');
      expect(row.claimBoundary).toContain('no child-device delivery');
    }
  });
}

function rejectsMissingWriterPackageAuditOrReportCoverage(): void {
  it('rejects rows that omit writer package-source audit or report coverage', () => {
    const proof = AppInstallPurchaseChildDeviceDeliveryRuntimeWriterProofReadModel;
    const row = proof.childDeviceDeliveryRuntimeWriterRows[0];

    expect(
      AppInstallPurchaseChildDeviceDeliveryRuntimeWriterProofSchema.safeParse({
        ...proof,
        childDeviceDeliveryRuntimeWriterRows: proof.childDeviceDeliveryRuntimeWriterRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseChildDeviceDeliveryRuntimeWriterRowSchema.safeParse({
        ...row,
        sourcePackageSourceCaptureRefs: row.sourcePackageSourceCaptureRefs.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseChildDeviceDeliveryRuntimeWriterRowSchema.safeParse({
        ...row,
        childDeliveryTargetRefs: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseChildDeviceDeliveryRuntimeWriterRowSchema.safeParse({
        ...row,
        runtimeWriterAuditEventRefs: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseChildDeviceDeliveryRuntimeWriterRowSchema.safeParse({
        ...row,
        packageSourceAuditEventRefs: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseChildDeviceDeliveryRuntimeWriterRowSchema.safeParse({ ...row, reportRuntimeRefs: [] }).success
    ).toBe(false);
  });
}

function rejectsWriterProviderStoreAdapterDeliveryCustodyInterceptionAndBlockingOverclaims(): void {
  it('rejects rows that claim writer execution provider store adapter delivery custody interception or blocking', () => {
    const row =
      AppInstallPurchaseChildDeviceDeliveryRuntimeWriterProofReadModel.childDeviceDeliveryRuntimeWriterRows[0];

    for (const invalidRow of [
      { ...row, childDeliveryEnvelopeState: 'delivered' },
      { ...row, runtimeWriterExecutionClaim: 'executed' },
      { ...row, runtimeWriterDeliveryClaim: 'delivered' },
      { ...row, parentActionRuntimeDeliveryClaim: 'delivered' },
      { ...row, providerApiExecutionClaim: 'executed' },
      { ...row, storeIntegrationClaim: 'claimed' },
      { ...row, platformAdapterClaim: 'implemented' },
      { ...row, childDeviceDeliveryClaim: 'delivered' },
      { ...row, runtimeReportDeliveryClaim: 'delivered' },
      { ...row, interceptionClaim: 'claimed' },
      { ...row, appBlockingClaim: 'claimed' },
      { ...row, childDataCustody: 'child-activity-data-included' },
      { ...row, ocentraHostedFamilyDataCustodyClaim: 'claimed' },
      { ...row, claimBoundary: 'runtime writer delivered the child device approval result' },
    ]) {
      expect(AppInstallPurchaseChildDeviceDeliveryRuntimeWriterRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingChildDeliveryRuntimeWriterNonClaims(): void {
  it('rejects child delivery runtime writer proof when delivery custody or blocking non-claims are removed', () => {
    const proof = AppInstallPurchaseChildDeviceDeliveryRuntimeWriterProofReadModel;

    for (const claim of [
      'no-runtime-writer-execution',
      'no-runtime-writer-delivery',
      'no-parent-action-runtime-delivery',
      'no-provider-api-execution',
      'no-store-integration',
      'no-platform-adapter-implementation',
      'no-child-device-delivery',
      'no-runtime-report-delivery',
      'no-child-activity-data',
      'no-app-blocking',
      'no-ocentra-hosted-family-data-custody',
    ] as const) {
      expect(
        AppInstallPurchaseChildDeviceDeliveryRuntimeWriterProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

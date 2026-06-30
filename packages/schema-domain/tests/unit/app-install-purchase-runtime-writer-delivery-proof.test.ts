import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseRuntimeWriterDeliveryProofReadModel,
  AppInstallPurchaseRuntimeWriterDeliveryProofSchema,
  AppInstallPurchaseRuntimeWriterDeliveryRowSchema,
  summarizeAppInstallPurchaseRuntimeWriterDeliveryProof,
} from '@ocentra-parent/schema-domain/app-install-purchase-runtime-writer-delivery-proof';

describe('app install and purchase runtime writer delivery proof', () => {
  acceptsRuntimeWriterDeliveryRowsWithoutRuntimeClaims();
  rejectsMissingParentActionStoreStatusAuditOrReportCoverage();
  rejectsWriterProviderStoreDeliveryCustodyInterceptionAndBlockingOverclaims();
  rejectsMissingRuntimeWriterDeliveryNonClaims();
});

function acceptsRuntimeWriterDeliveryRowsWithoutRuntimeClaims(): void {
  it('accepts runtime writer delivery rows linked to parent actions and store status handoffs without claims', () => {
    const proof = AppInstallPurchaseRuntimeWriterDeliveryProofSchema.parse(
      AppInstallPurchaseRuntimeWriterDeliveryProofReadModel
    );

    expect(summarizeAppInstallPurchaseRuntimeWriterDeliveryProof(proof)).toEqual({
      runtimeWriterDeliveryRows: 4,
      writerEnvelopeReadyRows: 3,
      manualReviewRequiredRows: 1,
      storeStatusLinkedRows: 4,
      writerImplementedRows: 0,
      runtimeDeliveredRows: 0,
    });
    expect(
      proof.runtimeWriterDeliveryRows.map(
        (row) => `${row.sourceDecisionAction}:${row.sourceRuntimeHandoffStatus}:${row.runtimeWriterDeliveryState}`
      )
    ).toEqual([
      'approve:queued-for-runtime-writer:writer-envelope-ready',
      'deny:queued-for-runtime-writer:writer-envelope-ready',
      'time-box:queued-for-runtime-writer:writer-envelope-ready',
      'review-needed:manual-review-required:manual-review-required',
    ]);
    for (const row of proof.runtimeWriterDeliveryRows) {
      expect(row.sourceStoreStatusHandoffRefs).toHaveLength(5);
      expect(row.sourceStoreStatusHandoffStates).toEqual([
        'approved-api-status-proof-required',
        'manual-platform-status-review-required',
        'platform-store-status-unavailable',
        'store-entitlement-status-proof-required',
        'store-entitlement-status-proof-required',
      ]);
      expect(row.storeStatusHandoffEvidenceRefs.length).toBeGreaterThanOrEqual(10);
      expect(row.auditEventRefs).toHaveLength(1);
      expect(row.reportRuntimeRefs).toHaveLength(5);
      expect(row.runtimeWriterImplementationClaim).toBe('not-implemented');
      expect(row.runtimeWriterDeliveryClaim).toBe('not-delivered');
      expect(row.parentActionRuntimeDeliveryClaim).toBe('not-delivered');
      expect(row.storeStatusHandoffDeliveryClaim).toBe('not-delivered');
      expect(row.providerApiExecutionClaim).toBe('not-executed');
      expect(row.storeIntegrationClaim).toBe('not-claimed');
      expect(row.platformAdapterClaim).toBe('not-implemented');
      expect(row.childDeliveryClaim).toBe('not-delivered');
      expect(row.runtimeReportDeliveryClaim).toBe('not-delivered');
      expect(row.interceptionClaim).toBe('not-claimed');
      expect(row.appBlockingClaim).toBe('not-claimed');
      expect(row.childDataCustody).toBe('no-child-activity-data');
      expect(row.ocentraHostedFamilyDataCustodyClaim).toBe('not-claimed');
      expect(row.claimBoundary).toContain('no runtime writer implementation');
      expect(row.claimBoundary).toContain('no parent action runtime delivery');
    }
  });
}

function rejectsMissingParentActionStoreStatusAuditOrReportCoverage(): void {
  it('rejects rows that omit parent action, store status, audit, evidence, or report coverage', () => {
    const proof = AppInstallPurchaseRuntimeWriterDeliveryProofReadModel;
    const row = proof.runtimeWriterDeliveryRows[0];

    expect(
      AppInstallPurchaseRuntimeWriterDeliveryProofSchema.safeParse({
        ...proof,
        runtimeWriterDeliveryRows: proof.runtimeWriterDeliveryRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseRuntimeWriterDeliveryRowSchema.safeParse({
        ...row,
        sourceStoreStatusHandoffRefs: row.sourceStoreStatusHandoffRefs.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseRuntimeWriterDeliveryRowSchema.safeParse({
        ...row,
        sourceStoreStatusHandoffStates: row.sourceStoreStatusHandoffStates.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseRuntimeWriterDeliveryRowSchema.safeParse({ ...row, storeStatusHandoffEvidenceRefs: [] }).success
    ).toBe(false);
    expect(AppInstallPurchaseRuntimeWriterDeliveryRowSchema.safeParse({ ...row, auditEventRefs: [] }).success).toBe(
      false
    );
    expect(AppInstallPurchaseRuntimeWriterDeliveryRowSchema.safeParse({ ...row, reportRuntimeRefs: [] }).success).toBe(
      false
    );
  });
}

function rejectsWriterProviderStoreDeliveryCustodyInterceptionAndBlockingOverclaims(): void {
  it('rejects rows that claim writer delivery provider store adapter custody interception or blocking behavior', () => {
    const row = AppInstallPurchaseRuntimeWriterDeliveryProofReadModel.runtimeWriterDeliveryRows[0];

    for (const invalidRow of [
      { ...row, runtimeWriterDeliveryState: 'delivered' },
      { ...row, runtimeWriterQueueState: 'implemented' },
      { ...row, runtimeWriterImplementationClaim: 'implemented' },
      { ...row, runtimeWriterDeliveryClaim: 'delivered' },
      { ...row, parentActionRuntimeDeliveryClaim: 'delivered' },
      { ...row, storeStatusHandoffDeliveryClaim: 'delivered' },
      { ...row, providerApiExecutionClaim: 'executed' },
      { ...row, storeIntegrationClaim: 'claimed' },
      { ...row, platformAdapterClaim: 'implemented' },
      { ...row, childDeliveryClaim: 'delivered' },
      { ...row, runtimeReportDeliveryClaim: 'delivered' },
      { ...row, interceptionClaim: 'claimed' },
      { ...row, appBlockingClaim: 'claimed' },
      { ...row, childDataCustody: 'child-activity-data-included' },
      { ...row, ocentraHostedFamilyDataCustodyClaim: 'claimed' },
      { ...row, claimBoundary: 'runtime writer delivered parent action to child device' },
    ]) {
      expect(AppInstallPurchaseRuntimeWriterDeliveryRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingRuntimeWriterDeliveryNonClaims(): void {
  it('rejects runtime writer delivery proof when delivery custody or blocking non-claims are removed', () => {
    const proof = AppInstallPurchaseRuntimeWriterDeliveryProofReadModel;

    for (const claim of [
      'no-runtime-writer-implementation',
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
        AppInstallPurchaseRuntimeWriterDeliveryProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

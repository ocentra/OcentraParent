import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseParentActionDeliveryReadinessProofReadModel,
  AppInstallPurchaseParentActionDeliveryReadinessProofSchema,
  AppInstallPurchaseParentActionDeliveryReadinessRowSchema,
  summarizeAppInstallPurchaseParentActionDeliveryReadinessProof,
} from '../../src/app-install-purchase-parent-action-delivery-readiness-proof';

describe('app install and purchase parent action delivery readiness proof', () => {
  acceptsParentActionDeliveryReadinessRowsWithoutRuntimeClaims();
  rejectsMissingParentActionChildEnvelopeOrReportCoverage();
  rejectsParentActionWriterProviderAdapterDeliveryCustodyInterceptionAndBlockingOverclaims();
  rejectsMissingParentActionDeliveryReadinessNonClaims();
});

function acceptsParentActionDeliveryReadinessRowsWithoutRuntimeClaims(): void {
  it('accepts parent action delivery readiness rows linked to parent handoff and child envelope refs', () => {
    const proof = AppInstallPurchaseParentActionDeliveryReadinessProofSchema.parse(
      AppInstallPurchaseParentActionDeliveryReadinessProofReadModel
    );

    expect(summarizeAppInstallPurchaseParentActionDeliveryReadinessProof(proof)).toEqual({
      parentActionDeliveryReadinessRows: 4,
      parentActionDeliveryReadyRows: 3,
      manualReviewRequiredRows: 1,
      childEnvelopeLinkedRows: 4,
      parentActionDeliveredRows: 0,
      runtimeWriterExecutedRows: 0,
    });
    expect(
      proof.parentActionDeliveryReadinessRows.map(
        (row) =>
          `${row.sourceDecisionAction}:${row.sourceRuntimeHandoffStatus}:${row.sourceChildDeliveryEnvelopeState}:${row.parentActionDeliveryReadinessState}`
      )
    ).toEqual([
      'approve:queued-for-runtime-writer:child-delivery-envelope-ready:parent-action-delivery-ready',
      'deny:queued-for-runtime-writer:child-delivery-envelope-ready:parent-action-delivery-ready',
      'time-box:queued-for-runtime-writer:child-delivery-envelope-ready:parent-action-delivery-ready',
      'review-needed:manual-review-required:manual-review-required:manual-review-required',
    ]);
    for (const row of proof.parentActionDeliveryReadinessRows) {
      expect(row.sourceParentActionRuntimeHandoffRowId).toContain(row.sourceDecisionAction);
      expect(row.sourceChildDeviceDeliveryRuntimeWriterRowId).toContain(row.sourceDecisionAction);
      expect(row.parentActionAuditEventRefs).toHaveLength(1);
      expect(row.childDeliveryTargetRefs.length).toBeGreaterThanOrEqual(5);
      expect(row.reportRuntimeRefs).toHaveLength(5);
      expect(row.parentActionRuntimeDeliveryClaim).toBe('not-delivered');
      expect(row.runtimeWriterExecutionClaim).toBe('not-executed');
      expect(row.runtimeWriterDeliveryClaim).toBe('not-delivered');
      expect(row.providerApiExecutionClaim).toBe('not-executed');
      expect(row.storeIntegrationClaim).toBe('not-claimed');
      expect(row.platformAdapterClaim).toBe('not-implemented');
      expect(row.childDeviceDeliveryClaim).toBe('not-delivered');
      expect(row.runtimeReportDeliveryClaim).toBe('not-delivered');
      expect(row.interceptionClaim).toBe('not-claimed');
      expect(row.appBlockingClaim).toBe('not-claimed');
      expect(row.childDataCustody).toBe('no-child-activity-data');
      expect(row.ocentraHostedFamilyDataCustodyClaim).toBe('not-claimed');
      expect(row.claimBoundary).toContain('no parent action runtime delivery');
      expect(row.claimBoundary).toContain('no runtime writer execution');
    }
  });
}

function rejectsMissingParentActionChildEnvelopeOrReportCoverage(): void {
  it('rejects rows that omit parent handoff child envelope or report coverage', () => {
    const proof = AppInstallPurchaseParentActionDeliveryReadinessProofReadModel;
    const row = proof.parentActionDeliveryReadinessRows[0];

    expect(
      AppInstallPurchaseParentActionDeliveryReadinessProofSchema.safeParse({
        ...proof,
        parentActionDeliveryReadinessRows: proof.parentActionDeliveryReadinessRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseParentActionDeliveryReadinessRowSchema.safeParse({
        ...row,
        sourceParentActionRuntimeHandoffRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseParentActionDeliveryReadinessRowSchema.safeParse({
        ...row,
        sourceChildDeviceDeliveryRuntimeWriterRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseParentActionDeliveryReadinessRowSchema.safeParse({
        ...row,
        parentActionAuditEventRefs: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseParentActionDeliveryReadinessRowSchema.safeParse({
        ...row,
        childDeliveryTargetRefs: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseParentActionDeliveryReadinessRowSchema.safeParse({
        ...row,
        reportRuntimeRefs: [],
      }).success
    ).toBe(false);
  });
}

function rejectsParentActionWriterProviderAdapterDeliveryCustodyInterceptionAndBlockingOverclaims(): void {
  it('rejects rows that claim parent action delivery writer execution provider adapter custody interception or blocking', () => {
    const row = AppInstallPurchaseParentActionDeliveryReadinessProofReadModel.parentActionDeliveryReadinessRows[0];

    for (const invalidRow of [
      { ...row, parentActionDeliveryReadinessState: 'delivered' },
      { ...row, parentActionRuntimeDeliveryClaim: 'delivered' },
      { ...row, runtimeWriterExecutionClaim: 'executed' },
      { ...row, runtimeWriterDeliveryClaim: 'delivered' },
      { ...row, providerApiExecutionClaim: 'executed' },
      { ...row, storeIntegrationClaim: 'claimed' },
      { ...row, platformAdapterClaim: 'implemented' },
      { ...row, childDeviceDeliveryClaim: 'delivered' },
      { ...row, runtimeReportDeliveryClaim: 'delivered' },
      { ...row, interceptionClaim: 'claimed' },
      { ...row, appBlockingClaim: 'claimed' },
      { ...row, childDataCustody: 'child-activity-data-included' },
      { ...row, ocentraHostedFamilyDataCustodyClaim: 'claimed' },
      { ...row, claimBoundary: 'parent action delivery was executed by the runtime writer' },
    ]) {
      expect(AppInstallPurchaseParentActionDeliveryReadinessRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingParentActionDeliveryReadinessNonClaims(): void {
  it('rejects parent action delivery readiness proof when delivery custody or blocking non-claims are removed', () => {
    const proof = AppInstallPurchaseParentActionDeliveryReadinessProofReadModel;

    for (const claim of [
      'no-parent-action-runtime-delivery',
      'no-runtime-writer-execution',
      'no-runtime-writer-delivery',
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
        AppInstallPurchaseParentActionDeliveryReadinessProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

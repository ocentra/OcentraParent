import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseParentActionRuntimeHandoffProofReadModel,
  AppInstallPurchaseParentActionRuntimeHandoffProofSchema,
  AppInstallPurchaseParentActionRuntimeHandoffRowSchema,
  summarizeAppInstallPurchaseParentActionRuntimeHandoffProof,
} from '../../src/app-install-purchase-parent-action-runtime-handoff-proof';

describe('app install and purchase parent action runtime handoff proof', () => {
  acceptsReviewActionRuntimeHandoffRowsWithoutDeliveryClaims();
  rejectsMissingReviewAdapterAuditOrReportCoverage();
  rejectsPortalWriterProviderDeliveryCustodyInterceptionAndBlockingOverclaims();
  rejectsMissingRuntimeHandoffNonClaims();
});

function acceptsReviewActionRuntimeHandoffRowsWithoutDeliveryClaims(): void {
  it('accepts parent action handoff rows linked to review actions and platform boundaries without runtime delivery claims', () => {
    const proof = AppInstallPurchaseParentActionRuntimeHandoffProofSchema.parse(
      AppInstallPurchaseParentActionRuntimeHandoffProofReadModel
    );

    expect(summarizeAppInstallPurchaseParentActionRuntimeHandoffProof(proof)).toEqual({
      runtimeHandoffRows: 4,
      queuedRuntimeWriterRows: 3,
      manualReviewRequiredRows: 1,
      platformBoundaryLinkedRows: 4,
      runtimeDeliveredRows: 0,
      childDeliveredRows: 0,
    });
    expect(proof.runtimeHandoffRows.map((row) => `${row.sourceDecisionAction}:${row.runtimeHandoffStatus}`)).toEqual([
      'approve:queued-for-runtime-writer',
      'deny:queued-for-runtime-writer',
      'time-box:queued-for-runtime-writer',
      'review-needed:manual-review-required',
    ]);
    for (const row of proof.runtimeHandoffRows) {
      expect(row.platformAdapterBoundaryRefs).toHaveLength(5);
      expect(row.auditEventRefs).toHaveLength(1);
      expect(row.reportRuntimeRefs).toHaveLength(4);
      expect(row.parentActionRuntimeDeliveryClaim).toBe('not-delivered');
      expect(row.portalApprovalUiClaim).toBe('not-implemented');
      expect(row.providerApiExecutionClaim).toBe('not-executed');
      expect(row.storeIntegrationClaim).toBe('not-claimed');
      expect(row.platformAdapterClaim).toBe('not-implemented');
      expect(row.childDeliveryClaim).toBe('not-delivered');
      expect(row.runtimeReportDeliveryClaim).toBe('not-delivered');
      expect(row.interceptionClaim).toBe('not-claimed');
      expect(row.appBlockingClaim).toBe('not-claimed');
      expect(row.childDataCustody).toBe('no-child-activity-data');
      expect(row.ocentraHostedFamilyDataCustodyClaim).toBe('not-claimed');
      expect(row.claimBoundary).toContain('no runtime action writer implementation');
      expect(row.claimBoundary).toContain('no parent action runtime delivery');
    }
  });
}

function rejectsMissingReviewAdapterAuditOrReportCoverage(): void {
  it('rejects handoff rows that omit actions, adapter boundaries, audit refs, or report refs', () => {
    const proof = AppInstallPurchaseParentActionRuntimeHandoffProofReadModel;
    const row = proof.runtimeHandoffRows[0];

    expect(
      AppInstallPurchaseParentActionRuntimeHandoffProofSchema.safeParse({
        ...proof,
        runtimeHandoffRows: proof.runtimeHandoffRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseParentActionRuntimeHandoffRowSchema.safeParse({
        ...row,
        platformAdapterBoundaryRefs: row.platformAdapterBoundaryRefs.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseParentActionRuntimeHandoffRowSchema.safeParse({ ...row, auditEventRefs: [] }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseParentActionRuntimeHandoffRowSchema.safeParse({ ...row, reportRuntimeRefs: [] }).success
    ).toBe(false);
  });
}

function rejectsPortalWriterProviderDeliveryCustodyInterceptionAndBlockingOverclaims(): void {
  it('rejects rows that claim portal writer delivery provider store adapter custody interception or blocking behavior', () => {
    const row = AppInstallPurchaseParentActionRuntimeHandoffProofReadModel.runtimeHandoffRows[0];

    for (const invalidRow of [
      { ...row, runtimeActionWriterClaim: 'implemented' },
      { ...row, parentActionRuntimeDeliveryClaim: 'delivered' },
      { ...row, portalApprovalUiClaim: 'implemented' },
      { ...row, providerApiExecutionClaim: 'executed' },
      { ...row, storeIntegrationClaim: 'claimed' },
      { ...row, platformAdapterClaim: 'implemented' },
      { ...row, childDeliveryClaim: 'delivered' },
      { ...row, runtimeReportDeliveryClaim: 'delivered' },
      { ...row, interceptionClaim: 'claimed' },
      { ...row, appBlockingClaim: 'claimed' },
      { ...row, childDataCustody: 'child-activity-data-included' },
      { ...row, ocentraHostedFamilyDataCustodyClaim: 'claimed' },
      { ...row, claimBoundary: 'parent action runtime delivered to child device' },
    ]) {
      expect(AppInstallPurchaseParentActionRuntimeHandoffRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingRuntimeHandoffNonClaims(): void {
  it('rejects parent action runtime handoff proof when runtime delivery or custody non-claims are removed', () => {
    const proof = AppInstallPurchaseParentActionRuntimeHandoffProofReadModel;

    for (const claim of [
      'no-runtime-action-writer-implementation',
      'no-parent-action-runtime-delivery',
      'no-provider-api-execution',
      'no-child-device-delivery',
      'no-runtime-report-delivery',
      'no-child-activity-data',
      'not-generic-app-blocking',
      'no-ocentra-hosted-family-data-custody',
    ] as const) {
      expect(
        AppInstallPurchaseParentActionRuntimeHandoffProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

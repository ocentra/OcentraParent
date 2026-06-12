import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseDispatchExecutorReceiptProofReadModel,
  AppInstallPurchaseDispatchExecutorReceiptProofSchema,
  AppInstallPurchaseDispatchExecutorReceiptRowSchema,
  summarizeAppInstallPurchaseDispatchExecutorReceiptProof,
} from '../../src/app-install-purchase-dispatch-executor-receipt-proof';

describe('app install and purchase dispatch executor receipt proof', () => {
  acceptsDispatchExecutorReceiptRowsWithoutExecutionClaims();
  rejectsMissingDispatchExecutorRefsOrArtifacts();
  rejectsWriterProviderPlatformChildReportCustodyAndBlockingOverclaims();
  rejectsMissingDispatchExecutorReceiptNonClaims();
});

function acceptsDispatchExecutorReceiptRowsWithoutExecutionClaims(): void {
  it('accepts dispatch executor receipt rows while executor artifacts are missing or manual', () => {
    const proof = AppInstallPurchaseDispatchExecutorReceiptProofSchema.parse(
      AppInstallPurchaseDispatchExecutorReceiptProofReadModel
    );

    expect(summarizeAppInstallPurchaseDispatchExecutorReceiptProof(proof)).toEqual({
      dispatchExecutorReceiptRows: 4,
      blockedDispatchExecutorRows: 3,
      manualRequiredRows: 1,
      acceptedDispatchExecutorArtifacts: 0,
      externalRuntimeWriterExecutedRows: 0,
      childDeviceDeliveredRows: 0,
    });
    expect(
      proof.dispatchExecutorReceiptRows.map(
        (row) =>
          `${row.sourceDecisionAction}:${row.dispatchExecutorReceiptState}:${row.dispatchExecutorReceiptArtifactState}`
      )
    ).toEqual([
      'approve:dispatch-executor-receipt-blocked:artifact-missing',
      'deny:dispatch-executor-receipt-blocked:artifact-missing',
      'time-box:dispatch-executor-receipt-blocked:artifact-missing',
      'review-needed:manual-required:manual-required',
    ]);

    for (const row of proof.dispatchExecutorReceiptRows) {
      expect(row.sourceExecutionReceiptGateRowId).toContain(row.sourceDecisionAction);
      expect(row.requiredDispatchExecutorArtifacts).toEqual([
        'external-writer-dispatch-executor-handler-proof',
        'external-writer-dispatch-executor-receipt-artifact',
        'external-writer-dispatch-executor-audit-artifact',
      ]);
      expect(row.dispatchExecutorHandlerProofRefs.length).toBeGreaterThan(0);
      expect(row.dispatchExecutorReceiptArtifactRefs[0]).toContain(row.sourceDecisionAction);
      expect(row.dispatchExecutorAuditArtifactRefs[0]).toContain(row.sourceDecisionAction);
      expect(row.externalRuntimeWriterExecutionClaim).toBe('not-executed');
      expect(row.externalRuntimeWriterDeliveryClaim).toBe('not-delivered');
      expect(row.providerApiExecutionClaim).toBe('not-executed');
      expect(row.platformAdapterClaim).toBe('not-implemented');
      expect(row.childDeviceDeliveryClaim).toBe('not-delivered');
      expect(row.claimBoundary).toContain('rows consume execution receipt gate rows');
      expect(row.claimBoundary).toContain('real external writer dispatch executor receipt');
      expect(row.claimBoundary).toContain('no child-device delivery');
    }
  });
}

function rejectsMissingDispatchExecutorRefsOrArtifacts(): void {
  it('rejects rows that omit source refs, required artifacts, or blocked reason refs', () => {
    const proof = AppInstallPurchaseDispatchExecutorReceiptProofReadModel;
    const row = proof.dispatchExecutorReceiptRows[0];

    expect(
      AppInstallPurchaseDispatchExecutorReceiptProofSchema.safeParse({
        ...proof,
        dispatchExecutorReceiptRows: proof.dispatchExecutorReceiptRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseDispatchExecutorReceiptRowSchema.safeParse({
        ...row,
        sourceExecutionReceiptGateRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseDispatchExecutorReceiptRowSchema.safeParse({
        ...row,
        requiredDispatchExecutorArtifacts: row.requiredDispatchExecutorArtifacts.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseDispatchExecutorReceiptRowSchema.safeParse({
        ...row,
        dispatchExecutorHandlerProofRefs: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseDispatchExecutorReceiptRowSchema.safeParse({
        ...row,
        dispatchExecutorBlockedReasonRefs: [],
      }).success
    ).toBe(false);
  });
}

function rejectsWriterProviderPlatformChildReportCustodyAndBlockingOverclaims(): void {
  it('rejects rows that claim dispatch executor artifacts, execution, delivery, provider, platform, report, custody, or blocking behavior', () => {
    const row = AppInstallPurchaseDispatchExecutorReceiptProofReadModel.dispatchExecutorReceiptRows[0];

    for (const invalidRow of [
      { ...row, dispatchExecutorReceiptState: 'dispatch-executor-receipt-accepted' },
      { ...row, dispatchExecutorReceiptArtifactState: 'artifact-attached' },
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
      { ...row, claimBoundary: 'dispatch executor receipt accepted every artifact' },
    ]) {
      expect(AppInstallPurchaseDispatchExecutorReceiptRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingDispatchExecutorReceiptNonClaims(): void {
  it('rejects proof when any required dispatch executor receipt non-claim is removed', () => {
    const proof = AppInstallPurchaseDispatchExecutorReceiptProofReadModel;

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
        AppInstallPurchaseDispatchExecutorReceiptProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

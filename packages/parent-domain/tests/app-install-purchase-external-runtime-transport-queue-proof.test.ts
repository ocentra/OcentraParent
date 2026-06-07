import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseExternalRuntimeTransportQueueProofReadModel,
  AppInstallPurchaseExternalRuntimeTransportQueueProofSchema,
  AppInstallPurchaseExternalRuntimeTransportQueueRowSchema,
  summarizeAppInstallPurchaseExternalRuntimeTransportQueueProof,
} from '../src/app-install-purchase-external-runtime-transport-queue-proof';

describe('app install and purchase external runtime transport queue proof', () => {
  acceptsQueueRowsThatBlockDispatch();
  rejectsMissingQueueRefsOrRuntimeBlockers();
  rejectsTransportProviderStorePlatformChildReportCustodyAndBlockingOverclaims();
  rejectsMissingTransportQueueNonClaims();
});

function acceptsQueueRowsThatBlockDispatch(): void {
  it('accepts deterministic parent-owned queue rows without dispatching delivery', () => {
    const proof = AppInstallPurchaseExternalRuntimeTransportQueueProofSchema.parse(
      AppInstallPurchaseExternalRuntimeTransportQueueProofReadModel
    );

    expect(summarizeAppInstallPurchaseExternalRuntimeTransportQueueProof(proof)).toEqual({
      externalRuntimeTransportQueueRows: 4,
      queuedBlockedRows: 3,
      manualRequiredRows: 1,
      dispatchBlockedRows: 3,
      retryScheduledRows: 1,
      externalRuntimeWriterDeliveredRows: 0,
    });
    expect(
      proof.externalRuntimeTransportQueueRows.map(
        (row) =>
          `${row.sourceDecisionAction}:${row.externalRuntimeTransportQueueState}:${row.externalRuntimeTransportDispatchState}`
      )
    ).toEqual([
      'approve:queued-blocked:dispatch-blocked',
      'deny:queued-blocked:dispatch-blocked',
      'time-box:queued-blocked:dispatch-blocked',
      'review-needed:manual-required:manual-required',
    ]);

    for (const row of proof.externalRuntimeTransportQueueRows) {
      expect(row.sourceExternalRuntimeWriterDeliveryBlockerRowId).toContain(row.sourceDecisionAction);
      expect(row.sourceDeliveryAttemptState).toBe('not-started');
      expect(row.parentOwnedTransportQueueRef).toContain(row.sourceDecisionAction);
      expect(row.queueGuardAuditEventRefs.length).toBeGreaterThan(0);
      expect(row.requiredRuntimeBlockers).toEqual([
        'external-writer-transport-proof-missing',
        'platform-adapter-proof-missing',
        'provider-store-execution-proof-missing',
        'child-device-transport-proof-missing',
      ]);
      expect(row.requiredExternalWriterTransportProofRefs.length).toBeGreaterThan(0);
      expect(row.requiredChildDeviceTransportProofRefs.length).toBeGreaterThan(0);
      expect(row.requiredProviderStoreProofRefs.length).toBeGreaterThan(0);
      expect(row.requiredPlatformAdapterProofRefs.length).toBeGreaterThan(0);
      expect(row.blockedDispatchReasonRefs).toEqual([
        `missing-external-writer-transport-${row.sourceDecisionAction}`,
        `missing-platform-adapter-execution-${row.sourceDecisionAction}`,
        `missing-provider-store-execution-${row.sourceDecisionAction}`,
        `missing-child-device-transport-${row.sourceDecisionAction}`,
      ]);
      expect(row.externalRuntimeWriterExecutionClaim).toBe('not-executed');
      expect(row.externalRuntimeWriterDeliveryClaim).toBe('not-delivered');
      expect(row.childDeviceDeliveryClaim).toBe('not-delivered');
      expect(row.claimBoundary).toContain('must not dispatch');
      expect(row.claimBoundary).toContain('no external runtime writer delivery');
      expect(row.claimBoundary).toContain('no child-device delivery');
    }
  });
}

function rejectsMissingQueueRefsOrRuntimeBlockers(): void {
  it('rejects queue rows that omit source refs, queue refs, or blocker refs', () => {
    const proof = AppInstallPurchaseExternalRuntimeTransportQueueProofReadModel;
    const row = proof.externalRuntimeTransportQueueRows[0];

    expect(
      AppInstallPurchaseExternalRuntimeTransportQueueProofSchema.safeParse({
        ...proof,
        externalRuntimeTransportQueueRows: proof.externalRuntimeTransportQueueRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseExternalRuntimeTransportQueueRowSchema.safeParse({
        ...row,
        sourceExternalRuntimeWriterDeliveryBlockerRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseExternalRuntimeTransportQueueRowSchema.safeParse({
        ...row,
        parentOwnedTransportQueueRef: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseExternalRuntimeTransportQueueRowSchema.safeParse({
        ...row,
        requiredRuntimeBlockers: row.requiredRuntimeBlockers.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseExternalRuntimeTransportQueueRowSchema.safeParse({
        ...row,
        blockedDispatchReasonRefs: [],
      }).success
    ).toBe(false);
  });
}

function rejectsTransportProviderStorePlatformChildReportCustodyAndBlockingOverclaims(): void {
  it('rejects queue rows that dispatch or claim runtime delivery exists', () => {
    const row = AppInstallPurchaseExternalRuntimeTransportQueueProofReadModel.externalRuntimeTransportQueueRows[0];

    for (const invalidRow of [
      { ...row, sourceDeliveryAttemptState: 'started' },
      { ...row, externalRuntimeTransportDispatchState: 'dispatch-ready' },
      { ...row, externalRuntimeTransportRetryState: 'scheduled' },
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
      { ...row, claimBoundary: 'external runtime transport queue delivered' },
    ]) {
      expect(AppInstallPurchaseExternalRuntimeTransportQueueRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingTransportQueueNonClaims(): void {
  it('rejects transport queue proof when required non-claims are removed', () => {
    const proof = AppInstallPurchaseExternalRuntimeTransportQueueProofReadModel;

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
        AppInstallPurchaseExternalRuntimeTransportQueueProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

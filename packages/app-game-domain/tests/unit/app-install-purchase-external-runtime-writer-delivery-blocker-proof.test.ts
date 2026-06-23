import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseExternalRuntimeWriterDeliveryBlockerProofReadModel,
  AppInstallPurchaseExternalRuntimeWriterDeliveryBlockerProofSchema,
  AppInstallPurchaseExternalRuntimeWriterDeliveryBlockerRowSchema,
  summarizeAppInstallPurchaseExternalRuntimeWriterDeliveryBlockerProof,
} from '@ocentra-parent/schema-domain/app-install-purchase-external-runtime-writer-delivery-blocker-proof';

describe('app install and purchase external runtime writer delivery blocker proof', () => {
  acceptsDeliveryBlockerRowsWithoutDeliveryClaims();
  rejectsMissingBoundaryRefsOrRuntimeBlockers();
  rejectsExternalRuntimeProviderStorePlatformChildReportCustodyAndBlockingOverclaims();
  rejectsMissingExternalRuntimeWriterDeliveryBlockerNonClaims();
});

function acceptsDeliveryBlockerRowsWithoutDeliveryClaims(): void {
  it('accepts deterministic external runtime writer delivery blockers without starting delivery', () => {
    const proof = AppInstallPurchaseExternalRuntimeWriterDeliveryBlockerProofSchema.parse(
      AppInstallPurchaseExternalRuntimeWriterDeliveryBlockerProofReadModel
    );

    expect(summarizeAppInstallPurchaseExternalRuntimeWriterDeliveryBlockerProof(proof)).toEqual({
      externalRuntimeWriterDeliveryBlockerRows: 4,
      blockedRuntimePrerequisiteRows: 3,
      manualRequiredRows: 1,
      deliveryAttemptStartedRows: 0,
      externalRuntimeWriterDeliveredRows: 0,
    });
    expect(
      proof.externalRuntimeWriterDeliveryBlockerRows.map(
        (row) => `${row.sourceDecisionAction}:${row.deliveryBlockerState}`
      )
    ).toEqual([
      'approve:blocked-runtime-prerequisites-missing',
      'deny:blocked-runtime-prerequisites-missing',
      'time-box:blocked-runtime-prerequisites-missing',
      'review-needed:manual-required',
    ]);

    for (const row of proof.externalRuntimeWriterDeliveryBlockerRows) {
      expect(row.sourceExternalRuntimeWriterDeliveryBoundaryRowId).toContain(row.sourceDecisionAction);
      expect(row.sourceExternalRuntimeWriterQueueRef).toContain(row.sourceDecisionAction);
      expect(row.requiredExternalWriterTransportProofRefs.length).toBeGreaterThan(0);
      expect(row.requiredPlatformAdapterProofRefs.length).toBeGreaterThan(0);
      expect(row.requiredProviderStoreProofRefs.length).toBeGreaterThan(0);
      expect(row.requiredChildDeviceDeliveryProofRefs.length).toBeGreaterThan(0);
      expect(row.requiredRuntimeBlockers).toEqual([
        'external-writer-transport-proof-missing',
        'platform-adapter-proof-missing',
        'provider-store-execution-proof-missing',
        'child-device-transport-proof-missing',
      ]);
      expect(row.manualBlockerRefs).toEqual([
        `missing-external-writer-transport-${row.sourceDecisionAction}`,
        `missing-platform-adapter-execution-${row.sourceDecisionAction}`,
        `missing-provider-store-execution-${row.sourceDecisionAction}`,
        `missing-child-device-transport-${row.sourceDecisionAction}`,
      ]);
      expect(row.deliveryAttemptState).toBe('not-started');
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
      expect(row.claimBoundary).toContain('delivery remains blocked');
      expect(row.claimBoundary).toContain('no external runtime writer delivery');
      expect(row.claimBoundary).toContain('no child-device delivery');
    }
  });
}

function rejectsMissingBoundaryRefsOrRuntimeBlockers(): void {
  it('rejects rows that omit source boundary refs or required blocker evidence', () => {
    const proof = AppInstallPurchaseExternalRuntimeWriterDeliveryBlockerProofReadModel;
    const row = proof.externalRuntimeWriterDeliveryBlockerRows[0];

    expect(
      AppInstallPurchaseExternalRuntimeWriterDeliveryBlockerProofSchema.safeParse({
        ...proof,
        externalRuntimeWriterDeliveryBlockerRows: proof.externalRuntimeWriterDeliveryBlockerRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseExternalRuntimeWriterDeliveryBlockerRowSchema.safeParse({
        ...row,
        sourceExternalRuntimeWriterDeliveryBoundaryRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseExternalRuntimeWriterDeliveryBlockerRowSchema.safeParse({
        ...row,
        requiredRuntimeBlockers: row.requiredRuntimeBlockers.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseExternalRuntimeWriterDeliveryBlockerRowSchema.safeParse({
        ...row,
        manualBlockerRefs: [],
      }).success
    ).toBe(false);
  });
}

function rejectsExternalRuntimeProviderStorePlatformChildReportCustodyAndBlockingOverclaims(): void {
  it('rejects rows that claim external runtime writer delivery or missing runtime execution exists', () => {
    const row =
      AppInstallPurchaseExternalRuntimeWriterDeliveryBlockerProofReadModel.externalRuntimeWriterDeliveryBlockerRows[0];

    for (const invalidRow of [
      { ...row, deliveryAttemptState: 'started' },
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
      { ...row, claimBoundary: 'external runtime writer delivery completed' },
    ]) {
      expect(AppInstallPurchaseExternalRuntimeWriterDeliveryBlockerRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingExternalRuntimeWriterDeliveryBlockerNonClaims(): void {
  it('rejects external runtime writer delivery blocker proof when required non-claims are removed', () => {
    const proof = AppInstallPurchaseExternalRuntimeWriterDeliveryBlockerProofReadModel;

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
        AppInstallPurchaseExternalRuntimeWriterDeliveryBlockerProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

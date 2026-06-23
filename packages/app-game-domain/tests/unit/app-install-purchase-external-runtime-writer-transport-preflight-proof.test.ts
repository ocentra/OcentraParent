import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseExternalRuntimeWriterTransportPreflightProofReadModel,
  AppInstallPurchaseExternalRuntimeWriterTransportPreflightProofSchema,
  AppInstallPurchaseExternalRuntimeWriterTransportPreflightRowSchema,
  summarizeAppInstallPurchaseExternalRuntimeWriterTransportPreflightProof,
} from '@ocentra-parent/schema-domain/app-install-purchase-external-runtime-writer-transport-preflight-proof';

describe('app install and purchase external runtime writer transport preflight proof', () => {
  acceptsExternalRuntimeWriterTransportPreflightRowsWithoutDeliveryClaims();
  rejectsMissingTransportQueueChildPlatformProviderOrReportCoverage();
  rejectsExternalRuntimeTransportProviderStorePlatformChildReportCustodyAndBlockingOverclaims();
  rejectsMissingExternalRuntimeWriterTransportPreflightNonClaims();
});

function acceptsExternalRuntimeWriterTransportPreflightRowsWithoutDeliveryClaims(): void {
  it('accepts deterministic transport preflight rows without external writer delivery claims', () => {
    const proof = AppInstallPurchaseExternalRuntimeWriterTransportPreflightProofSchema.parse(
      AppInstallPurchaseExternalRuntimeWriterTransportPreflightProofReadModel
    );

    expect(summarizeAppInstallPurchaseExternalRuntimeWriterTransportPreflightProof(proof)).toEqual({
      externalRuntimeWriterTransportPreflightRows: 4,
      transportPreflightReadyRows: 3,
      parentOwnedQueueRefReadyRows: 3,
      manualRequiredRows: 1,
      externalRuntimeWriterExecutedRows: 0,
      externalRuntimeWriterDeliveredRows: 0,
    });
    expect(
      proof.externalRuntimeWriterTransportPreflightRows.map(
        (row) =>
          `${row.sourceDecisionAction}:${row.sourceExternalRuntimeWriterReadinessState}:${row.externalRuntimeWriterTransportPreflightState}`
      )
    ).toEqual([
      'approve:writer-handoff-ready:transport-preflight-ready',
      'deny:writer-handoff-ready:transport-preflight-ready',
      'time-box:writer-handoff-ready:transport-preflight-ready',
      'review-needed:manual-required:manual-required',
    ]);

    for (const row of proof.externalRuntimeWriterTransportPreflightRows) {
      expect(row.sourceExternalRuntimeWriterReadinessRowId).toContain(row.sourceDecisionAction);
      expect(row.sourceExternalRuntimeWriterPreflightRef).toContain(row.sourceDecisionAction);
      expect(row.sourceExternalRuntimeWriterReceiptRef).toContain(row.sourceDecisionAction);
      expect(row.externalRuntimeWriterTransportPreflightRef).toContain(row.sourceDecisionAction);
      expect(row.requiredExternalWriterTransportProofRefs.length).toBeGreaterThan(0);
      expect(row.requiredExternalWriterQueueProofRefs.length).toBeGreaterThan(0);
      expect(row.requiredChildDeviceTransportProofRefs.length).toBeGreaterThan(0);
      expect(row.requiredPlatformAdapterProofRefs.length).toBeGreaterThan(0);
      expect(row.requiredProviderStoreProofRefs.length).toBeGreaterThan(0);
      expect(row.externalRuntimeWriterAuditEventRefs.length).toBeGreaterThan(0);
      expect(row.childDeliveryAuditEventRefs.length).toBeGreaterThan(0);
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
      expect(row.claimBoundary).toContain('parent-owned external writer transport');
      expect(row.claimBoundary).toContain('no external runtime writer delivery');
      expect(row.claimBoundary).toContain('no child-device delivery');
    }
  });
}

function rejectsMissingTransportQueueChildPlatformProviderOrReportCoverage(): void {
  it('rejects rows that omit source, transport, queue, child, platform, provider, audit, or report refs', () => {
    const proof = AppInstallPurchaseExternalRuntimeWriterTransportPreflightProofReadModel;
    const row = proof.externalRuntimeWriterTransportPreflightRows[0];

    expect(
      AppInstallPurchaseExternalRuntimeWriterTransportPreflightProofSchema.safeParse({
        ...proof,
        externalRuntimeWriterTransportPreflightRows: proof.externalRuntimeWriterTransportPreflightRows.slice(1),
      }).success
    ).toBe(false);

    for (const invalidRow of [
      { ...row, sourceExternalRuntimeWriterReadinessRowId: '' },
      { ...row, sourceExternalRuntimeWriterPreflightRef: '' },
      { ...row, sourceExternalRuntimeWriterReceiptRef: '' },
      { ...row, sourceExternalRuntimeWriterTargetRefs: [] },
      { ...row, externalRuntimeWriterTransportPreflightRef: '' },
      { ...row, requiredExternalWriterTransportProofRefs: [] },
      { ...row, requiredExternalWriterQueueProofRefs: [] },
      { ...row, requiredChildDeviceTransportProofRefs: [] },
      { ...row, requiredPlatformAdapterProofRefs: [] },
      { ...row, requiredProviderStoreProofRefs: [] },
      { ...row, externalRuntimeWriterAuditEventRefs: [] },
      { ...row, childDeliveryAuditEventRefs: [] },
      { ...row, reportRuntimeRefs: [] },
    ]) {
      expect(AppInstallPurchaseExternalRuntimeWriterTransportPreflightRowSchema.safeParse(invalidRow).success).toBe(
        false
      );
    }
  });
}

function rejectsExternalRuntimeTransportProviderStorePlatformChildReportCustodyAndBlockingOverclaims(): void {
  it('rejects rows that claim external transport provider store platform child report custody or blocking behavior', () => {
    const row =
      AppInstallPurchaseExternalRuntimeWriterTransportPreflightProofReadModel
        .externalRuntimeWriterTransportPreflightRows[0];

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
      { ...row, claimBoundary: 'external runtime writer transport delivered to a child device' },
    ]) {
      expect(AppInstallPurchaseExternalRuntimeWriterTransportPreflightRowSchema.safeParse(invalidRow).success).toBe(
        false
      );
    }
  });
}

function rejectsMissingExternalRuntimeWriterTransportPreflightNonClaims(): void {
  it('rejects transport preflight proof when required non-claims are removed', () => {
    const proof = AppInstallPurchaseExternalRuntimeWriterTransportPreflightProofReadModel;

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
        AppInstallPurchaseExternalRuntimeWriterTransportPreflightProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

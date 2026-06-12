import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseExternalRuntimeWriterReadinessProofReadModel,
  AppInstallPurchaseExternalRuntimeWriterReadinessProofSchema,
  AppInstallPurchaseExternalRuntimeWriterReadinessRowSchema,
  summarizeAppInstallPurchaseExternalRuntimeWriterReadinessProof,
} from '../../src/app-install-purchase-external-runtime-writer-readiness-proof';

describe('app install and purchase external runtime writer readiness proof', () => {
  acceptsExternalRuntimeWriterReadinessRowsWithoutDeliveryClaims();
  rejectsMissingSourceWriterQueueAuditOrReportCoverage();
  rejectsExternalRuntimeProviderStorePlatformChildReportCustodyAndBlockingOverclaims();
  rejectsMissingExternalRuntimeWriterReadinessNonClaims();
});

function acceptsExternalRuntimeWriterReadinessRowsWithoutDeliveryClaims(): void {
  it('accepts deterministic writer readiness rows without external writer delivery claims', () => {
    const proof = AppInstallPurchaseExternalRuntimeWriterReadinessProofSchema.parse(
      AppInstallPurchaseExternalRuntimeWriterReadinessProofReadModel
    );

    expect(summarizeAppInstallPurchaseExternalRuntimeWriterReadinessProof(proof)).toEqual({
      externalRuntimeWriterReadinessRows: 4,
      writerHandoffReadyRows: 3,
      queuePreflightReadyRows: 3,
      manualRequiredRows: 1,
      externalRuntimeWriterExecutedRows: 0,
      externalRuntimeWriterDeliveredRows: 0,
    });
    expect(
      proof.externalRuntimeWriterReadinessRows.map(
        (row) =>
          `${row.sourceDecisionAction}:${row.sourceExternalRuntimeEvidenceState}:${row.externalRuntimeWriterReadinessState}`
      )
    ).toEqual([
      'approve:external-runtime-evidence-ready:writer-handoff-ready',
      'deny:external-runtime-evidence-ready:writer-handoff-ready',
      'time-box:external-runtime-evidence-ready:writer-handoff-ready',
      'review-needed:manual-required:manual-required',
    ]);

    for (const row of proof.externalRuntimeWriterReadinessRows) {
      expect(row.sourceExternalRuntimeDeviceDeliveryRowId).toContain(row.sourceDecisionAction);
      expect(row.sourceRuntimeWriterEnvelopeRef).toContain(row.sourceDecisionAction);
      expect(row.sourceDeliveryResultReceiptRef).toContain(row.sourceDecisionAction);
      expect(row.externalRuntimeWriterPreflightRef).toContain(row.sourceDecisionAction);
      expect(row.externalRuntimeWriterReceiptRef).toContain(row.sourceDecisionAction);
      expect(row.externalRuntimeWriterTargetRefs).toContain(row.sourceRuntimeWriterEnvelopeRef);
      expect(row.externalRuntimeWriterTargetRefs).toContain(row.sourceDeliveryResultReceiptRef);
      expect(row.sourceExternalRuntimeWriterTargetRefs.length).toBeGreaterThan(0);
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
      expect(row.claimBoundary).toContain('external runtime target refs');
      expect(row.claimBoundary).toContain('no external runtime writer delivery');
      expect(row.claimBoundary).toContain('no child-device delivery');
    }
  });
}

function rejectsMissingSourceWriterQueueAuditOrReportCoverage(): void {
  it('rejects rows that omit source rows, writer refs, queue refs, audit refs, or report refs', () => {
    const proof = AppInstallPurchaseExternalRuntimeWriterReadinessProofReadModel;
    const row = proof.externalRuntimeWriterReadinessRows[0];

    expect(
      AppInstallPurchaseExternalRuntimeWriterReadinessProofSchema.safeParse({
        ...proof,
        externalRuntimeWriterReadinessRows: proof.externalRuntimeWriterReadinessRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseExternalRuntimeWriterReadinessRowSchema.safeParse({
        ...row,
        sourceRuntimeWriterEnvelopeRef: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseExternalRuntimeWriterReadinessRowSchema.safeParse({
        ...row,
        sourceDeliveryResultReceiptRef: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseExternalRuntimeWriterReadinessRowSchema.safeParse({
        ...row,
        sourceExternalRuntimeWriterTargetRefs: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseExternalRuntimeWriterReadinessRowSchema.safeParse({
        ...row,
        externalRuntimeWriterPreflightRef: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseExternalRuntimeWriterReadinessRowSchema.safeParse({
        ...row,
        externalRuntimeWriterReceiptRef: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseExternalRuntimeWriterReadinessRowSchema.safeParse({
        ...row,
        externalRuntimeWriterAuditEventRefs: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseExternalRuntimeWriterReadinessRowSchema.safeParse({
        ...row,
        childDeliveryAuditEventRefs: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseExternalRuntimeWriterReadinessRowSchema.safeParse({
        ...row,
        reportRuntimeRefs: [],
      }).success
    ).toBe(false);
  });
}

function rejectsExternalRuntimeProviderStorePlatformChildReportCustodyAndBlockingOverclaims(): void {
  it('rejects rows that claim external writer provider store platform child report custody or blocking behavior', () => {
    const row = AppInstallPurchaseExternalRuntimeWriterReadinessProofReadModel.externalRuntimeWriterReadinessRows[0];

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
      expect(AppInstallPurchaseExternalRuntimeWriterReadinessRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingExternalRuntimeWriterReadinessNonClaims(): void {
  it('rejects external runtime writer readiness proof when required non-claims are removed', () => {
    const proof = AppInstallPurchaseExternalRuntimeWriterReadinessProofReadModel;

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
        AppInstallPurchaseExternalRuntimeWriterReadinessProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

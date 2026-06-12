import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseExternalRuntimeDeviceDeliveryProofReadModel,
  AppInstallPurchaseExternalRuntimeDeviceDeliveryProofSchema,
  AppInstallPurchaseExternalRuntimeDeviceDeliveryRowSchema,
  summarizeAppInstallPurchaseExternalRuntimeDeviceDeliveryProof,
} from '../../src/app-install-purchase-external-runtime-device-delivery-proof';

describe('app install and purchase external runtime device delivery proof', () => {
  acceptsLinkedWriterReceiptAndChildEnvelopeEvidenceWithoutDeliveryClaims();
  rejectsMissingWriterReceiptChildEnvelopeAuditOrReportCoverage();
  rejectsExternalRuntimeProviderStorePlatformChildReportCustodyAndBlockingOverclaims();
  rejectsMissingExternalRuntimeDeviceDeliveryNonClaims();
});

function acceptsLinkedWriterReceiptAndChildEnvelopeEvidenceWithoutDeliveryClaims(): void {
  it('accepts deterministic external runtime evidence rows without external or child delivery claims', () => {
    const proof = AppInstallPurchaseExternalRuntimeDeviceDeliveryProofSchema.parse(
      AppInstallPurchaseExternalRuntimeDeviceDeliveryProofReadModel
    );

    expect(summarizeAppInstallPurchaseExternalRuntimeDeviceDeliveryProof(proof)).toEqual({
      externalRuntimeDeviceDeliveryRows: 4,
      externalRuntimeEvidenceReadyRows: 3,
      manualRequiredRows: 1,
      externalRuntimeWriterDeliveredRows: 0,
      childDeviceDeliveredRows: 0,
    });
    expect(
      proof.externalRuntimeDeviceDeliveryRows.map(
        (row) => `${row.sourceDecisionAction}:${row.externalRuntimeEvidenceState}`
      )
    ).toEqual([
      'approve:external-runtime-evidence-ready',
      'deny:external-runtime-evidence-ready',
      'time-box:external-runtime-evidence-ready',
      'review-needed:manual-required',
    ]);

    for (const row of proof.externalRuntimeDeviceDeliveryRows) {
      expect(row.sourceRuntimeWriterExecutionDeliveryRowId).toContain(row.sourceDecisionAction);
      expect(row.sourceChildDeviceDeliveryRuntimeWriterRowId).toContain(row.sourceDecisionAction);
      expect(row.sourceRuntimeWriterEnvelopeRef).toContain(row.sourceDecisionAction);
      expect(row.sourceDeliveryResultReceiptRef).toContain(row.sourceDecisionAction);
      expect(row.sourceChildDeliveryTargetRefs.length).toBeGreaterThan(0);
      expect(row.externalRuntimeWriterTargetRefs).toContain(row.sourceRuntimeWriterEnvelopeRef);
      expect(row.externalRuntimeWriterTargetRefs).toContain(row.sourceDeliveryResultReceiptRef);
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
      expect(row.claimBoundary).toContain('parent-owned runtime writer envelope');
      expect(row.claimBoundary).toContain('child delivery envelope');
      expect(row.claimBoundary).toContain('no external runtime writer delivery');
      expect(row.claimBoundary).toContain('no child-device delivery');
    }
  });
}

function rejectsMissingWriterReceiptChildEnvelopeAuditOrReportCoverage(): void {
  it('rejects rows that omit source rows, writer refs, child envelope refs, audit refs, or report refs', () => {
    const proof = AppInstallPurchaseExternalRuntimeDeviceDeliveryProofReadModel;
    const row = proof.externalRuntimeDeviceDeliveryRows[0];

    expect(
      AppInstallPurchaseExternalRuntimeDeviceDeliveryProofSchema.safeParse({
        ...proof,
        externalRuntimeDeviceDeliveryRows: proof.externalRuntimeDeviceDeliveryRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseExternalRuntimeDeviceDeliveryRowSchema.safeParse({
        ...row,
        sourceRuntimeWriterEnvelopeRef: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseExternalRuntimeDeviceDeliveryRowSchema.safeParse({
        ...row,
        sourceDeliveryResultReceiptRef: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseExternalRuntimeDeviceDeliveryRowSchema.safeParse({
        ...row,
        sourceChildDeliveryTargetRefs: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseExternalRuntimeDeviceDeliveryRowSchema.safeParse({
        ...row,
        externalRuntimeWriterAuditEventRefs: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseExternalRuntimeDeviceDeliveryRowSchema.safeParse({
        ...row,
        childDeliveryAuditEventRefs: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseExternalRuntimeDeviceDeliveryRowSchema.safeParse({ ...row, reportRuntimeRefs: [] }).success
    ).toBe(false);
  });
}

function rejectsExternalRuntimeProviderStorePlatformChildReportCustodyAndBlockingOverclaims(): void {
  it('rejects rows that claim external runtime delivery provider store platform child report custody or blocking behavior', () => {
    const row = AppInstallPurchaseExternalRuntimeDeviceDeliveryProofReadModel.externalRuntimeDeviceDeliveryRows[0];

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
      expect(AppInstallPurchaseExternalRuntimeDeviceDeliveryRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingExternalRuntimeDeviceDeliveryNonClaims(): void {
  it('rejects external runtime device delivery proof when required non-claims are removed', () => {
    const proof = AppInstallPurchaseExternalRuntimeDeviceDeliveryProofReadModel;

    for (const claim of [
      'no-external-runtime-writer-execution',
      'no-external-runtime-writer-delivery',
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
        AppInstallPurchaseExternalRuntimeDeviceDeliveryProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

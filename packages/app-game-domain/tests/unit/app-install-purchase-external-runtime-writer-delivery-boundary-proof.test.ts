import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseExternalRuntimeWriterDeliveryBoundaryProofReadModel,
  AppInstallPurchaseExternalRuntimeWriterDeliveryBoundaryProofSchema,
  AppInstallPurchaseExternalRuntimeWriterDeliveryBoundaryRowSchema,
  summarizeAppInstallPurchaseExternalRuntimeWriterDeliveryBoundaryProof,
} from '@ocentra-parent/schema-domain/app-install-purchase-external-runtime-writer-delivery-boundary-proof';

describe('app install and purchase external runtime writer delivery boundary proof', () => {
  acceptsExternalRuntimeWriterDeliveryPrerequisiteRowsWithoutDeliveryClaims();
  rejectsMissingHandoffOrRequiredDeliveryProofRefs();
  rejectsExternalRuntimeProviderStorePlatformChildReportCustodyAndBlockingOverclaims();
  rejectsMissingExternalRuntimeWriterDeliveryBoundaryNonClaims();
});

function acceptsExternalRuntimeWriterDeliveryPrerequisiteRowsWithoutDeliveryClaims(): void {
  it('accepts deterministic writer delivery boundary rows without external or child delivery claims', () => {
    const proof = AppInstallPurchaseExternalRuntimeWriterDeliveryBoundaryProofSchema.parse(
      AppInstallPurchaseExternalRuntimeWriterDeliveryBoundaryProofReadModel
    );

    expect(summarizeAppInstallPurchaseExternalRuntimeWriterDeliveryBoundaryProof(proof)).toEqual({
      externalRuntimeWriterDeliveryBoundaryRows: 4,
      prerequisiteReadyRows: 3,
      manualRequiredRows: 1,
      externalRuntimeWriterDeliveredRows: 0,
      childDeviceDeliveredRows: 0,
    });
    expect(
      proof.externalRuntimeWriterDeliveryBoundaryRows.map(
        (row) => `${row.sourceDecisionAction}:${row.externalRuntimeWriterDeliveryBoundaryState}`
      )
    ).toEqual([
      'approve:runtime-writer-delivery-prerequisites-ready',
      'deny:runtime-writer-delivery-prerequisites-ready',
      'time-box:runtime-writer-delivery-prerequisites-ready',
      'review-needed:manual-required',
    ]);

    for (const row of proof.externalRuntimeWriterDeliveryBoundaryRows) {
      expect(row.sourceExternalRuntimeDeliveryHandoffRowId).toContain(row.sourceDecisionAction);
      expect(row.sourceExternalRuntimeHandoffPacketRef).toContain(row.sourceDecisionAction);
      expect(row.sourceExternalRuntimeWriterQueueRef).toContain(row.sourceDecisionAction);
      expect(row.sourceExternalRuntimeWriterDispatchAuditEventRefs.length).toBeGreaterThan(0);
      expect(row.sourceReportRuntimeRefs.length).toBeGreaterThan(0);
      expect(row.requiredExternalWriterTransportProofRefs).toEqual([
        `external-writer-transport-proof-${row.sourceDecisionAction}`,
      ]);
      expect(row.requiredPlatformAdapterProofRefs).toEqual([`platform-adapter-proof-${row.sourceDecisionAction}`]);
      expect(row.requiredProviderStoreProofRefs).toEqual([
        `provider-store-execution-proof-${row.sourceDecisionAction}`,
      ]);
      expect(row.requiredChildDeviceDeliveryProofRefs).toEqual([
        `child-device-delivery-proof-${row.sourceDecisionAction}`,
      ]);
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
      expect(row.claimBoundary).toContain('required external writer transport proof refs');
      expect(row.claimBoundary).toContain('no external runtime writer delivery');
      expect(row.claimBoundary).toContain('no child-device delivery');
    }
  });
}

function rejectsMissingHandoffOrRequiredDeliveryProofRefs(): void {
  it('rejects rows that omit source handoff refs or required writer/platform/provider/child proof refs', () => {
    const proof = AppInstallPurchaseExternalRuntimeWriterDeliveryBoundaryProofReadModel;
    const row = proof.externalRuntimeWriterDeliveryBoundaryRows[0];

    expect(
      AppInstallPurchaseExternalRuntimeWriterDeliveryBoundaryProofSchema.safeParse({
        ...proof,
        externalRuntimeWriterDeliveryBoundaryRows: proof.externalRuntimeWriterDeliveryBoundaryRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseExternalRuntimeWriterDeliveryBoundaryRowSchema.safeParse({
        ...row,
        sourceExternalRuntimeDeliveryHandoffRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseExternalRuntimeWriterDeliveryBoundaryRowSchema.safeParse({
        ...row,
        requiredExternalWriterTransportProofRefs: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseExternalRuntimeWriterDeliveryBoundaryRowSchema.safeParse({
        ...row,
        requiredPlatformAdapterProofRefs: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseExternalRuntimeWriterDeliveryBoundaryRowSchema.safeParse({
        ...row,
        requiredProviderStoreProofRefs: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseExternalRuntimeWriterDeliveryBoundaryRowSchema.safeParse({
        ...row,
        requiredChildDeviceDeliveryProofRefs: [],
      }).success
    ).toBe(false);
  });
}

function rejectsExternalRuntimeProviderStorePlatformChildReportCustodyAndBlockingOverclaims(): void {
  it('rejects rows that claim external runtime delivery provider store platform child report custody or blocking behavior', () => {
    const row =
      AppInstallPurchaseExternalRuntimeWriterDeliveryBoundaryProofReadModel
        .externalRuntimeWriterDeliveryBoundaryRows[0];

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
      expect(AppInstallPurchaseExternalRuntimeWriterDeliveryBoundaryRowSchema.safeParse(invalidRow).success).toBe(
        false
      );
    }
  });
}

function rejectsMissingExternalRuntimeWriterDeliveryBoundaryNonClaims(): void {
  it('rejects external runtime writer delivery boundary proof when required non-claims are removed', () => {
    const proof = AppInstallPurchaseExternalRuntimeWriterDeliveryBoundaryProofReadModel;

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
        AppInstallPurchaseExternalRuntimeWriterDeliveryBoundaryProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

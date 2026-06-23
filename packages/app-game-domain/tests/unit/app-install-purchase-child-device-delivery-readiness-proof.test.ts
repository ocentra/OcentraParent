import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseChildDeviceDeliveryReadinessProofReadModel,
  AppInstallPurchaseChildDeviceDeliveryReadinessProofSchema,
  AppInstallPurchaseChildDeviceDeliveryReadinessRowSchema,
  summarizeAppInstallPurchaseChildDeviceDeliveryReadinessProof,
} from '@ocentra-parent/schema-domain/app-install-purchase-child-device-delivery-readiness-proof';

describe('app install and purchase child-device delivery readiness proof', () => {
  acceptsChildDeviceDeliveryReadinessRows();
  rejectsMissingEvidenceRefs();
  rejectsDeliveryProviderAdapterBlockingAndCustodyOverclaims();
  rejectsMissingChildDeviceDeliveryReadinessNonClaims();
});

function acceptsChildDeviceDeliveryReadinessRows(): void {
  it('accepts five platform readiness rows without child delivery claims', () => {
    const proof = AppInstallPurchaseChildDeviceDeliveryReadinessProofSchema.parse(
      AppInstallPurchaseChildDeviceDeliveryReadinessProofReadModel
    );

    expect(summarizeAppInstallPurchaseChildDeviceDeliveryReadinessProof(proof)).toEqual({
      childDeviceDeliveryReadinessRows: 5,
      deliveryEvidenceReadyRows: 1,
      manualProofRequiredRows: 1,
      platformUnavailableRows: 1,
      policyBlockedRows: 2,
      childDeviceDeliveredRows: 0,
    });
    expect(
      proof.childDeviceDeliveryReadinessRows.map((row) => `${row.platform}:${row.childDeviceDeliveryReadinessState}`)
    ).toEqual([
      'windows:delivery-evidence-ready',
      'macos:manual-proof-required',
      'linux:platform-unavailable',
      'android:policy-blocked',
      'ios:policy-blocked',
    ]);
    for (const row of proof.childDeviceDeliveryReadinessRows) {
      expect(row.sourceChildDeliveryRuntimeWriterRowIds).toHaveLength(4);
      expect(row.sourcePackageSourceAdapterExecutionRowId).toContain(row.platform);
      expect(row.sourcePlatformLimitationActionRowId).toContain(row.platform);
      expect(row.requiredDeliveryProofRefs.length).toBeGreaterThan(0);
      expect(row.parentVisibleStatusRefs.length).toBeGreaterThan(0);
      expect(row.childDeviceDeliveryClaim).toBe('not-delivered');
      expect(row.runtimeWriterExecutionClaim).toBe('not-executed');
      expect(row.runtimeWriterDeliveryClaim).toBe('not-delivered');
      expect(row.providerApiExecutionClaim).toBe('not-executed');
      expect(row.storeIntegrationClaim).toBe('not-claimed');
      expect(row.platformAdapterClaim).toBe('not-implemented');
      expect(row.appBlockingClaim).toBe('not-claimed');
      expect(row.childDataCustody).toBe('no-child-activity-data');
      expect(row.ocentraHostedFamilyDataCustodyClaim).toBe('not-claimed');
      expect(row.claimBoundary).toContain('child delivery runtime-writer envelopes');
      expect(row.claimBoundary).toContain('no child-device delivery');
    }
  });
}

function rejectsMissingEvidenceRefs(): void {
  it('rejects rows that omit source child delivery adapter limitation or status refs', () => {
    const proof = AppInstallPurchaseChildDeviceDeliveryReadinessProofReadModel;
    const row = proof.childDeviceDeliveryReadinessRows[0];

    expect(
      AppInstallPurchaseChildDeviceDeliveryReadinessProofSchema.safeParse({
        ...proof,
        childDeviceDeliveryReadinessRows: proof.childDeviceDeliveryReadinessRows.slice(1),
      }).success
    ).toBe(false);
    for (const invalidRow of [
      { ...row, sourceChildDeliveryRuntimeWriterRowIds: row.sourceChildDeliveryRuntimeWriterRowIds.slice(1) },
      { ...row, requiredDeliveryProofRefs: [] },
      { ...row, parentVisibleStatusRefs: [] },
      { ...row, claimBoundary: 'child delivery is complete' },
    ]) {
      expect(AppInstallPurchaseChildDeviceDeliveryReadinessRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsDeliveryProviderAdapterBlockingAndCustodyOverclaims(): void {
  it('rejects rows that claim delivery provider execution adapters blocking or custody', () => {
    const row = AppInstallPurchaseChildDeviceDeliveryReadinessProofReadModel.childDeviceDeliveryReadinessRows[0];

    for (const invalidRow of [
      { ...row, childDeviceDeliveryReadinessState: 'delivered' },
      { ...row, childDeviceDeliveryClaim: 'delivered' },
      { ...row, runtimeWriterExecutionClaim: 'executed' },
      { ...row, runtimeWriterDeliveryClaim: 'delivered' },
      { ...row, providerApiExecutionClaim: 'executed' },
      { ...row, storeIntegrationClaim: 'claimed' },
      { ...row, platformAdapterClaim: 'implemented' },
      { ...row, appBlockingClaim: 'claimed' },
      { ...row, childDataCustody: 'child-activity-data-included' },
      { ...row, ocentraHostedFamilyDataCustodyClaim: 'claimed' },
    ]) {
      expect(AppInstallPurchaseChildDeviceDeliveryReadinessRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingChildDeviceDeliveryReadinessNonClaims(): void {
  it('rejects proofs when delivery provider adapter blocking or custody non-claims are removed', () => {
    const proof = AppInstallPurchaseChildDeviceDeliveryReadinessProofReadModel;

    for (const claim of [
      'no-child-device-delivery',
      'no-runtime-writer-execution',
      'no-runtime-writer-delivery',
      'no-provider-api-execution',
      'no-store-integration',
      'no-platform-adapter-implementation',
      'no-app-blocking',
      'no-child-activity-data',
      'no-ocentra-hosted-family-data-custody',
    ] as const) {
      expect(
        AppInstallPurchaseChildDeviceDeliveryReadinessProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

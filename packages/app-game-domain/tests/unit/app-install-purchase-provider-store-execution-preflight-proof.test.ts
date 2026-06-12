import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseProviderStoreExecutionPreflightProofReadModel,
  AppInstallPurchaseProviderStoreExecutionPreflightProofSchema,
  AppInstallPurchaseProviderStoreExecutionPreflightRowSchema,
  summarizeAppInstallPurchaseProviderStoreExecutionPreflightProof,
} from '../../src/app-install-purchase-provider-store-execution-preflight-proof';

describe('app install and purchase provider store execution preflight proof', () => {
  acceptsProviderStoreExecutionPreflightRowsWithoutExecutionClaims();
  rejectsMissingProviderStoreExecutionPreflightCoverageOrRefs();
  rejectsProviderStoreExecutionRuntimeDeliveryCustodyAndBlockingOverclaims();
  rejectsMissingProviderStoreExecutionPreflightNonClaims();
});

function acceptsProviderStoreExecutionPreflightRowsWithoutExecutionClaims(): void {
  it('accepts provider store execution preflight rows linked to readiness and runtime writer receipts', () => {
    const proof = AppInstallPurchaseProviderStoreExecutionPreflightProofSchema.parse(
      AppInstallPurchaseProviderStoreExecutionPreflightProofReadModel
    );

    expect(summarizeAppInstallPurchaseProviderStoreExecutionPreflightProof(proof)).toEqual({
      providerStoreExecutionPreflightRows: 5,
      preflightReadyRows: 1,
      manualProviderProofRequiredRows: 3,
      providerUnavailableRows: 1,
      providerExecutedRows: 0,
      runtimeDeviceDeliveredRows: 0,
    });
    expect(
      proof.providerStoreExecutionPreflightRows.map(
        (row) =>
          `${row.platform}:${row.storeSurface}:${row.sourceProviderStoreExecutionReadinessState}:${row.providerStoreExecutionPreflightState}`
      )
    ).toEqual([
      'windows:microsoft-store:provider-store-execution-ready:preflight-ready',
      'macos:mac-app-store:manual-required:manual-provider-proof-required',
      'linux:linux-package-manager:unavailable:provider-unavailable',
      'android:google-play:manual-required:manual-provider-proof-required',
      'ios:apple-app-store:manual-required:manual-provider-proof-required',
    ]);
    for (const row of proof.providerStoreExecutionPreflightRows) {
      expect(row.sourceProviderStoreExecutionReadinessRowId).toContain(row.platform);
      expect(row.sourceRuntimeWriterExecutionDeliveryRowIds).toHaveLength(4);
      expect(row.sourceRuntimeWriterReceiptClaims).toEqual([
        'parent-owned-delivery-result-recorded',
        'parent-owned-delivery-result-recorded',
        'parent-owned-delivery-result-recorded',
        'manual-required',
      ]);
      expect(row.requiredProviderEvidenceRefs.length).toBeGreaterThan(0);
      expect(row.runtimeWriterReceiptRefs).toHaveLength(4);
      expect(row.auditEventRefs.length).toBeGreaterThan(0);
      expect(row.reportRuntimeRefs.length).toBeGreaterThan(0);
      expect(row.googlePlayExecutionClaim).toBe('not-executed');
      expect(row.appleAppStoreExecutionClaim).toBe('not-executed');
      expect(row.microsoftStoreExecutionClaim).toBe('not-executed');
      expect(row.billingProviderContactClaim).toBe('not-executed');
      expect(row.providerApiExecutionClaim).toBe('not-executed');
      expect(row.storeIntegrationClaim).toBe('not-claimed');
      expect(row.platformInterceptionClaim).toBe('not-claimed');
      expect(row.platformAdapterClaim).toBe('not-implemented');
      expect(row.runtimeDeviceDeliveryClaim).toBe('not-delivered');
      expect(row.childDeviceDeliveryClaim).toBe('not-delivered');
      expect(row.appBlockingClaim).toBe('not-claimed');
      expect(row.childDataCustody).toBe('no-child-activity-data');
      expect(row.ocentraHostedFamilyDataCustodyClaim).toBe('not-claimed');
      expect(row.claimBoundary).toContain('parent-owned preflight row');
      expect(row.claimBoundary).toContain('no runtime device delivery');
    }
  });
}

function rejectsMissingProviderStoreExecutionPreflightCoverageOrRefs(): void {
  it('rejects proofs that omit provider store rows or required source refs', () => {
    const proof = AppInstallPurchaseProviderStoreExecutionPreflightProofReadModel;
    const row = proof.providerStoreExecutionPreflightRows[0];

    expect(
      AppInstallPurchaseProviderStoreExecutionPreflightProofSchema.safeParse({
        ...proof,
        providerStoreExecutionPreflightRows: proof.providerStoreExecutionPreflightRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProviderStoreExecutionPreflightRowSchema.safeParse({
        ...row,
        sourceProviderStoreExecutionReadinessRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProviderStoreExecutionPreflightRowSchema.safeParse({
        ...row,
        sourceRuntimeWriterExecutionDeliveryRowIds: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProviderStoreExecutionPreflightRowSchema.safeParse({
        ...row,
        requiredProviderEvidenceRefs: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProviderStoreExecutionPreflightRowSchema.safeParse({
        ...row,
        runtimeWriterReceiptRefs: [],
      }).success
    ).toBe(false);
  });
}

function rejectsProviderStoreExecutionRuntimeDeliveryCustodyAndBlockingOverclaims(): void {
  it('rejects rows that claim provider execution runtime delivery custody or blocking', () => {
    const row = AppInstallPurchaseProviderStoreExecutionPreflightProofReadModel.providerStoreExecutionPreflightRows[0];

    for (const invalidRow of [
      { ...row, providerStoreExecutionPreflightState: 'executed' },
      { ...row, googlePlayExecutionClaim: 'executed' },
      { ...row, appleAppStoreExecutionClaim: 'executed' },
      { ...row, microsoftStoreExecutionClaim: 'executed' },
      { ...row, billingProviderContactClaim: 'contacted' },
      { ...row, providerApiExecutionClaim: 'executed' },
      { ...row, storeIntegrationClaim: 'claimed' },
      { ...row, platformInterceptionClaim: 'claimed' },
      { ...row, platformAdapterClaim: 'implemented' },
      { ...row, runtimeDeviceDeliveryClaim: 'delivered' },
      { ...row, childDeviceDeliveryClaim: 'delivered' },
      { ...row, appBlockingClaim: 'claimed' },
      { ...row, childDataCustody: 'child-activity-data-included' },
      { ...row, ocentraHostedFamilyDataCustodyClaim: 'claimed' },
      { ...row, claimBoundary: 'provider execution contacted Google Play and delivered to runtime device' },
    ]) {
      expect(AppInstallPurchaseProviderStoreExecutionPreflightRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingProviderStoreExecutionPreflightNonClaims(): void {
  it('rejects provider store execution preflight proof when required non-claims are removed', () => {
    const proof = AppInstallPurchaseProviderStoreExecutionPreflightProofReadModel;

    for (const claim of [
      'no-google-play-execution',
      'no-apple-app-store-execution',
      'no-microsoft-store-execution',
      'no-billing-provider-contact',
      'no-provider-api-execution',
      'no-store-integration',
      'no-platform-interception',
      'no-platform-adapter-implementation',
      'no-runtime-device-delivery',
      'no-child-device-delivery',
      'no-app-blocking',
      'no-child-activity-data',
      'no-ocentra-hosted-family-data-custody',
    ] as const) {
      expect(
        AppInstallPurchaseProviderStoreExecutionPreflightProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

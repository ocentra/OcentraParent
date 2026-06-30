import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseProviderStoreExecutionPreflightProofReadModel,
  AppInstallPurchaseProviderStoreExecutionPreflightProofSchema,
  AppInstallPurchaseProviderStoreExecutionPreflightRowSchema,
  summarizeAppInstallPurchaseProviderStoreExecutionPreflightProof,
} from '@ocentra-parent/schema-domain/app-install-purchase-provider-store-execution-preflight-proof';

describe('app install and purchase provider store execution preflight proof', () => {
  acceptsProviderStoreExecutionPreflightRows();
  rejectsMissingRefsOverclaimsAndMissingNonClaims();
});

function acceptsProviderStoreExecutionPreflightRows(): void {
  it('accepts parent-owned provider store execution preflight rows without provider execution or runtime delivery claims', () => {
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
      expect(row.providerApiExecutionClaim).toBe('not-executed');
      expect(row.storeIntegrationClaim).toBe('not-claimed');
      expect(row.platformInterceptionClaim).toBe('not-claimed');
      expect(row.platformAdapterClaim).toBe('not-implemented');
      expect(row.runtimeDeviceDeliveryClaim).toBe('not-delivered');
      expect(row.childDeviceDeliveryClaim).toBe('not-delivered');
      expect(row.childDataCustody).toBe('no-child-activity-data');
      expect(row.claimBoundary).toContain('no provider API execution');
      expect(row.claimBoundary).toContain('no runtime device delivery');
    }
  });
}

function rejectsMissingRefsOverclaimsAndMissingNonClaims(): void {
  it('rejects provider store execution preflight rows that omit refs or invent provider execution, delivery, or custody claims', () => {
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

    for (const invalidRow of [
      { ...row, providerApiExecutionClaim: 'executed' },
      { ...row, storeIntegrationClaim: 'claimed' },
      { ...row, platformInterceptionClaim: 'claimed' },
      { ...row, platformAdapterClaim: 'implemented' },
      { ...row, runtimeDeviceDeliveryClaim: 'delivered' },
      { ...row, childDeviceDeliveryClaim: 'delivered' },
      { ...row, appBlockingClaim: 'claimed' },
      { ...row, childDataCustody: 'child-activity-data-included' },
      { ...row, claimBoundary: 'provider execution preflight is complete' },
    ]) {
      expect(AppInstallPurchaseProviderStoreExecutionPreflightRowSchema.safeParse(invalidRow).success).toBe(false);
    }

    expect(
      AppInstallPurchaseProviderStoreExecutionPreflightProofSchema.safeParse({
        ...proof,
        nonClaims: proof.nonClaims.filter((claim) => claim !== 'no-provider-api-execution'),
      }).success
    ).toBe(false);
  });
}

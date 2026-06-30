import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseProviderStoreApiExecutionProofReadModel,
  AppInstallPurchaseProviderStoreApiExecutionProofSchema,
  AppInstallPurchaseProviderStoreApiExecutionRowSchema,
  summarizeAppInstallPurchaseProviderStoreApiExecutionProof,
} from '@ocentra-parent/schema-domain/app-install-purchase-provider-store-api-execution-proof';

describe('app install and purchase provider store api execution proof', () => {
  acceptsDeterministicProviderStoreApiExecutionRows();
  rejectsMissingRefsOverclaimsAndMissingNonClaims();
});

function acceptsDeterministicProviderStoreApiExecutionRows(): void {
  it('accepts provider store api execution rows without provider execution or approval claims', () => {
    const proof = AppInstallPurchaseProviderStoreApiExecutionProofSchema.parse(
      AppInstallPurchaseProviderStoreApiExecutionProofReadModel
    );

    expect(summarizeAppInstallPurchaseProviderStoreApiExecutionProof(proof)).toEqual({
      providerStoreApiExecutionRows: 5,
      executionReadyRows: 1,
      manualRequiredRows: 1,
      unavailableRows: 1,
      blockedBeforeClaimRows: 2,
      providerExecutedRows: 0,
      productClaimApprovedRows: 0,
    });
    expect(proof.providerStoreApiExecutionRows.map((row) => `${row.platform}:${row.providerStoreApiExecutionState}`)).toEqual([
      'windows:execution-ready',
      'macos:manual-required',
      'linux:unavailable',
      'android:blocked-before-claim',
      'ios:blocked-before-claim',
    ]);

    for (const row of proof.providerStoreApiExecutionRows) {
      expect(row.providerApiExecutionClaim).toBe('not-executed');
      expect(row.productClaimApprovalClaim).toBe('not-claimed');
      expect(row.storeIntegrationClaim).toBe('not-claimed');
      expect(row.platformInterceptionClaim).toBe('not-claimed');
      expect(row.platformAdapterClaim).toBe('not-implemented');
      expect(row.childDeviceDeliveryClaim).toBe('not-delivered');
      expect(row.runtimeWriterDeliveryClaim).toBe('not-delivered');
      expect(row.runtimeReportDeliveryClaim).toBe('not-delivered');
      expect(row.childDataCustody).toBe('no-child-activity-data');
      expect(row.claimBoundary).toContain('no provider API execution');
      expect(row.claimBoundary).toContain('no child-device delivery');
    }
  });
}

function rejectsMissingRefsOverclaimsAndMissingNonClaims(): void {
  it('rejects provider store api execution rows that omit refs or invent execution, approval, delivery, or custody claims', () => {
    const proof = AppInstallPurchaseProviderStoreApiExecutionProofReadModel;
    const row = proof.providerStoreApiExecutionRows[0];

    expect(
      AppInstallPurchaseProviderStoreApiExecutionProofSchema.safeParse({
        ...proof,
        providerStoreApiExecutionRows: proof.providerStoreApiExecutionRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProviderStoreApiExecutionRowSchema.safeParse({
        ...row,
        sourceProviderStoreRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProviderStoreApiExecutionRowSchema.safeParse({
        ...row,
        sourcePlatformLimitationFallbackRowId: '',
      }).success
    ).toBe(false);

    for (const invalidRow of [
      { ...row, providerApiExecutionClaim: 'executed' },
      { ...row, productClaimApprovalClaim: 'approved' },
      { ...row, storeIntegrationClaim: 'claimed' },
      { ...row, platformInterceptionClaim: 'claimed' },
      { ...row, platformAdapterClaim: 'implemented' },
      { ...row, childDeviceDeliveryClaim: 'delivered' },
      { ...row, runtimeWriterDeliveryClaim: 'delivered' },
      { ...row, runtimeReportDeliveryClaim: 'delivered' },
      { ...row, childDataCustody: 'child-activity-data-included' },
      { ...row, claimBoundary: 'provider api execution is implemented' },
    ]) {
      expect(AppInstallPurchaseProviderStoreApiExecutionRowSchema.safeParse(invalidRow).success).toBe(false);
    }

    expect(
      AppInstallPurchaseProviderStoreApiExecutionProofSchema.safeParse({
        ...proof,
        nonClaims: proof.nonClaims.filter((claim) => claim !== 'no-provider-api-execution'),
      }).success
    ).toBe(false);
  });
}

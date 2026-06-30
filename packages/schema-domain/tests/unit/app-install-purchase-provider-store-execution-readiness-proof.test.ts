import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseProviderStoreExecutionReadinessProofReadModel,
  AppInstallPurchaseProviderStoreExecutionReadinessProofSchema,
  AppInstallPurchaseProviderStoreExecutionReadinessRowSchema,
  summarizeAppInstallPurchaseProviderStoreExecutionReadinessProof,
} from '@ocentra-parent/schema-domain/app-install-purchase-provider-store-execution-readiness-proof';

describe('app install and purchase provider store execution readiness proof', () => {
  acceptsProviderStoreExecutionReadinessRows();
  rejectsMissingRefsOverclaimsAndMissingNonClaims();
});

function acceptsProviderStoreExecutionReadinessRows(): void {
  it('accepts provider store execution readiness rows that stay proof-only and parent-owned', () => {
    const proof = AppInstallPurchaseProviderStoreExecutionReadinessProofSchema.parse(
      AppInstallPurchaseProviderStoreExecutionReadinessProofReadModel
    );

    expect(summarizeAppInstallPurchaseProviderStoreExecutionReadinessProof(proof)).toEqual({
      providerStoreExecutionReadinessRows: 5,
      executionReadyRows: 1,
      manualRequiredRows: 3,
      unavailableRows: 1,
      packageSourceAdapterLinkedRows: 5,
      parentActionReadinessLinkedRows: 5,
      providerExecutedRows: 0,
      childDeliveredRows: 0,
    });
    expect(
      proof.providerStoreExecutionReadinessRows.map(
        (row) =>
          `${row.platform}:${row.storeSurface}:${row.sourceApiEntitlementEvidenceStatus}:${row.sourceStoreStatusHandoffState}:${row.sourcePackageSourceAdapterExecutionState}:${row.providerStoreExecutionReadinessState}`
      )
    ).toEqual([
      'windows:microsoft-store:approved-api-evidence-required:approved-api-status-proof-required:local-adapter-executed:provider-store-execution-ready',
      'macos:mac-app-store:manual-platform-review-required:manual-platform-status-review-required:manual-host-proof-required:manual-required',
      'linux:linux-package-manager:platform-unavailable:platform-store-status-unavailable:platform-unavailable:unavailable',
      'android:google-play:store-entitlement-evidence-required:store-entitlement-status-proof-required:device-management-required:manual-required',
      'ios:apple-app-store:store-entitlement-evidence-required:store-entitlement-status-proof-required:apple-entitlement-required:manual-required',
    ]);

    for (const row of proof.providerStoreExecutionReadinessRows) {
      expect(row.providerApiExecutionClaim).toBe('not-executed');
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
  it('rejects provider store execution readiness rows that omit source refs or invent execution, delivery, adapter, or custody claims', () => {
    const proof = AppInstallPurchaseProviderStoreExecutionReadinessProofReadModel;
    const row = proof.providerStoreExecutionReadinessRows[0];

    expect(
      AppInstallPurchaseProviderStoreExecutionReadinessProofSchema.safeParse({
        ...proof,
        providerStoreExecutionReadinessRows: proof.providerStoreExecutionReadinessRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProviderStoreExecutionReadinessRowSchema.safeParse({
        ...row,
        sourceApprovedApiEntitlementRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProviderStoreExecutionReadinessRowSchema.safeParse({
        ...row,
        sourceParentActionDeliveryReadinessRefs: [],
      }).success
    ).toBe(false);

    for (const invalidRow of [
      { ...row, providerApiExecutionClaim: 'executed' },
      { ...row, storeIntegrationClaim: 'claimed' },
      { ...row, platformInterceptionClaim: 'claimed' },
      { ...row, platformAdapterClaim: 'implemented' },
      { ...row, childDeviceDeliveryClaim: 'delivered' },
      { ...row, runtimeWriterDeliveryClaim: 'delivered' },
      { ...row, runtimeReportDeliveryClaim: 'delivered' },
      { ...row, appBlockingClaim: 'claimed' },
      { ...row, childDataCustody: 'child-activity-data-included' },
      { ...row, claimBoundary: 'provider store execution is implemented' },
    ]) {
      expect(AppInstallPurchaseProviderStoreExecutionReadinessRowSchema.safeParse(invalidRow).success).toBe(false);
    }

    expect(
      AppInstallPurchaseProviderStoreExecutionReadinessProofSchema.safeParse({
        ...proof,
        nonClaims: proof.nonClaims.filter((claim) => claim !== 'no-provider-api-execution'),
      }).success
    ).toBe(false);
  });
}

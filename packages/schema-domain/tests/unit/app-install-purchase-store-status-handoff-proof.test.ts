import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseStoreStatusHandoffProofReadModel,
  AppInstallPurchaseStoreStatusHandoffProofSchema,
  AppInstallPurchaseStoreStatusHandoffRowSchema,
  summarizeAppInstallPurchaseStoreStatusHandoffProof,
} from '@ocentra-parent/schema-domain/app-install-purchase-store-status-handoff-proof';

describe('app install and purchase store status handoff proof', () => {
  acceptsStoreStatusHandoffRows();
  rejectsMissingRefsOverclaimsAndMissingNonClaims();
});

function acceptsStoreStatusHandoffRows(): void {
  it('accepts store status handoff rows that stay proof-only and nondelivered', () => {
    const proof = AppInstallPurchaseStoreStatusHandoffProofSchema.parse(
      AppInstallPurchaseStoreStatusHandoffProofReadModel
    );

    expect(summarizeAppInstallPurchaseStoreStatusHandoffProof(proof)).toEqual({
      storeStatusHandoffRows: 5,
      approvedApiRequiredRows: 1,
      entitlementRequiredRows: 2,
      manualRequiredRows: 1,
      unavailableRows: 1,
      parentActionRuntimeLinkedRows: 5,
      deliveredRows: 0,
    });
    expect(
      proof.storeStatusHandoffRows.map(
        (row) => `${row.platform}:${row.storeSurface}:${row.storeStatusHandoffState}:${row.storeStatusRuntimeState}`
      )
    ).toEqual([
      'windows:microsoft-store:approved-api-status-proof-required:not-implemented',
      'macos:mac-app-store:manual-platform-status-review-required:manual-required',
      'linux:linux-package-manager:platform-store-status-unavailable:unavailable',
      'android:google-play:store-entitlement-status-proof-required:not-implemented',
      'ios:apple-app-store:store-entitlement-status-proof-required:not-implemented',
    ]);

    const windowsRow = proof.storeStatusHandoffRows.find((row) => row.platform === 'windows');
    if (windowsRow === undefined) {
      throw new Error('missing store status handoff row for windows');
    }

    expect(windowsRow.sourceParentActionRuntimeHandoffRefs.length).toBeGreaterThan(0);
    expect(windowsRow.sourceParentActionRuntimeStatuses).toContain('queued-for-runtime-writer');
    expect(windowsRow.sourceParentActionRuntimeStatuses).toContain('manual-review-required');
    expect(windowsRow.storeStatusHandoffEvidenceRefs.length).toBeGreaterThanOrEqual(4);
    expect(windowsRow.sourceReportRuntimeRefs.length).toBeGreaterThan(0);

    for (const row of proof.storeStatusHandoffRows) {
      expect(row.storeStatusHandoffClaim).toBe('status-handoff-proof-only');
      expect(row.statusHandoffDeliveryClaim).toBe('not-delivered');
      expect(row.providerApiExecutionClaim).toBe('not-executed');
      expect(row.storeIntegrationClaim).toBe('not-claimed');
      expect(row.platformAdapterClaim).toBe('not-implemented');
      expect(row.parentActionRuntimeDeliveryClaim).toBe('not-delivered');
      expect(row.childDeliveryClaim).toBe('not-delivered');
      expect(row.runtimeReportDeliveryClaim).toBe('not-delivered');
      expect(row.interceptionClaim).toBe('not-claimed');
      expect(row.appBlockingClaim).toBe('not-claimed');
      expect(row.childDataCustody).toBe('no-child-activity-data');
      expect(row.ocentraHostedFamilyDataCustodyClaim).toBe('not-claimed');
      expect(row.claimBoundary).toContain('no parent action runtime delivery');
      expect(row.claimBoundary).toContain('no real install or purchase interception');
    }
  });
}

function rejectsMissingRefsOverclaimsAndMissingNonClaims(): void {
  it('rejects store status handoff rows that omit adapter/runtime refs or invent provider, delivery, custody, interception, or blocking claims', () => {
    const proof = AppInstallPurchaseStoreStatusHandoffProofReadModel;
    const row = proof.storeStatusHandoffRows[0];

    expect(
      AppInstallPurchaseStoreStatusHandoffProofSchema.safeParse({
        ...proof,
        storeStatusHandoffRows: proof.storeStatusHandoffRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseStoreStatusHandoffRowSchema.safeParse({
        ...row,
        sourcePlatformAdapterBoundaryRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseStoreStatusHandoffRowSchema.safeParse({
        ...row,
        sourceParentActionRuntimeHandoffRefs: [],
      }).success
    ).toBe(false);

    for (const invalidRow of [
      { ...row, storeStatusHandoffClaim: 'claimed' },
      { ...row, statusHandoffDeliveryClaim: 'delivered' },
      { ...row, providerApiExecutionClaim: 'executed' },
      { ...row, storeIntegrationClaim: 'claimed' },
      { ...row, platformAdapterClaim: 'implemented' },
      { ...row, parentActionRuntimeDeliveryClaim: 'delivered' },
      { ...row, childDeliveryClaim: 'delivered' },
      { ...row, runtimeReportDeliveryClaim: 'delivered' },
      { ...row, interceptionClaim: 'claimed' },
      { ...row, appBlockingClaim: 'claimed' },
      { ...row, childDataCustody: 'child-activity-data-included' },
      { ...row, ocentraHostedFamilyDataCustodyClaim: 'claimed' },
      { ...row, claimBoundary: 'store status handoff is implemented' },
    ]) {
      expect(AppInstallPurchaseStoreStatusHandoffRowSchema.safeParse(invalidRow).success).toBe(false);
    }

    expect(
      AppInstallPurchaseStoreStatusHandoffProofSchema.safeParse({
        ...proof,
        nonClaims: proof.nonClaims.filter((claim) => claim !== 'no-provider-api-execution'),
      }).success
    ).toBe(false);
  });
}

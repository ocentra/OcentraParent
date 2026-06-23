import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseStoreStatusHandoffProofReadModel,
  AppInstallPurchaseStoreStatusHandoffProofSchema,
  AppInstallPurchaseStoreStatusHandoffRowSchema,
  summarizeAppInstallPurchaseStoreStatusHandoffProof,
} from '@ocentra-parent/schema-domain/app-install-purchase-store-status-handoff-proof';

describe('app install and purchase store status handoff proof', () => {
  acceptsStoreStatusHandoffRowsWithoutRuntimeClaims();
  rejectsMissingStoreStatusOrParentActionRuntimeCoverage();
  rejectsProviderStoreDeliveryCustodyInterceptionAndBlockingOverclaims();
  rejectsMissingStoreStatusHandoffNonClaims();
});

function acceptsStoreStatusHandoffRowsWithoutRuntimeClaims(): void {
  it('accepts store status handoff rows linked to adapter and parent action runtime evidence without claims', () => {
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
      proof.storeStatusHandoffRows.map((row) => [
        row.platform,
        row.storeSurface,
        row.storeStatusHandoffState,
        row.storeStatusRuntimeState,
      ])
    ).toEqual([
      ['windows', 'microsoft-store', 'approved-api-status-proof-required', 'not-implemented'],
      ['macos', 'mac-app-store', 'manual-platform-status-review-required', 'manual-required'],
      ['linux', 'linux-package-manager', 'platform-store-status-unavailable', 'unavailable'],
      ['android', 'google-play', 'store-entitlement-status-proof-required', 'not-implemented'],
      ['ios', 'apple-app-store', 'store-entitlement-status-proof-required', 'not-implemented'],
    ]);
    for (const row of proof.storeStatusHandoffRows) {
      expect(row.sourceParentActionRuntimeHandoffRefs).toHaveLength(4);
      expect(row.sourceParentActionRuntimeStatuses).toEqual([
        'queued-for-runtime-writer',
        'queued-for-runtime-writer',
        'queued-for-runtime-writer',
        'manual-review-required',
      ]);
      expect(row.storeStatusHandoffEvidenceRefs.length).toBeGreaterThanOrEqual(4);
      expect(row.sourceReportRuntimeRefs).toHaveLength(4);
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
      expect(row.claimBoundary).toContain('no provider API execution');
      expect(row.claimBoundary).toContain('no parent action runtime delivery');
    }
  });
}

function rejectsMissingStoreStatusOrParentActionRuntimeCoverage(): void {
  it('rejects proofs that omit platform status rows, required status states, parent action runtime, or refs', () => {
    const proof = AppInstallPurchaseStoreStatusHandoffProofReadModel;
    const row = proof.storeStatusHandoffRows[0];

    expect(
      AppInstallPurchaseStoreStatusHandoffProofSchema.safeParse({
        ...proof,
        storeStatusHandoffRows: proof.storeStatusHandoffRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseStoreStatusHandoffProofSchema.safeParse({
        ...proof,
        storeStatusHandoffRows: proof.storeStatusHandoffRows.map((entry) =>
          entry.storeStatusHandoffState === 'manual-platform-status-review-required'
            ? {
                ...entry,
                sourceAdapterEvidenceState: 'approved-api-adapter-evidence-required',
                sourceAdapterRuntimeState: 'not-implemented',
                storeStatusHandoffState: 'approved-api-status-proof-required',
                storeStatusRuntimeState: 'not-implemented',
              }
            : entry
        ),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseStoreStatusHandoffRowSchema.safeParse({
        ...row,
        sourceParentActionRuntimeStatuses: row.sourceParentActionRuntimeStatuses.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseStoreStatusHandoffRowSchema.safeParse({
        ...row,
        storeStatusHandoffEvidenceRefs: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseStoreStatusHandoffRowSchema.safeParse({
        ...row,
        sourceReportRuntimeRefs: [],
      }).success
    ).toBe(false);
  });
}

function rejectsProviderStoreDeliveryCustodyInterceptionAndBlockingOverclaims(): void {
  it('rejects rows that claim provider store adapter delivery custody interception or blocking behavior', () => {
    const row = AppInstallPurchaseStoreStatusHandoffProofReadModel.storeStatusHandoffRows[0];

    for (const invalidRow of [
      { ...row, storeStatusHandoffClaim: 'runtime-delivered' },
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
      { ...row, claimBoundary: 'store status delivered through provider and platform adapter' },
    ]) {
      expect(AppInstallPurchaseStoreStatusHandoffRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingStoreStatusHandoffNonClaims(): void {
  it('rejects store status handoff proof when provider delivery custody or blocking non-claims are removed', () => {
    const proof = AppInstallPurchaseStoreStatusHandoffProofReadModel;

    for (const claim of [
      'no-provider-api-execution',
      'no-store-integration',
      'no-platform-adapter-implementation',
      'no-parent-action-runtime-delivery',
      'no-child-device-delivery',
      'no-runtime-report-delivery',
      'no-child-activity-data',
      'no-app-blocking',
      'no-ocentra-hosted-family-data-custody',
    ] as const) {
      expect(
        AppInstallPurchaseStoreStatusHandoffProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

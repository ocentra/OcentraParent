import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchasePlatformProofReadinessProofReadModel,
  AppInstallPurchasePlatformProofReadinessProofSchema,
  AppInstallPurchasePlatformProofReadinessRowSchema,
  summarizeAppInstallPurchasePlatformProofReadiness,
} from '../../src/app-install-purchase-platform-proof-readiness';

describe('app install and purchase platform proof readiness', () => {
  acceptsPlatformProofReadinessRowsWithoutPlatformClaims();
  rejectsMissingPlatformCoverageOrEvidenceRefs();
  rejectsProviderStoreAdapterDeliveryCustodyAndBlockingOverclaims();
  rejectsMissingPlatformProofReadinessNonClaims();
});

function acceptsPlatformProofReadinessRowsWithoutPlatformClaims(): void {
  it('accepts platform rows that name required proof before product claims', () => {
    const proof = AppInstallPurchasePlatformProofReadinessProofSchema.parse(
      AppInstallPurchasePlatformProofReadinessProofReadModel
    );

    expect(summarizeAppInstallPurchasePlatformProofReadiness(proof)).toEqual({
      platformRows: 5,
      manualProofRequiredRows: 2,
      policyBlockedRows: 2,
      unavailableRows: 1,
      providerExecutedRows: 0,
      adapterImplementedRows: 0,
    });
    expect(proof.platformProofReadinessRows.map((row) => `${row.platform}:${row.platformProofReadinessState}`)).toEqual(
      [
        'windows:manual-proof-required',
        'macos:manual-proof-required',
        'linux:unavailable',
        'android:policy-blocked',
        'ios:policy-blocked',
      ]
    );
    for (const row of proof.platformProofReadinessRows) {
      expect(row.sourceLimitationSummaryRowIds).toEqual([
        'app-install-limitation-summary-ready',
        'app-install-limitation-summary-manual-required',
        'app-install-limitation-summary-unavailable',
      ]);
      expect(row.requiredManualEvidenceRefs.length).toBeGreaterThan(0);
      expect(row.providerApiExecutionClaim).toBe('not-executed');
      expect(row.storeIntegrationClaim).toBe('not-claimed');
      expect(row.platformAdapterClaim).toBe('not-implemented');
      expect(row.childDeviceDeliveryClaim).toBe('not-delivered');
      expect(row.runtimeReportDeliveryClaim).toBe('not-delivered');
      expect(row.appBlockingClaim).toBe('not-claimed');
      expect(row.childDataCustody).toBe('no-child-activity-data');
      expect(row.ocentraHostedFamilyDataCustodyClaim).toBe('not-claimed');
    }
  });
}

function rejectsMissingPlatformCoverageOrEvidenceRefs(): void {
  it('rejects proofs that omit a platform row or required manual evidence', () => {
    const proof = AppInstallPurchasePlatformProofReadinessProofReadModel;
    const row = proof.platformProofReadinessRows[0];

    expect(
      AppInstallPurchasePlatformProofReadinessProofSchema.safeParse({
        ...proof,
        platformProofReadinessRows: proof.platformProofReadinessRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchasePlatformProofReadinessRowSchema.safeParse({
        ...row,
        sourceLimitationSummaryRowIds: row.sourceLimitationSummaryRowIds.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchasePlatformProofReadinessRowSchema.safeParse({
        ...row,
        requiredManualEvidenceRefs: [],
      }).success
    ).toBe(false);
  });
}

function rejectsProviderStoreAdapterDeliveryCustodyAndBlockingOverclaims(): void {
  it('rejects rows that claim execution, adapters, delivery, custody, or blocking', () => {
    const row = AppInstallPurchasePlatformProofReadinessProofReadModel.platformProofReadinessRows[0];

    for (const invalidRow of [
      { ...row, providerApiExecutionClaim: 'executed' },
      { ...row, storeIntegrationClaim: 'claimed' },
      { ...row, platformAdapterClaim: 'implemented' },
      { ...row, childDeviceDeliveryClaim: 'delivered' },
      { ...row, runtimeReportDeliveryClaim: 'delivered' },
      { ...row, appBlockingClaim: 'claimed' },
      { ...row, childDataCustody: 'child-activity-data-included' },
      { ...row, ocentraHostedFamilyDataCustodyClaim: 'claimed' },
      { ...row, claimBoundary: 'platform proof implemented provider execution and adapter delivery' },
    ]) {
      expect(AppInstallPurchasePlatformProofReadinessRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingPlatformProofReadinessNonClaims(): void {
  it('rejects proofs that remove a required platform non-claim', () => {
    const proof = AppInstallPurchasePlatformProofReadinessProofReadModel;

    for (const claim of proof.nonClaims) {
      expect(
        AppInstallPurchasePlatformProofReadinessProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

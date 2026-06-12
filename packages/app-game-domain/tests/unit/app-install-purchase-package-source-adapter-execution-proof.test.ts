import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchasePackageSourceAdapterExecutionProofReadModel,
  AppInstallPurchasePackageSourceAdapterExecutionProofSchema,
  AppInstallPurchasePackageSourceAdapterExecutionRowSchema,
  summarizeAppInstallPurchasePackageSourceAdapterExecutionProof,
} from '../../src/app-install-purchase-package-source-adapter-execution-proof';

describe('app install and purchase package-source adapter execution proof', () => {
  acceptsPackageSourceAdapterExecutionRowsWithoutRuntimeDeliveryClaims();
  rejectsMissingPackageSourceAdapterExecutionCoverageOrRefs();
  rejectsProviderStorePortalProductionAdapterDeliveryCustodyInterceptionAndBlockingOverclaims();
  rejectsMissingPackageSourceAdapterExecutionNonClaims();
});

function acceptsPackageSourceAdapterExecutionRowsWithoutRuntimeDeliveryClaims(): void {
  it('accepts package-source adapter execution rows linked to capture status refs', () => {
    const proof = AppInstallPurchasePackageSourceAdapterExecutionProofSchema.parse(
      AppInstallPurchasePackageSourceAdapterExecutionProofReadModel
    );

    expect(summarizeAppInstallPurchasePackageSourceAdapterExecutionProof(proof)).toEqual({
      packageSourceAdapterExecutionRows: 5,
      localAdapterExecutedRows: 1,
      manualHostProofRows: 1,
      blockedRows: 2,
      unavailableRows: 1,
      artifactLinkedRows: 5,
      providerExecutedRows: 0,
      childDeliveredRows: 0,
    });
    expect(
      proof.packageSourceAdapterExecutionRows.map((row) => [
        row.platform,
        row.storeSurface,
        row.adapterKind,
        row.adapterExecutionState,
        row.sourcePackageSourceCaptureStatus,
      ])
    ).toEqual([
      ['windows', 'microsoft-store', 'windows-local-package-source-reader', 'local-adapter-executed', 'captured'],
      ['macos', 'mac-app-store', 'macos-manual-host-proof', 'manual-host-proof-required', 'manual-required'],
      ['linux', 'linux-package-manager', 'linux-package-manager-unavailable', 'platform-unavailable', 'unavailable'],
      ['android', 'google-play', 'android-device-owner-required', 'device-management-required', 'blocked'],
      ['ios', 'apple-app-store', 'ios-family-controls-entitlement-required', 'apple-entitlement-required', 'blocked'],
    ]);
    for (const row of proof.packageSourceAdapterExecutionRows) {
      expect(row.sourcePackageSourceCaptureArtifactRefs.length).toBeGreaterThan(0);
      expect(row.sourcePackageSourceAuditRefs.length).toBeGreaterThan(0);
      expect(row.adapterExecutionAttemptRefs).toHaveLength(1);
      expect(row.adapterExecutionArtifactRefs.length).toBeGreaterThanOrEqual(
        row.sourcePackageSourceCaptureArtifactRefs.length
      );
      expect(row.auditEventRefs).toHaveLength(1);
      expect(row.reportRefs.length).toBeGreaterThan(0);
      expect(row.requiredProofRefs.length).toBeGreaterThan(0);
      expect(row.packageSourceAdapterExecutionClaim).toBe('proof-executed');
      expect(row.providerApiExecutionClaim).toBe('not-executed');
      expect(row.storeIntegrationClaim).toBe('not-claimed');
      expect(row.portalApprovalUiClaim).toBe('not-implemented');
      expect(row.productionPlatformAdapterClaim).toBe('not-implemented');
      expect(row.childDeliveryClaim).toBe('not-delivered');
      expect(row.runtimeReportDeliveryClaim).toBe('not-delivered');
      expect(row.interceptionClaim).toBe('not-claimed');
      expect(row.appBlockingClaim).toBe('not-claimed');
      expect(row.childDataCustody).toBe('no-child-activity-data');
      expect(row.ocentraHostedFamilyDataCustodyClaim).toBe('not-claimed');
      expect(row.claimBoundary).toContain('no production platform adapter');
      expect(row.claimBoundary).toContain('no child-device delivery');
    }
  });
}

function rejectsMissingPackageSourceAdapterExecutionCoverageOrRefs(): void {
  it('rejects proofs that omit adapter states or source capture refs', () => {
    const proof = AppInstallPurchasePackageSourceAdapterExecutionProofReadModel;
    const row = proof.packageSourceAdapterExecutionRows[0];

    expect(
      AppInstallPurchasePackageSourceAdapterExecutionProofSchema.safeParse({
        ...proof,
        packageSourceAdapterExecutionRows: proof.packageSourceAdapterExecutionRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchasePackageSourceAdapterExecutionRowSchema.safeParse({
        ...row,
        sourcePackageSourceCaptureArtifactRefs: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchasePackageSourceAdapterExecutionRowSchema.safeParse({
        ...row,
        sourcePackageSourceAuditRefs: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchasePackageSourceAdapterExecutionRowSchema.safeParse({
        ...row,
        adapterExecutionAttemptRefs: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchasePackageSourceAdapterExecutionRowSchema.safeParse({
        ...row,
        adapterExecutionArtifactRefs: [],
      }).success
    ).toBe(false);
  });
}

function rejectsProviderStorePortalProductionAdapterDeliveryCustodyInterceptionAndBlockingOverclaims(): void {
  it('rejects rows that claim provider store portal production adapter delivery custody interception or blocking behavior', () => {
    const row = AppInstallPurchasePackageSourceAdapterExecutionProofReadModel.packageSourceAdapterExecutionRows[0];

    for (const invalidRow of [
      { ...row, packageSourceAdapterExecutionClaim: 'not-executed' },
      { ...row, providerApiExecutionClaim: 'executed' },
      { ...row, storeIntegrationClaim: 'claimed' },
      { ...row, portalApprovalUiClaim: 'implemented' },
      { ...row, productionPlatformAdapterClaim: 'implemented' },
      { ...row, childDeliveryClaim: 'delivered' },
      { ...row, runtimeReportDeliveryClaim: 'delivered' },
      { ...row, interceptionClaim: 'claimed' },
      { ...row, appBlockingClaim: 'claimed' },
      { ...row, childDataCustody: 'child-activity-data-included' },
      { ...row, ocentraHostedFamilyDataCustodyClaim: 'claimed' },
      { ...row, claimBoundary: 'package source adapter executed provider API and delivered to child device' },
    ]) {
      expect(AppInstallPurchasePackageSourceAdapterExecutionRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingPackageSourceAdapterExecutionNonClaims(): void {
  it('rejects package-source adapter execution proof when required non-claims are removed', () => {
    const proof = AppInstallPurchasePackageSourceAdapterExecutionProofReadModel;

    for (const claim of [
      'no-provider-api-execution',
      'no-store-integration',
      'no-portal-approval-ui',
      'no-production-platform-adapter',
      'no-child-device-delivery',
      'no-runtime-report-delivery',
      'no-real-install-or-purchase-interception',
      'no-child-activity-data',
      'no-app-blocking',
      'no-ocentra-hosted-family-data-custody',
    ] as const) {
      expect(
        AppInstallPurchasePackageSourceAdapterExecutionProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

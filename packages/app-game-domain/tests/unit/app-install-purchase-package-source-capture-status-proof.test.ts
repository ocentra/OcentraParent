import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchasePackageSourceCaptureStatusProofReadModel,
  AppInstallPurchasePackageSourceCaptureStatusProofSchema,
  AppInstallPurchasePackageSourceCaptureStatusRowSchema,
  summarizeAppInstallPurchasePackageSourceCaptureStatusProof,
} from '@ocentra-parent/schema-domain/app-install-purchase-package-source-capture-status-proof';

describe('app install and purchase package-source capture status proof', () => {
  acceptsPackageSourceCaptureStatusRowsWithoutRuntimeClaims();
  rejectsMissingPackageSourceCaptureCoverageOrRefs();
  rejectsProviderStorePortalAdapterDeliveryCustodyInterceptionAndBlockingOverclaims();
  rejectsMissingPackageSourceCaptureNonClaims();
});

function acceptsPackageSourceCaptureStatusRowsWithoutRuntimeClaims(): void {
  it('accepts package-source capture status rows with artifact audit and report refs only', () => {
    const proof = AppInstallPurchasePackageSourceCaptureStatusProofSchema.parse(
      AppInstallPurchasePackageSourceCaptureStatusProofReadModel
    );

    expect(summarizeAppInstallPurchasePackageSourceCaptureStatusProof(proof)).toEqual({
      packageSourceCaptureRows: 5,
      capturedRows: 1,
      blockedRows: 2,
      manualRequiredRows: 1,
      unavailableRows: 1,
      artifactLinkedRows: 5,
      auditLinkedRows: 5,
      reportLinkedRows: 5,
      deliveredRows: 0,
    });
    expect(
      proof.packageSourceCaptureRows.map((row) => [
        row.platform,
        row.storeSurface,
        row.captureRequestState,
        row.packageSourceCaptureStatus,
        row.platformLimitationState,
      ])
    ).toEqual([
      [
        'windows',
        'microsoft-store',
        'accepted-for-local-package-source-proof',
        'captured',
        'local-package-source-readable',
      ],
      ['macos', 'mac-app-store', 'manual-host-proof-required', 'manual-required', 'requires-manual-host-proof'],
      ['linux', 'linux-package-manager', 'platform-unavailable', 'unavailable', 'platform-unavailable'],
      [
        'android',
        'google-play',
        'blocked-by-device-management-policy',
        'blocked',
        'requires-device-owner-or-managed-profile',
      ],
      ['ios', 'apple-app-store', 'blocked-by-apple-entitlement', 'blocked', 'requires-apple-entitlement'],
    ]);
    for (const row of proof.packageSourceCaptureRows) {
      expect(row.packageSourceCaptureArtifactRefs.length).toBeGreaterThan(0);
      expect(row.sourceStoreStatusEvidenceRefs.length).toBeGreaterThan(0);
      expect(row.auditEventRefs).toHaveLength(1);
      expect(row.reportRefs.length).toBeGreaterThan(0);
      expect(row.requiredProofRefs.length).toBeGreaterThan(0);
      expect(row.packageSourceCaptureClaim).toBe('capture-status-proof-only');
      expect(row.packageSourceCaptureExecutionClaim).toBe('not-executed');
      expect(row.providerApiExecutionClaim).toBe('not-executed');
      expect(row.storeIntegrationClaim).toBe('not-claimed');
      expect(row.portalApprovalUiClaim).toBe('not-implemented');
      expect(row.platformAdapterClaim).toBe('not-implemented');
      expect(row.childDeliveryClaim).toBe('not-delivered');
      expect(row.runtimeReportDeliveryClaim).toBe('not-delivered');
      expect(row.interceptionClaim).toBe('not-claimed');
      expect(row.appBlockingClaim).toBe('not-claimed');
      expect(row.childDataCustody).toBe('no-child-activity-data');
      expect(row.ocentraHostedFamilyDataCustodyClaim).toBe('not-claimed');
      expect(row.claimBoundary).toContain('no provider API execution');
      expect(row.claimBoundary).toContain('no portal approval UI');
      expect(row.claimBoundary).toContain('no child-device delivery');
    }
  });
}

function rejectsMissingPackageSourceCaptureCoverageOrRefs(): void {
  it('rejects proofs that omit platform statuses or required source refs', () => {
    const proof = AppInstallPurchasePackageSourceCaptureStatusProofReadModel;
    const row = proof.packageSourceCaptureRows[0];

    expect(
      AppInstallPurchasePackageSourceCaptureStatusProofSchema.safeParse({
        ...proof,
        packageSourceCaptureRows: proof.packageSourceCaptureRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchasePackageSourceCaptureStatusProofSchema.safeParse({
        ...proof,
        packageSourceCaptureRows: proof.packageSourceCaptureRows.map((entry) =>
          entry.packageSourceCaptureStatus === 'manual-required'
            ? {
                ...entry,
                captureRequestState: 'accepted-for-local-package-source-proof',
                packageSourceCaptureStatus: 'captured',
                platformLimitationState: 'local-package-source-readable',
              }
            : entry
        ),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchasePackageSourceCaptureStatusRowSchema.safeParse({
        ...row,
        packageSourceCaptureArtifactRefs: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchasePackageSourceCaptureStatusRowSchema.safeParse({
        ...row,
        sourceStoreStatusEvidenceRefs: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchasePackageSourceCaptureStatusRowSchema.safeParse({
        ...row,
        auditEventRefs: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchasePackageSourceCaptureStatusRowSchema.safeParse({
        ...row,
        reportRefs: [],
      }).success
    ).toBe(false);
  });
}

function rejectsProviderStorePortalAdapterDeliveryCustodyInterceptionAndBlockingOverclaims(): void {
  it('rejects rows that claim provider store portal adapter delivery custody interception or blocking behavior', () => {
    const row = AppInstallPurchasePackageSourceCaptureStatusProofReadModel.packageSourceCaptureRows[0];

    for (const invalidRow of [
      { ...row, packageSourceCaptureClaim: 'runtime-captured' },
      { ...row, packageSourceCaptureExecutionClaim: 'executed' },
      { ...row, providerApiExecutionClaim: 'executed' },
      { ...row, storeIntegrationClaim: 'claimed' },
      { ...row, portalApprovalUiClaim: 'implemented' },
      { ...row, platformAdapterClaim: 'implemented' },
      { ...row, childDeliveryClaim: 'delivered' },
      { ...row, runtimeReportDeliveryClaim: 'delivered' },
      { ...row, interceptionClaim: 'claimed' },
      { ...row, appBlockingClaim: 'claimed' },
      { ...row, childDataCustody: 'child-activity-data-included' },
      { ...row, ocentraHostedFamilyDataCustodyClaim: 'claimed' },
      { ...row, claimBoundary: 'package source captured through store provider adapter and portal approval UI' },
    ]) {
      expect(AppInstallPurchasePackageSourceCaptureStatusRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingPackageSourceCaptureNonClaims(): void {
  it('rejects package-source capture status proof when required non-claims are removed', () => {
    const proof = AppInstallPurchasePackageSourceCaptureStatusProofReadModel;

    for (const claim of [
      'no-provider-api-execution',
      'no-store-integration',
      'no-portal-approval-ui',
      'no-platform-adapter-implementation',
      'no-child-device-delivery',
      'no-runtime-report-delivery',
      'no-child-activity-data',
      'no-app-blocking',
      'no-ocentra-hosted-family-data-custody',
    ] as const) {
      expect(
        AppInstallPurchasePackageSourceCaptureStatusProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

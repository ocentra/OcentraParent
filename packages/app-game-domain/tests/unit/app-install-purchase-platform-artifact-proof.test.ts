import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchasePlatformArtifactProofReadModel,
  AppInstallPurchasePlatformArtifactProofSchema,
  AppInstallPurchasePlatformStoreArtifactRowSchema,
  summarizeAppInstallPurchasePlatformArtifactProof,
} from '../../src/app-install-purchase-platform-artifact-proof';

describe('app install and purchase platform artifact proof', () => {
  acceptsPlatformStoreAndReportArtifactRefsWithoutRuntimeClaims();
  rejectsMissingPlatformOrReportArtifactCoverage();
  rejectsStoreProviderAdapterDeliveryReportAndBlockingOverclaims();
  rejectsMissingArtifactNonClaims();
});

function acceptsPlatformStoreAndReportArtifactRefsWithoutRuntimeClaims(): void {
  it('accepts parent-owned platform and report artifact refs without store provider adapter or delivery claims', () => {
    const proof = AppInstallPurchasePlatformArtifactProofSchema.parse(AppInstallPurchasePlatformArtifactProofReadModel);

    expect(summarizeAppInstallPurchasePlatformArtifactProof(proof)).toEqual({
      platformArtifactRows: 5,
      reportRuntimeEvidenceRows: 4,
      attachedPlatformArtifacts: 5,
      unavailableStoreMetadataRows: 1,
    });
    expect(
      proof.platformStoreArtifacts.map((row) => [
        row.platform,
        row.storeSurface,
        row.artifactKind,
        row.sourceStoreMetadataArtifactState,
      ])
    ).toEqual([
      ['windows', 'microsoft-store', 'platform-store-metadata-artifact', 'requires-platform-artifact'],
      ['macos', 'mac-app-store', 'platform-store-metadata-artifact', 'requires-platform-artifact'],
      ['linux', 'linux-package-manager', 'platform-limitation-report-artifact', 'platform-unavailable'],
      ['android', 'google-play', 'platform-store-metadata-artifact', 'requires-platform-artifact'],
      ['ios', 'apple-app-store', 'platform-store-metadata-artifact', 'requires-platform-artifact'],
    ]);
    expect(proof.reportRuntimeEvidence.map((row) => row.reportSurface)).toEqual([
      'request-audit-history',
      'parent-decision-audit-history',
      'child-facing-state-report',
      'platform-limitation-report',
    ]);
    for (const row of proof.platformStoreArtifacts) {
      expect(row.artifactSourceState).toBe('parent-owned-artifact-attached');
      expect(row.storeIntegrationClaim).toBe('not-claimed');
      expect(row.providerApiClaim).toBe('not-claimed');
      expect(row.platformAdapterClaim).toBe('not-implemented');
      expect(row.childDeliveryClaim).toBe('not-delivered');
      expect(row.runtimeReportDeliveryClaim).toBe('not-delivered');
      expect(row.appBlockingClaim).toBe('not-claimed');
      expect(row.claimBoundary).toContain('no provider API');
    }
    for (const row of proof.reportRuntimeEvidence) {
      expect(row.runtimeReportDeliveryClaim).toBe('not-delivered');
      expect(row.providerApiClaim).toBe('not-claimed');
      expect(row.platformAdapterClaim).toBe('not-implemented');
      expect(row.claimBoundary).toContain('no runtime report delivery');
    }
  });
}

function rejectsMissingPlatformOrReportArtifactCoverage(): void {
  it('rejects proof rows that omit platform artifact coverage or report runtime evidence', () => {
    const proof = AppInstallPurchasePlatformArtifactProofReadModel;

    expect(
      AppInstallPurchasePlatformArtifactProofSchema.safeParse({
        ...proof,
        platformStoreArtifacts: proof.platformStoreArtifacts.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchasePlatformArtifactProofSchema.safeParse({
        ...proof,
        reportRuntimeEvidence: proof.reportRuntimeEvidence.slice(1),
      }).success
    ).toBe(false);
  });
}

function rejectsStoreProviderAdapterDeliveryReportAndBlockingOverclaims(): void {
  it('rejects attached artifact rows that claim provider API adapters child delivery report delivery or app blocking', () => {
    const row = AppInstallPurchasePlatformArtifactProofReadModel.platformStoreArtifacts[0];

    for (const invalidRow of [
      { ...row, providerApiClaim: 'claimed' },
      { ...row, platformAdapterClaim: 'implemented' },
      { ...row, childDeliveryClaim: 'delivered' },
      { ...row, runtimeReportDeliveryClaim: 'delivered' },
      { ...row, appBlockingClaim: 'claimed' },
      { ...row, claimBoundary: 'platform artifact proof only' },
    ]) {
      expect(AppInstallPurchasePlatformStoreArtifactRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingArtifactNonClaims(): void {
  it('rejects platform artifact proof when store provider delivery or blocking non-claims are removed', () => {
    const proof = AppInstallPurchasePlatformArtifactProofReadModel;

    for (const claim of ['no-provider-api', 'no-child-device-delivery', 'not-generic-app-blocking'] as const) {
      expect(
        AppInstallPurchasePlatformArtifactProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

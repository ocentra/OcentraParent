import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseRuntimeProofReadModel,
  AppInstallPurchaseRuntimeProofSchema,
  summarizeAppInstallPurchaseRuntimeProof,
} from '../src/app-install-purchase-runtime-proof';

describe('app install and purchase runtime proof boundary', () => {
  acceptsRuntimeProofBoundaryRows();
  rejectsMissingPlatformArtifactCoverage();
  rejectsChildDeliveryOrReportRuntimeOverclaims();
  rejectsMissingRuntimeNonClaims();
});

function acceptsRuntimeProofBoundaryRows(): void {
  it('accepts platform artifact child delivery and report rows as boundary proof without runtime claims', () => {
    const proof = AppInstallPurchaseRuntimeProofSchema.parse(AppInstallPurchaseRuntimeProofReadModel);

    expect(summarizeAppInstallPurchaseRuntimeProof(proof)).toEqual({
      platformRows: 5,
      childDeliveryRows: 5,
      reportIntegrationRows: 4,
      boundaryOnlyRows: 5,
      unavailablePlatformRows: 1,
    });
    expect(proof.platformRuntimeArtifacts.map((row) => [row.platform, row.storeSurface])).toEqual([
      ['windows', 'microsoft-store'],
      ['macos', 'mac-app-store'],
      ['linux', 'linux-package-manager'],
      ['android', 'google-play'],
      ['ios', 'apple-app-store'],
    ]);
    expect(proof.platformRuntimeArtifacts.map((row) => row.runtimeClaimState)).toEqual([
      'boundary-only',
      'boundary-only',
      'boundary-only',
      'boundary-only',
      'boundary-only',
    ]);
    expect(proof.childDeliveryBoundaries.map((row) => row.runtimeDeliveryClaim)).toEqual([
      'not-delivered',
      'not-delivered',
      'not-delivered',
      'not-delivered',
      'not-delivered',
    ]);
    expect(proof.reportIntegrationBoundaries.map((row) => row.runtimeReportClaim)).toEqual([
      'not-delivered',
      'not-delivered',
      'not-delivered',
      'not-delivered',
    ]);
  });
}

function rejectsMissingPlatformArtifactCoverage(): void {
  it('rejects runtime proof rows that omit a platform or detach package-source artifact references', () => {
    const proof = AppInstallPurchaseRuntimeProofReadModel;
    const androidRow = proof.platformRuntimeArtifacts.find((row) => row.platform === 'android');
    if (androidRow === undefined) {
      throw new Error('missing android runtime artifact row');
    }

    expect(
      AppInstallPurchaseRuntimeProofSchema.safeParse({
        ...proof,
        platformRuntimeArtifacts: proof.platformRuntimeArtifacts.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseRuntimeProofSchema.safeParse({
        ...proof,
        platformRuntimeArtifacts: [
          { ...androidRow, requiredProofRefs: [] },
          ...proof.platformRuntimeArtifacts.filter((row) => row.platform !== 'android'),
        ],
      }).success
    ).toBe(false);
  });
}

function rejectsChildDeliveryOrReportRuntimeOverclaims(): void {
  it('rejects child delivery and report rows that claim runtime delivery or omit boundary refs', () => {
    const proof = AppInstallPurchaseRuntimeProofReadModel;
    const childRow = proof.childDeliveryBoundaries[0];
    const reportRow = proof.reportIntegrationBoundaries[0];

    expect(
      AppInstallPurchaseRuntimeProofSchema.safeParse({
        ...proof,
        childDeliveryBoundaries: [
          { ...childRow, runtimeDeliveryClaim: 'delivered' },
          ...proof.childDeliveryBoundaries.slice(1),
        ],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseRuntimeProofSchema.safeParse({
        ...proof,
        reportIntegrationBoundaries: [{ ...reportRow, reportRefs: [] }, ...proof.reportIntegrationBoundaries.slice(1)],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseRuntimeProofSchema.safeParse({
        ...proof,
        reportIntegrationBoundaries: [
          { ...reportRow, claimBoundary: 'runtime report delivery is implemented' },
          ...proof.reportIntegrationBoundaries.slice(1),
        ],
      }).success
    ).toBe(false);
  });
}

function rejectsMissingRuntimeNonClaims(): void {
  it('rejects runtime proof when child delivery report or store non-claims are removed', () => {
    const proof = AppInstallPurchaseRuntimeProofReadModel;

    for (const claim of ['no-child-device-delivery', 'no-runtime-report-delivery', 'no-store-integration'] as const) {
      expect(
        AppInstallPurchaseRuntimeProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

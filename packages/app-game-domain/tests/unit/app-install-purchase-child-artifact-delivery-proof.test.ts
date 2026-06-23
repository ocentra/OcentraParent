import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseChildArtifactDeliveryProofReadModel,
  AppInstallPurchaseChildArtifactDeliveryProofSchema,
  AppInstallPurchaseChildDeliveryBoundaryRowSchema,
  AppInstallPurchaseChildPackageArtifactRowSchema,
  summarizeAppInstallPurchaseChildArtifactDeliveryProof,
} from '@ocentra-parent/schema-domain/app-install-purchase-child-artifact-delivery-proof';

describe('app install and purchase child artifact delivery proof', () => {
  acceptsChildArtifactAndDeliveryBoundaryRefs();
  rejectsMissingChildArtifactOrDeliveryCoverage();
  rejectsChildArtifactRuntimeCaptureDeliveryAndAdapterOverclaims();
  rejectsMissingChildArtifactNonClaims();
});

function acceptsChildArtifactAndDeliveryBoundaryRefs(): void {
  it('accepts child package artifact and delivery boundary refs without runtime capture or delivery claims', () => {
    const proof = AppInstallPurchaseChildArtifactDeliveryProofSchema.parse(
      AppInstallPurchaseChildArtifactDeliveryProofReadModel
    );

    expect(summarizeAppInstallPurchaseChildArtifactDeliveryProof(proof)).toEqual({
      childArtifactRows: 5,
      childDeliveryRows: 5,
      attachedChildArtifactRefs: 4,
      unavailableChildArtifactRows: 1,
      notDeliveredRows: 5,
    });
    expect(
      proof.childPackageArtifacts.map((row) => [
        row.platform,
        row.storeSurface,
        row.packageSourceArtifactState,
        row.childArtifactSourceState,
        row.deliveryState,
      ])
    ).toEqual([
      [
        'windows',
        'microsoft-store',
        'requires-package-source-artifact',
        'child-package-artifact-ref-attached',
        'manual-required',
      ],
      [
        'macos',
        'mac-app-store',
        'requires-package-source-artifact',
        'child-package-artifact-ref-attached',
        'manual-required',
      ],
      ['linux', 'linux-package-manager', 'platform-unavailable', 'platform-unavailable', 'unavailable'],
      [
        'android',
        'google-play',
        'requires-device-proof-artifact',
        'child-package-artifact-ref-attached',
        'manual-required',
      ],
      [
        'ios',
        'apple-app-store',
        'requires-device-proof-artifact',
        'child-package-artifact-ref-attached',
        'manual-required',
      ],
    ]);
    expect(proof.childDeliveryBoundaries.map((row) => row.childVisibleStatus)).toEqual([
      'pending-parent-review-visible',
      'approved-visible',
      'denied-visible',
      'time-box-visible',
      'review-needed-visible',
    ]);
    for (const row of proof.childPackageArtifacts) {
      expect(row.childArtifactCaptureClaim).toBe('not-runtime-captured');
      expect(row.childDeliveryClaim).toBe('not-delivered');
      expect(row.providerApiClaim).toBe('not-claimed');
      expect(row.platformAdapterClaim).toBe('not-implemented');
      expect(row.interceptionClaim).toBe('not-claimed');
      expect(row.childDataCustody).toBe('no-child-activity-data');
      expect(row.claimBoundary).toContain('no child-device runtime capture');
    }
    for (const row of proof.childDeliveryBoundaries) {
      expect(row.childDeliveryClaim).toBe('not-delivered');
      expect(row.runtimeReportDeliveryClaim).toBe('not-delivered');
      expect(row.appBlockingClaim).toBe('not-claimed');
      expect(row.claimBoundary).toContain('no child-device delivery');
    }
  });
}

function rejectsMissingChildArtifactOrDeliveryCoverage(): void {
  it('rejects proof rows that omit child artifact or delivery boundary coverage', () => {
    const proof = AppInstallPurchaseChildArtifactDeliveryProofReadModel;

    expect(
      AppInstallPurchaseChildArtifactDeliveryProofSchema.safeParse({
        ...proof,
        childPackageArtifacts: proof.childPackageArtifacts.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseChildArtifactDeliveryProofSchema.safeParse({
        ...proof,
        childDeliveryBoundaries: proof.childDeliveryBoundaries.slice(1),
      }).success
    ).toBe(false);
  });
}

function rejectsChildArtifactRuntimeCaptureDeliveryAndAdapterOverclaims(): void {
  it('rejects child artifact rows and delivery rows that claim capture delivery adapter provider or blocking behavior', () => {
    const artifactRow = AppInstallPurchaseChildArtifactDeliveryProofReadModel.childPackageArtifacts[0];
    const deliveryRow = AppInstallPurchaseChildArtifactDeliveryProofReadModel.childDeliveryBoundaries[0];

    for (const invalidRow of [
      { ...artifactRow, childArtifactCaptureClaim: 'runtime-captured' },
      { ...artifactRow, childDeliveryClaim: 'delivered' },
      { ...artifactRow, providerApiClaim: 'claimed' },
      { ...artifactRow, platformAdapterClaim: 'implemented' },
      { ...artifactRow, childDataCustody: 'child-activity-data-included' },
      { ...artifactRow, claimBoundary: 'child artifact is delivered' },
    ]) {
      expect(AppInstallPurchaseChildPackageArtifactRowSchema.safeParse(invalidRow).success).toBe(false);
    }
    for (const invalidRow of [
      { ...deliveryRow, childDeliveryClaim: 'delivered' },
      { ...deliveryRow, runtimeReportDeliveryClaim: 'delivered' },
      { ...deliveryRow, appBlockingClaim: 'claimed' },
      { ...deliveryRow, claimBoundary: 'child delivery is implemented' },
    ]) {
      expect(AppInstallPurchaseChildDeliveryBoundaryRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingChildArtifactNonClaims(): void {
  it('rejects child artifact proof when capture delivery or child-data non-claims are removed', () => {
    const proof = AppInstallPurchaseChildArtifactDeliveryProofReadModel;

    for (const claim of [
      'no-child-device-runtime-capture',
      'no-child-device-delivery',
      'no-child-activity-data',
      'not-generic-app-blocking',
    ] as const) {
      expect(
        AppInstallPurchaseChildArtifactDeliveryProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

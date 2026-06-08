import { describe, expect, it } from 'vitest';
import {
  RequiredTrackingFullProductUiLocalRuntimeArtifactCaptures,
  TrackingFullProductUiLocalRuntimeArtifactCaptureRowSchema,
  buildTrackingFullProductUiLocalRuntimeArtifactCaptureProof,
} from '../src/tracking-full-product-ui-local-runtime-artifact-capture-proof';

const generatedAt = '2026-06-08T04:35:00.000Z';

describe('tracking full product UI local runtime artifact capture proof', () => {
  it('captures only locally provable parent UI artifacts without product claims', () => {
    const proof = buildTrackingFullProductUiLocalRuntimeArtifactCaptureProof(
      generatedAt,
      ['test-results/tracking-hosted-ui-artifact-inventory-proof/proof.json'],
      [
        capture('parent-overview-runtime-ui', '01-parent-overview-runtime.png', 2048, 1200, 900),
        capture('parent-device-detail-runtime-ui', '02-parent-device-detail-runtime.png', 4096, 1200, 900),
        capture('cross-surface-accessibility-report', '08-cross-surface-accessibility-report.json', 1024),
      ]
    );

    expect(proof.rows).toHaveLength(RequiredTrackingFullProductUiLocalRuntimeArtifactCaptures.length);
    expect(proof.localArtifactCount).toBe(3);
    expect(proof.rows.map((row) => row.status)).toEqual([
      'local-artifact-captured',
      'local-artifact-captured',
      'local-artifact-captured',
    ]);
    expect(proof.productClaims.parentOverviewLocalArtifactCaptured).toBe(true);
    expect(proof.productClaims.parentDeviceDetailLocalArtifactCaptured).toBe(true);
    expect(proof.productClaims.crossSurfaceAccessibilityLocalArtifactCaptured).toBe(true);
    expect(proof.productClaims.fullProductUiRuntimeClaimed).toBe(false);
    expect(proof.productClaims.childDeviceRuntimeClaimed).toBe(false);
    expect(proof.productClaims.productClaimReady).toBe(false);
  });

  it('rejects copied screenshot rows when byte sizes drift', () => {
    const invalid = TrackingFullProductUiLocalRuntimeArtifactCaptureRowSchema.safeParse({
      schemaVersion: 'v0.6-parent',
      artifactId: 'parent-overview-runtime-ui',
      status: 'local-artifact-captured',
      generatedAt,
      sourceArtifactRef:
        'output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-parent-overview-shell.png',
      outputArtifactRef: 'output/tracking-plan-proof/product-parent-child-ui-runtime/01-parent-overview-runtime.png',
      sourceBytes: 2048,
      outputBytes: 2047,
      width: 1200,
      height: 900,
      currentProofTier: 'P2_HOSTED_CI',
      requiredProofTier: 'P4_PHYSICAL_DEVICE',
      localParentUiArtifactCaptured: true,
      fullProductUiRuntimeClaimed: false,
      childDeviceRuntimeClaimed: false,
      physicalDeviceProofClaimed: false,
      authorityProofClaimed: false,
      providerDeliveryRuntimeClaimed: false,
      productionProductUiClaimed: false,
      productClaimReady: false,
    });

    expect(invalid.success).toBe(false);
  });
});

function capture(
  artifactId: 'parent-overview-runtime-ui' | 'parent-device-detail-runtime-ui' | 'cross-surface-accessibility-report',
  fileName: string,
  bytes: number,
  width?: number,
  height?: number
) {
  return {
    artifactId,
    sourceArtifactRef:
      artifactId === 'cross-surface-accessibility-report'
        ? 'test-results/tracking-plan-hosted-ui-proof/accessibility-summary.json'
        : `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-${fileName}`,
    outputArtifactRef: `output/tracking-plan-proof/product-parent-child-ui-runtime/${fileName}`,
    sourceBytes: bytes,
    outputBytes: bytes,
    width,
    height,
  };
}

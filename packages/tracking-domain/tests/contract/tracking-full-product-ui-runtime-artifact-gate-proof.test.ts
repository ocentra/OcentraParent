import { describe, expect, it } from 'vitest';
import {
  RequiredTrackingFullProductUiRuntimeArtifactPlan,
  TrackingFullProductUiRuntimeArtifactGateRowSchema,
  buildTrackingFullProductUiRuntimeArtifactGateProof,
} from '../../src/tracking-full-product-ui-runtime-artifact-gate-proof';

describe('tracking full product UI runtime artifact gate proof', () => {
  it('keeps full product UI manual-required when runtime artifacts are missing', () => {
    const proof = buildTrackingFullProductUiRuntimeArtifactGateProof('2026-06-07T23:10:00.000Z', {
      presentArtifacts: [],
    });

    expect(proof.rows).toHaveLength(1);
    expect(proof.rows[0].status).toBe('manual-required');
    expect(proof.rows[0].requiredProofTier).toBe('P4_PHYSICAL_DEVICE');
    expect(proof.rows[0].currentProofTier).toBe('P2_HOSTED_CI');
    expect(proof.rows[0].fullProductUiArtifactSetComplete).toBe(false);
    expect(proof.rows[0].missingArtifacts).toEqual([
      ...RequiredTrackingFullProductUiRuntimeArtifactPlan.requiredArtifacts,
    ]);
    expect(proof.productClaims.parentOverviewRuntimeUiClaimed).toBe(false);
    expect(proof.productClaims.renderedChildDeviceRuntimeUiClaimed).toBe(false);
    expect(proof.productClaims.productClaimReady).toBe(false);
  });

  it('marks artifact-set-present only when every required full product UI artifact exists', () => {
    const proof = buildTrackingFullProductUiRuntimeArtifactGateProof('2026-06-07T23:10:00.000Z', {
      presentArtifacts: RequiredTrackingFullProductUiRuntimeArtifactPlan.requiredArtifacts,
    });

    expect(proof.rows[0].status).toBe('artifact-set-present');
    expect(proof.rows[0].fullProductUiArtifactSetComplete).toBe(true);
    expect(proof.rows[0].missingArtifacts).toEqual([]);
    expect(proof.productClaims.crossSurfaceAccessibilityRuntimeClaimed).toBe(false);
    expect(proof.productClaims.productionProductUiClaimed).toBe(false);
    expect(proof.productClaims.productClaimReady).toBe(false);
  });

  it('rejects rows that claim full product UI runtime without product proof', () => {
    const invalid = TrackingFullProductUiRuntimeArtifactGateRowSchema.safeParse({
      schemaVersion: 'v0.5-tracking',
      rowId: 'tracking-full-product-ui-runtime-artifacts-invalid',
      generatedAt: '2026-06-07T23:10:00.000Z',
      proofRoot: RequiredTrackingFullProductUiRuntimeArtifactPlan.proofRoot,
      requiredProofTier: 'P4_PHYSICAL_DEVICE',
      currentProofTier: 'P2_HOSTED_CI',
      status: 'manual-required',
      requiredArtifacts: ['output/tracking-plan-proof/product-parent-child-ui-runtime/01-parent-overview-runtime.png'],
      presentArtifacts: [],
      missingArtifacts: ['output/tracking-plan-proof/product-parent-child-ui-runtime/01-parent-overview-runtime.png'],
      auditRefs: ['tracking-full-product-ui-runtime-artifacts-invalid-audit'],
      fullProductUiArtifactSetComplete: false,
      parentOverviewRuntimeUiClaimed: true,
      parentDeviceDetailRuntimeUiClaimed: false,
      parentNotificationHistoryPreferencesRuntimeClaimed: false,
      retentionSettingsProductionRuntimeUiClaimed: false,
      renderedChildDeviceRuntimeUiClaimed: false,
      childDeviceSafeHelpRuntimeUiClaimed: false,
      crossSurfaceAccessibilityRuntimeClaimed: false,
      productUiEndToEndRuntimeClaimed: false,
      childDeviceDeliveryRuntimeClaimed: false,
      physicalDeviceProofClaimed: false,
      authorityProofClaimed: false,
      providerDeliveryRuntimeClaimed: false,
      productionProductUiClaimed: false,
      productClaimReady: false,
    });

    expect(invalid.success).toBe(false);
  });
});

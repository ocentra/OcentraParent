import { describe, expect, it } from 'vitest';
import {
  RequiredTrackingChildRuntimeArtifactPlan,
  TrackingChildRuntimeArtifactGateRowSchema,
  buildTrackingChildRuntimeArtifactGateProof,
} from '../src/tracking-child-runtime-artifact-gate-proof';

describe('tracking child runtime artifact gate proof', () => {
  it('keeps child-device runtime manual-required when artifacts are missing', () => {
    const proof = buildTrackingChildRuntimeArtifactGateProof('2026-06-07T18:55:00.000Z', {
      presentArtifacts: [],
    });

    expect(proof.rows).toHaveLength(1);
    expect(proof.rows[0].status).toBe('manual-required');
    expect(proof.rows[0].requiredProofTier).toBe('P4_PHYSICAL_DEVICE');
    expect(proof.rows[0].currentProofTier).toBe('P3_LOCAL_DEV_MACHINE');
    expect(proof.rows[0].childRuntimeArtifactSetComplete).toBe(false);
    expect(proof.rows[0].missingArtifacts).toEqual([...RequiredTrackingChildRuntimeArtifactPlan.requiredArtifacts]);
    expect(proof.productClaims.productClaimReady).toBe(false);
    expect(proof.productClaims.childDeviceDeliveryRuntimeClaimed).toBe(false);
    expect(proof.productClaims.childDeviceExecutionRuntimeClaimed).toBe(false);
  });

  it('marks artifact-set-present only when every required artifact is present', () => {
    const proof = buildTrackingChildRuntimeArtifactGateProof('2026-06-07T18:55:00.000Z', {
      presentArtifacts: RequiredTrackingChildRuntimeArtifactPlan.requiredArtifacts,
    });

    expect(proof.rows[0].status).toBe('artifact-set-present');
    expect(proof.rows[0].childRuntimeArtifactSetComplete).toBe(true);
    expect(proof.rows[0].missingArtifacts).toEqual([]);
    expect(proof.productClaims.productClaimReady).toBe(false);
    expect(proof.productClaims.renderedChildDeviceUiRuntimeClaimed).toBe(false);
  });

  it('rejects rows that claim runtime delivery without the artifact set', () => {
    const invalid = TrackingChildRuntimeArtifactGateRowSchema.safeParse({
      schemaVersion: 'v0.5-tracking',
      rowId: 'tracking-child-runtime-artifacts-invalid',
      generatedAt: '2026-06-07T18:55:00.000Z',
      proofRoot: RequiredTrackingChildRuntimeArtifactPlan.proofRoot,
      requiredProofTier: 'P4_PHYSICAL_DEVICE',
      currentProofTier: 'P3_LOCAL_DEV_MACHINE',
      status: 'manual-required',
      requiredArtifacts: ['00-run-metadata.json'],
      presentArtifacts: [],
      missingArtifacts: ['00-run-metadata.json'],
      auditRefs: ['tracking-child-runtime-artifacts-invalid-audit'],
      childRuntimeArtifactSetComplete: false,
      childDeviceDeliveryRuntimeClaimed: true,
      childDeviceExecutionRuntimeClaimed: false,
      renderedChildDeviceUiRuntimeClaimed: false,
      parentReceiptRuntimeClaimed: false,
      runtimeObservationClaimed: false,
      physicalDeviceProofClaimed: false,
      authorityProofClaimed: false,
      providerDeliveryClaimed: false,
      productionWorkerClaimed: false,
      productClaimReady: false,
    });

    expect(invalid.success).toBe(false);
  });
});

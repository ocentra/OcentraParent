import { describe, expect, it } from 'vitest';
import {
  RequiredTrackingPhysicalDeviceArtifactPlans,
  TrackingPhysicalDeviceArtifactGateRowSchema,
  buildTrackingPhysicalDeviceArtifactGateProof,
} from '../src/tracking-physical-device-artifact-gate-proof';

describe('tracking physical device artifact gate proof', () => {
  it('keeps Android and iOS manual-required when physical artifacts are missing', () => {
    const proof = buildTrackingPhysicalDeviceArtifactGateProof('2026-06-07T18:20:00.000Z', []);

    expect(proof.rows).toHaveLength(2);
    expect(proof.rows.map((row) => row.platform)).toEqual(['android', 'ios']);
    expect(proof.rows.every((row) => row.status === 'manual-required')).toBe(true);
    expect(proof.rows.every((row) => row.requiredProofTier === 'P4_PHYSICAL_DEVICE')).toBe(true);
    expect(proof.rows.every((row) => row.currentProofTier === 'P3_LOCAL_DEV_MACHINE')).toBe(true);
    expect(proof.rows.every((row) => row.physicalArtifactSetComplete === false)).toBe(true);
    expect(proof.productClaims.productClaimReady).toBe(false);
    expect(proof.productClaims.physicalDeviceBehaviorClaimed).toBe(false);
  });

  it('marks artifact-set-present only when every required artifact is present', () => {
    const inventories = RequiredTrackingPhysicalDeviceArtifactPlans.map((plan) => ({
      platform: plan.platform,
      presentArtifacts: plan.requiredArtifacts,
    }));
    const proof = buildTrackingPhysicalDeviceArtifactGateProof('2026-06-07T18:20:00.000Z', inventories);

    expect(proof.rows.every((row) => row.status === 'artifact-set-present')).toBe(true);
    expect(proof.rows.every((row) => row.physicalArtifactSetComplete)).toBe(true);
    expect(proof.rows.every((row) => row.missingArtifacts.length === 0)).toBe(true);
    expect(proof.productClaims.productClaimReady).toBe(false);
    expect(proof.productClaims.physicalDeviceBehaviorClaimed).toBe(false);
  });

  it('rejects rows that claim physical behavior without the artifact set', () => {
    const invalid = TrackingPhysicalDeviceArtifactGateRowSchema.safeParse({
      schemaVersion: 'v0.5-tracking',
      rowId: 'tracking-physical-device-artifacts-invalid',
      generatedAt: '2026-06-07T18:20:00.000Z',
      platform: 'android',
      proofRoot: 'output/tracking-plan-proof/android-background-geofence',
      requiredProofTier: 'P4_PHYSICAL_DEVICE',
      currentProofTier: 'P3_LOCAL_DEV_MACHINE',
      status: 'manual-required',
      requiredArtifacts: ['00-run-metadata.json'],
      presentArtifacts: [],
      missingArtifacts: ['00-run-metadata.json'],
      auditRefs: ['tracking-physical-device-artifacts-invalid-audit'],
      physicalArtifactSetComplete: false,
      physicalDeviceBehaviorClaimed: true,
      authorityEnrollmentClaimed: false,
      providerDeliveryClaimed: false,
      productionWorkerClaimed: false,
      productClaimReady: false,
    });

    expect(invalid.success).toBe(false);
  });
});

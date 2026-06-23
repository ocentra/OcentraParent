import { describe, expect, it } from 'vitest';
import {
  RequiredTrackingPhysicalDeviceArtifactPlans,
  TrackingPhysicalDeviceArtifactGateRowSchema,
  buildTrackingPhysicalDeviceArtifactGateProof,
} from '@ocentra-parent/schema-domain/tracking-physical-device-artifact-gate-proof';

describe('tracking physical device artifact gate proof', () => {
  it('keeps Android and iOS manual-required when physical artifacts are missing', () => {
    const proof = buildTrackingPhysicalDeviceArtifactGateProof('2026-06-07T18:20:00.000Z', []);

    expect(proof.rows).toHaveLength(2);
    expect(proof.rows.map((row) => row.platform)).toEqual(['android', 'ios']);
    expect(proof.rows.every((row) => row.status === 'manual-required')).toBe(true);
    expect(proof.rows.every((row) => row.requiredProofTier === 'P4_PHYSICAL_DEVICE')).toBe(true);
    expect(proof.rows.every((row) => row.currentProofTier === 'P3_LOCAL_DEV_MACHINE')).toBe(true);
    expect(proof.rows.every((row) => row.physicalArtifactSetComplete === false)).toBe(true);
    expect(proof.rows.every((row) => row.physicalDeviceStatusObserved === false)).toBe(true);
    expect(proof.rows.every((row) => row.supportingStatusArtifacts.length === 0)).toBe(true);
    expect(proof.rows.every((row) => row.acceptanceCriteria.length >= 4)).toBe(true);
    expect(proof.rows.every((row) => row.manualValidationCommands.length >= 4)).toBe(true);
    expect(proof.rows.every((row) => row.artifactAcceptanceNotes.length >= 4)).toBe(true);
    expect(proof.rows[0]?.manualValidationCommands).toContain('adb devices -l');
    expect(proof.rows[1]?.manualValidationCommands).toContain('xcrun xctrace list devices');
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
    const invalid = TrackingPhysicalDeviceArtifactGateRowSchema.safeParse(
      invalidPhysicalGateRow({ physicalDeviceBehaviorClaimed: true })
    );

    expect(invalid.success).toBe(false);
  });

  it('rejects physical rows without enough manual acceptance metadata', () => {
    const invalid = TrackingPhysicalDeviceArtifactGateRowSchema.safeParse(
      invalidPhysicalGateRow({
        acceptanceCriteria: ['Record a real device run.'],
        manualValidationCommands: ['adb devices -l'],
        artifactAcceptanceNotes: ['Required artifacts: 1.'],
      })
    );

    expect(invalid.success).toBe(false);
  });
});

function invalidPhysicalGateRow(overrides = {}) {
  return {
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
    supportingStatusProofRef: 'output/tracking-plan-proof/android-background-geofence/status-support-not-collected',
    supportingStatusArtifacts: [],
    auditRefs: ['tracking-physical-device-artifacts-invalid-audit'],
    acceptanceCriteria: [
      'Record a real device run.',
      'Capture permission state.',
      'Capture delivery evidence.',
      'Keep product-ready false.',
    ],
    manualValidationCommands: [
      'adb devices -l',
      'adb shell dumpsys package com.ocentra.parent.child',
      'adb logcat -d',
      'node scripts/test/tracking-physical-device-artifact-gate-proof.mjs',
    ],
    artifactAcceptanceNotes: [
      'Required artifacts: 1.',
      'Presence is not behavior approval.',
      'Reviewer acceptance remains required.',
      'Product claims stay false.',
    ],
    physicalArtifactSetComplete: false,
    physicalDeviceStatusObserved: false,
    physicalDeviceBehaviorClaimed: false,
    authorityEnrollmentClaimed: false,
    providerDeliveryClaimed: false,
    productionWorkerClaimed: false,
    productClaimReady: false,
    ...overrides,
  };
}

import { describe, expect, it } from 'vitest';
import {
  RequiredTrackingPhysicalDeviceArtifactPlans,
  buildTrackingPhysicalDeviceArtifactGateProof,
} from '@ocentra-parent/schema-domain/tracking-physical-device-artifact-gate-proof';
import {
  TrackingPhysicalDeviceEvidenceReviewRowSchema,
  buildTrackingPhysicalDeviceEvidenceReviewProof,
} from '@ocentra-parent/schema-domain/tracking-physical-device-evidence-review-proof';

describe('tracking physical device evidence review proof', () => {
  it('keeps Android and iOS artifact-missing when the physical artifact gate is incomplete', () => {
    const gateProof = buildTrackingPhysicalDeviceArtifactGateProof('2026-06-08T14:20:00.000Z', []);
    const proof = buildTrackingPhysicalDeviceEvidenceReviewProof('2026-06-08T14:25:00.000Z', gateProof);

    expect(proof.rows).toHaveLength(2);
    expect(proof.rows.map((row) => row.platform)).toEqual(['android', 'ios']);
    expect(proof.rows.every((row) => row.status === 'artifact-missing')).toBe(true);
    expect(proof.rows.every((row) => row.reviewerRequired)).toBe(true);
    expect(proof.rows.every((row) => row.contentAccepted === false)).toBe(true);
    expect(proof.rows.every((row) => row.physicalDeviceStatusObserved === false)).toBe(true);
    expect(proof.summary.artifactMissingRows).toBe(2);
    expect(proof.summary.contentReviewRequiredRows).toBe(0);
    expect(proof.summary.contentAcceptedRows).toBe(0);
    expect(proof.summary.physicalDeviceStatusObservedRows).toBe(0);
    expect(proof.summary.supportingStatusArtifactCount).toBe(0);
    expect(proof.productClaims.productClaimReady).toBe(false);
    expect(proof.productClaims.physicalDeviceBehaviorClaimed).toBe(false);
  });

  it('requires content review even when every physical artifact file exists', () => {
    const inventories = RequiredTrackingPhysicalDeviceArtifactPlans.map((plan) => ({
      platform: plan.platform,
      presentArtifacts: plan.requiredArtifacts,
    }));
    const gateProof = buildTrackingPhysicalDeviceArtifactGateProof('2026-06-08T14:20:00.000Z', inventories);
    const proof = buildTrackingPhysicalDeviceEvidenceReviewProof('2026-06-08T14:25:00.000Z', gateProof);

    expect(proof.rows.every((row) => row.status === 'content-review-required')).toBe(true);
    expect(proof.rows.every((row) => row.artifactSetComplete)).toBe(true);
    expect(proof.rows.every((row) => row.missingArtifacts.length === 0)).toBe(true);
    expect(proof.rows.every((row) => row.contentAccepted === false)).toBe(true);
    expect(proof.summary.artifactMissingRows).toBe(0);
    expect(proof.summary.contentReviewRequiredRows).toBe(2);
    expect(proof.summary.contentAcceptedRows).toBe(0);
    expect(proof.productClaims.productClaimReady).toBe(false);
    expect(proof.productClaims.physicalDeviceBehaviorClaimed).toBe(false);
  });

  it('preserves artifact gate acceptance metadata for reviewer handoff', () => {
    const gateProof = buildTrackingPhysicalDeviceArtifactGateProof('2026-06-08T14:20:00.000Z', []);
    const proof = buildTrackingPhysicalDeviceEvidenceReviewProof('2026-06-08T14:25:00.000Z', gateProof);

    expect(proof.summary.acceptanceCriteriaCount).toBe(8);
    expect(proof.summary.manualValidationCommandCount).toBe(8);
    expect(proof.summary.artifactAcceptanceNoteCount).toBe(8);
    expect(proof.rows[0]?.manualValidationCommands).toContain('adb devices -l');
    expect(proof.rows[1]?.manualValidationCommands).toContain('xcrun xctrace list devices');
  });
});

describe('tracking physical device evidence review overclaim rejection', () => {
  it('rejects rows that mark complete artifacts as accepted content', () => {
    const invalid = TrackingPhysicalDeviceEvidenceReviewRowSchema.safeParse({
      schemaVersion: 'v0.5-tracking',
      rowId: 'tracking-physical-device-evidence-review-invalid',
      generatedAt: '2026-06-08T14:25:00.000Z',
      platform: 'android',
      sourceArtifactGateRowId: 'tracking-physical-device-artifacts-android',
      proofRoot: 'output/tracking-plan-proof/android-background-geofence',
      status: 'content-review-required',
      requiredProofTier: 'P4_PHYSICAL_DEVICE_CONTENT_REVIEW',
      currentProofTier: 'P3_LOCAL_DEV_MACHINE',
      requiredArtifacts: ['00-run-metadata.json'],
      presentArtifacts: ['00-run-metadata.json'],
      missingArtifacts: [],
      supportingStatusProofRef: 'test-results/tracking-android-physical-device-runtime-proof/proof.json',
      supportingStatusArtifacts: ['test-results/tracking-android-physical-device-runtime-proof/00-device.json'],
      acceptanceCriteria: [
        'Record a real child Android device run.',
        'Capture permission state.',
        'Show delivery evidence.',
        'Keep product-ready false.',
      ],
      manualValidationCommands: [
        'adb devices -l',
        'adb shell dumpsys package com.ocentra.parent.child',
        'adb logcat -d',
        'node scripts/test/tracking-physical-device-evidence-review-proof.mjs',
      ],
      artifactAcceptanceNotes: [
        'Required artifacts: 1.',
        'Presence is not behavior approval.',
        'Reviewer acceptance remains required.',
        'Product claims stay false.',
      ],
      artifactSetComplete: true,
      physicalDeviceStatusObserved: true,
      reviewerRequired: true,
      contentAccepted: true,
      physicalDeviceBehaviorClaimed: false,
      authorityEnrollmentClaimed: false,
      providerDeliveryClaimed: false,
      productionWorkerClaimed: false,
      productClaimReady: false,
    });

    expect(invalid.success).toBe(false);
  });
});

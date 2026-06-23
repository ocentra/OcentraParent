import { describe, expect, it } from 'vitest';
import { buildTrackingPhysicalDeviceArtifactGateProof } from '@ocentra-parent/schema-domain/tracking-physical-device-artifact-gate-proof';
import { buildTrackingPhysicalDeviceEvidenceReviewProof } from '@ocentra-parent/schema-domain/tracking-physical-device-evidence-review-proof';

const AndroidStatusProofRef = 'test-results/tracking-android-physical-device-runtime-proof/proof.json';
const AndroidStatusArtifacts = [
  'test-results/tracking-android-physical-device-runtime-proof/00-device.json',
  'test-results/tracking-android-physical-device-runtime-proof/07-battery.txt',
];

describe('tracking physical device evidence review status support', () => {
  it('preserves Android status support while keeping behavior artifacts review-blocked', () => {
    const gateProof = buildTrackingPhysicalDeviceArtifactGateProof('2026-06-08T14:20:00.000Z', [
      {
        platform: 'android',
        presentArtifacts: [],
        supportingStatusProofRef: AndroidStatusProofRef,
        supportingStatusArtifacts: AndroidStatusArtifacts,
      },
    ]);
    const proof = buildTrackingPhysicalDeviceEvidenceReviewProof('2026-06-08T14:25:00.000Z', gateProof);
    const android = proof.rows[0];

    expect(android?.platform).toBe('android');
    expect(android?.status).toBe('artifact-missing');
    expect(android?.artifactSetComplete).toBe(false);
    expect(android?.physicalDeviceStatusObserved).toBe(true);
    expect(android?.supportingStatusProofRef).toBe(AndroidStatusProofRef);
    expect(android?.supportingStatusArtifacts).toEqual(AndroidStatusArtifacts);
    expect(android?.contentAccepted).toBe(false);
    expect(android?.physicalDeviceBehaviorClaimed).toBe(false);
    expect(proof.summary.artifactMissingRows).toBe(2);
    expect(proof.summary.physicalDeviceStatusObservedRows).toBe(1);
    expect(proof.summary.supportingStatusArtifactCount).toBe(2);
    expect(proof.productClaims.productClaimReady).toBe(false);
  });
});

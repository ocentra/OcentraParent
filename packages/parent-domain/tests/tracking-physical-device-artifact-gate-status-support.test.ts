import { describe, expect, it } from 'vitest';
import {
  RequiredTrackingPhysicalDeviceArtifactPlans,
  buildTrackingPhysicalDeviceArtifactGateProof,
} from '../src/tracking-physical-device-artifact-gate-proof';

const AndroidStatusProofRef = 'test-results/tracking-android-physical-device-runtime-proof/proof.json';
const AndroidStatusArtifacts = [
  'test-results/tracking-android-physical-device-runtime-proof/00-device.json',
  'test-results/tracking-android-physical-device-runtime-proof/07-battery.txt',
];

describe('tracking physical device artifact gate status support', () => {
  it('records Android physical status support without completing behavior artifacts', () => {
    const proof = buildTrackingPhysicalDeviceArtifactGateProof('2026-06-07T18:20:00.000Z', [
      {
        platform: 'android',
        presentArtifacts: [],
        supportingStatusProofRef: AndroidStatusProofRef,
        supportingStatusArtifacts: AndroidStatusArtifacts,
      },
    ]);
    const android = proof.rows[0];
    const ios = proof.rows[1];

    expect(android?.platform).toBe('android');
    expect(android?.status).toBe('manual-required');
    expect(android?.physicalArtifactSetComplete).toBe(false);
    expect(android?.physicalDeviceStatusObserved).toBe(true);
    expect(android?.supportingStatusProofRef).toBe(AndroidStatusProofRef);
    expect(android?.supportingStatusArtifacts).toEqual(AndroidStatusArtifacts);
    expect(android?.missingArtifacts).toEqual(RequiredTrackingPhysicalDeviceArtifactPlans[0].requiredArtifacts);
    expect(android?.physicalDeviceBehaviorClaimed).toBe(false);
    expect(ios?.physicalDeviceStatusObserved).toBe(false);
    expect(proof.productClaims.productClaimReady).toBe(false);
  });
});

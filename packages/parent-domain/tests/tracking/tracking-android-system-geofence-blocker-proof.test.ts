import { describe, expect, it } from 'vitest';
import {
  LocalAndroidSystemGeofenceEvidenceArtifactRefs,
  RequiredAndroidSystemGeofenceRuntimeArtifactRefs,
  RequiredSystemGeofenceBlockers,
  TrackingAndroidSystemGeofenceBlockerRowSchema,
  buildTrackingAndroidSystemGeofenceBlockerProof,
} from '../../src/tracking-android-system-geofence-blocker-proof';

const GeneratedAt = '2026-06-07T15:45:00.000Z';
const SourceAndroidEmulatorProofRef = 'test-results/tracking-plan-android-emulator-proof/proof.json';

describe('tracking android system geofence blocker proof', () => {
  it('separates app-owned local geofence rows from Android system geofence delivery', () => {
    const proof = buildTrackingAndroidSystemGeofenceBlockerProof(
      GeneratedAt,
      SourceAndroidEmulatorProofRef,
      emulatorProof()
    );

    expect(proof.proofClaims).toEqual({
      localListenerGeofenceObserved: true,
      systemProximityRegistrationObserved: true,
      systemProximityDeliveryBlocked: true,
      noProductReadyClaim: true,
    });
    expect(proof.rows[0].localListenerGeofenceTransitionCount).toBe(3);
    expect(proof.rows[0].systemProximityTransitionCount).toBe(0);
    expect(proof.rows[0].localEvidenceArtifactRefs).toEqual([...LocalAndroidSystemGeofenceEvidenceArtifactRefs]);
    expect(proof.rows[0].requiredRuntimeArtifactRefs).toEqual([...RequiredAndroidSystemGeofenceRuntimeArtifactRefs]);
    expect(proof.rows[0].presentRuntimeArtifactRefs).toEqual([]);
    expect(proof.rows[0].missingRuntimeArtifactRefs).toEqual([...RequiredAndroidSystemGeofenceRuntimeArtifactRefs]);
    expect(proof.rows[0].runtimeArtifactSetComplete).toBe(false);
    expect(proof.rows[0].blockerRefs).toEqual([...RequiredSystemGeofenceBlockers]);
    expect(proof.productClaims.androidSystemGeofenceDeliveryClaimed).toBe(false);
    expect(proof.productClaims.productClaimReady).toBe(false);
  });

  it('rejects Android system geofence delivery overclaims', () => {
    const [row] = buildTrackingAndroidSystemGeofenceBlockerProof(
      GeneratedAt,
      SourceAndroidEmulatorProofRef,
      emulatorProof()
    ).rows;

    expect(
      TrackingAndroidSystemGeofenceBlockerRowSchema.safeParse({
        ...row,
        androidSystemGeofenceDeliveryClaimed: true,
      }).success
    ).toBe(false);
  });

  it('rejects missing blocker refs', () => {
    const [row] = buildTrackingAndroidSystemGeofenceBlockerProof(
      GeneratedAt,
      SourceAndroidEmulatorProofRef,
      emulatorProof()
    ).rows;

    expect(
      TrackingAndroidSystemGeofenceBlockerRowSchema.safeParse({
        ...row,
        blockerRefs: RequiredSystemGeofenceBlockers.slice(0, 1),
      }).success
    ).toBe(false);
  });

  it('rejects incomplete Android system geofence runtime artifact accounting', () => {
    const [row] = buildTrackingAndroidSystemGeofenceBlockerProof(
      GeneratedAt,
      SourceAndroidEmulatorProofRef,
      emulatorProof()
    ).rows;

    expect(
      TrackingAndroidSystemGeofenceBlockerRowSchema.safeParse({
        ...row,
        missingRuntimeArtifactRefs: row.missingRuntimeArtifactRefs.slice(0, 1),
      }).success
    ).toBe(false);
  });
});

function emulatorProof(): unknown {
  return {
    runtime: {
      geofenceTransitions: {
        transitionCount: 3,
        enterCount: 1,
        exitCount: 2,
        systemProximityRegistered: true,
        systemProximityTransitionCount: 0,
        systemProximityEnterCount: 0,
        systemProximityExitCount: 0,
      },
    },
  };
}

import { describe, expect, it } from 'vitest';
import {
  RequiredTrackingChildRuntimeAndroidEmulatorBridgeSourceRefs,
  TrackingChildRuntimeAndroidEmulatorBridgeRowSchema,
  buildTrackingChildRuntimeAndroidEmulatorBridgeProof,
} from '../src/tracking-child-runtime-android-emulator-readiness-bridge-proof';

const GeneratedAt = '2026-06-08T05:20:00.000Z';

describe('tracking child runtime Android emulator readiness bridge proof', () => {
  it('links emulator prerequisites to the child runtime artifact gate without child-device claims', () => {
    const proof = buildTrackingChildRuntimeAndroidEmulatorBridgeProof(GeneratedAt, bridgeInput());

    expect(proof.rows).toHaveLength(1);
    expect(proof.rows[0].status).toBe('emulator-prerequisites-observed-manual-runtime-required');
    expect(proof.rows[0].requiredProofTier).toBe('P4_PHYSICAL_DEVICE');
    expect(proof.rows[0].currentProofTier).toBe('P3_LOCAL_DEV_MACHINE');
    expect(proof.rows[0].sourceProofRefs).toEqual([...RequiredTrackingChildRuntimeAndroidEmulatorBridgeSourceRefs]);
    expect(proof.rows[0].emulatorPrerequisitesObserved).toBe(true);
    expect(proof.rows[0].childRuntimeArtifactSetComplete).toBe(false);
    expect(proof.rows[0].childRuntimeMissingArtifacts).toContain('04-visible-child-ui-snapshot.png');
    expect(proof.productClaims.androidEmulatorPrerequisitesObserved).toBe(true);
    expect(proof.productClaims.childDeviceDeliveryRuntimeClaimed).toBe(false);
    expect(proof.productClaims.childDeviceExecutionRuntimeClaimed).toBe(false);
    expect(proof.productClaims.renderedChildDeviceUiRuntimeClaimed).toBe(false);
    expect(proof.productClaims.productClaimReady).toBe(false);
  });

  it('rejects rows that mark the child-runtime artifact set complete from emulator evidence', () => {
    const [row] = buildTrackingChildRuntimeAndroidEmulatorBridgeProof(GeneratedAt, bridgeInput()).rows;

    expect(
      TrackingChildRuntimeAndroidEmulatorBridgeRowSchema.safeParse({
        ...row,
        childRuntimeArtifactSetComplete: true,
      }).success
    ).toBe(false);
  });

  it('rejects rows that claim rendered child-device runtime UI', () => {
    const [row] = buildTrackingChildRuntimeAndroidEmulatorBridgeProof(GeneratedAt, bridgeInput()).rows;

    expect(
      TrackingChildRuntimeAndroidEmulatorBridgeRowSchema.safeParse({
        ...row,
        renderedChildDeviceUiRuntimeClaimed: true,
      }).success
    ).toBe(false);
  });
});

function bridgeInput() {
  return {
    androidEmulatorProofRef: RequiredTrackingChildRuntimeAndroidEmulatorBridgeSourceRefs[0],
    childRuntimeArtifactGateProofRef: RequiredTrackingChildRuntimeAndroidEmulatorBridgeSourceRefs[1],
    androidProofStatus: 'emulator_scaffold_observed_nonvisual_screenshot',
    packageLaunchObserved: true,
    foregroundServiceObserved: true,
    foregroundPermissionGranted: true,
    backgroundPermissionGranted: true,
    localGeofenceTransitionCount: 3,
    androidEvidenceRefs: [
      'test-results/tracking-plan-android-emulator-proof/proof.json',
      'output/tracking-plan-proof/08-android-foreground-location-adapter/03-runtime-location-evidence.json',
    ],
    childRuntimeMissingArtifacts: [
      '02-delivery-envelope.json',
      '03-execution-result.json',
      '04-visible-child-ui-snapshot.png',
    ],
  };
}

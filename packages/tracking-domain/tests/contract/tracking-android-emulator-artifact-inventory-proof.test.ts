import { describe, expect, it } from 'vitest';
import {
  RequiredTrackingAndroidEmulatorArtifactRefs,
  TrackingAndroidEmulatorArtifactInventoryRowSchema,
  buildTrackingAndroidEmulatorArtifactInventoryProof,
} from '@ocentra-parent/schema-domain/tracking-android-emulator-artifact-inventory-proof';

const GeneratedAt = '2026-06-08T10:00:00.000Z';

describe('tracking Android emulator artifact inventory proof', () => {
  it('classifies existing emulator artifacts without claiming physical-device readiness', () => {
    const proof = buildTrackingAndroidEmulatorArtifactInventoryProof(GeneratedAt, input());

    expect(proof.rows).toHaveLength(1);
    expect(proof.summary.requiredArtifactCount).toBe(RequiredTrackingAndroidEmulatorArtifactRefs.length);
    expect(proof.summary.presentArtifactCount).toBe(RequiredTrackingAndroidEmulatorArtifactRefs.length);
    expect(proof.summary.missingArtifactCount).toBe(0);
    expect(proof.summary.permissionUiArtifactCount).toBe(3);
    expect(proof.summary.localGeofenceTransitionCount).toBe(3);
    expect(proof.summary.localGeofenceDwellCount).toBe(1);
    expect(proof.productClaims.androidEmulatorArtifactInventoryComplete).toBe(true);
    expect(proof.productClaims.androidSystemGeofenceDeliveryClaimed).toBe(false);
    expect(proof.productClaims.physicalDeviceProofClaimed).toBe(false);
    expect(proof.productClaims.authorityProofClaimed).toBe(false);
    expect(proof.productClaims.productionRuntimeClaimed).toBe(false);
    expect(proof.productClaims.productClaimReady).toBe(false);
  });

  it('keeps missing artifact accounting explicit', () => {
    const [firstArtifact, ...remainingArtifacts] = artifactRows();
    const proof = buildTrackingAndroidEmulatorArtifactInventoryProof(GeneratedAt, {
      ...input(),
      artifactRows: [
        {
          ...firstArtifact,
          present: false,
          byteSize: 0,
        },
        ...remainingArtifacts,
      ],
    });

    expect(proof.summary.presentArtifactCount).toBe(RequiredTrackingAndroidEmulatorArtifactRefs.length - 1);
    expect(proof.summary.missingArtifactCount).toBe(1);
    expect(proof.productClaims.androidEmulatorArtifactInventoryComplete).toBe(false);
  });

  it('rejects rows that claim product readiness', () => {
    const [row] = buildTrackingAndroidEmulatorArtifactInventoryProof(GeneratedAt, input()).rows;

    expect(
      TrackingAndroidEmulatorArtifactInventoryRowSchema.safeParse({
        ...row,
        productClaimReady: true,
      }).success
    ).toBe(false);
  });
});

function input() {
  return {
    sourceAndroidEmulatorProofRef: 'test-results/tracking-plan-android-emulator-proof/proof.json',
    androidSdkRoot: 'C:/Users/sujan/AppData/Local/Android/Sdk',
    androidProofStatus: 'emulator_scaffold_observed_nonvisual_screenshot',
    packageName: 'ca.ocentra.parent.agent',
    activityName: 'ca.ocentra.parent.agent/.MainActivity',
    deviceSerial: 'emulator-5554',
    androidRelease: '15',
    androidSdk: '35',
    productModel: 'sdk_gphone64_x86_64',
    abi: 'x86_64',
    foregroundPermissionGranted: true,
    backgroundPermissionGranted: true,
    foregroundPermissionUxObserved: true,
    backgroundSettingsPageObserved: true,
    packageLaunchObserved: true,
    foregroundServiceObserved: true,
    localGeofenceTransitionCount: 3,
    localGeofenceDwellCount: 1,
    systemProximityRegistered: true,
    systemProximityTransitionCount: 0,
    artifactRows: artifactRows(),
  };
}

function artifactRows() {
  return RequiredTrackingAndroidEmulatorArtifactRefs.map((artifactRef) => ({
    artifactRef,
    category: categoryFor(artifactRef),
    required: true,
    present: true,
    byteSize: 10,
  }));
}

function categoryFor(artifactRef: string) {
  if (artifactRef.includes('permission') || artifactRef.includes('settings-page')) return 'permission-ui';
  if (artifactRef.includes('location-evidence')) return 'location-runtime';
  if (artifactRef.includes('geofence-transition')) return 'geofence-runtime';
  if (artifactRef.includes('device-status')) return 'device-status';
  if (artifactRef.includes('validation-commands')) return 'validation-log';
  return 'adb-runtime-output';
}

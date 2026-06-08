import { describe, expect, it } from 'vitest';
import {
  RequiredTrackingAndroidPhysicalDeviceRuntimeArtifactRefs,
  TrackingAndroidPhysicalDeviceRuntimeRowSchema,
  buildTrackingAndroidPhysicalDeviceRuntimeProof,
} from '../src/tracking-android-physical-device-runtime-proof';

const GeneratedAt = '2026-06-08T16:10:00.000Z';

describe('tracking Android physical-device runtime proof', () => {
  it('classifies physical package runtime evidence without claiming location product readiness', () => {
    const proof = buildTrackingAndroidPhysicalDeviceRuntimeProof(GeneratedAt, input());

    expect(proof.rows).toHaveLength(1);
    expect(proof.summary.requiredArtifactCount).toBe(RequiredTrackingAndroidPhysicalDeviceRuntimeArtifactRefs.length);
    expect(proof.summary.presentArtifactCount).toBe(RequiredTrackingAndroidPhysicalDeviceRuntimeArtifactRefs.length);
    expect(proof.summary.missingArtifactCount).toBe(0);
    expect(proof.summary.physicalDeviceRuntimeObserved).toBe(true);
    expect(proof.summary.physicalLocationArtifactCount).toBe(1);
    expect(proof.summary.physicalGeofenceArtifactCount).toBe(1);
    expect(proof.summary.backgroundLocationSampleCount).toBe(1);
    expect(proof.summary.localGeofenceTransitionCount).toBe(2);
    expect(proof.summary.localGeofenceDwellCount).toBe(1);
    expect(proof.productClaims.physicalDeviceRuntimeObserved).toBe(true);
    expect(proof.productClaims.physicalLocationRuntimeClaimed).toBe(false);
    expect(proof.productClaims.physicalGeofenceRuntimeClaimed).toBe(false);
    expect(proof.productClaims.androidSystemGeofenceDeliveryClaimed).toBe(false);
    expect(proof.productClaims.authorityProofClaimed).toBe(false);
    expect(proof.productClaims.productionRuntimeClaimed).toBe(false);
    expect(proof.productClaims.productClaimReady).toBe(false);
  });

  it('keeps missing physical artifacts explicit', () => {
    const [firstArtifact, ...remainingArtifacts] = artifactRows();
    const proof = buildTrackingAndroidPhysicalDeviceRuntimeProof(GeneratedAt, {
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

    expect(proof.summary.presentArtifactCount).toBe(
      RequiredTrackingAndroidPhysicalDeviceRuntimeArtifactRefs.length - 1
    );
    expect(proof.summary.missingArtifactCount).toBe(1);
  });

  it('rejects rows that overclaim physical geofence runtime', () => {
    const [row] = buildTrackingAndroidPhysicalDeviceRuntimeProof(GeneratedAt, input()).rows;

    expect(
      TrackingAndroidPhysicalDeviceRuntimeRowSchema.safeParse({
        ...row,
        physicalGeofenceRuntimeClaimed: true,
      }).success
    ).toBe(false);
  });
});

function input() {
  return {
    physicalDeviceProofRef: 'test-results/tracking-android-physical-device-runtime-proof/proof.json',
    packageName: 'ca.ocentra.parent.agent',
    activityName: 'ca.ocentra.parent.agent/.MainActivity',
    deviceSerial: '192.168.2.45:5555',
    androidRelease: '10',
    androidSdk: '29',
    productModel: 'SM-G965W',
    productName: 'star2qltecs',
    abi: 'arm64-v8a',
    packageInstallObserved: true,
    packageLaunchObserved: true,
    foregroundServiceObserved: true,
    uiLaunchTextObserved: true,
    batteryDumpObserved: true,
    connectivityDumpObserved: true,
    foregroundPermissionGranted: false,
    backgroundPermissionGranted: false,
    locationSampleObserved: true,
    backgroundLocationSampleCount: 1,
    physicalRouteObservationWindowSeconds: 60,
    shellLocationInjectionAvailable: false,
    localGeofenceTransitionCount: 2,
    localGeofenceDwellCount: 1,
    androidSystemGeofenceTransitionCount: 0,
    artifactRows: artifactRows(),
  };
}

function artifactRows() {
  return RequiredTrackingAndroidPhysicalDeviceRuntimeArtifactRefs.map((artifactRef) => ({
    artifactRef,
    category: categoryFor(artifactRef),
    required: true,
    present: true,
    byteSize: 10,
  }));
}

function categoryFor(artifactRef: string) {
  if (artifactRef.includes('install') || artifactRef.includes('launch') || artifactRef.includes('activity')) {
    return 'package-runtime';
  }
  if (artifactRef.includes('service')) return 'foreground-service';
  if (artifactRef.includes('battery') || artifactRef.includes('connectivity')) return 'device-status';
  if (artifactRef.includes('ui') || artifactRef.includes('screen')) return 'ui-screenshot';
  if (artifactRef.includes('permission')) return 'permission-state';
  if (artifactRef.includes('sample')) return 'physical-location-runtime';
  if (artifactRef.includes('geofence')) return 'physical-geofence-runtime';
  if (artifactRef.includes('route') || artifactRef.includes('location-manager')) return 'physical-route-observation';
  if (artifactRef.includes('logcat')) return 'validation-log';
  return 'adb-runtime-output';
}

import { describe, expect, it } from 'vitest';
import {
  RequiredTrackingAndroidPhysicalDeviceRuntimeArtifactRefs,
  TrackingAndroidPhysicalDeviceRuntimeRowSchema,
  buildTrackingAndroidPhysicalDeviceRuntimeProof,
} from '../../src/tracking-android-physical-device-runtime-proof';

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
    expect(proof.summary.geofenceRegistrationObserved).toBe(true);
    expect(proof.summary.systemProximityRegistrationObserved).toBe(true);
    expect(proof.summary.localGeofenceTransitionCount).toBe(2);
    expect(proof.summary.localGeofenceDwellCount).toBe(1);
    expect(proof.proofClaims.geofenceRegistrationObserved).toBe(true);
    expect(proof.proofClaims.systemProximityRegistrationObserved).toBe(true);
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
    geofenceRegistrationObserved: true,
    systemProximityRegistrationObserved: true,
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

const artifactCategoryMatchers = [
  { category: 'package-runtime', tokens: ['install', 'launch', 'activity'] },
  { category: 'foreground-service', tokens: ['service'] },
  { category: 'device-status', tokens: ['battery', 'connectivity'] },
  { category: 'ui-screenshot', tokens: ['ui', 'screen'] },
  { category: 'permission-state', tokens: ['permission'] },
  { category: 'physical-location-runtime', tokens: ['sample'] },
  { category: 'physical-geofence-runtime', tokens: ['geofence'] },
  { category: 'physical-route-observation', tokens: ['route', 'location-manager'] },
  { category: 'validation-log', tokens: ['logcat'] },
] as const;

function categoryFor(artifactRef: string) {
  return (
    artifactCategoryMatchers.find(({ tokens }) => tokens.some((token) => artifactRef.includes(token)))?.category ??
    'adb-runtime-output'
  );
}

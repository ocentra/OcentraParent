import { describe, expect, it } from 'vitest';
import {
  RequiredTrackingCrossPlatformRuntimeCapabilityAreas,
  buildTrackingCrossPlatformRuntimeCapabilityProof,
} from '../src/tracking-cross-platform-runtime-capability-proof';

const baseRows = [
  {
    area: 'windows-host-toolchain',
    status: 'local-proof-passed',
    proofRef: 'test-results/tracking-cross-platform-runtime-capability-proof/windows-host.json',
    sourceRefs: [],
    currentProofTier: 'P3_LOCAL_WINDOWS_HOST',
    requiredProofTier: 'P3_LOCAL_DEV_MACHINE',
    observedTooling: ['Windows 10.0.26200', 'Node 22', 'Cargo 1.90'],
    passedAssertions: ['Windows host shell and toolchain are reachable'],
    remainingBlockers: ['Windows host proof is not desktop precise location approval'],
    artifactCount: 4,
    ciRunnable: true,
    localRuntimeClaimed: true,
  },
  {
    area: 'wsl-linux-replay',
    status: 'local-proof-passed',
    proofRef: 'test-results/tracking-plan-wsl-local-proof/proof.json',
    sourceRefs: ['output/tracking-plan-proof/wsl-local-replay/proof.json'],
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    requiredProofTier: 'P3_LOCAL_DEV_MACHINE',
    observedTooling: ['Ubuntu-22.04', 'WSL2'],
    passedAssertions: ['WSL Linux replay proof is present'],
    remainingBlockers: ['WSL replay is not mobile physical-device proof'],
    artifactCount: 3,
    ciRunnable: true,
    localRuntimeClaimed: true,
  },
  {
    area: 'docker-container-runtime',
    status: 'host-tool-unavailable',
    proofRef: 'test-results/tracking-cross-platform-runtime-capability-proof/docker.json',
    sourceRefs: [],
    currentProofTier: 'P2_HOST_TOOL_DISCOVERED',
    requiredProofTier: 'P3_LOCAL_CONTAINER_RUNTIME',
    observedTooling: ['Docker CLI path discovered'],
    passedAssertions: [],
    remainingBlockers: ['Docker daemon is not currently reachable'],
    artifactCount: 1,
    ciRunnable: true,
    localRuntimeClaimed: false,
  },
  {
    area: 'android-sdk-toolchain',
    status: 'local-proof-passed',
    proofRef: 'test-results/tracking-cross-platform-runtime-capability-proof/android-sdk-toolchain.json',
    sourceRefs: [],
    currentProofTier: 'P3_LOCAL_ANDROID_TOOLCHAIN',
    requiredProofTier: 'P3_LOCAL_DEV_MACHINE',
    observedTooling: ['ANDROID_HOME=C:\\Users\\sujan\\AppData\\Local\\Android\\Sdk', 'adb 36.0.0', 'Java 21'],
    passedAssertions: ['Android SDK, adb, and Java toolchain are reachable'],
    remainingBlockers: ['Android SDK proof is not device-owner or physical background proof'],
    artifactCount: 3,
    ciRunnable: true,
    localRuntimeClaimed: true,
  },
  {
    area: 'android-gradle-project-build',
    status: 'local-proof-passed',
    proofRef: 'test-results/tracking-cross-platform-runtime-capability-proof/android-gradle-project-build.json',
    sourceRefs: ['platforms/android/agent/app/build.gradle'],
    currentProofTier: 'P3_LOCAL_ANDROID_GRADLE_BUILD',
    requiredProofTier: 'P3_LOCAL_DEV_MACHINE',
    observedTooling: ['Gradle 8.12.1', ':app:assembleDebug'],
    passedAssertions: ['Android agent Gradle project can build the debug APK'],
    remainingBlockers: ['Gradle build is not physical-device runtime behavior'],
    artifactCount: 2,
    ciRunnable: true,
    localRuntimeClaimed: true,
  },
  {
    area: 'android-emulator-runtime',
    status: 'local-proof-passed',
    proofRef: 'test-results/tracking-plan-android-emulator-proof/proof.json',
    sourceRefs: ['test-results/tracking-android-emulator-artifact-inventory-proof/proof.json'],
    currentProofTier: 'P3_LOCAL_ANDROID_EMULATOR',
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    observedTooling: ['adb', 'Android emulator'],
    passedAssertions: ['Android emulator runtime artifacts are present'],
    remainingBlockers: ['Android system geofence and physical-device behavior remain required'],
    artifactCount: 12,
    ciRunnable: false,
    localRuntimeClaimed: true,
  },
  {
    area: 'android-physical-device-status',
    status: 'local-proof-passed',
    proofRef: 'test-results/tracking-android-physical-device-runtime-proof/proof.json',
    sourceRefs: [],
    currentProofTier: 'P4_PHYSICAL_DEVICE_STATUS_ONLY',
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    observedTooling: ['Samsung S9 adb over Wi-Fi'],
    observedCapabilityRefs: [
      'android-physical-geofence-registration',
      'android-physical-system-proximity-registration',
    ],
    passedAssertions: ['Physical package, service, battery, and connectivity artifacts are present'],
    remainingBlockers: ['Physical location/geofence behavior remains unclaimed'],
    artifactCount: 13,
    ciRunnable: false,
    localRuntimeClaimed: true,
  },
  {
    area: 'macos-ios-ci-manual-routing',
    status: 'ci-manual-required',
    proofRef: 'test-results/tracking-ios-simulator-artifact-inventory-proof/proof.json',
    sourceRefs: ['test-results/tracking-plan-ios-simulator-proof/proof.json'],
    currentProofTier: 'P2_CI_OR_MANUAL_REQUIRED',
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    observedTooling: ['macOS package-preview routing', 'iOS simulator proof routing'],
    passedAssertions: ['CI/manual routing is documented'],
    remainingBlockers: ['macOS/iOS runtime cannot be executed on this Windows host'],
    artifactCount: 2,
    ciRunnable: true,
    localRuntimeClaimed: false,
  },
] as const;

describe('tracking cross-platform runtime capability proof', () => {
  it('covers the local, container, mobile, and CI/manual platform areas', () => {
    expect(RequiredTrackingCrossPlatformRuntimeCapabilityAreas).toEqual([
      'windows-host-toolchain',
      'wsl-linux-replay',
      'docker-container-runtime',
      'android-sdk-toolchain',
      'android-gradle-project-build',
      'android-emulator-runtime',
      'android-physical-device-status',
      'macos-ios-ci-manual-routing',
    ]);
  });

  it('keeps product and physical behavior claims false while aggregating local proof', () => {
    const proof = buildTrackingCrossPlatformRuntimeCapabilityProof('2026-06-08T16:30:00.000Z', baseRows);

    expect(proof.rows).toHaveLength(8);
    expect(proof.summary.localProofPassedRows).toBe(6);
    expect(proof.summary.hostToolUnavailableRows).toBe(1);
    expect(proof.summary.ciManualRequiredRows).toBe(1);
    expect(proof.productClaims.windowsHostToolchainObserved).toBe(true);
    expect(proof.productClaims.wslLinuxReplayObserved).toBe(true);
    expect(proof.productClaims.dockerContainerRuntimeObserved).toBe(false);
    expect(proof.productClaims.androidSdkToolchainObserved).toBe(true);
    expect(proof.productClaims.androidGradleProjectBuildObserved).toBe(true);
    expect(proof.productClaims.androidPhysicalGeofenceRegistrationObserved).toBe(true);
    expect(proof.productClaims.androidPhysicalSystemProximityRegistrationObserved).toBe(true);
    expect(proof.productClaims.productClaimReady).toBe(false);
    expect(proof.rows.every((row) => row.physicalDeviceBehaviorClaimed === false)).toBe(true);
  });

  it('rejects a missing required platform area', () => {
    expect(() =>
      buildTrackingCrossPlatformRuntimeCapabilityProof(
        '2026-06-08T16:30:00.000Z',
        baseRows.filter((row) => row.area !== 'docker-container-runtime')
      )
    ).toThrow('Missing cross-platform runtime capability area: docker-container-runtime');
  });
});

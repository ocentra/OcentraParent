import { describe, expect, it } from 'vitest';
import {
  RequiredTrackingLocalPlatformProofBatchAreas,
  buildTrackingLocalPlatformProofBatch,
} from '../src/tracking-local-platform-proof-batch';

const baseRows = [
  {
    area: 'android-emulator-runtime',
    status: 'local-proof-passed',
    proofRef: 'test-results/tracking-plan-android-emulator-proof/proof.json',
    sourceRefs: ['test-results/tracking-android-emulator-artifact-inventory-proof/proof.json'],
    currentProofTier: 'P3_LOCAL_ANDROID_EMULATOR',
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    passedLocalAssertions: ['background permission grant observed on emulator'],
    remainingBlockers: ['physical Android system geofence and dwell delivery required'],
    metrics: [
      { name: 'localGeofenceTransitionCount', value: 4 },
      { name: 'systemProximityBroadcastCount', value: 0 },
    ],
    ciRunnable: false,
  },
  {
    area: 'cross-platform-runtime-capability',
    status: 'local-proof-passed',
    proofRef: 'test-results/tracking-cross-platform-runtime-capability-proof/proof.json',
    sourceRefs: [
      'test-results/tracking-cross-platform-runtime-capability-proof/android-sdk-toolchain.json',
      'test-results/tracking-cross-platform-runtime-capability-proof/android-gradle-project-build.json',
    ],
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    passedLocalAssertions: [
      'Windows host, WSL/Linux, Android SDK, Android Gradle build, and Android emulator capability are accounted',
    ],
    remainingBlockers: ['macOS/iOS require CI/manual routing and physical mobile behavior remains separately gated'],
    metrics: [
      { name: 'crossPlatformRowCount', value: 8 },
      { name: 'localProofPassedRows', value: 6 },
      { name: 'productReadyRows', value: 0 },
    ],
    ciRunnable: true,
  },
  {
    area: 'wsl-local-replay',
    status: 'local-proof-passed',
    proofRef: 'test-results/tracking-plan-wsl-local-proof/proof.json',
    sourceRefs: ['output/tracking-plan-proof/wsl-local-replay/proof.json'],
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    requiredProofTier: 'P3_LOCAL_DEV_MACHINE',
    passedLocalAssertions: ['WSL replay proof status is proved'],
    remainingBlockers: [],
    metrics: [{ name: 'commandCount', value: 3 }],
    ciRunnable: true,
  },
  {
    area: 'hosted-parent-ui-accessibility',
    status: 'local-proof-passed',
    proofRef: 'test-results/tracking-plan-hosted-ui-proof/accessibility-summary.json',
    sourceRefs: ['test-results/tracking-hosted-ui-artifact-inventory-proof/proof.json'],
    currentProofTier: 'P2_HOSTED_CI',
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    passedLocalAssertions: ['hosted parent route accessibility summary has a named region'],
    remainingBlockers: ['full product parent/child UI runtime required'],
    metrics: [
      { name: 'headingCount', value: 28 },
      { name: 'unlabeledButtonCount', value: 0 },
    ],
    ciRunnable: true,
  },
  {
    area: 'parent-child-local-runtime-bridge',
    status: 'local-proof-passed',
    proofRef: 'test-results/tracking-parent-child-local-runtime-bridge-proof/proof.json',
    sourceRefs: ['test-results/eventing-parent-child-runtime-proof/proof.json'],
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    passedLocalAssertions: ['local parent-child runtime bridge has nine ordered events and zero dead letters'],
    remainingBlockers: ['physical child-device delivery and rendered child UI runtime required'],
    metrics: [
      { name: 'storedEventCount', value: 9 },
      { name: 'deadLetterCount', value: 0 },
    ],
    ciRunnable: true,
  },
  {
    area: 'product-parent-child-ui-local-artifacts',
    status: 'local-proof-passed',
    proofRef: 'test-results/tracking-full-product-ui-local-runtime-artifact-capture-proof/proof.json',
    sourceRefs: ['test-results/tracking-full-product-ui-runtime-preflight-proof/proof.json'],
    currentProofTier: 'P2_HOSTED_CI',
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    passedLocalAssertions: ['local product UI artifact capture has eight artifacts'],
    remainingBlockers: ['rendered child-device runtime artifacts required'],
    metrics: [
      { name: 'localArtifactCount', value: 8 },
      { name: 'missingRuntimeArtifactCount', value: 4 },
    ],
    ciRunnable: true,
  },
  {
    area: 'real-runtime-handoff-accounting',
    status: 'manual-required',
    proofRef: 'test-results/tracking-real-runtime-handoff-proof/proof.json',
    sourceRefs: ['test-results/tracking-product-readiness-closure-proof/proof.json'],
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    passedLocalAssertions: ['handoff carries local evidence accounting'],
    remainingBlockers: ['physical device, provider, authority, and production runtime proof required'],
    metrics: [
      { name: 'handoffRowCount', value: 10 },
      { name: 'productReadyRows', value: 0 },
    ],
    ciRunnable: false,
  },
] as const;

describe('tracking local platform proof batch', () => {
  it('requires every local platform proof area', () => {
    expect(RequiredTrackingLocalPlatformProofBatchAreas).toEqual([
      'android-emulator-runtime',
      'cross-platform-runtime-capability',
      'wsl-local-replay',
      'hosted-parent-ui-accessibility',
      'parent-child-local-runtime-bridge',
      'product-parent-child-ui-local-artifacts',
      'real-runtime-handoff-accounting',
    ]);
  });

  it('aggregates local proof rows while keeping runtime and product claims false', () => {
    const proof = buildTrackingLocalPlatformProofBatch('2026-06-08T15:30:00.000Z', baseRows);

    expect(proof.rows).toHaveLength(7);
    expect(proof.summary.localProofPassedRows).toBe(6);
    expect(proof.summary.manualRequiredRows).toBe(1);
    expect(proof.summary.ciRunnableRows).toBe(5);
    expect(proof.summary.productReadyRows).toBe(0);
    expect(proof.productClaims.androidEmulatorLocalProofPassed).toBe(true);
    expect(proof.productClaims.crossPlatformRuntimeCapabilityPassed).toBe(true);
    expect(proof.productClaims.parentChildLocalRuntimeBridgePassed).toBe(true);
    expect(proof.productClaims.productClaimReady).toBe(false);
    expect(proof.rows.every((row) => row.physicalDeviceClaimed === false)).toBe(true);
    expect(proof.rows.every((row) => row.iosRuntimeClaimed === false)).toBe(true);
    expect(proof.rows.every((row) => row.childDeviceRuntimeClaimed === false)).toBe(true);
  });

  it('rejects a missing required local platform area', () => {
    expect(() =>
      buildTrackingLocalPlatformProofBatch(
        '2026-06-08T15:30:00.000Z',
        baseRows.filter((row) => row.area !== 'wsl-local-replay')
      )
    ).toThrow('Missing tracking local platform proof batch area: wsl-local-replay');
  });
});

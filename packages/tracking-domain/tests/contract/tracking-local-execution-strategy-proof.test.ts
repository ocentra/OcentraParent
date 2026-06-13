import { describe, expect, it } from 'vitest';
import {
  RequiredTrackingLocalExecutionStrategyAreas,
  buildTrackingLocalExecutionStrategyProof,
} from '../../src/tracking-local-execution-strategy-proof';

const GeneratedAt = '2026-06-08T21:40:00.000Z';

describe('tracking local execution strategy proof', () => {
  it('requires the full batch execution strategy area set', () => {
    expect(RequiredTrackingLocalExecutionStrategyAreas).toEqual([
      'windows-host-local-validation',
      'wsl-local-replay',
      'docker-host-availability',
      'android-emulator-runtime',
      'android-physical-status-runtime',
      'macos-ios-ci-route',
      'physical-manual-runtime-route',
      'final-sync-validation-gate',
    ]);
  });

  it('classifies local, CI, unavailable, and manual routes without product claims', () => {
    const proof = buildTrackingLocalExecutionStrategyProof(GeneratedAt, rows());

    expect(proof.rows).toHaveLength(8);
    expect(proof.summary.localRunnableRows).toBe(4);
    expect(proof.summary.ciRunnableRows).toBe(4);
    expect(proof.summary.manualRequiredRows).toBe(2);
    expect(proof.summary.unavailableHereRows).toBe(1);
    expect(proof.summary.physicalDeviceRequiredRows).toBe(2);
    expect(proof.summary.macHostRequiredRows).toBe(1);
    expect(proof.summary.dockerHostRequiredRows).toBe(1);
    expect(proof.summary.productReadyRows).toBe(0);
    expect(proof.productClaims.localBatchStrategyReady).toBe(true);
    expect(proof.productClaims.finalSyncRequiredBeforePr).toBe(true);
    expect(proof.productClaims.dockerUnavailableOnCurrentHost).toBe(true);
    expect(proof.productClaims.physicalBehaviorClaimed).toBe(false);
    expect(proof.productClaims.iosRuntimeClaimed).toBe(false);
    expect(proof.productClaims.childRuntimeClaimed).toBe(false);
    expect(proof.productClaims.productionRuntimeClaimed).toBe(false);
    expect(proof.productClaims.productClaimReady).toBe(false);
  });

  it('rejects a missing strategy area', () => {
    expect(() =>
      buildTrackingLocalExecutionStrategyProof(
        GeneratedAt,
        rows().filter((candidate) => candidate.area !== 'docker-host-availability')
      )
    ).toThrow('Missing tracking execution strategy area: docker-host-availability');
  });
});

function rows() {
  return [
    localRow('windows-host-local-validation', 'test-results/tracking-local-execution-strategy-proof/windows.json'),
    localRow('wsl-local-replay', 'test-results/tracking-plan-wsl-local-proof/proof.json'),
    unavailableDockerRow(),
    localRow('android-emulator-runtime', 'test-results/tracking-plan-android-emulator-proof/proof.json'),
    androidPhysicalStatusRow(),
    macosIosCiRouteRow(),
    manualRuntimeRow(),
    finalSyncRow(),
  ] as const;
}

function localRow(
  area: 'windows-host-local-validation' | 'wsl-local-replay' | 'android-emulator-runtime',
  proofRef: string
) {
  return {
    area,
    route: 'local-runnable',
    status: 'ready',
    proofRef,
    sourceRefs: [],
    commandsToRunAfterCodeBatch: ['cmd /c npm run format:check'],
    evidenceRefsExpected: [proofRef],
    passedEvidenceRefs: [proofRef],
    blockers: [],
    localRunnable: true,
    ciRunnable: area !== 'android-emulator-runtime',
    requiresPhysicalDevice: false,
    requiresMacHost: false,
    requiresDockerHost: false,
  } as const;
}

function unavailableDockerRow() {
  return {
    area: 'docker-host-availability',
    route: 'unavailable-here',
    status: 'unavailable-here',
    proofRef: 'test-results/tracking-local-execution-strategy-proof/docker-host.json',
    sourceRefs: [],
    commandsToRunAfterCodeBatch: ['docker --version'],
    evidenceRefsExpected: ['test-results/tracking-local-execution-strategy-proof/docker-host.json'],
    passedEvidenceRefs: [],
    blockers: ['Docker CLI is not available on the current Windows PATH'],
    localRunnable: false,
    ciRunnable: false,
    requiresPhysicalDevice: false,
    requiresMacHost: false,
    requiresDockerHost: true,
  } as const;
}

function androidPhysicalStatusRow() {
  return {
    area: 'android-physical-status-runtime',
    route: 'local-runnable',
    status: 'observed',
    proofRef: 'test-results/tracking-android-physical-device-runtime-proof/proof.json',
    sourceRefs: ['test-results/tracking-physical-device-evidence-review-proof/proof.json'],
    commandsToRunAfterCodeBatch: ['node scripts/test/tracking-android-physical-device-runtime-proof.mjs'],
    evidenceRefsExpected: ['test-results/tracking-android-physical-device-runtime-proof/proof.json'],
    passedEvidenceRefs: ['test-results/tracking-android-physical-device-runtime-proof/proof.json'],
    blockers: ['Physical geofence and Android system geofence delivery remain unproved'],
    localRunnable: true,
    ciRunnable: false,
    requiresPhysicalDevice: true,
    requiresMacHost: false,
    requiresDockerHost: false,
  } as const;
}

function macosIosCiRouteRow() {
  return {
    area: 'macos-ios-ci-route',
    route: 'ci-runnable',
    status: 'manual-required',
    proofRef: 'test-results/tracking-plan-ios-simulator-proof/proof.json',
    sourceRefs: ['test-results/tracking-ios-simulator-artifact-inventory-proof/proof.json'],
    commandsToRunAfterCodeBatch: ['GitHub macOS package-preview job uploads simulator artifacts'],
    evidenceRefsExpected: ['test-results/tracking-plan-ios-simulator-proof/proof.json'],
    passedEvidenceRefs: [],
    blockers: ['Windows host cannot run macOS/iOS simulator runtime locally'],
    localRunnable: false,
    ciRunnable: true,
    requiresPhysicalDevice: false,
    requiresMacHost: true,
    requiresDockerHost: false,
  } as const;
}

function manualRuntimeRow() {
  return {
    area: 'physical-manual-runtime-route',
    route: 'manual-required',
    status: 'manual-required',
    proofRef: 'test-results/tracking-real-runtime-handoff-proof/proof.json',
    sourceRefs: ['test-results/tracking-claim-audit-proof/proof.json'],
    commandsToRunAfterCodeBatch: ['node scripts/test/tracking-real-runtime-handoff-proof.mjs'],
    evidenceRefsExpected: ['test-results/tracking-real-runtime-handoff-proof/proof.json'],
    passedEvidenceRefs: [],
    blockers: [
      'Physical movement, authority enrollment, child-device runtime, provider, and production artifacts remain required',
    ],
    localRunnable: false,
    ciRunnable: false,
    requiresPhysicalDevice: true,
    requiresMacHost: false,
    requiresDockerHost: false,
  } as const;
}

function finalSyncRow() {
  return {
    area: 'final-sync-validation-gate',
    route: 'final-checkpoint',
    status: 'ready',
    proofRef: 'test-results/tracking-local-platform-proof-batch/proof.json',
    sourceRefs: [],
    commandsToRunAfterCodeBatch: ['git fetch origin main', 'git rebase origin/main', 'cmd /c npm run validate'],
    evidenceRefsExpected: ['test-results/tracking-local-execution-strategy-proof/proof.json'],
    passedEvidenceRefs: ['test-results/tracking-local-execution-strategy-proof/proof.json'],
    blockers: [],
    localRunnable: false,
    ciRunnable: true,
    requiresPhysicalDevice: false,
    requiresMacHost: false,
    requiresDockerHost: false,
  } as const;
}

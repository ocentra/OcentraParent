import { describe, expect, it } from 'vitest';
import {
  AppGameLinuxWslRuntimeProofSchema,
  decodeAppGameLinuxWslRuntimeProof,
  summarizeAppGameLinuxWslRuntimeProof,
} from '../../src/app-game-linux-wsl-runtime-proof';

describe('app-game Linux WSL runtime proof', () => {
  registerAcceptedWslProofTest();
  registerRedactionRejectionTest();
  registerClaimRejectionTest();
  registerDisplayReadinessRejectionTest();
});

function registerAcceptedWslProofTest() {
  it('accepts a redacted WSL2 Ubuntu runtime proof without upgrading Linux blocking claims', () => {
    const proof = decodeAppGameLinuxWslRuntimeProof(validWslProof());
    const summary = summarizeAppGameLinuxWslRuntimeProof(proof);

    expect(proof.targetKind).toBe('wsl2-distro');
    expect(proof.distroId).toBe('ubuntu');
    expect(proof.packageManagerVisibleCount).toBeGreaterThan(0);
    expect(proof.processSnapshotCount).toBeGreaterThan(0);
    expect(proof.displayProofAttached).toBe(true);
    expect(proof.foregroundCaptureClaimed).toBe(false);
    expect(summary.displayState).toBe('wslg-display-observed');
    expect(summary.foregroundProbeState).toBe('active-window-tool-missing');
    expect(summary.proofComplete).toBe(false);
    expect(summary.adapterDispatchClaimed).toBe(false);
    expect(summary.platformEnforcementClaimed).toBe(false);
  });
}

function registerRedactionRejectionTest() {
  it('rejects Linux proof artifacts that expose raw package, process, or distro names', () => {
    expect(
      AppGameLinuxWslRuntimeProofSchema.safeParse({
        ...validWslProof(),
        packageNamesRedacted: false,
      }).success
    ).toBe(false);
    expect(
      AppGameLinuxWslRuntimeProofSchema.safeParse({
        ...validWslProof(),
        processNamesRedacted: false,
      }).success
    ).toBe(false);
    expect(
      AppGameLinuxWslRuntimeProofSchema.safeParse({
        ...validWslProof(),
        rawDistroNameRedacted: false,
      }).success
    ).toBe(false);
  });
}

function registerClaimRejectionTest() {
  it('rejects broad blocking or platform enforcement claims from incomplete WSL proof', () => {
    expect(
      AppGameLinuxWslRuntimeProofSchema.safeParse({
        ...validWslProof(),
        foregroundCaptureClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameLinuxWslRuntimeProofSchema.safeParse({
        ...validWslProof(),
        adapterDispatchClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameLinuxWslRuntimeProofSchema.safeParse({
        ...validWslProof(),
        broadBlockingClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameLinuxWslRuntimeProofSchema.safeParse({
        ...validWslProof(),
        platformEnforcementClaimed: true,
      }).success
    ).toBe(false);
  });
}

function registerDisplayReadinessRejectionTest() {
  it('rejects WSLg display readiness when X11 or Wayland socket proof is missing', () => {
    expect(
      AppGameLinuxWslRuntimeProofSchema.safeParse({
        ...validWslProof(),
        x11SocketState: 'socket-not-proved',
      }).success
    ).toBe(false);
    expect(
      AppGameLinuxWslRuntimeProofSchema.safeParse({
        ...validWslProof(),
        waylandSocketState: 'socket-not-proved',
      }).success
    ).toBe(false);
    expect(
      AppGameLinuxWslRuntimeProofSchema.safeParse({
        ...validWslProof(),
        displayProofAttached: false,
      }).success
    ).toBe(false);
  });
}

function validWslProof() {
  return {
    schemaVersion: 'app-game-linux-wsl-runtime-proof',
    proofId: 'app-game-linux-wsl-runtime-proof-ubuntu',
    targetKind: 'wsl2-distro',
    runtimeState: 'runtime-observed',
    distroRef: 'linux-wsl-distro-ref',
    distroId: 'ubuntu',
    distroVersion: '22.04',
    kernelRelease: '5.15.167.4-microsoft-standard-WSL2',
    architecture: 'x86_64',
    packageManagerVisibleCount: 100,
    processSnapshotCount: 5,
    systemdSessionState: 'systemd-session-observed',
    displayState: 'wslg-display-observed',
    x11SocketState: 'socket-observed',
    waylandSocketState: 'socket-observed',
    foregroundProbeState: 'active-window-tool-missing',
    dockerState: 'docker-cli-unavailable',
    proofRefs: [
      'linux-wsl-distro-ref',
      'linux-wsl-kernel-ref',
      'linux-wsl-package-manager-ref',
      'linux-wsl-process-ref',
      'linux-wsl-session-ref',
      'linux-wslg-display-ref',
      'linux-wslg-x11-socket-ref',
      'linux-wslg-wayland-socket-ref',
      'linux-docker-cli-ref',
    ],
    packageNamesRedacted: true,
    processNamesRedacted: true,
    rawDistroNameRedacted: true,
    mechanismProofAttached: true,
    distroProofAttached: true,
    sessionProofAttached: true,
    displayProofAttached: true,
    rollbackProofAttached: false,
    auditProofAttached: false,
    foregroundCaptureClaimed: false,
    adapterDispatchClaimed: false,
    broadBlockingClaimed: false,
    platformEnforcementClaimed: false,
    parentVisibleSummary:
      'WSL2 Ubuntu runtime, package manager, process, and systemd-session facts are observed; Linux broad blocking remains unavailable until rollback and audit proof are attached.',
    checkedAt: '2026-06-08T16:10:00.000Z',
  };
}

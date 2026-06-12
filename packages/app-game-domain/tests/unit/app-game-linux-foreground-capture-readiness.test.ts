import { describe, expect, it } from 'vitest';
import { decodeAppGameLinuxWslRuntimeProof } from '../../src/app-game-linux-wsl-runtime-proof';
import {
  AppGameLinuxForegroundCaptureReadinessSchema,
  createAppGameLinuxForegroundCaptureReadiness,
  summarizeAppGameLinuxForegroundCaptureReadiness,
} from '../../src/app-game-linux-foreground-capture-readiness';

describe('app-game Linux foreground capture readiness', () => {
  it('summarizes WSLg display and socket proof as capture-tool-missing readiness', () => {
    const readiness = createAppGameLinuxForegroundCaptureReadiness({
      linuxProof: decodeAppGameLinuxWslRuntimeProof(validWslProof()),
      generatedAt: '2026-06-08T17:45:00.000Z',
    });
    const summary = summarizeAppGameLinuxForegroundCaptureReadiness(readiness);

    expect(summary.readinessState).toBe('display-ready-capture-tool-missing');
    expect(summary.sourceState).toBe('wslg-display-sockets-observed');
    expect(summary.displayProofAttached).toBe(true);
    expect(summary.foregroundToolAvailable).toBe(false);
    expect(summary.foregroundCaptureReady).toBe(false);
    expect(readiness.proofRefs).toEqual(
      expect.arrayContaining([
        'linux-foreground-capture-readiness-ref',
        'linux-wslg-display-ref',
        'linux-wslg-x11-socket-ref',
        'linux-wslg-wayland-socket-ref',
      ])
    );
    expect(readiness.openGaps).toEqual(
      expect.arrayContaining([
        'linux-active-window-tool-not-available',
        'linux-active-window-title-not-captured',
        'linux-foreground-capture-not-proved',
        'linux-platform-enforcement-not-proved',
        'linux-child-device-delivery-not-proved',
      ])
    );
  });

  it('rejects raw window title custody and enforcement claim upgrades', () => {
    const readiness = createAppGameLinuxForegroundCaptureReadiness({
      linuxProof: decodeAppGameLinuxWslRuntimeProof(validWslProof()),
      generatedAt: '2026-06-08T17:45:00.000Z',
    });

    expect(
      AppGameLinuxForegroundCaptureReadinessSchema.safeParse({
        ...readiness,
        rawWindowTitleClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameLinuxForegroundCaptureReadinessSchema.safeParse({
        ...readiness,
        foregroundCaptureReady: true,
      }).success
    ).toBe(false);
    expect(
      AppGameLinuxForegroundCaptureReadinessSchema.safeParse({
        ...readiness,
        platformEnforcementClaimed: true,
      }).success
    ).toBe(false);
  });

  it('keeps display-not-ready when WSLg display proof is absent', () => {
    const readiness = createAppGameLinuxForegroundCaptureReadiness({
      linuxProof: decodeAppGameLinuxWslRuntimeProof(displayNotReadyProof()),
      generatedAt: '2026-06-08T17:45:00.000Z',
    });

    expect(readiness.readinessState).toBe('display-not-ready');
    expect(readiness.sourceState).toBe('source-not-available');
    expect(readiness.proofRefs).toEqual(['linux-foreground-capture-readiness-ref']);
  });
});

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

function displayNotReadyProof() {
  return {
    ...validWslProof(),
    displayState: 'display-not-proved',
    x11SocketState: 'socket-not-proved',
    waylandSocketState: 'socket-not-proved',
    foregroundProbeState: 'active-window-not-proved',
    proofRefs: [
      'linux-wsl-distro-ref',
      'linux-wsl-kernel-ref',
      'linux-wsl-package-manager-ref',
      'linux-wsl-process-ref',
      'linux-wsl-session-ref',
      'linux-docker-cli-ref',
    ],
    displayProofAttached: false,
  };
}

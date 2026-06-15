import { describe, expect, it } from 'vitest';
import { decodeAppGameLinuxWslRuntimeProof } from '../../src/app-game-linux-wsl-runtime-proof';
import {
  createAppGameLinuxForegroundCaptureReadiness,
  type AppGameLinuxForegroundCaptureReadiness,
} from '../../src/app-game-linux-foreground-capture-readiness';
import {
  AppGameLinuxForegroundSourcePreflightReadModelSchema,
  createAppGameLinuxForegroundSourcePreflightReadModel,
  summarizeAppGameLinuxForegroundSourcePreflightReadModel,
} from '../../src/app-game-linux-foreground-source-preflight';

describe('app-game Linux foreground source preflight', () => {
  it('reports WSLg display ready but active-window tool install required on this host', () => {
    const readModel = createAppGameLinuxForegroundSourcePreflightReadModel({
      readiness: createAppGameLinuxForegroundCaptureReadiness({
        linuxProof: decodeAppGameLinuxWslRuntimeProof(wslgToolMissingProof()),
        generatedAt: '2026-06-08T17:46:00.000Z',
      }),
      generatedAt: '2026-06-08T17:47:00.000Z',
    });
    const summary = summarizeAppGameLinuxForegroundSourcePreflightReadModel(readModel);

    expect(summary.preflightState).toBe('foreground-tool-install-required');
    expect(summary.displayProofAttached).toBe(true);
    expect(summary.foregroundToolAvailable).toBe(false);
    expect(summary.foregroundSourcePreflightReady).toBe(false);
    expect(readModel.proofRefs).toEqual(
      expect.arrayContaining([
        'linux-foreground-source-preflight-ref',
        'linux-foreground-capture-readiness-ref',
        'linux-wslg-display-ref',
        'linux-wslg-x11-socket-ref',
        'linux-wslg-wayland-socket-ref',
      ])
    );
    expect(readModel.openGaps).toEqual(
      expect.arrayContaining([
        'linux-active-window-tool-not-available',
        'linux-active-window-title-not-captured',
        'linux-foreground-capture-not-proved',
      ])
    );
  });

  it('marks foreground source preflight ready only when display and tool are available', () => {
    const readModel = createAppGameLinuxForegroundSourcePreflightReadModel({
      readiness: foregroundToolAvailableReadiness(),
      generatedAt: '2026-06-08T17:47:00.000Z',
    });

    expect(readModel.preflightState).toBe('foreground-source-preflight-ready');
    expect(readModel.foregroundSourcePreflightReady).toBe(true);
    expect(readModel.rawWindowTitleCaptured).toBe(false);
    expect(readModel.foregroundCaptureClaimed).toBe(false);
  });

  it('rejects raw title capture foreground capture and enforcement overclaims', () => {
    const readModel = createAppGameLinuxForegroundSourcePreflightReadModel({
      readiness: createAppGameLinuxForegroundCaptureReadiness({
        linuxProof: decodeAppGameLinuxWslRuntimeProof(wslgToolMissingProof()),
        generatedAt: '2026-06-08T17:46:00.000Z',
      }),
      generatedAt: '2026-06-08T17:47:00.000Z',
    });

    expect(
      AppGameLinuxForegroundSourcePreflightReadModelSchema.safeParse({
        ...readModel,
        rawWindowTitleCaptured: true,
      }).success
    ).toBe(false);
    expect(
      AppGameLinuxForegroundSourcePreflightReadModelSchema.safeParse({
        ...readModel,
        foregroundCaptureClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameLinuxForegroundSourcePreflightReadModelSchema.safeParse({
        ...readModel,
        platformEnforcementClaimed: true,
      }).success
    ).toBe(false);
  });
});

function wslgToolMissingProof() {
  return {
    schemaVersion: 'app-game-linux-wsl-runtime-proof',
    proofId: 'app-game-linux-wsl-runtime-proof-local',
    targetKind: 'wsl2-distro',
    runtimeState: 'runtime-observed',
    distroRef: 'linux-wsl-distro-ref',
    distroId: 'ubuntu-redacted',
    distroVersion: '22.04-redacted',
    kernelRelease: '5.15.167.4-microsoft-standard-WSL2',
    architecture: 'x86_64',
    packageManagerVisibleCount: 1,
    processSnapshotCount: 1,
    systemdSessionState: 'session-not-proved',
    displayState: 'wslg-display-observed',
    x11SocketState: 'socket-observed',
    waylandSocketState: 'socket-observed',
    foregroundProbeState: 'active-window-tool-missing',
    dockerState: 'docker-cli-unavailable',
    displayProofAttached: true,
    packageNamesRedacted: true,
    processNamesRedacted: true,
    rawDistroNameRedacted: true,
    mechanismProofAttached: false,
    distroProofAttached: false,
    sessionProofAttached: false,
    rollbackProofAttached: false,
    auditProofAttached: false,
    foregroundCaptureClaimed: false,
    adapterDispatchClaimed: false,
    broadBlockingClaimed: false,
    platformEnforcementClaimed: false,
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
    parentVisibleSummary:
      'WSL2 and WSLg display are visible for Linux runtime proof; active foreground capture remains blocked until a foreground tool is installed and title custody is proved.',
    checkedAt: '2026-06-08T17:16:00.000Z',
  };
}

function foregroundToolAvailableReadiness(): AppGameLinuxForegroundCaptureReadiness {
  return {
    schemaVersion: 'app-game-linux-foreground-capture-readiness',
    readModelId: 'linux-foreground-capture-readiness-ref',
    generatedAt: '2026-06-08T17:46:00.000Z',
    sourceProofId: 'app-game-linux-wsl-runtime-proof-local',
    readinessState: 'foreground-capture-not-proved',
    sourceState: 'wslg-display-sockets-observed',
    custodyState: 'no-window-title-custody',
    displayProofAttached: true,
    x11SocketObserved: true,
    waylandSocketObserved: true,
    foregroundToolAvailable: true,
    foregroundCaptureReady: false,
    rawWindowTitleClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    childDeviceDeliveryClaimed: false,
    proofRefs: [
      'linux-foreground-capture-readiness-ref',
      'linux-wslg-display-ref',
      'linux-wslg-x11-socket-ref',
      'linux-wslg-wayland-socket-ref',
    ],
    openGaps: [
      'linux-active-window-title-not-captured',
      'linux-foreground-capture-not-proved',
      'linux-platform-enforcement-not-proved',
      'linux-child-device-delivery-not-proved',
    ],
    parentVisibleSummary:
      'Linux WSLg display and socket readiness is visible, but active foreground capture remains unproved until a real foreground source is attached without raw title custody.',
  };
}

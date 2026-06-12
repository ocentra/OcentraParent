import { describe, expect, it } from 'vitest';
import { createAppGameAndroidAccessibilityOverlayPreflightReadModel } from '../../src/app-game-android-accessibility-overlay-preflight';
import { createAppGameAndroidAuthorityPreflightReadModel } from '../../src/app-game-android-authority-preflight';
import { decodeAppGameAndroidPhysicalDeviceProof } from '../../src/app-game-android-physical-device-proof';
import { createAppGameAndroidUsageEventsReplayReadModel } from '../../src/app-game-android-usage-events-replay';
import { createAppGameAppleCiPlatformProofPreflightReadModel } from '../../src/app-game-apple-ci-platform-proof-preflight';
import { createAppGameLinuxDockerHostPreflightReadModel } from '../../src/app-game-linux-docker-host-preflight';
import { createAppGameLinuxForegroundCaptureReadiness } from '../../src/app-game-linux-foreground-capture-readiness';
import { decodeAppGameLinuxWslRuntimeProof } from '../../src/app-game-linux-wsl-runtime-proof';
import {
  AppGamePlatformProofStatusReadModelSchema,
  createAppGamePlatformProofStatusReadModel,
  summarizeAppGamePlatformProofStatus,
} from '../../src/app-game-platform-proof-status';
import { createAppGameWindowsBroadBlockingAuthorityPreflightReadModel } from '../../src/app-game-windows-broad-blocking-authority-preflight';

describe('app-game platform proof status', () => {
  it('summarizes Windows, Android, and Linux proof as visibility-only product status', () => {
    const readModel = createAppGamePlatformProofStatusReadModel({
      androidProof: androidProof(),
      androidAuthorityPreflight: androidAuthorityPreflight(),
      androidAccessibilityOverlayPreflight: androidAccessibilityOverlayPreflight(),
      androidUsageEventsReplay: androidReplay(),
      linuxProof: linuxProof(),
      linuxDockerHostPreflight: linuxDockerHostPreflight(),
      linuxForegroundCaptureReadiness: linuxForegroundReadiness(),
      windowsBroadBlockingAuthorityPreflight: windowsBroadBlockingAuthorityPreflight(),
      appleCiPlatformProofPreflight: appleCiPlatformProofPreflight(),
      generatedAt: '2026-06-08T16:20:00.000Z',
    });
    const summary = summarizeAppGamePlatformProofStatus(readModel);

    expect(summary.platformProofObservedCount).toBe(5);
    expect(summary.visibilityOnlyCount).toBe(5);
    expect(summary.enforcementReadyCount).toBe(0);
    expect(summary.openGapCount).toBeGreaterThan(0);
    expect(summary.platforms).toEqual(['windows', 'android', 'linux', 'macos', 'ios']);
    expectPlatformRows(readModel.rows);
    expectPlatformGaps(readModel.rows);
  });

  it('rejects platform status rows that upgrade visibility proof into enforcement claims', () => {
    const readModel = createAppGamePlatformProofStatusReadModel({
      androidProof: androidProof(),
      androidAuthorityPreflight: androidAuthorityPreflight(),
      androidAccessibilityOverlayPreflight: androidAccessibilityOverlayPreflight(),
      androidUsageEventsReplay: androidReplay(),
      linuxProof: linuxProof(),
      linuxDockerHostPreflight: linuxDockerHostPreflight(),
      linuxForegroundCaptureReadiness: linuxForegroundReadiness(),
      windowsBroadBlockingAuthorityPreflight: windowsBroadBlockingAuthorityPreflight(),
      appleCiPlatformProofPreflight: appleCiPlatformProofPreflight(),
      generatedAt: '2026-06-08T16:20:00.000Z',
    });

    expect(
      AppGamePlatformProofStatusReadModelSchema.safeParse({
        ...readModel,
        rows: [
          {
            ...readModel.rows[0],
            platformEnforcementClaimed: true,
          },
          readModel.rows[1],
          readModel.rows[2],
          readModel.rows[3],
          readModel.rows[4],
        ],
      }).success
    ).toBe(false);
  });

  it('rejects summary counts that drift from the rows', () => {
    const readModel = createAppGamePlatformProofStatusReadModel({
      androidProof: androidProof(),
      androidAuthorityPreflight: androidAuthorityPreflight(),
      androidAccessibilityOverlayPreflight: androidAccessibilityOverlayPreflight(),
      androidUsageEventsReplay: androidReplay(),
      linuxProof: linuxProof(),
      linuxDockerHostPreflight: linuxDockerHostPreflight(),
      linuxForegroundCaptureReadiness: linuxForegroundReadiness(),
      windowsBroadBlockingAuthorityPreflight: windowsBroadBlockingAuthorityPreflight(),
      appleCiPlatformProofPreflight: appleCiPlatformProofPreflight(),
      generatedAt: '2026-06-08T16:20:00.000Z',
    });

    expect(
      AppGamePlatformProofStatusReadModelSchema.safeParse({
        ...readModel,
        openGapCount: readModel.openGapCount + 1,
      }).success
    ).toBe(false);
  });
});

function validAndroidPhysicalProof() {
  return {
    schemaVersion: 'app-game-android-physical-device-proof',
    proofId: 'app-game-android-physical-device-proof-s9',
    targetKind: 'physical-device',
    connectionState: 'physical-device-connected',
    adbTargetRef: 'android-physical-adb-device-ref',
    product: 'star2qltecs',
    model: 'SM_G965W',
    device: 'star2qltecs',
    androidRelease: '10',
    sdkInt: 29,
    supportedAbiCount: 4,
    packageManagerVisibleCount: 200,
    usageStatsServiceState: 'service-not-visible',
    usageEventsDumpState: 'usage-events-dump-observed',
    usageEventsSampleCount: 80,
    foregroundActivityEventCount: 12,
    deviceOwnerState: 'not-device-owner',
    profileOwnerState: 'not-profile-owner',
    proofRefs: [
      'android-physical-adb-device-ref',
      'android-physical-build-prop-ref',
      'android-physical-package-manager-ref',
      'android-physical-usage-stats-service-ref',
      'android-physical-usage-events-dump-ref',
      'android-physical-device-policy-ref',
    ],
    packageNamesRedacted: true,
    usageEventsPackageNamesRedacted: true,
    rawDeviceSerialRedacted: true,
    foregroundEvidenceObserved: true,
    hideSuspendClaimed: false,
    adapterDispatchClaimed: false,
    broadBlockingClaimed: false,
    platformEnforcementClaimed: false,
    parentVisibleSummary:
      'Physical Android 10 device is reachable for package and policy-state proof; normal-mode hide/suspend remains blocked until Device Owner or Profile Owner proof is attached.',
    checkedAt: '2026-06-08T15:55:00.000Z',
  };
}

function expectPlatformRows(rows: readonly unknown[]) {
  expect(rows).toEqual(
    expect.arrayContaining([
      expect.objectContaining({
        platform: 'windows',
        proofState: 'windows-policy-preflight-observed',
        authorityState: 'visibility-only',
        mechanismProofAttached: false,
        rollbackProofAttached: false,
        auditProofAttached: false,
        adapterDispatchClaimed: false,
        broadBlockingClaimed: false,
        platformEnforcementClaimed: false,
        childDeliveryClaimed: false,
        proofRefs: ['windows-broad-blocking-authority-preflight-ref'],
      }),
      expect.objectContaining({
        platform: 'android',
        proofState: 'physical-device-observed',
        authorityState: 'visibility-only',
        adapterDispatchClaimed: false,
        platformEnforcementClaimed: false,
        childDeliveryClaimed: false,
        ownerProofAttached: false,
        runtimeVisibilityCount: 80,
        mechanismProofAttached: true,
        proofRefs: [
          'android-physical-device-proof-ref',
          'android-authority-preflight-ref',
          'android-accessibility-overlay-preflight-ref',
          'android-usage-events-replay-ref',
        ],
      }),
      expect.objectContaining({
        platform: 'linux',
        proofState: 'wsl-runtime-observed',
        authorityState: 'visibility-only',
        mechanismProofAttached: false,
        adapterDispatchClaimed: false,
        platformEnforcementClaimed: false,
        childDeliveryClaimed: false,
        proofRefs: [
          'linux-wsl-runtime-proof-ref',
          'linux-foreground-capture-readiness-ref',
          'linux-docker-host-preflight-ref',
        ],
      }),
      expect.objectContaining({
        platform: 'macos',
        proofState: 'apple-ci-artifacts-required',
        authorityState: 'visibility-only',
        packageVisibilityCount: 1,
        runtimeVisibilityCount: 5,
        adapterDispatchClaimed: false,
        platformEnforcementClaimed: false,
        childDeliveryClaimed: false,
        proofRefs: ['apple-ci-platform-proof-preflight-ref'],
      }),
      expect.objectContaining({
        platform: 'ios',
        proofState: 'apple-ci-artifacts-required',
        authorityState: 'visibility-only',
        packageVisibilityCount: 6,
        runtimeVisibilityCount: 5,
        adapterDispatchClaimed: false,
        platformEnforcementClaimed: false,
        childDeliveryClaimed: false,
        proofRefs: ['apple-ci-platform-proof-preflight-ref'],
      }),
    ])
  );
}

function expectPlatformGaps(rows: readonly { readonly openGaps: readonly string[] }[]) {
  expect(rows[0].openGaps).toEqual(
    expect.arrayContaining([
      'windows-broad-blocking-not-proved',
      'windows-applocker-enforce-not-proved',
      'windows-app-control-not-proved',
      'windows-system-app-allowlist-not-proved',
      'windows-rollback-not-proved',
      'windows-audit-custody-not-proved',
      'cross-platform-child-delivery-not-proved',
    ])
  );
  expect(rows[1].openGaps).toEqual(
    expect.arrayContaining([
      'android-device-owner-not-proved',
      'android-profile-owner-not-proved',
      'android-child-runtime-replay-consumer-not-attached',
      'android-accessibility-overlay-not-proved',
      'android-hide-suspend-not-proved',
      'cross-platform-child-delivery-not-proved',
    ])
  );
  expect(rows[1].openGaps).not.toContain('android-authority-preflight-not-attached');
  expect(rows[1].openGaps).not.toContain('android-usage-events-not-proved');
  expect(rows[1].openGaps).not.toContain('android-durable-usage-events-replay-not-proved');
  expect(rows[2].openGaps).toEqual(
    expect.arrayContaining([
      'linux-policy-mechanism-not-proved',
      'linux-container-policy-not-proved',
      'linux-foreground-capture-not-proved',
      'linux-rollback-not-proved',
      'linux-audit-not-proved',
      'cross-platform-child-delivery-not-proved',
    ])
  );
  expect(rows[3].openGaps).toEqual(
    expect.arrayContaining([
      'macos-ci-runner-not-proved',
      'macos-permission-profile-not-proved',
      'macos-mdm-endpoint-not-proved',
      'macos-rollback-audit-not-proved',
      'apple-platform-adapter-dispatch-blocked-before-ci-proof',
      'cross-platform-child-delivery-not-proved',
    ])
  );
  expect(rows[4].openGaps).toEqual(
    expect.arrayContaining([
      'ios-ci-runner-not-proved',
      'ios-family-controls-not-proved',
      'ios-device-activity-not-proved',
      'ios-managed-settings-not-proved',
      'ios-testflight-device-not-proved',
      'apple-platform-adapter-dispatch-blocked-before-ci-proof',
      'cross-platform-child-delivery-not-proved',
    ])
  );
}

function androidProof() {
  return decodeAppGameAndroidPhysicalDeviceProof(validAndroidPhysicalProof());
}

function androidReplay() {
  return createAppGameAndroidUsageEventsReplayReadModel({
    androidProof: androidProof(),
    generatedAt: '2026-06-08T16:19:00.000Z',
  });
}

function androidAuthorityPreflight() {
  return createAppGameAndroidAuthorityPreflightReadModel({
    androidProof: androidProof(),
    generatedAt: '2026-06-08T16:19:10.000Z',
  });
}

function androidAccessibilityOverlayPreflight() {
  return createAppGameAndroidAccessibilityOverlayPreflightReadModel({
    androidProof: androidProof(),
    accessibilitySettings: {
      accessibilityEnabled: true,
      enabledServiceCount: 1,
      serviceNamesRedacted: true,
      settingsReadable: true,
    },
    generatedAt: '2026-06-08T16:19:20.000Z',
  });
}

function linuxProof() {
  return decodeAppGameLinuxWslRuntimeProof(validLinuxWslProof());
}

function linuxForegroundReadiness() {
  return createAppGameLinuxForegroundCaptureReadiness({
    linuxProof: linuxProof(),
    generatedAt: '2026-06-08T16:19:30.000Z',
  });
}

function linuxDockerHostPreflight() {
  return createAppGameLinuxDockerHostPreflightReadModel({
    generatedAt: '2026-06-08T16:19:35.000Z',
    dockerCliObserved: true,
    dockerDaemonObserved: true,
    contextCount: 1,
    imageCount: 1,
    containerCount: 1,
  });
}

function windowsBroadBlockingAuthorityPreflight() {
  return createAppGameWindowsBroadBlockingAuthorityPreflightReadModel({
    generatedAt: '2026-06-08T16:19:40.000Z',
  });
}

function appleCiPlatformProofPreflight() {
  return createAppGameAppleCiPlatformProofPreflightReadModel({
    generatedAt: '2026-06-08T16:19:50.000Z',
  });
}

function validLinuxWslProof() {
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
    mechanismProofAttached: false,
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
      'WSL2 Ubuntu runtime, package manager, process, and systemd-session facts are observed; Linux broad blocking remains unavailable until mechanism, rollback, and audit proof are attached.',
    checkedAt: '2026-06-08T16:10:00.000Z',
  };
}

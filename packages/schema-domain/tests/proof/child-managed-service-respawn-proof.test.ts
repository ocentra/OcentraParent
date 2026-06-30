import { describe, expect, it } from 'vitest';
import {
  type ChildManagedServiceRespawnReadModel,
  ChildManagedServiceRespawnReadModelSchema,
} from '../../src/child-managed-service-respawn-proof';

describe('child managed service respawn proof contracts', () => {
  acceptsExplicitDesktopRespawnAndMobileLimits();
  rejectsMissingPlatformRows();
  rejectsHidingDesktopStopAsAutomaticRespawn();
  rejectsAndroidRespawnUpgrade();
  rejectsIosUnsupportedUpgrade();
  rejectsParentProofReuseBoundaryRemoval();
});

function acceptsExplicitDesktopRespawnAndMobileLimits(): void {
  it('ChildManagedServiceRespawnReadModelSchema: accepts explicit desktop respawn support and mobile manual or unsupported states', () => {
    const parsed = ChildManagedServiceRespawnReadModelSchema.parse(validReadModel());

    expect(platformState(parsed, 'windows')).toEqual({
      respawnState: 'proved',
      stopRecoveryState: 'manual-required',
      teardownState: 'proved',
    });
    expect(platformState(parsed, 'macos')).toEqual({
      respawnState: 'proved',
      stopRecoveryState: 'manual-required',
      teardownState: 'proved',
    });
    expect(platformState(parsed, 'linux')).toEqual({
      respawnState: 'proved',
      stopRecoveryState: 'manual-required',
      teardownState: 'proved',
    });
    expect(platformState(parsed, 'android')).toEqual({
      respawnState: 'manual-required',
      stopRecoveryState: 'manual-required',
      teardownState: 'manual-required',
    });
    expect(platformState(parsed, 'ios')).toEqual({
      respawnState: 'unsupported',
      stopRecoveryState: 'unsupported',
      teardownState: 'unsupported',
    });
    expect(parsed.claimBoundaries.mobileNoReuse).toContain('Android stays manual-required');
    expect(parsed.claimBoundaries.parentProofSeparation).toContain('Parent client update');
  });
}

function rejectsMissingPlatformRows(): void {
  it('ChildManagedServiceRespawnReadModelSchema: rejects missing platform rows', () => {
    const model = validReadModel();

    expect(
      ChildManagedServiceRespawnReadModelSchema.safeParse({
        ...model,
        platformProofs: model.platformProofs.filter((entry) => entry.platform !== 'linux'),
      }).success
    ).toBe(false);
  });
}

function rejectsHidingDesktopStopAsAutomaticRespawn(): void {
  it('ChildManagedServiceRespawnReadModelSchema: rejects treating deliberate desktop stop as automatic respawn', () => {
    const model = validReadModel();

    expect(
      ChildManagedServiceRespawnReadModelSchema.safeParse({
        ...model,
        platformProofs: model.platformProofs.map((entry) =>
          entry.platform === 'windows' ? { ...entry, stopRecoveryState: 'proved' } : entry
        ),
      }).success
    ).toBe(false);
  });
}

function rejectsAndroidRespawnUpgrade(): void {
  it('ChildManagedServiceRespawnReadModelSchema: rejects Android respawn upgrades without device proof', () => {
    const model = validReadModel();

    expect(
      ChildManagedServiceRespawnReadModelSchema.safeParse({
        ...model,
        platformProofs: model.platformProofs.map((entry) =>
          entry.platform === 'android'
            ? {
                ...entry,
                proofState: 'ci-mechanical-proof',
                respawnState: 'proved',
                restartSurvivalState: 'proved',
                killRecoveryState: 'proved',
                rebootRecoveryState: 'proved',
              }
            : entry
        ),
      }).success
    ).toBe(false);
  });
}

function rejectsIosUnsupportedUpgrade(): void {
  it('ChildManagedServiceRespawnReadModelSchema: rejects iOS managed-service support upgrades', () => {
    const model = validReadModel();

    expect(
      ChildManagedServiceRespawnReadModelSchema.safeParse({
        ...model,
        platformProofs: model.platformProofs.map((entry) =>
          entry.platform === 'ios'
            ? {
                ...entry,
                proofState: 'manual-required',
                respawnState: 'manual-required',
                claimBoundary: 'iOS managed service respawn is available through desktop proof reuse',
              }
            : entry
        ),
      }).success
    ).toBe(false);
  });
}

function rejectsParentProofReuseBoundaryRemoval(): void {
  it('ChildManagedServiceRespawnReadModelSchema: rejects parent-client proof reuse boundaries being removed', () => {
    const model = validReadModel();

    expect(
      ChildManagedServiceRespawnReadModelSchema.safeParse({
        ...model,
        claimBoundaries: {
          ...model.claimBoundaries,
          parentProofSeparation: 'Parent client proof closes child respawn claims.',
        },
      }).success
    ).toBe(false);
  });
}

function validReadModel(): ChildManagedServiceRespawnReadModel {
  return ChildManagedServiceRespawnReadModelSchema.parse({
    schemaVersion: 'child-managed-service-respawn-proof',
    platformProofs: [
      platformProof(
        'windows',
        'winsw-service',
        'ci-mechanical-proof',
        'windows-service-installer',
        'proved',
        'proved',
        'proved',
        'manual-required',
        'proved',
        'proved',
        'proved',
        'WinSW service XML, WiX service install, and Windows lifecycle cleanup prove auto-start, failure restart, and uninstall stop/remove boundaries for the child agent service.',
        'Windows MSI uninstall stops and removes the child agent service, then lifecycle cleanup asserts no child agent service processes remain.',
        'Windows respawn proof is service-manager configuration only; it does not claim live post-install runtime health beyond the named package lifecycle surfaces.',
        [
          'scripts/release/windows/OcentraParentAgentService.xml',
          'scripts/release/windows/OcentraParentAgent.wxs',
          'scripts/release/windows/package-lifecycle-host.mjs',
        ]
      ),
      platformProof(
        'macos',
        'launchd-daemon',
        'ci-mechanical-proof',
        'macos-launchd-package',
        'proved',
        'proved',
        'proved',
        'manual-required',
        'proved',
        'proved',
        'proved',
        'launchd plist and package install scripts prove RunAtLoad, KeepAlive, bootstrap, and enable behavior for the child agent daemon.',
        'macOS package preinstall and postinstall bootout commands make the stop path explicit instead of hiding teardown behind respawn language.',
        'macOS respawn proof is launchd/package configuration only; it does not claim notarization, live host install success, or non-launchd runtime health.',
        ['scripts/release/macos/build-agent-package.sh', 'scripts/release/macos/ca.ocentra.parent.agent.plist']
      ),
      platformProof(
        'linux',
        'systemd-service',
        'ci-mechanical-proof',
        'linux-systemd-package',
        'proved',
        'proved',
        'proved',
        'manual-required',
        'proved',
        'proved',
        'proved',
        'systemd unit and Debian package scripts prove Restart=always, boot enablement, and restart wiring for the child agent service.',
        'Linux package prerm and postrm scripts stop, disable, and reload systemd so the stop path remains explicit and testable.',
        'Linux respawn proof is systemd/package configuration only; it does not claim non-systemd hosts, baseline portability, or live runtime health beyond this slice.',
        ['scripts/release/linux/build-agent-package.sh', 'scripts/release/linux/ocentra-parent-agent.service']
      ),
      platformProof(
        'android',
        'android-foreground-service',
        'manual-required',
        'android-device-proof',
        'manual-required',
        'manual-required',
        'manual-required',
        'manual-required',
        'manual-required',
        'unsupported',
        'manual-required',
        'Android child-agent package proof keeps foreground service and reboot recovery manual-required until emulator or physical-device artifacts exist.',
        'Android stop, reboot, uninstall, and restart survival remain manual-required because this slice has no real device lifecycle artifacts.',
        'Android does not reuse desktop service-manager proof; foreground-service runtime parity stays manual-required until device proof exists.',
        ['platforms/android/README.md', 'packages/schema-domain/src/child-android-lifecycle-proof.ts']
      ),
      platformProof(
        'ios',
        'ios-capability-surface',
        'unsupported',
        'ios-capability-package',
        'unsupported',
        'unsupported',
        'unsupported',
        'unsupported',
        'unsupported',
        'unsupported',
        'unsupported',
        'iOS child-agent proof is capability-only; no persistent background daemon or managed service respawn surface is claimed.',
        'iOS capability packaging does not expose a managed service stop or respawn path in this slice.',
        'iOS cannot reuse desktop service-manager or Android foreground-service proof; managed service respawn is unsupported here.',
        [
          'platforms/ios/README.md',
          'platforms/ios/OcentraParentAgent/AgentStatusViewController.swift',
          'packages/schema-domain/src/child-ios-entitlement-capability-proof.ts',
        ]
      ),
    ],
    claimBoundaries: {
      desktopServiceManagers:
        'Only Windows WinSW, macOS launchd, and Linux systemd rows claim managed respawn in this slice.',
      stopPathNegativeCases:
        'Supported desktop rows keep deliberate stop and teardown paths explicit instead of treating them as silent respawn.',
      mobileNoReuse:
        'Android stays manual-required and iOS stays unsupported; mobile rows do not inherit desktop respawn support.',
      parentProofSeparation:
        'Parent client update, rollback, installer, or release proofs do not close child managed service respawn claims.',
      runtimeHealthSeparation:
        'Service-manager configuration proof does not claim live post-install runtime health beyond the named package and lifecycle surfaces.',
    },
    updatedAt: '2026-06-28T00:00:00.000Z',
  });
}

function platformProof(
  platform: string,
  supervisor: string,
  proofState: string,
  runtimeOwner: string,
  respawnState: string,
  restartSurvivalState: string,
  killRecoveryState: string,
  stopRecoveryState: string,
  rebootRecoveryState: string,
  serviceManagerRestartState: string,
  teardownState: string,
  proofRequirement: string,
  teardownRequirement: string,
  claimBoundary: string,
  sourceRefs: ReadonlyArray<string>
) {
  return {
    platform,
    supervisor,
    proofState,
    runtimeOwner,
    respawnState,
    restartSurvivalState,
    killRecoveryState,
    stopRecoveryState,
    rebootRecoveryState,
    serviceManagerRestartState,
    teardownState,
    proofRequirement,
    teardownRequirement,
    claimBoundary,
    sourceRefs,
  };
}

function platformState(
  model: ChildManagedServiceRespawnReadModel,
  platform: ChildManagedServiceRespawnReadModel['platformProofs'][number]['platform']
) {
  const entry = model.platformProofs.find((proof) => proof.platform === platform);
  return {
    respawnState: entry?.respawnState,
    stopRecoveryState: entry?.stopRecoveryState,
    teardownState: entry?.teardownState,
  };
}

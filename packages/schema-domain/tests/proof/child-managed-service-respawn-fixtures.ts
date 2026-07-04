export const managedServiceRespawnReadModelInput = {
  schemaVersion: 'child-managed-service-respawn-proof',
  platformProofs: [
    {
      platform: 'windows',
      supervisor: 'winsw-service',
      proofState: 'ci-mechanical-proof',
      runtimeOwner: 'windows-service-installer',
      respawnState: 'proved',
      restartSurvivalState: 'proved',
      killRecoveryState: 'proved',
      stopRecoveryState: 'manual-required',
      rebootRecoveryState: 'proved',
      serviceManagerRestartState: 'proved',
      teardownState: 'proved',
      proofRequirement:
        'WinSW service XML, WiX service install, and Windows lifecycle cleanup prove auto-start, failure restart, and uninstall stop/remove boundaries for the child agent service.',
      teardownRequirement:
        'Windows MSI uninstall stops and removes the child agent service, then lifecycle cleanup asserts no child agent service processes remain.',
      claimBoundary:
        'Windows respawn proof is service-manager configuration only; it does not claim live post-install runtime health beyond the named package lifecycle surfaces.',
      sourceRefs: [
        'scripts/release/windows/OcentraParentAgentService.xml',
        'scripts/release/windows/OcentraParentAgent.wxs',
        'scripts/release/windows/package-lifecycle-host.mjs',
      ],
    },
    {
      platform: 'macos',
      supervisor: 'launchd-daemon',
      proofState: 'ci-mechanical-proof',
      runtimeOwner: 'macos-launchd-package',
      respawnState: 'proved',
      restartSurvivalState: 'proved',
      killRecoveryState: 'proved',
      stopRecoveryState: 'manual-required',
      rebootRecoveryState: 'proved',
      serviceManagerRestartState: 'proved',
      teardownState: 'proved',
      proofRequirement:
        'launchd plist and package install scripts prove RunAtLoad, KeepAlive, bootstrap, and enable behavior for the child agent daemon.',
      teardownRequirement:
        'macOS package preinstall and postinstall bootout commands make the stop path explicit instead of hiding teardown behind respawn language.',
      claimBoundary:
        'macOS respawn proof is launchd/package configuration only; it does not claim notarization, live host install success, or non-launchd runtime health.',
      sourceRefs: [
        'scripts/release/macos/build-agent-package.sh',
        'scripts/release/macos/ca.ocentra.parent.agent.plist',
      ],
    },
    {
      platform: 'linux',
      supervisor: 'systemd-service',
      proofState: 'ci-mechanical-proof',
      runtimeOwner: 'linux-systemd-package',
      respawnState: 'proved',
      restartSurvivalState: 'proved',
      killRecoveryState: 'proved',
      stopRecoveryState: 'manual-required',
      rebootRecoveryState: 'proved',
      serviceManagerRestartState: 'proved',
      teardownState: 'proved',
      proofRequirement:
        'systemd unit and Debian package scripts prove Restart=always, boot enablement, and restart wiring for the child agent service.',
      teardownRequirement:
        'Linux package prerm and postrm scripts stop, disable, and reload systemd so the stop path remains explicit and testable.',
      claimBoundary:
        'Linux respawn proof is systemd/package configuration only; it does not claim non-systemd hosts, baseline portability, or live runtime health beyond this slice.',
      sourceRefs: [
        'scripts/release/linux/build-agent-package.sh',
        'scripts/release/linux/ocentra-parent-agent.service',
      ],
    },
    {
      platform: 'android',
      supervisor: 'android-foreground-service',
      proofState: 'manual-required',
      runtimeOwner: 'android-device-proof',
      respawnState: 'manual-required',
      restartSurvivalState: 'manual-required',
      killRecoveryState: 'manual-required',
      stopRecoveryState: 'manual-required',
      rebootRecoveryState: 'manual-required',
      serviceManagerRestartState: 'unsupported',
      teardownState: 'manual-required',
      proofRequirement:
        'Android child-agent package proof keeps foreground service and reboot recovery manual-required until emulator or physical-device artifacts exist.',
      teardownRequirement:
        'Android stop, reboot, uninstall, and restart survival remain manual-required because this slice has no real device lifecycle artifacts.',
      claimBoundary:
        'Android does not reuse desktop service-manager proof; foreground-service runtime parity stays manual-required until device proof exists.',
      sourceRefs: [
        'platforms/android/README.md',
        'packages/schema-domain/src/child-android-lifecycle-proof.ts',
      ],
    },
    {
      platform: 'ios',
      supervisor: 'ios-capability-surface',
      proofState: 'unsupported',
      runtimeOwner: 'ios-capability-package',
      respawnState: 'unsupported',
      restartSurvivalState: 'unsupported',
      killRecoveryState: 'unsupported',
      stopRecoveryState: 'unsupported',
      rebootRecoveryState: 'unsupported',
      serviceManagerRestartState: 'unsupported',
      teardownState: 'unsupported',
      proofRequirement:
        'iOS child-agent proof is capability-only; no persistent background daemon or managed service respawn surface is claimed.',
      teardownRequirement: 'iOS capability packaging does not expose a managed service stop or respawn path in this slice.',
      claimBoundary:
        'iOS cannot reuse desktop service-manager or Android foreground-service proof; managed service respawn is unsupported here.',
      sourceRefs: [
        'platforms/ios/README.md',
        'platforms/ios/OcentraParentAgent/AgentStatusViewController.swift',
        'packages/schema-domain/src/child-ios-entitlement-capability-proof.ts',
      ],
    },
  ],
  claimBoundaries: {
    desktopServiceManagers: 'Only Windows WinSW, macOS launchd, and Linux systemd rows claim managed respawn in this slice.',
    stopPathNegativeCases:
      'Supported desktop rows keep deliberate stop and teardown paths explicit instead of treating them as silent respawn.',
    mobileNoReuse: 'Android stays manual-required and iOS stays unsupported; mobile rows do not inherit desktop respawn support.',
    parentProofSeparation:
      'Parent client update, rollback, installer, or release proofs do not close child managed service respawn claims.',
    runtimeHealthSeparation:
      'Service-manager configuration proof does not claim live post-install runtime health beyond the named package and lifecycle surfaces.',
  },
  updatedAt: '2026-06-28T00:00:00.000Z',
} as const;

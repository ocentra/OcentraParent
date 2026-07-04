import { type Infer, NonEmptyStringSchema, Schema, withParser } from './effect';
import { ParentTimestampSchema } from './family-reference-primitives';

export const ChildManagedServiceRespawnProofSchemaVersionSchema = withParser(
  Schema.Literal('child-managed-service-respawn-proof')
);
export const ChildManagedServiceRespawnPlatformSchema = withParser(
  Schema.Literal('windows', 'macos', 'linux', 'android', 'ios')
);
export const ChildManagedServiceRespawnSupervisorSchema = withParser(
  Schema.Literal(
    'winsw-service',
    'launchd-daemon',
    'systemd-service',
    'android-foreground-service',
    'ios-capability-surface'
  )
);
export const ChildManagedServiceRespawnProofStateSchema = withParser(
  Schema.Literal('ci-mechanical-proof', 'manual-required', 'unsupported')
);
export const ChildManagedServiceRecoveryStateSchema = withParser(
  Schema.Literal('proved', 'manual-required', 'unsupported')
);
export const ChildManagedServiceRespawnRuntimeOwnerSchema = withParser(
  Schema.Literal(
    'windows-service-installer',
    'macos-launchd-package',
    'linux-systemd-package',
    'android-device-proof',
    'ios-capability-package'
  )
);

const ChildManagedServiceRespawnBoundarySchema = withParser(NonEmptyStringSchema);
const ChildManagedServiceRespawnSourceRefSchema = withParser(NonEmptyStringSchema);

export const ChildManagedServiceRespawnPlatformProofSchema = withParser(
  Schema.Struct({
    platform: ChildManagedServiceRespawnPlatformSchema,
    supervisor: ChildManagedServiceRespawnSupervisorSchema,
    proofState: ChildManagedServiceRespawnProofStateSchema,
    runtimeOwner: ChildManagedServiceRespawnRuntimeOwnerSchema,
    respawnState: ChildManagedServiceRecoveryStateSchema,
    restartSurvivalState: ChildManagedServiceRecoveryStateSchema,
    killRecoveryState: ChildManagedServiceRecoveryStateSchema,
    stopRecoveryState: ChildManagedServiceRecoveryStateSchema,
    rebootRecoveryState: ChildManagedServiceRecoveryStateSchema,
    serviceManagerRestartState: ChildManagedServiceRecoveryStateSchema,
    teardownState: ChildManagedServiceRecoveryStateSchema,
    proofRequirement: ChildManagedServiceRespawnBoundarySchema,
    teardownRequirement: ChildManagedServiceRespawnBoundarySchema,
    claimBoundary: ChildManagedServiceRespawnBoundarySchema,
    sourceRefs: Schema.Array(ChildManagedServiceRespawnSourceRefSchema),
  })
);

export const ChildManagedServiceRespawnClaimBoundariesSchema = withParser(
  Schema.Struct({
    desktopServiceManagers: ChildManagedServiceRespawnBoundarySchema,
    stopPathNegativeCases: ChildManagedServiceRespawnBoundarySchema,
    mobileNoReuse: ChildManagedServiceRespawnBoundarySchema,
    parentProofSeparation: ChildManagedServiceRespawnBoundarySchema,
    runtimeHealthSeparation: ChildManagedServiceRespawnBoundarySchema,
  })
);

const ChildManagedServiceRespawnReadModelBaseSchema = Schema.Struct({
  schemaVersion: ChildManagedServiceRespawnProofSchemaVersionSchema,
  platformProofs: Schema.Array(ChildManagedServiceRespawnPlatformProofSchema),
  claimBoundaries: ChildManagedServiceRespawnClaimBoundariesSchema,
  updatedAt: ParentTimestampSchema,
});

type ChildManagedServiceRespawnReadModelCandidate = Infer<typeof ChildManagedServiceRespawnReadModelBaseSchema>;

export const ChildManagedServiceRespawnReadModelSchema = withParser(
  ChildManagedServiceRespawnReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        childManagedServiceRespawnReadModelIsHonest(readModel) ||
        'Expected child managed service respawn proof to limit support to Windows WinSW, macOS launchd, and Linux systemd service-manager surfaces while keeping deliberate stop recovery explicit, Android manual-required, iOS unsupported, mobile non-reuse, and parent-proof separation visible'
    )
  )
);

const RequiredPlatforms = [
  'windows',
  'macos',
  'linux',
  'android',
  'ios',
] as const satisfies ReadonlyArray<ChildManagedServiceRespawnPlatform>;

const ExpectedClaimBoundaries = {
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
} as const satisfies Record<keyof ChildManagedServiceRespawnClaimBoundaries, string>;

type ExpectedPlatformProof = Omit<ChildManagedServiceRespawnPlatformProof, 'platform'>;

const ExpectedPlatformProofs = {
  windows: {
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
  macos: {
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
    sourceRefs: ['scripts/release/macos/build-agent-package.sh', 'scripts/release/macos/ca.ocentra.parent.agent.plist'],
  },
  linux: {
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
    sourceRefs: ['scripts/release/linux/build-agent-package.sh', 'scripts/release/linux/ocentra-parent-agent.service'],
  },
  android: {
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
    sourceRefs: ['platforms/android/README.md', 'packages/schema-domain/src/child-android-lifecycle-proof.ts'],
  },
  ios: {
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
} as const satisfies Record<ChildManagedServiceRespawnPlatform, ExpectedPlatformProof>;

function childManagedServiceRespawnReadModelIsHonest(readModel: ChildManagedServiceRespawnReadModelCandidate): boolean {
  const byPlatform = new Map(readModel.platformProofs.map((proof) => [proof.platform, proof] as const));

  return (
    byPlatform.size === readModel.platformProofs.length &&
    RequiredPlatforms.every((platform) => platformProofMatches(byPlatform.get(platform), platform)) &&
    claimBoundariesAreHonest(readModel.claimBoundaries)
  );
}

function platformProofMatches(
  proof: ChildManagedServiceRespawnPlatformProof | undefined,
  platform: ChildManagedServiceRespawnPlatform
): boolean {
  const expected = ExpectedPlatformProofs[platform];
  return (
    proof !== undefined &&
    platformProofIdentityMatches(proof, platform, expected) &&
    platformProofStateMatches(proof, expected) &&
    platformProofRequirementsMatch(proof, expected)
  );
}

function platformProofIdentityMatches(
  proof: ChildManagedServiceRespawnPlatformProof,
  platform: ChildManagedServiceRespawnPlatform,
  expected: (typeof ExpectedPlatformProofs)[ChildManagedServiceRespawnPlatform]
): boolean {
  return (
    proof.platform === platform &&
    proof.supervisor === expected.supervisor &&
    proof.runtimeOwner === expected.runtimeOwner &&
    proof.respawnState === expected.respawnState &&
    proof.restartSurvivalState === expected.restartSurvivalState &&
    proof.killRecoveryState === expected.killRecoveryState &&
    proof.stopRecoveryState === expected.stopRecoveryState
  );
}

function platformProofStateMatches(
  proof: ChildManagedServiceRespawnPlatformProof,
  expected: (typeof ExpectedPlatformProofs)[ChildManagedServiceRespawnPlatform]
): boolean {
  return (
    proof.proofState === expected.proofState &&
    proof.serviceManagerRestartState === expected.serviceManagerRestartState &&
    proof.teardownState === expected.teardownState
  );
}

function platformProofRequirementsMatch(
  proof: ChildManagedServiceRespawnPlatformProof,
  expected: (typeof ExpectedPlatformProofs)[ChildManagedServiceRespawnPlatform]
): boolean {
  return (
    proof.proofRequirement === expected.proofRequirement &&
    proof.teardownRequirement === expected.teardownRequirement &&
    proof.claimBoundary === expected.claimBoundary &&
    requiredValuesArePresent(proof.sourceRefs, expected.sourceRefs)
  );
}

function claimBoundariesAreHonest(boundaries: ChildManagedServiceRespawnClaimBoundaries): boolean {
  return Object.entries(ExpectedClaimBoundaries).every(
    ([key, value]) => boundaries[key as keyof ChildManagedServiceRespawnClaimBoundaries] === value
  );
}

function requiredValuesArePresent<Value extends string>(
  values: ReadonlyArray<Value>,
  required: ReadonlyArray<Value>
): boolean {
  const valueSet = new Set(values);
  return valueSet.size === values.length && required.every((value) => valueSet.has(value));
}

export type ChildManagedServiceRespawnPlatform = Infer<typeof ChildManagedServiceRespawnPlatformSchema>;
export type ChildManagedServiceRespawnSupervisor = Infer<typeof ChildManagedServiceRespawnSupervisorSchema>;
export type ChildManagedServiceRespawnProofState = Infer<typeof ChildManagedServiceRespawnProofStateSchema>;
export type ChildManagedServiceRecoveryState = Infer<typeof ChildManagedServiceRecoveryStateSchema>;
export type ChildManagedServiceRespawnRuntimeOwner = Infer<typeof ChildManagedServiceRespawnRuntimeOwnerSchema>;
export type ChildManagedServiceRespawnPlatformProof = Infer<typeof ChildManagedServiceRespawnPlatformProofSchema>;
export type ChildManagedServiceRespawnClaimBoundaries = Infer<typeof ChildManagedServiceRespawnClaimBoundariesSchema>;
export type ChildManagedServiceRespawnReadModel = Infer<typeof ChildManagedServiceRespawnReadModelSchema>;

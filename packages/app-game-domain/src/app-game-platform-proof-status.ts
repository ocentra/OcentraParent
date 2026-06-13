import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { type AppGameAndroidAccessibilityOverlayPreflightReadModel } from './app-game-android-accessibility-overlay-preflight';
import { type AppGameAndroidAccessibilityRuntimeProof } from './app-game-android-accessibility-runtime-proof';
import { type AppGameAndroidAuthorityPreflightReadModel } from './app-game-android-authority-preflight';
import { type AppGameAndroidPhysicalDeviceProof } from './app-game-android-physical-device-proof';
import { type AppGameAndroidUsageEventsReplayReadModel } from './app-game-android-usage-events-replay';
import { type AppGameAppleCiPlatformProofPreflightReadModel } from './app-game-apple-ci-platform-proof-preflight';
import { type AppGameLinuxActiveWindowToolProof } from './app-game-linux-active-window-tool-proof';
import { type AppGameLinuxDockerHostPreflightReadModel } from './app-game-linux-docker-host-preflight';
import { type AppGameLinuxForegroundCaptureReadiness } from './app-game-linux-foreground-capture-readiness';
import { type AppGameLinuxWslRuntimeProof } from './app-game-linux-wsl-runtime-proof';
import { type AppGameWindowsBroadBlockingAuthorityPreflightReadModel } from './app-game-windows-broad-blocking-authority-preflight';
import { type AppGameWindowsLocalPolicyEvidenceProof } from './app-game-windows-local-policy-evidence-proof';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';

export const AppGamePlatformProofStatusSchemaVersionSchema = withParser(
  Schema.Literal('app-game-platform-proof-status')
);

export const AppGamePlatformProofStatusPlatformSchema = withParser(
  Schema.Literal('windows', 'android', 'linux', 'macos', 'ios')
);

export const AppGamePlatformProofStatusStateSchema = withParser(
  Schema.Literal(
    'windows-policy-preflight-observed',
    'physical-device-observed',
    'wsl-runtime-observed',
    'apple-ci-artifacts-required'
  )
);

export const AppGamePlatformProofStatusAuthoritySchema = withParser(
  Schema.Literal('visibility-only', 'enforcement-not-proved')
);

export const AppGamePlatformProofStatusGapSchema = withParser(
  Schema.Literal(
    'android-device-owner-not-proved',
    'android-profile-owner-not-proved',
    'android-usage-events-not-proved',
    'android-durable-usage-events-replay-not-proved',
    'android-child-runtime-replay-consumer-not-attached',
    'android-authority-preflight-not-attached',
    'android-accessibility-overlay-not-proved',
    'android-hide-suspend-not-proved',
    'windows-applocker-enforce-not-proved',
    'windows-app-control-not-proved',
    'windows-system-app-allowlist-not-proved',
    'windows-rollback-not-proved',
    'windows-audit-custody-not-proved',
    'windows-broad-blocking-not-proved',
    'macos-ci-runner-not-proved',
    'macos-permission-profile-not-proved',
    'macos-mdm-endpoint-not-proved',
    'macos-rollback-audit-not-proved',
    'ios-ci-runner-not-proved',
    'ios-family-controls-not-proved',
    'ios-device-activity-not-proved',
    'ios-managed-settings-not-proved',
    'ios-testflight-device-not-proved',
    'apple-platform-adapter-dispatch-blocked-before-ci-proof',
    'linux-foreground-capture-not-proved',
    'linux-container-policy-not-proved',
    'linux-native-session-not-proved',
    'linux-policy-mechanism-not-proved',
    'linux-rollback-not-proved',
    'linux-audit-not-proved',
    'cross-platform-child-delivery-not-proved'
  )
);

export const AppGamePlatformProofStatusRefSchema = withParser(
  Schema.Literal(
    'android-physical-device-proof-ref',
    'android-authority-preflight-ref',
    'android-accessibility-overlay-preflight-ref',
    'android-accessibility-runtime-proof-ref',
    'android-usage-events-replay-ref',
    'windows-broad-blocking-authority-preflight-ref',
    'windows-local-policy-evidence-proof-ref',
    'apple-ci-platform-proof-preflight-ref',
    'linux-foreground-capture-readiness-ref',
    'linux-active-window-tool-proof-ref',
    'linux-docker-host-preflight-ref',
    'linux-wsl-runtime-proof-ref',
    'app-game-platform-proof-status-ref'
  )
);

const PlatformProofStatusLabelSchema = brandedNonEmptyStringSchema('AppGamePlatformProofStatusLabel');
const decodePlatformProofStatusLabel = Schema.decodeUnknownSync(PlatformProofStatusLabelSchema);

const PlatformProofStatusCountSchema = Schema.Number.pipe(Schema.int(), Schema.greaterThanOrEqualTo(0));

const AppGamePlatformProofStatusRowBaseSchema = Schema.Struct({
  platform: AppGamePlatformProofStatusPlatformSchema,
  proofState: AppGamePlatformProofStatusStateSchema,
  authorityState: AppGamePlatformProofStatusAuthoritySchema,
  parentVisibleSummary: PlatformProofStatusLabelSchema,
  packageVisibilityCount: PlatformProofStatusCountSchema,
  runtimeVisibilityCount: PlatformProofStatusCountSchema,
  ownerProofAttached: Schema.Boolean,
  mechanismProofAttached: Schema.Boolean,
  rollbackProofAttached: Schema.Boolean,
  auditProofAttached: Schema.Boolean,
  adapterDispatchClaimed: Schema.Boolean,
  broadBlockingClaimed: Schema.Boolean,
  platformEnforcementClaimed: Schema.Boolean,
  childDeliveryClaimed: Schema.Boolean,
  proofRefs: Schema.Array(AppGamePlatformProofStatusRefSchema),
  openGaps: Schema.Array(AppGamePlatformProofStatusGapSchema),
});

const AppGamePlatformProofStatusReadModelBaseSchema = Schema.Struct({
  schemaVersion: AppGamePlatformProofStatusSchemaVersionSchema,
  readModelId: PlatformProofStatusLabelSchema,
  generatedAt: ParentTimestampSchema,
  rows: Schema.Array(AppGamePlatformProofStatusRowBaseSchema),
  platformProofObservedCount: PlatformProofStatusCountSchema,
  visibilityOnlyCount: PlatformProofStatusCountSchema,
  enforcementReadyCount: PlatformProofStatusCountSchema,
  openGapCount: PlatformProofStatusCountSchema,
  productClaim: PlatformProofStatusLabelSchema,
});

type AppGamePlatformProofStatusRowCandidate = Infer<typeof AppGamePlatformProofStatusRowBaseSchema>;
type AppGamePlatformProofStatusReadModelCandidate = Infer<typeof AppGamePlatformProofStatusReadModelBaseSchema>;
type AppGamePlatformProofStatusGap = Infer<typeof AppGamePlatformProofStatusGapSchema>;
type AppGamePlatformProofStatusRef = Infer<typeof AppGamePlatformProofStatusRefSchema>;

export const AppGamePlatformProofStatusRowSchema = withParser(
  AppGamePlatformProofStatusRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        appGamePlatformProofStatusRowIsHonest(row) ||
        'Expected app/game platform proof status rows to expose visibility-only platform proof and keep enforcement, adapter dispatch, broad blocking, and child delivery unclaimed'
    )
  )
);

export const AppGamePlatformProofStatusReadModelSchema = withParser(
  AppGamePlatformProofStatusReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        appGamePlatformProofStatusReadModelCountsMatch(readModel) ||
        'Expected app/game platform proof status summary counts to match the platform proof rows'
    )
  )
);

export type AppGamePlatformProofStatusRow = Infer<typeof AppGamePlatformProofStatusRowSchema>;
export type AppGamePlatformProofStatusReadModel = Infer<typeof AppGamePlatformProofStatusReadModelSchema>;

export const decodeAppGamePlatformProofStatusReadModel = Schema.decodeUnknownSync(
  AppGamePlatformProofStatusReadModelSchema
);

export function createAppGamePlatformProofStatusReadModel(input: {
  readonly androidProof: AppGameAndroidPhysicalDeviceProof;
  readonly androidAuthorityPreflight?: AppGameAndroidAuthorityPreflightReadModel;
  readonly androidAccessibilityOverlayPreflight?: AppGameAndroidAccessibilityOverlayPreflightReadModel;
  readonly androidAccessibilityRuntimeProof?: AppGameAndroidAccessibilityRuntimeProof;
  readonly androidUsageEventsReplay?: AppGameAndroidUsageEventsReplayReadModel;
  readonly linuxForegroundCaptureReadiness?: AppGameLinuxForegroundCaptureReadiness;
  readonly linuxActiveWindowToolProof?: AppGameLinuxActiveWindowToolProof;
  readonly linuxDockerHostPreflight?: AppGameLinuxDockerHostPreflightReadModel;
  readonly linuxProof: AppGameLinuxWslRuntimeProof;
  readonly windowsBroadBlockingAuthorityPreflight?: AppGameWindowsBroadBlockingAuthorityPreflightReadModel;
  readonly windowsLocalPolicyEvidenceProof?: AppGameWindowsLocalPolicyEvidenceProof;
  readonly appleCiPlatformProofPreflight?: AppGameAppleCiPlatformProofPreflightReadModel;
  readonly generatedAt: AppGamePlatformProofStatusReadModel['generatedAt'];
}): AppGamePlatformProofStatusReadModel {
  const rows = statusRows([
    input.windowsBroadBlockingAuthorityPreflight
      ? windowsProofStatusRow(input.windowsBroadBlockingAuthorityPreflight, input.windowsLocalPolicyEvidenceProof)
      : undefined,
    androidProofStatusRow(
      input.androidProof,
      input.androidUsageEventsReplay,
      input.androidAuthorityPreflight,
      input.androidAccessibilityOverlayPreflight,
      input.androidAccessibilityRuntimeProof
    ),
    linuxProofStatusRow(
      input.linuxProof,
      input.linuxForegroundCaptureReadiness,
      input.linuxActiveWindowToolProof,
      input.linuxDockerHostPreflight
    ),
    ...appleCiProofStatusRows(input.appleCiPlatformProofPreflight),
  ]);
  const readModel = {
    schemaVersion: 'app-game-platform-proof-status',
    readModelId: 'app-game-platform-proof-status-ref',
    generatedAt: input.generatedAt,
    rows,
    platformProofObservedCount: rows.length,
    visibilityOnlyCount: rows.filter((row) => row.authorityState === 'visibility-only').length,
    enforcementReadyCount: rows.filter((row) => row.platformEnforcementClaimed).length,
    openGapCount: rows.reduce((total, row) => total + row.openGaps.length, 0),
    productClaim:
      'Windows broad-blocking preflight, Android physical-device/accessibility authority proof, Linux WSL runtime proof, and Apple CI-required macOS/iOS proof rows are parent-visible platform evidence only; native enforcement, broad blocking, rollback, audit, and child delivery remain unclaimed until platform authority proof is attached.',
  };

  return decodeAppGamePlatformProofStatusReadModel(readModel);
}

export function summarizeAppGamePlatformProofStatus(readModel: AppGamePlatformProofStatusReadModel) {
  return {
    platformProofObservedCount: readModel.platformProofObservedCount,
    visibilityOnlyCount: readModel.visibilityOnlyCount,
    enforcementReadyCount: readModel.enforcementReadyCount,
    openGapCount: readModel.openGapCount,
    platforms: readModel.rows.map((row) => row.platform),
  } as const;
}

function androidProofStatusRow(
  proof: AppGameAndroidPhysicalDeviceProof,
  replay?: AppGameAndroidUsageEventsReplayReadModel,
  authorityPreflight?: AppGameAndroidAuthorityPreflightReadModel,
  accessibilityOverlayPreflight?: AppGameAndroidAccessibilityOverlayPreflightReadModel,
  accessibilityRuntimeProof?: AppGameAndroidAccessibilityRuntimeProof
): AppGamePlatformProofStatusRowCandidate {
  return {
    platform: 'android',
    proofState: 'physical-device-observed',
    authorityState: 'visibility-only',
    parentVisibleSummary: platformProofStatusLabel(proof.parentVisibleSummary),
    packageVisibilityCount: proof.packageManagerVisibleCount,
    runtimeVisibilityCount: proof.usageEventsSampleCount,
    ownerProofAttached: androidOwnerProofAttached(proof, authorityPreflight),
    mechanismProofAttached: proof.foregroundEvidenceObserved,
    rollbackProofAttached: false,
    auditProofAttached: false,
    adapterDispatchClaimed: proof.adapterDispatchClaimed,
    broadBlockingClaimed: proof.broadBlockingClaimed,
    platformEnforcementClaimed: proof.platformEnforcementClaimed,
    childDeliveryClaimed: false,
    proofRefs: androidProofRefs(replay, authorityPreflight, accessibilityOverlayPreflight, accessibilityRuntimeProof),
    openGaps: androidOpenGaps(
      proof,
      replay,
      authorityPreflight,
      accessibilityOverlayPreflight,
      accessibilityRuntimeProof
    ),
  };
}

function windowsProofStatusRow(
  proof: AppGameWindowsBroadBlockingAuthorityPreflightReadModel,
  localPolicyEvidence?: AppGameWindowsLocalPolicyEvidenceProof
): AppGamePlatformProofStatusRowCandidate {
  return {
    platform: 'windows',
    proofState: 'windows-policy-preflight-observed',
    authorityState: 'visibility-only',
    parentVisibleSummary: platformProofStatusLabel(proof.parentVisibleSummary),
    packageVisibilityCount: proof.windowsHostProbeAttached ? 1 : 0,
    runtimeVisibilityCount: proof.rows.length + (localPolicyEvidence ? 1 : 0),
    ownerProofAttached: false,
    mechanismProofAttached:
      proof.appLockerProofAttached ||
      proof.appControlProofAttached ||
      localPolicyEvidence?.enforceModeObserved === true ||
      localPolicyEvidence?.appControlEnforcementObserved === true,
    rollbackProofAttached: proof.rollbackProofAttached,
    auditProofAttached: proof.auditCustodyProofAttached,
    adapterDispatchClaimed: proof.adapterDispatchClaimed,
    broadBlockingClaimed: proof.broadBlockingClaimed,
    platformEnforcementClaimed: proof.platformEnforcementClaimed,
    childDeliveryClaimed: false,
    proofRefs: windowsProofRefs(localPolicyEvidence),
    openGaps: windowsOpenGaps(proof, localPolicyEvidence),
  };
}

function appleCiProofStatusRows(
  proof?: AppGameAppleCiPlatformProofPreflightReadModel
): readonly AppGamePlatformProofStatusRowCandidate[] {
  if (!proof) {
    return [];
  }

  return proof.rows.map((row) => ({
    platform: row.platform,
    proofState: 'apple-ci-artifacts-required',
    authorityState: 'visibility-only',
    parentVisibleSummary: platformProofStatusLabel(proof.parentVisibleSummary),
    packageVisibilityCount: row.sourceGateIds.length,
    runtimeVisibilityCount: row.requiredProofRefs.length,
    ownerProofAttached: false,
    mechanismProofAttached: false,
    rollbackProofAttached: false,
    auditProofAttached: false,
    adapterDispatchClaimed: proof.adapterDispatchClaimed,
    broadBlockingClaimed: false,
    platformEnforcementClaimed: proof.platformEnforcementClaimed,
    childDeliveryClaimed: false,
    proofRefs: ['apple-ci-platform-proof-preflight-ref'],
    openGaps: appleCiOpenGaps(row.blockerRefs),
  }));
}

function linuxProofStatusRow(
  proof: AppGameLinuxWslRuntimeProof,
  foregroundReadiness?: AppGameLinuxForegroundCaptureReadiness,
  activeWindowToolProof?: AppGameLinuxActiveWindowToolProof,
  dockerHostPreflight?: AppGameLinuxDockerHostPreflightReadModel
): AppGamePlatformProofStatusRowCandidate {
  return {
    platform: 'linux',
    proofState: 'wsl-runtime-observed',
    authorityState: 'visibility-only',
    parentVisibleSummary: platformProofStatusLabel(proof.parentVisibleSummary),
    packageVisibilityCount: proof.packageManagerVisibleCount,
    runtimeVisibilityCount: proof.processSnapshotCount + (activeWindowToolProof?.toolAvailable ? 1 : 0),
    ownerProofAttached: false,
    mechanismProofAttached: proof.mechanismProofAttached,
    rollbackProofAttached: proof.rollbackProofAttached,
    auditProofAttached: proof.auditProofAttached,
    adapterDispatchClaimed: proof.adapterDispatchClaimed,
    broadBlockingClaimed: proof.broadBlockingClaimed,
    platformEnforcementClaimed: proof.platformEnforcementClaimed,
    childDeliveryClaimed: false,
    proofRefs: linuxProofRefs(foregroundReadiness, activeWindowToolProof, dockerHostPreflight),
    openGaps: linuxOpenGaps(proof, activeWindowToolProof, dockerHostPreflight),
  };
}

function androidProofRefs(
  replay?: AppGameAndroidUsageEventsReplayReadModel,
  authorityPreflight?: AppGameAndroidAuthorityPreflightReadModel,
  accessibilityOverlayPreflight?: AppGameAndroidAccessibilityOverlayPreflightReadModel,
  accessibilityRuntimeProof?: AppGameAndroidAccessibilityRuntimeProof
) {
  const refs: AppGamePlatformProofStatusRef[] = ['android-physical-device-proof-ref'];
  if (authorityPreflight) {
    refs.push('android-authority-preflight-ref');
  }
  if (accessibilityOverlayPreflight) {
    refs.push('android-accessibility-overlay-preflight-ref');
  }
  if (accessibilityRuntimeProof) {
    refs.push('android-accessibility-runtime-proof-ref');
  }
  if (replay?.durableReplayReady) {
    refs.push('android-usage-events-replay-ref');
  }
  return refs;
}

// eslint-disable-next-line complexity -- proof honesty predicates intentionally enumerate required evidence gates.
function androidOpenGaps(
  proof: AppGameAndroidPhysicalDeviceProof,
  replay?: AppGameAndroidUsageEventsReplayReadModel,
  authorityPreflight?: AppGameAndroidAuthorityPreflightReadModel,
  accessibilityOverlayPreflight?: AppGameAndroidAccessibilityOverlayPreflightReadModel,
  accessibilityRuntimeProof?: AppGameAndroidAccessibilityRuntimeProof
) {
  const gaps: AppGamePlatformProofStatusGap[] = ['cross-platform-child-delivery-not-proved'];
  if (proof.deviceOwnerState === 'not-device-owner') {
    gaps.push('android-device-owner-not-proved');
  }
  if (proof.profileOwnerState === 'not-profile-owner') {
    gaps.push('android-profile-owner-not-proved');
  }
  if (proof.usageEventsDumpState !== 'usage-events-dump-observed' || !proof.foregroundEvidenceObserved) {
    gaps.push('android-usage-events-not-proved');
  }
  if (!replay?.durableReplayReady) {
    gaps.push('android-durable-usage-events-replay-not-proved');
  }
  if (replay?.openGaps.includes('android-child-runtime-replay-consumer-not-attached')) {
    gaps.push('android-child-runtime-replay-consumer-not-attached');
  }
  if (!authorityPreflight) {
    gaps.push('android-authority-preflight-not-attached');
  }
  if (
    accessibilityOverlayPreflight?.openBlockers.includes('android-overlay-runtime-not-proved') ||
    accessibilityRuntimeProof?.openGaps.includes('android-accessibility-overlay-runtime-not-proved')
  ) {
    gaps.push('android-accessibility-overlay-not-proved');
  }
  if (!proof.hideSuspendClaimed) {
    gaps.push('android-hide-suspend-not-proved');
  }
  return gaps;
}

function androidOwnerProofAttached(
  proof: AppGameAndroidPhysicalDeviceProof,
  authorityPreflight?: AppGameAndroidAuthorityPreflightReadModel
) {
  return (
    authorityPreflight?.deviceOwnerProofAttached === true ||
    authorityPreflight?.profileOwnerProofAttached === true ||
    (proof.deviceOwnerState !== 'not-device-owner' && proof.deviceOwnerState !== 'not-proved') ||
    (proof.profileOwnerState !== 'not-profile-owner' && proof.profileOwnerState !== 'not-proved')
  );
}

function windowsProofRefs(localPolicyEvidence?: AppGameWindowsLocalPolicyEvidenceProof) {
  const refs: AppGamePlatformProofStatusRef[] = ['windows-broad-blocking-authority-preflight-ref'];
  if (localPolicyEvidence) {
    refs.push('windows-local-policy-evidence-proof-ref');
  }
  return refs;
}

function windowsOpenGaps(
  proof: AppGameWindowsBroadBlockingAuthorityPreflightReadModel,
  localPolicyEvidence?: AppGameWindowsLocalPolicyEvidenceProof
) {
  const gaps: AppGamePlatformProofStatusGap[] = [
    'cross-platform-child-delivery-not-proved',
    'windows-broad-blocking-not-proved',
  ];
  if (
    proof.openBlockers.includes('windows-applocker-enforce-not-proved') ||
    localPolicyEvidence?.openGaps.includes('windows-applocker-enforce-policy-not-observed')
  ) {
    gaps.push('windows-applocker-enforce-not-proved');
  }
  if (
    proof.openBlockers.includes('windows-app-control-not-proved') ||
    localPolicyEvidence?.openGaps.includes('windows-app-control-enforcement-not-observed')
  ) {
    gaps.push('windows-app-control-not-proved');
  }
  if (proof.openBlockers.includes('windows-system-app-allowlist-not-proved')) {
    gaps.push('windows-system-app-allowlist-not-proved');
  }
  if (proof.openBlockers.includes('windows-rollback-not-proved')) {
    gaps.push('windows-rollback-not-proved');
  }
  if (proof.openBlockers.includes('windows-audit-custody-not-proved')) {
    gaps.push('windows-audit-custody-not-proved');
  }
  return gaps;
}

function appleCiOpenGaps(
  blockerRefs: readonly AppGameAppleCiPlatformProofPreflightReadModel['openBlockers'][number][]
) {
  const gaps: AppGamePlatformProofStatusGap[] = ['cross-platform-child-delivery-not-proved'];
  for (const blockerRef of blockerRefs) {
    if (AppGamePlatformProofStatusGapSchema.safeParse(blockerRef).success) {
      gaps.push(blockerRef);
    }
  }
  return gaps;
}

function linuxProofRefs(
  foregroundReadiness?: AppGameLinuxForegroundCaptureReadiness,
  activeWindowToolProof?: AppGameLinuxActiveWindowToolProof,
  dockerHostPreflight?: AppGameLinuxDockerHostPreflightReadModel
) {
  const refs: AppGamePlatformProofStatusRef[] = ['linux-wsl-runtime-proof-ref'];
  if (foregroundReadiness) {
    refs.push('linux-foreground-capture-readiness-ref');
  }
  if (activeWindowToolProof) {
    refs.push('linux-active-window-tool-proof-ref');
  }
  if (dockerHostPreflight) {
    refs.push('linux-docker-host-preflight-ref');
  }
  return refs;
}

function linuxOpenGaps(
  proof: AppGameLinuxWslRuntimeProof,
  activeWindowToolProof?: AppGameLinuxActiveWindowToolProof,
  dockerHostPreflight?: AppGameLinuxDockerHostPreflightReadModel
) {
  const gaps: AppGamePlatformProofStatusGap[] = ['cross-platform-child-delivery-not-proved'];
  if (!proof.sessionProofAttached) {
    gaps.push('linux-native-session-not-proved');
  }
  if (
    !proof.displayProofAttached ||
    !proof.foregroundCaptureClaimed ||
    activeWindowToolProof?.openGaps.includes('linux-foreground-capture-not-proved')
  ) {
    gaps.push('linux-foreground-capture-not-proved');
  }
  if (!proof.mechanismProofAttached || !proof.distroProofAttached) {
    gaps.push('linux-policy-mechanism-not-proved');
  }
  if (!proof.rollbackProofAttached) {
    gaps.push('linux-rollback-not-proved');
  }
  if (!proof.auditProofAttached) {
    gaps.push('linux-audit-not-proved');
  }
  if (dockerHostPreflight?.openGaps.includes('linux-container-policy-not-proved')) {
    gaps.push('linux-container-policy-not-proved');
  }
  return gaps;
}

function statusRows(
  rows: readonly (AppGamePlatformProofStatusRowCandidate | undefined)[]
): readonly AppGamePlatformProofStatusRowCandidate[] {
  return rows.filter((row): row is AppGamePlatformProofStatusRowCandidate => row !== undefined);
}

function platformProofStatusLabel(value: unknown): AppGamePlatformProofStatusRowCandidate['parentVisibleSummary'] {
  return decodePlatformProofStatusLabel(value);
}

function appGamePlatformProofStatusRowIsHonest(row: AppGamePlatformProofStatusRowCandidate): boolean {
  return (
    row.authorityState === 'visibility-only' &&
    !row.adapterDispatchClaimed &&
    !row.broadBlockingClaimed &&
    !row.platformEnforcementClaimed &&
    !row.childDeliveryClaimed &&
    row.openGaps.includes('cross-platform-child-delivery-not-proved') &&
    row.proofRefs.length > 0
  );
}

function appGamePlatformProofStatusReadModelCountsMatch(
  readModel: AppGamePlatformProofStatusReadModelCandidate
): boolean {
  return (
    readModel.platformProofObservedCount === readModel.rows.length &&
    readModel.visibilityOnlyCount === readModel.rows.filter((row) => row.authorityState === 'visibility-only').length &&
    readModel.enforcementReadyCount === readModel.rows.filter((row) => row.platformEnforcementClaimed).length &&
    readModel.openGapCount === readModel.rows.reduce((total, row) => total + row.openGaps.length, 0)
  );
}


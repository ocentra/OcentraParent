import { Schema, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import {
  AppGamePlatformProofStatusGapSchema,
  AppGamePlatformProofStatusReadModelSchema,
  type AppGamePlatformProofStatusReadModel,
  type AppGamePlatformProofStatusRow,
} from '@ocentra-parent/schema-domain/app-game-platform-proof-status';
import { type AppGameAndroidAccessibilityOverlayPreflightReadModel } from '@ocentra-parent/schema-domain/app-game-android-accessibility-overlay-preflight';
import { type AppGameAndroidAccessibilityRuntimeProof } from '@ocentra-parent/schema-domain/app-game-android-accessibility-runtime-proof';
import { type AppGameAndroidAuthorityPreflightReadModel } from '@ocentra-parent/schema-domain/app-game-android-authority-preflight';
import { type AppGameAndroidPhysicalDeviceProof } from '@ocentra-parent/schema-domain/app-game-android-physical-device-proof';
import { type AppGameAndroidUsageEventsReplayReadModel } from '@ocentra-parent/schema-domain/app-game-android-usage-events-replay';
import { type AppGameAppleCiPlatformProofPreflightReadModel } from '@ocentra-parent/schema-domain/app-game-apple-ci-platform-proof-preflight';
import { type AppGameLinuxActiveWindowToolProof } from '@ocentra-parent/schema-domain/app-game-linux-active-window-tool-proof';
import { type AppGameLinuxDockerHostPreflightReadModel } from '@ocentra-parent/schema-domain/app-game-linux-docker-host-preflight';
import { type AppGameLinuxForegroundCaptureReadiness } from '@ocentra-parent/schema-domain/app-game-linux-foreground-capture-readiness';
import { type AppGameLinuxWslRuntimeProof } from '@ocentra-parent/schema-domain/app-game-linux-wsl-runtime-proof';
import { type AppGameWindowsBroadBlockingAuthorityPreflightReadModel } from '@ocentra-parent/schema-domain/app-game-windows-broad-blocking-authority-preflight';
import { type AppGameWindowsLocalPolicyEvidenceProof } from '@ocentra-parent/schema-domain/app-game-windows-local-policy-evidence-proof';

const PlatformProofStatusLabelSchema = brandedNonEmptyStringSchema('AppGamePlatformProofStatusLabel');
const decodePlatformProofStatusLabel = Schema.decodeUnknownSync(PlatformProofStatusLabelSchema);

type AppGamePlatformProofStatusGap = typeof AppGamePlatformProofStatusGapSchema.Type;
type AppGamePlatformProofStatusRef = AppGamePlatformProofStatusRow['proofRefs'][number];

const decodeAppGamePlatformProofStatusReadModel = Schema.decodeUnknownSync(AppGamePlatformProofStatusReadModelSchema);

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
): AppGamePlatformProofStatusRow {
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
): AppGamePlatformProofStatusRow {
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
): readonly AppGamePlatformProofStatusRow[] {
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
): AppGamePlatformProofStatusRow {
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

function androidOpenGaps(
  proof: AppGameAndroidPhysicalDeviceProof,
  replay?: AppGameAndroidUsageEventsReplayReadModel,
  authorityPreflight?: AppGameAndroidAuthorityPreflightReadModel,
  accessibilityOverlayPreflight?: AppGameAndroidAccessibilityOverlayPreflightReadModel,
  accessibilityRuntimeProof?: AppGameAndroidAccessibilityRuntimeProof
) {
  const gaps: AppGamePlatformProofStatusGap[] = ['cross-platform-child-delivery-not-proved'];
  const conditionalGaps: ReadonlyArray<readonly [boolean, AppGamePlatformProofStatusGap]> = [
    [proof.deviceOwnerState === 'not-device-owner', 'android-device-owner-not-proved'],
    [proof.profileOwnerState === 'not-profile-owner', 'android-profile-owner-not-proved'],
    [androidUsageEventsNotProved(proof), 'android-usage-events-not-proved'],
    [!replay?.durableReplayReady, 'android-durable-usage-events-replay-not-proved'],
    [androidReplayConsumerMissing(replay), 'android-child-runtime-replay-consumer-not-attached'],
    [!authorityPreflight, 'android-authority-preflight-not-attached'],
    [
      androidAccessibilityOverlayNotProved(accessibilityOverlayPreflight, accessibilityRuntimeProof),
      'android-accessibility-overlay-not-proved',
    ],
    [!proof.hideSuspendClaimed, 'android-hide-suspend-not-proved'],
  ];

  for (const [condition, gap] of conditionalGaps) {
    if (condition) {
      gaps.push(gap);
    }
  }

  return gaps;
}

function androidUsageEventsNotProved(proof: AppGameAndroidPhysicalDeviceProof) {
  return proof.usageEventsDumpState !== 'usage-events-dump-observed' || !proof.foregroundEvidenceObserved;
}

function androidReplayConsumerMissing(replay?: AppGameAndroidUsageEventsReplayReadModel) {
  return replay?.openGaps.includes('android-child-runtime-replay-consumer-not-attached') === true;
}

function androidAccessibilityOverlayNotProved(
  accessibilityOverlayPreflight?: AppGameAndroidAccessibilityOverlayPreflightReadModel,
  accessibilityRuntimeProof?: AppGameAndroidAccessibilityRuntimeProof
) {
  return (
    accessibilityOverlayPreflight?.openBlockers.includes('android-overlay-runtime-not-proved') === true ||
    accessibilityRuntimeProof?.openGaps.includes('android-accessibility-overlay-runtime-not-proved') === true
  );
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
  rows: readonly (AppGamePlatformProofStatusRow | undefined)[]
): readonly AppGamePlatformProofStatusRow[] {
  return rows.filter((row): row is AppGamePlatformProofStatusRow => row !== undefined);
}

function platformProofStatusLabel(value: unknown): AppGamePlatformProofStatusRow['parentVisibleSummary'] {
  return decodePlatformProofStatusLabel(value);
}

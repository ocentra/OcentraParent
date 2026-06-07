import { AppGameChildUxCapabilityState, AppGameChildUxTargetKind } from './app-game-child-facing-ux-rules';

export const AppGameChildUxRuntimeAuditHandoffState = {
  RuntimeAuditReady: 'runtime-audit-ready',
  BlockedMissingChildReason: 'blocked-missing-child-reason',
  BlockedMissingChildStatus: 'blocked-missing-child-status',
  ManualRequiredNoAdapter: 'manual-required-no-adapter',
} as const;

export type AppGameChildUxRuntimeAuditHandoffStateValue =
  (typeof AppGameChildUxRuntimeAuditHandoffState)[keyof typeof AppGameChildUxRuntimeAuditHandoffState];

export const RequiredAppGameChildUxRuntimeAuditHandoffNonClaims = [
  'no-child-runtime-delivery',
  'no-child-request-ui-rendering',
  'no-child-status-runtime-persistence',
  'no-runtime-audit-persistence',
  'no-adapter-dispatch',
  'no-platform-enforcement',
  'no-private-diagnostics',
] as const;

export const AppGameChildUxRuntimeAuditHandoffNoClaimFlags = {
  childRuntimeDelivered: false,
  childRequestUiRendered: false,
  childStatusRuntimePersisted: false,
  runtimeAuditPersisted: false,
  adapterDispatchClaimed: false,
  platformEnforcementClaimed: false,
  privateDiagnosticsExposed: false,
} as const;

type AppGameChildUxCapabilityStateValue =
  (typeof AppGameChildUxCapabilityState)[keyof typeof AppGameChildUxCapabilityState];
type AppGameChildUxTargetKindValue = (typeof AppGameChildUxTargetKind)[keyof typeof AppGameChildUxTargetKind];

type ChildUxRuntimeAuditCounts = {
  rows: ReadonlyArray<{
    targetDomain: 'native-app' | 'native-game';
    runtimeAuditHandoffState: AppGameChildUxRuntimeAuditHandoffStateValue;
  }>;
  nativeAppRowCount: number;
  nativeGameRowCount: number;
  runtimeAuditReadyCount: number;
  blockedMissingChildReasonCount: number;
  blockedMissingChildStatusCount: number;
  manualRequiredNoAdapterCount: number;
};

export function appGameChildUxRuntimeAuditHandoffCountsMatch(handoff: ChildUxRuntimeAuditCounts): boolean {
  return (
    handoff.nativeAppRowCount === handoff.rows.filter((row) => row.targetDomain === 'native-app').length &&
    handoff.nativeGameRowCount === handoff.rows.filter((row) => row.targetDomain === 'native-game').length &&
    handoff.runtimeAuditReadyCount ===
      handoff.rows.filter(
        (row) => row.runtimeAuditHandoffState === AppGameChildUxRuntimeAuditHandoffState.RuntimeAuditReady
      ).length &&
    handoff.blockedMissingChildReasonCount ===
      handoff.rows.filter(
        (row) => row.runtimeAuditHandoffState === AppGameChildUxRuntimeAuditHandoffState.BlockedMissingChildReason
      ).length &&
    handoff.blockedMissingChildStatusCount ===
      handoff.rows.filter(
        (row) => row.runtimeAuditHandoffState === AppGameChildUxRuntimeAuditHandoffState.BlockedMissingChildStatus
      ).length &&
    handoff.manualRequiredNoAdapterCount ===
      handoff.rows.filter(
        (row) => row.runtimeAuditHandoffState === AppGameChildUxRuntimeAuditHandoffState.ManualRequiredNoAdapter
      ).length
  );
}

export function appGameChildUxRuntimeAuditHandoffHasNoRuntimeClaims(
  handoff: Readonly<Record<string, unknown>>
): boolean {
  return Object.keys(AppGameChildUxRuntimeAuditHandoffNoClaimFlags).every((key) => handoff[key] === false);
}

export function appGameChildUxTargetKindToDomain(
  targetKind: AppGameChildUxTargetKindValue
): 'native-app' | 'native-game' {
  if (
    targetKind === AppGameChildUxTargetKind.NativeGame ||
    targetKind === AppGameChildUxTargetKind.LauncherGameCandidate ||
    targetKind === AppGameChildUxTargetKind.UnknownGame
  ) {
    return 'native-game';
  }
  return 'native-app';
}

export function appGameChildUxRuntimeAuditStateForCard(card: {
  readonly capabilityState: AppGameChildUxCapabilityStateValue;
  readonly childReasonReferences: ReadonlyArray<unknown>;
  readonly childStatusReferences: ReadonlyArray<unknown>;
}): AppGameChildUxRuntimeAuditHandoffStateValue {
  if (
    card.capabilityState === AppGameChildUxCapabilityState.ManualRequired ||
    card.capabilityState === AppGameChildUxCapabilityState.Unavailable
  ) {
    return AppGameChildUxRuntimeAuditHandoffState.ManualRequiredNoAdapter;
  }
  if (card.childReasonReferences.length === 0) {
    return AppGameChildUxRuntimeAuditHandoffState.BlockedMissingChildReason;
  }
  if (card.childStatusReferences.length === 0) {
    return AppGameChildUxRuntimeAuditHandoffState.BlockedMissingChildStatus;
  }
  return AppGameChildUxRuntimeAuditHandoffState.RuntimeAuditReady;
}

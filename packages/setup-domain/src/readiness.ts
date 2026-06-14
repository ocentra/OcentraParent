import {
  ChildProfileReferenceSchema,
  FamilyReferenceSchema,
  ParentAccountReferenceSchema,
  ParentDeviceReferenceSchema,
} from '@ocentra-parent/family-domain/references';
import {
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/family-domain/reference-primitives';
import { SetupPairingState, SetupPairingStateSchema, SetupPairingIntentIdSchema } from './pairing-intent';
import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

function brandedNonEmptyStringSchema<const Brand extends string>(brand: Brand) {
  return Schema.String.pipe(Schema.minLength(1), Schema.brand(brand));
}

export const SetupReadinessReportIdSchema = brandedNonEmptyStringSchema('SetupReadinessReportId');
export const SetupReadinessChecklistItemIdSchema = brandedNonEmptyStringSchema('SetupReadinessChecklistItemId');
export const SetupSupportCodeSchema = brandedNonEmptyStringSchema('SetupSupportCode');
export const SetupRecoveryOperationIdSchema = brandedNonEmptyStringSchema('SetupRecoveryOperationId');

export const SetupAccountReadinessStateLiteral = {
  Ready: 'ready',
  WrongAccount: 'wrong-account',
  RecoveryRequired: 'recovery-required',
} as const;

export const SetupParentAppReadinessStateLiteral = {
  Ready: 'ready',
  Missing: 'missing',
  Offline: 'offline',
  Stale: 'stale',
} as const;

export const SetupChildAppReadinessStateLiteral = {
  Ready: 'ready',
  Offline: 'offline',
  Revoked: 'revoked',
  ReinstallRequired: 'reinstall-required',
} as const;

export const SetupChildInstallStateLiteral = {
  NotInstalled: 'not-installed',
  Installed: 'installed',
  ReinstallRequired: 'reinstall-required',
} as const;

export const SetupChildServiceStateLiteral = {
  NotStarted: 'not-started',
  ServiceStarted: 'service-started',
  Offline: 'offline',
  Revoked: 'revoked',
} as const;

export const SetupPermissionReadinessStateLiteral = {
  Granted: 'granted',
  Missing: 'missing',
  Revoked: 'revoked',
} as const;

export const SetupPolicyBaselineStateLiteral = {
  Applied: 'applied',
  Missing: 'missing',
  Stale: 'stale',
} as const;

export const SetupDataCustodySyncStateLiteral = {
  Synced: 'synced',
  SyncPending: 'sync-pending',
  Blocked: 'blocked',
} as const;

export const SetupNetworkReachabilityStateLiteral = {
  Reachable: 'reachable',
  OfflineChild: 'offline-child',
  LanUnavailable: 'lan-unavailable',
  DirectEntryRequired: 'direct-entry-required',
} as const;

export const SetupRecoveryKindLiteral = {
  LostParentDevice: 'lost-parent-device',
  ChildReinstall: 'child-reinstall',
  RevokedChild: 'revoked-child',
  WrongAccount: 'wrong-account',
  OfflineDevice: 'offline-device',
  PermissionLoss: 'permission-loss',
  StaleCode: 'stale-code',
} as const;

export const SetupRecoveryStateLiteral = {
  Normal: 'normal',
  Required: 'required',
  InProgress: 'in-progress',
  Recovered: 'recovered',
} as const;

export const SetupReadinessChecklistItemStateLiteral = {
  Complete: 'complete',
  ActionRequired: 'action-required',
  Degraded: 'degraded',
} as const;

export const SetupReadinessOverallStateLiteral = {
  Ready: 'ready',
  Degraded: 'degraded',
  Blocked: 'blocked',
} as const;

// Keep canonical setup-plan stages coarse. Service start, trust, and policy
// readiness stay available through dedicated report fields and checklist items.
export const SetupChildInstallJourneyStageLiteral = {
  InstallRequired: 'install-required',
  Installed: 'installed',
  Permissioned: 'permissioned',
  Paired: 'paired',
} as const;

export const SetupAccountReadinessStateSchema = withParser(
  Schema.Literal(
    SetupAccountReadinessStateLiteral.Ready,
    SetupAccountReadinessStateLiteral.WrongAccount,
    SetupAccountReadinessStateLiteral.RecoveryRequired
  )
);

export const SetupParentAppReadinessStateSchema = withParser(
  Schema.Literal(
    SetupParentAppReadinessStateLiteral.Ready,
    SetupParentAppReadinessStateLiteral.Missing,
    SetupParentAppReadinessStateLiteral.Offline,
    SetupParentAppReadinessStateLiteral.Stale
  )
);

export const SetupChildAppReadinessStateSchema = withParser(
  Schema.Literal(
    SetupChildAppReadinessStateLiteral.Ready,
    SetupChildAppReadinessStateLiteral.Offline,
    SetupChildAppReadinessStateLiteral.Revoked,
    SetupChildAppReadinessStateLiteral.ReinstallRequired
  )
);

export const SetupChildInstallStateSchema = withParser(
  Schema.Literal(
    SetupChildInstallStateLiteral.NotInstalled,
    SetupChildInstallStateLiteral.Installed,
    SetupChildInstallStateLiteral.ReinstallRequired
  )
);

export const SetupChildServiceStateSchema = withParser(
  Schema.Literal(
    SetupChildServiceStateLiteral.NotStarted,
    SetupChildServiceStateLiteral.ServiceStarted,
    SetupChildServiceStateLiteral.Offline,
    SetupChildServiceStateLiteral.Revoked
  )
);

export const SetupPermissionReadinessStateSchema = withParser(
  Schema.Literal(
    SetupPermissionReadinessStateLiteral.Granted,
    SetupPermissionReadinessStateLiteral.Missing,
    SetupPermissionReadinessStateLiteral.Revoked
  )
);

export const SetupPolicyBaselineStateSchema = withParser(
  Schema.Literal(
    SetupPolicyBaselineStateLiteral.Applied,
    SetupPolicyBaselineStateLiteral.Missing,
    SetupPolicyBaselineStateLiteral.Stale
  )
);

export const SetupDataCustodySyncStateSchema = withParser(
  Schema.Literal(
    SetupDataCustodySyncStateLiteral.Synced,
    SetupDataCustodySyncStateLiteral.SyncPending,
    SetupDataCustodySyncStateLiteral.Blocked
  )
);

export const SetupNetworkReachabilityStateSchema = withParser(
  Schema.Literal(
    SetupNetworkReachabilityStateLiteral.Reachable,
    SetupNetworkReachabilityStateLiteral.OfflineChild,
    SetupNetworkReachabilityStateLiteral.LanUnavailable,
    SetupNetworkReachabilityStateLiteral.DirectEntryRequired
  )
);

export const SetupRecoveryKindSchema = withParser(
  Schema.Literal(
    SetupRecoveryKindLiteral.LostParentDevice,
    SetupRecoveryKindLiteral.ChildReinstall,
    SetupRecoveryKindLiteral.RevokedChild,
    SetupRecoveryKindLiteral.WrongAccount,
    SetupRecoveryKindLiteral.OfflineDevice,
    SetupRecoveryKindLiteral.PermissionLoss,
    SetupRecoveryKindLiteral.StaleCode
  )
);

export const SetupRecoveryStateSchema = withParser(
  Schema.Literal(
    SetupRecoveryStateLiteral.Normal,
    SetupRecoveryStateLiteral.Required,
    SetupRecoveryStateLiteral.InProgress,
    SetupRecoveryStateLiteral.Recovered
  )
);

export const SetupReadinessChecklistItemStateSchema = withParser(
  Schema.Literal(
    SetupReadinessChecklistItemStateLiteral.Complete,
    SetupReadinessChecklistItemStateLiteral.ActionRequired,
    SetupReadinessChecklistItemStateLiteral.Degraded
  )
);

export const SetupReadinessOverallStateSchema = withParser(
  Schema.Literal(
    SetupReadinessOverallStateLiteral.Ready,
    SetupReadinessOverallStateLiteral.Degraded,
    SetupReadinessOverallStateLiteral.Blocked
  )
);

export const SetupChildInstallJourneyStageSchema = withParser(
  Schema.Literal(
    SetupChildInstallJourneyStageLiteral.InstallRequired,
    SetupChildInstallJourneyStageLiteral.Installed,
    SetupChildInstallJourneyStageLiteral.Permissioned,
    SetupChildInstallJourneyStageLiteral.Paired
  )
);

export const SetupReadinessChecklistItemSchema = withParser(
  Schema.Struct({
    checklistItemId: SetupReadinessChecklistItemIdSchema,
    label: Schema.String.pipe(Schema.minLength(1)),
    state: SetupReadinessChecklistItemStateSchema,
    blocking: Schema.Boolean,
    supportCode: SetupSupportCodeSchema,
  })
);

export const SetupReadinessReportSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    readinessReportId: SetupReadinessReportIdSchema,
    family: FamilyReferenceSchema,
    parentAccount: ParentAccountReferenceSchema,
    parentDevice: ParentDeviceReferenceSchema,
    childProfile: ChildProfileReferenceSchema,
    pairingIntentId: SetupPairingIntentIdSchema,
    accountState: SetupAccountReadinessStateSchema,
    parentAppState: SetupParentAppReadinessStateSchema,
    childAppState: SetupChildAppReadinessStateSchema,
    childInstallState: Schema.optionalWith(Schema.Union(SetupChildInstallStateSchema, Schema.Null), {
      default: () => null,
    }),
    childServiceState: Schema.optionalWith(Schema.Union(SetupChildServiceStateSchema, Schema.Null), {
      default: () => null,
    }),
    permissionState: SetupPermissionReadinessStateSchema,
    pairingState: SetupPairingStateSchema,
    policyBaselineState: SetupPolicyBaselineStateSchema,
    dataCustodySyncState: SetupDataCustodySyncStateSchema,
    networkReachabilityState: SetupNetworkReachabilityStateSchema,
    recoveryState: SetupRecoveryStateSchema,
    observedAt: ParentTimestampSchema,
    checklist: Schema.Array(SetupReadinessChecklistItemSchema),
  })
);

export const SetupRecoveryOperationSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    recoveryOperationId: SetupRecoveryOperationIdSchema,
    family: FamilyReferenceSchema,
    parentAccount: ParentAccountReferenceSchema,
    parentDevice: ParentDeviceReferenceSchema,
    childProfile: ChildProfileReferenceSchema,
    childDevice: Schema.Union(ParentDeviceReferenceSchema, Schema.Null),
    kind: SetupRecoveryKindSchema,
    state: SetupRecoveryStateSchema,
    sourcePairingState: SetupPairingStateSchema,
    openedAt: ParentTimestampSchema,
    resolvedAt: Schema.Union(ParentTimestampSchema, Schema.Null),
  })
);

export type SetupAccountReadinessState = Infer<typeof SetupAccountReadinessStateSchema>;
export type SetupParentAppReadinessState = Infer<typeof SetupParentAppReadinessStateSchema>;
export type SetupChildAppReadinessState = Infer<typeof SetupChildAppReadinessStateSchema>;
export type SetupChildInstallState = Infer<typeof SetupChildInstallStateSchema>;
export type SetupChildServiceState = Infer<typeof SetupChildServiceStateSchema>;
export type SetupPermissionReadinessState = Infer<typeof SetupPermissionReadinessStateSchema>;
export type SetupPolicyBaselineState = Infer<typeof SetupPolicyBaselineStateSchema>;
export type SetupDataCustodySyncState = Infer<typeof SetupDataCustodySyncStateSchema>;
export type SetupNetworkReachabilityState = Infer<typeof SetupNetworkReachabilityStateSchema>;
export type SetupRecoveryKind = Infer<typeof SetupRecoveryKindSchema>;
export type SetupRecoveryState = Infer<typeof SetupRecoveryStateSchema>;
export type SetupReadinessChecklistItemState = Infer<typeof SetupReadinessChecklistItemStateSchema>;
export type SetupReadinessOverallState = Infer<typeof SetupReadinessOverallStateSchema>;
export type SetupChildInstallJourneyStage = Infer<typeof SetupChildInstallJourneyStageSchema>;
export type SetupReadinessChecklistItem = Infer<typeof SetupReadinessChecklistItemSchema>;
export type SetupReadinessReport = Infer<typeof SetupReadinessReportSchema>;
export type SetupRecoveryOperation = Infer<typeof SetupRecoveryOperationSchema>;

export const SetupAccountReadinessState = {
  Ready: SetupAccountReadinessStateSchema.parse(SetupAccountReadinessStateLiteral.Ready),
  WrongAccount: SetupAccountReadinessStateSchema.parse(SetupAccountReadinessStateLiteral.WrongAccount),
  RecoveryRequired: SetupAccountReadinessStateSchema.parse(SetupAccountReadinessStateLiteral.RecoveryRequired),
} as const;

export const SetupParentAppReadinessState = {
  Ready: SetupParentAppReadinessStateSchema.parse(SetupParentAppReadinessStateLiteral.Ready),
  Missing: SetupParentAppReadinessStateSchema.parse(SetupParentAppReadinessStateLiteral.Missing),
  Offline: SetupParentAppReadinessStateSchema.parse(SetupParentAppReadinessStateLiteral.Offline),
  Stale: SetupParentAppReadinessStateSchema.parse(SetupParentAppReadinessStateLiteral.Stale),
} as const;

export const SetupChildAppReadinessState = {
  Ready: SetupChildAppReadinessStateSchema.parse(SetupChildAppReadinessStateLiteral.Ready),
  Offline: SetupChildAppReadinessStateSchema.parse(SetupChildAppReadinessStateLiteral.Offline),
  Revoked: SetupChildAppReadinessStateSchema.parse(SetupChildAppReadinessStateLiteral.Revoked),
  ReinstallRequired: SetupChildAppReadinessStateSchema.parse(SetupChildAppReadinessStateLiteral.ReinstallRequired),
} as const;

export const SetupChildInstallState = {
  NotInstalled: SetupChildInstallStateSchema.parse(SetupChildInstallStateLiteral.NotInstalled),
  Installed: SetupChildInstallStateSchema.parse(SetupChildInstallStateLiteral.Installed),
  ReinstallRequired: SetupChildInstallStateSchema.parse(SetupChildInstallStateLiteral.ReinstallRequired),
} as const;

export const SetupChildServiceState = {
  NotStarted: SetupChildServiceStateSchema.parse(SetupChildServiceStateLiteral.NotStarted),
  ServiceStarted: SetupChildServiceStateSchema.parse(SetupChildServiceStateLiteral.ServiceStarted),
  Offline: SetupChildServiceStateSchema.parse(SetupChildServiceStateLiteral.Offline),
  Revoked: SetupChildServiceStateSchema.parse(SetupChildServiceStateLiteral.Revoked),
} as const;

export const SetupPermissionReadinessState = {
  Granted: SetupPermissionReadinessStateSchema.parse(SetupPermissionReadinessStateLiteral.Granted),
  Missing: SetupPermissionReadinessStateSchema.parse(SetupPermissionReadinessStateLiteral.Missing),
  Revoked: SetupPermissionReadinessStateSchema.parse(SetupPermissionReadinessStateLiteral.Revoked),
} as const;

export const SetupPolicyBaselineState = {
  Applied: SetupPolicyBaselineStateSchema.parse(SetupPolicyBaselineStateLiteral.Applied),
  Missing: SetupPolicyBaselineStateSchema.parse(SetupPolicyBaselineStateLiteral.Missing),
  Stale: SetupPolicyBaselineStateSchema.parse(SetupPolicyBaselineStateLiteral.Stale),
} as const;

export const SetupDataCustodySyncState = {
  Synced: SetupDataCustodySyncStateSchema.parse(SetupDataCustodySyncStateLiteral.Synced),
  SyncPending: SetupDataCustodySyncStateSchema.parse(SetupDataCustodySyncStateLiteral.SyncPending),
  Blocked: SetupDataCustodySyncStateSchema.parse(SetupDataCustodySyncStateLiteral.Blocked),
} as const;

export const SetupNetworkReachabilityState = {
  Reachable: SetupNetworkReachabilityStateSchema.parse(SetupNetworkReachabilityStateLiteral.Reachable),
  OfflineChild: SetupNetworkReachabilityStateSchema.parse(SetupNetworkReachabilityStateLiteral.OfflineChild),
  LanUnavailable: SetupNetworkReachabilityStateSchema.parse(SetupNetworkReachabilityStateLiteral.LanUnavailable),
  DirectEntryRequired: SetupNetworkReachabilityStateSchema.parse(
    SetupNetworkReachabilityStateLiteral.DirectEntryRequired
  ),
} as const;

export const SetupRecoveryKind = {
  LostParentDevice: SetupRecoveryKindSchema.parse(SetupRecoveryKindLiteral.LostParentDevice),
  ChildReinstall: SetupRecoveryKindSchema.parse(SetupRecoveryKindLiteral.ChildReinstall),
  RevokedChild: SetupRecoveryKindSchema.parse(SetupRecoveryKindLiteral.RevokedChild),
  WrongAccount: SetupRecoveryKindSchema.parse(SetupRecoveryKindLiteral.WrongAccount),
  OfflineDevice: SetupRecoveryKindSchema.parse(SetupRecoveryKindLiteral.OfflineDevice),
  PermissionLoss: SetupRecoveryKindSchema.parse(SetupRecoveryKindLiteral.PermissionLoss),
  StaleCode: SetupRecoveryKindSchema.parse(SetupRecoveryKindLiteral.StaleCode),
} as const;

export const SetupRecoveryState = {
  Normal: SetupRecoveryStateSchema.parse(SetupRecoveryStateLiteral.Normal),
  Required: SetupRecoveryStateSchema.parse(SetupRecoveryStateLiteral.Required),
  InProgress: SetupRecoveryStateSchema.parse(SetupRecoveryStateLiteral.InProgress),
  Recovered: SetupRecoveryStateSchema.parse(SetupRecoveryStateLiteral.Recovered),
} as const;

export const SetupReadinessChecklistItemState = {
  Complete: SetupReadinessChecklistItemStateSchema.parse(SetupReadinessChecklistItemStateLiteral.Complete),
  ActionRequired: SetupReadinessChecklistItemStateSchema.parse(SetupReadinessChecklistItemStateLiteral.ActionRequired),
  Degraded: SetupReadinessChecklistItemStateSchema.parse(SetupReadinessChecklistItemStateLiteral.Degraded),
} as const;

export const SetupReadinessOverallState = {
  Ready: SetupReadinessOverallStateSchema.parse(SetupReadinessOverallStateLiteral.Ready),
  Degraded: SetupReadinessOverallStateSchema.parse(SetupReadinessOverallStateLiteral.Degraded),
  Blocked: SetupReadinessOverallStateSchema.parse(SetupReadinessOverallStateLiteral.Blocked),
} as const;

export const SetupChildInstallJourneyStage = {
  InstallRequired: SetupChildInstallJourneyStageSchema.parse(SetupChildInstallJourneyStageLiteral.InstallRequired),
  Installed: SetupChildInstallJourneyStageSchema.parse(SetupChildInstallJourneyStageLiteral.Installed),
  Permissioned: SetupChildInstallJourneyStageSchema.parse(SetupChildInstallJourneyStageLiteral.Permissioned),
  Paired: SetupChildInstallJourneyStageSchema.parse(SetupChildInstallJourneyStageLiteral.Paired),
} as const;

function readinessChecklistItem(
  checklistItemId: string,
  label: string,
  state: SetupReadinessChecklistItemState,
  blocking: boolean,
  supportCode: string
): SetupReadinessChecklistItem {
  return SetupReadinessChecklistItemSchema.parse({
    checklistItemId,
    label,
    state,
    blocking,
    supportCode,
  });
}

export function deriveSetupChildInstallStateFromAppState(
  childAppState: SetupChildAppReadinessState
): SetupChildInstallState {
  switch (childAppState) {
    case SetupChildAppReadinessState.ReinstallRequired:
      return SetupChildInstallState.ReinstallRequired;
    default:
      return SetupChildInstallState.Installed;
  }
}

export function deriveSetupChildServiceStateFromAppState(
  childAppState: SetupChildAppReadinessState
): SetupChildServiceState {
  switch (childAppState) {
    case SetupChildAppReadinessState.Ready:
      return SetupChildServiceState.ServiceStarted;
    case SetupChildAppReadinessState.Offline:
      return SetupChildServiceState.Offline;
    case SetupChildAppReadinessState.Revoked:
      return SetupChildServiceState.Revoked;
    case SetupChildAppReadinessState.ReinstallRequired:
      return SetupChildServiceState.NotStarted;
  }
}

function resolvedSetupChildInstallState(report: SetupReadinessReport): SetupChildInstallState {
  return report.childInstallState ?? deriveSetupChildInstallStateFromAppState(report.childAppState);
}

function resolvedSetupChildServiceState(report: SetupReadinessReport): SetupChildServiceState {
  return report.childServiceState ?? deriveSetupChildServiceStateFromAppState(report.childAppState);
}

export function resolveSetupChildInstallState(input: SetupReadinessReport): SetupChildInstallState {
  return resolvedSetupChildInstallState(SetupReadinessReportSchema.parse(input));
}

export function resolveSetupChildServiceState(input: SetupReadinessReport): SetupChildServiceState {
  return resolvedSetupChildServiceState(SetupReadinessReportSchema.parse(input));
}

export function deriveSetupChildInstallJourneyStage(input: SetupReadinessReport): SetupChildInstallJourneyStage {
  const report = SetupReadinessReportSchema.parse(input);
  const childInstallState = resolvedSetupChildInstallState(report);
  const childServiceState = resolvedSetupChildServiceState(report);
  const childServiceReady = childServiceState === SetupChildServiceState.ServiceStarted;
  const permissionsReady = report.permissionState === SetupPermissionReadinessState.Granted;

  if (childInstallState !== SetupChildInstallState.Installed) {
    return SetupChildInstallJourneyStage.InstallRequired;
  }

  if (!childServiceReady || !permissionsReady) {
    return SetupChildInstallJourneyStage.Installed;
  }

  if (
    report.pairingState !== SetupPairingState.Accepted &&
    report.pairingState !== SetupPairingState.Trusted &&
    report.pairingState !== SetupPairingState.Recovered
  ) {
    return SetupChildInstallJourneyStage.Permissioned;
  }

  return SetupChildInstallJourneyStage.Paired;
}

export function deriveSetupReadinessOverallState(input: SetupReadinessReport): SetupReadinessOverallState {
  const report = SetupReadinessReportSchema.parse(input);
  const childInstallState = resolvedSetupChildInstallState(report);
  const childServiceState = resolvedSetupChildServiceState(report);

  const fullyReady =
    report.accountState === SetupAccountReadinessState.Ready &&
    report.parentAppState === SetupParentAppReadinessState.Ready &&
    childInstallState === SetupChildInstallState.Installed &&
    childServiceState === SetupChildServiceState.ServiceStarted &&
    report.permissionState === SetupPermissionReadinessState.Granted &&
    (report.pairingState === SetupPairingState.Trusted || report.pairingState === SetupPairingState.Recovered) &&
    report.policyBaselineState === SetupPolicyBaselineState.Applied &&
    report.dataCustodySyncState === SetupDataCustodySyncState.Synced &&
    report.networkReachabilityState === SetupNetworkReachabilityState.Reachable &&
    (report.recoveryState === SetupRecoveryState.Normal || report.recoveryState === SetupRecoveryState.Recovered);

  if (fullyReady) {
    return SetupReadinessOverallState.Ready;
  }

  const degraded =
    childServiceState === SetupChildServiceState.Offline ||
    report.networkReachabilityState === SetupNetworkReachabilityState.OfflineChild ||
    report.dataCustodySyncState === SetupDataCustodySyncState.SyncPending;

  return degraded ? SetupReadinessOverallState.Degraded : SetupReadinessOverallState.Blocked;
}

export function isSetupReadinessReady(input: SetupReadinessReport): boolean {
  return deriveSetupReadinessOverallState(input) === SetupReadinessOverallState.Ready;
}

export function setupReadinessNeedsManualRecovery(input: SetupReadinessReport): boolean {
  const report = SetupReadinessReportSchema.parse(input);

  return report.recoveryState !== SetupRecoveryState.Normal && report.recoveryState !== SetupRecoveryState.Recovered;
}

export function createSetupReadinessChecklist(input: SetupReadinessReport): readonly SetupReadinessChecklistItem[] {
  const report = SetupReadinessReportSchema.parse(input);
  const overallState = deriveSetupReadinessOverallState(report);
  const childInstallState = resolvedSetupChildInstallState(report);
  const childServiceState = resolvedSetupChildServiceState(report);

  return [
    readinessChecklistItem(
      'setup-account-state',
      'Account',
      report.accountState === SetupAccountReadinessState.Ready
        ? SetupReadinessChecklistItemState.Complete
        : SetupReadinessChecklistItemState.ActionRequired,
      report.accountState !== SetupAccountReadinessState.Ready,
      report.accountState
    ),
    readinessChecklistItem(
      'setup-parent-app-state',
      'Parent app',
      report.parentAppState === SetupParentAppReadinessState.Ready
        ? SetupReadinessChecklistItemState.Complete
        : SetupReadinessChecklistItemState.ActionRequired,
      report.parentAppState !== SetupParentAppReadinessState.Ready,
      report.parentAppState
    ),
    readinessChecklistItem(
      'setup-child-install-state',
      'Child install',
      childInstallState === SetupChildInstallState.Installed
        ? SetupReadinessChecklistItemState.Complete
        : SetupReadinessChecklistItemState.ActionRequired,
      childInstallState !== SetupChildInstallState.Installed,
      childInstallState
    ),
    readinessChecklistItem(
      'setup-child-service-state',
      'Child service',
      childServiceState === SetupChildServiceState.ServiceStarted
        ? SetupReadinessChecklistItemState.Complete
        : childServiceState === SetupChildServiceState.Offline
          ? SetupReadinessChecklistItemState.Degraded
          : SetupReadinessChecklistItemState.ActionRequired,
      childServiceState !== SetupChildServiceState.ServiceStarted,
      childServiceState
    ),
    readinessChecklistItem(
      'setup-permission-state',
      'Permissions',
      report.permissionState === SetupPermissionReadinessState.Granted
        ? SetupReadinessChecklistItemState.Complete
        : SetupReadinessChecklistItemState.ActionRequired,
      report.permissionState !== SetupPermissionReadinessState.Granted,
      report.permissionState
    ),
    readinessChecklistItem(
      'setup-pairing-state',
      'Pairing',
      report.pairingState === SetupPairingState.Trusted || report.pairingState === SetupPairingState.Recovered
        ? SetupReadinessChecklistItemState.Complete
        : SetupReadinessChecklistItemState.ActionRequired,
      report.pairingState !== SetupPairingState.Trusted && report.pairingState !== SetupPairingState.Recovered,
      report.pairingState
    ),
    readinessChecklistItem(
      'setup-policy-baseline-state',
      'Policy baseline',
      report.policyBaselineState === SetupPolicyBaselineState.Applied
        ? SetupReadinessChecklistItemState.Complete
        : SetupReadinessChecklistItemState.ActionRequired,
      report.policyBaselineState !== SetupPolicyBaselineState.Applied,
      report.policyBaselineState
    ),
    readinessChecklistItem(
      'setup-custody-sync-state',
      'Custody sync',
      report.dataCustodySyncState === SetupDataCustodySyncState.Synced
        ? SetupReadinessChecklistItemState.Complete
        : report.dataCustodySyncState === SetupDataCustodySyncState.SyncPending
          ? SetupReadinessChecklistItemState.Degraded
          : SetupReadinessChecklistItemState.ActionRequired,
      report.dataCustodySyncState === SetupDataCustodySyncState.Blocked,
      report.dataCustodySyncState
    ),
    readinessChecklistItem(
      'setup-network-state',
      'Network reachability',
      report.networkReachabilityState === SetupNetworkReachabilityState.Reachable
        ? SetupReadinessChecklistItemState.Complete
        : report.networkReachabilityState === SetupNetworkReachabilityState.OfflineChild
          ? SetupReadinessChecklistItemState.Degraded
          : SetupReadinessChecklistItemState.ActionRequired,
      report.networkReachabilityState !== SetupNetworkReachabilityState.Reachable &&
        report.networkReachabilityState !== SetupNetworkReachabilityState.OfflineChild,
      report.networkReachabilityState
    ),
    readinessChecklistItem(
      'setup-overall-state',
      'Overall readiness',
      overallState === SetupReadinessOverallState.Ready
        ? SetupReadinessChecklistItemState.Complete
        : overallState === SetupReadinessOverallState.Degraded
          ? SetupReadinessChecklistItemState.Degraded
          : SetupReadinessChecklistItemState.ActionRequired,
      overallState === SetupReadinessOverallState.Blocked,
      overallState
    ),
  ];
}

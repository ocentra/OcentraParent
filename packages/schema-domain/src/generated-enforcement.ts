/* generated from crates/schema/src/policy_enforcement_ts.rs */

import { type Infer, Schema, brandedNonEmptyStringSchema, withParser } from './effect';
import {
  ParentActionReferenceSchema,
  ParentActorReferenceSchema,
  ParentDeviceReferenceSchema,
  ParentEvidenceReferenceSchema,
} from './family-references';
import {
  ParentContractSchemaVersionSchema,
  ParentPlatformSchema,
  ParentPolicyVersionSchema,
  ParentTimestampSchema,
} from './family-reference-primitives';
import {
  LocalAiResultReferenceIdSchema,
  PolicyActionSchema,
  PolicyDecisionIdSchema,
  PolicyReasonCodeSchema,
  PolicyTargetSchema,
} from './policy-contracts';

export const GeneratedEnforcementCapabilityStateValues = [
  'supported',
  'unavailable',
  'degraded',
  'dry-run',
  'observe-only',
  'manual-required',
] as const;

export const GeneratedEnforcementUnavailableReasonValues = [
  'unsupported-platform',
  'unsupported-action',
  'missing-permission',
  'missing-dependency',
  'adapter-unavailable',
  'adapter-error',
  'manual-required',
] as const;

export const GeneratedEnforcementResultStatusValues = [
  'would-enforce',
  'actually-enforced',
  'unavailable',
  'failed',
  'expired',
  'rolled-back',
  'superseded',
  'no-op',
] as const;

export const GeneratedEnforcementRollbackStateValues = [
  'not-required',
  'available',
  'requested',
  'completed',
  'unavailable',
  'failed',
] as const;

export const GeneratedEnforcementTimerEventKindValues = [
  'created',
  'extended',
  'expired',
  'cancelled',
  'restart-recovered',
  'rollback-requested',
  'rollback-completed',
  'recovery-needed',
  'unavailable',
] as const;

export const GeneratedEnforcementAuditEventKindValues = [
  'attempted',
  'succeeded',
  'failed',
  'rollback-requested',
  'rollback-completed',
  'expired',
  'unavailable',
  'cancelled',
] as const;

export type GeneratedEnforcementCapabilityState = (typeof GeneratedEnforcementCapabilityStateValues)[number];

export type GeneratedEnforcementUnavailableReason = (typeof GeneratedEnforcementUnavailableReasonValues)[number];

export type GeneratedEnforcementResultStatus = (typeof GeneratedEnforcementResultStatusValues)[number];

export type GeneratedEnforcementRollbackState = (typeof GeneratedEnforcementRollbackStateValues)[number];

export type GeneratedEnforcementTimerEventKind = (typeof GeneratedEnforcementTimerEventKindValues)[number];

export type GeneratedEnforcementAuditEventKind = (typeof GeneratedEnforcementAuditEventKindValues)[number];

export type GeneratedEnforcementCapabilityStatusLike = {
  readonly schemaVersion: string;
  readonly platform: string;
  readonly adapterKind: string;
  readonly capabilityState: GeneratedEnforcementCapabilityState;
  readonly permissionState: string;
  readonly dependencyState: string;
  readonly supportedActions: ReadonlyArray<string>;
  readonly degradedReason: string | null;
  readonly lastCheckedAt: string;
};

export type GeneratedEnforcementUnavailableStatusLike = {
  readonly schemaVersion: string;
  readonly capability: GeneratedEnforcementCapabilityStatusLike;
  readonly unavailableReason: GeneratedEnforcementUnavailableReason;
  readonly retryable: boolean;
  readonly checkedAt: string;
};

export type GeneratedEnforcementResultLike = {
  readonly schemaVersion: string;
  readonly resultId: string;
  readonly actionId: string;
  readonly status: GeneratedEnforcementResultStatus;
  readonly adapterResultCode: string;
  readonly startedAt: string;
  readonly completedAt: string | null;
  readonly rollbackToken: string | null;
  readonly rollbackState: GeneratedEnforcementRollbackState;
  readonly unavailableReason: string | null;
  readonly unavailableStatus: GeneratedEnforcementUnavailableStatusLike | null;
  readonly failedReason: string | null;
  readonly nextCheckAt: string | null;
  readonly capability: GeneratedEnforcementCapabilityStatusLike;
};

export type GeneratedEnforcementTimerEventLike = {
  readonly timerEventKind: GeneratedEnforcementTimerEventKind;
  readonly unavailableReason: GeneratedEnforcementUnavailableReason | null;
};

export type GeneratedEnforcementAuditEventLike = {
  readonly auditEventKind: GeneratedEnforcementAuditEventKind;
  readonly capability: GeneratedEnforcementCapabilityStatusLike;
  readonly result: GeneratedEnforcementResultLike;
  readonly unavailableStatus: GeneratedEnforcementUnavailableStatusLike | null;
};

export type GeneratedEnforcementActiveTimerStateLike = {
  readonly action: {
    readonly actionId: string;
    readonly policyDecisionId: string;
  };
  readonly result: {
    readonly actionId: string;
  };
  readonly auditEvent: {
    readonly action: {
      readonly actionId: string;
    };
  };
  readonly timerEvent: {
    readonly actionId: string;
    readonly policyDecisionId: string;
  };
};

const EnforcementIntentIdSchema = brandedNonEmptyStringSchema('EnforcementIntentId');
const EnforcementActionIdSchema = brandedNonEmptyStringSchema('EnforcementActionId');
const EnforcementResultIdSchema = brandedNonEmptyStringSchema('EnforcementResultId');
const EnforcementAuditEventIdSchema = brandedNonEmptyStringSchema('EnforcementAuditEventId');
const EnforcementTimerEventIdSchema = brandedNonEmptyStringSchema('EnforcementTimerEventId');
const EnforcementTimerStateIdSchema = brandedNonEmptyStringSchema('EnforcementTimerStateId');
const EnforcementRollbackTokenSchema = brandedNonEmptyStringSchema('EnforcementRollbackToken');
const EnforcementIdempotencyKeySchema = brandedNonEmptyStringSchema('EnforcementIdempotencyKey');
const EnforcementStatusReasonSchema = brandedNonEmptyStringSchema('EnforcementStatusReason');
const EnforcementJournalSequenceSchema = brandedNonEmptyStringSchema('EnforcementJournalSequence');

const EnforcementIntentSourceSchema = withParser(
  Schema.Literal('parent-portal', 'parent-rule', 'local-policy-evaluator', 'system-recovery')
);

export const EnforcementIntentSource = {
  ParentPortal: EnforcementIntentSourceSchema.parse('parent-portal'),
  ParentRule: EnforcementIntentSourceSchema.parse('parent-rule'),
  LocalPolicyEvaluator: EnforcementIntentSourceSchema.parse('local-policy-evaluator'),
  SystemRecovery: EnforcementIntentSourceSchema.parse('system-recovery'),
} as const;

export const EnforcementAdapterKindSchema = withParser(
  Schema.Literal('process-control', 'network-control', 'managed-browser-control', 'timer-control')
);

export const EnforcementModeSchema = withParser(
  Schema.Literal('terminate-process', 'block-process', 'temporary-block', 'time-limit', 'ask-parent', 'observe-only')
);

export const EnforcementCapabilityStateSchema = withParser(
  Schema.Literal('supported', 'unavailable', 'degraded', 'dry-run', 'observe-only', 'manual-required')
);

const EnforcementUnavailableReasonSchema = withParser(
  Schema.Literal(
    'unsupported-platform',
    'unsupported-action',
    'missing-permission',
    'missing-dependency',
    'adapter-unavailable',
    'adapter-error',
    'manual-required'
  )
);

const EnforcementPermissionStateSchema = withParser(
  Schema.Literal('allowed', 'missing-permission', 'not-required', 'unknown')
);

const EnforcementDependencyStateSchema = withParser(Schema.Literal('installed', 'missing', 'not-required', 'unknown'));

const EnforcementResultStatusSchema = withParser(
  Schema.Literal(
    'would-enforce',
    'actually-enforced',
    'unavailable',
    'failed',
    'expired',
    'rolled-back',
    'superseded',
    'no-op'
  )
);

const EnforcementRollbackStateSchema = withParser(
  Schema.Literal('not-required', 'available', 'requested', 'completed', 'unavailable', 'failed')
);

const EnforcementAdapterResultCodeSchema = withParser(
  Schema.Literal(
    'process-terminated',
    'process-already-exited',
    'left-running-observe-only',
    'dry-run-no-action',
    'unsupported-platform',
    'adapter-unavailable',
    'adapter-failed',
    'timer-expired',
    'rollback-completed',
    'no-op'
  )
);

const EnforcementTimerEventKindSchema = withParser(
  Schema.Literal(
    'created',
    'extended',
    'expired',
    'cancelled',
    'restart-recovered',
    'rollback-requested',
    'rollback-completed',
    'recovery-needed',
    'unavailable'
  )
);

const EnforcementAuditEventKindSchema = withParser(
  Schema.Literal(
    'attempted',
    'succeeded',
    'failed',
    'rollback-requested',
    'rollback-completed',
    'expired',
    'unavailable',
    'cancelled'
  )
);

const EnforcementCapabilityStatusBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  platform: ParentPlatformSchema,
  adapterKind: EnforcementAdapterKindSchema,
  capabilityState: EnforcementCapabilityStateSchema,
  permissionState: EnforcementPermissionStateSchema,
  dependencyState: EnforcementDependencyStateSchema,
  supportedActions: Schema.Array(EnforcementModeSchema),
  degradedReason: Schema.Union(EnforcementStatusReasonSchema, Schema.Null),
  lastCheckedAt: ParentTimestampSchema,
});

export const EnforcementCapabilityStatusSchema = withParser(
  EnforcementCapabilityStatusBaseSchema.pipe(
    Schema.filter(
      (capability) =>
        enforcementCapabilityStatusReasonIsConsistent(capability) ||
        'Expected unavailable and degraded enforcement capabilities to include typed degraded reason'
    )
  )
);

export const EnforcementUnavailableStatusSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    capability: EnforcementCapabilityStatusSchema,
    unavailableReason: EnforcementUnavailableReasonSchema,
    retryable: Schema.Boolean,
    checkedAt: ParentTimestampSchema,
  })
);

export const EnforcementIntentSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    intentId: EnforcementIntentIdSchema,
    source: EnforcementIntentSourceSchema,
    actor: Schema.Union(ParentActorReferenceSchema, Schema.Null),
    device: ParentDeviceReferenceSchema,
    policyDecisionId: PolicyDecisionIdSchema,
    target: PolicyTargetSchema,
    requestedAction: PolicyActionSchema,
    evidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
    parentApproval: Schema.Union(ParentActionReferenceSchema, Schema.Null),
    idempotencyKey: EnforcementIdempotencyKeySchema,
  })
);

export const EnforcementActionSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    actionId: EnforcementActionIdSchema,
    intentId: EnforcementIntentIdSchema,
    policyDecisionId: PolicyDecisionIdSchema,
    policyAction: PolicyActionSchema,
    adapterKind: EnforcementAdapterKindSchema,
    platform: ParentPlatformSchema,
    target: PolicyTargetSchema,
    mode: EnforcementModeSchema,
    capability: EnforcementCapabilityStatusSchema,
    reasonCodes: Schema.Array(PolicyReasonCodeSchema),
    evidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
    localAiResultId: Schema.Union(LocalAiResultReferenceIdSchema, Schema.Null),
    parentApproval: Schema.Union(ParentActionReferenceSchema, Schema.Null),
    dryRun: Schema.Boolean,
    requestedAt: ParentTimestampSchema,
    expiresAt: Schema.Union(ParentTimestampSchema, Schema.Null),
    rollbackToken: Schema.Union(EnforcementRollbackTokenSchema, Schema.Null),
  })
);

const EnforcementResultBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  resultId: EnforcementResultIdSchema,
  actionId: EnforcementActionIdSchema,
  status: EnforcementResultStatusSchema,
  adapterResultCode: EnforcementAdapterResultCodeSchema,
  startedAt: ParentTimestampSchema,
  completedAt: Schema.Union(ParentTimestampSchema, Schema.Null),
  rollbackToken: Schema.Union(EnforcementRollbackTokenSchema, Schema.Null),
  rollbackState: EnforcementRollbackStateSchema,
  unavailableReason: Schema.Union(EnforcementStatusReasonSchema, Schema.Null),
  unavailableStatus: Schema.Union(EnforcementUnavailableStatusSchema, Schema.Null),
  failedReason: Schema.Union(EnforcementStatusReasonSchema, Schema.Null),
  nextCheckAt: Schema.Union(ParentTimestampSchema, Schema.Null),
  capability: EnforcementCapabilityStatusSchema,
});

export const EnforcementResultSchema = withParser(
  EnforcementResultBaseSchema.pipe(
    Schema.filter(
      (result) =>
        enforcementUnavailableStatusIsConsistent(result) ||
        'Expected unavailable enforcement results to include typed unavailable status'
    )
  )
);

const EnforcementAuditEventBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  auditEventId: EnforcementAuditEventIdSchema,
  auditEventKind: EnforcementAuditEventKindSchema,
  action: EnforcementActionSchema,
  result: EnforcementResultSchema,
  capability: EnforcementCapabilityStatusSchema,
  unavailableStatus: Schema.Union(EnforcementUnavailableStatusSchema, Schema.Null),
  policyVersion: ParentPolicyVersionSchema,
  evidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
  actor: Schema.Union(ParentActorReferenceSchema, Schema.Null),
  parentOverride: Schema.Union(ParentActionReferenceSchema, Schema.Null),
  journalSequence: Schema.Union(EnforcementJournalSequenceSchema, Schema.Null),
  observedAt: ParentTimestampSchema,
});

export const EnforcementAuditEventSchema = withParser(
  EnforcementAuditEventBaseSchema.pipe(
    Schema.filter(
      (auditEvent) =>
        enforcementAuditEventBoundaryIsConsistent(auditEvent) ||
        'Expected enforcement audit events to mirror result capability and unavailable status'
    )
  )
);

const EnforcementTimerEventBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  timerEventId: EnforcementTimerEventIdSchema,
  timerEventKind: EnforcementTimerEventKindSchema,
  actionId: EnforcementActionIdSchema,
  policyDecisionId: PolicyDecisionIdSchema,
  evidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
  scheduledAt: ParentTimestampSchema,
  effectiveAt: Schema.Union(ParentTimestampSchema, Schema.Null),
  rollbackToken: Schema.Union(EnforcementRollbackTokenSchema, Schema.Null),
  recoveredAfterRestart: Schema.Boolean,
  unavailableReason: Schema.Union(EnforcementUnavailableReasonSchema, Schema.Null),
});

export const EnforcementTimerEventSchema = withParser(
  EnforcementTimerEventBaseSchema.pipe(
    Schema.filter(
      (timerEvent) =>
        enforcementTimerUnavailableReasonIsConsistent(timerEvent) ||
        'Expected unavailable and recovery-needed enforcement timer events to include typed unavailable reason'
    )
  )
);

const EnforcementActiveTimerStateBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  stateId: EnforcementTimerStateIdSchema,
  action: EnforcementActionSchema,
  result: EnforcementResultSchema,
  auditEvent: EnforcementAuditEventSchema,
  timerEvent: EnforcementTimerEventSchema,
  storedAt: ParentTimestampSchema,
});

export const EnforcementActiveTimerStateSchema = withParser(
  EnforcementActiveTimerStateBaseSchema.pipe(
    Schema.filter(
      (state) =>
        enforcementActiveTimerStateIsConsistent(state) ||
        'Expected active enforcement timer state to preserve action/result/audit/timer identity'
    )
  )
);

export type EnforcementCapabilityStatus = Infer<typeof EnforcementCapabilityStatusSchema>;
export type EnforcementUnavailableStatus = Infer<typeof EnforcementUnavailableStatusSchema>;
export type EnforcementIntent = Infer<typeof EnforcementIntentSchema>;
export type EnforcementAction = Infer<typeof EnforcementActionSchema>;
export type EnforcementResult = Infer<typeof EnforcementResultSchema>;
export type EnforcementAuditEvent = Infer<typeof EnforcementAuditEventSchema>;
export type EnforcementTimerEvent = Infer<typeof EnforcementTimerEventSchema>;
export type EnforcementActiveTimerState = Infer<typeof EnforcementActiveTimerStateSchema>;

export function enforcementCapabilityStatusReasonIsConsistent(
  capability: GeneratedEnforcementCapabilityStatusLike
): boolean {
  if (
    capability.capabilityState === 'unavailable' ||
    capability.capabilityState === 'degraded' ||
    capability.capabilityState === 'manual-required'
  ) {
    return capability.degradedReason !== null;
  }

  return true;
}

export function enforcementUnavailableStatusIsConsistent(result: GeneratedEnforcementResultLike): boolean {
  if (result.status === 'unavailable') {
    return result.unavailableStatus !== null;
  }

  return result.unavailableStatus === null;
}

export function enforcementCapabilityStatusesMatch(
  left: GeneratedEnforcementCapabilityStatusLike,
  right: GeneratedEnforcementCapabilityStatusLike
): boolean {
  return (
    left.schemaVersion === right.schemaVersion &&
    left.platform === right.platform &&
    left.adapterKind === right.adapterKind &&
    left.capabilityState === right.capabilityState &&
    left.permissionState === right.permissionState &&
    left.dependencyState === right.dependencyState &&
    left.degradedReason === right.degradedReason &&
    left.lastCheckedAt === right.lastCheckedAt &&
    left.supportedActions.length === right.supportedActions.length &&
    left.supportedActions.every((action, index) => action === right.supportedActions[index])
  );
}

export function enforcementUnavailableStatusesMatch(
  left: GeneratedEnforcementUnavailableStatusLike | null,
  right: GeneratedEnforcementUnavailableStatusLike | null
): boolean {
  if (left === null || right === null) {
    return left === right;
  }

  return (
    left.schemaVersion === right.schemaVersion &&
    left.unavailableReason === right.unavailableReason &&
    left.retryable === right.retryable &&
    left.checkedAt === right.checkedAt &&
    enforcementCapabilityStatusesMatch(left.capability, right.capability)
  );
}

export function enforcementAuditEventBoundaryIsConsistent(auditEvent: GeneratedEnforcementAuditEventLike): boolean {
  if (!enforcementCapabilityStatusesMatch(auditEvent.capability, auditEvent.result.capability)) {
    return false;
  }

  if (!enforcementUnavailableStatusesMatch(auditEvent.unavailableStatus, auditEvent.result.unavailableStatus)) {
    return false;
  }

  if (auditEvent.result.status === 'unavailable') {
    return auditEvent.auditEventKind === 'unavailable' && auditEvent.unavailableStatus !== null;
  }

  return auditEvent.auditEventKind !== 'unavailable' && auditEvent.unavailableStatus === null;
}

export function enforcementTimerUnavailableReasonIsConsistent(timerEvent: GeneratedEnforcementTimerEventLike): boolean {
  if (timerEvent.timerEventKind === 'unavailable' || timerEvent.timerEventKind === 'recovery-needed') {
    return timerEvent.unavailableReason !== null;
  }

  return timerEvent.unavailableReason === null;
}

export function enforcementActiveTimerStateIsConsistent(state: GeneratedEnforcementActiveTimerStateLike): boolean {
  return (
    state.action.actionId === state.result.actionId &&
    state.action.actionId === state.auditEvent.action.actionId &&
    state.action.actionId === state.timerEvent.actionId &&
    state.action.policyDecisionId === state.timerEvent.policyDecisionId
  );
}

export const EnforcementAdapterKind = {
  ProcessControl: EnforcementAdapterKindSchema.parse('process-control'),
  NetworkControl: EnforcementAdapterKindSchema.parse('network-control'),
  ManagedBrowserControl: EnforcementAdapterKindSchema.parse('managed-browser-control'),
  TimerControl: EnforcementAdapterKindSchema.parse('timer-control'),
} as const;

export const EnforcementMode = {
  TerminateProcess: EnforcementModeSchema.parse('terminate-process'),
  BlockProcess: EnforcementModeSchema.parse('block-process'),
  TemporaryBlock: EnforcementModeSchema.parse('temporary-block'),
  TimeLimit: EnforcementModeSchema.parse('time-limit'),
  AskParent: EnforcementModeSchema.parse('ask-parent'),
  ObserveOnly: EnforcementModeSchema.parse('observe-only'),
} as const;

export const EnforcementCapabilityState = {
  Supported: EnforcementCapabilityStateSchema.parse('supported'),
  Unavailable: EnforcementCapabilityStateSchema.parse('unavailable'),
  Degraded: EnforcementCapabilityStateSchema.parse('degraded'),
  DryRun: EnforcementCapabilityStateSchema.parse('dry-run'),
  ObserveOnly: EnforcementCapabilityStateSchema.parse('observe-only'),
  ManualRequired: EnforcementCapabilityStateSchema.parse('manual-required'),
} as const;

export const EnforcementUnavailableReason = {
  UnsupportedPlatform: EnforcementUnavailableReasonSchema.parse('unsupported-platform'),
  UnsupportedAction: EnforcementUnavailableReasonSchema.parse('unsupported-action'),
  MissingPermission: EnforcementUnavailableReasonSchema.parse('missing-permission'),
  MissingDependency: EnforcementUnavailableReasonSchema.parse('missing-dependency'),
  AdapterUnavailable: EnforcementUnavailableReasonSchema.parse('adapter-unavailable'),
  AdapterError: EnforcementUnavailableReasonSchema.parse('adapter-error'),
  ManualRequired: EnforcementUnavailableReasonSchema.parse('manual-required'),
} as const;

export const EnforcementResultStatus = {
  WouldEnforce: EnforcementResultStatusSchema.parse('would-enforce'),
  ActuallyEnforced: EnforcementResultStatusSchema.parse('actually-enforced'),
  Unavailable: EnforcementResultStatusSchema.parse('unavailable'),
  Failed: EnforcementResultStatusSchema.parse('failed'),
  Expired: EnforcementResultStatusSchema.parse('expired'),
  RolledBack: EnforcementResultStatusSchema.parse('rolled-back'),
  Superseded: EnforcementResultStatusSchema.parse('superseded'),
  NoOp: EnforcementResultStatusSchema.parse('no-op'),
} as const;

export const EnforcementRollbackState = {
  NotRequired: EnforcementRollbackStateSchema.parse('not-required'),
  Available: EnforcementRollbackStateSchema.parse('available'),
  Requested: EnforcementRollbackStateSchema.parse('requested'),
  Completed: EnforcementRollbackStateSchema.parse('completed'),
  Unavailable: EnforcementRollbackStateSchema.parse('unavailable'),
  Failed: EnforcementRollbackStateSchema.parse('failed'),
} as const;

export const EnforcementAdapterResultCode = {
  ProcessTerminated: EnforcementAdapterResultCodeSchema.parse('process-terminated'),
  ProcessAlreadyExited: EnforcementAdapterResultCodeSchema.parse('process-already-exited'),
  LeftRunningObserveOnly: EnforcementAdapterResultCodeSchema.parse('left-running-observe-only'),
  DryRunNoAction: EnforcementAdapterResultCodeSchema.parse('dry-run-no-action'),
  UnsupportedPlatform: EnforcementAdapterResultCodeSchema.parse('unsupported-platform'),
  AdapterUnavailable: EnforcementAdapterResultCodeSchema.parse('adapter-unavailable'),
  AdapterFailed: EnforcementAdapterResultCodeSchema.parse('adapter-failed'),
  TimerExpired: EnforcementAdapterResultCodeSchema.parse('timer-expired'),
  RollbackCompleted: EnforcementAdapterResultCodeSchema.parse('rollback-completed'),
  NoOp: EnforcementAdapterResultCodeSchema.parse('no-op'),
} as const;

export const EnforcementTimerEventKind = {
  Created: EnforcementTimerEventKindSchema.parse('created'),
  Extended: EnforcementTimerEventKindSchema.parse('extended'),
  Expired: EnforcementTimerEventKindSchema.parse('expired'),
  Cancelled: EnforcementTimerEventKindSchema.parse('cancelled'),
  RestartRecovered: EnforcementTimerEventKindSchema.parse('restart-recovered'),
  RollbackRequested: EnforcementTimerEventKindSchema.parse('rollback-requested'),
  RollbackCompleted: EnforcementTimerEventKindSchema.parse('rollback-completed'),
  RecoveryNeeded: EnforcementTimerEventKindSchema.parse('recovery-needed'),
  Unavailable: EnforcementTimerEventKindSchema.parse('unavailable'),
} as const;

export const EnforcementAuditEventKind = {
  Attempted: EnforcementAuditEventKindSchema.parse('attempted'),
  Succeeded: EnforcementAuditEventKindSchema.parse('succeeded'),
  Failed: EnforcementAuditEventKindSchema.parse('failed'),
  RollbackRequested: EnforcementAuditEventKindSchema.parse('rollback-requested'),
  RollbackCompleted: EnforcementAuditEventKindSchema.parse('rollback-completed'),
  Expired: EnforcementAuditEventKindSchema.parse('expired'),
  Unavailable: EnforcementAuditEventKindSchema.parse('unavailable'),
  Cancelled: EnforcementAuditEventKindSchema.parse('cancelled'),
} as const;

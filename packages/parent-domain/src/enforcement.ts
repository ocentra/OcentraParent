import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ParentActionReferenceSchema,
  ParentActorReferenceSchema,
  ParentDeviceReferenceSchema,
  ParentEvidenceReferenceSchema,
} from './references';
import {
  ParentContractSchemaVersionSchema,
  ParentPlatformSchema,
  ParentPolicyVersionSchema,
  ParentTimestampSchema,
} from './reference-primitives';
import {
  LocalAiResultReferenceIdSchema,
  PolicyActionSchema,
  PolicyDecisionIdSchema,
  PolicyReasonCodeSchema,
  PolicyTargetSchema,
} from './policy';

const NonEmptyEnforcementText = Schema.String.pipe(Schema.minLength(1));

const EnforcementIntentIdSchema = NonEmptyEnforcementText.pipe(Schema.brand('EnforcementIntentId'));
const EnforcementActionIdSchema = NonEmptyEnforcementText.pipe(Schema.brand('EnforcementActionId'));
const EnforcementResultIdSchema = NonEmptyEnforcementText.pipe(Schema.brand('EnforcementResultId'));
const EnforcementAuditEventIdSchema = NonEmptyEnforcementText.pipe(Schema.brand('EnforcementAuditEventId'));
const EnforcementTimerEventIdSchema = NonEmptyEnforcementText.pipe(Schema.brand('EnforcementTimerEventId'));
const EnforcementTimerStateIdSchema = NonEmptyEnforcementText.pipe(Schema.brand('EnforcementTimerStateId'));
const EnforcementRollbackTokenSchema = NonEmptyEnforcementText.pipe(Schema.brand('EnforcementRollbackToken'));
const EnforcementIdempotencyKeySchema = NonEmptyEnforcementText.pipe(Schema.brand('EnforcementIdempotencyKey'));
const EnforcementStatusReasonSchema = NonEmptyEnforcementText.pipe(Schema.brand('EnforcementStatusReason'));
const EnforcementJournalSequenceSchema = NonEmptyEnforcementText.pipe(Schema.brand('EnforcementJournalSequence'));

const EnforcementIntentSourceSchema = withParser(
  Schema.Literal('parent-portal', 'parent-rule', 'local-policy-evaluator', 'system-recovery')
);

const EnforcementAdapterKindSchema = withParser(
  Schema.Literal('process-control', 'network-control', 'managed-browser-control', 'timer-control')
);

const EnforcementModeSchema = withParser(
  Schema.Literal('terminate-process', 'block-process', 'temporary-block', 'time-limit', 'ask-parent', 'observe-only')
);

const EnforcementCapabilityStateSchema = withParser(
  Schema.Literal('supported', 'unavailable', 'degraded', 'dry-run', 'observe-only')
);

const EnforcementUnavailableReasonSchema = withParser(
  Schema.Literal(
    'unsupported-platform',
    'unsupported-action',
    'missing-permission',
    'missing-dependency',
    'adapter-unavailable',
    'adapter-error'
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

type EnforcementCapabilityStatusCandidate = Infer<typeof EnforcementCapabilityStatusBaseSchema>;

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

type EnforcementResultCandidate = Infer<typeof EnforcementResultBaseSchema>;

export const EnforcementResultSchema = withParser(
  EnforcementResultBaseSchema.pipe(
    Schema.filter(
      (result) =>
        enforcementUnavailableStatusIsConsistent(result) ||
        'Expected unavailable enforcement results to include typed unavailable status'
    )
  )
);

function enforcementUnavailableStatusIsConsistent(result: EnforcementResultCandidate): boolean {
  if (result.status === 'unavailable') {
    return result.unavailableStatus !== null;
  }

  return result.unavailableStatus === null;
}

function enforcementCapabilityStatusReasonIsConsistent(capability: EnforcementCapabilityStatusCandidate): boolean {
  if (capability.capabilityState === 'unavailable' || capability.capabilityState === 'degraded') {
    return capability.degradedReason !== null;
  }

  return true;
}

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

type EnforcementAuditEventCandidate = Infer<typeof EnforcementAuditEventBaseSchema>;

export const EnforcementAuditEventSchema = withParser(
  EnforcementAuditEventBaseSchema.pipe(
    Schema.filter(
      (auditEvent) =>
        enforcementAuditEventBoundaryIsConsistent(auditEvent) ||
        'Expected enforcement audit events to mirror result capability and unavailable status'
    )
  )
);

function enforcementAuditEventBoundaryIsConsistent(auditEvent: EnforcementAuditEventCandidate): boolean {
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

function enforcementCapabilityStatusesMatch(
  left: EnforcementCapabilityStatusCandidate,
  right: EnforcementCapabilityStatusCandidate
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

function enforcementUnavailableStatusesMatch(
  left: EnforcementUnavailableStatus | null,
  right: EnforcementUnavailableStatus | null
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

type EnforcementTimerEventCandidate = Infer<typeof EnforcementTimerEventBaseSchema>;

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

type EnforcementActiveTimerStateCandidate = Infer<typeof EnforcementActiveTimerStateBaseSchema>;

export const EnforcementActiveTimerStateSchema = withParser(
  EnforcementActiveTimerStateBaseSchema.pipe(
    Schema.filter(
      (state) =>
        enforcementActiveTimerStateIsConsistent(state) ||
        'Expected active enforcement timer state to preserve action/result/audit/timer identity'
    )
  )
);

function enforcementActiveTimerStateIsConsistent(state: EnforcementActiveTimerStateCandidate): boolean {
  return (
    state.action.actionId === state.result.actionId &&
    state.action.actionId === state.auditEvent.action.actionId &&
    state.action.actionId === state.timerEvent.actionId &&
    state.action.policyDecisionId === state.timerEvent.policyDecisionId
  );
}

function enforcementTimerUnavailableReasonIsConsistent(timerEvent: EnforcementTimerEventCandidate): boolean {
  if (timerEvent.timerEventKind === 'unavailable' || timerEvent.timerEventKind === 'recovery-needed') {
    return timerEvent.unavailableReason !== null;
  }

  return timerEvent.unavailableReason === null;
}

export type EnforcementCapabilityStatus = Infer<typeof EnforcementCapabilityStatusSchema>;
export type EnforcementUnavailableStatus = Infer<typeof EnforcementUnavailableStatusSchema>;
export type EnforcementIntent = Infer<typeof EnforcementIntentSchema>;
export type EnforcementAction = Infer<typeof EnforcementActionSchema>;
export type EnforcementResult = Infer<typeof EnforcementResultSchema>;
export type EnforcementAuditEvent = Infer<typeof EnforcementAuditEventSchema>;
export type EnforcementTimerEvent = Infer<typeof EnforcementTimerEventSchema>;
export type EnforcementActiveTimerState = Infer<typeof EnforcementActiveTimerStateSchema>;

export const EnforcementIntentSource = {
  ParentPortal: EnforcementIntentSourceSchema.parse('parent-portal'),
  ParentRule: EnforcementIntentSourceSchema.parse('parent-rule'),
  LocalPolicyEvaluator: EnforcementIntentSourceSchema.parse('local-policy-evaluator'),
  SystemRecovery: EnforcementIntentSourceSchema.parse('system-recovery'),
} as const;

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
} as const;

export const EnforcementUnavailableReason = {
  UnsupportedPlatform: EnforcementUnavailableReasonSchema.parse('unsupported-platform'),
  UnsupportedAction: EnforcementUnavailableReasonSchema.parse('unsupported-action'),
  MissingPermission: EnforcementUnavailableReasonSchema.parse('missing-permission'),
  MissingDependency: EnforcementUnavailableReasonSchema.parse('missing-dependency'),
  AdapterUnavailable: EnforcementUnavailableReasonSchema.parse('adapter-unavailable'),
  AdapterError: EnforcementUnavailableReasonSchema.parse('adapter-error'),
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

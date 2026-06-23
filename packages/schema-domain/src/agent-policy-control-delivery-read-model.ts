import { AgentLogSnapshotSchema, LogFieldsSchema, LogLevelSchema } from './logging-contracts';
import {
  AgentCorrelationIdSchema,
  AgentEventIdSchema,
  AgentPeerSchema,
  AgentProtocolSchemaVersion,
  AgentTimestampSchema,
} from './event-primitives';
import { type Infer, NonEmptyStringSchema, Schema, withParser } from './effect';

const PolicyControlDeliveryCountSchema = Schema.Number.pipe(Schema.nonNegative(), Schema.int());
const NullablePolicyControlDeliveryTextSchema = Schema.Union(NonEmptyStringSchema, Schema.Null);

export const PolicyControlDeliveryReadModelSchemaVersion = 'policy-control-delivery-read-model.v1' as const;
export const PolicyControlDeliveryReadModelPayloadField = 'policyControlDeliveryReadModel' as const;
export const PolicyControlDeliveryReadModelReportedEventName =
  'agent.policy-control.delivery.read-model.reported' as const;

export const AgentPolicyControlDeliveryDomainSchema = withParser(
  Schema.Literal('browser', 'app-game', 'network', 'screen', 'tracking')
);
export const AgentPolicyControlDeliveryIntentStateSchema = withParser(
  Schema.Literal('drafted', 'previewed', 'confirmed')
);
export const AgentPolicyControlDeliveryTransportStateSchema = withParser(
  Schema.Literal(
    'queued',
    'delivered',
    'retry-scheduled',
    'offline',
    'permission-blocked',
    'platform-blocked',
    'account-blocked',
    'superseded',
    'expired-before-delivery'
  )
);
export const AgentPolicyControlDeliveryAckStateSchema = withParser(
  Schema.Literal('not-required', 'pending', 'acknowledged', 'rejected', 'manual-required', 'stale')
);
export const AgentPolicyControlDeliveryApplyStateSchema = withParser(
  Schema.Literal(
    'pending',
    'applied',
    'partially-applied',
    'rejected',
    'superseded',
    'rolled-back',
    'expired-before-delivery',
    'degraded',
    'manual-required'
  )
);
export const AgentPolicyControlDeliveryParentVisibleStateSchema = withParser(
  Schema.Literal(
    'pending',
    'acknowledged',
    'degraded',
    'manual-required',
    'applied',
    'partially-applied',
    'rejected',
    'rolled-back',
    'superseded',
    'expired-before-delivery'
  )
);
export const AgentPolicyControlDeliveryBlockedReasonSchema = withParser(
  Schema.Literal(
    'offline-child',
    'permission-loss',
    'platform-state',
    'account-state',
    'retry-exhausted',
    'stale-policy',
    'superseded-policy'
  )
);

const AgentPolicyControlDeliveryDomainStateBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(PolicyControlDeliveryReadModelSchemaVersion),
  domainId: AgentPolicyControlDeliveryDomainSchema,
  deliveryState: Schema.Literal(
    'pending',
    'delivered',
    'acknowledged',
    'applied',
    'degraded',
    'manual-required',
    'rejected',
    'rolled-back',
    'superseded',
    'expired-before-delivery',
    'blocked'
  ),
  auditRefs: Schema.Array(NonEmptyStringSchema).pipe(
    Schema.filter((value) => value.length > 0 || 'Expected per-domain audit refs')
  ),
  lastAckEventId: NullablePolicyControlDeliveryTextSchema,
  lastAppliedEventId: NullablePolicyControlDeliveryTextSchema,
});

type PolicyControlDeliveryReadModelDomainStateCandidate = Infer<typeof AgentPolicyControlDeliveryDomainStateBaseSchema>;

export const AgentPolicyControlDeliveryDomainStateSchema = withParser(
  AgentPolicyControlDeliveryDomainStateBaseSchema.pipe(
    Schema.filter((state: PolicyControlDeliveryReadModelDomainStateCandidate) => {
      const validation = validateDomainState(state);
      return validation === true
        ? true
        : 'Expected per-domain delivery states to keep acknowledgement/apply refs aligned';
    })
  )
);

const PolicyControlDeliveryReadModelRowBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(PolicyControlDeliveryReadModelSchemaVersion),
  deliveryRowId: NonEmptyStringSchema,
  policyVersionRef: NonEmptyStringSchema,
  childDeviceId: NonEmptyStringSchema,
  generatedAt: NonEmptyStringSchema,
  parentVisibleState: AgentPolicyControlDeliveryParentVisibleStateSchema,
  intentState: AgentPolicyControlDeliveryIntentStateSchema,
  transportState: AgentPolicyControlDeliveryTransportStateSchema,
  acknowledgementRequired: Schema.Boolean,
  ackState: AgentPolicyControlDeliveryAckStateSchema,
  applyState: AgentPolicyControlDeliveryApplyStateSchema,
  blockedReason: Schema.Union(AgentPolicyControlDeliveryBlockedReasonSchema, Schema.Null),
  latestAuditEventId: NullablePolicyControlDeliveryTextSchema,
  auditRefs: Schema.Array(NonEmptyStringSchema).pipe(
    Schema.filter((value) => value.length > 0 || 'Expected delivery audit refs')
  ),
  retryScheduleRefs: Schema.Array(NonEmptyStringSchema),
  manualProofRequirements: Schema.Array(NonEmptyStringSchema),
  domainStates: Schema.Array(AgentPolicyControlDeliveryDomainStateSchema).pipe(
    Schema.filter((value) => value.length > 0 || 'Expected per-domain delivery states')
  ),
});

type PolicyControlDeliveryReadModelRowCandidate = Infer<typeof PolicyControlDeliveryReadModelRowBaseSchema>;

export const PolicyControlDeliveryReadModelRowSchema = withParser(
  PolicyControlDeliveryReadModelRowBaseSchema.pipe(
    Schema.filter((row: PolicyControlDeliveryReadModelRowCandidate) => {
      const validation = validateRowState(row);
      return validation === true
        ? true
        : 'Expected delivery rows to keep parent-visible, ack, apply, and manual states honest';
    })
  )
);

const PolicyControlDeliveryReadModelSnapshotBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(PolicyControlDeliveryReadModelSchemaVersion),
  readModelId: NonEmptyStringSchema,
  generatedAt: NonEmptyStringSchema,
  rows: Schema.Array(PolicyControlDeliveryReadModelRowSchema).pipe(
    Schema.filter((value) => value.length > 0 || 'Expected at least one delivery row')
  ),
  pendingCount: PolicyControlDeliveryCountSchema,
  acknowledgedCount: PolicyControlDeliveryCountSchema,
  degradedCount: PolicyControlDeliveryCountSchema,
  manualRequiredCount: PolicyControlDeliveryCountSchema,
  appliedCount: PolicyControlDeliveryCountSchema,
  partiallyAppliedCount: PolicyControlDeliveryCountSchema,
  rejectedCount: PolicyControlDeliveryCountSchema,
  rolledBackCount: PolicyControlDeliveryCountSchema,
  supersededCount: PolicyControlDeliveryCountSchema,
  expiredBeforeDeliveryCount: PolicyControlDeliveryCountSchema,
  parentVisibleState: AgentPolicyControlDeliveryParentVisibleStateSchema,
  activationBlocked: Schema.Boolean,
  nonClaims: Schema.Array(NonEmptyStringSchema),
});

type PolicyControlDeliveryReadModelSnapshotCandidate = Infer<typeof PolicyControlDeliveryReadModelSnapshotBaseSchema>;

export const PolicyControlDeliveryReadModelSnapshotSchema = withParser(
  PolicyControlDeliveryReadModelSnapshotBaseSchema.pipe(
    Schema.filter((snapshot: PolicyControlDeliveryReadModelSnapshotCandidate) => {
      const validation = validateSnapshotState(snapshot);
      return validation === true ? true : 'Expected delivery snapshot counts and severity ordering to match its rows';
    })
  )
);

export const PolicyControlDeliveryReadModelEventEnvelopeSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolSchemaVersion),
    eventId: AgentEventIdSchema,
    correlationId: AgentCorrelationIdSchema,
    sentAt: AgentTimestampSchema,
    source: AgentPeerSchema,
    target: AgentPeerSchema,
    event: Schema.Literal(PolicyControlDeliveryReadModelReportedEventName),
    severity: LogLevelSchema,
    payload: LogFieldsSchema,
    snapshot: Schema.Union(AgentLogSnapshotSchema, Schema.Null),
  })
);

export type AgentPolicyControlDeliveryDomain = Infer<typeof AgentPolicyControlDeliveryDomainSchema>;
export type AgentPolicyControlDeliveryIntentState = Infer<typeof AgentPolicyControlDeliveryIntentStateSchema>;
export type AgentPolicyControlDeliveryTransportState = Infer<typeof AgentPolicyControlDeliveryTransportStateSchema>;
export type AgentPolicyControlDeliveryAckState = Infer<typeof AgentPolicyControlDeliveryAckStateSchema>;
export type AgentPolicyControlDeliveryApplyState = Infer<typeof AgentPolicyControlDeliveryApplyStateSchema>;
export type AgentPolicyControlDeliveryParentVisibleState = Infer<
  typeof AgentPolicyControlDeliveryParentVisibleStateSchema
>;
export type AgentPolicyControlDeliveryBlockedReason = Infer<typeof AgentPolicyControlDeliveryBlockedReasonSchema>;
export type PolicyControlDeliveryReadModelDomainState = Infer<typeof AgentPolicyControlDeliveryDomainStateSchema>;
export type PolicyControlDeliveryReadModelRow = Infer<typeof PolicyControlDeliveryReadModelRowSchema>;
export type PolicyControlDeliveryReadModelSnapshot = Infer<typeof PolicyControlDeliveryReadModelSnapshotSchema>;
export type PolicyControlDeliveryReadModelEventEnvelope = Infer<
  typeof PolicyControlDeliveryReadModelEventEnvelopeSchema
>;
type PolicyControlDeliveryRowValidator = (row: PolicyControlDeliveryReadModelRowCandidate) => true | string;
type PolicyControlDeliverySnapshotValidator = (
  snapshot: PolicyControlDeliveryReadModelSnapshotCandidate
) => true | string;
type PolicyControlDeliverySnapshotCountField =
  | 'pendingCount'
  | 'acknowledgedCount'
  | 'degradedCount'
  | 'manualRequiredCount'
  | 'appliedCount'
  | 'partiallyAppliedCount'
  | 'rejectedCount'
  | 'rolledBackCount'
  | 'supersededCount'
  | 'expiredBeforeDeliveryCount';

function validateDomainState(state: PolicyControlDeliveryReadModelDomainStateCandidate): true | string {
  if (state.deliveryState === 'acknowledged' && state.lastAckEventId === null) {
    return 'Acknowledged domain rows require an acknowledgement event id';
  }
  if (state.deliveryState === 'applied' && (state.lastAckEventId === null || state.lastAppliedEventId === null)) {
    return 'Applied domain rows require acknowledgement and applied event ids';
  }
  return true;
}

function validateRowState(row: PolicyControlDeliveryReadModelRowCandidate): true | string {
  const validators: readonly PolicyControlDeliveryRowValidator[] = [
    validateAcknowledgementRequirement,
    validateAcknowledgedRowState,
    validateAppliedRowState,
    validateDegradedRowState,
    validateManualRequiredRowState,
    validatePartialApplyRowState,
  ];

  return firstValidationFailure(row, validators);
}

function validateAcknowledgementRequirement(row: PolicyControlDeliveryReadModelRowCandidate): true | string {
  if (row.acknowledgementRequired && row.ackState === 'not-required') {
    return 'Acknowledgement-required rows must not use not-required ack state';
  }
  return true;
}

function validateAcknowledgedRowState(row: PolicyControlDeliveryReadModelRowCandidate): true | string {
  if (row.parentVisibleState === 'acknowledged' && (row.ackState !== 'acknowledged' || row.applyState !== 'pending')) {
    return 'Acknowledged rows must distinguish acknowledgement from applied policy state';
  }
  return true;
}

function validateAppliedRowState(row: PolicyControlDeliveryReadModelRowCandidate): true | string {
  if (
    row.parentVisibleState === 'applied' &&
    (row.ackState !== 'acknowledged' ||
      row.applyState !== 'applied' ||
      row.blockedReason !== null ||
      row.manualProofRequirements.length > 0 ||
      !row.domainStates.every((state) => state.deliveryState === 'applied'))
  ) {
    return 'Applied rows require acknowledged delivery, applied domain states, and no manual/degraded blockers';
  }
  return true;
}

function validateDegradedRowState(row: PolicyControlDeliveryReadModelRowCandidate): true | string {
  if (
    row.parentVisibleState === 'degraded' &&
    row.blockedReason === null &&
    row.retryScheduleRefs.length === 0 &&
    row.applyState !== 'degraded' &&
    row.transportState !== 'offline' &&
    row.transportState !== 'retry-scheduled' &&
    !row.domainStates.some((state) => state.deliveryState === 'degraded' || state.deliveryState === 'blocked')
  ) {
    return 'Degraded rows require offline, retry, blocked, or degraded domain evidence';
  }
  return true;
}

function validateManualRequiredRowState(row: PolicyControlDeliveryReadModelRowCandidate): true | string {
  if (row.parentVisibleState === 'manual-required' && row.manualProofRequirements.length === 0) {
    return 'Manual-required rows must surface explicit manual proof requirements';
  }
  return true;
}

function validatePartialApplyRowState(row: PolicyControlDeliveryReadModelRowCandidate): true | string {
  if (row.applyState === 'partially-applied' && !hasMixedDomainOutcome(row.domainStates)) {
    return 'Partially applied rows require mixed per-domain outcomes';
  }
  return true;
}

function validateSnapshotState(snapshot: PolicyControlDeliveryReadModelSnapshotCandidate): true | string {
  const validators: readonly PolicyControlDeliverySnapshotValidator[] = [
    validateSnapshotRowCounts,
    validateSnapshotParentVisibleState,
    validateSnapshotActivationBlockedState,
  ];

  return firstValidationFailure(snapshot, validators);
}

function validateSnapshotRowCounts(snapshot: PolicyControlDeliveryReadModelSnapshotCandidate): true | string {
  const expectations = [
    { field: 'pendingCount', state: 'pending', message: 'Pending count must match pending rows' },
    { field: 'acknowledgedCount', state: 'acknowledged', message: 'Acknowledged count must match acknowledged rows' },
    { field: 'degradedCount', state: 'degraded', message: 'Degraded count must match degraded rows' },
    {
      field: 'manualRequiredCount',
      state: 'manual-required',
      message: 'Manual-required count must match manual-required rows',
    },
    { field: 'appliedCount', state: 'applied', message: 'Applied count must match applied rows' },
    {
      field: 'partiallyAppliedCount',
      state: 'partially-applied',
      message: 'Partially applied count must match partially applied rows',
    },
    { field: 'rejectedCount', state: 'rejected', message: 'Rejected count must match rejected rows' },
    { field: 'rolledBackCount', state: 'rolled-back', message: 'Rolled-back count must match rolled-back rows' },
    { field: 'supersededCount', state: 'superseded', message: 'Superseded count must match superseded rows' },
    {
      field: 'expiredBeforeDeliveryCount',
      state: 'expired-before-delivery',
      message: 'Expired-before-delivery count must match expired rows',
    },
  ] as const satisfies ReadonlyArray<{
    field: PolicyControlDeliverySnapshotCountField;
    state: PolicyControlDeliveryReadModelRowCandidate['parentVisibleState'];
    message: string;
  }>;

  for (const expectation of expectations) {
    if (snapshot[expectation.field] !== countRows(snapshot.rows, expectation.state)) {
      return expectation.message;
    }
  }

  return true;
}

function validateSnapshotParentVisibleState(snapshot: PolicyControlDeliveryReadModelSnapshotCandidate): true | string {
  return snapshot.parentVisibleState === deriveSnapshotState(snapshot)
    ? true
    : 'Snapshot parent-visible state must match row severity ordering';
}

function validateSnapshotActivationBlockedState(
  snapshot: PolicyControlDeliveryReadModelSnapshotCandidate
): true | string {
  const shouldBeBlocked = snapshot.parentVisibleState !== 'applied';
  return snapshot.activationBlocked === shouldBeBlocked
    ? true
    : 'Activation blocked must reflect whether every row is applied';
}

function firstValidationFailure<T>(
  candidate: T,
  validators: readonly ((candidate: T) => true | string)[]
): true | string {
  for (const validator of validators) {
    const validation = validator(candidate);
    if (validation !== true) {
      return validation;
    }
  }

  return true;
}

function hasMixedDomainOutcome(states: readonly PolicyControlDeliveryReadModelDomainStateCandidate[]): boolean {
  const applied = states.some((state) => state.deliveryState === 'applied');
  const blocked = states.some(
    (state) =>
      state.deliveryState === 'degraded' ||
      state.deliveryState === 'manual-required' ||
      state.deliveryState === 'blocked'
  );
  return applied && blocked;
}

function countRows(
  rows: readonly PolicyControlDeliveryReadModelRowCandidate[],
  state: PolicyControlDeliveryReadModelRowCandidate['parentVisibleState']
): number {
  return rows.filter((row) => row.parentVisibleState === state).length;
}

function deriveSnapshotState(
  snapshot: PolicyControlDeliveryReadModelSnapshotCandidate
): PolicyControlDeliveryReadModelSnapshotCandidate['parentVisibleState'] {
  if (snapshot.manualRequiredCount > 0) return 'manual-required';
  if (snapshot.degradedCount > 0) return 'degraded';
  if (snapshot.partiallyAppliedCount > 0) return 'partially-applied';
  if (snapshot.rejectedCount > 0) return 'rejected';
  if (snapshot.rolledBackCount > 0) return 'rolled-back';
  if (snapshot.supersededCount === snapshot.rows.length) return 'superseded';
  if (snapshot.expiredBeforeDeliveryCount === snapshot.rows.length) return 'expired-before-delivery';
  if (snapshot.appliedCount === snapshot.rows.length) return 'applied';
  if (snapshot.acknowledgedCount > 0 && snapshot.pendingCount === 0) return 'acknowledged';
  return 'pending';
}

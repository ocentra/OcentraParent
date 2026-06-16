import {
  AgentLogSnapshotSchema,
  LogFieldsSchema,
  LogLevelSchema,
} from '@ocentra-parent/logging-domain/contracts';
import {
  AgentCorrelationIdSchema,
  AgentEventIdSchema,
  AgentPeerSchema,
  AgentProtocolSchemaVersion,
  AgentTimestampSchema,
} from '@ocentra-parent/event-domain/primitives';
import { type Infer, NonEmptyStringSchema, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const PolicyControlDeliveryCountSchema = Schema.Number.pipe(Schema.nonNegative(), Schema.int());
const NullablePolicyControlDeliveryTextSchema = Schema.Union(NonEmptyStringSchema, Schema.Null);

export const PolicyControlDeliveryReadModelSchemaVersion = 'policy-control-delivery-read-model.v1' as const;
export const PolicyControlDeliveryReadModelPayloadField = 'policyControlDeliveryReadModel' as const;
export const PolicyControlDeliveryReadModelReportedEventName = 'agent.policy-control.delivery.read-model.reported' as const;

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
export const AgentPolicyControlDeliveryDomainStateSchema = withParser(
  Schema.Struct({
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
  }).pipe(Schema.filter(validateDomainState))
);

export const PolicyControlDeliveryReadModelRowSchema = withParser(
  Schema.Struct({
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
  }).pipe(Schema.filter(validateRowState))
);

export const PolicyControlDeliveryReadModelSnapshotSchema = withParser(
  Schema.Struct({
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
  }).pipe(Schema.filter(validateSnapshotState))
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
export type PolicyControlDeliveryReadModelEventEnvelope = Infer<typeof PolicyControlDeliveryReadModelEventEnvelopeSchema>;

export type AgentPolicyControlDeliveryReadModelFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-payload';

export type AgentPolicyControlDeliveryReadModelResult =
  | {
      readonly ok: true;
      readonly value: PolicyControlDeliveryReadModelSnapshot;
    }
  | {
      readonly ok: false;
      readonly reason: AgentPolicyControlDeliveryReadModelFailureReason;
    };

export function parseAgentPolicyControlDeliveryReadModelEvent(event: {
  readonly event: string;
  readonly payload: Record<string, unknown>;
}): AgentPolicyControlDeliveryReadModelResult {
  if (event.event !== PolicyControlDeliveryReadModelReportedEventName) {
    return adapterFailure('wrong-event');
  }

  const raw = event.payload[PolicyControlDeliveryReadModelPayloadField];
  if (typeof raw !== 'string' || raw.trim().length === 0) {
    return adapterFailure('missing-json-field');
  }

  let decoded: unknown;
  try {
    decoded = JSON.parse(raw);
  } catch {
    return adapterFailure('invalid-json');
  }

  const parsed = PolicyControlDeliveryReadModelSnapshotSchema.safeParse(decoded);
  if (!parsed.success || parsed.data === undefined) {
    return adapterFailure('invalid-payload');
  }

  return {
    ok: true,
    value: parsed.data,
  };
}

function validateDomainState(state: PolicyControlDeliveryReadModelDomainState): true | string {
  if (state.deliveryState === 'acknowledged' && state.lastAckEventId === null) {
    return 'Acknowledged domain rows require an acknowledgement event id';
  }
  if (state.deliveryState === 'applied' && (state.lastAckEventId === null || state.lastAppliedEventId === null)) {
    return 'Applied domain rows require acknowledgement and applied event ids';
  }
  return true;
}

function validateRowState(row: PolicyControlDeliveryReadModelRow): true | string {
  if (row.acknowledgementRequired && row.ackState === 'not-required') {
    return 'Acknowledgement-required rows must not use not-required ack state';
  }
  if (row.parentVisibleState === 'acknowledged' && (row.ackState !== 'acknowledged' || row.applyState !== 'pending')) {
    return 'Acknowledged rows must distinguish acknowledgement from applied policy state';
  }
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
  if (
    row.parentVisibleState === 'manual-required' &&
    row.manualProofRequirements.length === 0
  ) {
    return 'Manual-required rows must surface explicit manual proof requirements';
  }
  if (row.applyState === 'partially-applied' && !hasMixedDomainOutcome(row.domainStates)) {
    return 'Partially applied rows require mixed per-domain outcomes';
  }
  return true;
}

function validateSnapshotState(snapshot: PolicyControlDeliveryReadModelSnapshot): true | string {
  if (snapshot.pendingCount !== countRows(snapshot.rows, 'pending')) {
    return 'Pending count must match pending rows';
  }
  if (snapshot.acknowledgedCount !== countRows(snapshot.rows, 'acknowledged')) {
    return 'Acknowledged count must match acknowledged rows';
  }
  if (snapshot.degradedCount !== countRows(snapshot.rows, 'degraded')) {
    return 'Degraded count must match degraded rows';
  }
  if (snapshot.manualRequiredCount !== countRows(snapshot.rows, 'manual-required')) {
    return 'Manual-required count must match manual-required rows';
  }
  if (snapshot.appliedCount !== countRows(snapshot.rows, 'applied')) {
    return 'Applied count must match applied rows';
  }
  if (snapshot.partiallyAppliedCount !== countRows(snapshot.rows, 'partially-applied')) {
    return 'Partially applied count must match partially applied rows';
  }
  if (snapshot.rejectedCount !== countRows(snapshot.rows, 'rejected')) {
    return 'Rejected count must match rejected rows';
  }
  if (snapshot.rolledBackCount !== countRows(snapshot.rows, 'rolled-back')) {
    return 'Rolled-back count must match rolled-back rows';
  }
  if (snapshot.supersededCount !== countRows(snapshot.rows, 'superseded')) {
    return 'Superseded count must match superseded rows';
  }
  if (snapshot.expiredBeforeDeliveryCount !== countRows(snapshot.rows, 'expired-before-delivery')) {
    return 'Expired-before-delivery count must match expired rows';
  }
  if (snapshot.parentVisibleState !== deriveSnapshotState(snapshot)) {
    return 'Snapshot parent-visible state must match row severity ordering';
  }
  if (snapshot.activationBlocked !== (snapshot.parentVisibleState !== 'applied')) {
    return 'Activation blocked must reflect whether every row is applied';
  }
  return true;
}

function hasMixedDomainOutcome(states: readonly PolicyControlDeliveryReadModelDomainState[]): boolean {
  const applied = states.some((state) => state.deliveryState === 'applied');
  const blocked = states.some((state) =>
    state.deliveryState === 'degraded' || state.deliveryState === 'manual-required' || state.deliveryState === 'blocked'
  );
  return applied && blocked;
}

function countRows(
  rows: readonly PolicyControlDeliveryReadModelRow[],
  state: PolicyControlDeliveryReadModelRow['parentVisibleState']
): number {
  return rows.filter((row) => row.parentVisibleState === state).length;
}

function deriveSnapshotState(snapshot: PolicyControlDeliveryReadModelSnapshot): PolicyControlDeliveryReadModelSnapshot['parentVisibleState'] {
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

function adapterFailure(reason: AgentPolicyControlDeliveryReadModelFailureReason): AgentPolicyControlDeliveryReadModelResult {
  return {
    ok: false,
    reason,
  };
}

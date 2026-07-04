import { AgentLogSnapshotSchema, LogFieldsSchema, LogLevelSchema } from './logging-contracts';
import {
  AgentCorrelationIdSchema,
  AgentEventIdSchema,
  AgentPeerSchema,
  AgentProtocolSchemaVersion,
  AgentTimestampSchema,
} from './event-primitives';
import { type Infer, NonEmptyStringSchema, Schema, withParser } from './effect';
import { validateDomainState, validateRowState } from './agent-policy-control-delivery-read-model-row-validators';
import {
  validateSnapshotActivationBlockedState,
  validateSnapshotParentVisibleState,
  validateSnapshotRowCounts,
} from './agent-policy-control-delivery-read-model-snapshot-validators';

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

function validateSnapshotState(snapshot: PolicyControlDeliveryReadModelSnapshotCandidate): true | string {
  const validators = [
    validateSnapshotRowCounts,
    validateSnapshotParentVisibleState,
    validateSnapshotActivationBlockedState,
  ] as const;

  for (const validator of validators) {
    const validation = validator(snapshot);
    if (validation !== true) {
      return validation;
    }
  }

  return true;
}

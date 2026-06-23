import { type Infer, Schema, brandedNonEmptyStringSchema, withParser } from './effect';
import {
  EventingAggregateKeySchema,
  EventingEnvelopeMetadataSchema,
  EventingIdempotencyKeySchema,
  type EventingCausationId,
  type EventingCorrelationId,
  type EventingEventId,
  type EventingRecordedAt,
} from './eventing';
import {
  ChildProfileIdSchema,
  FamilyIdSchema,
  ParentDeviceIdSchema,
  ParentPolicyVersionSchema,
} from './family-reference-primitives';
import { hasUniqueValues, literalSchema, literalValues, parsedLiteralRecord } from './policy-literal-contracts';
import {
  PolicyApprovalIdSchema,
  PolicyAuditReferenceIdSchema,
  PolicyOverrideIdSchema,
  type PolicyAuditReferenceId,
} from './policy-authority';
import { PolicyCompilerSourceDocumentIdSchema } from './policy-compiler';
import { PolicyReasonCodeSchema } from './policy-contracts';

export const PolicyEventSchemaVersionSchema = withParser(Schema.Literal(1));

export const PolicyEventFamilyNamespaceLiteral = {
  Policy: 'policy',
} as const;

export const PolicyEventFamilyNamespaceSchema = withParser(Schema.Literal(PolicyEventFamilyNamespaceLiteral.Policy));

export const PolicyEventKindLiteral = {
  DraftCreated: 'policy.draft.created',
  PreviewRequested: 'policy.preview.requested',
  PreviewGenerated: 'policy.preview.generated',
  Confirmed: 'policy.confirmed',
  VersionSuperseded: 'policy.version.superseded',
  CompilerRequested: 'policy.compiler.requested',
  CompilerCompleted: 'policy.compiler.completed',
  DeliveryQueued: 'policy.delivery.queued',
  DeliverySent: 'policy.delivery.sent',
  DeliveryAcknowledged: 'policy.delivery.acknowledged',
  DeliveryRejected: 'policy.delivery.rejected',
  DeliveryExpired: 'policy.delivery.expired',
  DeliveryRetryScheduled: 'policy.delivery.retry-scheduled',
  DomainApplied: 'policy.domain.applied',
  DomainPartial: 'policy.domain.partial',
  RollbackRequested: 'policy.rollback.requested',
  RollbackApplied: 'policy.rollback.applied',
  AskParentRequested: 'policy.ask-parent.requested',
  AskParentApproved: 'policy.ask-parent.approved',
  AskParentDenied: 'policy.ask-parent.denied',
  OverrideCreated: 'policy.override.created',
  OverrideExpired: 'policy.override.expired',
  AuditRecorded: 'policy.audit.recorded',
  DeadLetterRecorded: 'policy.dead-letter.recorded',
  ManualRequired: 'policy.manual-required',
} as const;

const PolicyEventKindValues = literalValues(PolicyEventKindLiteral);

export const PolicyEventKindSchema = literalSchema(PolicyEventKindLiteral);

export const PolicyConsumerDomainLiteral = {
  App: 'app',
  Browser: 'browser',
  Network: 'network',
  Tracking: 'tracking',
  Screen: 'screen',
  Ai: 'ai',
} as const;

export const PolicyConsumerDomainSchema = literalSchema(PolicyConsumerDomainLiteral);

export const PolicyEventDeadLetterReasonLiteral = {
  DuplicateIdempotency: 'duplicate-idempotency',
  ReplayRejected: 'replay-rejected',
  StaleSequence: 'stale-sequence',
  UnsupportedTarget: 'unsupported-target',
  MissingSubscriber: 'missing-subscriber',
  ManualRequired: 'manual-required',
} as const;

export const PolicyEventDeadLetterReasonSchema = literalSchema(PolicyEventDeadLetterReasonLiteral);

export const PolicyEventScopeLiteral = {
  SourceDocument: 'source-document',
  Request: 'request',
  Approval: 'approval',
  Override: 'override',
  Delivery: 'delivery',
  Rollback: 'rollback',
  Audit: 'audit',
} as const;

type PolicyEventScopeKind = (typeof PolicyEventScopeLiteral)[keyof typeof PolicyEventScopeLiteral];

export const PolicyEventRequestIdSchema = brandedNonEmptyStringSchema('PolicyEventRequestId');
export const PolicyEventDeliveryIdSchema = brandedNonEmptyStringSchema('PolicyEventDeliveryId');
const PolicyEventAggregateKeySchema = withParser(EventingAggregateKeySchema);
const PolicyEventIdempotencyKeySchema = withParser(EventingIdempotencyKeySchema);

export const PolicyEventSourceDocumentScopeSchema = withParser(
  Schema.Struct({
    scope: Schema.Literal(PolicyEventScopeLiteral.SourceDocument),
    householdId: FamilyIdSchema,
    sourceDocumentId: PolicyCompilerSourceDocumentIdSchema,
    policyVersion: ParentPolicyVersionSchema,
  })
);

export const PolicyEventRequestScopeSchema = withParser(
  Schema.Struct({
    scope: Schema.Literal(PolicyEventScopeLiteral.Request),
    householdId: FamilyIdSchema,
    requestId: PolicyEventRequestIdSchema,
    childProfileId: ChildProfileIdSchema,
    sourceDocumentId: PolicyCompilerSourceDocumentIdSchema,
    policyVersion: ParentPolicyVersionSchema,
  })
);

export const PolicyEventApprovalScopeSchema = withParser(
  Schema.Struct({
    scope: Schema.Literal(PolicyEventScopeLiteral.Approval),
    householdId: FamilyIdSchema,
    approvalId: PolicyApprovalIdSchema,
    requestId: PolicyEventRequestIdSchema,
    sourceDocumentId: PolicyCompilerSourceDocumentIdSchema,
    policyVersion: ParentPolicyVersionSchema,
  })
);

export const PolicyEventOverrideScopeSchema = withParser(
  Schema.Struct({
    scope: Schema.Literal(PolicyEventScopeLiteral.Override),
    householdId: FamilyIdSchema,
    overrideId: PolicyOverrideIdSchema,
    approvalId: PolicyApprovalIdSchema,
    requestId: PolicyEventRequestIdSchema,
    sourceDocumentId: PolicyCompilerSourceDocumentIdSchema,
    policyVersion: ParentPolicyVersionSchema,
  })
);

export const PolicyEventDeliveryScopeSchema = withParser(
  Schema.Struct({
    scope: Schema.Literal(PolicyEventScopeLiteral.Delivery),
    householdId: FamilyIdSchema,
    deliveryId: PolicyEventDeliveryIdSchema,
    childProfileId: ChildProfileIdSchema,
    deviceId: ParentDeviceIdSchema,
    domain: PolicyConsumerDomainSchema,
    sourceDocumentId: PolicyCompilerSourceDocumentIdSchema,
    policyVersion: ParentPolicyVersionSchema,
  })
);

export const PolicyEventRollbackRefSchema = withParser(
  Schema.Struct({
    householdId: FamilyIdSchema,
    rolledBackDocumentId: PolicyCompilerSourceDocumentIdSchema,
    rolledBackPolicyVersion: ParentPolicyVersionSchema,
    restoredDocumentId: PolicyCompilerSourceDocumentIdSchema,
    restoredPolicyVersion: ParentPolicyVersionSchema,
  })
);

export const PolicyEventRollbackScopeSchema = withParser(
  Schema.Struct({
    scope: Schema.Literal(PolicyEventScopeLiteral.Rollback),
    householdId: FamilyIdSchema,
    rollbackRef: PolicyEventRollbackRefSchema,
  })
);

export const PolicyEventAuditScopeSchema = withParser(
  Schema.Struct({
    scope: Schema.Literal(PolicyEventScopeLiteral.Audit),
    householdId: FamilyIdSchema,
    auditReferenceId: PolicyAuditReferenceIdSchema,
    sourceDocumentId: PolicyCompilerSourceDocumentIdSchema,
    policyVersion: ParentPolicyVersionSchema,
  })
);

export const PolicyEventScopeSchema = withParser(
  Schema.Union(
    PolicyEventSourceDocumentScopeSchema,
    PolicyEventRequestScopeSchema,
    PolicyEventApprovalScopeSchema,
    PolicyEventOverrideScopeSchema,
    PolicyEventDeliveryScopeSchema,
    PolicyEventRollbackScopeSchema,
    PolicyEventAuditScopeSchema
  )
);

const PolicyEventBaseSchema = Schema.Struct({
  schemaVersion: PolicyEventSchemaVersionSchema,
  kind: PolicyEventKindSchema,
  sequence: Schema.Number.pipe(Schema.int(), Schema.positive()),
  scope: PolicyEventScopeSchema,
  auditReferenceIds: Schema.Array(PolicyAuditReferenceIdSchema),
  reasonCode: Schema.Union(PolicyReasonCodeSchema, Schema.Null),
  deadLetterReason: Schema.Union(PolicyEventDeadLetterReasonSchema, Schema.Null),
});

export const PolicyEventSchema = withParser(
  PolicyEventBaseSchema.pipe(
    Schema.filter(
      (event) =>
        policyEventIsConsistent(event) ||
        'Expected policy event kind, scope, reason code, and dead-letter details to stay aligned'
    )
  )
);

export const PolicyEventContractSchema = withParser(
  Schema.Struct({
    eventType: PolicyEventKindSchema,
    schemaVersion: PolicyEventSchemaVersionSchema,
  })
);

const PolicyEventEnvelopeBaseSchema = Schema.Struct({
  contract: PolicyEventContractSchema,
  metadata: EventingEnvelopeMetadataSchema,
  payload: PolicyEventSchema,
});

export const PolicyEventEnvelopeSchema = withParser(
  PolicyEventEnvelopeBaseSchema.pipe(
    Schema.filter(
      (envelope) =>
        policyEventEnvelopeIsConsistent(envelope) ||
        'Expected policy event envelope to keep contract and deterministic keys aligned'
    )
  )
);

export type PolicyEventSchemaVersion = typeof PolicyEventSchemaVersionSchema.Type;
export type PolicyEventFamilyNamespace = typeof PolicyEventFamilyNamespaceSchema.Type;
export type PolicyEventKind = typeof PolicyEventKindSchema.Type;
export type PolicyConsumerDomain = Infer<typeof PolicyConsumerDomainSchema>;
export type PolicyEventDeadLetterReason = Infer<typeof PolicyEventDeadLetterReasonSchema>;
export type PolicyEventScope = Infer<typeof PolicyEventScopeSchema>;
export type PolicyEvent = Infer<typeof PolicyEventSchema>;
export type PolicyEventContract = Infer<typeof PolicyEventContractSchema>;
export type PolicyEventEnvelope = Infer<typeof PolicyEventEnvelopeSchema>;
export type PolicyEventRequestId = typeof PolicyEventRequestIdSchema.Type;
export type PolicyEventDeliveryId = typeof PolicyEventDeliveryIdSchema.Type;
export type PolicyEventAggregateKey = typeof EventingAggregateKeySchema.Type;
export type PolicyEventIdempotencyKey = typeof EventingIdempotencyKeySchema.Type;
export type PolicyEventCorrelationId = EventingCorrelationId;
export type PolicyEventCausationId = EventingCausationId;
export type PolicyEventEventId = EventingEventId;
export type PolicyEventRecordedAt = EventingRecordedAt;

export const PolicyEventFamilyNamespace = {
  Policy: PolicyEventFamilyNamespaceSchema.parse(PolicyEventFamilyNamespaceLiteral.Policy),
} as const;

export const PolicyEventKind = Object.freeze(
  Object.fromEntries(
    Object.entries(PolicyEventKindLiteral).map(([key, value]) => [key, PolicyEventKindSchema.parse(value)])
  )
) as Readonly<Record<keyof typeof PolicyEventKindLiteral, PolicyEventKind>>;

export const PolicyConsumerDomain = parsedLiteralRecord(PolicyConsumerDomainLiteral, (value) =>
  PolicyConsumerDomainSchema.parse(value)
);

export const PolicyEventDeadLetterReason = parsedLiteralRecord(PolicyEventDeadLetterReasonLiteral, (value) =>
  PolicyEventDeadLetterReasonSchema.parse(value)
);

export const PolicyEventScope = {
  SourceDocument: PolicyEventSourceDocumentScopeSchema.parse({
    scope: PolicyEventScopeLiteral.SourceDocument,
    householdId: 'household-default',
    sourceDocumentId: 'policy-source-default',
    policyVersion: '5',
  }),
  Request: PolicyEventRequestScopeSchema.parse({
    scope: PolicyEventScopeLiteral.Request,
    householdId: 'household-default',
    requestId: 'policy-request-default',
    childProfileId: 'child-primary',
    sourceDocumentId: 'policy-source-default',
    policyVersion: '5',
  }),
  Approval: PolicyEventApprovalScopeSchema.parse({
    scope: PolicyEventScopeLiteral.Approval,
    householdId: 'household-default',
    approvalId: 'policy-approval-default',
    requestId: 'policy-request-default',
    sourceDocumentId: 'policy-source-default',
    policyVersion: '5',
  }),
  Override: PolicyEventOverrideScopeSchema.parse({
    scope: PolicyEventScopeLiteral.Override,
    householdId: 'household-default',
    overrideId: 'policy-override-default',
    approvalId: 'policy-approval-default',
    requestId: 'policy-request-default',
    sourceDocumentId: 'policy-source-default',
    policyVersion: '5',
  }),
  Delivery: PolicyEventDeliveryScopeSchema.parse({
    scope: PolicyEventScopeLiteral.Delivery,
    householdId: 'household-default',
    deliveryId: 'policy-delivery-default',
    childProfileId: 'child-primary',
    deviceId: 'device-laptop',
    domain: PolicyConsumerDomainLiteral.Tracking,
    sourceDocumentId: 'policy-source-default',
    policyVersion: '5',
  }),
  Rollback: PolicyEventRollbackScopeSchema.parse({
    scope: PolicyEventScopeLiteral.Rollback,
    householdId: 'household-default',
    rollbackRef: {
      householdId: 'household-default',
      rolledBackDocumentId: 'policy-source-default',
      rolledBackPolicyVersion: '5',
      restoredDocumentId: 'policy-source-previous',
      restoredPolicyVersion: '4',
    },
  }),
  Audit: PolicyEventAuditScopeSchema.parse({
    scope: PolicyEventScopeLiteral.Audit,
    householdId: 'household-default',
    auditReferenceId: 'audit-policy-event',
    sourceDocumentId: 'policy-source-default',
    policyVersion: '5',
  }),
} as const;

export function policyEventSchemaVersion(): PolicyEventSchemaVersion {
  return PolicyEventSchemaVersionSchema.parse(1);
}

export function policyEventFamilyNamespaceValue(): PolicyEventFamilyNamespace {
  return PolicyEventFamilyNamespaceSchema.parse(PolicyEventFamilyNamespaceLiteral.Policy);
}

export function policyEventFamilyVariants(): readonly PolicyEventFamilyVariant[] {
  return POLICY_EVENT_KIND_VALUES.map((kind) => ({
    family: policyEventFamilyNamespaceValue(),
    eventType: kind,
  }));
}

export function policyEventFamilyRegistry(): readonly PolicyEventContract[] {
  return POLICY_EVENT_KIND_VALUES.map((kind) => policyEventContractForKind(kind));
}

export function policyEventContractForKind(kind: PolicyEventKind): PolicyEventContract {
  return PolicyEventContractSchema.parse({
    eventType: kind,
    schemaVersion: policyEventSchemaVersion(),
  });
}

export function policyEventContract(kind: PolicyEventKind): PolicyEventContract {
  return policyEventContractForKind(kind);
}

export function policyEventAggregateKey(event: Pick<PolicyEvent, 'scope'>): PolicyEventAggregateKey {
  return PolicyEventAggregateKeySchema.parse(policyEventAggregateKeyValue(event.scope));
}

export function policyEventIdempotencyKey(event: PolicyEvent): PolicyEventIdempotencyKey {
  return PolicyEventIdempotencyKeySchema.parse(policyEventIdempotencyKeyValue(event));
}

export function policyEventRedactedSummary(event: Pick<PolicyEvent, 'kind' | 'scope' | 'sequence'>): string {
  const value = [
    `policy-event kind=${event.kind}`,
    `scope=${policyEventScopeFamily(event.scope)}`,
    `sequence=${event.sequence}`,
  ];
  if (event.kind === PolicyEventKind.ManualRequired) {
    value.push('manual-required');
  }
  if (event.kind === PolicyEventKind.DeadLetterRecorded) {
    value.push('dead-lettered');
  }
  return value.join(' ');
}

export function parsePolicyEvent(input: unknown): PolicyEvent {
  return PolicyEventSchema.parse(input);
}

export function parsePolicyEventEnvelope(input: unknown): PolicyEventEnvelope {
  return PolicyEventEnvelopeSchema.parse(input);
}

export function applyPolicyEventReplay(current: PolicyEventReplayRecord, next: PolicyEvent): PolicyEventApplyOutcome {
  const nextAggregateKey = policyEventAggregateKey(next);
  const nextIdempotencyKey = policyEventIdempotencyKey(next);

  if (nextAggregateKey !== current.aggregateKey) {
    throw new Error(`policy event replay aggregate key mismatch: ${nextAggregateKey}`);
  }

  if (next.sequence < current.lastSequence) {
    return { state: 'stale', record: current };
  }

  if (next.sequence === current.lastSequence) {
    if (nextIdempotencyKey === current.lastIdempotencyKey && next.kind === current.lastEventType) {
      return { state: 'duplicate', record: current };
    }

    throw new Error(`conflicting replay for sequence ${next.sequence} on ${current.lastEventType}`);
  }

  return {
    state: 'advanced',
    record: {
      aggregateKey: nextAggregateKey,
      lastSequence: next.sequence,
      lastEventType: next.kind,
      lastIdempotencyKey: nextIdempotencyKey,
    },
  };
}

export type PolicyEventReplayRecord = {
  aggregateKey: PolicyEventAggregateKey;
  lastSequence: number;
  lastEventType: PolicyEventKind;
  lastIdempotencyKey: PolicyEventIdempotencyKey;
};

export type PolicyEventApplyOutcome =
  | { state: 'advanced'; record: PolicyEventReplayRecord }
  | { state: 'duplicate'; record: PolicyEventReplayRecord }
  | { state: 'stale'; record: PolicyEventReplayRecord };

export type PolicyEventFamilyVariant = {
  family: PolicyEventFamilyNamespace;
  eventType: PolicyEventKind;
};

const POLICY_EVENT_KIND_VALUES = Object.freeze(PolicyEventKindValues);

const POLICY_EVENT_SCOPE_FAMILY_BY_KIND: Readonly<Record<PolicyEventKind, PolicyEventScopeKind>> = {
  [PolicyEventKindLiteral.DraftCreated]: PolicyEventScopeLiteral.SourceDocument,
  [PolicyEventKindLiteral.PreviewRequested]: PolicyEventScopeLiteral.SourceDocument,
  [PolicyEventKindLiteral.PreviewGenerated]: PolicyEventScopeLiteral.SourceDocument,
  [PolicyEventKindLiteral.Confirmed]: PolicyEventScopeLiteral.SourceDocument,
  [PolicyEventKindLiteral.VersionSuperseded]: PolicyEventScopeLiteral.SourceDocument,
  [PolicyEventKindLiteral.CompilerRequested]: PolicyEventScopeLiteral.SourceDocument,
  [PolicyEventKindLiteral.CompilerCompleted]: PolicyEventScopeLiteral.SourceDocument,
  [PolicyEventKindLiteral.DeliveryQueued]: PolicyEventScopeLiteral.Delivery,
  [PolicyEventKindLiteral.DeliverySent]: PolicyEventScopeLiteral.Delivery,
  [PolicyEventKindLiteral.DeliveryAcknowledged]: PolicyEventScopeLiteral.Delivery,
  [PolicyEventKindLiteral.DeliveryRejected]: PolicyEventScopeLiteral.Delivery,
  [PolicyEventKindLiteral.DeliveryExpired]: PolicyEventScopeLiteral.Delivery,
  [PolicyEventKindLiteral.DeliveryRetryScheduled]: PolicyEventScopeLiteral.Delivery,
  [PolicyEventKindLiteral.DomainApplied]: PolicyEventScopeLiteral.Delivery,
  [PolicyEventKindLiteral.DomainPartial]: PolicyEventScopeLiteral.Delivery,
  [PolicyEventKindLiteral.RollbackRequested]: PolicyEventScopeLiteral.Rollback,
  [PolicyEventKindLiteral.RollbackApplied]: PolicyEventScopeLiteral.Rollback,
  [PolicyEventKindLiteral.AskParentRequested]: PolicyEventScopeLiteral.Request,
  [PolicyEventKindLiteral.AskParentApproved]: PolicyEventScopeLiteral.Request,
  [PolicyEventKindLiteral.AskParentDenied]: PolicyEventScopeLiteral.Request,
  [PolicyEventKindLiteral.OverrideCreated]: PolicyEventScopeLiteral.Override,
  [PolicyEventKindLiteral.OverrideExpired]: PolicyEventScopeLiteral.Override,
  [PolicyEventKindLiteral.AuditRecorded]: PolicyEventScopeLiteral.SourceDocument,
  [PolicyEventKindLiteral.DeadLetterRecorded]: PolicyEventScopeLiteral.SourceDocument,
  [PolicyEventKindLiteral.ManualRequired]: PolicyEventScopeLiteral.SourceDocument,
};

const POLICY_EVENT_REASON_CODE_BY_KIND: Readonly<Partial<Record<PolicyEventKind, string>>> = {
  [PolicyEventKindLiteral.DeliveryRejected]: 'delivery-rejected',
  [PolicyEventKindLiteral.DeliveryExpired]: 'delivery-expired',
  [PolicyEventKindLiteral.DeliveryRetryScheduled]: 'delivery-retry-scheduled',
  [PolicyEventKindLiteral.DomainPartial]: 'domain-partial',
  [PolicyEventKindLiteral.RollbackApplied]: 'rollback-applied',
  [PolicyEventKindLiteral.AskParentDenied]: 'ask-parent-denied',
  [PolicyEventKindLiteral.OverrideExpired]: 'override-expired',
  [PolicyEventKindLiteral.ManualRequired]: 'manual-required',
};

function policyEventIsConsistent(event: Infer<typeof PolicyEventBaseSchema>): boolean {
  if (policyEventScopeFamily(event.scope) !== policyEventExpectedScopeFamily(event.kind)) {
    return false;
  }

  if (!hasUniqueAuditReferenceIds(event.auditReferenceIds)) {
    return false;
  }

  if (policyEventKindRequiresReason(event.kind)) {
    if (event.reasonCode !== policyEventKindReasonCodeValue(event.kind)) {
      return false;
    }
  } else if (event.reasonCode !== null) {
    return false;
  }

  if ((event.kind === PolicyEventKind.DeadLetterRecorded) !== (event.deadLetterReason !== null)) {
    return false;
  }

  if (event.deadLetterReason !== null && event.kind !== PolicyEventKind.DeadLetterRecorded) {
    return false;
  }

  return true;
}

function policyEventEnvelopeIsConsistent(envelope: Infer<typeof PolicyEventEnvelopeBaseSchema>): boolean {
  return (
    envelope.contract.eventType === envelope.payload.kind &&
    envelope.contract.schemaVersion === envelope.payload.schemaVersion &&
    envelope.metadata.aggregateKey === policyEventAggregateKey(envelope.payload) &&
    envelope.metadata.idempotencyKey === policyEventIdempotencyKey(envelope.payload)
  );
}

function policyEventScopeFamily(scope: PolicyEventScope): PolicyEventScopeKind {
  return scope.scope;
}

function policyEventExpectedScopeFamily(kind: PolicyEventKind): PolicyEventScopeKind {
  return POLICY_EVENT_SCOPE_FAMILY_BY_KIND[kind];
}

function policyEventKindRequiresReason(kind: PolicyEventKind): boolean {
  return POLICY_EVENT_REASON_CODE_BY_KIND[kind] !== undefined;
}

function policyEventKindReasonCodeValue(kind: PolicyEventKind): string {
  return POLICY_EVENT_REASON_CODE_BY_KIND[kind] ?? 'policy-event';
}

function policyEventAggregateKeyValue(scope: PolicyEventScope): string {
  switch (scope.scope) {
    case PolicyEventScopeLiteral.SourceDocument:
      return joinPolicyEventKey(['policy-source', scope.householdId, scope.sourceDocumentId, scope.policyVersion]);
    case PolicyEventScopeLiteral.Request:
      return joinPolicyEventKey(['policy-request', scope.householdId, scope.requestId, scope.policyVersion]);
    case PolicyEventScopeLiteral.Approval:
      return joinPolicyEventKey([
        'policy-approval',
        scope.householdId,
        scope.approvalId,
        scope.requestId,
        scope.policyVersion,
      ]);
    case PolicyEventScopeLiteral.Override:
      return joinPolicyEventKey([
        'policy-override',
        scope.householdId,
        scope.overrideId,
        scope.approvalId,
        scope.requestId,
        scope.policyVersion,
      ]);
    case PolicyEventScopeLiteral.Delivery:
      return joinPolicyEventKey([
        'policy-delivery',
        scope.householdId,
        scope.deliveryId,
        scope.childProfileId,
        scope.deviceId,
        scope.domain,
        scope.policyVersion,
      ]);
    case PolicyEventScopeLiteral.Rollback:
      return joinPolicyEventKey([
        'policy-rollback',
        scope.householdId,
        scope.rollbackRef.rolledBackDocumentId,
        scope.rollbackRef.rolledBackPolicyVersion,
        scope.rollbackRef.restoredDocumentId,
        scope.rollbackRef.restoredPolicyVersion,
      ]);
    case PolicyEventScopeLiteral.Audit:
      return joinPolicyEventKey([
        'policy-audit',
        scope.householdId,
        scope.auditReferenceId,
        scope.sourceDocumentId,
        scope.policyVersion,
      ]);
  }
}

function policyEventIdempotencyKeyValue(event: PolicyEvent): string {
  return [
    'policy-event',
    event.kind,
    policyEventAggregateKeyValue(event.scope),
    String(event.sequence),
    policyEventScopeFamily(event.scope),
    joinAuditReferenceIds(event.auditReferenceIds),
    event.reasonCode ?? 'none',
    event.deadLetterReason ?? 'none',
  ].join('|');
}

function joinPolicyEventKey(parts: readonly string[]): string {
  return parts.join(':');
}

function joinAuditReferenceIds(auditReferenceIds: readonly PolicyAuditReferenceId[]): string {
  return auditReferenceIds.join(',');
}

function hasUniqueAuditReferenceIds(auditReferenceIds: readonly PolicyAuditReferenceId[]): boolean {
  return auditReferenceIds.length > 0 && hasUniqueValues(auditReferenceIds);
}

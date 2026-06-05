import { ActivityNetworkEvidenceGradeSchema } from '@ocentra-parent/activity-domain/network-flow';
import { type Infer, type SafeParseResult, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NetworkRuntimeText = Schema.String.pipe(Schema.minLength(1));
const NetworkRuntimeConfidence = Schema.Number.pipe(Schema.between(0, 1));
const NetworkRuntimeNonEmptyRefs = Schema.Array(NetworkRuntimeText).pipe(
  Schema.filter((refs) => refs.length > 0 || 'Expected network runtime event refs to be non-empty')
);
const NullableNetworkRuntimeText = Schema.Union(NetworkRuntimeText, Schema.Null);

export const AgentNetworkRuntimeEventSchemaVersion = 1;

export const AgentNetworkRuntimeEventType = {
  NetworkFlowObserved: 'network.flow.observed',
  NetworkDomainObserved: 'network.domain.observed',
  NetworkActivityClassified: 'network.activity.classified',
  AiAnalysisRequested: 'ai.analysis.requested',
  AiAnalysisCompleted: 'ai.analysis.completed',
  PolicyEvaluationRequested: 'policy.evaluation.requested',
  PolicyDecisionCompleted: 'policy.decision.completed',
  EnforcementCommandIssued: 'enforcement.command.issued',
  EnforcementResultObserved: 'enforcement.result.observed',
  AuditEntryCommitted: 'audit.entry.committed',
  PortalReadModelUpdated: 'portal.read_model.updated',
} as const;

export const AgentNetworkRuntimeEventTypeSchema = withParser(
  Schema.Literal(
    AgentNetworkRuntimeEventType.NetworkFlowObserved,
    AgentNetworkRuntimeEventType.NetworkDomainObserved,
    AgentNetworkRuntimeEventType.NetworkActivityClassified,
    AgentNetworkRuntimeEventType.AiAnalysisRequested,
    AgentNetworkRuntimeEventType.AiAnalysisCompleted,
    AgentNetworkRuntimeEventType.PolicyEvaluationRequested,
    AgentNetworkRuntimeEventType.PolicyDecisionCompleted,
    AgentNetworkRuntimeEventType.EnforcementCommandIssued,
    AgentNetworkRuntimeEventType.EnforcementResultObserved,
    AgentNetworkRuntimeEventType.AuditEntryCommitted,
    AgentNetworkRuntimeEventType.PortalReadModelUpdated
  )
);

export const AgentNetworkDomainAttributionKindSchema = withParser(
  Schema.Literal('dns-answer', 'sni-visible', 'http-host', 'reverse-lookup', 'ip-only', 'unavailable')
);

export const AgentNetworkRuntimeActivityKindSchema = withParser(
  Schema.Literal('social-candidate', 'video-candidate', 'game-candidate', 'vpn-proxy-tunnel-candidate', 'unknown')
);

export const AgentNetworkAiAdvisoryStateSchema = withParser(
  Schema.Literal('requested', 'completed', 'manual-review-required', 'provider-unavailable')
);

export const AgentNetworkPolicyDecisionActionSchema = withParser(
  Schema.Literal('observe', 'warn', 'ask-parent', 'limit', 'block', 'manual-review', 'unknown')
);

export const AgentNetworkEnforcementModeSchema = withParser(
  Schema.Literal('dry-run', 'manual-required', 'unavailable')
);

export const AgentNetworkEnforcementResultStatusSchema = withParser(
  Schema.Literal('dry-run', 'manual-required', 'unavailable', 'rejected')
);

export const AgentNetworkAuditOutcomeSchema = withParser(Schema.Literal('committed', 'failed'));

export const AgentNetworkPortalUpdateKindSchema = withParser(
  Schema.Literal('network-read-model', 'capability-state', 'manual-required-state')
);

export const AgentNetworkClaimBoundarySchema = withParser(
  Schema.Struct({
    exactUrlAvailable: Schema.Boolean,
    decryptedHttpsPayloadAvailable: Schema.Boolean,
    messageContentAvailable: Schema.Boolean,
    searchQueryAvailable: Schema.Boolean,
    adapterActionExecuted: Schema.Boolean,
  }).pipe(
    Schema.filter(
      (boundary) =>
        (!boundary.exactUrlAvailable &&
          !boundary.decryptedHttpsPayloadAvailable &&
          !boundary.messageContentAvailable &&
          !boundary.searchQueryAvailable &&
          !boundary.adapterActionExecuted) ||
        'Network runtime events cannot claim exact URL, decrypted payload, message content, search query, or adapter action'
    )
  )
);

const NetworkRuntimeEventBase = {
  schemaVersion: Schema.Literal(AgentNetworkRuntimeEventSchemaVersion),
};

export const AgentNetworkFlowObservedEventSchema = withParser(
  Schema.Struct({
    ...NetworkRuntimeEventBase,
    flowEventRef: NetworkRuntimeText,
    observedAt: NetworkRuntimeText,
    deviceRef: NetworkRuntimeText,
    flowEvidenceRef: NetworkRuntimeText,
    custody: NetworkRuntimeText,
    evidenceGrade: ActivityNetworkEvidenceGradeSchema,
    claimBoundary: AgentNetworkClaimBoundarySchema,
  })
);

export const AgentNetworkDomainObservedEventSchema = withParser(
  Schema.Struct({
    ...NetworkRuntimeEventBase,
    domainEventRef: NetworkRuntimeText,
    previousEventRef: NetworkRuntimeText,
    flowEvidenceRef: NetworkRuntimeText,
    domainEvidenceRef: NetworkRuntimeText,
    attribution: AgentNetworkDomainAttributionKindSchema,
    evidenceGrade: ActivityNetworkEvidenceGradeSchema,
    uncertaintyCodes: Schema.Array(NetworkRuntimeText),
    claimBoundary: AgentNetworkClaimBoundarySchema,
  })
);

export const AgentNetworkActivityClassifiedEventSchema = withParser(
  Schema.Struct({
    ...NetworkRuntimeEventBase,
    classificationEventRef: NetworkRuntimeText,
    previousEventRef: NetworkRuntimeText,
    evidenceRefs: NetworkRuntimeNonEmptyRefs,
    activityKind: AgentNetworkRuntimeActivityKindSchema,
    confidence: NetworkRuntimeConfidence,
    evidenceGrade: ActivityNetworkEvidenceGradeSchema,
    uncertaintyCodes: Schema.Array(NetworkRuntimeText),
  })
);

export const AgentNetworkAiAnalysisRequestedEventSchema = withParser(
  Schema.Struct({
    ...NetworkRuntimeEventBase,
    aiRequestRef: NetworkRuntimeText,
    previousEventRef: NetworkRuntimeText,
    evidenceRefs: NetworkRuntimeNonEmptyRefs,
    promptTemplateRef: NetworkRuntimeText,
    custody: NetworkRuntimeText,
    rawPacketPayloadIncluded: Schema.Boolean,
  }).pipe(
    Schema.filter((event) => !event.rawPacketPayloadIncluded || 'Network AI events cannot include raw packet payloads')
  )
);

export const AgentNetworkAiAnalysisCompletedEventSchema = withParser(
  Schema.Struct({
    ...NetworkRuntimeEventBase,
    aiAnalysisRef: NetworkRuntimeText,
    aiRequestRef: NetworkRuntimeText,
    previousEventRef: NetworkRuntimeText,
    advisoryState: AgentNetworkAiAdvisoryStateSchema,
    evidenceRefs: NetworkRuntimeNonEmptyRefs,
    unsupportedClaims: Schema.Array(NetworkRuntimeText),
  })
);

export const AgentNetworkPolicyEvaluationRequestedEventSchema = withParser(
  Schema.Struct({
    ...NetworkRuntimeEventBase,
    policyEvaluationRef: NetworkRuntimeText,
    previousEventRef: NetworkRuntimeText,
    evidenceRefs: NetworkRuntimeNonEmptyRefs,
    aiAnalysisRef: NullableNetworkRuntimeText,
    parentRuleRefs: NetworkRuntimeNonEmptyRefs,
    dryRun: Schema.Boolean,
  })
);

export const AgentNetworkPolicyDecisionCompletedEventSchema = withParser(
  Schema.Struct({
    ...NetworkRuntimeEventBase,
    policyDecisionRef: NetworkRuntimeText,
    policyEvaluationRef: NetworkRuntimeText,
    previousEventRef: NetworkRuntimeText,
    decisionAction: AgentNetworkPolicyDecisionActionSchema,
    evidenceRefs: NetworkRuntimeNonEmptyRefs,
    parentRuleRefs: NetworkRuntimeNonEmptyRefs,
    adapterCapabilityRequired: Schema.Boolean,
  })
);

export const AgentNetworkEnforcementCommandIssuedEventSchema = withParser(
  Schema.Struct({
    ...NetworkRuntimeEventBase,
    enforcementCommandRef: NetworkRuntimeText,
    previousEventRef: NetworkRuntimeText,
    policyDecisionRef: NetworkRuntimeText,
    adapterCapabilityRef: NetworkRuntimeText,
    enforcementMode: AgentNetworkEnforcementModeSchema,
    evidenceRefs: NetworkRuntimeNonEmptyRefs,
    rollbackRef: NullableNetworkRuntimeText,
  })
);

export const AgentNetworkEnforcementResultObservedEventSchema = withParser(
  Schema.Struct({
    ...NetworkRuntimeEventBase,
    enforcementResultRef: NetworkRuntimeText,
    enforcementCommandRef: NetworkRuntimeText,
    previousEventRef: NetworkRuntimeText,
    resultStatus: AgentNetworkEnforcementResultStatusSchema,
    adapterActionExecuted: Schema.Boolean,
    rollbackRef: NullableNetworkRuntimeText,
    unavailableReasonCode: NullableNetworkRuntimeText,
  }).pipe(
    Schema.filter((event) => !event.adapterActionExecuted || 'Network enforcement result cannot claim adapter action')
  )
);

export const AgentNetworkAuditEntryCommittedEventSchema = withParser(
  Schema.Struct({
    ...NetworkRuntimeEventBase,
    auditEntryRef: NetworkRuntimeText,
    previousEventRef: NetworkRuntimeText,
    policyDecisionRef: NetworkRuntimeText,
    enforcementCommandRef: NullableNetworkRuntimeText,
    enforcementResultRef: NullableNetworkRuntimeText,
    evidenceRefs: NetworkRuntimeNonEmptyRefs,
    auditOutcome: AgentNetworkAuditOutcomeSchema,
  })
);

export const AgentNetworkPortalReadModelUpdatedEventSchema = withParser(
  Schema.Struct({
    ...NetworkRuntimeEventBase,
    readModelRef: NetworkRuntimeText,
    previousEventRef: NetworkRuntimeText,
    auditEntryRef: NetworkRuntimeText,
    updateKind: AgentNetworkPortalUpdateKindSchema,
    visibleManualRequired: Schema.Boolean,
    visibleUnavailable: Schema.Boolean,
  })
);

export type AgentNetworkRuntimeEventType = Infer<typeof AgentNetworkRuntimeEventTypeSchema>;
export type AgentNetworkClaimBoundary = Infer<typeof AgentNetworkClaimBoundarySchema>;
export type AgentNetworkFlowObservedEvent = Infer<typeof AgentNetworkFlowObservedEventSchema>;
export type AgentNetworkDomainObservedEvent = Infer<typeof AgentNetworkDomainObservedEventSchema>;
export type AgentNetworkActivityClassifiedEvent = Infer<typeof AgentNetworkActivityClassifiedEventSchema>;
export type AgentNetworkAiAnalysisRequestedEvent = Infer<typeof AgentNetworkAiAnalysisRequestedEventSchema>;
export type AgentNetworkAiAnalysisCompletedEvent = Infer<typeof AgentNetworkAiAnalysisCompletedEventSchema>;
export type AgentNetworkPolicyEvaluationRequestedEvent = Infer<typeof AgentNetworkPolicyEvaluationRequestedEventSchema>;
export type AgentNetworkPolicyDecisionCompletedEvent = Infer<typeof AgentNetworkPolicyDecisionCompletedEventSchema>;
export type AgentNetworkEnforcementCommandIssuedEvent = Infer<typeof AgentNetworkEnforcementCommandIssuedEventSchema>;
export type AgentNetworkEnforcementResultObservedEvent = Infer<typeof AgentNetworkEnforcementResultObservedEventSchema>;
export type AgentNetworkAuditEntryCommittedEvent = Infer<typeof AgentNetworkAuditEntryCommittedEventSchema>;
export type AgentNetworkPortalReadModelUpdatedEvent = Infer<typeof AgentNetworkPortalReadModelUpdatedEventSchema>;

export type AgentNetworkRuntimeEventPayload =
  | AgentNetworkFlowObservedEvent
  | AgentNetworkDomainObservedEvent
  | AgentNetworkActivityClassifiedEvent
  | AgentNetworkAiAnalysisRequestedEvent
  | AgentNetworkAiAnalysisCompletedEvent
  | AgentNetworkPolicyEvaluationRequestedEvent
  | AgentNetworkPolicyDecisionCompletedEvent
  | AgentNetworkEnforcementCommandIssuedEvent
  | AgentNetworkEnforcementResultObservedEvent
  | AgentNetworkAuditEntryCommittedEvent
  | AgentNetworkPortalReadModelUpdatedEvent;

export type AgentNetworkRuntimeEventFailureReason = 'invalid-event-type' | 'invalid-payload';

export type AgentNetworkRuntimeEventResult =
  | {
      readonly ok: true;
      readonly eventType: AgentNetworkRuntimeEventType;
      readonly value: AgentNetworkRuntimeEventPayload;
    }
  | {
      readonly ok: false;
      readonly reason: AgentNetworkRuntimeEventFailureReason;
    };

export function parseAgentNetworkRuntimeEvent(input: unknown): AgentNetworkRuntimeEventResult {
  if (!isNetworkRuntimeEventInput(input)) {
    return parserFailure('invalid-event-type');
  }

  const eventType = AgentNetworkRuntimeEventTypeSchema.safeParse(input.eventType);
  if (!eventType.success) {
    return parserFailure('invalid-event-type');
  }

  const payload = parseNetworkRuntimePayload(eventType.data, input.payload);
  if (!payload.ok) {
    return payload;
  }

  return {
    ok: true,
    eventType: eventType.data,
    value: payload.value,
  };
}

function parseNetworkRuntimePayload(
  eventType: AgentNetworkRuntimeEventType,
  payload: unknown
): AgentNetworkRuntimeEventResult {
  switch (eventType) {
    case AgentNetworkRuntimeEventType.NetworkFlowObserved:
      return payloadResult(AgentNetworkFlowObservedEventSchema.safeParse(payload));
    case AgentNetworkRuntimeEventType.NetworkDomainObserved:
      return payloadResult(AgentNetworkDomainObservedEventSchema.safeParse(payload));
    case AgentNetworkRuntimeEventType.NetworkActivityClassified:
      return payloadResult(AgentNetworkActivityClassifiedEventSchema.safeParse(payload));
    case AgentNetworkRuntimeEventType.AiAnalysisRequested:
      return payloadResult(AgentNetworkAiAnalysisRequestedEventSchema.safeParse(payload));
    case AgentNetworkRuntimeEventType.AiAnalysisCompleted:
      return payloadResult(AgentNetworkAiAnalysisCompletedEventSchema.safeParse(payload));
    case AgentNetworkRuntimeEventType.PolicyEvaluationRequested:
      return payloadResult(AgentNetworkPolicyEvaluationRequestedEventSchema.safeParse(payload));
    case AgentNetworkRuntimeEventType.PolicyDecisionCompleted:
      return payloadResult(AgentNetworkPolicyDecisionCompletedEventSchema.safeParse(payload));
    case AgentNetworkRuntimeEventType.EnforcementCommandIssued:
      return payloadResult(AgentNetworkEnforcementCommandIssuedEventSchema.safeParse(payload));
    case AgentNetworkRuntimeEventType.EnforcementResultObserved:
      return payloadResult(AgentNetworkEnforcementResultObservedEventSchema.safeParse(payload));
    case AgentNetworkRuntimeEventType.AuditEntryCommitted:
      return payloadResult(AgentNetworkAuditEntryCommittedEventSchema.safeParse(payload));
    case AgentNetworkRuntimeEventType.PortalReadModelUpdated:
      return payloadResult(AgentNetworkPortalReadModelUpdatedEventSchema.safeParse(payload));
  }
}

function payloadResult(parsed: SafeParseResult<AgentNetworkRuntimeEventPayload>): AgentNetworkRuntimeEventResult {
  if (!parsed.success) {
    return parserFailure('invalid-payload');
  }
  return {
    ok: true,
    eventType: AgentNetworkRuntimeEventType.NetworkFlowObserved,
    value: parsed.data,
  };
}

function parserFailure(reason: AgentNetworkRuntimeEventFailureReason): AgentNetworkRuntimeEventResult {
  return {
    ok: false,
    reason,
  };
}

function isNetworkRuntimeEventInput(input: unknown): input is { eventType: unknown; payload: unknown } {
  return typeof input === 'object' && input !== null && 'eventType' in input && 'payload' in input;
}

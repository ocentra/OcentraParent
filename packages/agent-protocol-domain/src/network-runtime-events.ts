import {
  AgentNetworkActivityClassifiedEventSchema,
  AgentNetworkAiAnalysisCompletedEventSchema,
  AgentNetworkAiAnalysisRequestedEventSchema,
  AgentNetworkAuditEntryCommittedEventSchema,
  AgentNetworkDomainObservedEventSchema,
  AgentNetworkEnforcementCommandIssuedEventSchema,
  AgentNetworkEnforcementResultObservedEventSchema,
  AgentNetworkFlowObservedEventSchema,
  AgentNetworkPolicyDecisionCompletedEventSchema,
  AgentNetworkPolicyEvaluationRequestedEventSchema,
  AgentNetworkPortalReadModelUpdatedEventSchema,
  AgentNetworkRuntimeEventType,
  AgentNetworkRuntimeEventTypeSchema,
  type AgentNetworkRuntimeEventPayload,
  type AgentNetworkRuntimeEventType as AgentNetworkRuntimeEventTypeValue,
} from '@ocentra-parent/schema-domain/network-runtime-events';
import { type SafeParseResult } from '@ocentra-parent/schema-domain/effect';

export type AgentNetworkRuntimeEventFailureReason = 'invalid-event-type' | 'invalid-payload';

export type AgentNetworkRuntimeEventResult =
  | {
      readonly ok: true;
      readonly eventType: AgentNetworkRuntimeEventTypeValue;
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
  eventType: AgentNetworkRuntimeEventTypeValue,
  payload: unknown
): AgentNetworkRuntimeEventResult {
  switch (eventType) {
    case AgentNetworkRuntimeEventType.NetworkFlowObserved:
      return payloadResult(eventType, AgentNetworkFlowObservedEventSchema.safeParse(payload));
    case AgentNetworkRuntimeEventType.NetworkDomainObserved:
      return payloadResult(eventType, AgentNetworkDomainObservedEventSchema.safeParse(payload));
    case AgentNetworkRuntimeEventType.NetworkActivityClassified:
      return payloadResult(eventType, AgentNetworkActivityClassifiedEventSchema.safeParse(payload));
    case AgentNetworkRuntimeEventType.AiAnalysisRequested:
      return payloadResult(eventType, AgentNetworkAiAnalysisRequestedEventSchema.safeParse(payload));
    case AgentNetworkRuntimeEventType.AiAnalysisCompleted:
      return payloadResult(eventType, AgentNetworkAiAnalysisCompletedEventSchema.safeParse(payload));
    case AgentNetworkRuntimeEventType.PolicyEvaluationRequested:
      return payloadResult(eventType, AgentNetworkPolicyEvaluationRequestedEventSchema.safeParse(payload));
    case AgentNetworkRuntimeEventType.PolicyDecisionCompleted:
      return payloadResult(eventType, AgentNetworkPolicyDecisionCompletedEventSchema.safeParse(payload));
    case AgentNetworkRuntimeEventType.EnforcementCommandIssued:
      return payloadResult(eventType, AgentNetworkEnforcementCommandIssuedEventSchema.safeParse(payload));
    case AgentNetworkRuntimeEventType.EnforcementResultObserved:
      return payloadResult(eventType, AgentNetworkEnforcementResultObservedEventSchema.safeParse(payload));
    case AgentNetworkRuntimeEventType.AuditEntryCommitted:
      return payloadResult(eventType, AgentNetworkAuditEntryCommittedEventSchema.safeParse(payload));
    case AgentNetworkRuntimeEventType.PortalReadModelUpdated:
      return payloadResult(eventType, AgentNetworkPortalReadModelUpdatedEventSchema.safeParse(payload));
  }
}

function payloadResult(
  eventType: AgentNetworkRuntimeEventTypeValue,
  parsed: SafeParseResult<AgentNetworkRuntimeEventPayload>
): AgentNetworkRuntimeEventResult {
  if (!parsed.success) {
    return parserFailure('invalid-payload');
  }
  return {
    ok: true,
    eventType,
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

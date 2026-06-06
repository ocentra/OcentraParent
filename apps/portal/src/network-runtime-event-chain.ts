import { AgentProtocolDefaults, type AgentProtocolLogFields } from '@ocentra-parent/agent-protocol-domain/contracts';
import {
  AgentNetworkRuntimeEventType,
  parseAgentNetworkRuntimeEvent,
  type AgentNetworkActivityClassifiedEvent,
  type AgentNetworkAiAnalysisCompletedEvent,
  type AgentNetworkAuditEntryCommittedEvent,
  type AgentNetworkDomainObservedEvent,
  type AgentNetworkEnforcementResultObservedEvent,
  type AgentNetworkFlowObservedEvent,
  type AgentNetworkPolicyDecisionCompletedEvent,
  type AgentNetworkPortalReadModelUpdatedEvent,
  type AgentNetworkRuntimeEventPayload,
  type AgentNetworkRuntimeEventType as AgentNetworkRuntimeEventTypeValue,
} from '@ocentra-parent/agent-protocol-domain/network-runtime-events';
import {
  PortalFormatting,
  PortalText,
  PortalTextToken,
  decodePortalDetailValue,
  type PortalDetailValue,
} from '@ocentra-parent/portal-domain/contracts';

export type NetworkRuntimeEventChainSummary = {
  readonly eventHistoryRef: PortalDetailValue;
  readonly aiAuditRef: PortalDetailValue;
  readonly auditRef: PortalDetailValue;
  readonly policyDecisionRef: PortalDetailValue;
  readonly interventionResultRef: PortalDetailValue;
  readonly retentionState: PortalDetailValue;
  readonly evidenceGrade: PortalDetailValue;
  readonly confidence: PortalDetailValue;
  readonly manualRequiredState: PortalDetailValue;
  readonly unavailableState: PortalDetailValue;
};

type RuntimeChainEntry = {
  readonly eventType: unknown;
  readonly eventRef: unknown;
  readonly payload: unknown;
};
type RuntimeChainRecord = Record<PropertyKey, unknown>;

type ParsedRuntimeChainEvent = {
  readonly eventType: AgentNetworkRuntimeEventTypeValue;
  readonly eventRef: PortalDetailValue | null;
  readonly value: AgentNetworkRuntimeEventPayload;
};

type RuntimeChainAccumulator = {
  eventRefs: PortalDetailValue[];
  aiAuditRefs: PortalDetailValue[];
  auditRefs: PortalDetailValue[];
  policyDecisionRefs: PortalDetailValue[];
  interventionResultRefs: PortalDetailValue[];
  evidenceGrades: PortalDetailValue[];
  confidenceValues: PortalDetailValue[];
  manualRequiredStates: PortalDetailValue[];
  unavailableStates: PortalDetailValue[];
};

type RuntimeChainEventHandler = (accumulated: RuntimeChainAccumulator, event: ParsedRuntimeChainEvent) => void;

const RuntimeChainEventHandlers = new Map<AgentNetworkRuntimeEventTypeValue, RuntimeChainEventHandler>([
  [AgentNetworkRuntimeEventType.NetworkFlowObserved, recordNetworkFlowObserved],
  [AgentNetworkRuntimeEventType.NetworkDomainObserved, recordNetworkDomainObserved],
  [AgentNetworkRuntimeEventType.NetworkActivityClassified, recordNetworkActivityClassified],
  [AgentNetworkRuntimeEventType.AiAnalysisCompleted, recordAiAnalysisCompleted],
  [AgentNetworkRuntimeEventType.PolicyDecisionCompleted, recordPolicyDecisionCompleted],
  [AgentNetworkRuntimeEventType.EnforcementResultObserved, recordEnforcementResultObserved],
  [AgentNetworkRuntimeEventType.AuditEntryCommitted, recordAuditEntryCommitted],
  [AgentNetworkRuntimeEventType.PortalReadModelUpdated, recordPortalReadModelUpdated],
]);

export function parseNetworkRuntimeEventChain(payload: AgentProtocolLogFields): NetworkRuntimeEventChainSummary | null {
  const streamEntries = parseStreamEntries(payload[AgentProtocolDefaults.Field.NetworkRuntimeEventChainStream]);
  const parsedEvents = streamEntries.map(parseRuntimeChainEvent).filter(isParsedRuntimeChainEvent);
  const counters = runtimeChainCounters(payload);

  if (parsedEvents.length === 0 && counters.length === 0) {
    return null;
  }

  const accumulated = parsedEvents.reduce(accumulateRuntimeChainEvent, emptyAccumulator());
  return {
    eventHistoryRef: joinedDetail(accumulated.eventRefs),
    aiAuditRef: joinedDetail(accumulated.aiAuditRefs),
    auditRef: joinedDetail(accumulated.auditRefs),
    policyDecisionRef: joinedDetail(accumulated.policyDecisionRefs),
    interventionResultRef: joinedDetail(accumulated.interventionResultRefs),
    retentionState: joinedDetail(counters),
    evidenceGrade: joinedDetail(accumulated.evidenceGrades),
    confidence: joinedDetail(accumulated.confidenceValues),
    manualRequiredState: joinedDetail(accumulated.manualRequiredStates),
    unavailableState: joinedDetail(accumulated.unavailableStates),
  };
}

function parseStreamEntries(value: unknown): RuntimeChainEntry[] {
  if (typeof value !== AgentProtocolDefaults.Primitive.String) {
    return [];
  }

  try {
    const decoded = JSON.parse(String(value)) as unknown;
    if (!Array.isArray(decoded)) {
      return [];
    }
    return decoded.filter(isRuntimeChainEntry);
  } catch {
    return [];
  }
}

function isRuntimeChainEntry(value: unknown): value is RuntimeChainEntry {
  return isRuntimeChainRecord(value) && AgentProtocolDefaults.Field.EventType in value;
}

function isRuntimeChainRecord(value: unknown): value is RuntimeChainRecord {
  return typeof value === AgentProtocolDefaults.Primitive.Object && value !== null;
}

function parseRuntimeChainEvent(entry: RuntimeChainEntry): ParsedRuntimeChainEvent | null {
  const parsed = parseAgentNetworkRuntimeEvent(entry);
  if (!parsed.ok) {
    return null;
  }

  return {
    eventType: parsed.eventType,
    eventRef: typeof entry.eventRef === AgentProtocolDefaults.Primitive.String ? detailValue(entry.eventRef) : null,
    value: parsed.value,
  };
}

function isParsedRuntimeChainEvent(value: ParsedRuntimeChainEvent | null): value is ParsedRuntimeChainEvent {
  return value !== null;
}

function emptyAccumulator(): RuntimeChainAccumulator {
  return {
    eventRefs: [],
    aiAuditRefs: [],
    auditRefs: [],
    policyDecisionRefs: [],
    interventionResultRefs: [],
    evidenceGrades: [],
    confidenceValues: [],
    manualRequiredStates: [],
    unavailableStates: [],
  };
}

function accumulateRuntimeChainEvent(
  accumulated: RuntimeChainAccumulator,
  event: ParsedRuntimeChainEvent
): RuntimeChainAccumulator {
  if (event.eventRef !== null) {
    accumulated.eventRefs.push(event.eventRef);
  }

  RuntimeChainEventHandlers.get(event.eventType)?.(accumulated, event);

  return accumulated;
}

function recordNetworkFlowObserved(accumulated: RuntimeChainAccumulator, event: ParsedRuntimeChainEvent): void {
  const value = event.value as AgentNetworkFlowObservedEvent;
  accumulated.evidenceGrades.push(detailValue(value.evidenceGrade));
}

function recordNetworkDomainObserved(accumulated: RuntimeChainAccumulator, event: ParsedRuntimeChainEvent): void {
  const value = event.value as AgentNetworkDomainObservedEvent;
  accumulated.evidenceGrades.push(detailValue(value.evidenceGrade));
}

function recordNetworkActivityClassified(accumulated: RuntimeChainAccumulator, event: ParsedRuntimeChainEvent): void {
  const value = event.value as AgentNetworkActivityClassifiedEvent;
  accumulated.evidenceGrades.push(detailValue(value.evidenceGrade));
  accumulated.confidenceValues.push(detailValue(value.confidence));
}

function recordAiAnalysisCompleted(accumulated: RuntimeChainAccumulator, event: ParsedRuntimeChainEvent): void {
  const value = event.value as AgentNetworkAiAnalysisCompletedEvent;
  accumulated.aiAuditRefs.push(joinRuntimeValues([value.aiAnalysisRef, value.advisoryState]));
}

function recordPolicyDecisionCompleted(accumulated: RuntimeChainAccumulator, event: ParsedRuntimeChainEvent): void {
  const value = event.value as AgentNetworkPolicyDecisionCompletedEvent;
  accumulated.policyDecisionRefs.push(joinRuntimeValues([value.policyDecisionRef, value.decisionAction]));
}

function recordEnforcementResultObserved(accumulated: RuntimeChainAccumulator, event: ParsedRuntimeChainEvent): void {
  const value = event.value as AgentNetworkEnforcementResultObservedEvent;
  accumulated.interventionResultRefs.push(
    joinRuntimeValues([value.enforcementResultRef, value.resultStatus, value.unavailableReasonCode])
  );
  if (value.unavailableReasonCode !== null) {
    accumulated.unavailableStates.push(detailValue(value.unavailableReasonCode));
  }
}

function recordAuditEntryCommitted(accumulated: RuntimeChainAccumulator, event: ParsedRuntimeChainEvent): void {
  const value = event.value as AgentNetworkAuditEntryCommittedEvent;
  accumulated.auditRefs.push(joinRuntimeValues([value.auditEntryRef, value.auditOutcome]));
}

function recordPortalReadModelUpdated(accumulated: RuntimeChainAccumulator, event: ParsedRuntimeChainEvent): void {
  const value = event.value as AgentNetworkPortalReadModelUpdatedEvent;
  if (value.visibleManualRequired) {
    accumulated.manualRequiredStates.push(detailValue(value.updateKind));
  }
  if (value.visibleUnavailable) {
    accumulated.unavailableStates.push(detailValue(value.updateKind));
  }
}

function runtimeChainCounters(payload: AgentProtocolLogFields): PortalDetailValue[] {
  return [
    payload[AgentProtocolDefaults.Field.ActiveRows],
    payload[AgentProtocolDefaults.Field.TombstoneRows],
    payload[AgentProtocolDefaults.Field.ExportableRows],
    payload[AgentProtocolDefaults.Field.DeletedEvidenceReferenceIds],
  ]
    .filter(isReportedValue)
    .map(detailValue);
}

function joinRuntimeValues(values: readonly unknown[]): PortalDetailValue {
  return joinedDetail(values);
}

function joinedDetail(values: readonly unknown[]): PortalDetailValue {
  const normalized = values.filter(isReportedValue).map((value) => String(value));
  if (normalized.length === 0) {
    return notReported();
  }
  return decodePortalDetailValue(normalized.join(PortalFormatting.EventDetailSeparator));
}

function detailValue(value: unknown): PortalDetailValue {
  return decodePortalDetailValue(String(value));
}

function isReportedValue(value: unknown): boolean {
  return value !== undefined && value !== null && String(value).length > 0;
}

function notReported(): PortalDetailValue {
  return decodePortalDetailValue(PortalText.Resolve(PortalTextToken.NotReported));
}

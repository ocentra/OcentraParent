import type { PortalActivityMemoryGraphReadModel } from './activity-memory-graph';
import {
  PortalAgentEvent as AgentEvent,
  PortalAgentProtocolField,
  type PortalRouteEventRecord,
} from './portal-contract-adapter';
import { decodePortalDetailValue, type PortalDetailValue } from './portal-contract-text-contracts';
import { decodeDisplayText, type DisplayText } from './display-text';
import { PortalDetails } from './details';
import {
  localAiRuntimePanelTemplate,
  type LocalAiHouseholdJobInput,
  type LocalAiMemoryGraphInput,
  type LocalAiRemoteAssistantBoundaryInput,
  type LocalAiRuntimePanelCardKind,
  type LocalAiRuntimePanelFieldKey,
  type LocalAiRuntimeStatusInput,
} from './local-ai-runtime-panel.generated';

export type LocalAiRuntimePanelDetail = {
  readonly label: DisplayText;
  readonly value: PortalDetailValue;
};

export type LocalAiRuntimePanelCard = {
  readonly title: DisplayText;
  readonly details: readonly LocalAiRuntimePanelDetail[];
};

export type LocalAiRuntimePanelIntent = {
  readonly eyebrow: DisplayText;
  readonly title: DisplayText;
  readonly body: DisplayText;
  readonly emptyMessage: DisplayText;
  readonly emptyStatus: PortalDetailValue;
  readonly productClaim: PortalDetailValue;
  readonly summaryDetails: readonly LocalAiRuntimePanelDetail[];
  readonly cards: readonly LocalAiRuntimePanelCard[];
};

const LocalAiRuntimePanelCardTitles: Readonly<Record<LocalAiRuntimePanelCardKind, DisplayText>> = {
  'runtime-status': decodeDisplayText('Local AI runtime status'),
  'household-job': decodeDisplayText('Household AI job activity'),
  'memory-graph': decodeDisplayText('Cited memory and graph evidence'),
  'remote-assistant-boundary': decodeDisplayText('Remote assistant boundary'),
};

const LocalAiRuntimePanelFieldLabels: Readonly<Record<LocalAiRuntimePanelFieldKey, DisplayText>> = {
  eventId: PortalDetails.EventId,
  sentAt: PortalDetails.SentAt,
  runtimeReference: PortalDetails.RuntimeReference,
  provider: PortalDetails.Provider,
  model: PortalDetails.Model,
  loadState: PortalDetails.LoadState,
  capability: PortalDetails.Capability,
  resourceClass: PortalDetails.ResourceClass,
  degradedState: PortalDetails.DegradedState,
  privacyMode: PortalDetails.PrivacyMode,
  executionState: PortalDetails.ExecutionState,
  reason: PortalDetails.Reason,
  requestId: PortalDetails.RequestId,
  status: PortalDetails.Status,
  state: PortalDetails.State,
  providerSource: PortalDetails.ProviderSource,
  custody: PortalDetails.Custody,
  policyReadiness: PortalDetails.PolicyReadiness,
  adapterBoundary: PortalDetails.AdapterBoundary,
  lastChecked: PortalDetails.LastChecked,
  lastObserved: PortalDetails.LastObserved,
  decisionSource: PortalDetails.DecisionSource,
  productClaim: PortalDetails.ProductClaim,
  generatedAt: PortalDetails.GeneratedAt,
  graphNodes: PortalDetails.GraphNodes,
  graphEdges: PortalDetails.GraphEdges,
  graphOmittedEdges: PortalDetails.GraphOmittedEdges,
  evidenceReferences: PortalDetails.EvidenceReferences,
  deletedEvidence: PortalDetails.DeletedEvidence,
  rowCount: PortalDetails.RowCount,
};

export function createLocalAiRuntimePanelIntent(
  runtimeEvent: PortalRouteEventRecord | null,
  lanAiJobEvent: PortalRouteEventRecord | null,
  memoryGraphReadModel: PortalActivityMemoryGraphReadModel | null = null,
  parentAssistantBoundaryEvent: PortalRouteEventRecord | null = null
): LocalAiRuntimePanelIntent {
  const template = localAiRuntimePanelTemplate({
    runtimeStatus: normalizeRuntimeStatus(runtimeEvent),
    householdJob: normalizeHouseholdJob(lanAiJobEvent),
    memoryGraph: normalizeMemoryGraph(memoryGraphReadModel),
    remoteAssistantBoundary: normalizeRemoteAssistantBoundary(parentAssistantBoundaryEvent),
  });

  return {
    eyebrow: decodeDisplayText('Local child-device AI'),
    title: decodeDisplayText('AI jobs and runtime activity'),
    body: decodeDisplayText('Service-reported local AI status and LAN job rows only.'),
    emptyMessage: decodeDisplayText('No local AI runtime or job event has been reported yet.'),
    emptyStatus: decodePortalDetailValue('not-reported'),
    productClaim: decodePortalDetailValue('runtime-read-model-only'),
    summaryDetails: [
      {
        label: PortalDetails.Status,
        value: decodePortalDetailValue(template.summaryStatus),
      },
      {
        label: PortalDetails.ReadModelRows,
        value: decodePortalDetailValue(template.summaryReadModelRows),
      },
      {
        label: PortalDetails.ProductClaim,
        value: decodePortalDetailValue(template.summaryProductClaim),
      },
    ],
    cards: template.cards.map((card) => ({
      title: LocalAiRuntimePanelCardTitles[card.kind],
      details: card.details.map((detail) => ({
        label: LocalAiRuntimePanelFieldLabels[detail.fieldKey],
        value: decodePortalDetailValue(detail.value),
      })),
    })),
  };
}

function normalizeRuntimeStatus(event: PortalRouteEventRecord | null): LocalAiRuntimeStatusInput | null {
  if (event?.event !== AgentEvent.LocalAiRuntimeStatusReported) {
    return null;
  }
  return {
    eventId: stringOrNull(event.eventId),
    sentAt: stringOrNull(event.sentAt),
    runtimeReference: payloadValue(event, PortalAgentProtocolField.LocalAiRuntimeReferenceId),
    provider: payloadValue(event, PortalAgentProtocolField.LocalAiProviderId),
    model: payloadValue(event, PortalAgentProtocolField.LocalAiModelId),
    loadState: payloadValue(event, PortalAgentProtocolField.LoadState),
    capability: payloadValue(event, PortalAgentProtocolField.LocalAiCapabilityFlags),
    resourceClass: payloadValue(event, PortalAgentProtocolField.LocalAiResourceClass),
    degradedState: payloadValue(event, PortalAgentProtocolField.LocalAiDegradedState),
    privacyMode: payloadValue(event, PortalAgentProtocolField.LocalAiPrivacyMode),
    executionState: payloadValue(event, PortalAgentProtocolField.LocalAiExecutionState),
    reason: payloadValue(event, PortalAgentProtocolField.LocalAiUnavailableReason),
  };
}

function normalizeHouseholdJob(event: PortalRouteEventRecord | null): LocalAiHouseholdJobInput | null {
  if (event?.event !== AgentEvent.LanAiJobReported) {
    return null;
  }
  return {
    eventId: stringOrNull(event.eventId),
    sentAt: stringOrNull(event.sentAt),
    requestId: payloadValue(event, PortalAgentProtocolField.LanAiJobId),
    status: payloadValue(event, PortalAgentProtocolField.LanAiJobStatus),
    state: payloadValue(event, PortalAgentProtocolField.LanAiJobState),
    provider: payloadValue(event, PortalAgentProtocolField.LocalAiProviderId),
    providerSource: payloadValue(event, PortalAgentProtocolField.LocalAiProviderSource),
    capability: payloadValue(event, PortalAgentProtocolField.LocalAiCapabilityFlags),
    resourceClass: payloadValue(event, PortalAgentProtocolField.LocalAiResourceClass),
    loadState: payloadValue(event, PortalAgentProtocolField.LocalAiAdapterReadinessState),
    privacyMode: payloadValue(event, PortalAgentProtocolField.LocalAiPrivacyMode),
    custody: payloadValue(event, PortalAgentProtocolField.LanAiProviderCustodyLabel),
    policyReadiness: payloadValue(event, PortalAgentProtocolField.LanAiProviderRoutingState),
    adapterBoundary: payloadValue(event, PortalAgentProtocolField.ClaimBoundary),
    leaseId: payloadValue(event, PortalAgentProtocolField.LanControllerLeaseId),
    lastChecked: payloadValue(event, PortalAgentProtocolField.LanControllerLeaseIssuedAt),
    lastObserved: payloadValue(event, PortalAgentProtocolField.LanControllerLeaseExpiresAt),
    decisionSource: payloadValue(event, PortalAgentProtocolField.LanParentAuthority),
    executionState: payloadValue(event, PortalAgentProtocolField.LocalAiExecutionState),
    reason: payloadValue(event, PortalAgentProtocolField.Reason),
  };
}

function normalizeMemoryGraph(readModel: PortalActivityMemoryGraphReadModel | null): LocalAiMemoryGraphInput | null {
  if (readModel === null) {
    return null;
  }
  return {
    custody: stringOrNull(readModel.custody),
    capabilityStatus: stringOrNull(readModel.capabilityStatus),
    generatedAt: stringOrNull(readModel.generatedAt),
    returnedNodeCount: readModel.returnedNodeCount,
    returnedEdgeCount: readModel.returnedEdgeCount,
    omittedEdgeCount: readModel.omittedEdgeCount,
    degradedReasons: [...readModel.degradedReasons],
    evidenceReferenceIds: collectMemoryGraphEvidenceReferenceIds(readModel),
  };
}

function normalizeRemoteAssistantBoundary(
  event: PortalRouteEventRecord | null
): LocalAiRemoteAssistantBoundaryInput | null {
  if (
    event?.event !== AgentEvent.ParentAssistantAnswerReported &&
    event?.event !== AgentEvent.ParentAssistantProviderDegraded &&
    event?.event !== AgentEvent.ParentAssistantErrorReported
  ) {
    return null;
  }
  return {
    eventId: stringOrNull(event.eventId),
    sentAt: stringOrNull(event.sentAt),
    requestId: payloadValue(event, PortalAgentProtocolField.ParentAssistantRequestId),
    state: payloadValue(event, PortalAgentProtocolField.ParentAssistantAnswerState),
    provider: payloadValue(event, PortalAgentProtocolField.ParentAssistantProviderRoute),
    adapterBoundary: payloadValue(event, PortalAgentProtocolField.ParentAssistantApiProviderBoundary),
    policyReadiness: payloadValue(event, PortalAgentProtocolField.ParentAssistantApiAuthorizationState),
    custody: payloadValue(event, PortalAgentProtocolField.ParentAssistantApiCustodyLabel),
    deletedEvidence: payloadValue(event, PortalAgentProtocolField.ParentAssistantApiDeletionState),
    privacyMode: payloadValue(event, PortalAgentProtocolField.ParentAssistantApiRetentionState),
    evidenceReferences: payloadValue(event, PortalAgentProtocolField.ParentAssistantEvidenceSummary),
    rowCount: payloadValue(event, PortalAgentProtocolField.ParentAssistantCitationCount),
  };
}

function collectMemoryGraphEvidenceReferenceIds(readModel: PortalActivityMemoryGraphReadModel): string[] {
  const refs: string[] = [];
  for (const node of readModel.nodes) {
    for (const evidenceRef of node.trace.sourceEvidenceReferences) {
      refs.push(evidenceRef.evidenceReferenceId);
    }
  }
  for (const edge of readModel.edges) {
    for (const evidenceRef of edge.trace.sourceEvidenceReferences) {
      refs.push(evidenceRef.evidenceReferenceId);
    }
  }
  return refs;
}

function payloadValue(event: PortalRouteEventRecord, key: string): string | null {
  return scalarString(event.payload?.[key]);
}

function scalarString(value: unknown): string | null {
  if (typeof value === 'string') {
    return value;
  }
  if (typeof value === 'number' || typeof value === 'boolean') {
    return String(value);
  }
  return null;
}

function stringOrNull(value: unknown): string | null {
  return typeof value === 'string' ? value : null;
}

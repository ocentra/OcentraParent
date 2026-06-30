import { AgentEvent } from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';
import { decodeDisplayText, type DisplayText } from '@ocentra-parent/schema-domain/text-contracts';
import type { PortalActivityMemoryGraphReadModel } from './activity-memory-graph';
import { decodePortalDetailValue, type PortalDetailValue, type PortalRouteEventRecord } from './portal-contract-adapter';
import { PortalDetails } from './details';
import {
  localAiRuntimePanelTemplate,
  type LocalAiHouseholdJobInput,
  type LocalAiMemoryGraphInput,
  type LocalAiRemoteAssistantBoundaryInput,
  type LocalAiRuntimePanelCardKind,
  type LocalAiRuntimePanelFieldKey,
  type LocalAiRuntimeStatusInput,
} from './generated/local-ai-runtime-panel';

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
    runtimeReference: payloadValue(event, AgentProtocolDefaults.Field.LocalAiRuntimeReferenceId),
    provider: payloadValue(event, AgentProtocolDefaults.Field.LocalAiProviderId),
    model: payloadValue(event, AgentProtocolDefaults.Field.LocalAiModelId),
    loadState: payloadValue(event, AgentProtocolDefaults.Field.LoadState),
    capability: payloadValue(event, AgentProtocolDefaults.Field.LocalAiCapabilityFlags),
    resourceClass: payloadValue(event, AgentProtocolDefaults.Field.LocalAiResourceClass),
    degradedState: payloadValue(event, AgentProtocolDefaults.Field.LocalAiDegradedState),
    privacyMode: payloadValue(event, AgentProtocolDefaults.Field.LocalAiPrivacyMode),
    executionState: payloadValue(event, AgentProtocolDefaults.Field.LocalAiExecutionState),
    reason: payloadValue(event, AgentProtocolDefaults.Field.LocalAiUnavailableReason),
  };
}

function normalizeHouseholdJob(event: PortalRouteEventRecord | null): LocalAiHouseholdJobInput | null {
  if (event?.event !== AgentEvent.LanAiJobReported) {
    return null;
  }
  return {
    eventId: stringOrNull(event.eventId),
    sentAt: stringOrNull(event.sentAt),
    requestId: payloadValue(event, AgentProtocolDefaults.Field.LanAiJobId),
    status: payloadValue(event, AgentProtocolDefaults.Field.LanAiJobStatus),
    state: payloadValue(event, AgentProtocolDefaults.Field.LanAiJobState),
    provider: payloadValue(event, AgentProtocolDefaults.Field.LocalAiProviderId),
    providerSource: payloadValue(event, AgentProtocolDefaults.Field.LocalAiProviderSource),
    capability: payloadValue(event, AgentProtocolDefaults.Field.LocalAiCapabilityFlags),
    resourceClass: payloadValue(event, AgentProtocolDefaults.Field.LocalAiResourceClass),
    loadState: payloadValue(event, AgentProtocolDefaults.Field.LocalAiAdapterReadinessState),
    privacyMode: payloadValue(event, AgentProtocolDefaults.Field.LocalAiPrivacyMode),
    custody: payloadValue(event, AgentProtocolDefaults.Field.LanAiProviderCustodyLabel),
    policyReadiness: payloadValue(event, AgentProtocolDefaults.Field.LanAiProviderRoutingState),
    adapterBoundary: payloadValue(event, AgentProtocolDefaults.Field.ClaimBoundary),
    leaseId: payloadValue(event, AgentProtocolDefaults.Field.LanControllerLeaseId),
    lastChecked: payloadValue(event, AgentProtocolDefaults.Field.LanControllerLeaseIssuedAt),
    lastObserved: payloadValue(event, AgentProtocolDefaults.Field.LanControllerLeaseExpiresAt),
    decisionSource: payloadValue(event, AgentProtocolDefaults.Field.LanParentAuthority),
    executionState: payloadValue(event, AgentProtocolDefaults.Field.LocalAiExecutionState),
    reason: payloadValue(event, AgentProtocolDefaults.Field.Reason),
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

function normalizeRemoteAssistantBoundary(event: PortalRouteEventRecord | null): LocalAiRemoteAssistantBoundaryInput | null {
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
    requestId: payloadValue(event, AgentProtocolDefaults.Field.ParentAssistantRequestId),
    state: payloadValue(event, AgentProtocolDefaults.Field.ParentAssistantAnswerState),
    provider: payloadValue(event, AgentProtocolDefaults.Field.ParentAssistantProviderRoute),
    adapterBoundary: payloadValue(event, AgentProtocolDefaults.Field.ParentAssistantApiProviderBoundary),
    policyReadiness: payloadValue(event, AgentProtocolDefaults.Field.ParentAssistantApiAuthorizationState),
    custody: payloadValue(event, AgentProtocolDefaults.Field.ParentAssistantApiCustodyLabel),
    deletedEvidence: payloadValue(event, AgentProtocolDefaults.Field.ParentAssistantApiDeletionState),
    privacyMode: payloadValue(event, AgentProtocolDefaults.Field.ParentAssistantApiRetentionState),
    evidenceReferences: payloadValue(event, AgentProtocolDefaults.Field.ParentAssistantEvidenceSummary),
    rowCount: payloadValue(event, AgentProtocolDefaults.Field.ParentAssistantCitationCount),
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

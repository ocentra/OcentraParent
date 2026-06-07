import {
  AgentEvent,
  AgentProtocolDefaults,
  type AgentEventEnvelope,
} from '@ocentra-parent/agent-protocol-domain/contracts';
import { decodeDisplayText, type DisplayText } from '@ocentra-parent/text-domain/contracts';
import type { PortalActivityMemoryGraphReadModel } from './activity-memory-graph';
import { decodePortalDetailValue, type PortalDetailValue } from './detail-values';
import { PortalDetails } from './details';

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

export function createLocalAiRuntimePanelIntent(
  runtimeEvent: AgentEventEnvelope | null,
  lanAiJobEvent: AgentEventEnvelope | null,
  memoryGraphReadModel: PortalActivityMemoryGraphReadModel | null = null,
  parentAssistantBoundaryEvent: AgentEventEnvelope | null = null
): LocalAiRuntimePanelIntent {
  const cards = [
    runtimeStatusCard(runtimeEvent),
    lanAiJobCard(lanAiJobEvent),
    memoryGraphCard(memoryGraphReadModel),
    parentAssistantBoundaryCard(parentAssistantBoundaryEvent),
  ].filter((card): card is LocalAiRuntimePanelCard => card !== null);
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
        value: decodePortalDetailValue(cards.length > 0 ? 'reported' : 'not-reported'),
      },
      {
        label: PortalDetails.ReadModelRows,
        value: decodePortalDetailValue(String(cards.length)),
      },
      {
        label: PortalDetails.ProductClaim,
        value: decodePortalDetailValue('no-model-quality-or-enforcement-claim'),
      },
    ],
    cards,
  };
}

function runtimeStatusCard(event: AgentEventEnvelope | null): LocalAiRuntimePanelCard | null {
  if (event === null || event.event !== AgentEvent.LocalAiRuntimeStatusReported) {
    return null;
  }
  return {
    title: decodeDisplayText('Local AI runtime status'),
    details: [
      detail(PortalDetails.EventId, event.eventId),
      detail(PortalDetails.SentAt, event.sentAt),
      detail(PortalDetails.RuntimeReference, event.payload[AgentProtocolDefaults.Field.LocalAiRuntimeReferenceId]),
      detail(PortalDetails.Provider, event.payload[AgentProtocolDefaults.Field.LocalAiProviderId]),
      detail(PortalDetails.Model, event.payload[AgentProtocolDefaults.Field.LocalAiModelId]),
      detail(PortalDetails.LoadState, event.payload[AgentProtocolDefaults.Field.LoadState]),
      detail(PortalDetails.Capability, event.payload[AgentProtocolDefaults.Field.LocalAiCapabilityFlags]),
      detail(PortalDetails.ResourceClass, event.payload[AgentProtocolDefaults.Field.LocalAiResourceClass]),
      detail(PortalDetails.DegradedState, event.payload[AgentProtocolDefaults.Field.LocalAiDegradedState]),
      detail(PortalDetails.PrivacyMode, event.payload[AgentProtocolDefaults.Field.LocalAiPrivacyMode]),
      detail(PortalDetails.ExecutionState, event.payload[AgentProtocolDefaults.Field.LocalAiExecutionState]),
      detail(PortalDetails.Reason, event.payload[AgentProtocolDefaults.Field.LocalAiUnavailableReason]),
    ],
  };
}

function lanAiJobCard(event: AgentEventEnvelope | null): LocalAiRuntimePanelCard | null {
  if (event === null || event.event !== AgentEvent.LanAiJobReported) {
    return null;
  }
  return {
    title: decodeDisplayText('Household AI job activity'),
    details: [
      detail(PortalDetails.EventId, event.eventId),
      detail(PortalDetails.SentAt, event.sentAt),
      detail(PortalDetails.RequestId, event.payload[AgentProtocolDefaults.Field.LanAiJobId]),
      detail(PortalDetails.Status, event.payload[AgentProtocolDefaults.Field.LanAiJobStatus]),
      detail(PortalDetails.State, event.payload[AgentProtocolDefaults.Field.LanAiJobState]),
      detail(PortalDetails.Provider, event.payload[AgentProtocolDefaults.Field.LocalAiProviderId]),
      detail(PortalDetails.ProviderSource, event.payload[AgentProtocolDefaults.Field.LocalAiProviderSource]),
      detail(PortalDetails.Capability, event.payload[AgentProtocolDefaults.Field.LocalAiCapabilityFlags]),
      detail(PortalDetails.ResourceClass, event.payload[AgentProtocolDefaults.Field.LocalAiResourceClass]),
      detail(PortalDetails.LoadState, event.payload[AgentProtocolDefaults.Field.LocalAiAdapterReadinessState]),
      detail(PortalDetails.PrivacyMode, event.payload[AgentProtocolDefaults.Field.LocalAiPrivacyMode]),
      detail(PortalDetails.Custody, event.payload[AgentProtocolDefaults.Field.LanAiProviderCustodyLabel]),
      detail(PortalDetails.PolicyReadiness, event.payload[AgentProtocolDefaults.Field.LanAiProviderRoutingState]),
      detail(PortalDetails.AdapterBoundary, event.payload[AgentProtocolDefaults.Field.ClaimBoundary]),
      detail(PortalDetails.RequestId, event.payload[AgentProtocolDefaults.Field.LanControllerLeaseId]),
      detail(PortalDetails.LastChecked, event.payload[AgentProtocolDefaults.Field.LanControllerLeaseIssuedAt]),
      detail(PortalDetails.LastObserved, event.payload[AgentProtocolDefaults.Field.LanControllerLeaseExpiresAt]),
      detail(PortalDetails.DecisionSource, event.payload[AgentProtocolDefaults.Field.LanParentAuthority]),
      detail(PortalDetails.ExecutionState, event.payload[AgentProtocolDefaults.Field.LocalAiExecutionState]),
      detail(PortalDetails.Reason, event.payload[AgentProtocolDefaults.Field.Reason]),
      detail(PortalDetails.ProductClaim, 'worker-only-child-agent-authority'),
    ],
  };
}

function memoryGraphCard(readModel: PortalActivityMemoryGraphReadModel | null): LocalAiRuntimePanelCard | null {
  if (readModel === null) {
    return null;
  }
  return {
    title: decodeDisplayText('Cited memory and graph evidence'),
    details: [
      detail(PortalDetails.Custody, readModel.custody),
      detail(PortalDetails.Capability, readModel.capabilityStatus),
      detail(PortalDetails.GeneratedAt, readModel.generatedAt),
      detail(PortalDetails.GraphNodes, readModel.returnedNodeCount),
      detail(PortalDetails.GraphEdges, readModel.returnedEdgeCount),
      detail(PortalDetails.GraphOmittedEdges, readModel.omittedEdgeCount),
      detail(PortalDetails.EvidenceReferences, memoryGraphEvidenceRefs(readModel)),
      detail(PortalDetails.DegradedState, detailList(readModel.degradedReasons)),
      detail(PortalDetails.ProductClaim, 'source-cited-memory-graph-read-model-only'),
    ],
  };
}

function parentAssistantBoundaryCard(event: AgentEventEnvelope | null): LocalAiRuntimePanelCard | null {
  if (
    event === null ||
    ![
      AgentEvent.ParentAssistantAnswerReported,
      AgentEvent.ParentAssistantProviderDegraded,
      AgentEvent.ParentAssistantErrorReported,
    ].includes(event.event)
  ) {
    return null;
  }
  return {
    title: decodeDisplayText('Remote assistant boundary'),
    details: [
      detail(PortalDetails.EventId, event.eventId),
      detail(PortalDetails.SentAt, event.sentAt),
      detail(PortalDetails.RequestId, event.payload[AgentProtocolDefaults.Field.ParentAssistantRequestId]),
      detail(PortalDetails.State, event.payload[AgentProtocolDefaults.Field.ParentAssistantAnswerState]),
      detail(PortalDetails.Provider, event.payload[AgentProtocolDefaults.Field.ParentAssistantProviderRoute]),
      detail(
        PortalDetails.AdapterBoundary,
        event.payload[AgentProtocolDefaults.Field.ParentAssistantApiProviderBoundary]
      ),
      detail(
        PortalDetails.PolicyReadiness,
        event.payload[AgentProtocolDefaults.Field.ParentAssistantApiAuthorizationState]
      ),
      detail(PortalDetails.Custody, event.payload[AgentProtocolDefaults.Field.ParentAssistantApiCustodyLabel]),
      detail(PortalDetails.DeletedEvidence, event.payload[AgentProtocolDefaults.Field.ParentAssistantApiDeletionState]),
      detail(PortalDetails.PrivacyMode, event.payload[AgentProtocolDefaults.Field.ParentAssistantApiRetentionState]),
      detail(
        PortalDetails.EvidenceReferences,
        event.payload[AgentProtocolDefaults.Field.ParentAssistantEvidenceSummary]
      ),
      detail(PortalDetails.RowCount, event.payload[AgentProtocolDefaults.Field.ParentAssistantCitationCount]),
      detail(PortalDetails.ProductClaim, 'remote-assistant-report-only-local-policy-authority'),
    ],
  };
}

function memoryGraphEvidenceRefs(readModel: PortalActivityMemoryGraphReadModel): string {
  const refs = new Set<string>();
  for (const node of readModel.nodes) {
    for (const evidenceRef of node.trace.sourceEvidenceReferences) {
      refs.add(evidenceRef.evidenceReferenceId);
    }
  }
  for (const edge of readModel.edges) {
    for (const evidenceRef of edge.trace.sourceEvidenceReferences) {
      refs.add(evidenceRef.evidenceReferenceId);
    }
  }
  return refs.size === 0 ? 'not-reported' : Array.from(refs).join(',');
}

function detailList(values: readonly string[]): string {
  return values.length === 0 ? 'not-reported' : values.join(',');
}

function detail(label: DisplayText, value: unknown): LocalAiRuntimePanelDetail {
  return {
    label,
    value: decodePortalDetailValue(value === undefined || value === null ? 'not-reported' : String(value)),
  };
}

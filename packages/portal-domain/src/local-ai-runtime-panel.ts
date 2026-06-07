import {
  AgentEvent,
  AgentProtocolDefaults,
  type AgentEventEnvelope,
} from '@ocentra-parent/agent-protocol-domain/contracts';
import { decodeDisplayText, type DisplayText } from '@ocentra-parent/text-domain/contracts';
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
  lanAiJobEvent: AgentEventEnvelope | null
): LocalAiRuntimePanelIntent {
  const cards = [runtimeStatusCard(runtimeEvent), lanAiJobCard(lanAiJobEvent)].filter(
    (card): card is LocalAiRuntimePanelCard => card !== null
  );
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
      detail(PortalDetails.Custody, event.payload[AgentProtocolDefaults.Field.LanAiProviderCustodyLabel]),
      detail(PortalDetails.ExecutionState, event.payload[AgentProtocolDefaults.Field.LocalAiExecutionState]),
      detail(PortalDetails.Reason, event.payload[AgentProtocolDefaults.Field.Reason]),
    ],
  };
}

function detail(label: DisplayText, value: unknown): LocalAiRuntimePanelDetail {
  return {
    label,
    value: decodePortalDetailValue(value === undefined || value === null ? 'not-reported' : String(value)),
  };
}

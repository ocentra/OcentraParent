import {
  AgentEvent,
  AgentProtocolDefaults,
  type AgentEventEnvelope,
} from '@ocentra-parent/agent-protocol-domain/contracts';
import type { LogFieldValue } from '@ocentra-parent/logging-domain/contracts';
import {
  PortalDetails,
  PortalDom,
  PortalFormatting,
  PortalText,
  PortalTextToken,
  decodePortalDetailValue,
  type PortalDetailValue,
  type PortalDisplayText,
} from '@ocentra-parent/portal-domain/contracts';
import { appendDetail } from './detail-list';
import { latestCommandResult } from './event-results';
import type { PortalLiveActivityState } from './live-activity-state';
import type { PortalRuntimeState } from './portal-state';

type AgentPayloadField = (typeof AgentProtocolDefaults.Field)[keyof typeof AgentProtocolDefaults.Field];

export function renderPolicyPreview(
  container: HTMLElement,
  state: PortalRuntimeState,
  liveActivity: PortalLiveActivityState
): void {
  const runtimeEvent = latestCommandResult(state.events, AgentEvent.LocalAiRuntimeStatusReported);
  const panel = panelWithTitle(PortalText.Resolve(PortalTextToken.PolicyPreview));
  const metadata = document.createElement(PortalDom.Tags.DefinitionList);

  appendRuntimeDetails(metadata, runtimeEvent);
  appendDecisionPreviewDetails(metadata, liveActivity);
  appendDetail(
    metadata,
    PortalDetails.Enforcement,
    decodePortalDetailValue(PortalText.Resolve(PortalTextToken.PolicyPreviewNoEnforcement))
  );

  panel.append(metadata);
  appendRuntimeEmptyState(panel, runtimeEvent);
  panel.append(emptyMessage(PortalText.Resolve(PortalTextToken.NoPolicyPreview)));
  container.append(panel);
}

function appendRuntimeDetails(metadata: HTMLDListElement, runtimeEvent: AgentEventEnvelope | null): void {
  appendDetail(metadata, PortalDetails.Status, eventStatus(runtimeEvent));
  appendDetail(
    metadata,
    PortalDetails.RuntimeReference,
    payloadDetail(runtimeEvent, AgentProtocolDefaults.Field.LocalAiRuntimeReferenceId)
  );
  appendDetail(
    metadata,
    PortalDetails.Provider,
    payloadDetail(runtimeEvent, AgentProtocolDefaults.Field.LocalAiProviderId)
  );
  appendDetail(metadata, PortalDetails.Model, payloadDetail(runtimeEvent, AgentProtocolDefaults.Field.LocalAiModelId));
  appendDetail(metadata, PortalDetails.LoadState, payloadDetail(runtimeEvent, AgentProtocolDefaults.Field.LoadState));
  appendDetail(
    metadata,
    PortalDetails.Capability,
    payloadDetail(runtimeEvent, AgentProtocolDefaults.Field.LocalAiCapabilityFlags)
  );
  appendDetail(
    metadata,
    PortalDetails.ResourceClass,
    payloadDetail(runtimeEvent, AgentProtocolDefaults.Field.LocalAiResourceClass)
  );
  appendDetail(
    metadata,
    PortalDetails.DegradedState,
    payloadDetail(runtimeEvent, AgentProtocolDefaults.Field.LocalAiDegradedState)
  );
  appendDetail(metadata, PortalDetails.LastChecked, payloadDetail(runtimeEvent, AgentProtocolDefaults.Field.CheckedAt));
  appendDetail(
    metadata,
    PortalDetails.Reason,
    payloadDetail(runtimeEvent, AgentProtocolDefaults.Field.LocalAiUnavailableReason)
  );
}

function appendDecisionPreviewDetails(metadata: HTMLDListElement, liveActivity: PortalLiveActivityState): void {
  appendDetail(metadata, PortalDetails.DecisionAction, notReported());
  appendDetail(metadata, PortalDetails.ReasonCodes, notReported());
  appendDetail(metadata, PortalDetails.EvidenceReferences, evidenceReferencesDetail(liveActivity));
  appendDetail(metadata, PortalDetails.UnknownState, notReported());
}

function evidenceReferencesDetail(liveActivity: PortalLiveActivityState): PortalDetailValue {
  const references: LogFieldValue[] = [];
  appendReference(references, liveActivity.browserEvidenceSummary?.browserEvidenceId);
  appendReference(references, liveActivity.networkFlowReadModel?.rows[0]?.eventId);
  appendReference(references, liveActivity.recentSummary?.lastEventId);
  if (references.length === 0) {
    return notReported();
  }
  return decodePortalDetailValue(references.map(String).join(PortalFormatting.EventDetailSeparator));
}

function appendReference(references: LogFieldValue[], reference: LogFieldValue | undefined): void {
  if (reference === undefined || reference === null) {
    return;
  }
  references.push(reference);
}

function panelWithTitle(titleText: PortalDisplayText): HTMLElement {
  const panel = document.createElement(PortalDom.Tags.Section);
  panel.className = PortalDom.Classes.Summary;

  const title = document.createElement(PortalDom.Tags.HeadingTwo);
  title.textContent = titleText;

  panel.append(title);
  return panel;
}

function emptyMessage(messageText: PortalDisplayText): HTMLElement {
  const message = document.createElement(PortalDom.Tags.Paragraph);
  message.className = PortalDom.Classes.CommandResultEmpty;
  message.textContent = messageText;
  return message;
}

function appendRuntimeEmptyState(panel: HTMLElement, runtimeEvent: AgentEventEnvelope | null): void {
  if (runtimeEvent === null) {
    panel.append(emptyMessage(PortalText.Resolve(PortalTextToken.NoLocalAiRuntimeStatus)));
  }
}

function eventStatus(event: AgentEventEnvelope | null): PortalDetailValue {
  if (event === null) {
    return notReported();
  }
  return decodePortalDetailValue(event.severity);
}

function payloadDetail(event: AgentEventEnvelope | null, field: AgentPayloadField): PortalDetailValue {
  if (event === null) {
    return notReported();
  }
  return detailFromValue(event.payload[field]);
}

function detailFromValue(value: LogFieldValue | undefined): PortalDetailValue {
  if (value === undefined || value === null) {
    return notReported();
  }
  return decodePortalDetailValue(String(value));
}

function notReported(): PortalDetailValue {
  return decodePortalDetailValue(PortalText.Resolve(PortalTextToken.NotReported));
}

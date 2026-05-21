import { AgentEvent, type AgentEventEnvelope } from '@ocentra-parent/agent-protocol-domain/contracts';
import {
  PortalDetails,
  PortalDom,
  PortalText,
  PortalTextToken,
  type PortalDisplayText,
} from '@ocentra-parent/portal-domain/contracts';
import { appendDetail } from './detail-list';
import { detailFromValue } from './event-detail-values';
import { latestCommandResult } from './event-results';
import { appendRuntimeDetails } from './local-ai-runtime-details';
import type { PortalLiveActivityState } from './live-activity-state';
import type { PortalRuntimeState } from './portal-state';
import { appendDecisionPreviewDetails, appendReadModelDetails } from './policy-preview-details';
import type { PortalPolicyPreviewReadModel } from './policy-preview-read-model';

export function renderPolicyPreview(
  container: HTMLElement,
  state: PortalRuntimeState,
  liveActivity: PortalLiveActivityState
): void {
  const runtimeEvent = latestCommandResult(state.events, AgentEvent.LocalAiRuntimeStatusReported);
  const policyEvent = liveActivity.policyPreviewEvent;
  const policyReadModel = liveActivity.policyPreviewReadModel;
  const panel = panelWithTitle(PortalText.Resolve(PortalTextToken.PolicyPreview));
  const metadata = document.createElement(PortalDom.Tags.DefinitionList);

  appendRuntimeDetails(metadata, runtimeEvent);
  appendReadModelDetails(metadata, policyEvent, policyReadModel);
  appendDecisionPreviewDetails(metadata, policyReadModel);
  appendDetail(
    metadata,
    PortalDetails.Enforcement,
    detailFromValue(PortalText.Resolve(PortalTextToken.PolicyPreviewNoEnforcement))
  );

  panel.append(metadata);
  appendRuntimeEmptyState(panel, runtimeEvent);
  appendPolicyPreviewState(panel, policyEvent, policyReadModel);
  container.append(panel);
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

function appendPolicyPreviewState(
  panel: HTMLElement,
  event: AgentEventEnvelope | null,
  readModel: PortalPolicyPreviewReadModel | null
): void {
  if (event === null || readModel === null || hasNoRows(readModel)) {
    panel.append(emptyMessage(PortalText.Resolve(PortalTextToken.NoPolicyPreview)));
  }
}

function hasNoRows(readModel: PortalPolicyPreviewReadModel): boolean {
  return readModel.returned === 0;
}

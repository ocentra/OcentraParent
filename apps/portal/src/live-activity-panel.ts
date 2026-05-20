import { AgentProtocolDefaults, type AgentEventEnvelope } from '@ocentra-parent/agent-protocol-domain/contracts';
import type { LogFieldValue } from '@ocentra-parent/logging-domain/contracts';
import {
  PortalDetails,
  PortalDom,
  PortalText,
  PortalTextToken,
  decodePortalDetailValue,
  type PortalDetailValue,
  type PortalDisplayText,
} from '@ocentra-parent/portal-domain/contracts';
import { appendDetail } from './detail-list';
import { renderDiagnosticsPanel } from './diagnostics-panel';
import { resolveLiveActivityState, type PortalLiveActivityState } from './live-activity-state';
import type { PortalRuntimeState } from './portal-state';

export function renderLiveActivityOverview(container: HTMLElement, state: PortalRuntimeState): void {
  const liveActivity = resolveLiveActivityState(state.events);
  renderEvidenceStore(container, liveActivity);
  renderRecentActivity(container, liveActivity);
  renderDiagnosticsPanel(container, state);
}

function renderEvidenceStore(container: HTMLElement, liveActivity: PortalLiveActivityState): void {
  const panel = panelWithTitle(PortalText.Resolve(PortalTextToken.EvidenceStore));
  const metadata = document.createElement(PortalDom.Tags.DefinitionList);

  appendDetail(metadata, PortalDetails.Status, eventStatus(liveActivity.ingestEvent));
  if (liveActivity.ingestStatus === null) {
    appendDetail(metadata, PortalDetails.Reason, eventReason(liveActivity.ingestEvent));
    panel.append(metadata, emptyMessage(PortalText.Resolve(PortalTextToken.NoActivityStatus)));
    container.append(panel);
    return;
  }

  appendDetail(metadata, PortalDetails.Database, detailFromValue(liveActivity.ingestStatus.databaseReady));
  appendDetail(metadata, PortalDetails.EventsStored, detailFromValue(liveActivity.ingestStatus.eventsStored));
  appendDetail(metadata, PortalDetails.EventsIngested, detailFromValue(liveActivity.ingestStatus.eventsIngested));
  appendDetail(metadata, PortalDetails.DuplicateEvents, detailFromValue(liveActivity.ingestStatus.duplicateEvents));
  appendDetail(metadata, PortalDetails.LastEvent, detailFromValue(liveActivity.ingestStatus.lastEventId));
  panel.append(metadata);
  container.append(panel);
}

function renderRecentActivity(container: HTMLElement, liveActivity: PortalLiveActivityState): void {
  const panel = panelWithTitle(PortalText.Resolve(PortalTextToken.RecentActivity));
  const metadata = document.createElement(PortalDom.Tags.DefinitionList);

  appendDetail(metadata, PortalDetails.Status, eventStatus(liveActivity.recentSummaryEvent));
  if (liveActivity.recentSummary === null) {
    appendDetail(metadata, PortalDetails.Reason, eventReason(liveActivity.recentSummaryEvent));
    panel.append(metadata, emptyMessage(PortalText.Resolve(PortalTextToken.NoActivityStatus)));
    container.append(panel);
    return;
  }

  appendDetail(metadata, PortalDetails.RowsReturned, detailFromValue(liveActivity.recentSummary.returned));
  appendDetail(metadata, PortalDetails.LastObserved, detailFromValue(liveActivity.recentSummary.lastObservedAt));
  appendDetail(metadata, PortalDetails.FirstObserved, detailFromValue(liveActivity.recentSummary.firstObservedAt));
  appendDetail(metadata, PortalDetails.ActivityKind, detailFromValue(liveActivity.recentSummary.mostRecentKind));
  appendDetail(metadata, PortalDetails.Observer, detailFromValue(liveActivity.recentSummary.mostRecentObserver));
  appendDetail(metadata, PortalDetails.SubjectKind, detailFromValue(liveActivity.recentSummary.mostRecentSubjectKind));
  appendDetail(metadata, PortalDetails.SubjectId, detailFromValue(liveActivity.recentSummary.mostRecentSubjectId));
  appendDetail(metadata, PortalDetails.Subject, detailFromValue(liveActivity.recentSummary.mostRecentSubjectName));
  panel.append(metadata);

  if (liveActivity.recentSummary.returned === 0) {
    panel.append(emptyMessage(PortalText.Resolve(PortalTextToken.NoRecentActivity)));
  }

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

function eventStatus(event: AgentEventEnvelope | null): PortalDetailValue {
  if (event === null) {
    return notReported();
  }
  return decodePortalDetailValue(event.severity);
}

function eventReason(event: AgentEventEnvelope | null): PortalDetailValue {
  if (event === null) {
    return notReported();
  }
  return detailFromValue(event.payload[AgentProtocolDefaults.Field.Reason]);
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

import type { BrowserEvidenceReadModel, BrowserTabEvidence } from '@ocentra-parent/activity-domain/browser';
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
import { renderActivityMemoryGraph } from './activity-memory-graph-panel';
import { renderBrowserManagedStatus } from './browser-status-panel';
import { renderDiagnosticsPanel } from './diagnostics-panel';
import { resolveLiveActivityState, type PortalLiveActivityState } from './live-activity-state';
import { renderNetworkFlow } from './live-network-flow-panel';
import { renderPolicyPreview } from './policy-preview-panel';
import type { PortalRuntimeState } from './portal-state';

export function renderLiveActivityOverview(container: HTMLElement, state: PortalRuntimeState): void {
  const liveActivity = resolveLiveActivityState(state.events);
  renderEvidenceStore(container, liveActivity);
  renderBrowserManagedStatus(container, liveActivity);
  renderBrowserEvidence(container, liveActivity);
  renderActivityMemoryGraph(container, liveActivity);
  renderNetworkFlow(container, liveActivity);
  renderPolicyPreview(container, state, liveActivity);
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

function renderBrowserEvidence(container: HTMLElement, liveActivity: PortalLiveActivityState): void {
  const panel = panelWithTitle(PortalText.Resolve(PortalTextToken.BrowserEvidence));
  const metadata = document.createElement(PortalDom.Tags.DefinitionList);

  appendDetail(metadata, PortalDetails.Status, eventStatus(liveActivity.browserEvidenceEvent));
  if (liveActivity.browserEvidenceReadModel === null) {
    appendDetail(metadata, PortalDetails.Reason, eventReason(liveActivity.browserEvidenceEvent));
    panel.append(metadata, emptyMessage(PortalText.Resolve(PortalTextToken.NoBrowserEvidence)));
    container.append(panel);
    return;
  }
  appendBrowserEvidenceReadModelDetails(metadata, liveActivity.browserEvidenceReadModel);
  panel.append(metadata);

  if (liveActivity.browserEvidenceReadModel.returned === 0) {
    panel.append(emptyMessage(PortalText.Resolve(PortalTextToken.NoBrowserEvidence)));
  }

  container.append(panel);
}

function appendBrowserEvidenceReadModelDetails(metadata: HTMLDListElement, readModel: BrowserEvidenceReadModel): void {
  const latestRow = readModel.rows[0] ?? null;
  appendDetail(metadata, PortalDetails.RowsReturned, detailFromValue(readModel.returned));
  appendDetail(metadata, PortalDetails.LastObserved, detailFromValue(readModel.latestObservedAt));
  appendDetail(metadata, PortalDetails.GeneratedAt, detailFromValue(readModel.generatedAt));
  appendDetail(metadata, PortalDetails.EventId, detailFromValue(readModel.latestEventId));
  appendBrowserEvidenceRowIdentity(metadata, latestRow);
  appendBrowserEvidenceRowTarget(metadata, latestRow);
  appendBrowserEvidenceRowState(metadata, readModel, latestRow);
}

function appendBrowserEvidenceRowIdentity(metadata: HTMLDListElement, latestRow: BrowserTabEvidence | null): void {
  appendDetail(metadata, PortalDetails.BrowserEvidence, detailFromValue(latestRow?.browserEvidenceId));
  appendDetail(metadata, PortalDetails.BrowserFamily, detailFromValue(latestRow?.browserFamily));
  appendDetail(metadata, PortalDetails.BrowserChannel, detailFromValue(latestRow?.browserChannel));
  appendDetail(metadata, PortalDetails.Source, detailFromValue(latestRow?.sourceId));
  appendDetail(metadata, PortalDetails.ManagedSession, detailFromValue(latestRow?.managedBrowserSessionId));
  appendDetail(metadata, PortalDetails.Profile, detailFromValue(latestRow?.profileId));
}

function appendBrowserEvidenceRowTarget(metadata: HTMLDListElement, latestRow: BrowserTabEvidence | null): void {
  appendDetail(metadata, PortalDetails.Domain, detailFromValue(latestRow?.domain));
  appendDetail(metadata, PortalDetails.Url, detailFromValue(latestRow?.url));
  appendDetail(metadata, PortalDetails.Title, detailFromValue(latestRow?.title));
  appendDetail(metadata, PortalDetails.ProcessId, detailFromValue(latestRow?.processId));
  appendDetail(metadata, PortalDetails.TargetId, detailFromValue(latestRow?.targetId));
}

function appendBrowserEvidenceRowState(
  metadata: HTMLDListElement,
  readModel: BrowserEvidenceReadModel,
  latestRow: BrowserTabEvidence | null
): void {
  appendDetail(metadata, PortalDetails.ActiveState, detailFromValue(latestRow?.activeState));
  appendDetail(metadata, PortalDetails.Capability, detailFromValue(readModel.capabilityStatus));
  appendDetail(metadata, PortalDetails.Custody, detailFromValue(readModel.custodyLabel));
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

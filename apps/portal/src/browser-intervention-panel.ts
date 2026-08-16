import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/portal-domain/display-text';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { PortalDetails } from '@ocentra-parent/portal-domain/details';
import type {
  PortalBrowserInterventionReadModel,
  PortalBrowserInterventionRow,
} from '@ocentra-parent/portal-domain/live-activity-state';
import {
  decodeParentPortalDetailValue,
  ParentAgentProtocolField,
  type ParentRouteEventSnapshot,
  type ParentPortalDetailValue,
} from '../generated/parent-ui-bridge';
import {
  appendDetail,
  notReportedDetail,
  portalDetailFromSequence as detailFromSequence,
  portalDetailFromValue as detailFromValue,
} from './detail-list';
import { type PortalDisplayText } from '@ocentra-parent/portal-domain/display-text';
import type { PortalLiveActivityState } from './live-activity-state';

export function renderBrowserIntervention(container: HTMLElement, liveActivity: PortalLiveActivityState): void {
  const panel = panelWithTitle(resolvePortalDevText(PortalDevTextToken.BrowserIntervention));
  const metadata = document.createElement(PortalDom.Tags.DefinitionList);

  appendDetail(metadata, PortalDetails.Status, eventStatus(liveActivity.browserInterventionEvent));
  if (liveActivity.browserInterventionReadModel === null) {
    appendDetail(metadata, PortalDetails.Reason, eventReason(liveActivity.browserInterventionEvent));
    panel.append(metadata, emptyMessage(resolvePortalDevText(PortalDevTextToken.NoBrowserIntervention)));
    container.append(panel);
    return;
  }
  appendBrowserInterventionReadModelDetails(metadata, liveActivity.browserInterventionReadModel);
  panel.append(metadata);

  if (liveActivity.browserInterventionReadModel.returned === 0) {
    panel.append(emptyMessage(resolvePortalDevText(PortalDevTextToken.NoBrowserIntervention)));
  }

  container.append(panel);
}

function appendBrowserInterventionReadModelDetails(
  metadata: HTMLDListElement,
  readModel: PortalBrowserInterventionReadModel
): void {
  const latestRow = readModel.rows[0] ?? null;
  appendDetail(metadata, PortalDetails.RowsReturned, detailFromValue(readModel.returned));
  appendDetail(metadata, PortalDetails.LastObserved, detailFromValue(readModel.latestObservedAt));
  appendDetail(metadata, PortalDetails.GeneratedAt, detailFromValue(readModel.generatedAt));
  appendDetail(metadata, PortalDetails.EventId, detailFromValue(readModel.latestEventId));
  appendDetail(
    metadata,
    PortalDetails.ManagedSessionIntervention,
    detailFromValue(readModel.managedSessionInterventionCapability)
  );
  appendDetail(
    metadata,
    PortalDetails.UnmanagedBrowserEnforcement,
    detailFromValue(readModel.unmanagedBrowserEnforcement)
  );
  appendBrowserInterventionDecisionDetails(metadata, latestRow);
  appendBrowserInterventionTargetDetails(metadata, latestRow);
  appendBrowserInterventionStateDetails(metadata, latestRow);
}

function appendBrowserInterventionDecisionDetails(
  metadata: HTMLDListElement,
  latestRow: PortalBrowserInterventionRow | null
): void {
  appendDetail(metadata, PortalDetails.BrowserIntervention, detailFromValue(latestRow?.browserInterventionId));
  appendDetail(metadata, PortalDetails.DecisionSource, detailFromValue(latestRow?.decisionSource));
  appendDetail(metadata, PortalDetails.DecisionId, detailFromValue(latestRow?.policyDecisionId));
  appendDetail(metadata, PortalDetails.InterventionActionId, detailFromValue(latestRow?.interventionActionId));
  appendDetail(metadata, PortalDetails.InterventionAuditId, detailFromValue(latestRow?.interventionAuditId));
  appendDetail(metadata, PortalDetails.EvidenceReferences, detailFromList(latestRow?.evidenceReferenceIds));
  appendDetail(metadata, PortalDetails.InterventionAction, detailFromValue(latestRow?.interventionAction));
}

function appendBrowserInterventionTargetDetails(
  metadata: HTMLDListElement,
  latestRow: PortalBrowserInterventionRow | null
): void {
  appendDetail(metadata, PortalDetails.InterventionTargetType, detailFromValue(latestRow?.interventionTargetType));
  appendDetail(metadata, PortalDetails.InterventionTarget, detailFromValue(latestRow?.interventionTargetValue));
  appendDetail(metadata, PortalDetails.Url, detailFromValue(latestRow?.requestedUrl));
  appendDetail(metadata, PortalDetails.ProcessId, detailFromValue(latestRow?.processId));
}

function appendBrowserInterventionStateDetails(
  metadata: HTMLDListElement,
  latestRow: PortalBrowserInterventionRow | null
): void {
  appendDetail(metadata, PortalDetails.InterventionMechanism, detailFromValue(latestRow?.interventionMechanism));
  appendDetail(metadata, PortalDetails.InterventionOutcome, detailFromValue(latestRow?.interventionOutcome));
  appendDetail(metadata, PortalDetails.BrowserBoundary, detailFromValue(latestRow?.browserBoundaryState));
  appendDetail(metadata, PortalDetails.ExactUrlClaim, detailFromValue(latestRow?.exactUrlClaimState));
  appendDetail(metadata, PortalDetails.UnmanagedDetection, detailFromValue(latestRow?.unmanagedDetectionState));
  appendDetail(metadata, PortalDetails.UnmanagedFallbackAction, detailFromValue(latestRow?.unmanagedFallbackAction));
  appendDetail(metadata, PortalDetails.InterventionChildDelivery, detailFromValue(latestRow?.childDeliveryState));
  appendDetail(metadata, PortalDetails.Reason, detailFromValue(latestRow?.reason));
  appendDetail(metadata, PortalDetails.Custody, detailFromValue(latestRow?.custodyLabel));
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

function eventStatus(event: ParentRouteEventSnapshot | null): ParentPortalDetailValue {
  if (event === null) {
    return notReportedDetail();
  }
  return decodeParentPortalDetailValue(event.severity ?? resolvePortalDevText(PortalDevTextToken.NotReported));
}

function eventReason(event: ParentRouteEventSnapshot | null): ParentPortalDetailValue {
  if (event === null) {
    return notReportedDetail();
  }
  return detailFromValue(event.payload?.[ParentAgentProtocolField.Reason]);
}

function detailFromList(values: readonly unknown[] | undefined): ParentPortalDetailValue {
  if (values === undefined || values.length === 0) {
    return notReportedDetail();
  }
  return detailFromSequence(values);
}

import {
  PortalDetails,
  PortalDom,
  PortalReadableValues,
  PortalText,
  PortalTextToken,
  decodePortalDetailValue,
  type PortalDetailValue,
  type PortalDisplayText,
} from '@ocentra-parent/portal-domain/contracts';
import { appendDetail } from './detail-list';
import { eventStatus, notReported } from './event-detail-values';
import type { PortalLiveActivityState } from './live-activity-state';

export function renderBrowserStatusSummary(container: HTMLElement, liveActivity: PortalLiveActivityState): void {
  const panel = panelWithTitle(PortalText.Resolve(PortalTextToken.BrowserManagedStatus));
  const metadata = document.createElement(PortalDom.Tags.DefinitionList);

  appendDetail(metadata, PortalDetails.Status, eventStatus(liveActivity.browserManagedEvent));
  appendDetail(metadata, PortalDetails.ManagedState, detail(liveActivity.browserManagedStatus?.managedState));
  appendDetail(metadata, PortalDetails.Capability, detail(liveActivity.browserManagedStatus?.capabilityStatus));
  appendDetail(metadata, PortalDetails.BrowserFamily, detail(liveActivity.browserManagedStatus?.browserFamily));
  appendDetail(metadata, PortalDetails.LastObserved, detail(liveActivity.browserManagedStatus?.checkedAt));
  panel.append(metadata);

  if (liveActivity.browserManagedStatus === null) {
    panel.append(emptyMessage(PortalText.Resolve(PortalTextToken.NoBrowserManagedStatus)));
  }

  container.append(panel);
}

export function renderBrowserEvidenceSummary(container: HTMLElement, liveActivity: PortalLiveActivityState): void {
  const panel = panelWithTitle(PortalText.Resolve(PortalTextToken.BrowserEvidence));
  const metadata = document.createElement(PortalDom.Tags.DefinitionList);
  const readModel = liveActivity.browserEvidenceReadModel;
  const latestRow = readModel?.rows[0] ?? null;

  appendDetail(metadata, PortalDetails.Status, eventStatus(liveActivity.browserEvidenceEvent));
  appendDetail(metadata, PortalDetails.RowsReturned, detail(readModel?.returned));
  appendDetail(metadata, PortalDetails.Domain, detail(latestRow?.domain));
  appendDetail(metadata, PortalDetails.ActiveState, detail(latestRow?.activeState));
  appendDetail(metadata, PortalDetails.Custody, detail(readModel?.custodyLabel));
  panel.append(metadata);

  if (readModel === null || readModel.returned === 0) {
    panel.append(emptyMessage(PortalText.Resolve(PortalTextToken.NoBrowserEvidence)));
  }

  container.append(panel);
}

export function renderBrowserProtectionSummary(container: HTMLElement, liveActivity: PortalLiveActivityState): void {
  const panel = panelWithTitle(PortalText.Resolve(PortalTextToken.BrowserIntervention));
  const metadata = document.createElement(PortalDom.Tags.DefinitionList);
  const readModel = liveActivity.browserInterventionReadModel;
  const latestRow = readModel?.rows[0] ?? null;

  appendDetail(metadata, PortalDetails.Status, eventStatus(liveActivity.browserInterventionEvent));
  appendDetail(metadata, PortalDetails.RowsReturned, detail(readModel?.returned));
  appendDetail(
    metadata,
    PortalDetails.ManagedSessionIntervention,
    detail(readModel?.managedSessionInterventionCapability)
  );
  appendDetail(metadata, PortalDetails.UnmanagedBrowserEnforcement, detail(readModel?.unmanagedBrowserEnforcement));
  appendDetail(metadata, PortalDetails.InterventionAction, detail(latestRow?.interventionAction));
  panel.append(metadata);

  if (readModel === null || readModel.returned === 0) {
    panel.append(emptyMessage(PortalText.Resolve(PortalTextToken.NoBrowserIntervention)));
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

function detail(value: unknown): PortalDetailValue {
  if (value === undefined || value === null) {
    return notReported();
  }
  const readableValue = PortalReadableValues[String(value)];
  if (readableValue !== undefined) {
    return decodePortalDetailValue(readableValue);
  }
  return decodePortalDetailValue(String(value));
}

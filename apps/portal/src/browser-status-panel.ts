import { AgentProtocolDefaults, type AgentEventEnvelope } from '@ocentra-parent/agent-protocol-domain/contracts';
import {
  PortalDetails,
  PortalDom,
  PortalText,
  PortalTextToken,
  decodePortalDetailValue,
  type PortalDetailValue,
} from '@ocentra-parent/portal-domain/contracts';
import { appendDetail } from './detail-list';
import type { PortalLiveActivityState } from './live-activity-state';

export function renderBrowserManagedStatus(container: HTMLElement, liveActivity: PortalLiveActivityState): void {
  const panel = document.createElement(PortalDom.Tags.Section);
  panel.className = PortalDom.Classes.Summary;

  const title = document.createElement(PortalDom.Tags.HeadingTwo);
  title.textContent = PortalText.Resolve(PortalTextToken.BrowserManagedStatus);

  const metadata = document.createElement(PortalDom.Tags.DefinitionList);
  appendDetail(metadata, PortalDetails.Status, eventStatus(liveActivity.browserManagedEvent));

  if (liveActivity.browserManagedStatus === null) {
    appendDetail(metadata, PortalDetails.Reason, eventReason(liveActivity.browserManagedEvent));
    panel.append(title, metadata, emptyMessage());
    container.append(panel);
    return;
  }

  appendDetail(metadata, PortalDetails.ManagedState, detailFromValue(liveActivity.browserManagedStatus.managedState));
  appendDetail(metadata, PortalDetails.Capability, detailFromValue(liveActivity.browserManagedStatus.capabilityStatus));
  appendDetail(metadata, PortalDetails.Reason, detailFromValue(liveActivity.browserManagedStatus.degradedReason));
  appendDetail(metadata, PortalDetails.BrowserFamily, detailFromValue(liveActivity.browserManagedStatus.browserFamily));
  appendDetail(
    metadata,
    PortalDetails.BrowserChannel,
    detailFromValue(liveActivity.browserManagedStatus.browserChannel)
  );
  appendDetail(
    metadata,
    PortalDetails.BrowserVersion,
    detailFromValue(liveActivity.browserManagedStatus.browserVersion)
  );
  appendDetail(
    metadata,
    PortalDetails.ManagedSession,
    detailFromValue(liveActivity.browserManagedStatus.managedBrowserSessionId)
  );
  appendDetail(metadata, PortalDetails.Profile, detailFromValue(liveActivity.browserManagedStatus.profilePathRef));
  appendDetail(metadata, PortalDetails.Bridge, detailFromValue(liveActivity.browserManagedStatus.bridgeEndpointRef));
  appendDetail(metadata, PortalDetails.Custody, detailFromValue(liveActivity.browserManagedStatus.custodyLabel));
  appendDetail(metadata, PortalDetails.LastObserved, detailFromValue(liveActivity.browserManagedStatus.checkedAt));
  panel.append(title, metadata);
  container.append(panel);
}

function emptyMessage(): HTMLElement {
  const message = document.createElement(PortalDom.Tags.Paragraph);
  message.className = PortalDom.Classes.CommandResultEmpty;
  message.textContent = PortalText.Resolve(PortalTextToken.NoBrowserManagedStatus);
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

function detailFromValue(value: unknown): PortalDetailValue {
  if (value === undefined || value === null) {
    return notReported();
  }
  return decodePortalDetailValue(String(value));
}

function notReported(): PortalDetailValue {
  return decodePortalDetailValue(PortalText.Resolve(PortalTextToken.NotReported));
}

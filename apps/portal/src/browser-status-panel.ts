import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/portal-domain/display-text';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { PortalDetails } from '@ocentra-parent/portal-domain/details';
import {
  ParentAgentProtocolField,
  decodeParentPortalDetailValue,
  type ParentRouteEventSnapshot,
  type ParentPortalDetailValue,
} from '../generated/parent-ui-bridge';
import { appendDetail, notReportedDetail, portalDetailFromValue as detailFromValue } from './detail-list';
import type { PortalLiveActivityState } from './live-activity-state';

export function renderBrowserManagedStatus(container: HTMLElement, liveActivity: PortalLiveActivityState): void {
  const panel = document.createElement(PortalDom.Tags.Section);
  panel.className = PortalDom.Classes.Summary;

  const title = document.createElement(PortalDom.Tags.HeadingTwo);
  title.textContent = resolvePortalDevText(PortalDevTextToken.BrowserManagedStatus);

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
  message.textContent = resolvePortalDevText(PortalDevTextToken.NoBrowserManagedStatus);
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

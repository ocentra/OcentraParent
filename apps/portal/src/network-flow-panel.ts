import { AgentProtocolDefaults, type AgentEventEnvelope } from '@ocentra-parent/agent-protocol-domain/contracts';
import {
  PortalDetails,
  PortalDom,
  PortalText,
  PortalTextToken,
  decodePortalDetailValue,
  type PortalDisplayText,
  type PortalDetailValue,
} from '@ocentra-parent/portal-domain/contracts';
import type {
  ActivityNetworkFlowIndicator,
  ActivityNetworkFlowRollup,
} from '@ocentra-parent/activity-domain/network-flow';
import type { LogFieldValue } from '@ocentra-parent/logging-domain/contracts';
import { decodeDisplayText } from '@ocentra-parent/text-domain/contracts';
import { appendDetail } from './detail-list';
import type { PortalLiveActivityState } from './live-activity-state';

export function renderNetworkFlow(container: HTMLElement, liveActivity: PortalLiveActivityState): void {
  const panel = panelWithTitle(PortalText.Resolve(PortalTextToken.NetworkFlow));
  const metadata = document.createElement(PortalDom.Tags.DefinitionList);

  appendDetail(metadata, PortalDetails.Status, eventStatus(liveActivity.networkFlowEvent));
  if (liveActivity.networkFlowDigest === null) {
    appendDetail(metadata, PortalDetails.Reason, eventReason(liveActivity.networkFlowEvent));
    panel.append(metadata, emptyMessage(PortalText.Resolve(PortalTextToken.NoNetworkFlow)));
    container.append(panel);
    return;
  }

  appendDetail(
    metadata,
    PortalDetails.RowsReturned,
    detailFromValue(liveActivity.networkFlowEvent?.payload[AgentProtocolDefaults.Field.Returned])
  );
  appendDetail(
    metadata,
    PortalDetails.Capability,
    detailFromValue(liveActivity.networkFlowEvent?.payload[AgentProtocolDefaults.Field.CapabilityStatus])
  );
  appendDetail(
    metadata,
    PortalDetails.Custody,
    detailFromValue(liveActivity.networkFlowEvent?.payload[AgentProtocolDefaults.Field.CustodyLabel])
  );
  panel.append(metadata);

  panel.append(
    renderRollupSection(PortalDetails.TopProcesses, liveActivity.networkFlowDigest.topProcesses),
    renderRollupSection(PortalDetails.TopDestinations, liveActivity.networkFlowDigest.topDestinations),
    renderIndicatorSection(liveActivity.networkFlowDigest.unusualIndicators)
  );

  if (
    liveActivity.networkFlowDigest.topProcesses.length === 0 &&
    liveActivity.networkFlowDigest.topDestinations.length === 0
  ) {
    panel.append(emptyMessage(PortalText.Resolve(PortalTextToken.NoNetworkFlow)));
  }

  container.append(panel);
}

function renderRollupSection(titleText: PortalDisplayText, rollups: readonly ActivityNetworkFlowRollup[]): HTMLElement {
  const section = document.createElement(PortalDom.Tags.Section);

  const heading = document.createElement(PortalDom.Tags.HeadingThree);
  heading.textContent = titleText;
  section.append(heading);

  if (rollups.length === 0) {
    section.append(emptyMessage(PortalText.Resolve(PortalTextToken.NoNetworkFlow)));
    return section;
  }

  const list = document.createElement(PortalDom.Tags.OrderedList);
  for (const rollup of rollups) {
    list.append(renderRollupItem(rollup));
  }
  section.append(list);
  return section;
}

function renderIndicatorSection(indicators: readonly ActivityNetworkFlowIndicator[]): HTMLElement {
  const section = document.createElement(PortalDom.Tags.Section);
  const heading = document.createElement(PortalDom.Tags.HeadingThree);
  heading.textContent = PortalDetails.Indicators;
  section.append(heading);

  if (indicators.length === 0) {
    section.append(emptyMessage(PortalText.Resolve(PortalTextToken.NoNetworkFlow)));
    return section;
  }

  const list = document.createElement(PortalDom.Tags.OrderedList);
  for (const indicator of indicators) {
    list.append(renderIndicatorItem(indicator));
  }
  section.append(list);
  return section;
}

function renderRollupItem(rollup: ActivityNetworkFlowRollup): HTMLElement {
  const item = document.createElement(PortalDom.Tags.ListItem);
  item.append(renderItemHeading(decodeDisplayText(rollup.label)));

  const metadata = document.createElement(PortalDom.Tags.DefinitionList);
  appendDetail(metadata, PortalDetails.Connections, detailFromValue(rollup.connectionCount));
  appendDetail(metadata, PortalDetails.BytesSent, detailFromValue(rollup.bytesSent));
  appendDetail(metadata, PortalDetails.BytesReceived, detailFromValue(rollup.bytesReceived));
  item.append(metadata);
  return item;
}

function renderIndicatorItem(indicator: ActivityNetworkFlowIndicator): HTMLElement {
  const item = document.createElement(PortalDom.Tags.ListItem);
  item.append(renderItemHeading(decodeDisplayText(indicator.label)));

  const metadata = document.createElement(PortalDom.Tags.DefinitionList);
  appendDetail(metadata, PortalDetails.ObservedAt, detailFromValue(indicator.observedAt));
  item.append(metadata);
  return item;
}

function renderItemHeading(titleText: PortalDisplayText): HTMLElement {
  const heading = document.createElement(PortalDom.Tags.Strong);
  heading.textContent = titleText;
  return heading;
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

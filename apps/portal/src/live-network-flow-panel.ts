import type {
  ActivityNetworkEndpoint,
  ActivityNetworkFlowObservation,
  ActivityNetworkFlowReadModel,
} from '@ocentra-parent/activity-domain/network-flow';
import { AgentProtocolDefaults } from '@ocentra-parent/agent-protocol-domain/contracts';
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
import type { PortalLiveActivityState } from './live-activity-state';

export function renderNetworkFlow(container: HTMLElement, liveActivity: PortalLiveActivityState): void {
  const panel = panelWithTitle(PortalText.Resolve(PortalTextToken.NetworkFlow));
  const metadata = document.createElement(PortalDom.Tags.DefinitionList);

  appendDetail(metadata, PortalDetails.Status, eventStatus(liveActivity));
  if (liveActivity.networkFlowReadModel === null) {
    appendDetail(metadata, PortalDetails.Reason, eventReason(liveActivity));
    panel.append(metadata, emptyMessage(PortalText.Resolve(PortalTextToken.NoNetworkFlow)));
    container.append(panel);
    return;
  }

  appendNetworkFlowDetails(metadata, liveActivity.networkFlowReadModel);
  panel.append(metadata);
  appendEmptyNetworkFlow(panel, liveActivity.networkFlowReadModel);
  container.append(panel);
}

function appendNetworkFlowDetails(metadata: HTMLDListElement, readModel: ActivityNetworkFlowReadModel): void {
  const row = readModel.rows[0] ?? null;
  appendDetail(metadata, PortalDetails.RowsReturned, detailFromValue(readModel.returned));
  appendDetail(metadata, PortalDetails.LastObserved, detailFromValue(row?.observedAt));
  appendDetail(metadata, PortalDetails.EventId, detailFromValue(row?.eventId));
  appendDetail(metadata, PortalDetails.Capability, detailFromValue(readModel.capabilityStatus));
  appendDetail(metadata, PortalDetails.Custody, detailFromValue(readModel.custody));
  appendNetworkEndpointDetails(metadata, row);
  appendNetworkProcessDetails(metadata, row);
  appendNetworkCounterDetails(metadata, row);
}

function appendNetworkEndpointDetails(metadata: HTMLDListElement, row: ActivityNetworkFlowObservation | null): void {
  appendDetail(metadata, PortalDetails.Domain, detailFromValue(row?.destinationDomain));
  appendDetail(metadata, PortalDetails.Destination, endpointDetail(row?.destinationEndpoint));
  appendDetail(metadata, PortalDetails.Source, endpointDetail(row?.localEndpoint));
  appendDetail(metadata, PortalDetails.NetworkProtocol, detailFromValue(row?.protocol));
  appendDetail(metadata, PortalDetails.TcpState, detailFromValue(row?.tcpState));
  appendDetail(metadata, PortalDetails.DomainAttribution, detailFromValue(row?.domainAttributionStatus));
}

function appendNetworkProcessDetails(metadata: HTMLDListElement, row: ActivityNetworkFlowObservation | null): void {
  appendDetail(metadata, PortalDetails.Process, detailFromValue(row?.processName));
  appendDetail(metadata, PortalDetails.ProcessId, detailFromValue(row?.processId));
  appendDetail(metadata, PortalDetails.ProcessAttribution, detailFromValue(row?.processAttributionStatus));
}

function appendNetworkCounterDetails(metadata: HTMLDListElement, row: ActivityNetworkFlowObservation | null): void {
  appendDetail(metadata, PortalDetails.Connections, detailFromValue(row?.counters.connectionCount));
  appendDetail(metadata, PortalDetails.BytesSent, detailFromValue(row?.counters.bytesSent));
  appendDetail(metadata, PortalDetails.BytesReceived, detailFromValue(row?.counters.bytesReceived));
}

function appendEmptyNetworkFlow(panel: HTMLElement, readModel: ActivityNetworkFlowReadModel): void {
  if (readModel.returned === 0) {
    panel.append(emptyMessage(PortalText.Resolve(PortalTextToken.NoNetworkFlow)));
  }
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

function eventStatus(liveActivity: PortalLiveActivityState): PortalDetailValue {
  if (liveActivity.networkFlowEvent === null) {
    return notReported();
  }
  return decodePortalDetailValue(liveActivity.networkFlowEvent.severity);
}

function eventReason(liveActivity: PortalLiveActivityState): PortalDetailValue {
  if (liveActivity.networkFlowEvent === null) {
    return notReported();
  }
  return detailFromValue(liveActivity.networkFlowEvent.payload[AgentProtocolDefaults.Field.Reason]);
}

function detailFromValue(value: LogFieldValue | undefined): PortalDetailValue {
  if (value === undefined || value === null) {
    return notReported();
  }
  return decodePortalDetailValue(String(value));
}

function endpointDetail(endpoint: ActivityNetworkEndpoint | null | undefined): PortalDetailValue {
  if (endpoint === null || endpoint === undefined || endpoint.ip === null) {
    return notReported();
  }
  if (endpoint.port === null) {
    return decodePortalDetailValue(endpoint.ip);
  }
  return decodePortalDetailValue([endpoint.ip, String(endpoint.port)].join(PortalFormatting.EndpointSeparator));
}

function notReported(): PortalDetailValue {
  return decodePortalDetailValue(PortalText.Resolve(PortalTextToken.NotReported));
}

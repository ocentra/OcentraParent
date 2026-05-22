import { AgentProtocolDefaults } from '@ocentra-parent/agent-protocol-domain/contracts';
import type { LogFieldValue } from '@ocentra-parent/logging-domain/contracts';
import {
  PortalDetails,
  PortalDom,
  PortalFormatting,
  PortalText,
  PortalTextToken,
  decodePortalDetailValue,
  type PortalActivityMemoryGraphEdge,
  type PortalActivityMemoryGraphNode,
  type PortalActivityMemoryGraphNodeId,
  type PortalDetailValue,
  type PortalDisplayText,
} from '@ocentra-parent/portal-domain/contracts';
import { appendDetail } from './detail-list';
import type { PortalActivityMemoryGraphReadModel } from '@ocentra-parent/portal-domain/contracts';
import type { PortalLiveActivityState } from './live-activity-state';

export function renderActivityMemoryGraph(container: HTMLElement, liveActivity: PortalLiveActivityState): void {
  const panel = panelWithTitle(PortalText.Resolve(PortalTextToken.ActivityMemoryGraph));
  const metadata = document.createElement(PortalDom.Tags.DefinitionList);

  appendDetail(metadata, PortalDetails.Status, eventStatus(liveActivity));
  if (liveActivity.activityMemoryGraphReadModel === null) {
    appendDetail(metadata, PortalDetails.Reason, eventReason(liveActivity));
    panel.append(metadata, emptyMessage(PortalText.Resolve(PortalTextToken.NoActivityMemoryGraph)));
    container.append(panel);
    return;
  }

  appendActivityMemoryGraphDetails(metadata, liveActivity.activityMemoryGraphReadModel);
  panel.append(metadata);
  appendActivityMemoryGraphNavigation(panel, liveActivity.activityMemoryGraphReadModel);
  appendEmptyGraphMessage(panel, liveActivity.activityMemoryGraphReadModel);
  container.append(panel);
}

function appendActivityMemoryGraphDetails(
  metadata: HTMLDListElement,
  readModel: PortalActivityMemoryGraphReadModel
): void {
  appendDetail(metadata, PortalDetails.GraphEdges, detailFromValue(readModel.returnedEdgeCount));
  appendDetail(metadata, PortalDetails.GraphNodes, detailFromValue(readModel.returnedNodeCount));
  appendDetail(metadata, PortalDetails.GraphOmittedEdges, detailFromValue(readModel.omittedEdgeCount));
  appendDetail(metadata, PortalDetails.Capability, detailFromValue(readModel.capabilityStatus));
  appendDetail(metadata, PortalDetails.Custody, detailFromValue(readModel.custody));
  appendDetail(metadata, PortalDetails.GeneratedAt, detailFromValue(readModel.generatedAt));
}

function appendActivityMemoryGraphNavigation(panel: HTMLElement, readModel: PortalActivityMemoryGraphReadModel): void {
  if (readModel.edges.length === 0) {
    return;
  }
  const list = document.createElement(PortalDom.Tags.OrderedList);
  for (const edge of readModel.edges) {
    const item = document.createElement(PortalDom.Tags.ListItem);
    const details = document.createElement(PortalDom.Tags.Details);
    const summary = document.createElement(PortalDom.Tags.SummaryTag);
    summary.textContent = edgeSummary(edge, readModel.nodes);
    details.append(summary, edgeDetails(edge));
    item.append(details);
    list.append(item);
  }
  panel.append(sectionLabel(PortalDetails.GraphNavigation), list);
}

function edgeDetails(edge: PortalActivityMemoryGraphEdge): HTMLDListElement {
  const metadata = document.createElement(PortalDom.Tags.DefinitionList);
  appendDetail(metadata, PortalDetails.EventId, decodePortalDetailValue(edge.edgeId));
  appendDetail(metadata, PortalDetails.FirstObserved, decodePortalDetailValue(edge.observedFrom));
  appendDetail(metadata, PortalDetails.LastObserved, detailFromValue(edge.observedUntil));
  appendDetail(
    metadata,
    PortalDetails.EvidenceReferences,
    decodePortalDetailValue(evidenceLabels(edge).join(PortalFormatting.EventDetailSeparator))
  );
  appendDetail(metadata, PortalDetails.DegradedState, decodePortalDetailValue(edge.trace.entryStatus));
  return metadata;
}

function edgeSummary(
  edge: PortalActivityMemoryGraphEdge,
  nodes: readonly PortalActivityMemoryGraphNode[]
): PortalDetailValue {
  return decodePortalDetailValue(
    [edge.edgeKind, nodeLabel(edge.fromNodeId, nodes), nodeLabel(edge.toNodeId, nodes)].join(
      PortalFormatting.GraphEdgeSeparator
    )
  );
}

function evidenceLabels(edge: PortalActivityMemoryGraphEdge) {
  return edge.trace.sourceEvidenceReferences.map((reference) => reference.evidenceReferenceId);
}

function nodeLabel(
  nodeId: PortalActivityMemoryGraphNodeId,
  nodes: readonly PortalActivityMemoryGraphNode[]
): PortalDetailValue {
  return decodePortalDetailValue(nodes.find((node) => node.nodeId === nodeId)?.label ?? nodeId);
}

function appendEmptyGraphMessage(panel: HTMLElement, readModel: PortalActivityMemoryGraphReadModel): void {
  if (readModel.returnedEdgeCount === 0) {
    panel.append(emptyMessage(PortalText.Resolve(PortalTextToken.NoActivityMemoryGraph)));
  }
}

function sectionLabel(label: PortalDisplayText): HTMLElement {
  const labelElement = document.createElement(PortalDom.Tags.Strong);
  labelElement.textContent = label;
  return labelElement;
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
  if (liveActivity.activityMemoryGraphEvent === null) {
    return notReported();
  }
  return decodePortalDetailValue(liveActivity.activityMemoryGraphEvent.severity);
}

function eventReason(liveActivity: PortalLiveActivityState): PortalDetailValue {
  if (liveActivity.activityMemoryGraphEvent === null) {
    return notReported();
  }
  return detailFromValue(liveActivity.activityMemoryGraphEvent.payload[AgentProtocolDefaults.Field.Reason]);
}

function detailFromValue(value: LogFieldValue | null | undefined): PortalDetailValue {
  if (value === undefined || value === null) {
    return notReported();
  }
  return decodePortalDetailValue(String(value));
}

function notReported(): PortalDetailValue {
  return decodePortalDetailValue(PortalText.Resolve(PortalTextToken.NotReported));
}

import type { AgentEventEnvelope } from '@ocentra-parent/agent-protocol-domain/contracts';
import {
  PortalDetails,
  PortalDiagnostics,
  PortalDom,
  PortalFormatting,
  PortalText,
  PortalTextToken,
  decodePortalDetailValue,
} from '@ocentra-parent/portal-domain/contracts';
import { appendDetail } from './detail-list';

export function renderActivityTimeline(container: HTMLElement, events: readonly AgentEventEnvelope[]): void {
  const panel = document.createElement(PortalDom.Tags.Section);
  panel.className = PortalDom.Classes.Summary;

  const title = document.createElement(PortalDom.Tags.HeadingTwo);
  title.textContent = PortalText.Resolve(PortalTextToken.ActivityTimeline);
  panel.append(title);

  if (events.length === 0) {
    panel.append(emptyMessage());
    container.append(panel);
    return;
  }

  const list = document.createElement(PortalDom.Tags.OrderedList);
  list.className = PortalDom.Classes.LogList;
  for (const event of events.slice(0, PortalDiagnostics.TimelineLimit)) {
    list.append(renderTimelineEvent(event));
  }

  panel.append(list);
  container.append(panel);
}

function renderTimelineEvent(event: AgentEventEnvelope): HTMLLIElement {
  const item = document.createElement(PortalDom.Tags.ListItem);
  item.className = [PortalDom.Classes.Log, `${PortalDom.Classes.LogLevelPrefix}${event.severity}`].join(
    PortalDom.Classes.ClassNameSeparator
  );

  const message = document.createElement(PortalDom.Tags.Strong);
  message.textContent = event.event;

  const detail = document.createElement(PortalDom.Tags.Span);
  detail.textContent = [
    event.sentAt,
    event.eventId,
    `${PortalFormatting.CorrelationPrefix}${event.correlationId}`,
  ].join(PortalFormatting.EventDetailSeparator);

  const metadata = document.createElement(PortalDom.Tags.DefinitionList);
  appendDetail(metadata, PortalDetails.EventId, decodePortalDetailValue(event.eventId));
  appendDetail(metadata, PortalDetails.SentAt, decodePortalDetailValue(event.sentAt));
  appendDetail(metadata, PortalDetails.Severity, decodePortalDetailValue(event.severity));
  appendDetail(metadata, PortalDetails.Observer, decodePortalDetailValue(event.source.peerId));

  item.append(message, detail, metadata);
  return item;
}

function emptyMessage(): HTMLElement {
  const message = document.createElement(PortalDom.Tags.Paragraph);
  message.className = PortalDom.Classes.CommandResultEmpty;
  message.textContent = PortalText.Resolve(PortalTextToken.NoEvents);
  return message;
}

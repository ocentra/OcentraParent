import type { AgentEventEnvelope } from '@ocentra-parent/agent-protocol-domain/contracts';
import { PortalDom, PortalFormatting, PortalText, PortalTextToken } from '@ocentra-parent/portal-domain/contracts';

export function renderEvents(container: HTMLElement, events: readonly AgentEventEnvelope[]): void {
  const title = document.createElement(PortalDom.Tags.HeadingTwo);
  title.textContent = PortalText.Resolve(PortalTextToken.AgentEvents);
  container.append(title);

  const list = document.createElement(PortalDom.Tags.OrderedList);
  list.className = PortalDom.Classes.LogList;

  for (const event of events) {
    list.append(renderEvent(event));
  }

  container.append(list);
}

function renderEvent(event: AgentEventEnvelope): HTMLLIElement {
  const item = document.createElement(PortalDom.Tags.ListItem);
  item.className = [PortalDom.Classes.Log, `${PortalDom.Classes.LogLevelPrefix}${event.severity}`].join(
    PortalDom.Classes.ClassNameSeparator
  );

  const message = document.createElement(PortalDom.Tags.Strong);
  message.textContent = event.event;

  const detail = document.createElement(PortalDom.Tags.Span);
  detail.textContent = [
    event.sentAt,
    event.source.peerId,
    `${PortalFormatting.CorrelationPrefix}${event.correlationId}`,
  ].join(PortalFormatting.EventDetailSeparator);

  const fields = document.createElement(PortalDom.Tags.Code);
  fields.textContent = JSON.stringify(event.payload, null, 2);

  item.append(message, detail, fields);
  return item;
}

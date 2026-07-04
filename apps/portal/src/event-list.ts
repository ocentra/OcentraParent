import { PortalDevTextToken, resolvePortalDevText } from './portal-dev-text';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { PortalFormatting } from '@ocentra-parent/portal-domain/formatting';
import type { ParentRouteEventSnapshot } from '../generated/parent-ui-bridge';

export function renderEvents(container: HTMLElement, events: readonly ParentRouteEventSnapshot[]): void {
  const title = document.createElement(PortalDom.Tags.HeadingTwo);
  title.textContent = resolvePortalDevText(PortalDevTextToken.AgentEvents);
  container.append(title);

  const list = document.createElement(PortalDom.Tags.OrderedList);
  list.className = PortalDom.Classes.LogList;

  for (const event of events) {
    list.append(renderEvent(event));
  }

  container.append(list);
}

function renderEvent(event: ParentRouteEventSnapshot): HTMLLIElement {
  const item = document.createElement(PortalDom.Tags.ListItem);
  item.className = [
    PortalDom.Classes.Log,
    `${PortalDom.Classes.LogLevelPrefix}${event.severity ?? 'info'}`,
  ].join(PortalDom.Classes.ClassNameSeparator);

  const message = document.createElement(PortalDom.Tags.Strong);
  message.textContent = event.event ?? 'unknown-event';

  const detail = document.createElement(PortalDom.Tags.Span);
  detail.textContent = [
    event.sentAt ?? 'not-reported',
    event.sourcePeerId ?? 'not-reported',
    `${PortalFormatting.CorrelationPrefix}${event.correlationId ?? event.eventId ?? 'not-reported'}`,
  ].join(PortalFormatting.EventDetailSeparator);

  const fields = document.createElement(PortalDom.Tags.Code);
  fields.textContent = JSON.stringify(event.payload ?? {}, null, 2);

  item.append(message, detail, fields);
  return item;
}

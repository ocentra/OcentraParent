import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/portal-domain/display-text';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { PortalFormatting } from '@ocentra-parent/portal-domain/formatting';
import { ParentHostBridgeRuntime, type ParentRouteEventSnapshot } from '../generated/parent-ui-bridge';

export function renderEvents(container: HTMLElement, events: readonly ParentRouteEventSnapshot[]): void {
  const panel = document.createElement(PortalDom.Tags.Section);
  panel.className = PortalDom.Classes.Summary;

  const title = document.createElement(PortalDom.Tags.HeadingTwo);
  title.textContent = resolvePortalDevText(PortalDevTextToken.AgentEvents);
  panel.append(title);

  if (events.length === 0) {
    const message = document.createElement(PortalDom.Tags.Paragraph);
    message.className = PortalDom.Classes.CommandResultEmpty;
    message.textContent = resolvePortalDevText(PortalDevTextToken.EmptyAgentEvents);
    panel.append(message);
    container.append(panel);
    return;
  }

  const list = document.createElement(PortalDom.Tags.OrderedList);
  list.className = PortalDom.Classes.LogList;

  for (const event of events) {
    list.append(renderEvent(event));
  }

  panel.append(list);
  container.append(panel);
}

function renderEvent(event: ParentRouteEventSnapshot): HTMLLIElement {
  const item = document.createElement(PortalDom.Tags.ListItem);
  item.className = [PortalDom.Classes.Log, `${PortalDom.Classes.LogLevelPrefix}${eventSeverity(event)}`].join(
    PortalDom.Classes.ClassNameSeparator
  );

  const message = document.createElement(PortalDom.Tags.Strong);
  message.textContent = resolvedEventName(event);

  const detail = document.createElement(PortalDom.Tags.Span);
  detail.textContent = [
    event.sentAt ?? resolvePortalDevText(PortalDevTextToken.NotReported),
    event.sourcePeerId ?? resolvePortalDevText(PortalDevTextToken.NotReported),
    `${PortalFormatting.CorrelationPrefix}${event.correlationId ?? event.eventId ?? resolvePortalDevText(PortalDevTextToken.NotReported)}`,
  ].join(PortalFormatting.EventDetailSeparator);

  const fields = document.createElement(PortalDom.Tags.Code);
  fields.textContent = JSON.stringify(event.payload ?? {}, null, 2);

  item.append(message, detail, fields);
  return item;
}

function eventSeverity(event: ParentRouteEventSnapshot) {
  return event.severity ?? ParentHostBridgeRuntime.InfoSeverity;
}

function resolvedEventName(event: ParentRouteEventSnapshot) {
  return event.event ?? resolvePortalDevText(PortalDevTextToken.UnknownEvent);
}

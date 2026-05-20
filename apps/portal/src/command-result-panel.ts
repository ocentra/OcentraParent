import type { AgentEventEnvelope } from '@ocentra-parent/agent-protocol-domain/contracts';
import { PortalDom, PortalFormatting, PortalText, PortalTextToken } from '@ocentra-parent/portal-domain/contracts';
import { latestCommandResult } from './event-results';
import type { PortalRuntimeState } from './portal-state';

export function renderCommandResultPanel(container: HTMLElement, state: PortalRuntimeState): void {
  const panel = document.createElement(PortalDom.Tags.Division);
  panel.className = PortalDom.Classes.CommandResultPanel;

  const title = document.createElement(PortalDom.Tags.HeadingTwo);
  title.textContent = PortalText.Resolve(PortalTextToken.CommandResult);

  panel.append(title, renderSelectedResult(state));
  container.append(panel);
}

function renderSelectedResult(state: PortalRuntimeState): HTMLElement {
  const event = latestCommandResult(state.events, state.selectedCommandResultEvent);
  const panel = document.createElement(PortalDom.Tags.Division);

  if (event === null) {
    const empty = document.createElement(PortalDom.Tags.Paragraph);
    empty.className = PortalDom.Classes.CommandResultEmpty;
    empty.textContent = PortalText.Resolve(PortalTextToken.NoCommandResult);
    panel.append(empty);
    return panel;
  }

  panel.append(renderResultEvent(event));
  return panel;
}

function renderResultEvent(event: AgentEventEnvelope): HTMLElement {
  const card = document.createElement(PortalDom.Tags.Division);
  card.className = [PortalDom.Classes.Log, `${PortalDom.Classes.LogLevelPrefix}${event.severity}`].join(
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

  card.append(message, detail, fields);
  return card;
}

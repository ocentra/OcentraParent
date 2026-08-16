import {
  GeneratedDevLogField as DevLogField,
  GeneratedDevLogMessage as DevLogMessage,
} from '@ocentra-parent/logging-domain/generated/logging-contracts';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/portal-domain/display-text';
import { PortalDom, PortalTiming } from '@ocentra-parent/portal-domain/contracts';
import { PortalFormatting } from '@ocentra-parent/portal-domain/formatting';
import {
  decodeParentPortalClipboardText,
  ParentHostBridgeRuntime,
  type ParentCommandResultDetailSnapshot,
  type ParentRouteEventSnapshot,
} from '../generated/parent-ui-bridge';
import { writeClipboardText } from './clipboard';
import { writePortalDevLog } from './dev-logger';
import { latestParentRouteEventSnapshot } from './parent-route-event-snapshot';
import type { PortalRuntimeState } from './portal-state';

export function renderCommandResultPanel(container: HTMLElement, state: PortalRuntimeState): void {
  const panel = document.createElement(PortalDom.Tags.Division);
  panel.className = PortalDom.Classes.CommandResultPanel;

  const title = document.createElement(PortalDom.Tags.HeadingTwo);
  title.textContent = resolvePortalDevText(PortalDevTextToken.CommandResult);

  panel.append(title, renderSelectedResult(state));
  container.append(panel);
}

function renderSelectedResult(state: PortalRuntimeState): HTMLElement {
  const event = latestParentRouteEventSnapshot(state.events, state.selectedCommandResultEvent);
  const panel = document.createElement(PortalDom.Tags.Division);

  if (event === null) {
    const empty = document.createElement(PortalDom.Tags.Paragraph);
    empty.className = PortalDom.Classes.CommandResultEmpty;
    empty.textContent = resolvePortalDevText(PortalDevTextToken.NoCommandResult);
    panel.append(empty);
    return panel;
  }

  panel.append(renderResultEvent(event));
  return panel;
}

function renderResultEvent(event: ParentRouteEventSnapshot): HTMLElement {
  const card = document.createElement(PortalDom.Tags.Division);
  card.className = [PortalDom.Classes.Log, `${PortalDom.Classes.LogLevelPrefix}${eventSeverity(event)}`].join(
    PortalDom.Classes.ClassNameSeparator
  );

  const header = document.createElement(PortalDom.Tags.Division);
  header.className = PortalDom.Classes.CommandResultHeader;

  const message = document.createElement(PortalDom.Tags.Strong);
  message.textContent = resolvedEventName(event);

  const copyButton = document.createElement(PortalDom.Tags.Button);
  copyButton.type = PortalDom.ButtonType.Button;
  copyButton.className = PortalDom.Classes.CopyResultButton;
  copyButton.textContent = resolvePortalDevText(PortalDevTextToken.CopyResult);
  copyButton.addEventListener(PortalDom.Events.Click, () => {
    void copyResultEvent(copyButton, event);
  });

  const detail = document.createElement(PortalDom.Tags.Span);
  detail.textContent = [
    event.sentAt ?? resolvePortalDevText(PortalDevTextToken.NotReported),
    event.sourcePeerId ?? resolvePortalDevText(PortalDevTextToken.NotReported),
    `${PortalFormatting.CorrelationPrefix}${event.correlationId ?? event.eventId ?? resolvePortalDevText(PortalDevTextToken.NotReported)}`,
  ].join(PortalFormatting.EventDetailSeparator);

  const fields = document.createElement(PortalDom.Tags.Code);
  fields.textContent = JSON.stringify(event.payload ?? {}, null, 2);

  header.append(message, copyButton);
  const resultSummary = renderAppGameTimerParentPreferenceSetupCommandResult(event);

  card.append(header, detail);
  if (resultSummary !== null) {
    card.append(resultSummary);
  }
  card.append(fields);
  return card;
}

function renderAppGameTimerParentPreferenceSetupCommandResult(event: ParentRouteEventSnapshot): HTMLElement | null {
  const projection = event.commandResultProjection;
  if (projection === undefined || projection === null) {
    return null;
  }

  return renderDetailList(projection.details);
}

function renderDetailList(details: readonly ParentCommandResultDetailSnapshot[]): HTMLElement {
  const list = document.createElement(PortalDom.Tags.DefinitionList);
  for (const item of details) {
    const term = document.createElement(PortalDom.Tags.DefinitionTerm);
    term.textContent = item.label;

    const description = document.createElement(PortalDom.Tags.DefinitionDescription);
    description.textContent = item.value;

    list.append(term, description);
  }
  return list;
}

async function copyResultEvent(button: HTMLButtonElement, event: ParentRouteEventSnapshot): Promise<void> {
  button.disabled = true;
  try {
    const didCopy = await writeClipboardText(decodeParentPortalClipboardText(JSON.stringify(event, null, 2)));
    if (!didCopy) {
      button.textContent = resolvePortalDevText(PortalDevTextToken.CopyResultFailed);
      return;
    }
    writePortalDevLog(DevLogMessage.PortalResultCopied, {
      [DevLogField.Event]: resolvedEventName(event),
    });
    button.textContent = resolvePortalDevText(PortalDevTextToken.CopiedResult);
  } catch {
    button.textContent = resolvePortalDevText(PortalDevTextToken.CopyResultFailed);
  } finally {
    button.disabled = false;
    window.setTimeout(() => {
      button.textContent = resolvePortalDevText(PortalDevTextToken.CopyResult);
    }, PortalTiming.CopyFeedbackMs);
  }
}

function eventSeverity(event: ParentRouteEventSnapshot) {
  return event.severity ?? ParentHostBridgeRuntime.InfoSeverity;
}

function resolvedEventName(event: ParentRouteEventSnapshot) {
  return event.event ?? resolvePortalDevText(PortalDevTextToken.UnknownEvent);
}

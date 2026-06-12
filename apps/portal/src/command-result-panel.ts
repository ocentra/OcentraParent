import { AgentEvent, type AgentEventEnvelope } from '@ocentra-parent/agent-protocol-domain/contracts';
import {
  PortalDom,
  PortalFormatting,
  PortalText,
  PortalTextToken,
  PortalTiming,
  createAppGameTimerParentPreferenceSetupCommandResultDetails,
  decodePortalClipboardText,
  latestCommandResult,
  type AppGameTimerParentSurfacePanelDetail,
} from '@ocentra-parent/portal-domain/contracts';
import { writeClipboardText } from './clipboard';
import { DevLogField, DevLogMessage, writePortalDevLog } from './dev-logger';
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

  const header = document.createElement(PortalDom.Tags.Division);
  header.className = PortalDom.Classes.CommandResultHeader;

  const message = document.createElement(PortalDom.Tags.Strong);
  message.textContent = event.event;

  const copyButton = document.createElement(PortalDom.Tags.Button);
  copyButton.type = PortalDom.ButtonType.Button;
  copyButton.className = PortalDom.Classes.CopyResultButton;
  copyButton.textContent = PortalText.Resolve(PortalTextToken.CopyResult);
  copyButton.addEventListener(PortalDom.Events.Click, () => {
    void copyResultEvent(copyButton, event);
  });

  const detail = document.createElement(PortalDom.Tags.Span);
  detail.textContent = [
    event.sentAt,
    event.source.peerId,
    `${PortalFormatting.CorrelationPrefix}${event.correlationId}`,
  ].join(PortalFormatting.EventDetailSeparator);

  const fields = document.createElement(PortalDom.Tags.Code);
  fields.textContent = JSON.stringify(event.payload, null, 2);

  header.append(message, copyButton);
  const resultSummary = renderAppGameTimerParentPreferenceSetupCommandResult(event);

  card.append(header, detail);
  if (resultSummary !== null) {
    card.append(resultSummary);
  }
  card.append(fields);
  return card;
}

function renderAppGameTimerParentPreferenceSetupCommandResult(event: AgentEventEnvelope): HTMLElement | null {
  if (event.event !== AgentEvent.ActivityAppGameTimerParentPreferenceSetupRequested) {
    return null;
  }

  return renderDetailList(createAppGameTimerParentPreferenceSetupCommandResultDetails(event));
}

function renderDetailList(details: readonly AppGameTimerParentSurfacePanelDetail[]): HTMLElement {
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

async function copyResultEvent(button: HTMLButtonElement, event: AgentEventEnvelope): Promise<void> {
  button.disabled = true;
  try {
    const didCopy = await writeClipboardText(decodePortalClipboardText(JSON.stringify(event, null, 2)));
    if (!didCopy) {
      button.textContent = PortalText.Resolve(PortalTextToken.CopyResultFailed);
      return;
    }
    writePortalDevLog(DevLogMessage.PortalResultCopied, {
      [DevLogField.Event]: event.event,
    });
    button.textContent = PortalText.Resolve(PortalTextToken.CopiedResult);
  } catch {
    button.textContent = PortalText.Resolve(PortalTextToken.CopyResultFailed);
  } finally {
    button.disabled = false;
    window.setTimeout(() => {
      button.textContent = PortalText.Resolve(PortalTextToken.CopyResult);
    }, PortalTiming.CopyFeedbackMs);
  }
}

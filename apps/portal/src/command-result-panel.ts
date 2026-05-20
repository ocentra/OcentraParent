import type { AgentEventEnvelope } from '@ocentra-parent/agent-protocol-domain/contracts';
import {
  PortalClipboard,
  type PortalClipboardText,
  PortalDom,
  PortalFormatting,
  PortalText,
  PortalTextToken,
  PortalTiming,
  decodePortalClipboardText,
} from '@ocentra-parent/portal-domain/contracts';
import { DevLogField, DevLogMessage, writePortalDevLog } from './dev-logger';
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
  card.append(header, detail, fields);
  return card;
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

async function writeClipboardText(text: PortalClipboardText): Promise<boolean> {
  if (navigator.clipboard !== undefined) {
    try {
      await navigator.clipboard.writeText(text);
      return true;
    } catch {
      return writeClipboardTextWithSelection(text);
    }
  }
  return writeClipboardTextWithSelection(text);
}

function writeClipboardTextWithSelection(text: PortalClipboardText): boolean {
  const buffer = document.createElement(PortalDom.Tags.TextArea);
  buffer.className = PortalDom.Classes.ClipboardBuffer;
  buffer.setAttribute(PortalDom.Attributes.ReadOnly, PortalDom.Attributes.ReadOnly);
  buffer.value = text;
  document.body.append(buffer);
  buffer.focus();
  buffer.select();
  const copied = document.execCommand(PortalClipboard.CommandCopy);
  buffer.remove();
  return copied;
}

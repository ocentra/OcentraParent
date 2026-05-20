import {
  PortalDetails,
  PortalDom,
  PortalText,
  PortalTextToken,
  PortalTiming,
  decodePortalDetailValue,
} from '@ocentra-parent/portal-domain/contracts';
import { writeClipboardText } from './clipboard';
import { appendDetail } from './detail-list';
import { buildDiagnosticsExport } from './diagnostics-export';
import { DevLogField, DevLogMessage, writePortalDevLog } from './dev-logger';
import type { PortalRuntimeState } from './portal-state';

export function renderDiagnosticsPanel(container: HTMLElement, state: PortalRuntimeState): void {
  const panel = document.createElement(PortalDom.Tags.Section);
  panel.className = PortalDom.Classes.Summary;

  const title = document.createElement(PortalDom.Tags.HeadingTwo);
  title.textContent = PortalText.Resolve(PortalTextToken.DeviceDiagnostics);

  const copyButton = document.createElement(PortalDom.Tags.Button);
  copyButton.type = PortalDom.ButtonType.Button;
  copyButton.className = PortalDom.Classes.CopyResultButton;
  copyButton.textContent = PortalText.Resolve(PortalTextToken.CopyDiagnostics);
  copyButton.addEventListener(PortalDom.Events.Click, () => {
    void copyDiagnostics(copyButton, state);
  });

  const metadata = document.createElement(PortalDom.Tags.DefinitionList);
  const latestEvent = state.events[0] ?? null;
  appendDetail(metadata, PortalDetails.AgentUrl, decodePortalDetailValue(state.agentWsUrl));
  appendDetail(metadata, PortalDetails.State, decodePortalDetailValue(state.connectionState));
  appendDetail(metadata, PortalDetails.Events, decodePortalDetailValue(String(state.events.length)));
  appendDetail(metadata, PortalDetails.LastEvent, detailFromValue(latestEvent?.event));
  appendDetail(metadata, PortalDetails.EventId, detailFromValue(latestEvent?.eventId));

  panel.append(title, copyButton, metadata);
  container.append(panel);
}

async function copyDiagnostics(button: HTMLButtonElement, state: PortalRuntimeState): Promise<void> {
  button.disabled = true;
  try {
    const didCopy = await writeClipboardText(buildDiagnosticsExport(state));
    button.textContent = PortalText.Resolve(
      didCopy ? PortalTextToken.CopiedDiagnostics : PortalTextToken.CopyDiagnosticsFailed
    );
    if (didCopy) {
      writePortalDevLog(DevLogMessage.PortalResultCopied, {
        [DevLogField.EventsBuffered]: state.events.length,
      });
    }
  } catch {
    button.textContent = PortalText.Resolve(PortalTextToken.CopyDiagnosticsFailed);
  } finally {
    button.disabled = false;
    window.setTimeout(() => {
      button.textContent = PortalText.Resolve(PortalTextToken.CopyDiagnostics);
    }, PortalTiming.CopyFeedbackMs);
  }
}

function detailFromValue(value: unknown) {
  if (value === undefined || value === null) {
    return decodePortalDetailValue(PortalText.Resolve(PortalTextToken.NotReported));
  }
  return decodePortalDetailValue(String(value));
}

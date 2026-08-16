import {
  GeneratedDevLogField as DevLogField,
  GeneratedDevLogMessage as DevLogMessage,
} from '@ocentra-parent/logging-domain/generated/logging-contracts';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/portal-domain/display-text';
import { PortalDom, PortalTiming } from '@ocentra-parent/portal-domain/contracts';
import { PortalDetails } from '@ocentra-parent/portal-domain/details';
import { decodeParentPortalDetailValue } from '../generated/parent-ui-bridge';
import { writeClipboardText } from './clipboard';
import { appendDetail } from './detail-list';
import { buildDiagnosticsExport } from './diagnostics-export';
import { writePortalDevLog } from './dev-logger';
import type { PortalRuntimeState } from './portal-state';

export function renderDiagnosticsPanel(container: HTMLElement, state: PortalRuntimeState): void {
  const panel = document.createElement(PortalDom.Tags.Section);
  panel.className = PortalDom.Classes.Summary;

  const title = document.createElement(PortalDom.Tags.HeadingTwo);
  title.textContent = resolvePortalDevText(PortalDevTextToken.DeviceDiagnostics);

  const copyButton = document.createElement(PortalDom.Tags.Button);
  copyButton.type = PortalDom.ButtonType.Button;
  copyButton.className = PortalDom.Classes.CopyResultButton;
  copyButton.textContent = resolvePortalDevText(PortalDevTextToken.CopyDiagnostics);
  copyButton.addEventListener(PortalDom.Events.Click, () => {
    void copyDiagnostics(copyButton, state);
  });

  const metadata = document.createElement(PortalDom.Tags.DefinitionList);
  const latestEvent = state.events[0] ?? null;
  appendDetail(metadata, PortalDetails.AgentUrl, decodeParentPortalDetailValue(state.agentEndpoint));
  appendDetail(metadata, PortalDetails.State, decodeParentPortalDetailValue(state.connectionState));
  appendDetail(metadata, PortalDetails.Events, decodeParentPortalDetailValue(String(state.events.length)));
  appendDetail(metadata, PortalDetails.LastEvent, detailFromValue(latestEvent?.event));
  appendDetail(metadata, PortalDetails.EventId, detailFromValue(latestEvent?.eventId));

  panel.append(title, copyButton, metadata);
  container.append(panel);
}

async function copyDiagnostics(button: HTMLButtonElement, state: PortalRuntimeState): Promise<void> {
  button.disabled = true;
  try {
    const didCopy = await writeClipboardText(buildDiagnosticsExport(state));
    button.textContent = resolvePortalDevText(
      didCopy ? PortalDevTextToken.CopiedDiagnostics : PortalDevTextToken.CopyDiagnosticsFailed
    );
    if (didCopy) {
      writePortalDevLog(DevLogMessage.PortalResultCopied, {
        [DevLogField.EventsBuffered]: state.events.length,
      });
    }
  } catch {
    button.textContent = resolvePortalDevText(PortalDevTextToken.CopyDiagnosticsFailed);
  } finally {
    button.disabled = false;
    window.setTimeout(() => {
      button.textContent = resolvePortalDevText(PortalDevTextToken.CopyDiagnostics);
    }, PortalTiming.CopyFeedbackMs);
  }
}

function detailFromValue(value: unknown) {
  if (value === undefined || value === null) {
    return decodeParentPortalDetailValue(resolvePortalDevText(PortalDevTextToken.NotReported));
  }
  return decodeParentPortalDetailValue(String(value));
}

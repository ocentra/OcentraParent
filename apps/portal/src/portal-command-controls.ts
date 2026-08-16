import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/portal-domain/display-text';
import { PortalCommandButtons } from '@ocentra-parent/portal-domain/commands';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { renderCommandResultPanel } from './command-result-panel';
import type { PortalRenderActions } from './portal-actions';
import type { PortalRuntimeState } from './portal-state';

export function renderCommands(container: HTMLElement, state: PortalRuntimeState, actions: PortalRenderActions): void {
  const panel = document.createElement(PortalDom.Tags.Section);
  panel.className = PortalDom.Classes.Summary;

  const title = document.createElement(PortalDom.Tags.HeadingTwo);
  title.textContent = resolvePortalDevText(PortalDevTextToken.AgentCommands);

  const commandGrid = document.createElement(PortalDom.Tags.Division);
  commandGrid.className = PortalDom.Classes.CommandGrid;
  commandGrid.append(...PortalCommandButtons.map((command) => commandButton(command, state, actions)));

  panel.append(title, commandGrid);
  renderCommandResultPanel(panel, state);
  container.append(panel);
}

function commandButton(
  command: (typeof PortalCommandButtons)[number],
  state: PortalRuntimeState,
  actions: PortalRenderActions
): HTMLButtonElement {
  const button = document.createElement(PortalDom.Tags.Button);
  button.type = PortalDom.ButtonType.Button;
  button.textContent = command.label;
  button.className = activeCommandButtonClass(command.resultEvent === state.selectedCommandResultEvent);
  button.disabled = !state.commandEnabled;
  button.addEventListener(PortalDom.Events.Click, () => {
    void actions.sendCommand(command.command, command.payload);
    actions.selectCommandResult(command.resultEvent);
  });
  return button;
}

function activeCommandButtonClass(active: boolean) {
  if (!active) {
    return PortalDom.Classes.CommandResultTab;
  }
  return [PortalDom.Classes.CommandResultTab, PortalDom.Classes.CommandResultTabActive].join(
    PortalDom.Classes.ClassNameSeparator
  );
}

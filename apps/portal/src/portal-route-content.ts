import { PortalRoute, type PortalRoute as PortalRouteValue } from '@ocentra-parent/schema-domain/portal-contracts';
import { renderDevLogPanel } from './dev-log-panel';
import { renderEvents } from './event-list';
import { renderCommands } from './portal-command-controls';
import type { PortalRenderActions } from './portal-actions';
import type { PortalRuntimeState } from './portal-state';

export function renderRouteContent(
  container: HTMLElement,
  route: PortalRouteValue,
  state: PortalRuntimeState,
  actions: PortalRenderActions,
  _theme: unknown,
  _rerender: () => void
): void {
  if (route === PortalRoute.Commands) {
    renderCommands(container, state, actions);
    return;
  }
  if (route === PortalRoute.Events) {
    renderEvents(container, state.events);
    return;
  }
  if (route === PortalRoute.Logs) {
    renderDevLogPanel(container, state.latestSnapshot);
    return;
  }
}

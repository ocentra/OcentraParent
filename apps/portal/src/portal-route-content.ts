import { ParentRoute, type ParentRouteId } from '../generated/parent-ui-bridge';
import { renderDevLogPanel } from './dev-log-panel';
import { renderEvents } from './event-list';
import { renderCommands } from './portal-command-controls';
import type { PortalRenderActions } from './portal-actions';
import type { PortalRuntimeState } from './portal-state';

export function renderRouteContent(
  container: HTMLElement,
  route: ParentRouteId,
  state: PortalRuntimeState,
  actions: PortalRenderActions,
  _theme: unknown,
  _rerender: () => void
): void {
  if (route === ParentRoute.Commands) {
    renderCommands(container, state, actions);
    return;
  }
  if (route === ParentRoute.Events) {
    renderEvents(container, state.events);
    return;
  }
  if (route === ParentRoute.Logs) {
    renderDevLogPanel(container, state.latestSnapshot);
    return;
  }
}

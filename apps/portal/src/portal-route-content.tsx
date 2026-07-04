import { useLayoutEffect, useRef, type ReactElement } from 'react';
import { PortalDom, type PortalThemeValue } from '@ocentra-parent/portal-domain/contracts';
import { ParentRoute, type ParentRouteId } from '../generated/parent-ui-bridge';
import type { PortalRenderActions } from './portal-actions';
import type { PortalRuntimeState } from './portal-state';
import { renderCommands } from './portal-command-controls';
import { renderDevLogPanel } from './dev-log-panel';
import { renderEvents } from './event-list';

export type PortalRouteContentMountProps = {
  readonly actions: PortalRenderActions;
  readonly rerender: () => void;
  readonly revision: number;
  readonly route: ParentRouteId;
  readonly state: PortalRuntimeState;
  readonly theme: PortalThemeValue;
};

export function PortalRouteContentMount({
  actions,
  rerender,
  revision,
  route,
  state,
  theme,
}: PortalRouteContentMountProps): ReactElement {
  const hostRef = useRef<HTMLElement | null>(null);
  useLayoutEffect(() => {
    const host = hostRef.current;
    if (host === null) {
      return;
    }
    clear(host);
    renderPortalRouteContent(host, route, state, actions, theme, rerender);
    return () => clear(host);
  }, [actions, rerender, revision, route, state, theme]);
  return <section className={PortalDom.Classes.State} ref={hostRef} />;
}

function renderPortalRouteContent(
  host: HTMLElement,
  route: ParentRouteId,
  state: PortalRuntimeState,
  actions: PortalRenderActions,
  theme: PortalThemeValue,
  rerender: () => void
): void {
  void theme;
  void rerender;
  if (route === ParentRoute.Commands) {
    renderCommands(host, state, actions);
    return;
  }
  if (route === ParentRoute.Events) {
    renderEvents(host, state.events);
    return;
  }
  if (route === ParentRoute.Logs) {
    renderDevLogPanel(host, state.latestSnapshot);
  }
}

function clear(element: HTMLElement): void {
  while (element.firstChild !== null) {
    element.firstChild.remove();
  }
}

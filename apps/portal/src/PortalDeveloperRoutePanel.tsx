import { useLayoutEffect, useRef, type ReactElement } from 'react';
import { type PortalRoute as PortalRouteValue } from '@ocentra-parent/schema-domain/portal-contracts';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import {
  isPortalDeveloperCommandRoute,
  isPortalDeveloperEventRoute,
  isPortalDeveloperLogRoute,
  isPortalDeveloperRoute,
} from '@ocentra-parent/portal-domain/routes';
import { latestCommandResult } from '@ocentra-parent/portal-domain/command-results';
import type { PortalRenderActions } from './portal-actions';
import { renderDevLogPanel } from './dev-log-panel';
import { renderEvents } from './event-list';
import type { PortalRuntimeState } from './portal-state';
import { renderCommands } from './portal-command-controls';

export function shouldRenderPortalDeveloperRoute(route: PortalRouteValue): boolean {
  return isPortalDeveloperRoute(route);
}

export function PortalDeveloperRoutePanel({
  actions,
  route,
  state,
}: {
  readonly actions: PortalRenderActions;
  readonly route: PortalRouteValue;
  readonly state: PortalRuntimeState;
}): ReactElement {
  const hostRef = useRef<HTMLDivElement | null>(null);
  const socketReadyState = state.socket?.readyState ?? null;
  const latestSelectedCommandResultEventId =
    latestCommandResult(state.events, state.selectedCommandResultEvent)?.eventId ?? null;
  const latestSnapshotEntryId = state.latestSnapshot?.entries[0]?.id ?? null;
  const eventCount = state.events.length;

  useLayoutEffect(() => {
    const host = hostRef.current;
    if (host === null) {
      return;
    }
    clear(host);
    if (isPortalDeveloperCommandRoute(route)) {
      renderCommands(host, state, actions);
      return () => clear(host);
    }
    if (isPortalDeveloperEventRoute(route)) {
      renderEvents(host, state.events);
      return () => clear(host);
    }
    if (isPortalDeveloperLogRoute(route)) {
      renderDevLogPanel(host, state.latestSnapshot);
      return () => clear(host);
    }
    return () => clear(host);
  }, [actions, eventCount, latestSelectedCommandResultEventId, latestSnapshotEntryId, route, socketReadyState]);

  return (
    <section aria-label="Developer tools" className={PortalDom.Classes.DeveloperRoutePanel}>
      <div className={PortalDom.Classes.DeveloperRouteContent} ref={hostRef} />
    </section>
  );
}

function clear(element: HTMLElement): void {
  while (element.firstChild !== null) {
    element.firstChild.remove();
  }
}

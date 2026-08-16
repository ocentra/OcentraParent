import { useLayoutEffect, useRef, type ReactElement } from 'react';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { ParentRoute, type ParentRouteId } from '../generated/parent-ui-bridge';
import type { PortalRenderActions } from './portal-actions';
import { renderDevLogPanel } from './dev-log-panel';
import { renderEvents } from './event-list';
import { latestParentRouteEventSnapshot } from './parent-route-event-snapshot';
import type { PortalRuntimeState } from './portal-state';
import { renderCommands } from './portal-command-controls';

export function shouldRenderPortalDeveloperRoute(route: ParentRouteId): boolean {
  return isDeveloperRoute(route);
}

export function PortalDeveloperRoutePanel({
  actions,
  route,
  state,
}: {
  readonly actions: PortalRenderActions;
  readonly route: ParentRouteId;
  readonly state: PortalRuntimeState;
}): ReactElement {
  const hostRef = useRef<HTMLDivElement | null>(null);
  const commandEnabled = state.commandEnabled;
  const latestSelectedCommandResultEventId =
    latestParentRouteEventSnapshot(state.events, state.selectedCommandResultEvent)?.eventId ?? null;
  const latestSnapshotEntryId = state.latestSnapshot?.entries[0]?.id ?? null;
  const eventCount = state.events.length;

  useLayoutEffect(() => {
    const host = hostRef.current;
    if (host === null) {
      return;
    }
    clear(host);
    if (route === ParentRoute.Commands) {
      renderCommands(host, state, actions);
      return () => clear(host);
    }
    if (route === ParentRoute.Events) {
      renderEvents(host, state.events);
      return () => clear(host);
    }
    if (route === ParentRoute.Logs) {
      renderDevLogPanel(host, state.latestSnapshot);
      return () => clear(host);
    }
    return () => clear(host);
  }, [actions, commandEnabled, eventCount, latestSelectedCommandResultEventId, latestSnapshotEntryId, route]);

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

function isDeveloperRoute(route: ParentRouteId): boolean {
  return route === ParentRoute.Commands || route === ParentRoute.Events || route === ParentRoute.Logs;
}

import { useLayoutEffect, useRef, type ReactElement } from 'react';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { renderDiagnosticsPanel } from './diagnostics-panel';
import type { PortalRuntimeState } from './portal-state';

export function PortalDiagnosticsRoutePanel({ state }: { readonly state: PortalRuntimeState }): ReactElement {
  const hostRef = useRef<HTMLDivElement | null>(null);
  const latestSnapshotEntryId = state.latestSnapshot?.entries[0]?.id ?? null;
  const eventCount = state.events.length;

  useLayoutEffect(() => {
    const host = hostRef.current;
    if (host === null) {
      return;
    }
    clear(host);
    renderDiagnosticsPanel(host, state);
    return () => clear(host);
  }, [eventCount, latestSnapshotEntryId, state.connectionState, state.selectedCommandResultEvent]);

  return (
    <section aria-label="Device diagnostics" className={PortalDom.Classes.DeveloperRoutePanel}>
      <div className={PortalDom.Classes.DeveloperRouteContent} ref={hostRef} />
    </section>
  );
}

function clear(element: HTMLElement): void {
  while (element.firstChild !== null) {
    element.firstChild.remove();
  }
}

import { useLayoutEffect, useRef, useState, type ReactElement } from 'react';
import {
  PortalDom,
  type PortalRoute as PortalRouteValue,
  type PortalThemeValue,
} from '@ocentra-parent/portal-domain/contracts';
import type { PortalRenderActions } from './portal-actions';
import { routeDescriptor } from './portal-route-descriptor';
import { renderRouteContent } from './portal-route-content';
import type { PortalRuntimeState } from './portal-state';
import { PortalAuthDialog } from './PortalAuthDialog';
import { PortalSidebar } from './PortalSidebar';
import { UnifiedFooterChrome, UnifiedHeaderChrome } from './PortalUnifiedChrome';

type PortalAppProps = {
  readonly actions: PortalRenderActions;
  readonly rerender: () => void;
  readonly revision: number;
  readonly route: PortalRouteValue;
  readonly state: PortalRuntimeState;
  readonly theme: PortalThemeValue;
};

export function PortalApp(props: PortalAppProps): ReactElement {
  const [authOpen, setAuthOpen] = useState(false);
  return (
    <div className={PortalDom.Classes.AppFrame}>
      <UnifiedHeaderChrome activeRoute={props.route} onAuthOpen={() => setAuthOpen(true)} />
      <PortalSidebar actions={props.actions} route={props.route} state={props.state} />
      <main className={PortalDom.Classes.AppMain}>
        <PageHeader route={props.route} />
        <RouteContentMount
          actions={props.actions}
          rerender={props.rerender}
          revision={props.revision}
          route={props.route}
          state={props.state}
          theme={props.theme}
        />
      </main>
      <UnifiedFooterChrome />
      {authOpen ? <PortalAuthDialog onClose={() => setAuthOpen(false)} /> : null}
    </div>
  );
}

function PageHeader({ route }: { readonly route: PortalRouteValue }): ReactElement {
  const descriptor = routeDescriptor(route);
  return (
    <header className={PortalDom.Classes.AppHeader}>
      <div className={PortalDom.Classes.PageHeader}>
        <h2 className={PortalDom.Classes.PageTitle}>{descriptor.label}</h2>
        <p className={PortalDom.Classes.PageDescription}>{descriptor.description}</p>
      </div>
    </header>
  );
}

function RouteContentMount(props: PortalAppProps): ReactElement {
  const hostRef = useRef<HTMLElement | null>(null);
  useLayoutEffect(() => {
    const host = hostRef.current;
    if (host === null) {
      return;
    }
    clear(host);
    renderRouteContent(host, props.route, props.state, props.actions, props.theme, props.rerender);
    return () => clear(host);
  }, [props.actions, props.rerender, props.revision, props.route, props.state, props.theme]);
  return <section className={PortalDom.Classes.State} ref={hostRef} />;
}

function clear(element: HTMLElement): void {
  while (element.firstChild !== null) {
    element.firstChild.remove();
  }
}

import { useLayoutEffect, useMemo, useRef, useState, type CSSProperties, type ReactElement } from 'react';
import {
  PortalDom,
  PortalFrameTuner,
  PortalRoute,
  type PortalRoute as PortalRouteValue,
  type PortalThemeValue,
} from '@ocentra-parent/portal-domain/contracts';
import type { PortalRenderActions } from './portal-actions';
import { routeDescriptor } from './portal-route-descriptor';
import { renderRouteContent } from './portal-route-content';
import type { PortalRuntimeState } from './portal-state';
import { ParentLeaderboardCopyRoute } from './ParentLeaderboardCopyRoute';
import { PortalAuthDialog } from './PortalAuthDialog';
import { PortalFrameBackdrop, PortalFrameBoundsOverlay } from './PortalFrameSurface';
import { PortalFrameTunerRoute } from './PortalFrameTunerRoute';
import { PortalSidebar } from './PortalSidebar';
import { PortalUnifiedShell } from './PortalUnifiedChrome';
import {
  carouselStyle,
  frameContentStyle,
  frameContentTarget,
  frameHostClassName,
  goldenCardStyle,
} from './portal-frame-layout';
import { usePortalFrameLayout } from './use-portal-frame-layout';

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
  const isFrameTuner = props.route === PortalRoute.FrameTuner;
  const isDevProtocolRoute = props.route === PortalRoute.Commands || props.route === PortalRoute.Events;
  const isProductRoute = !isFrameTuner && !isDevProtocolRoute;
  const [frameLayout, setFrameLayout] = usePortalFrameLayout(!isFrameTuner && import.meta.env.DEV);
  const appFrameStyle = useMemo<CSSProperties>(
    () => ({
      columnGap: frameLayout.shell.frameGap,
      gridTemplateColumns: `${frameLayout.shell.sidebarWidth}px minmax(0, 1fr)`,
      padding: frameLayout.shell.shellEdge,
      ...carouselStyle(frameLayout.carousel),
      ...goldenCardStyle(frameLayout.goldenCard),
      [PortalFrameTuner.CssVar.SideBottomHeight]: `${frameLayout.shell.sideBottomHeight}px`,
      [PortalFrameTuner.CssVar.SideStackGap]: `${frameLayout.shell.sideStackGap}px`,
    }),
    [
      frameLayout.shell.frameGap,
      frameLayout.carousel,
      frameLayout.goldenCard,
      frameLayout.shell.shellEdge,
      frameLayout.shell.sidebarWidth,
      frameLayout.shell.sideBottomHeight,
      frameLayout.shell.sideStackGap,
    ]
  );
  const mainContent = frameContentTarget(frameLayout, PortalFrameTuner.FrameTarget.Main);
  const appMainStyle = useMemo<CSSProperties>(
    () => frameContentStyle(mainContent, frameLayout.main) as CSSProperties,
    [frameLayout.main, mainContent]
  );
  const appMainClassName = useMemo(() => frameHostClassName(PortalDom.Classes.AppMain, mainContent), [mainContent]);
  if (isFrameTuner) {
    return <PortalFrameTunerRoute layout={frameLayout} onLayoutChange={setFrameLayout} />;
  }
  if (isProductRoute) {
    return (
      <>
        <PortalUnifiedShell onAuthOpen={() => setAuthOpen(true)}>
          <ParentLeaderboardCopyRoute
            key={props.route}
            actions={props.actions}
            route={props.route}
            state={props.state}
          />
        </PortalUnifiedShell>
        {authOpen ? <PortalAuthDialog onClose={() => setAuthOpen(false)} /> : null}
      </>
    );
  }
  return (
    <>
      <PortalUnifiedShell onAuthOpen={() => setAuthOpen(true)}>
        <div className={PortalDom.Classes.AppFrame} style={appFrameStyle}>
          <PortalSidebar actions={props.actions} frameLayout={frameLayout} route={props.route} state={props.state} />
          <main aria-label={PortalFrameTuner.Text.TargetMain} className={appMainClassName} style={appMainStyle}>
            <PortalFrameBackdrop ariaLabel={PortalFrameTuner.Text.PreviewMain} controls={frameLayout.main} />
            <PortalFrameBoundsOverlay content={mainContent} />
            <div className={PortalFrameTuner.Classes.FrameContent}>
              <PageHeader route={props.route} />
              <RouteContentMount
                actions={props.actions}
                rerender={props.rerender}
                revision={props.revision}
                route={props.route}
                state={props.state}
                theme={props.theme}
              />
            </div>
          </main>
        </div>
      </PortalUnifiedShell>
      {authOpen ? <PortalAuthDialog onClose={() => setAuthOpen(false)} /> : null}
    </>
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

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
import { ParentPortalRoute } from './ParentPortalRoute';
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
import type { PortalFrameContentTargetLayout, PortalFrameLayout } from './portal-frame-layout';
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
  const routeFrameLayout = useMemo(
    () => frameLayoutVisibleForProtocolRoute(frameLayout, isDevProtocolRoute),
    [frameLayout, isDevProtocolRoute]
  );
  const appFrameStyle = useMemo<CSSProperties>(
    () => ({
      columnGap: routeFrameLayout.shell.frameGap,
      gridTemplateColumns: `${routeFrameLayout.shell.sidebarWidth}px minmax(0, 1fr)`,
      padding: routeFrameLayout.shell.shellEdge,
      ...carouselStyle(routeFrameLayout.carousel),
      ...goldenCardStyle(routeFrameLayout.goldenCard),
      [PortalFrameTuner.CssVar.SideBottomHeight]: `${routeFrameLayout.shell.sideBottomHeight}px`,
      [PortalFrameTuner.CssVar.SideStackGap]: `${routeFrameLayout.shell.sideStackGap}px`,
    }),
    [
      routeFrameLayout.shell.frameGap,
      routeFrameLayout.carousel,
      routeFrameLayout.goldenCard,
      routeFrameLayout.shell.shellEdge,
      routeFrameLayout.shell.sidebarWidth,
      routeFrameLayout.shell.sideBottomHeight,
      routeFrameLayout.shell.sideStackGap,
    ]
  );
  const mainContent = frameContentTarget(routeFrameLayout, PortalFrameTuner.FrameTarget.Main);
  const appMainStyle = useMemo<CSSProperties>(
    () => frameContentStyle(mainContent, routeFrameLayout.main) as CSSProperties,
    [routeFrameLayout.main, mainContent]
  );
  const appMainClassName = useMemo(() => frameHostClassName(PortalDom.Classes.AppMain, mainContent), [mainContent]);
  if (isFrameTuner) {
    return <PortalFrameTunerRoute layout={frameLayout} onLayoutChange={setFrameLayout} />;
  }
  if (isProductRoute) {
    const controls =
      props.route === PortalRoute.Assistant ? frameLayout.parentPortal.chatInterface : frameLayout.parentPortal.mainApp;
    return (
      <>
        <PortalUnifiedShell onAuthOpen={() => setAuthOpen(true)}>
          <ParentPortalRoute
            key={props.route}
            actions={props.actions}
            controls={controls}
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
          <PortalSidebar
            actions={props.actions}
            frameLayout={routeFrameLayout}
            route={props.route}
            state={props.state}
          />
          <main aria-label={PortalFrameTuner.Text.TargetMain} className={appMainClassName} style={appMainStyle}>
            <PortalFrameBackdrop ariaLabel={PortalFrameTuner.Text.PreviewMain} controls={routeFrameLayout.main} />
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

function frameLayoutVisibleForProtocolRoute(layout: PortalFrameLayout, isDevProtocolRoute: boolean): PortalFrameLayout {
  if (!isDevProtocolRoute) {
    return layout;
  }
  return {
    ...layout,
    content: {
      sideTop: visibleContentTarget(layout.content.sideTop),
      sideBottom: visibleContentTarget(layout.content.sideBottom),
      main: visibleContentTarget(layout.content.main),
    },
  };
}

function visibleContentTarget(content: PortalFrameContentTargetLayout): PortalFrameContentTargetLayout {
  return content.showContent ? content : { ...content, showContent: true };
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

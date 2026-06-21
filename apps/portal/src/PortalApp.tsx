import {
  useEffect,
  useLayoutEffect,
  useMemo,
  useRef,
  useState,
  type CSSProperties,
  type Dispatch,
  type MutableRefObject,
  type ReactElement,
  type SetStateAction,
} from 'react';
import {
  AgentCommand,
  AgentEvent,
  type AgentEventEnvelope,
  type AgentEventName,
} from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';
import { type AgentEventId } from '@ocentra-parent/schema-domain/event-primitives';
import {
  PortalDom,
  PortalLanPairingScan,
  type PortalThemeValue,
} from '@ocentra-parent/portal-domain/contracts';
import { PortalFrameTuner } from '@ocentra-parent/portal-domain/frame-tuner';
import { PortalRoute, type PortalRoute as PortalRouteValue } from '@ocentra-parent/portal-domain/routes';
import type { PortalRenderActions } from './portal-actions';
import { routeDescriptor } from './portal-route-descriptor';
import { renderRouteContent } from './portal-route-content';
import type { PortalRuntimeState } from './portal-state';
import { ParentPortalRoute } from './ParentPortalRoute';
import { PortalAuthDialog } from './PortalAuthDialog';
import { PortalFrameBackdrop, PortalFrameBoundsOverlay } from './PortalFrameSurface';
import { PortalFrameTunerRoute } from './PortalFrameTunerRoute';
import { PortalSidebar } from './PortalSidebar';
import { PortalShellStatusBar } from './PortalShellStatusBar';
import { PortalUnifiedShell } from './PortalUnifiedChrome';
import {
  carouselStyle,
  frameContentStyle,
  frameHostClassName,
  goldenCardStyle,
} from './portal-frame-layout-style';
import { frameContentTarget } from './portal-frame-layout-state';
import type { PortalFrameContentTargetLayout, PortalFrameLayout } from './portal-frame-layout-types';
import { usePortalNetworkActivityRefresh } from './use-portal-network-activity-refresh';
import { usePortalFrameLayout } from './use-portal-frame-layout';

type PortalAppProps = {
  readonly actions: PortalRenderActions;
  readonly rerender: () => void;
  readonly revision: number;
  readonly route: PortalRouteValue;
  readonly state: PortalRuntimeState;
  readonly theme: PortalThemeValue;
  readonly onThemeChange: (theme: PortalThemeValue) => void;
  readonly onProductSurfaceReady: () => void;
};

const PORTAL_HEADER_ROUTE_TRANSITION_MS = 1040;

export function PortalApp(props: PortalAppProps): ReactElement {
  const [authOpen, setAuthOpen] = useState(false);
  const [headerRouteTransitionActive, setHeaderRouteTransitionActive] = useState(false);
  const [lanPairingAutoScanSequence, setLanPairingAutoScanSequence] = useState(0);
  const previousRouteRef = useRef<PortalRouteValue>(props.route);
  const autoLanScanRequestedForRouteRef = useRef(false);
  const autoLanScanStartedAfterEventIdRef = useRef<AgentEventId | null>(null);
  const networkActivityRefreshRequestedForRouteRef = useRef(false);
  const isFrameTuner = props.route === PortalRoute.FrameTuner;
  const isDevProtocolRoute = isPortalDevProtocolRoute(props.route);
  const isProductRoute = !isFrameTuner && !isDevProtocolRoute;
  const [frameLayout, setFrameLayout] = usePortalFrameLayout(!isFrameTuner && import.meta.env.DEV);
  const routeFrameLayout = useMemo(
    () => frameLayoutVisibleForProtocolRoute(frameLayout, isDevProtocolRoute),
    [frameLayout, isDevProtocolRoute]
  );
  const latestLanPairingScanEventId =
    latestPortalEvent(props.state.events, AgentEvent.LanPairingBrowserDiscoveryReported)?.eventId ?? null;
  const hasNetworkFlowReadModelEvent =
    latestPortalEvent(props.state.events, AgentEvent.NetworkFlowReadModelReported) !== null;
  const openAuthDialog = (): void => setAuthOpen(true);
  const closeAuthDialog = (): void => setAuthOpen(false);
  usePortalProductReady(isProductRoute, props.onProductSurfaceReady);
  usePortalRouteTransition({
    autoLanScanRequestedForRouteRef,
    autoLanScanStartedAfterEventIdRef,
    previousRouteRef,
    route: props.route,
    setHeaderRouteTransitionActive,
  });
  usePortalDeviceAutoScan({
    actions: props.actions,
    autoLanScanRequestedForRouteRef,
    autoLanScanStartedAfterEventIdRef,
    latestLanPairingScanEventId,
    route: props.route,
    setHeaderRouteTransitionActive,
    setLanPairingAutoScanSequence,
    socket: props.state.socket,
  });
  usePortalDeviceScanCompletion({
    autoLanScanStartedAfterEventIdRef,
    latestLanPairingScanEventId,
    route: props.route,
    setHeaderRouteTransitionActive,
  });
  usePortalNetworkActivityRefresh({
    actions: props.actions,
    connectionState: props.state.connectionState,
    hasNetworkFlowReadModelEvent,
    networkActivityRefreshRequestedForRouteRef,
    route: props.route,
  });
  if (isFrameTuner) {
    return <PortalFrameTunerRoute layout={frameLayout} onLayoutChange={setFrameLayout} />;
  }
  if (isProductRoute) {
    return (
      <PortalProductRouteShell
        {...props}
        authOpen={authOpen}
        frameLayout={frameLayout}
        headerRouteTransitionActive={headerRouteTransitionActive}
        lanPairingAutoScanSequence={lanPairingAutoScanSequence}
        onAuthClose={closeAuthDialog}
        onAuthOpen={openAuthDialog}
      />
    );
  }
  return (
    <PortalProtocolRouteShell
      {...props}
      authOpen={authOpen}
      headerRouteTransitionActive={headerRouteTransitionActive}
      onAuthClose={closeAuthDialog}
      onAuthOpen={openAuthDialog}
      routeFrameLayout={routeFrameLayout}
    />
  );
}

type PortalProductRouteShellProps = PortalAppProps & {
  readonly authOpen: boolean;
  readonly frameLayout: PortalFrameLayout;
  readonly headerRouteTransitionActive: boolean;
  readonly lanPairingAutoScanSequence: number;
  readonly onAuthClose: () => void;
  readonly onAuthOpen: () => void;
};

type PortalProtocolRouteShellProps = PortalAppProps & {
  readonly authOpen: boolean;
  readonly headerRouteTransitionActive: boolean;
  readonly onAuthClose: () => void;
  readonly onAuthOpen: () => void;
  readonly routeFrameLayout: PortalFrameLayout;
};

function PortalProductRouteShell({
  actions,
  authOpen,
  frameLayout,
  headerRouteTransitionActive,
  lanPairingAutoScanSequence,
  onAuthClose,
  onAuthOpen,
  onProductSurfaceReady,
  onThemeChange,
  route,
  state,
  theme,
}: PortalProductRouteShellProps): ReactElement {
  const controls =
    route === PortalRoute.Assistant ? frameLayout.parentPortal.chatInterface : frameLayout.parentPortal.mainApp;
  return (
    <>
      <PortalUnifiedShell
        onAuthOpen={onAuthOpen}
        onThemeChange={onThemeChange}
        routeTransitionActive={headerRouteTransitionActive}
        theme={theme}
      >
        <PortalShellStatusBar route={route} state={state} />
        <ParentPortalRoute
          actions={actions}
          controls={controls}
          lanPairingAutoScanSequence={lanPairingAutoScanSequence}
          onProductSurfaceReady={onProductSurfaceReady}
          route={route}
          state={state}
        />
      </PortalUnifiedShell>
      <PortalAuthDialogMount open={authOpen} onClose={onAuthClose} />
    </>
  );
}

function PortalProtocolRouteShell(props: PortalProtocolRouteShellProps): ReactElement {
  const { appFrameStyle, appMainClassName, appMainStyle, mainContent } = usePortalProtocolFrameState(
    props.routeFrameLayout
  );
  return (
    <>
      <PortalUnifiedShell
        onAuthOpen={props.onAuthOpen}
        onThemeChange={props.onThemeChange}
        routeTransitionActive={props.headerRouteTransitionActive}
        theme={props.theme}
      >
        <PortalShellStatusBar route={props.route} state={props.state} />
        <div className={PortalDom.Classes.AppFrame} style={appFrameStyle}>
          <PortalSidebar
            actions={props.actions}
            frameLayout={props.routeFrameLayout}
            route={props.route}
            state={props.state}
          />
          <main aria-label={PortalFrameTuner.Text.TargetMain} className={appMainClassName} style={appMainStyle}>
            <PortalFrameBackdrop ariaLabel={PortalFrameTuner.Text.PreviewMain} controls={props.routeFrameLayout.main} />
            <PortalFrameBoundsOverlay content={mainContent} />
            <div className={PortalFrameTuner.Classes.FrameContent}>
              <PageHeader route={props.route} />
              <RouteContentMount {...props} />
            </div>
          </main>
        </div>
      </PortalUnifiedShell>
      <PortalAuthDialogMount open={props.authOpen} onClose={props.onAuthClose} />
    </>
  );
}

type PortalProtocolFrameState = {
  readonly appFrameStyle: CSSProperties;
  readonly appMainClassName: string;
  readonly appMainStyle: CSSProperties;
  readonly mainContent: PortalFrameContentTargetLayout;
};

function usePortalProtocolFrameState(routeFrameLayout: PortalFrameLayout): PortalProtocolFrameState {
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
  return { appFrameStyle, appMainClassName, appMainStyle, mainContent };
}

function PortalAuthDialogMount({
  onClose,
  open,
}: {
  readonly onClose: () => void;
  readonly open: boolean;
}): ReactElement | null {
  return open ? <PortalAuthDialog onClose={onClose} /> : null;
}

type PortalRouteTransitionHook = {
  readonly autoLanScanRequestedForRouteRef: MutableRefObject<boolean>;
  readonly autoLanScanStartedAfterEventIdRef: MutableRefObject<AgentEventId | null>;
  readonly previousRouteRef: MutableRefObject<PortalRouteValue>;
  readonly route: PortalRouteValue;
  readonly setHeaderRouteTransitionActive: Dispatch<SetStateAction<boolean>>;
};

type PortalDeviceAutoScanHook = {
  readonly actions: PortalRenderActions;
  readonly autoLanScanRequestedForRouteRef: MutableRefObject<boolean>;
  readonly autoLanScanStartedAfterEventIdRef: MutableRefObject<AgentEventId | null>;
  readonly latestLanPairingScanEventId: AgentEventId | null;
  readonly route: PortalRouteValue;
  readonly setHeaderRouteTransitionActive: Dispatch<SetStateAction<boolean>>;
  readonly setLanPairingAutoScanSequence: Dispatch<SetStateAction<number>>;
  readonly socket: WebSocket | null;
};

type PortalDeviceScanCompletionHook = {
  readonly autoLanScanStartedAfterEventIdRef: MutableRefObject<AgentEventId | null>;
  readonly latestLanPairingScanEventId: AgentEventId | null;
  readonly route: PortalRouteValue;
  readonly setHeaderRouteTransitionActive: Dispatch<SetStateAction<boolean>>;
};

function usePortalProductReady(isProductRoute: boolean, onProductSurfaceReady: () => void): void {
  useLayoutEffect(() => {
    if (!isProductRoute) {
      onProductSurfaceReady();
    }
  }, [isProductRoute, onProductSurfaceReady]);
}

function isPortalDevProtocolRoute(route: PortalRouteValue): boolean {
  return route === PortalRoute.Commands || route === PortalRoute.Events || route === PortalRoute.Logs;
}

function usePortalRouteTransition({
  autoLanScanRequestedForRouteRef,
  autoLanScanStartedAfterEventIdRef,
  previousRouteRef,
  route,
  setHeaderRouteTransitionActive,
}: PortalRouteTransitionHook): void {
  useEffect(() => {
    if (previousRouteRef.current === route) {
      return;
    }
    previousRouteRef.current = route;
    autoLanScanRequestedForRouteRef.current = false;
    autoLanScanStartedAfterEventIdRef.current = null;
    setHeaderRouteTransitionActive(true);
    const timeout = window.setTimeout(() => setHeaderRouteTransitionActive(false), PORTAL_HEADER_ROUTE_TRANSITION_MS);
    return () => window.clearTimeout(timeout);
  }, [
    autoLanScanRequestedForRouteRef,
    autoLanScanStartedAfterEventIdRef,
    previousRouteRef,
    route,
    setHeaderRouteTransitionActive,
  ]);
}

function usePortalDeviceAutoScan({
  actions,
  autoLanScanRequestedForRouteRef,
  autoLanScanStartedAfterEventIdRef,
  latestLanPairingScanEventId,
  route,
  setHeaderRouteTransitionActive,
  setLanPairingAutoScanSequence,
  socket,
}: PortalDeviceAutoScanHook): void {
  useEffect(() => {
    if (route !== PortalRoute.Devices) {
      autoLanScanRequestedForRouteRef.current = false;
      autoLanScanStartedAfterEventIdRef.current = null;
      return undefined;
    }
    if (autoLanScanRequestedForRouteRef.current || socket?.readyState !== WebSocket.OPEN) {
      return undefined;
    }
    autoLanScanRequestedForRouteRef.current = true;
    autoLanScanStartedAfterEventIdRef.current = latestLanPairingScanEventId;
    setHeaderRouteTransitionActive(true);
    setLanPairingAutoScanSequence((sequence) => sequence + 1);
    actions.sendCommand(AgentCommand.LanPairingBrowserDiscoveryScan, {
      [AgentProtocolDefaults.Field.LanRouteId]: AgentProtocolDefaults.Target.LocalNetworkWindowsAgent.route,
    });
    const timeout = window.setTimeout(
      () => setHeaderRouteTransitionActive(false),
      PortalLanPairingScan.PendingIndicatorMs
    );
    return () => window.clearTimeout(timeout);
  }, [
    actions,
    autoLanScanRequestedForRouteRef,
    autoLanScanStartedAfterEventIdRef,
    latestLanPairingScanEventId,
    route,
    setHeaderRouteTransitionActive,
    setLanPairingAutoScanSequence,
    socket,
  ]);
}

function usePortalDeviceScanCompletion({
  autoLanScanStartedAfterEventIdRef,
  latestLanPairingScanEventId,
  route,
  setHeaderRouteTransitionActive,
}: PortalDeviceScanCompletionHook): void {
  useEffect(() => {
    if (
      route !== PortalRoute.Devices ||
      latestLanPairingScanEventId === null ||
      latestLanPairingScanEventId === autoLanScanStartedAfterEventIdRef.current
    ) {
      return;
    }
    setHeaderRouteTransitionActive(false);
  }, [autoLanScanStartedAfterEventIdRef, latestLanPairingScanEventId, route, setHeaderRouteTransitionActive]);
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

function latestPortalEvent(
  events: readonly AgentEventEnvelope[],
  eventName: AgentEventName
): AgentEventEnvelope | null {
  let latest: AgentEventEnvelope | null = null;
  let latestTime = Number.NEGATIVE_INFINITY;
  let latestIndex = -1;
  for (let index = 0; index < events.length; index += 1) {
    const event = events[index];
    if (event !== undefined && event.event === eventName) {
      const sentAt = Date.parse(event.sentAt);
      const eventTime = Number.isFinite(sentAt) ? sentAt : index;
      if (eventTime > latestTime || (eventTime === latestTime && index > latestIndex)) {
        latest = event;
        latestTime = eventTime;
        latestIndex = index;
      }
    }
  }
  return latest;
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

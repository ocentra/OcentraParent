import {
  useEffect,
  useLayoutEffect,
  useMemo,
  useRef,
  useState,
  type Dispatch,
  type MutableRefObject,
  type SetStateAction,
} from 'react';
import { AgentEventIdSchema, type AgentEventId } from '@ocentra-parent/schema-domain/event-primitives';
import { safeParseUnknown } from '@ocentra-parent/schema-domain/effect';
import { PortalLanPairingScan } from '@ocentra-parent/portal-domain/contracts';
import { ParentBridgeConnectionState, ParentRoute, type ParentRouteId } from '../generated/parent-ui-bridge';
import type { ParentScreenSummaryPanelSnapshot } from '../generated/parent-ui-bridge';
import type { PortalRenderActions } from './portal-actions';
import { resolveSnapshotLiveActivityState } from './route-live-activity-state';
import type { PortalRuntimeState } from './portal-state';
import type { PortalFrameContentTargetLayout, PortalFrameLayout } from './portal-frame-layout-types';
import { usePortalFrameLayout } from './use-portal-frame-layout';

export type PortalAppBehaviorProps = {
  readonly actions: PortalRenderActions;
  readonly onProductSurfaceReady: () => void;
  readonly route: ParentRouteId;
  readonly state: PortalRuntimeState;
};

export type PortalAppBehavior = {
  readonly authOpen: boolean;
  readonly closeAuthDialog: () => void;
  readonly frameLayout: PortalFrameLayout;
  readonly headerRouteTransitionActive: boolean;
  readonly isFrameTuner: boolean;
  readonly isProductRoute: boolean;
  readonly lanPairingAutoScanSequence: number;
  readonly latestLanPairingScanEventId: AgentEventId | null;
  readonly openAuthDialog: () => void;
  readonly routeFrameLayout: PortalFrameLayout;
  readonly screenSummaryPanel: ParentScreenSummaryPanelSnapshot | null;
  readonly setFrameLayout: (layout: PortalFrameLayout) => void;
};

const PORTAL_HEADER_ROUTE_TRANSITION_MS = 1040;

export function usePortalAppBehavior({
  actions,
  onProductSurfaceReady,
  route,
  state,
}: PortalAppBehaviorProps): PortalAppBehavior {
  const [authOpen, setAuthOpen] = useState(false);
  const [headerRouteTransitionActive, setHeaderRouteTransitionActive] = useState(false);
  const [lanPairingAutoScanSequence, setLanPairingAutoScanSequence] = useState(0);
  const previousRouteRef = useRef<ParentRouteId>(route);
  const autoLanScanRequestedForRouteRef = useRef(false);
  const autoLanScanStartedAfterEventIdRef = useRef<AgentEventId | null>(null);
  const networkActivityRefreshRequestedForRouteRef = useRef(false);
  const isFrameTuner = route === ParentRoute.AppLayout || route === ParentRoute.FrameTuner;
  const isDevProtocolRoute = isParentDevProtocolRoute(route);
  const isProductRoute = portalRouteUsesProductShell(route);
  const [frameLayout, setFrameLayout] = usePortalFrameLayout(!isFrameTuner && import.meta.env.DEV);
  const routeFrameLayout = useMemo(
    () => frameLayoutVisibleForProtocolRoute(frameLayout, isDevProtocolRoute),
    [frameLayout, isDevProtocolRoute]
  );
  const routeLiveActivity = resolveSnapshotLiveActivityState(state.routeSnapshot?.liveActivity ?? null);
  const latestLanPairingScanEventId = decodeSnapshotEventId(
    routeLiveActivity?.lanPairingBrowserDiscoveryEvent?.eventId
  );
  const openAuthDialog = (): void => setAuthOpen(true);
  const closeAuthDialog = (): void => setAuthOpen(false);
  usePortalProductReady(isProductRoute, onProductSurfaceReady);
  usePortalRouteTransition({
    autoLanScanRequestedForRouteRef,
    autoLanScanStartedAfterEventIdRef,
    previousRouteRef,
    route,
    setHeaderRouteTransitionActive,
  });
  usePortalDeviceAutoScan({
    actions,
    autoLanScanRequestedForRouteRef,
    autoLanScanStartedAfterEventIdRef,
    commandEnabled: state.commandEnabled,
    latestLanPairingScanEventId,
    route,
    setHeaderRouteTransitionActive,
    setLanPairingAutoScanSequence,
  });
  usePortalDeviceScanCompletion({
    autoLanScanStartedAfterEventIdRef,
    latestLanPairingScanEventId,
    route,
    setHeaderRouteTransitionActive,
  });
  usePortalNetworkActivityRefresh({
    actions,
    connectionState: state.connectionState,
    networkActivityRefreshRequestedForRouteRef,
    route,
  });

  return {
    authOpen,
    closeAuthDialog,
    frameLayout,
    headerRouteTransitionActive,
    isFrameTuner,
    isProductRoute,
    lanPairingAutoScanSequence,
    latestLanPairingScanEventId,
    openAuthDialog,
    routeFrameLayout,
    screenSummaryPanel: routeLiveActivity.screenSummaryPanel ?? null,
    setFrameLayout,
  };
}

type PortalRouteTransitionHook = {
  readonly autoLanScanRequestedForRouteRef: MutableRefObject<boolean>;
  readonly autoLanScanStartedAfterEventIdRef: MutableRefObject<AgentEventId | null>;
  readonly previousRouteRef: MutableRefObject<ParentRouteId>;
  readonly route: ParentRouteId;
  readonly setHeaderRouteTransitionActive: Dispatch<SetStateAction<boolean>>;
};

type PortalDeviceAutoScanHook = {
  readonly actions: PortalRenderActions;
  readonly autoLanScanRequestedForRouteRef: MutableRefObject<boolean>;
  readonly autoLanScanStartedAfterEventIdRef: MutableRefObject<AgentEventId | null>;
  readonly commandEnabled: boolean;
  readonly latestLanPairingScanEventId: AgentEventId | null;
  readonly route: ParentRouteId;
  readonly setHeaderRouteTransitionActive: Dispatch<SetStateAction<boolean>>;
  readonly setLanPairingAutoScanSequence: Dispatch<SetStateAction<number>>;
};

type PortalDeviceScanCompletionHook = {
  readonly autoLanScanStartedAfterEventIdRef: MutableRefObject<AgentEventId | null>;
  readonly latestLanPairingScanEventId: AgentEventId | null;
  readonly route: ParentRouteId;
  readonly setHeaderRouteTransitionActive: Dispatch<SetStateAction<boolean>>;
};

function usePortalProductReady(isProductRoute: boolean, onProductSurfaceReady: () => void): void {
  useLayoutEffect(() => {
    if (!isProductRoute) {
      onProductSurfaceReady();
    }
  }, [isProductRoute, onProductSurfaceReady]);
}

function isParentDevProtocolRoute(route: ParentRouteId): boolean {
  return route === ParentRoute.Commands || route === ParentRoute.Events || route === ParentRoute.Logs;
}

export function portalRouteUsesProductShell(route: ParentRouteId): boolean {
  const isFrameTuner = route === ParentRoute.AppLayout || route === ParentRoute.FrameTuner;
  return !isFrameTuner && !isParentDevProtocolRoute(route);
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
  commandEnabled,
  latestLanPairingScanEventId,
  route,
  setHeaderRouteTransitionActive,
  setLanPairingAutoScanSequence,
}: PortalDeviceAutoScanHook): void {
  useEffect(() => {
    if (route !== ParentRoute.Devices) {
      autoLanScanRequestedForRouteRef.current = false;
      autoLanScanStartedAfterEventIdRef.current = null;
      return undefined;
    }
    if (autoLanScanRequestedForRouteRef.current || !commandEnabled) {
      return undefined;
    }
    autoLanScanRequestedForRouteRef.current = true;
    autoLanScanStartedAfterEventIdRef.current = latestLanPairingScanEventId;
    setHeaderRouteTransitionActive(true);
    setLanPairingAutoScanSequence((sequence) => sequence + 1);
    void actions.requestLanPairingBrowserDiscoveryScan?.();
    const timeout = window.setTimeout(
      () => setHeaderRouteTransitionActive(false),
      PortalLanPairingScan.PendingIndicatorMs
    );
    return () => window.clearTimeout(timeout);
  }, [
    actions,
    autoLanScanRequestedForRouteRef,
    autoLanScanStartedAfterEventIdRef,
    commandEnabled,
    latestLanPairingScanEventId,
    route,
    setHeaderRouteTransitionActive,
    setLanPairingAutoScanSequence,
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
      route !== ParentRoute.Devices ||
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

function decodeSnapshotEventId(eventId: unknown): AgentEventId | null {
  const parsed = safeParseUnknown(AgentEventIdSchema, eventId);
  return parsed.success ? parsed.data : null;
}

function usePortalNetworkActivityRefresh({
  actions,
  connectionState,
  networkActivityRefreshRequestedForRouteRef,
  route,
}: {
  readonly actions: PortalRenderActions;
  readonly connectionState: ParentBridgeConnectionState;
  readonly networkActivityRefreshRequestedForRouteRef: MutableRefObject<boolean>;
  readonly route: ParentRouteId;
}): void {
  useEffect(() => {
    if (route !== ParentRoute.NetworkActivity) {
      networkActivityRefreshRequestedForRouteRef.current = false;
      return;
    }
    if (
      networkActivityRefreshRequestedForRouteRef.current ||
      connectionState === ParentBridgeConnectionState.Disconnected
    ) {
      return;
    }
    networkActivityRefreshRequestedForRouteRef.current = true;
    void actions.requestNetworkFlowReadModelRefresh?.();
  }, [actions, connectionState, networkActivityRefreshRequestedForRouteRef, route]);
}

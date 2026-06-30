import { createElement } from 'react';
import { createRoot } from 'react-dom/client';
import { AgentCommand } from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import {
  GeneratedDevLogField as DevLogField,
  GeneratedDevLogMessage as DevLogMessage,
} from '@ocentra-parent/schema-domain/generated/logging-contracts';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/schema-domain/text-portal-dev';
import { PortalDom, type PortalThemeValue } from '@ocentra-parent/portal-domain/contracts';
import {
  readStoredManageTargetSelection,
  selectedChildDeviceIdFromManageTargetSelection,
} from '@ocentra-parent/portal-domain/manage-target-selection';
import { writePortalDevLog } from './dev-logger';
import {
  ParentBridgeConnectionState,
  ParentRoute,
  ParentUiActionKind,
  type ParentRouteContext,
  type ParentRouteId,
  type ParentUiAction,
  type ParentUiActionResult,
  parentRouteFromHashPath,
  parentRouteHashPath,
} from '../generated/parent-ui-bridge';
import { createHostBridge } from './host-bridge';
import { fadePortalBackgroundBootLayer, removePortalBackgroundBootLayer } from './portal-background-boot';
import { HostedPortalDistribution, resolveHostedPortalDistributionState } from './hosted-portal-distribution';
import { PortalBackgroundDevTool } from './PortalBackgroundDevTool';
import { PortalApp } from './PortalApp';
import type { PortalRenderActions } from './portal-actions';
import {
  applyParentRouteEvents,
  applyParentRouteSnapshot,
  applyParentSubscriptionEvent,
  createPortalRuntimeState,
} from './portal-state';
import { applyTheme, resolveTheme, selectTheme } from './portal-theme';
import './styles.css';
import './portal-unified-chrome.css';
import './styles/deck-frame-fit.css';
import './styles/control-card-frame.css';
import './styles/frame-tuner.css';
import './styles/parent-portal-route.css';

const app = requirePortalRoot();
const root = createRoot(app);
const hostedPortalDistributionState = resolveHostedPortalDistributionState(
  {
    hash: window.location.hash,
    origin: window.location.origin,
    pathname: window.location.pathname,
    search: window.location.search,
  },
  import.meta.env
);
const bridge = createHostBridge();
const state = createPortalRuntimeState();
let revision = 0;
let appLoadingHideRequested = false;
let routeLoadSequence = 0;
let routeSubscriptionToken = 0;
let activeRouteUnsubscribe: (() => void) | null = null;
const APP_LOADING_FADE_FALLBACK_MS = 920;

installAppLoadingHider();

writePortalDevLog(DevLogMessage.PortalStarted, {
  hostBridgeEndpoint: state.agentEndpoint,
});

const actions: PortalRenderActions = {
  reconnect() {
    void loadCurrentRoute();
  },
  selectCommandResult(resultEvent) {
    state.selectedCommandResultEvent = resultEvent;
    refresh();
  },
  async sendCommand(command, payload) {
    return dispatchHostAction({
      action: ParentUiActionKind.AgentCommandRequested,
      route: getRoute(),
      command,
      payload,
    });
  },
  async refreshRouteSnapshot() {
    return dispatchHostAction({
      action: ParentUiActionKind.RefreshRoute,
      route: getRoute(),
      payload: {},
    });
  },
  async requestLanPairingBrowserDiscoveryScan() {
    return dispatchHostAction({
      action: ParentUiActionKind.LanPairingBrowserDiscoveryScanRequested,
      route: getRoute(),
      payload: {},
    });
  },
  async requestNetworkFlowReadModelRefresh() {
    return dispatchHostAction({
      action: ParentUiActionKind.NetworkFlowReadModelRefreshRequested,
      route: getRoute(),
      payload: {},
    });
  },
  async requestTrackingRetentionSettingsWrite() {
    return dispatchHostAction({
      action: ParentUiActionKind.TrackingRetentionSettingsWriteRequested,
      route: getRoute(),
      payload: {},
    });
  },
  async requestPolicyRequestAssistantPreviewConfirm(payload) {
    return dispatchHostAction({
      action: ParentUiActionKind.PolicyRequestAssistantPreviewConfirmRequested,
      route: getRoute(),
      payload,
    });
  },
  async requestScreenSettingsGet(payload) {
    return dispatchHostAction({
      action: ParentUiActionKind.ScreenSettingsGetRequested,
      route: getRoute(),
      payload,
    });
  },
  async requestScreenSettingsReplace(payload) {
    return dispatchHostAction({
      action: ParentUiActionKind.ScreenSettingsReplaceRequested,
      route: getRoute(),
      payload,
    });
  },
  async requestAppGameAdapterDispatchExecute() {
    return dispatchHostAction({
      action: ParentUiActionKind.AppGameAdapterDispatchExecuteRequested,
      route: getRoute(),
      payload: {},
    });
  },
  async requestAppGameTimerParentPreferenceSetup(payload) {
    return dispatchHostAction({
      action: ParentUiActionKind.AppGameTimerParentPreferenceSetupRequested,
      route: getRoute(),
      payload,
    });
  },
};

async function loadCurrentRoute(): Promise<void> {
  const route = getRoute();
  const sequence = routeLoadSequence + 1;
  routeLoadSequence = sequence;
  disposeRouteSubscription();
  state.connectionState = ParentBridgeConnectionState.Connecting;
  refresh();
  try {
    const snapshot = await bridge.loadRoute(route, currentRouteContext());
    if (sequence !== routeLoadSequence) {
      return;
    }
    applyParentRouteSnapshot(state, snapshot);
    refresh();
    await installRouteSubscription(route);
    await maybePrimeDeveloperRoute(route);
  } catch (error) {
    if (sequence !== routeLoadSequence) {
      return;
    }
    state.connectionState = ParentBridgeConnectionState.Error;
    state.commandEnabled = false;
    state.lastHostMessage = error instanceof Error ? error.message : String(error);
    refresh();
  }
}

async function maybePrimeDeveloperRoute(route: ParentRouteId): Promise<void> {
  if (!shouldPrimeDeveloperRoute(route)) {
    return;
  }
  if (state.events.length > 0 || state.latestSnapshot !== null) {
    return;
  }

  await dispatchHostAction({
    action: ParentUiActionKind.AgentCommandRequested,
    route,
    command: AgentCommand.LogSnapshotGet,
    payload: {},
  });
}

async function dispatchHostAction(action: ParentUiAction): Promise<ParentUiActionResult | null> {
  const context = currentRouteContext();
  const actionWithContext =
    context.selectedChildDeviceId === undefined ? action : { ...action, context };
  try {
    writePortalDevLog(DevLogMessage.PortalCommandSent, {
      [DevLogField.Command]: action.command ?? action.action,
      [DevLogField.ConnectionState]: state.connectionState,
    });
    const result = await bridge.dispatch(actionWithContext);
    state.connectionState = result.connectionState;
    state.commandEnabled = result.connectionState === ParentBridgeConnectionState.Connected;
    state.lastHostMessage = result.message;
    applyParentRouteEvents(state, result.events);
    if (result.snapshot !== null) {
      applyParentRouteSnapshot(state, result.snapshot);
      state.lastHostMessage = result.message;
    }
    refresh();
    await restartRouteSubscription();
    return result;
  } catch (error) {
    state.connectionState = ParentBridgeConnectionState.Error;
    state.commandEnabled = false;
    state.lastHostMessage = error instanceof Error ? error.message : String(error);
    refresh();
    return null;
  }
}

async function restartRouteSubscription(): Promise<void> {
  disposeRouteSubscription();
  await installRouteSubscription(getRoute());
}

async function installRouteSubscription(route: ParentRouteId): Promise<void> {
  const token = routeSubscriptionToken + 1;
  routeSubscriptionToken = token;
  try {
    const unsubscribe = await bridge.subscribe(route, currentRouteContext(), (event) => {
      if (token !== routeSubscriptionToken || event.route !== getRoute()) {
        return;
      }
      writePortalDevLog(DevLogMessage.PortalEventReceived, {
        [DevLogField.Event]: String(event.route),
        [DevLogField.ConnectionState]: event.snapshot.connectionState,
      });
      applyParentSubscriptionEvent(state, event);
      refresh();
    });
    if (token !== routeSubscriptionToken || route !== getRoute()) {
      unsubscribe();
      return;
    }
    activeRouteUnsubscribe = unsubscribe;
  } catch (error) {
    if (token !== routeSubscriptionToken) {
      return;
    }
    state.lastHostMessage = error instanceof Error ? error.message : String(error);
    refresh();
  }
}

function disposeRouteSubscription(): void {
  routeSubscriptionToken += 1;
  activeRouteUnsubscribe?.();
  activeRouteUnsubscribe = null;
}

function currentRouteContext(): ParentRouteContext {
  const selectedChildDeviceId = selectedChildDeviceIdFromManageTargetSelection(readStoredManageTargetSelection());
  return selectedChildDeviceId ? { selectedChildDeviceId } : {};
}

function shouldPrimeDeveloperRoute(route: ParentRouteId): boolean {
  return route === ParentRoute.Commands || route === ParentRoute.Events || route === ParentRoute.Logs;
}

function refresh(): void {
  revision += 1;
  const theme = resolveTheme();
  applyTheme(theme);
  if (hostedPortalDistributionState !== null) {
    root.render(createElement(HostedPortalDistribution, { state: hostedPortalDistributionState }));
    hideAppLoadingAfterPaint();
    return;
  }
  const backgroundDevToolMode = isBackgroundDevToolMode();
  if (backgroundDevToolMode) {
    root.render(createElement(PortalBackgroundDevTool, { initialTheme: theme }));
    hideAppLoadingAfterPaint();
    return;
  }
  root.render(
    createElement(PortalApp, {
      actions,
      revision,
      route: getRoute(),
      state,
      theme,
      onThemeChange: updateTheme,
      onProductSurfaceReady: hideAppLoadingAfterPaint,
      rerender: refresh,
    })
  );
}

function isBackgroundDevToolMode(): boolean {
  return window.location.hash.includes(PortalDom.BackgroundDevToolHashFlag);
}

function updateTheme(theme: PortalThemeValue): void {
  selectTheme(theme);
  refresh();
}

function getRoute(): ParentRouteId {
  const route = parentRouteFromHashPath(window.location.hash);
  if (route !== null) {
    return route;
  }
  replaceHashIfNeeded(ParentRoute.Overview);
  return ParentRoute.Overview;
}

function replaceHashIfNeeded(route: ParentRouteId): void {
  const nextHash = parentRouteHashPath(route);
  if (window.location.hash === nextHash) {
    return;
  }
  window.history.replaceState(null, document.title, nextHash);
}

function requirePortalRoot(): HTMLDivElement {
  const rootElement = document.querySelector<HTMLDivElement>(PortalDom.RootSelector);
  if (rootElement === null) {
    throw new Error(resolvePortalDevText(PortalDevTextToken.RootMissing));
  }
  return rootElement;
}

function installAppLoadingHider(): void {
  (globalThis as unknown as Record<typeof PortalDom.Runtime.HideAppLoading, unknown>)[
    PortalDom.Runtime.HideAppLoading
  ] = () => {
    fadePortalBackgroundBootLayer();
    const loader = document.getElementById(PortalDom.Ids.AppLoading);
    if (loader === null) {
      removePortalBackgroundBootLayer();
      return;
    }
    loader.classList.add(PortalDom.Classes.AppLoadingHide);
    const applyHidden = () => {
      if (!loader.classList.contains(PortalDom.Classes.AppLoadingHidden)) {
        loader.classList.add(PortalDom.Classes.AppLoadingHidden);
      }
      if (loader.isConnected) {
        loader.remove();
      }
      removePortalBackgroundBootLayer();
    };
    loader.addEventListener(
      PortalDom.Events.TransitionEnd,
      (event) => {
        if (event.target === loader) {
          applyHidden();
        }
      },
      { once: true }
    );
    window.setTimeout(applyHidden, APP_LOADING_FADE_FALLBACK_MS);
  };
}

function hideAppLoadingAfterPaint(): void {
  if (appLoadingHideRequested || typeof window === PortalDom.Runtime.Undefined) {
    return;
  }
  appLoadingHideRequested = true;
  window.requestAnimationFrame(() => {
    window.requestAnimationFrame(() => {
      const hideAppLoading = (globalThis as unknown as Record<typeof PortalDom.Runtime.HideAppLoading, unknown>)[
        PortalDom.Runtime.HideAppLoading
      ] as (() => void) | undefined;
      hideAppLoading?.();
    });
  });
}

refresh();
if (hostedPortalDistributionState === null) {
  void loadCurrentRoute();
  window.addEventListener(PortalDom.Events.HashChange, () => {
    void loadCurrentRoute();
  });
  window.onbeforeunload = () => {
    disposeRouteSubscription();
  };
}

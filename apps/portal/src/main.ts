import { createElement } from 'react';
import { createRoot } from 'react-dom/client';
import { DevLogMessage } from '@ocentra-parent/schema-domain/logging-contracts';
import { PortalRoute, type PortalRoute as PortalRouteValue } from '@ocentra-parent/schema-domain/portal-contracts';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/schema-domain/text-portal-dev';
import { PortalDom, type PortalThemeValue } from '@ocentra-parent/portal-domain/contracts';
import { PortalRoutes, portalRouteFromHashPath } from '@ocentra-parent/portal-domain/routes';
import { writePortalDevLog } from './dev-logger';
import { type ParentRouteContext, type ParentUiAction, type ParentUiActionResult } from './generated/parent-ui-bridge';
import { createHostBridge } from './host-bridge';
import { fadePortalBackgroundBootLayer, removePortalBackgroundBootLayer } from './portal-background-boot';
import { PortalBackgroundDevTool } from './PortalBackgroundDevTool';
import { PortalApp } from './PortalApp';
import type { PortalRenderActions } from './portal-actions';
import { applyParentRouteSnapshot, createPortalRuntimeState } from './portal-state';
import { applyTheme, resolveTheme, selectTheme } from './portal-theme';
import './styles.css';
import './portal-unified-chrome.css';
import './styles/deck-frame-fit.css';
import './styles/control-card-frame.css';
import './styles/frame-tuner.css';
import './styles/parent-portal-route.css';

const app = requirePortalRoot();
const root = createRoot(app);
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
      action: 'agent-command-requested',
      route: getRoute(),
      command,
      payload,
    });
  },
  async refreshRouteSnapshot() {
    return dispatchHostAction({
      action: 'refresh-route',
      route: getRoute(),
      payload: {},
    });
  },
  async requestLanPairingBrowserDiscoveryScan() {
    return dispatchHostAction({
      action: 'lan-pairing-browser-discovery-scan-requested',
      route: getRoute(),
      payload: {},
    });
  },
  async requestNetworkFlowReadModelRefresh() {
    return dispatchHostAction({
      action: 'network-flow-read-model-refresh-requested',
      route: getRoute(),
      payload: {},
    });
  },
  async requestTrackingRetentionSettingsWrite() {
    return dispatchHostAction({
      action: 'tracking-retention-settings-write-requested',
      route: getRoute(),
      payload: {},
    });
  },
  async requestScreenSettingsGet(payload) {
    return dispatchHostAction({
      action: 'screen-settings-get-requested',
      route: getRoute(),
      payload,
    });
  },
  async requestScreenSettingsReplace(payload) {
    return dispatchHostAction({
      action: 'screen-settings-replace-requested',
      route: getRoute(),
      payload,
    });
  },
  async requestAppGameAdapterDispatchExecute() {
    return dispatchHostAction({
      action: 'app-game-adapter-dispatch-execute-requested',
      route: getRoute(),
      payload: {},
    });
  },
  async requestAppGameTimerParentPreferenceSetup(payload) {
    return dispatchHostAction({
      action: 'app-game-timer-parent-preference-setup-requested',
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
  state.connectionState = 'connecting';
  refresh();
  try {
    const snapshot = await bridge.loadRoute(route, currentRouteContext());
    if (sequence !== routeLoadSequence) {
      return;
    }
    applyParentRouteSnapshot(state, snapshot);
    refresh();
    await installRouteSubscription(route);
  } catch (error) {
    if (sequence !== routeLoadSequence) {
      return;
    }
    state.connectionState = 'error';
    state.commandEnabled = false;
    state.lastHostMessage = error instanceof Error ? error.message : String(error);
    refresh();
  }
}

async function dispatchHostAction(action: ParentUiAction): Promise<ParentUiActionResult | null> {
  try {
    const result = await bridge.dispatch(action);
    state.connectionState = result.connectionState;
    state.commandEnabled = result.connectionState === 'connected';
    state.lastHostMessage = result.message;
    if (result.snapshot !== null) {
      applyParentRouteSnapshot(state, result.snapshot);
    }
    refresh();
    await restartRouteSubscription();
    return result;
  } catch (error) {
    state.connectionState = 'error';
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

async function installRouteSubscription(route: PortalRouteValue): Promise<void> {
  const token = routeSubscriptionToken + 1;
  routeSubscriptionToken = token;
  try {
    const unsubscribe = await bridge.subscribe(route, currentRouteContext(), (event) => {
      if (token !== routeSubscriptionToken || event.route !== getRoute()) {
        return;
      }
      applyParentRouteSnapshot(state, event.snapshot);
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
  return {};
}

function refresh(): void {
  revision += 1;
  const backgroundDevToolMode = isBackgroundDevToolMode();
  const theme = resolveTheme();
  applyTheme(theme);
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

function getRoute(): PortalRouteValue {
  const route = portalRouteFromHashPath(window.location.hash);
  if (route !== null && PortalRoutes.some((portalRoute) => portalRoute === route)) {
    return route;
  }
  replaceHashIfNeeded(PortalRoute.Overview);
  return PortalRoute.Overview;
}

function replaceHashIfNeeded(route: PortalRouteValue): void {
  const nextHash = `${PortalDom.HashPrefix}${route}`;
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
void loadCurrentRoute();
window.addEventListener(PortalDom.Events.HashChange, () => {
  void loadCurrentRoute();
});
window.addEventListener('beforeunload', () => {
  disposeRouteSubscription();
});

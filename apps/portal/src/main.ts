import { createElement } from 'react';
import { createRoot } from 'react-dom/client';
import { AgentProtocolDefaults, decodeAgentWebSocketUrl } from '@ocentra-parent/agent-protocol-domain/contracts';
import {
  PortalDom,
  PortalEnvironment,
  PortalRoute,
  PortalRouteSchema,
  PortalRoutes,
  PortalText,
  PortalTextToken,
  type PortalRoute as PortalRouteValue,
  type PortalThemeValue,
} from '@ocentra-parent/portal-domain/contracts';
import { DevLogField, DevLogMessage, writePortalDevLog } from './dev-logger';
import { fadePortalBackgroundBootLayer, removePortalBackgroundBootLayer } from './portal-background-boot';
import { PortalBackgroundDevTool } from './PortalBackgroundDevTool';
import { PortalApp } from './PortalApp';
import type { PortalRenderActions } from './portal-actions';
import { createPortalRuntimeState } from './portal-state';
import { applyTheme, resolveTheme, selectTheme } from './portal-theme';
import { connectWebSocket, sendCommand } from './transport';
import './styles.css';
import './portal-unified-chrome.css';
import './styles/deck-frame-fit.css';
import './styles/control-card-frame.css';
import './styles/frame-tuner.css';
import './styles/parent-portal-route.css';

const agentWsUrl = decodeAgentWebSocketUrl(
  import.meta.env[PortalEnvironment.AgentWebSocketUrl] ?? AgentProtocolDefaults.WebSocketUrl
);
const app = requirePortalRoot();
const root = createRoot(app);
const state = createPortalRuntimeState(agentWsUrl);
let revision = 0;
let appLoadingHideRequested = false;
const APP_LOADING_FADE_FALLBACK_MS = 920;

installAppLoadingHider();

writePortalDevLog(DevLogMessage.PortalStarted, {
  [DevLogField.AgentWebSocketUrl]: agentWsUrl,
});

const actions: PortalRenderActions = {
  reconnect() {
    connectWebSocket(state, refresh);
  },
  selectCommandResult(resultEvent) {
    state.selectedCommandResultEvent = resultEvent;
    refresh();
  },
  sendCommand(command, payload) {
    sendCommand(state, refresh, command, payload);
  },
};

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
  const routeHash = window.location.hash.replace(/^#\/?/u, PortalDom.EmptyHashRoute);
  const route = routeHash.split(PortalDom.HashQuerySeparator)[0] ?? PortalDom.EmptyHashRoute;
  const parsedRoute = PortalRouteSchema.safeParse(route);
  if (parsedRoute.success && PortalRoutes.some((portalRoute) => portalRoute === parsedRoute.data)) {
    return parsedRoute.data;
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
    throw new Error(PortalText.Resolve(PortalTextToken.RootMissing));
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
connectWebSocket(state, refresh);
window.addEventListener(PortalDom.Events.HashChange, refresh);

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
} from '@ocentra-parent/portal-domain/contracts';
import { DevLogField, DevLogMessage, writePortalDevLog } from './dev-logger';
import { PortalApp } from './PortalApp';
import type { PortalRenderActions } from './portal-actions';
import { createPortalRuntimeState } from './portal-state';
import { applyTheme, resolveTheme } from './portal-theme';
import { connectWebSocket, sendCommand } from './transport';
import './styles.css';
import './portal-unified-chrome.css';
import './styles/deck-frame-fit.css';
import './styles/control-card-frame.css';
import './styles/frame-tuner.css';

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
  const theme = resolveTheme();
  applyTheme(theme);
  root.render(
    createElement(PortalApp, {
      actions,
      revision,
      route: getRoute(),
      state,
      theme,
      onProductSurfaceReady: hideAppLoadingAfterPaint,
      rerender: refresh,
    })
  );
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
    const loader = document.getElementById(PortalDom.Ids.AppLoading);
    if (loader === null) {
      return;
    }
    loader.classList.add(PortalDom.Classes.AppLoadingHide);
    const applyHidden = () => {
      if (!loader.classList.contains(PortalDom.Classes.AppLoadingHidden)) {
        loader.classList.add(PortalDom.Classes.AppLoadingHidden);
      }
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

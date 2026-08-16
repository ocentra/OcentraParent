import { createElement } from 'react';
import { createRoot } from 'react-dom/client';
import { GeneratedDevLogMessage as DevLogMessage } from '@ocentra-parent/logging-domain/generated/logging-contracts';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/portal-domain/display-text';
import { PortalDom, type PortalThemeValue } from '@ocentra-parent/portal-domain/contracts';
import { writePortalDevLog } from './dev-logger';
import {
  parentRouteFromHashPath,
  parentRouteHashPath,
  ParentRoute,
  type ParentRouteId,
} from '../generated/parent-ui-bridge';
import { createHostBridge } from './host-bridge';
import { fadePortalBackgroundBootLayer, removePortalBackgroundBootLayer } from './portal-background-boot';
import { HostedPortalDistribution, resolveHostedPortalDistributionState } from './hosted-portal-distribution';
import { PortalBackgroundDevTool } from './PortalBackgroundDevTool';
import { PortalApp } from './PortalApp';
import { createPortalRuntimeController } from './portal-runtime-controller';
import { createPortalRuntimeState } from './portal-state';
import type { PortalRenderActions } from './portal-actions';
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
const runtimeController = createPortalRuntimeController({
  bridge,
  state,
  refresh,
  getRoute,
});
let revision = 0;
let appLoadingHideRequested = false;
const APP_LOADING_FADE_FALLBACK_MS = 920;

installAppLoadingHider();

writePortalDevLog(DevLogMessage.PortalStarted, {
  hostBridgeEndpoint: state.agentEndpoint,
});

const actions: PortalRenderActions = {
  ...runtimeController.actions,
};

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
  void runtimeController.start();
  window.addEventListener(PortalDom.Events.HashChange, () => {
    void runtimeController.handleRouteChange();
  });
  window.onbeforeunload = () => {
    runtimeController.dispose();
  };
}

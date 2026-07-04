import { PortalDevTextToken, resolvePortalDevText } from './portal-dev-text';
import { WebviewWindow, getAllWebviewWindows } from '@tauri-apps/api/webviewWindow';
import {
  ParentHostBridgeRuntime,
  ParentRoute,
  parentRouteHashPath,
  type ParentRouteHashPath,
  type ParentRouteId,
} from '../generated/parent-ui-bridge';

const FrameTunerWindowChrome = {
  Height: 900,
  Hash: parentRouteHashPath(ParentRoute.FrameTuner),
  Label: 'portal-app-layout',
  PopupFeatures: 'popup=yes,width=1280,height=900,resizable=yes,scrollbars=yes',
  TauriErrorEvent: 'tauri://error',
  Width: 1280,
} as const;

type ParentDevToolUrl = `${string}${ParentRouteHashPath}`;

export async function openPortalFrameTunerWindow(): Promise<void> {
  const url = parentDevToolUrl(window.location.origin, window.location.pathname, ParentRoute.FrameTuner);
  if (!isTauriRuntime()) {
    openBrowserFrameTunerWindow(url);
    return;
  }
  try {
    const existingWindow = (await getAllWebviewWindows()).find(
      (webviewWindow) => webviewWindow.label === FrameTunerWindowChrome.Label
    );
    if (existingWindow !== undefined) {
      await existingWindow.show();
      await existingWindow.unminimize();
      await existingWindow.setFocus();
      return;
    }
    const webview = new WebviewWindow(FrameTunerWindowChrome.Label, {
      decorations: true,
      height: FrameTunerWindowChrome.Height,
      resizable: true,
      title: resolvePortalDevText(PortalDevTextToken.FrameTuner),
      url,
      width: FrameTunerWindowChrome.Width,
    });
    webview.once(FrameTunerWindowChrome.TauriErrorEvent, () => {
      openBrowserFrameTunerWindow(url);
    });
  } catch {
    openBrowserFrameTunerWindow(url);
  }
}

function isTauriRuntime(): boolean {
  return (
    typeof window !== ParentHostBridgeRuntime.TypeofUndefined &&
    ParentHostBridgeRuntime.TauriInternalWindowKey in window
  );
}

function parentDevToolUrl(origin: string, pathname: string, route: ParentRouteId): ParentDevToolUrl {
  return `${origin}${pathname}${parentRouteHashPath(route)}` as ParentDevToolUrl;
}

function openBrowserFrameTunerWindow(url: ParentDevToolUrl): void {
  const popup = window.open(url, FrameTunerWindowChrome.Label, FrameTunerWindowChrome.PopupFeatures);
  if (popup === null) {
    window.location.hash = FrameTunerWindowChrome.Hash;
    return;
  }
  popup.focus();
}

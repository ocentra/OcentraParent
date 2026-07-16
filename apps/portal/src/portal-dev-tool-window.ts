import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/portal-domain/display-text';
import { PortalDevToolWindow, portalDevToolUrl } from '@ocentra-parent/portal-domain/routes';
import { WebviewWindow, getAllWebviewWindows } from '@tauri-apps/api/webviewWindow';
import { ParentHostBridgeRuntime, ParentRoute } from '../generated/parent-ui-bridge';

export async function openPortalFrameTunerWindow(): Promise<void> {
  const url = portalDevToolUrl(window.location.origin, window.location.pathname, ParentRoute.FrameTuner);
  if (!isTauriRuntime()) {
    openBrowserFrameTunerWindow(url);
    return;
  }
  try {
    const existingWindow = (await getAllWebviewWindows()).find(
      (webviewWindow) => webviewWindow.label === PortalDevToolWindow.FrameTunerLabel
    );
    if (existingWindow !== undefined) {
      await existingWindow.show();
      await existingWindow.unminimize();
      await existingWindow.setFocus();
      return;
    }
    const webview = new WebviewWindow(PortalDevToolWindow.FrameTunerLabel, {
      decorations: true,
      height: PortalDevToolWindow.FrameTunerHeight,
      resizable: true,
      title: resolvePortalDevText(PortalDevTextToken.FrameTuner),
      url,
      width: PortalDevToolWindow.FrameTunerWidth,
    });
    webview.once(PortalDevToolWindow.TauriErrorEvent, () => {
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

function openBrowserFrameTunerWindow(url: ReturnType<typeof portalDevToolUrl>): void {
  const popup = window.open(url, PortalDevToolWindow.FrameTunerLabel, PortalDevToolWindow.PopupFeatures);
  if (popup === null) {
    window.location.hash = PortalDevToolWindow.FrameTunerHash;
    return;
  }
  popup.focus();
}

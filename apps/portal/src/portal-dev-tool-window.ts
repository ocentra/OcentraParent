import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/portal-domain/display-text';
import {
  PortalDevToolWindow,
  portalFrameTunerUrl,
  type PortalDevToolWindowLabel,
} from '@ocentra-parent/portal-domain/routes';
import { WebviewWindow, getAllWebviewWindows } from '@tauri-apps/api/webviewWindow';
import { isParentTauriRuntime } from './tauri-runtime';

export async function openPortalFrameTunerWindow(
  routePath: unknown = PortalDevToolWindow.FrameTunerHash
): Promise<void> {
  const backgroundOnly = routePath === PortalDevToolWindow.BackgroundFrameTunerHash;
  const windowLabel = backgroundOnly
    ? PortalDevToolWindow.BackgroundFrameTunerLabel
    : PortalDevToolWindow.FrameTunerLabel;
  const url = portalFrameTunerUrl(window.location.origin, window.location.pathname, backgroundOnly);
  if (!isParentTauriRuntime()) {
    openBrowserFrameTunerWindow(url, windowLabel, backgroundOnly);
    return;
  }
  try {
    const existingWindow = (await getAllWebviewWindows()).find((webviewWindow) => webviewWindow.label === windowLabel);
    if (existingWindow !== undefined) {
      await existingWindow.show();
      await existingWindow.unminimize();
      await existingWindow.setFocus();
      return;
    }
    const webview = new WebviewWindow(windowLabel, {
      decorations: true,
      height: PortalDevToolWindow.FrameTunerHeight,
      resizable: true,
      title: resolvePortalDevText(PortalDevTextToken.FrameTuner),
      url,
      width: PortalDevToolWindow.FrameTunerWidth,
    });
    webview.once(PortalDevToolWindow.TauriErrorEvent, () => {
      openBrowserFrameTunerWindow(url, windowLabel, backgroundOnly);
    });
  } catch {
    openBrowserFrameTunerWindow(url, windowLabel, backgroundOnly);
  }
}

function openBrowserFrameTunerWindow(
  url: ReturnType<typeof portalFrameTunerUrl>,
  windowLabel: PortalDevToolWindowLabel,
  backgroundOnly: boolean
): void {
  const popup = window.open(url, windowLabel, PortalDevToolWindow.PopupFeatures);
  if (popup === null) {
    window.location.hash = backgroundOnly
      ? PortalDevToolWindow.BackgroundFrameTunerHash
      : PortalDevToolWindow.FrameTunerHash;
    return;
  }
  popup.focus();
}

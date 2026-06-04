import {
  PortalDevToolWindow,
  PortalDom,
  PortalRoute,
  PortalText,
  PortalTextToken,
} from '@ocentra-parent/portal-domain/contracts';

export async function openPortalFrameTunerWindow(): Promise<void> {
  const url = `${window.location.origin}${window.location.pathname}${PortalDom.HashPrefix}${PortalRoute.FrameTuner}`;
  if (!isTauriRuntime()) {
    openBrowserFrameTunerWindow(url);
    return;
  }
  try {
    const { WebviewWindow, getAllWebviewWindows } = await import('@tauri-apps/api/webviewWindow');
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
      title: PortalText.Resolve(PortalTextToken.FrameTuner),
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
  return typeof window !== PortalDom.Runtime.Undefined && PortalDevToolWindow.TauriInternalKey in window;
}

function openBrowserFrameTunerWindow(url: string): void {
  const popup = window.open(url, PortalDevToolWindow.FrameTunerLabel, PortalDevToolWindow.PopupFeatures);
  if (popup === null) {
    window.location.hash = `${PortalDom.HashPrefix}${PortalRoute.FrameTuner}`;
    return;
  }
  popup.focus();
}

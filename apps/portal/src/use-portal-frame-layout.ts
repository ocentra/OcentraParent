import { useEffect, useState } from 'react';
import { PortalFrameTuner } from '@ocentra-parent/portal-domain/frame-tuner';
import { normalizePortalFrameLayout } from './portal-frame-layout-state';
import { DEFAULT_PORTAL_FRAME_LAYOUT, type PortalFrameLayout } from './portal-frame-layout-types';

export function usePortalFrameLayout(
  pollDevLayout: boolean
): readonly [PortalFrameLayout, (layout: PortalFrameLayout) => void] {
  const [layout, setLayout] = useState<PortalFrameLayout>(DEFAULT_PORTAL_FRAME_LAYOUT);

  useEffect(() => {
    let active = true;
    void loadInitialLayouts((url) => loadPortalFrameLayout(url, setLayout, () => active));
    const intervalId = pollDevLayout
      ? window.setInterval(() => {
          void loadPortalFrameLayout(PortalFrameTuner.Api.LayoutEndpoint, setLayout, () => active);
        }, PortalFrameTuner.Timing.LayoutPollMs)
      : undefined;
    return () => {
      active = false;
      if (intervalId !== undefined) {
        window.clearInterval(intervalId);
      }
    };
  }, [pollDevLayout]);

  return [layout, setLayout];
}

async function loadPortalFrameLayout(
  url: (typeof PortalFrameTuner.Api)[keyof typeof PortalFrameTuner.Api],
  setLayout: (layout: PortalFrameLayout) => void,
  isActive: () => boolean
): Promise<void> {
  try {
    const response = await fetch(url, { cache: PortalFrameTuner.FetchCache.NoStore });
    const value = response.ok ? await response.json() : undefined;
    if (isActive()) {
      setLayout(normalizePortalFrameLayout(value));
    }
  } catch {
    return;
  }
}

async function loadInitialLayouts(
  loadLayout: (url: (typeof PortalFrameTuner.Api)[keyof typeof PortalFrameTuner.Api]) => Promise<void>
): Promise<void> {
  await loadLayout(PortalFrameTuner.Api.StaticLayoutAsset);
  await loadLayout(PortalFrameTuner.Api.LayoutEndpoint);
}

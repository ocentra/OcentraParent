import { useEffect, useState } from 'react';
import { PortalFrameTuner } from '@ocentra-parent/portal-domain/contracts';
import { DEFAULT_PORTAL_FRAME_LAYOUT, normalizePortalFrameLayout, type PortalFrameLayout } from './portal-frame-layout';

export function usePortalFrameLayout(
  pollDevLayout: boolean
): readonly [PortalFrameLayout, (layout: PortalFrameLayout) => void] {
  const [layout, setLayout] = useState<PortalFrameLayout>(DEFAULT_PORTAL_FRAME_LAYOUT);

  useEffect(() => {
    let active = true;
    const loadLayout = async (url: (typeof PortalFrameTuner.Api)[keyof typeof PortalFrameTuner.Api]): Promise<void> => {
      await fetch(url, { cache: PortalFrameTuner.FetchCache.NoStore })
        .then((response) => (response.ok ? response.json() : undefined))
        .then((value: unknown) => {
          if (active) {
            setLayout(normalizePortalFrameLayout(value));
          }
        })
        .catch(() => undefined);
    };
    void loadInitialLayouts(loadLayout);
    const intervalId = pollDevLayout
      ? window.setInterval(() => loadLayout(PortalFrameTuner.Api.LayoutEndpoint), PortalFrameTuner.Timing.LayoutPollMs)
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

async function loadInitialLayouts(
  loadLayout: (url: (typeof PortalFrameTuner.Api)[keyof typeof PortalFrameTuner.Api]) => Promise<void>
): Promise<void> {
  await loadLayout(PortalFrameTuner.Api.StaticLayoutAsset);
  await loadLayout(PortalFrameTuner.Api.LayoutEndpoint);
}

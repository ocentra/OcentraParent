import { useState, type ReactElement } from 'react';
import { type PortalDisplayText } from '@ocentra-parent/portal-domain/display-text';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { PortalFrameTuner } from '@ocentra-parent/portal-domain/frame-tuner';
import {
  type PortalAppLayoutSurfaceContentDraft,
  type PortalAppLayoutSurfaceKey,
} from '@ocentra-parent/portal-domain/app-layout';
import { TunerActionButton, TunerTabButton } from './PortalFrameTunerControls';
import { PortalAppLayoutSurfacePanel } from './PortalAppLayoutSurfacePanel';
import {
  normalizePortalFrameLayout,
  resetPortalParentPortalContent,
  resetPortalParentPortalSurface,
  setPortalFrameLayoutValue,
} from './portal-frame-layout-state';
import type { PortalFrameLayout } from './portal-frame-layout-types';
import type { ParentPortalSvgControls } from '../../../vendor/ocentra-parent-core-ui/AppPages/ParentPortal/ParentPortalSvgSurfaceControls';

type PortalFrameTunerRouteProps = {
  readonly layout: PortalFrameLayout;
  readonly onLayoutChange: (layout: PortalFrameLayout) => void;
};

type CommitFrameLayout = (layout: PortalFrameLayout) => void;

type FrameTunerActions = {
  readonly resetParentPortalContent: (surface: PortalAppLayoutSurfaceKey) => void;
  readonly resetParentPortalSurface: (surface: PortalAppLayoutSurfaceKey) => void;
  readonly updateParentPortalContent: (
    surface: PortalAppLayoutSurfaceKey,
    content: PortalAppLayoutSurfaceContentDraft
  ) => void;
  readonly updateParentPortalSurface: (surface: PortalAppLayoutSurfaceKey, controls: ParentPortalSvgControls) => void;
};

const AppLayoutSurfaceTabs = [
  { id: PortalFrameTuner.AppSurface.MainApp, label: PortalFrameTuner.Text.PanelMainApp },
  { id: PortalFrameTuner.AppSurface.ChatInterface, label: PortalFrameTuner.Text.PanelChatInterface },
] as const;

export function PortalFrameTunerRoute({ layout, onLayoutChange }: PortalFrameTunerRouteProps): ReactElement {
  const [activeSurface, setActiveSurface] = useState<PortalAppLayoutSurfaceKey>(PortalFrameTuner.AppSurface.MainApp);
  const [status, setStatus] = useState(PortalFrameTuner.Text.Ready);
  const commitLayout = (nextLayout: PortalFrameLayout): void => {
    onLayoutChange(nextLayout);
    void saveDraftLayout(nextLayout, setStatus);
  };
  const actions = frameTunerActions(layout, commitLayout);

  return (
    <section className={PortalFrameTuner.Classes.TunerStandalone}>
      <div className={PortalFrameTuner.Classes.TunerWorkspace}>
        <aside className={PortalFrameTuner.Classes.TunerInspector}>
          <TunerHeader layout={layout} onLayoutChange={onLayoutChange} setStatus={setStatus} status={status} />
          <SurfaceTabs activeSurface={activeSurface} onSurfaceChange={setActiveSurface} />
          <div className={PortalFrameTuner.Classes.TunerControlGrid}>
            <PortalAppLayoutSurfacePanel
              content={layout.parentPortal.contentDraft[activeSurface]}
              controls={layout.parentPortal[activeSurface]}
              key={activeSurface}
              onContentChange={(content) => actions.updateParentPortalContent(activeSurface, content)}
              onControlsChange={(controls) => actions.updateParentPortalSurface(activeSurface, controls)}
              onResetContent={() => actions.resetParentPortalContent(activeSurface)}
              onResetSurface={() => actions.resetParentPortalSurface(activeSurface)}
              surface={activeSurface}
            />
          </div>
        </aside>
      </div>
    </section>
  );
}

function frameTunerActions(layout: PortalFrameLayout, commitLayout: CommitFrameLayout): FrameTunerActions {
  return {
    resetParentPortalContent: (surface) => commitLayout(resetPortalParentPortalContent(layout, surface)),
    resetParentPortalSurface: (surface) => commitLayout(resetPortalParentPortalSurface(layout, surface)),
    updateParentPortalContent: (surface, content) =>
      commitLayout(
        normalizePortalFrameLayout(
          setPortalFrameLayoutValue(
            layout,
            [PortalFrameTuner.LayoutKey.ParentPortal, PortalFrameTuner.LayoutKey.ContentDraft, surface],
            content
          )
        )
      ),
    updateParentPortalSurface: (surface, controls) =>
      commitLayout(
        normalizePortalFrameLayout(
          setPortalFrameLayoutValue(layout, [PortalFrameTuner.LayoutKey.ParentPortal, surface], controls)
        )
      ),
  };
}

function SurfaceTabs({
  activeSurface,
  onSurfaceChange,
}: {
  readonly activeSurface: PortalAppLayoutSurfaceKey;
  readonly onSurfaceChange: (surface: PortalAppLayoutSurfaceKey) => void;
}): ReactElement {
  return (
    <div className={PortalFrameTuner.Classes.TunerSurfaceTabs} role={PortalDom.Attributes.TabList}>
      {AppLayoutSurfaceTabs.map((surface) => (
        <TunerTabButton
          active={surface.id === activeSurface}
          key={surface.id}
          label={surface.label}
          onClick={() => onSurfaceChange(surface.id)}
        />
      ))}
    </div>
  );
}

function TunerHeader({
  layout,
  onLayoutChange,
  setStatus,
  status,
}: {
  readonly layout: PortalFrameLayout;
  readonly onLayoutChange: (layout: PortalFrameLayout) => void;
  readonly setStatus: (value: PortalDisplayText) => void;
  readonly status: PortalDisplayText;
}): ReactElement {
  return (
    <div className={PortalFrameTuner.Classes.TunerToolbar}>
      <div>
        <h1 className={PortalDom.Classes.PageTitle}>{PortalFrameTuner.Text.RouteTitle}</h1>
        <strong className={PortalFrameTuner.Classes.TunerStatus}>{status}</strong>
      </div>
      <div className={PortalFrameTuner.Classes.TunerActions}>
        <TunerActionButton label={PortalFrameTuner.Text.Save} onClick={() => saveSavedLayout(layout, setStatus)} />
        <TunerActionButton
          label={PortalFrameTuner.Text.Reset}
          onClick={() => {
            void resetToSavedLayout(onLayoutChange, setStatus);
          }}
        />
      </div>
    </div>
  );
}

async function saveDraftLayout(
  layout: PortalFrameLayout,
  setStatus: (value: PortalDisplayText) => void
): Promise<void> {
  const ok = await putLayout(PortalFrameTuner.Api.LayoutEndpoint, layout);
  setStatus(ok ? PortalFrameTuner.Text.Ready : PortalFrameTuner.Text.SaveFailed);
}

async function saveSavedLayout(
  layout: PortalFrameLayout,
  setStatus: (value: PortalDisplayText) => void
): Promise<void> {
  const ok = await putLayout(PortalFrameTuner.Api.SavedLayoutEndpoint, layout);
  setStatus(ok ? PortalFrameTuner.Text.Saved : PortalFrameTuner.Text.SaveFailed);
}

async function resetToSavedLayout(
  onLayoutChange: (layout: PortalFrameLayout) => void,
  setStatus: (value: PortalDisplayText) => void
): Promise<void> {
  const savedLayout = await getLayout(PortalFrameTuner.Api.SavedLayoutEndpoint);
  if (savedLayout === undefined) {
    setStatus(PortalFrameTuner.Text.SaveFailed);
    return;
  }
  onLayoutChange(savedLayout);
  const ok = await putLayout(PortalFrameTuner.Api.LayoutEndpoint, savedLayout);
  setStatus(ok ? PortalFrameTuner.Text.ResetLoaded : PortalFrameTuner.Text.SaveFailed);
}

async function putLayout(
  url: (typeof PortalFrameTuner.Api)[keyof typeof PortalFrameTuner.Api],
  layout: PortalFrameLayout
): Promise<boolean> {
  const response = await fetch(url, {
    body: JSON.stringify(layout, null, 2),
    headers: {
      [PortalFrameTuner.HttpHeader.ContentType]: PortalFrameTuner.ContentType.Json,
    },
    method: PortalFrameTuner.HttpMethod.Put,
  }).catch(() => undefined);
  return response?.ok === true;
}

async function getLayout(
  url: (typeof PortalFrameTuner.Api)[keyof typeof PortalFrameTuner.Api]
): Promise<PortalFrameLayout | undefined> {
  const response = await fetch(url, { cache: PortalFrameTuner.FetchCache.NoStore }).catch(() => undefined);
  if (response?.ok !== true) {
    return undefined;
  }
  return normalizePortalFrameLayout(await response.json());
}

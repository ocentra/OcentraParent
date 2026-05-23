import { useMemo, useState, type ReactElement } from 'react';
import {
  PortalDom,
  PortalFrameTuner,
  type PortalDisplayText,
  type PortalFrameBooleanField,
  type PortalFrameColorField,
  type PortalFrameNumberField,
  type PortalFrameTargetValue,
  type PortalFrameTunerPanelValue,
} from '@ocentra-parent/portal-domain/contracts';
import { TunerActionButton, TunerTabs } from './PortalFrameTunerControls';
import { FrameTunerActivePanel, type SideFrameTargetValue } from './PortalFrameTunerPanels';
import {
  DEFAULT_PORTAL_FRAME_LAYOUT,
  normalizePortalFrameLayout,
  resetPortalCarousel,
  resetPortalGoldenCard,
  resetPortalFrameTarget,
  setPortalFrameLayoutValue,
  type PortalFrameLayout,
} from './portal-frame-layout';

type PortalFrameTunerRouteProps = {
  readonly layout: PortalFrameLayout;
  readonly onLayoutChange: (layout: PortalFrameLayout) => void;
};

type CommitFrameLayout = (layout: PortalFrameLayout) => void;

type FrameTunerActions = {
  readonly resetFrame: (target: PortalFrameTargetValue) => void;
  readonly resetCarousel: () => void;
  readonly resetGoldenCard: () => void;
  readonly resetShell: () => void;
  readonly updateFrame: (target: PortalFrameTargetValue, field: PortalFrameNumberField, value: number) => void;
  readonly updateFrameColor: (target: PortalFrameTargetValue, field: PortalFrameColorField, value: unknown) => void;
  readonly updateFrameValue: (target: PortalFrameTargetValue, path: readonly PropertyKey[], value: unknown) => void;
  readonly updateFrameContentBoolean: (
    target: PortalFrameTargetValue,
    field: PortalFrameBooleanField,
    value: boolean
  ) => void;
  readonly updateFrameContentNumber: (
    target: PortalFrameTargetValue,
    field: PortalFrameNumberField,
    value: number
  ) => void;
  readonly updateGoldenCardBoolean: (field: PortalFrameBooleanField, value: boolean) => void;
  readonly updateGoldenCardNumber: (field: PortalFrameNumberField, value: number) => void;
  readonly updateCarouselNumber: (field: PortalFrameNumberField, value: number) => void;
  readonly updateShell: (field: PortalFrameNumberField, value: number) => void;
};

export function PortalFrameTunerRoute({ layout, onLayoutChange }: PortalFrameTunerRouteProps): ReactElement {
  const [activePanel, setActivePanel] = useState<PortalFrameTunerPanelValue>(PortalFrameTuner.Panel.SidePanel);
  const [sideTarget, setSideTarget] = useState<SideFrameTargetValue>(PortalFrameTuner.FrameTarget.SideTop);
  const [status, setStatus] = useState(PortalFrameTuner.Text.Ready);
  const jsonPreview = useMemo(() => JSON.stringify(layout, null, 2), [layout]);
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
          <TunerTabs activePanel={activePanel} onPanelChange={setActivePanel} />
          <div className={PortalFrameTuner.Classes.TunerControlGrid}>
            <FrameTunerActivePanel
              activePanel={activePanel}
              jsonPreview={jsonPreview}
              layout={layout}
              resetCarousel={actions.resetCarousel}
              resetFrame={actions.resetFrame}
              resetGoldenCard={actions.resetGoldenCard}
              resetShell={actions.resetShell}
              sideTarget={sideTarget}
              setSideTarget={setSideTarget}
              updateFrame={actions.updateFrame}
              updateFrameColor={actions.updateFrameColor}
              updateFrameValue={actions.updateFrameValue}
              updateFrameContentBoolean={actions.updateFrameContentBoolean}
              updateFrameContentNumber={actions.updateFrameContentNumber}
              updateGoldenCardBoolean={actions.updateGoldenCardBoolean}
              updateGoldenCardNumber={actions.updateGoldenCardNumber}
              updateCarouselNumber={actions.updateCarouselNumber}
              updateShell={actions.updateShell}
            />
          </div>
        </aside>
      </div>
    </section>
  );
}

function frameTunerActions(layout: PortalFrameLayout, commitLayout: CommitFrameLayout): FrameTunerActions {
  return {
    resetFrame: (target) => commitLayout(resetPortalFrameTarget(layout, target)),
    resetCarousel: () => commitLayout(resetPortalCarousel(layout)),
    resetGoldenCard: () => commitLayout(resetPortalGoldenCard(layout)),
    resetShell: () => commitLayout(normalizePortalFrameLayout({ ...layout, shell: DEFAULT_PORTAL_FRAME_LAYOUT.shell })),
    updateFrame: (target, field, value) =>
      commitLayout(normalizePortalFrameLayout(setPortalFrameLayoutValue(layout, [target, ...field.path], value))),
    updateFrameColor: (target, field, value) =>
      commitLayout(normalizePortalFrameLayout(setPortalFrameLayoutValue(layout, [target, ...field.path], value))),
    updateFrameValue: (target, path, value) =>
      commitLayout(normalizePortalFrameLayout(setPortalFrameLayoutValue(layout, [target, ...path], value))),
    updateFrameContentBoolean: (target, field, value) =>
      commitLayout(
        normalizePortalFrameLayout(
          setPortalFrameLayoutValue(layout, [PortalFrameTuner.LayoutKey.Content, target, ...field.path], value)
        )
      ),
    updateFrameContentNumber: (target, field, value) =>
      commitLayout(
        normalizePortalFrameLayout(
          setPortalFrameLayoutValue(layout, [PortalFrameTuner.LayoutKey.Content, target, ...field.path], value)
        )
      ),
    updateGoldenCardBoolean: (field, value) =>
      commitLayout(
        normalizePortalFrameLayout(
          setPortalFrameLayoutValue(layout, [PortalFrameTuner.LayoutKey.GoldenCard, ...field.path], value)
        )
      ),
    updateGoldenCardNumber: (field, value) =>
      commitLayout(
        normalizePortalFrameLayout(
          setPortalFrameLayoutValue(layout, [PortalFrameTuner.LayoutKey.GoldenCard, ...field.path], value)
        )
      ),
    updateCarouselNumber: (field, value) =>
      commitLayout(
        normalizePortalFrameLayout(
          setPortalFrameLayoutValue(layout, [PortalFrameTuner.LayoutKey.Carousel, ...field.path], value)
        )
      ),
    updateShell: (field, value) =>
      commitLayout(
        normalizePortalFrameLayout(
          setPortalFrameLayoutValue(layout, [PortalFrameTuner.LayoutKey.Shell, ...field.path], value)
        )
      ),
  };
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
        <TunerActionButton
          active={allContentBooleanEnabled(layout, PortalFrameTuner.LayoutKey.ShowContent)}
          label={PortalFrameTuner.Text.ToggleContent}
          onClick={() =>
            toggleBulkContentBoolean(layout, onLayoutChange, setStatus, PortalFrameTuner.LayoutKey.ShowContent)
          }
        />
        <TunerActionButton
          active={allContentBooleanEnabled(layout, PortalFrameTuner.LayoutKey.ShowFrameBounds)}
          label={PortalFrameTuner.Text.ToggleFrameBounds}
          onClick={() =>
            toggleBulkContentBoolean(layout, onLayoutChange, setStatus, PortalFrameTuner.LayoutKey.ShowFrameBounds)
          }
        />
        <TunerActionButton
          active={allContentBooleanEnabled(layout, PortalFrameTuner.LayoutKey.ShowContentBounds)}
          label={PortalFrameTuner.Text.ToggleContentBounds}
          onClick={() =>
            toggleBulkContentBoolean(layout, onLayoutChange, setStatus, PortalFrameTuner.LayoutKey.ShowContentBounds)
          }
        />
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

function toggleBulkContentBoolean(
  layout: PortalFrameLayout,
  onLayoutChange: (layout: PortalFrameLayout) => void,
  setStatus: (value: PortalDisplayText) => void,
  key: BulkContentBooleanKey
): void {
  const value = !allContentBooleanEnabled(layout, key);
  const frameContentLayout = BulkContentTargets.reduce(
    (current, target) => setPortalFrameLayoutValue(current, [PortalFrameTuner.LayoutKey.Content, target, key], value),
    layout
  );
  const nextLayout = normalizePortalFrameLayout(
    setPortalFrameLayoutValue(frameContentLayout, [PortalFrameTuner.LayoutKey.GoldenCard, key], value)
  );
  onLayoutChange(nextLayout);
  void saveDraftLayout(nextLayout, setStatus);
}

function allContentBooleanEnabled(layout: PortalFrameLayout, key: BulkContentBooleanKey): boolean {
  return BulkContentTargets.every((target) => layout.content[target][key]) && layout.goldenCard[key];
}

type BulkContentBooleanKey =
  | typeof PortalFrameTuner.LayoutKey.ShowContent
  | typeof PortalFrameTuner.LayoutKey.ShowContentBounds
  | typeof PortalFrameTuner.LayoutKey.ShowFrameBounds;

const BulkContentTargets = [
  PortalFrameTuner.FrameTarget.SideTop,
  PortalFrameTuner.FrameTarget.SideBottom,
  PortalFrameTuner.FrameTarget.Main,
] as const;

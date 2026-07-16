import { useMemo, useState, type ReactElement } from 'react';
import { type PortalDisplayText } from '@ocentra-parent/portal-domain/display-text';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import {
  PortalFrameTuner,
  type PortalFrameColorField,
  type PortalFrameNumberField,
} from '@ocentra-parent/portal-domain/frame-tuner';
import {
  type PortalAppLayoutSurfaceContentDraft,
  type PortalAppLayoutSurfaceKey,
} from '@ocentra-parent/portal-domain/app-layout';
import {
  PARENT_PORTAL_SVG_COLOR_FIELDS,
  PARENT_PORTAL_SVG_NUMBER_FIELDS,
  normalizeParentPortalSvgControls,
  type ParentPortalSvgControls,
} from '../../../vendor/ocentra-parent-core-ui/AppPages/ParentPortal/ParentPortalSvgSurfaceControls';
import {
  ColorControlSection,
  NumberControlSection,
  TunerActionButton,
  TunerTabButton,
  classNames,
} from './PortalFrameTunerControls';
import { PortalAppLayoutContentPanel } from './PortalAppLayoutContentPanel';
import { DEFAULT_PORTAL_FRAME_LAYOUT } from './portal-frame-layout-types';

type PortalAppLayoutSurfacePanelProps = {
  readonly content: PortalAppLayoutSurfaceContentDraft;
  readonly controls: ParentPortalSvgControls;
  readonly onContentChange: (content: PortalAppLayoutSurfaceContentDraft) => void;
  readonly onControlsChange: (controls: ParentPortalSvgControls) => void;
  readonly onResetContent: () => void;
  readonly onResetSurface: () => void;
  readonly surface: PortalAppLayoutSurfaceKey;
};

type LayoutRegion = (typeof PortalFrameTuner.AppLayoutRegion)[keyof typeof PortalFrameTuner.AppLayoutRegion];
type LayoutSection = (typeof PortalFrameTuner.AppLayoutSection)[keyof typeof PortalFrameTuner.AppLayoutSection];
type AppLayoutFieldKeyValue =
  (typeof PortalFrameTuner.AppLayoutFieldKey)[keyof typeof PortalFrameTuner.AppLayoutFieldKey];
type PortalAppLayoutSurfaceLabels = {
  readonly activeSectionLabel: PortalDisplayText;
  readonly panelLabel: PortalDisplayText;
  readonly surfaceLabel: PortalDisplayText;
};

const fieldKey = PortalFrameTuner.AppLayoutFieldKey;

const SideTopKeys = [
  fieldKey.LayoutOuterPad,
  fieldKey.LayoutLeftW,
  fieldKey.LayoutTopY,
  fieldKey.LayoutHeaderH,
  fieldKey.LayoutGap,
] as const;
const SideBottomKeys = [
  fieldKey.LayoutLeftW,
  fieldKey.LayoutBottomY,
  fieldKey.LayoutBottomH,
  fieldKey.LayoutGap,
  fieldKey.ChromeRowHeight,
  fieldKey.ChromeRowGap,
] as const;
const MainTopKeys = [
  fieldKey.LayoutMainY,
  fieldKey.LayoutTabsY,
  fieldKey.LayoutTabsH,
  fieldKey.LayoutHeaderH,
  fieldKey.LayoutRightW,
  fieldKey.LayoutGap,
] as const;
const MainBottomKeys = [
  fieldKey.LayoutBottomY,
  fieldKey.LayoutBottomH,
  fieldKey.LayoutGap,
  fieldKey.LayoutOuterPad,
] as const;
const ChromeKeys = [
  fieldKey.CanvasWidth,
  fieldKey.CanvasHeight,
  fieldKey.ChromePanelCut,
  fieldKey.ChromePanelStrokeWidth,
  fieldKey.ChromePanelInnerInset,
  fieldKey.ChromeHoverPad,
  fieldKey.ChromeGlowOpacity,
  fieldKey.ChromeButtonArrowWidth,
  fieldKey.ChromeRowHeight,
  fieldKey.ChromeRowGap,
  fieldKey.ChromeAvatarRadius,
] as const;

export function PortalAppLayoutSurfacePanel({
  content,
  controls,
  onContentChange,
  onControlsChange,
  onResetContent,
  onResetSurface,
  surface,
}: PortalAppLayoutSurfacePanelProps): ReactElement {
  const [region, setRegion] = useState<LayoutRegion>(PortalFrameTuner.AppLayoutRegion.SidePanel);
  const [section, setSection] = useState<LayoutSection>(PortalFrameTuner.AppLayoutSection.Top);
  const defaultControls = DEFAULT_PORTAL_FRAME_LAYOUT.parentPortal[surface];
  const mainTopLabel =
    surface === PortalFrameTuner.AppSurface.ChatInterface
      ? PortalFrameTuner.Text.AppLayoutTopChoices
      : PortalFrameTuner.Text.AppLayoutTopCards;
  const labels = surfaceLabelsFor(surface, region, section, mainTopLabel);
  const activeNumberFields = useMemo(() => numberFieldsFor(keysFor(region, section)), [region, section]);
  const updateNumber = (field: PortalFrameNumberField, value: number): void => {
    onControlsChange(nextControlsWithNumber(controls, field, value));
  };
  const updateColor = (field: PortalFrameColorField, value: unknown): void => {
    onControlsChange(nextControlsWithColor(controls, field, value));
  };
  const selectRegion = (nextRegion: LayoutRegion): void => {
    setRegion(nextRegion);
    setSection(PortalFrameTuner.AppLayoutSection.Top);
  };
  return (
    <div className={PortalFrameTuner.Classes.TunerAppLayoutSurface}>
      <PortalAppLayoutSurfaceHeader labels={labels} onResetSurface={onResetSurface} />
      <PortalAppLayoutRegionTabs region={region} onSelectRegion={selectRegion} />
      <PortalAppLayoutSectionTabs
        mainTopLabel={mainTopLabel}
        onSelectSection={setSection}
        region={region}
        section={section}
      />
      <PortalAppLayoutPane
        activeNumberFields={activeNumberFields}
        activeSectionLabel={labels.activeSectionLabel}
        content={content}
        controls={controls}
        defaultControls={defaultControls}
        mainTopLabel={mainTopLabel}
        onColorChange={updateColor}
        onContentChange={onContentChange}
        onNumberChange={updateNumber}
        onResetContent={onResetContent}
        section={section}
      />
    </div>
  );
}

function PortalAppLayoutSurfaceHeader({
  labels,
  onResetSurface,
}: {
  readonly labels: PortalAppLayoutSurfaceLabels;
  readonly onResetSurface: () => void;
}): ReactElement {
  return (
    <div className={PortalFrameTuner.Classes.TunerAppLayoutHeader}>
      <div className={PortalFrameTuner.Classes.TunerHierarchy}>
        <span>{labels.surfaceLabel}</span>
        <span>{labels.panelLabel}</span>
        <span>{labels.activeSectionLabel}</span>
      </div>
      <div className={PortalFrameTuner.Classes.TunerActions}>
        <TunerActionButton label={PortalFrameTuner.Text.ResetSurface} onClick={onResetSurface} />
      </div>
    </div>
  );
}

function PortalAppLayoutRegionTabs({
  region,
  onSelectRegion,
}: {
  readonly region: LayoutRegion;
  readonly onSelectRegion: (region: LayoutRegion) => void;
}): ReactElement {
  return (
    <div
      className={classNames(PortalFrameTuner.Classes.TunerTabs, PortalFrameTuner.Classes.TunerPanelTabs)}
      role={PortalDom.Attributes.TabList}
    >
      <TunerTabButton
        active={region === PortalFrameTuner.AppLayoutRegion.SidePanel}
        label={PortalFrameTuner.Text.AppLayoutSidePanel}
        onClick={() => onSelectRegion(PortalFrameTuner.AppLayoutRegion.SidePanel)}
      />
      <TunerTabButton
        active={region === PortalFrameTuner.AppLayoutRegion.MainPanel}
        label={PortalFrameTuner.Text.AppLayoutMainPanel}
        onClick={() => onSelectRegion(PortalFrameTuner.AppLayoutRegion.MainPanel)}
      />
    </div>
  );
}

function PortalAppLayoutSectionTabs({
  mainTopLabel,
  onSelectSection,
  region,
  section,
}: {
  readonly mainTopLabel: PortalDisplayText;
  readonly onSelectSection: (section: LayoutSection) => void;
  readonly region: LayoutRegion;
  readonly section: LayoutSection;
}): ReactElement {
  return (
    <div
      className={classNames(PortalFrameTuner.Classes.TunerTabs, PortalFrameTuner.Classes.TunerSectionTabs)}
      role={PortalDom.Attributes.TabList}
    >
      <TunerTabButton
        active={section === PortalFrameTuner.AppLayoutSection.Top}
        label={sectionLabelFor(region, PortalFrameTuner.AppLayoutSection.Top, mainTopLabel)}
        onClick={() => onSelectSection(PortalFrameTuner.AppLayoutSection.Top)}
      />
      <TunerTabButton
        active={section === PortalFrameTuner.AppLayoutSection.Bottom}
        label={sectionLabelFor(region, PortalFrameTuner.AppLayoutSection.Bottom, mainTopLabel)}
        onClick={() => onSelectSection(PortalFrameTuner.AppLayoutSection.Bottom)}
      />
      <TunerTabButton
        active={section === PortalFrameTuner.AppLayoutSection.Chrome}
        label={PortalFrameTuner.Text.AppLayoutChrome}
        onClick={() => onSelectSection(PortalFrameTuner.AppLayoutSection.Chrome)}
      />
      <TunerTabButton
        active={section === PortalFrameTuner.AppLayoutSection.Colors}
        label={PortalFrameTuner.Text.AppLayoutColors}
        onClick={() => onSelectSection(PortalFrameTuner.AppLayoutSection.Colors)}
      />
      <TunerTabButton
        active={section === PortalFrameTuner.AppLayoutSection.Content}
        label={PortalFrameTuner.Text.AppLayoutContent}
        onClick={() => onSelectSection(PortalFrameTuner.AppLayoutSection.Content)}
      />
    </div>
  );
}

function PortalAppLayoutPane({
  activeNumberFields,
  activeSectionLabel,
  content,
  controls,
  defaultControls,
  mainTopLabel,
  onColorChange,
  onContentChange,
  onNumberChange,
  onResetContent,
  section,
}: {
  readonly activeNumberFields: readonly PortalFrameNumberField[];
  readonly activeSectionLabel: PortalDisplayText;
  readonly content: PortalAppLayoutSurfaceContentDraft;
  readonly controls: ParentPortalSvgControls;
  readonly defaultControls: ParentPortalSvgControls;
  readonly mainTopLabel: PortalDisplayText;
  readonly onColorChange: (field: PortalFrameColorField, value: unknown) => void;
  readonly onContentChange: (content: PortalAppLayoutSurfaceContentDraft) => void;
  readonly onNumberChange: (field: PortalFrameNumberField, value: number) => void;
  readonly onResetContent: () => void;
  readonly section: LayoutSection;
}): ReactElement {
  return (
    <div className={PortalFrameTuner.Classes.TunerAppLayoutPane}>
      {section === PortalFrameTuner.AppLayoutSection.Colors ? (
        <ColorControlSection
          defaultRoot={defaultControls}
          fields={colorFields()}
          onChange={onColorChange}
          root={controls}
          title={PortalFrameTuner.Text.AppLayoutColors}
        />
      ) : null}
      {section === PortalFrameTuner.AppLayoutSection.Content ? (
        <PortalAppLayoutContentPanel
          content={content}
          mainTopLabel={mainTopLabel}
          onContentChange={onContentChange}
          onReset={onResetContent}
        />
      ) : null}
      {section !== PortalFrameTuner.AppLayoutSection.Colors && section !== PortalFrameTuner.AppLayoutSection.Content ? (
        <NumberControlSection
          defaultRoot={defaultControls}
          fields={activeNumberFields}
          onChange={onNumberChange}
          root={controls}
          title={
            section === PortalFrameTuner.AppLayoutSection.Chrome
              ? PortalFrameTuner.Text.AppLayoutChrome
              : activeSectionLabel
          }
        />
      ) : null}
    </div>
  );
}

function surfaceLabelsFor(
  surface: PortalAppLayoutSurfaceKey,
  region: LayoutRegion,
  section: LayoutSection,
  mainTopLabel: PortalDisplayText
): PortalAppLayoutSurfaceLabels {
  return {
    activeSectionLabel: sectionLabelFor(region, section, mainTopLabel),
    panelLabel:
      region === PortalFrameTuner.AppLayoutRegion.MainPanel
        ? PortalFrameTuner.Text.AppLayoutMainPanel
        : PortalFrameTuner.Text.AppLayoutSidePanel,
    surfaceLabel:
      surface === PortalFrameTuner.AppSurface.ChatInterface
        ? PortalFrameTuner.Text.PanelChatInterface
        : PortalFrameTuner.Text.PanelMainApp,
  };
}

function sectionLabelFor(
  region: LayoutRegion,
  section: LayoutSection,
  mainTopLabel: PortalDisplayText
): PortalDisplayText {
  if (section === PortalFrameTuner.AppLayoutSection.Top) {
    return region === PortalFrameTuner.AppLayoutRegion.MainPanel
      ? mainTopLabel
      : PortalFrameTuner.Text.AppLayoutSideTop;
  }
  if (section === PortalFrameTuner.AppLayoutSection.Bottom) {
    return region === PortalFrameTuner.AppLayoutRegion.MainPanel
      ? PortalFrameTuner.Text.AppLayoutMainBottom
      : PortalFrameTuner.Text.AppLayoutSideBottom;
  }
  if (section === PortalFrameTuner.AppLayoutSection.Chrome) {
    return PortalFrameTuner.Text.AppLayoutChrome;
  }
  if (section === PortalFrameTuner.AppLayoutSection.Colors) {
    return PortalFrameTuner.Text.AppLayoutColors;
  }
  return PortalFrameTuner.Text.AppLayoutContent;
}

function keysFor(region: LayoutRegion, section: LayoutSection): readonly AppLayoutFieldKeyValue[] {
  if (section === PortalFrameTuner.AppLayoutSection.Chrome) {
    return ChromeKeys;
  }
  if (region === PortalFrameTuner.AppLayoutRegion.SidePanel) {
    return section === PortalFrameTuner.AppLayoutSection.Bottom ? SideBottomKeys : SideTopKeys;
  }
  return section === PortalFrameTuner.AppLayoutSection.Bottom ? MainBottomKeys : MainTopKeys;
}

function numberFieldsFor(keys: readonly AppLayoutFieldKeyValue[]): readonly PortalFrameNumberField[] {
  const allowed = new Set(keys);
  const separator = PortalFrameTuner.AppLayoutFieldSeparator;
  return PARENT_PORTAL_SVG_NUMBER_FIELDS.filter((field) =>
    allowed.has(`${field.group}${separator}${field.key}` as AppLayoutFieldKeyValue)
  ).map((field) => ({
    path: [field.group, field.key],
    label: field.label as PortalDisplayText,
    min: field.min,
    max: field.max,
    step: field.step,
  }));
}

function colorFields(): readonly PortalFrameColorField[] {
  return PARENT_PORTAL_SVG_COLOR_FIELDS.map((field) => ({
    path: [PortalFrameTuner.FrameSection.Colors, field.key],
    label: field.label as PortalDisplayText,
  }));
}

function nextControlsWithNumber(
  controls: ParentPortalSvgControls,
  field: PortalFrameNumberField,
  value: number
): ParentPortalSvgControls {
  const group = field.path[0] as Exclude<keyof ParentPortalSvgControls, typeof PortalFrameTuner.FrameSection.Colors>;
  const key = field.path[1] as keyof ParentPortalSvgControls[typeof group];
  return normalizeParentPortalSvgControls({
    ...controls,
    [group]: {
      ...controls[group],
      [key]: value,
    },
  });
}

function nextControlsWithColor(
  controls: ParentPortalSvgControls,
  field: PortalFrameColorField,
  value: unknown
): ParentPortalSvgControls {
  const colorSection = PortalFrameTuner.FrameSection.Colors;
  const key = field.path[1] as keyof ParentPortalSvgControls[typeof colorSection];
  return normalizeParentPortalSvgControls({
    ...controls,
    [colorSection]: {
      ...controls[colorSection],
      [key]: typeof value === PortalFrameTuner.ValueType.String ? value : controls[colorSection][key],
    },
  });
}

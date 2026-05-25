import { useState, type ReactElement } from 'react';
import {
  PortalDom,
  PortalCarouselContentNumberFields,
  PortalCarouselFrameNumberFields,
  PortalCarouselRailNumberFields,
  PortalFrameColorFields,
  PortalFrameContentBooleanFields,
  PortalFrameContentNumberFields,
  PortalFrameGeometryNumberFields,
  PortalGoldenCardBooleanFields,
  PortalGoldenCardContentNumberFields,
  PortalGoldenCardFrameNumberFields,
  PortalFrameInnerEdgeNumberFields,
  PortalFrameInnerGapNumberFields,
  PortalFrameInnerSegmentNumberFields,
  PortalFrameInnerShapeNumberFields,
  PortalFrameOuterEdgeNumberFields,
  PortalFrameOuterGapNumberFields,
  PortalFrameOuterSegmentNumberFields,
  PortalFrameOuterShapeNumberFields,
  PortalFrameShellNumberFields,
  PortalFrameSlotNumberFields,
  PortalFrameTuner,
  PortalFrameTunerFrameSections,
  type PortalDisplayText,
  type PortalFrameBooleanField,
  type PortalFrameColorField,
  type PortalFrameNumberField,
  type PortalFrameTargetValue,
  type PortalFrameTunerFrameSectionValue,
  type PortalFrameTunerPanelValue,
} from '@ocentra-parent/portal-domain/contracts';
import { GoldenFrameForeignObjectControlsRoute } from '../../../vendor/ocentra-games-core-ui/AppPages/ParentPortal/ParentPortalGoldenFrameForeignObject';
import {
  BooleanControlSection,
  ColorControlSection,
  NumberControlSection,
  TunerActionButton,
  TunerTabButton,
  classNames,
} from './PortalFrameTunerControls';
import {
  DEFAULT_PORTAL_FRAME_LAYOUT,
  frameContentTarget,
  frameTargetControls,
  valueAt,
  type PortalFrameLayout,
} from './portal-frame-layout';

export type SideFrameTargetValue =
  | typeof PortalFrameTuner.FrameTarget.SideTop
  | typeof PortalFrameTuner.FrameTarget.SideBottom;

type ActivePanelProps = {
  readonly activePanel: PortalFrameTunerPanelValue;
  readonly jsonPreview: ReturnType<typeof JSON.stringify>;
  readonly layout: PortalFrameLayout;
  readonly resetCarousel: () => void;
  readonly resetFrame: (target: PortalFrameTargetValue) => void;
  readonly resetGoldenCard: () => void;
  readonly resetShell: () => void;
  readonly sideTarget: SideFrameTargetValue;
  readonly setSideTarget: (target: SideFrameTargetValue) => void;
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

export function FrameTunerActivePanel({
  activePanel,
  jsonPreview,
  layout,
  resetCarousel,
  resetFrame,
  resetGoldenCard,
  resetShell,
  sideTarget,
  setSideTarget,
  updateFrame,
  updateFrameColor,
  updateFrameValue,
  updateFrameContentBoolean,
  updateFrameContentNumber,
  updateGoldenCardBoolean,
  updateGoldenCardNumber,
  updateCarouselNumber,
  updateShell,
}: ActivePanelProps): ReactElement {
  if (activePanel === PortalFrameTuner.Panel.Shell) {
    return <ShellPanel layout={layout} onReset={resetShell} updateShell={updateShell} />;
  }
  if (activePanel === PortalFrameTuner.Panel.Preview) {
    return <JsonSettings jsonPreview={jsonPreview} />;
  }
  if (activePanel === PortalFrameTuner.Panel.Carousel) {
    return <CarouselPanel layout={layout} onReset={resetCarousel} updateCarouselNumber={updateCarouselNumber} />;
  }
  if (activePanel === PortalFrameTuner.Panel.GoldenCard) {
    return (
      <GoldenCardPanel
        layout={layout}
        onReset={resetGoldenCard}
        updateGoldenCardBoolean={updateGoldenCardBoolean}
        updateGoldenCardNumber={updateGoldenCardNumber}
      />
    );
  }
  if (activePanel === PortalFrameTuner.Panel.SidePanel) {
    return (
      <SidePanelControls
        layout={layout}
        resetFrame={resetFrame}
        sideTarget={sideTarget}
        setSideTarget={setSideTarget}
        updateFrame={updateFrame}
        updateFrameColor={updateFrameColor}
        updateFrameContentBoolean={updateFrameContentBoolean}
        updateFrameContentNumber={updateFrameContentNumber}
        updateFrameValue={updateFrameValue}
      />
    );
  }
  return (
    <MainPanelControls
      layout={layout}
      resetFrame={resetFrame}
      updateFrame={updateFrame}
      updateFrameColor={updateFrameColor}
      updateFrameContentBoolean={updateFrameContentBoolean}
      updateFrameContentNumber={updateFrameContentNumber}
      updateFrameValue={updateFrameValue}
    />
  );
}

function ShellPanel({
  layout,
  onReset,
  updateShell,
}: {
  readonly layout: PortalFrameLayout;
  readonly onReset: () => void;
  readonly updateShell: (field: PortalFrameNumberField, value: number) => void;
}): ReactElement {
  return (
    <ControlTargetPanel onReset={onReset} resetLabel={PortalFrameTuner.Text.ResetShell}>
      <NumberControlSection
        defaultRoot={DEFAULT_PORTAL_FRAME_LAYOUT.shell}
        fields={PortalFrameShellNumberFields}
        onChange={updateShell}
        root={layout.shell}
        title={PortalFrameTuner.Text.ShellSection}
      />
    </ControlTargetPanel>
  );
}

function CarouselPanel({
  layout,
  onReset,
  updateCarouselNumber,
}: {
  readonly layout: PortalFrameLayout;
  readonly onReset: () => void;
  readonly updateCarouselNumber: (field: PortalFrameNumberField, value: number) => void;
}): ReactElement {
  return (
    <ControlTargetPanel onReset={onReset} resetLabel={PortalFrameTuner.Text.ResetCarousel}>
      <NumberControlSection
        defaultRoot={DEFAULT_PORTAL_FRAME_LAYOUT.carousel}
        fields={PortalCarouselFrameNumberFields}
        onChange={updateCarouselNumber}
        root={layout.carousel}
        title={PortalFrameTuner.Text.CarouselFrameFit}
      />
      <NumberControlSection
        defaultRoot={DEFAULT_PORTAL_FRAME_LAYOUT.carousel}
        fields={PortalCarouselContentNumberFields}
        onChange={updateCarouselNumber}
        root={layout.carousel}
        title={PortalFrameTuner.Text.CarouselContentFit}
      />
      <NumberControlSection
        defaultRoot={DEFAULT_PORTAL_FRAME_LAYOUT.carousel}
        fields={PortalCarouselRailNumberFields}
        onChange={updateCarouselNumber}
        root={layout.carousel}
        title={PortalFrameTuner.Text.CarouselRailFit}
      />
    </ControlTargetPanel>
  );
}

function GoldenCardPanel({
  layout,
  onReset,
  updateGoldenCardBoolean,
  updateGoldenCardNumber,
}: {
  readonly layout: PortalFrameLayout;
  readonly onReset: () => void;
  readonly updateGoldenCardBoolean: (field: PortalFrameBooleanField, value: boolean) => void;
  readonly updateGoldenCardNumber: (field: PortalFrameNumberField, value: number) => void;
}): ReactElement {
  return (
    <ControlTargetPanel onReset={onReset} resetLabel={PortalFrameTuner.Text.ResetGoldenCard}>
      <section
        className={[
          PortalFrameTuner.Classes.TunerControlPanel,
          PortalFrameTuner.Classes.TunerGoldenCardOriginalControls,
        ].join(PortalDom.Classes.ClassNameSeparator)}
      >
        <GoldenFrameForeignObjectControlsRoute />
      </section>
      <BooleanControlSection
        defaultRoot={DEFAULT_PORTAL_FRAME_LAYOUT.goldenCard}
        fields={PortalGoldenCardBooleanFields}
        onChange={updateGoldenCardBoolean}
        root={layout.goldenCard}
        title={PortalFrameTuner.Text.BoundsGroup}
      />
      <NumberControlSection
        defaultRoot={DEFAULT_PORTAL_FRAME_LAYOUT.goldenCard}
        fields={PortalGoldenCardFrameNumberFields}
        onChange={updateGoldenCardNumber}
        root={layout.goldenCard}
        title={PortalFrameTuner.Text.GoldenCardFit}
      />
      <NumberControlSection
        defaultRoot={DEFAULT_PORTAL_FRAME_LAYOUT.goldenCard}
        fields={PortalGoldenCardContentNumberFields}
        onChange={updateGoldenCardNumber}
        root={layout.goldenCard}
        title={PortalFrameTuner.Text.GoldenCardContent}
      />
    </ControlTargetPanel>
  );
}

function SidePanelControls({
  layout,
  resetFrame,
  sideTarget,
  setSideTarget,
  updateFrame,
  updateFrameColor,
  updateFrameContentBoolean,
  updateFrameContentNumber,
  updateFrameValue,
}: {
  readonly layout: PortalFrameLayout;
  readonly resetFrame: (target: PortalFrameTargetValue) => void;
  readonly sideTarget: SideFrameTargetValue;
  readonly setSideTarget: (target: SideFrameTargetValue) => void;
  readonly updateFrame: (target: PortalFrameTargetValue, field: PortalFrameNumberField, value: number) => void;
  readonly updateFrameColor: (target: PortalFrameTargetValue, field: PortalFrameColorField, value: unknown) => void;
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
  readonly updateFrameValue: (target: PortalFrameTargetValue, path: readonly PropertyKey[], value: unknown) => void;
}): ReactElement {
  const resetLabel =
    sideTarget === PortalFrameTuner.FrameTarget.SideTop
      ? PortalFrameTuner.Text.ResetSideTop
      : PortalFrameTuner.Text.ResetSideBottom;
  const colorTitle =
    sideTarget === PortalFrameTuner.FrameTarget.SideTop
      ? PortalFrameTuner.Text.SideTopColors
      : PortalFrameTuner.Text.SideBottomColors;
  return (
    <ControlTargetPanel onReset={() => resetFrame(sideTarget)} resetLabel={resetLabel}>
      <TargetTabs activeTarget={sideTarget} onTargetChange={setSideTarget} />
      <FrameControlGroup
        colorTitle={colorTitle}
        layout={layout}
        target={sideTarget}
        updateFrame={updateFrame}
        updateFrameColor={updateFrameColor}
        updateFrameValue={updateFrameValue}
        updateFrameContentBoolean={updateFrameContentBoolean}
        updateFrameContentNumber={updateFrameContentNumber}
      />
    </ControlTargetPanel>
  );
}

function MainPanelControls({
  layout,
  resetFrame,
  updateFrame,
  updateFrameColor,
  updateFrameContentBoolean,
  updateFrameContentNumber,
  updateFrameValue,
}: {
  readonly layout: PortalFrameLayout;
  readonly resetFrame: (target: PortalFrameTargetValue) => void;
  readonly updateFrame: (target: PortalFrameTargetValue, field: PortalFrameNumberField, value: number) => void;
  readonly updateFrameColor: (target: PortalFrameTargetValue, field: PortalFrameColorField, value: unknown) => void;
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
  readonly updateFrameValue: (target: PortalFrameTargetValue, path: readonly PropertyKey[], value: unknown) => void;
}): ReactElement {
  return (
    <ControlTargetPanel
      onReset={() => resetFrame(PortalFrameTuner.FrameTarget.Main)}
      resetLabel={PortalFrameTuner.Text.ResetMain}
    >
      <FrameControlGroup
        colorTitle={PortalFrameTuner.Text.MainColors}
        layout={layout}
        target={PortalFrameTuner.FrameTarget.Main}
        updateFrame={updateFrame}
        updateFrameColor={updateFrameColor}
        updateFrameValue={updateFrameValue}
        updateFrameContentBoolean={updateFrameContentBoolean}
        updateFrameContentNumber={updateFrameContentNumber}
      />
    </ControlTargetPanel>
  );
}

function ControlTargetPanel({
  children,
  onReset,
  resetLabel,
}: {
  readonly children: ReactElement | readonly ReactElement[];
  readonly onReset: () => void;
  readonly resetLabel: PortalDisplayText;
}): ReactElement {
  return (
    <>
      <div className={PortalFrameTuner.Classes.TunerActions}>
        <TunerActionButton label={resetLabel} onClick={onReset} />
      </div>
      {children}
    </>
  );
}

function TargetTabs({
  activeTarget,
  onTargetChange,
}: {
  readonly activeTarget: SideFrameTargetValue;
  readonly onTargetChange: (target: SideFrameTargetValue) => void;
}): ReactElement {
  return (
    <div className={PortalFrameTuner.Classes.TunerTabs} role={PortalDom.Attributes.TabList}>
      <TunerTabButton
        active={activeTarget === PortalFrameTuner.FrameTarget.SideTop}
        label={PortalFrameTuner.Text.SideTopTab}
        onClick={() => onTargetChange(PortalFrameTuner.FrameTarget.SideTop)}
      />
      <TunerTabButton
        active={activeTarget === PortalFrameTuner.FrameTarget.SideBottom}
        label={PortalFrameTuner.Text.SideBottomTab}
        onClick={() => onTargetChange(PortalFrameTuner.FrameTarget.SideBottom)}
      />
    </div>
  );
}

function FrameControlGroup({
  colorTitle,
  layout,
  target,
  updateFrame,
  updateFrameColor,
  updateFrameValue,
  updateFrameContentBoolean,
  updateFrameContentNumber,
}: {
  readonly colorTitle: PortalDisplayText;
  readonly layout: PortalFrameLayout;
  readonly target: PortalFrameTargetValue;
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
}): ReactElement {
  const [activeSection, setActiveSection] = useState<PortalFrameTunerFrameSectionValue>(
    PortalFrameTuner.FrameSection.OuterAnchors
  );
  const root = frameTargetControls(layout, target);
  const defaultRoot = frameTargetControls(DEFAULT_PORTAL_FRAME_LAYOUT, target);
  const contentRoot = frameContentTarget(layout, target);
  const defaultContentRoot = frameContentTarget(DEFAULT_PORTAL_FRAME_LAYOUT, target);
  return (
    <>
      <FrameSectionTabs activeSection={activeSection} onSectionChange={setActiveSection} />
      {activeSection === PortalFrameTuner.FrameSection.Content ? (
        <>
          <BooleanControlSection
            defaultRoot={defaultContentRoot}
            fields={PortalFrameContentBooleanFields}
            onChange={(field, value) => updateFrameContentBoolean(target, field, value)}
            root={contentRoot}
            title={PortalFrameTuner.Text.BoundsGroup}
          />
          <NumberControlSection
            defaultRoot={defaultContentRoot}
            fields={PortalFrameContentNumberFields}
            onChange={(field, value) => updateFrameContentNumber(target, field, value)}
            root={contentRoot}
            title={PortalFrameTuner.Text.ContentFitGroup}
          />
        </>
      ) : null}
      {activeSection === PortalFrameTuner.FrameSection.Viewport ? (
        <NumberControlSection
          defaultRoot={defaultRoot}
          fields={fieldsIn(PortalFrameGeometryNumberFields, PortalFrameTuner.LayoutKey.ViewBox)}
          onChange={(field, value) => updateFrame(target, field, value)}
          root={root}
          title={PortalFrameTuner.Text.ViewportGroup}
        />
      ) : null}
      {activeSection === PortalFrameTuner.FrameSection.Placement ? (
        <NumberControlSection
          defaultRoot={defaultRoot}
          fields={fieldsIn(PortalFrameGeometryNumberFields, PortalFrameTuner.LayoutKey.FrameGroup)}
          onChange={(field, value) => updateFrame(target, field, value)}
          root={root}
          title={PortalFrameTuner.Text.FrameGroup}
        />
      ) : null}
      {activeSection === PortalFrameTuner.FrameSection.OuterAnchors ? (
        <>
          <OrientationControlSection
            onChange={(value) => updateFrameValue(target, [PortalFrameTuner.LayoutKey.Orientation], value)}
            root={root}
          />
          <NumberControlSection
            defaultRoot={defaultRoot}
            fields={fieldsIn(PortalFrameGeometryNumberFields, PortalFrameTuner.LayoutKey.FrameSpace)}
            onChange={(field, value) => updateFrame(target, field, value)}
            root={root}
            title={PortalFrameTuner.Text.FrameSpaceGroup}
          />
          <NumberControlSection
            defaultRoot={defaultContentRoot}
            fields={PortalFrameSlotNumberFields}
            onChange={(field, value) => updateFrameContentNumber(target, field, value)}
            root={contentRoot}
            title={PortalFrameTuner.Text.FrameSlotGroup}
          />
          <NumberControlSection
            defaultRoot={defaultRoot}
            fields={fieldsIn(PortalFrameGeometryNumberFields, PortalFrameTuner.LayoutKey.OuterAnchor)}
            onChange={(field, value) => updateFrame(target, field, value)}
            root={root}
            title={PortalFrameTuner.Text.OuterAnchorGroup}
          />
          <NumberControlSection
            defaultRoot={defaultRoot}
            fields={fieldsIn(PortalFrameGeometryNumberFields, PortalFrameTuner.LayoutKey.FrameGroup)}
            onChange={(field, value) => updateFrame(target, field, value)}
            root={root}
            title={PortalFrameTuner.Text.FrameGroup}
          />
        </>
      ) : null}
      {activeSection === PortalFrameTuner.FrameSection.InnerAnchors ? (
        <NumberControlSection
          defaultRoot={defaultRoot}
          fields={fieldsIn(PortalFrameGeometryNumberFields, PortalFrameTuner.LayoutKey.InnerAnchor)}
          onChange={(field, value) => updateFrame(target, field, value)}
          root={root}
          title={PortalFrameTuner.Text.InnerAnchorGroup}
        />
      ) : null}
      {activeSection === PortalFrameTuner.FrameSection.OuterFrame ? (
        <NumberControlSection
          defaultRoot={defaultRoot}
          fields={PortalFrameOuterShapeNumberFields}
          onChange={(field, value) => updateFrame(target, field, value)}
          root={root}
          title={PortalFrameTuner.Text.OuterFrameGroup}
        />
      ) : null}
      {activeSection === PortalFrameTuner.FrameSection.OuterEdges ? (
        <NumberControlSection
          defaultRoot={defaultRoot}
          fields={PortalFrameOuterEdgeNumberFields}
          onChange={(field, value) => updateFrame(target, field, value)}
          root={root}
          title={PortalFrameTuner.Text.OuterEdgeGroup}
        />
      ) : null}
      {activeSection === PortalFrameTuner.FrameSection.OuterSegments ? (
        <NumberControlSection
          defaultRoot={defaultRoot}
          fields={PortalFrameOuterSegmentNumberFields}
          onChange={(field, value) => updateFrame(target, field, value)}
          root={root}
          title={PortalFrameTuner.Text.OuterSegmentGroup}
        />
      ) : null}
      {activeSection === PortalFrameTuner.FrameSection.OuterGaps ? (
        <NumberControlSection
          defaultRoot={defaultRoot}
          fields={PortalFrameOuterGapNumberFields}
          onChange={(field, value) => updateFrame(target, field, value)}
          root={root}
          title={PortalFrameTuner.Text.OuterGapGroup}
        />
      ) : null}
      {activeSection === PortalFrameTuner.FrameSection.InnerFrame ? (
        <NumberControlSection
          defaultRoot={defaultRoot}
          fields={PortalFrameInnerShapeNumberFields}
          onChange={(field, value) => updateFrame(target, field, value)}
          root={root}
          title={PortalFrameTuner.Text.InnerFrameGroup}
        />
      ) : null}
      {activeSection === PortalFrameTuner.FrameSection.InnerEdges ? (
        <NumberControlSection
          defaultRoot={defaultRoot}
          fields={PortalFrameInnerEdgeNumberFields}
          onChange={(field, value) => updateFrame(target, field, value)}
          root={root}
          title={PortalFrameTuner.Text.InnerEdgeGroup}
        />
      ) : null}
      {activeSection === PortalFrameTuner.FrameSection.InnerSegments ? (
        <NumberControlSection
          defaultRoot={defaultRoot}
          fields={PortalFrameInnerSegmentNumberFields}
          onChange={(field, value) => updateFrame(target, field, value)}
          root={root}
          title={PortalFrameTuner.Text.InnerSegmentGroup}
        />
      ) : null}
      {activeSection === PortalFrameTuner.FrameSection.InnerGaps ? (
        <NumberControlSection
          defaultRoot={defaultRoot}
          fields={PortalFrameInnerGapNumberFields}
          onChange={(field, value) => updateFrame(target, field, value)}
          root={root}
          title={PortalFrameTuner.Text.InnerGapGroup}
        />
      ) : null}
      {activeSection === PortalFrameTuner.FrameSection.Colors ? (
        <ColorControlSection
          defaultRoot={defaultRoot}
          fields={PortalFrameColorFields}
          onChange={(field, value) => updateFrameColor(target, field, value)}
          root={root}
          title={colorTitle}
        />
      ) : null}
    </>
  );
}

function OrientationControlSection({
  onChange,
  root,
}: {
  readonly onChange: (value: (typeof PortalFrameTuner.Orientation)[keyof typeof PortalFrameTuner.Orientation]) => void;
  readonly root: unknown;
}): ReactElement {
  const orientation = valueAt(root, [PortalFrameTuner.LayoutKey.Orientation]);
  const activeOrientation =
    orientation === PortalFrameTuner.Orientation.Landscape
      ? PortalFrameTuner.Orientation.Landscape
      : PortalFrameTuner.Orientation.Portrait;
  return (
    <section className={PortalFrameTuner.Classes.TunerControlPanel}>
      <h3>{PortalFrameTuner.Text.OrientationGroup}</h3>
      <div className={PortalFrameTuner.Classes.TunerActions}>
        <TunerActionButton
          active={activeOrientation === PortalFrameTuner.Orientation.Portrait}
          label={PortalFrameTuner.Text.Portrait}
          onClick={() => onChange(PortalFrameTuner.Orientation.Portrait)}
        />
        <TunerActionButton
          active={activeOrientation === PortalFrameTuner.Orientation.Landscape}
          label={PortalFrameTuner.Text.Landscape}
          onClick={() => onChange(PortalFrameTuner.Orientation.Landscape)}
        />
      </div>
    </section>
  );
}

function FrameSectionTabs({
  activeSection,
  onSectionChange,
}: {
  readonly activeSection: PortalFrameTunerFrameSectionValue;
  readonly onSectionChange: (section: PortalFrameTunerFrameSectionValue) => void;
}): ReactElement {
  return (
    <div
      className={classNames(PortalFrameTuner.Classes.TunerTabs, PortalFrameTuner.Classes.TunerSectionTabs)}
      role={PortalDom.Attributes.TabList}
    >
      {PortalFrameTunerFrameSections.map((section) => (
        <TunerTabButton
          active={section.id === activeSection}
          key={section.id}
          label={section.label}
          onClick={() => onSectionChange(section.id)}
        />
      ))}
    </div>
  );
}

function fieldsIn(
  fields: readonly PortalFrameNumberField[],
  key: (typeof PortalFrameTuner.LayoutKey)[keyof typeof PortalFrameTuner.LayoutKey]
): readonly PortalFrameNumberField[] {
  return fields.filter((field) => field.path[0] === key);
}

function JsonSettings({ jsonPreview }: { readonly jsonPreview: ReturnType<typeof JSON.stringify> }): ReactElement {
  return (
    <section className={PortalFrameTuner.Classes.TunerControlPanel}>
      <h3>{PortalFrameTuner.Text.PanelPreview}</h3>
      <strong>{PortalFrameTuner.Text.JsonTitle}</strong>
      <textarea className={PortalFrameTuner.Classes.TunerJson} readOnly={true} value={jsonPreview} />
    </section>
  );
}

import { PortalTheme, type PortalThemeValue } from '@ocentra-parent/portal-domain/contracts';
import type { ReactElement, ReactNode } from 'react';
import type { PortalBackgroundTunerControlState } from './portal-background-tuner-control-state';
import { PortalBackgroundTunerPanelClasses } from './portal-background-tuner-controls-panels-style';

export function PortalBackgroundTunerHeader({
  dirty,
  onOpenChange,
  open,
  status,
}: {
  readonly dirty: boolean;
  readonly onOpenChange: (open: boolean) => void;
  readonly open: boolean;
  readonly status: string;
}): ReactElement {
  return (
    <div className={PortalBackgroundTunerPanelClasses.header}>
      <div>
        <div className={PortalBackgroundTunerPanelClasses.headerTitle}>BG tuner</div>
        <div className={PortalBackgroundTunerPanelClasses.status}>{dirty ? 'Draft changes' : status}</div>
      </div>
      <SmallButton onClick={() => onOpenChange(!open)}>{open ? 'Hide' : 'Show'}</SmallButton>
    </div>
  );
}

export function PortalBackgroundTunerEditor({
  controls,
  dirty,
  onPreviewThemeChange,
  onResetAll,
  onResetTheme,
  onSave,
  previewTheme,
}: {
  readonly controls: PortalBackgroundTunerControlState;
  readonly dirty: boolean;
  readonly onPreviewThemeChange: (theme: PortalThemeValue) => void;
  readonly onResetAll: () => void;
  readonly onResetTheme: () => void;
  readonly onSave: () => void;
  readonly previewTheme: PortalThemeValue;
}): ReactElement {
  return (
    <div className={PortalBackgroundTunerPanelClasses.editor}>
      <PortalBackgroundTunerActions
        dirty={dirty}
        onPreviewThemeChange={onPreviewThemeChange}
        onResetAll={onResetAll}
        onResetTheme={onResetTheme}
        onSave={onSave}
        previewTheme={previewTheme}
      />
      <PortalBackgroundTunerGrid controls={controls} />
      <PortalBackgroundTunerJson controls={controls} />
    </div>
  );
}

function PortalBackgroundTunerActions({
  dirty,
  onPreviewThemeChange,
  onResetAll,
  onResetTheme,
  onSave,
  previewTheme,
}: {
  readonly dirty: boolean;
  readonly onPreviewThemeChange: (theme: PortalThemeValue) => void;
  readonly onResetAll: () => void;
  readonly onResetTheme: () => void;
  readonly onSave: () => void;
  readonly previewTheme: PortalThemeValue;
}): ReactElement {
  return (
    <div className={PortalBackgroundTunerPanelClasses.actions}>
      <SmallButton active={previewTheme === PortalTheme.Dark} onClick={() => onPreviewThemeChange(PortalTheme.Dark)}>
        Dark
      </SmallButton>
      <SmallButton active={previewTheme === PortalTheme.Light} onClick={() => onPreviewThemeChange(PortalTheme.Light)}>
        Light
      </SmallButton>
      <SmallButton active={dirty} onClick={onSave}>
        Save
      </SmallButton>
      <SmallButton onClick={onResetTheme}>Reset theme</SmallButton>
      <SmallButton onClick={onResetAll}>Reset all</SmallButton>
    </div>
  );
}

function PortalBackgroundTunerGrid({
  controls,
}: {
  readonly controls: PortalBackgroundTunerControlState;
}): ReactElement {
  return (
    <div className={PortalBackgroundTunerPanelClasses.grid}>
      <HexControls controls={controls} />
      <BgControls controls={controls} />
      <LightControls controls={controls} />
      <MoreLightControls controls={controls} />
    </div>
  );
}

function HexControls({ controls }: { readonly controls: PortalBackgroundTunerControlState }): ReactElement {
  return (
    <ControlColumn title="Hex">
      <RangeInput
        label="Size"
        max={80}
        min={20}
        onChange={(hexRadius) => controls.updateConfig({ hexRadius })}
        value={controls.config.hexRadius}
      />
      <RangeInput
        label="Gap"
        max={20}
        min={0}
        onChange={(gap) => controls.updateConfig({ gap })}
        step={0.5}
        value={controls.config.gap}
      />
      <RangeInput
        label="Line"
        max={3}
        min={0.5}
        onChange={(hexStrokeWidth) => controls.updateConfig({ hexStrokeWidth })}
        step={0.1}
        value={controls.config.hexStrokeWidth}
      />
      <RangeInput
        label="Opacity"
        max={0.5}
        min={0.05}
        onChange={(hexOpacity) => controls.updateConfig({ hexOpacity })}
        step={0.01}
        value={controls.config.hexOpacity}
      />
      <ColorInput
        label="Hex Color"
        onChange={(hexStroke) => controls.updateActivePalette({ hexStroke })}
        value={controls.activePalette.hexStroke}
      />
    </ControlColumn>
  );
}

function BgControls({ controls }: { readonly controls: PortalBackgroundTunerControlState }): ReactElement {
  return (
    <ControlColumn title="BG">
      <ColorInput
        label="Base 1"
        onChange={(bgBaseStart) => controls.updateActivePalette({ bgBaseStart })}
        value={controls.activePalette.bgBaseStart}
      />
      <ColorInput
        label="Base 2"
        onChange={(bgBaseMid) => controls.updateActivePalette({ bgBaseMid })}
        value={controls.activePalette.bgBaseMid}
      />
      <ColorInput
        label="Base 3"
        onChange={(bgBaseEnd) => controls.updateActivePalette({ bgBaseEnd })}
        value={controls.activePalette.bgBaseEnd}
      />
      <ColorInput
        label="Vignette"
        onChange={(vignetteCenter) => controls.updateActivePalette({ vignetteCenter })}
        value={controls.activePalette.vignetteCenter}
      />
    </ControlColumn>
  );
}

function LightControls({ controls }: { readonly controls: PortalBackgroundTunerControlState }): ReactElement {
  return (
    <ControlColumn title="Lights">
      <RangeInput
        label="Strength"
        max={2}
        min={0}
        onChange={(lightStrength) => controls.updateConfig({ lightStrength })}
        step={0.05}
        value={controls.config.lightStrength}
      />
      <RangeInput
        label="Blob Blur"
        max={140}
        min={30}
        onChange={(blobBlur) => controls.updateConfig({ blobBlur })}
        value={controls.config.blobBlur}
      />
      <RangeInput
        label="Beam Blur"
        max={60}
        min={8}
        onChange={(beamBlur) => controls.updateConfig({ beamBlur })}
        value={controls.config.beamBlur}
      />
      <ColorInput
        label="Blob 1"
        onChange={(value) => controls.updateBlobColor(0, value)}
        value={controls.config.blobColors[0]}
      />
      <ColorInput
        label="Blob 2"
        onChange={(value) => controls.updateBlobColor(1, value)}
        value={controls.config.blobColors[1]}
      />
      <ColorInput
        label="Blob 3"
        onChange={(value) => controls.updateBlobColor(2, value)}
        value={controls.config.blobColors[2]}
      />
    </ControlColumn>
  );
}

function MoreLightControls({ controls }: { readonly controls: PortalBackgroundTunerControlState }): ReactElement {
  return (
    <ControlColumn title="More Light">
      <ColorInput
        label="Blob 4"
        onChange={(value) => controls.updateBlobColor(3, value)}
        value={controls.config.blobColors[3]}
      />
      <ColorInput
        label="Blob 5"
        onChange={(value) => controls.updateBlobColor(4, value)}
        value={controls.config.blobColors[4]}
      />
      <ColorInput
        label="Blob 6"
        onChange={(value) => controls.updateBlobColor(5, value)}
        value={controls.config.blobColors[5]}
      />
      <ColorInput
        label="Beam 1"
        onChange={(value) => controls.updateBeamColor(0, value)}
        value={controls.config.beamColors[0]}
      />
      <ColorInput
        label="Beam 2"
        onChange={(value) => controls.updateBeamColor(1, value)}
        value={controls.config.beamColors[1]}
      />
      <ColorInput
        label="Beam 3"
        onChange={(value) => controls.updateBeamColor(2, value)}
        value={controls.config.beamColors[2]}
      />
    </ControlColumn>
  );
}

function PortalBackgroundTunerJson({
  controls,
}: {
  readonly controls: PortalBackgroundTunerControlState;
}): ReactElement {
  return (
    <details className={PortalBackgroundTunerPanelClasses.json}>
      <summary className={PortalBackgroundTunerPanelClasses.jsonSummary}>JSON</summary>
      <div className={PortalBackgroundTunerPanelClasses.jsonActions}>
        <SmallButton onClick={controls.copyJson}>{controls.copied ? 'Copied' : 'Copy JSON'}</SmallButton>
        <SmallButton onClick={controls.selectJson}>Select JSON</SmallButton>
        <div
          className={
            controls.copyError
              ? PortalBackgroundTunerPanelClasses.jsonError
              : PortalBackgroundTunerPanelClasses.jsonStatus
          }
        >
          {controls.copyError || 'One save stores dark and light background values.'}
        </div>
      </div>
      <textarea
        onFocus={(event) => {
          event.currentTarget.select();
          event.currentTarget.setSelectionRange(0, event.currentTarget.value.length);
        }}
        readOnly={true}
        ref={controls.jsonRef}
        className={PortalBackgroundTunerPanelClasses.jsonTextarea}
        value={controls.jsonText}
      />
    </details>
  );
}

function ControlColumn({ children, title }: { readonly children: ReactNode; readonly title: string }): ReactElement {
  return (
    <div className={PortalBackgroundTunerPanelClasses.column}>
      <div className={PortalBackgroundTunerPanelClasses.columnTitle}>{title}</div>
      {children}
    </div>
  );
}

function ColorInput({
  label,
  onChange,
  value,
}: {
  readonly label: string;
  readonly onChange: (value: string) => void;
  readonly value: string;
}): ReactElement {
  return (
    <label className={PortalBackgroundTunerPanelClasses.colorLabel}>
      <span>{label}</span>
      <input
        onChange={(event) => onChange(event.target.value)}
        className={PortalBackgroundTunerPanelClasses.colorInput}
        type="color"
        value={value}
      />
    </label>
  );
}

function RangeInput({
  label,
  max,
  min,
  onChange,
  step,
  value,
}: {
  readonly label: string;
  readonly max: number;
  readonly min: number;
  readonly onChange: (value: number) => void;
  readonly step?: number;
  readonly value: number;
}): ReactElement {
  return (
    <label className={PortalBackgroundTunerPanelClasses.rangeLabel}>
      <span className={PortalBackgroundTunerPanelClasses.rangeValue}>
        {label}: {Number.isInteger(value) ? value : value.toFixed(2)}
      </span>
      <input
        max={max}
        min={min}
        onChange={(event) => onChange(Number(event.target.value))}
        step={step}
        className={PortalBackgroundTunerPanelClasses.rangeInput}
        type="range"
        value={value}
      />
    </label>
  );
}

function SmallButton({
  active = false,
  children,
  onClick,
}: {
  readonly active?: boolean;
  readonly children: ReactNode;
  readonly onClick: () => void;
}): ReactElement {
  return (
    <button
      className={active ? PortalBackgroundTunerPanelClasses.buttonActive : PortalBackgroundTunerPanelClasses.button}
      onClick={onClick}
      type="button"
    >
      {children}
    </button>
  );
}

import { useState, type ChangeEvent, type ReactElement } from 'react';
import { type PortalDisplayText } from '@ocentra-parent/portal-domain/display-text';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { PortalFrameTuner } from '@ocentra-parent/portal-domain/frame-tuner';
import {
  createPortalAppLayoutButtonDraft,
  createPortalAppLayoutFoldoutDraft,
  type PortalAppLayoutButtonDraft,
  type PortalAppLayoutContentAreaKey,
  type PortalAppLayoutFoldoutDraft,
  type PortalAppLayoutSurfaceContentDraft,
  type PortalAppLayoutTone,
} from '@ocentra-parent/portal-domain/app-layout';
import { TunerActionButton, TunerTabButton } from './PortalFrameTunerControls';

type PortalAppLayoutContentPanelProps = {
  readonly content: PortalAppLayoutSurfaceContentDraft;
  readonly mainTopLabel: PortalDisplayText;
  readonly onContentChange: (content: PortalAppLayoutSurfaceContentDraft) => void;
  readonly onReset: () => void;
};

const ToneValues = [
  PortalFrameTuner.AppLayoutTone.Cyan,
  PortalFrameTuner.AppLayoutTone.Gold,
  PortalFrameTuner.AppLayoutTone.Purple,
  PortalFrameTuner.AppLayoutTone.Red,
  PortalFrameTuner.AppLayoutTone.Muted,
] as const;

export function PortalAppLayoutContentPanel({
  content,
  mainTopLabel,
  onContentChange,
  onReset,
}: PortalAppLayoutContentPanelProps): ReactElement {
  const [area, setArea] = useState<PortalAppLayoutContentAreaKey>(PortalFrameTuner.AppContentArea.SidePanelFoldouts);
  const [selectedFoldoutIndex, setSelectedFoldoutIndex] = useState(0);
  const foldouts = content[area];
  const safeIndex = foldouts.length === 0 ? -1 : Math.min(selectedFoldoutIndex, foldouts.length - 1);
  const selectedFoldout = safeIndex >= 0 ? foldouts[safeIndex] : undefined;
  const areaTabs = contentAreaTabs(mainTopLabel);
  const updateFoldouts = (nextFoldouts: readonly PortalAppLayoutFoldoutDraft[]): void => {
    onContentChange({ ...content, [area]: nextFoldouts });
  };
  const updateFoldout = (index: number, patch: Partial<PortalAppLayoutFoldoutDraft>): void => {
    updateFoldouts(
      foldouts.map((foldout, foldoutIndex) => (foldoutIndex === index ? { ...foldout, ...patch } : foldout))
    );
  };
  const addFoldout = (): void => {
    const next = [...foldouts, createPortalAppLayoutFoldoutDraft(area, foldouts.length)];
    updateFoldouts(next);
    setSelectedFoldoutIndex(next.length - 1);
  };
  const removeFoldout = (index: number): void => {
    const next = foldouts.filter((_, foldoutIndex) => foldoutIndex !== index);
    updateFoldouts(next);
    setSelectedFoldoutIndex(Math.max(0, index - 1));
  };
  return (
    <section className={PortalFrameTuner.Classes.TunerControlPanel}>
      <div className={PortalFrameTuner.Classes.TunerActions}>
        <TunerActionButton label={PortalFrameTuner.Text.AddFoldout} onClick={addFoldout} />
        <TunerActionButton label={PortalFrameTuner.Text.ResetContentDraft} onClick={onReset} />
      </div>
      <p className={PortalFrameTuner.Classes.TunerContentSummary}>{PortalFrameTuner.Text.ContentAuthoringSummary}</p>
      <div className={PortalFrameTuner.Classes.TunerTabs} role={PortalDom.Attributes.TabList}>
        {areaTabs.map((tab) => (
          <TunerTabButton
            active={area === tab.id}
            key={tab.id}
            label={tab.label}
            onClick={() => {
              setArea(tab.id);
              setSelectedFoldoutIndex(0);
            }}
          />
        ))}
      </div>
      <div className={PortalFrameTuner.Classes.TunerContentEditor}>
        <div className={PortalFrameTuner.Classes.TunerContentList}>
          {foldouts.map((foldout, index) => (
            <button
              className={PortalFrameTuner.Classes.TunerButton}
              key={foldout.id}
              onClick={() => setSelectedFoldoutIndex(index)}
              type={PortalDom.ButtonType.Button}
            >
              {foldout.label}
            </button>
          ))}
        </div>
        {selectedFoldout ? (
          <FoldoutEditor
            foldout={selectedFoldout}
            onFoldoutChange={(patch) => updateFoldout(safeIndex, patch)}
            onRemove={() => removeFoldout(safeIndex)}
          />
        ) : null}
      </div>
    </section>
  );
}

function contentAreaTabs(mainTopLabel: PortalDisplayText): readonly {
  readonly id: PortalAppLayoutContentAreaKey;
  readonly label: PortalDisplayText;
}[] {
  return [
    { id: PortalFrameTuner.AppContentArea.SidePanelFoldouts, label: PortalFrameTuner.Text.SidePanelFoldouts },
    { id: PortalFrameTuner.AppContentArea.MainPanelTop, label: mainTopLabel },
    { id: PortalFrameTuner.AppContentArea.MainPanelBottom, label: PortalFrameTuner.Text.MainPanelBottomContent },
  ];
}

function FoldoutEditor({
  foldout,
  onFoldoutChange,
  onRemove,
}: {
  readonly foldout: PortalAppLayoutFoldoutDraft;
  readonly onFoldoutChange: (patch: Partial<PortalAppLayoutFoldoutDraft>) => void;
  readonly onRemove: () => void;
}): ReactElement {
  const updateButtons = (buttons: readonly PortalAppLayoutButtonDraft[]): void => onFoldoutChange({ buttons });
  const updateButton = (index: number, patch: Partial<PortalAppLayoutButtonDraft>): void => {
    updateButtons(
      foldout.buttons.map((button, buttonIndex) => (buttonIndex === index ? { ...button, ...patch } : button))
    );
  };
  const addButton = (): void => {
    updateButtons([
      ...foldout.buttons,
      createPortalAppLayoutButtonDraft(foldout.id, foldout.buttons.length, foldout.tone),
    ]);
  };
  return (
    <div className={PortalFrameTuner.Classes.TunerContentItem}>
      <div className={PortalFrameTuner.Classes.TunerContentFields}>
        <TextInput
          label={PortalFrameTuner.Text.FoldoutName}
          value={foldout.label}
          onChange={(label) => onFoldoutChange({ label })}
        />
        <ToneSelect value={foldout.tone} onChange={(tone) => onFoldoutChange({ tone })} />
      </div>
      <div className={PortalFrameTuner.Classes.TunerActions}>
        <TunerActionButton label={PortalFrameTuner.Text.AddButton} onClick={addButton} />
        <TunerActionButton label={PortalFrameTuner.Text.Remove} onClick={onRemove} />
      </div>
      <div className={PortalFrameTuner.Classes.TunerContentButtons}>
        {foldout.buttons.map((button, index) => (
          <div className={PortalFrameTuner.Classes.TunerContentFields} key={button.id}>
            <TextInput
              label={PortalFrameTuner.Text.ButtonLabel}
              value={button.label}
              onChange={(label) => updateButton(index, { label })}
            />
            <TextInput
              label={PortalFrameTuner.Text.ButtonDetail}
              value={button.detail}
              onChange={(detail) => updateButton(index, { detail })}
            />
            <TextInput
              label={PortalFrameTuner.Text.ButtonRoute}
              value={button.routePath}
              onChange={(routePath) => updateButton(index, { routePath })}
            />
            <TextInput
              label={PortalFrameTuner.Text.ButtonIcon}
              value={button.icon}
              onChange={(icon) => updateButton(index, { icon })}
            />
            <ToneSelect value={button.tone} onChange={(tone) => updateButton(index, { tone })} />
            <TunerActionButton
              label={PortalFrameTuner.Text.Remove}
              onClick={() => updateButtons(foldout.buttons.filter((_, buttonIndex) => buttonIndex !== index))}
            />
          </div>
        ))}
      </div>
    </div>
  );
}

function TextInput({
  label,
  onChange,
  value,
}: {
  readonly label: PortalDisplayText;
  readonly onChange: (value: string) => void;
  readonly value: string;
}): ReactElement {
  const handleChange = (event: ChangeEvent<HTMLInputElement>): void => onChange(event.currentTarget.value);
  return (
    <label className={PortalFrameTuner.Classes.TunerField}>
      <span>{label}</span>
      <input
        className={PortalFrameTuner.Classes.TunerContentInput}
        onChange={handleChange}
        type={PortalFrameTuner.InputType.Text}
        value={value}
      />
    </label>
  );
}

function ToneSelect({
  onChange,
  value,
}: {
  readonly onChange: (value: PortalAppLayoutTone) => void;
  readonly value: PortalAppLayoutTone;
}): ReactElement {
  const handleChange = (event: ChangeEvent<HTMLSelectElement>): void =>
    onChange(event.currentTarget.value as PortalAppLayoutTone);
  return (
    <label className={PortalFrameTuner.Classes.TunerField}>
      <span>{PortalFrameTuner.Text.Theme}</span>
      <select className={PortalFrameTuner.Classes.TunerContentInput} onChange={handleChange} value={value}>
        {ToneValues.map((tone) => (
          <option key={tone} value={tone}>
            {tone}
          </option>
        ))}
      </select>
    </label>
  );
}

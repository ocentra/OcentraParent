import { type ChangeEvent, type ReactElement } from 'react';
import { type PortalDisplayText } from '@ocentra-parent/portal-domain/display-text';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import {
  PortalFrameTuner,
  type PortalFrameBooleanField,
  type PortalFrameColorField,
  type PortalFrameNumberField,
} from '@ocentra-parent/portal-domain/frame-tuner';
import { valueAt } from './portal-frame-layout-state';

type TunerActionButtonProps = {
  readonly active?: boolean;
  readonly label: PortalDisplayText;
  readonly onClick: () => void;
};

type NumberSectionProps = {
  readonly defaultRoot: unknown;
  readonly fields: readonly PortalFrameNumberField[];
  readonly onChange: (field: PortalFrameNumberField, value: number) => void;
  readonly root: unknown;
  readonly title: PortalDisplayText;
};

type BooleanSectionProps = {
  readonly defaultRoot: unknown;
  readonly fields: readonly PortalFrameBooleanField[];
  readonly onChange: (field: PortalFrameBooleanField, value: boolean) => void;
  readonly root: unknown;
  readonly title: PortalDisplayText;
};

type ColorSectionProps = {
  readonly defaultRoot: unknown;
  readonly fields: readonly PortalFrameColorField[];
  readonly onChange: (field: PortalFrameColorField, value: unknown) => void;
  readonly root: unknown;
  readonly title: PortalDisplayText;
};

export function TunerTabButton({ active, label, onClick }: TunerActionButtonProps): ReactElement {
  return (
    <button
      className={classNames(
        PortalFrameTuner.Classes.TunerTab,
        active === true ? PortalFrameTuner.Classes.TunerTabActive : undefined
      )}
      aria-selected={active === true ? PortalDom.Attributes.True : PortalDom.Attributes.False}
      onClick={onClick}
      role={PortalDom.Attributes.Tab}
      type={PortalDom.ButtonType.Button}
    >
      {label}
    </button>
  );
}

export function TunerActionButton({ active, label, onClick }: TunerActionButtonProps): ReactElement {
  return (
    <button
      className={classNames(
        PortalFrameTuner.Classes.TunerButton,
        active === true ? PortalFrameTuner.Classes.TunerButtonActive : undefined
      )}
      onClick={onClick}
      type={PortalDom.ButtonType.Button}
    >
      {label}
    </button>
  );
}

export function NumberControlSection({ defaultRoot, fields, onChange, root, title }: NumberSectionProps): ReactElement {
  return (
    <section className={PortalFrameTuner.Classes.TunerControlPanel}>
      <h3>{title}</h3>
      {fields.map((field) => (
        <NumberField defaultRoot={defaultRoot} field={field} key={field.label} onChange={onChange} root={root} />
      ))}
    </section>
  );
}

export function BooleanControlSection({
  defaultRoot,
  fields,
  onChange,
  root,
  title,
}: BooleanSectionProps): ReactElement {
  return (
    <section className={PortalFrameTuner.Classes.TunerControlPanel}>
      <h3>{title}</h3>
      {fields.map((field) => (
        <BooleanField defaultRoot={defaultRoot} field={field} key={field.label} onChange={onChange} root={root} />
      ))}
    </section>
  );
}

export function ColorControlSection({ defaultRoot, fields, onChange, root, title }: ColorSectionProps): ReactElement {
  return (
    <section className={PortalFrameTuner.Classes.TunerControlPanel}>
      <h3>{title}</h3>
      {fields.map((field) => (
        <ColorField defaultRoot={defaultRoot} field={field} key={field.label} onChange={onChange} root={root} />
      ))}
    </section>
  );
}

function NumberField({
  defaultRoot,
  field,
  onChange,
  root,
}: {
  readonly defaultRoot: unknown;
  readonly field: PortalFrameNumberField;
  readonly onChange: (field: PortalFrameNumberField, value: number) => void;
  readonly root: unknown;
}): ReactElement {
  const value = Number(valueAt(root, field.path));
  const safeValue = Number.isFinite(value) ? value : field.min;
  const defaultValue = Number(valueAt(defaultRoot, field.path));
  const safeDefaultValue = Number.isFinite(defaultValue) ? defaultValue : field.min;
  const handleChange = (event: ChangeEvent<HTMLInputElement>): void =>
    onChange(field, event.currentTarget.valueAsNumber);
  const handleReset = (): void => onChange(field, safeDefaultValue);
  return (
    <label className={PortalFrameTuner.Classes.TunerField}>
      <span>{field.label}</span>
      <input
        max={field.max}
        min={field.min}
        onChange={handleChange}
        step={field.step}
        type={PortalFrameTuner.InputType.Range}
        value={safeValue}
      />
      <input
        max={field.max}
        min={field.min}
        onChange={handleChange}
        step={field.step}
        type={PortalFrameTuner.InputType.Number}
        value={safeValue}
      />
      <button
        className={PortalFrameTuner.Classes.TunerFieldReset}
        aria-label={PortalFrameTuner.Text.Reset}
        onClick={handleReset}
        title={PortalFrameTuner.Text.Reset}
        type={PortalDom.ButtonType.Button}
      >
        <span aria-hidden={PortalDom.Attributes.True}>{PortalFrameTuner.Text.ResetGlyph}</span>
      </button>
    </label>
  );
}

function BooleanField({
  defaultRoot,
  field,
  onChange,
  root,
}: {
  readonly defaultRoot: unknown;
  readonly field: PortalFrameBooleanField;
  readonly onChange: (field: PortalFrameBooleanField, value: boolean) => void;
  readonly root: unknown;
}): ReactElement {
  const rawValue = valueAt(root, field.path);
  const value = rawValue === true || rawValue === false ? rawValue : false;
  const rawDefaultValue = valueAt(defaultRoot, field.path);
  const defaultValue = rawDefaultValue === true || rawDefaultValue === false ? rawDefaultValue : false;
  const handleChange = (event: ChangeEvent<HTMLInputElement>): void => onChange(field, event.currentTarget.checked);
  const handleReset = (): void => onChange(field, defaultValue);
  return (
    <label className={classNames(PortalFrameTuner.Classes.TunerField, PortalFrameTuner.Classes.TunerBoolean)}>
      <span>{field.label}</span>
      <input checked={value} onChange={handleChange} type={PortalFrameTuner.InputType.Checkbox} />
      <span className={PortalFrameTuner.Classes.TunerFieldValue}>
        {value ? PortalDom.Attributes.True : PortalDom.Attributes.False}
      </span>
      <button
        className={PortalFrameTuner.Classes.TunerFieldReset}
        aria-label={PortalFrameTuner.Text.Reset}
        onClick={handleReset}
        title={PortalFrameTuner.Text.Reset}
        type={PortalDom.ButtonType.Button}
      >
        <span aria-hidden={PortalDom.Attributes.True}>{PortalFrameTuner.Text.ResetGlyph}</span>
      </button>
    </label>
  );
}

function ColorField({
  defaultRoot,
  field,
  onChange,
  root,
}: {
  readonly defaultRoot: unknown;
  readonly field: PortalFrameColorField;
  readonly onChange: (field: PortalFrameColorField, value: unknown) => void;
  readonly root: unknown;
}): ReactElement {
  const fallback = PortalFrameTuner.Color.Cyan;
  const rawValue = valueAt(root, field.path);
  const value = typeof rawValue === typeof fallback ? rawValue : fallback;
  const rawDefaultValue = valueAt(defaultRoot, field.path);
  const defaultValue = typeof rawDefaultValue === typeof fallback ? rawDefaultValue : fallback;
  const handleChange = (event: ChangeEvent<HTMLInputElement>): void => onChange(field, event.currentTarget.value);
  const handleReset = (): void => onChange(field, defaultValue);
  return (
    <label className={classNames(PortalFrameTuner.Classes.TunerField, PortalFrameTuner.Classes.TunerColor)}>
      <span>{field.label}</span>
      <input onChange={handleChange} type={PortalFrameTuner.InputType.Color} value={value as typeof fallback} />
      <span className={PortalFrameTuner.Classes.TunerFieldValue}>{value as typeof fallback}</span>
      <button
        className={PortalFrameTuner.Classes.TunerFieldReset}
        aria-label={PortalFrameTuner.Text.Reset}
        onClick={handleReset}
        title={PortalFrameTuner.Text.Reset}
        type={PortalDom.ButtonType.Button}
      >
        <span aria-hidden={PortalDom.Attributes.True}>{PortalFrameTuner.Text.ResetGlyph}</span>
      </button>
    </label>
  );
}

export function classNames(...values: readonly (string | undefined)[]): string {
  return values.filter(Boolean).join(PortalDom.Classes.ClassNameSeparator);
}

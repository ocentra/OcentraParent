/* thin adapter helpers for Rust-seeded screen control catalog metadata */

import { slugToken } from './catalog-metadata-text';
import { ScreenControlOptionIdSchema } from './screen-control-catalog-schema';
import type {
  ScreenControlCatalogCardKind,
  ScreenControlCatalogControlKind,
  ScreenControlCatalogLayoutHints,
  ScreenControlCatalogOption,
  ScreenControlCatalogSelectionMode,
} from './screen-control-catalog-schema';

const FixedCardKinds = {
  duration: 'duration-card',
  number: 'number-card',
  'read-only-status': 'status-card',
  retention: 'retention-card',
  'rule-list': 'rule-list-card',
  schedule: 'schedule-card',
  'target-list': 'target-list-card',
  threshold: 'threshold-card',
  toggle: 'toggle',
} as const satisfies Partial<Record<ScreenControlCatalogControlKind, ScreenControlCatalogCardKind>>;

const FixedSelectionModes = {
  number: 'numeric',
  duration: 'numeric',
  threshold: 'numeric',
  schedule: 'schedule',
  'read-only-status': 'status',
} as const satisfies Partial<Record<ScreenControlCatalogControlKind, ScreenControlCatalogSelectionMode>>;

const MultiSelectionKinds = new Set<ScreenControlCatalogControlKind>(['multi-choice', 'rule-list', 'target-list']);

export function screenOptionsForSetting(
  sourceText: string,
  controlKind: ScreenControlCatalogControlKind,
  acceptedOptions: readonly string[]
): ScreenControlCatalogOption[] {
  if (acceptedOptions.length > 0) {
    return screenOptions(acceptedOptions);
  }
  const automaticOptions = ScreenAutomaticOptionsByKind[controlKind];
  if (automaticOptions !== undefined) {
    return screenOptions(automaticOptions);
  }
  const explicitOptions = screenExplicitOptionLabels(sourceText);
  return explicitOptions.length > 0 ? screenOptions(explicitOptions) : screenOptions(['Configured', 'Unavailable']);
}

const ScreenAutomaticOptionsByKind = {
  toggle: ['Enabled', 'Disabled'],
  number: ['Configured Value', 'Minimum', 'Maximum'],
  duration: ['Configured Value', 'Minimum', 'Maximum'],
  threshold: ['Configured Value', 'Minimum', 'Maximum'],
} as const satisfies Partial<Record<ScreenControlCatalogControlKind, readonly string[]>>;

export function screenSelectionModeFor(
  controlKind: ScreenControlCatalogControlKind,
  settingOptions: readonly ScreenControlCatalogOption[]
): ScreenControlCatalogSelectionMode {
  const fixedSelectionMode = FixedSelectionModes[controlKind];
  if (fixedSelectionMode !== undefined) {
    return fixedSelectionMode;
  }
  return MultiSelectionKinds.has(controlKind) || settingOptions.length > 4 ? 'multi' : 'single';
}

export function screenCardKindFor(
  controlKind: ScreenControlCatalogControlKind,
  selectionMode: ScreenControlCatalogSelectionMode,
  settingOptions: readonly ScreenControlCatalogOption[]
): ScreenControlCatalogCardKind {
  const fixedCardKind = FixedCardKinds[controlKind];
  if (fixedCardKind !== undefined) {
    return fixedCardKind;
  }
  return selectionMode === 'multi'
    ? settingOptions.length > 4
      ? 'multi-choice-many'
      : 'multi-choice-normal'
    : settingOptions.length > 4
      ? 'single-choice-many'
      : 'single-choice-compact';
}

export function screenLayoutHintsFor(
  selectionMode: ScreenControlCatalogSelectionMode,
  settingOptions: readonly ScreenControlCatalogOption[]
): ScreenControlCatalogLayoutHints {
  const manyOptions = settingOptions.length > 4;
  return {
    preferredColumnSpan: manyOptions ? 2 : 1,
    collapsible: manyOptions || selectionMode === 'multi' || selectionMode === 'status',
    searchableOptions: manyOptions,
    optionGroupCount: manyOptions ? Math.ceil(settingOptions.length / 4) : 1,
    showAsMatrixWhenLarge: manyOptions && selectionMode === 'multi',
    showSelectedCount: selectionMode === 'multi',
  };
}

function screenOptions(labels: readonly string[]): ScreenControlCatalogOption[] {
  return labels.map((label) => {
    const value = slugToken(label, 'option');
    return {
      optionId: ScreenControlOptionIdSchema.parse(`screen-catalog-option-${value}`),
      label,
      value,
      originalSourceText: label,
      meaning: null,
      defaultSelected: false,
    };
  });
}

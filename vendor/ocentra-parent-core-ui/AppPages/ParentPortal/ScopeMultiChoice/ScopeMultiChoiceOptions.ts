import type { ScopeMultiChoiceConfig, ScopeMultiChoiceOption } from './ScopeMultiChoiceTypes';

export function normalizeScopeMultiChoiceOptions(
  config: ScopeMultiChoiceConfig,
  options?: readonly ScopeMultiChoiceOption[]
): readonly ScopeMultiChoiceOption[] {
  const fallbackOptions = config.text.options.map((option) => ({ ...option }));
  const incomingOptions = options && options.length > 0 ? options : fallbackOptions;
  const normalizedOptions = incomingOptions
    .filter((option) => option.value.length > 0 && option.label.length > 0)
    .slice(0, config.layout.maxOptions);

  return normalizedOptions.length > 0 ? normalizedOptions : fallbackOptions.slice(0, 1);
}

export function toggleScopeMultiChoiceValue(
  current: readonly string[],
  nextValue: string,
  multiSelect: boolean
): readonly string[] {
  if (!multiSelect) {
    return [nextValue];
  }

  return current.includes(nextValue) ? current.filter((value) => value !== nextValue) : [...current, nextValue];
}

export function scopeMultiChoiceSelectionLabel(selectedValues: readonly string[]): string {
  return selectedValues.length > 0 ? selectedValues.join(' + ') : 'none';
}

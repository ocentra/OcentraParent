import type { ScopeToggleConfig } from './ScopeToggleConfig';
import type { ScopeToggleOption } from './ScopeToggleTypes';

export function normalizeScopeToggleOptions(
  config: ScopeToggleConfig,
  options?: ScopeToggleOption[],
  leftOption?: string,
  rightOption?: string
): ScopeToggleOption[] {
  const fallbackOptions = config.text.options.map((option) => ({ ...option }));
  const incomingOptions = options && options.length > 0 ? options : fallbackOptions;
  const normalizedOptions = incomingOptions
    .filter((option) => option && option.value && option.label)
    .slice(0, config.layout.maxOptions);

  if (!options && (leftOption || rightOption)) {
    normalizedOptions[0] = {
      ...(normalizedOptions[0] ?? { value: 'family', label: 'Family' }),
      label: leftOption ?? normalizedOptions[0]?.label ?? 'Family',
    };
    normalizedOptions[1] = {
      ...(normalizedOptions[1] ?? { value: 'device', label: 'Per Device' }),
      label: rightOption ?? normalizedOptions[1]?.label ?? 'Per Device',
    };
  }

  return normalizedOptions.length < 2 ? fallbackOptions.slice(0, 2) : normalizedOptions;
}

export function getNextScopeToggleValue(value: string, options: ScopeToggleOption[]): string {
  const selectedIndex = Math.max(
    0,
    options.findIndex((option) => option.value === value)
  );
  const nextIndex = (selectedIndex + 1) % options.length;
  return options[nextIndex]?.value ?? options[0]?.value ?? '';
}

export function getSelectedScopeToggleIndex(value: string, options: ScopeToggleOption[]): number {
  const selectedIndex = options.findIndex((option) => option.value === value);
  return selectedIndex >= 0 ? selectedIndex : 0;
}

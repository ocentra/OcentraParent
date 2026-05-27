import type { GridToggleOption } from './GridToggleTypes';

export function normalizeGridToggleOptions(
  options: GridToggleOption[] | undefined,
  totalCells: number,
  fallbackOptions: GridToggleOption[]
): GridToggleOption[] {
  const source = options && options.length > 0 ? options : fallbackOptions;
  const normalized = source.filter((option) => option && option.value && option.label).slice(0, totalCells);

  while (normalized.length < totalCells) {
    const index = normalized.length;
    normalized.push({ value: `device-${index + 1}`, label: `Device ${index + 1}` });
  }

  return normalized;
}

export function getSelectedGridToggleIndex(value: string, options: GridToggleOption[]): number {
  const selectedIndex = options.findIndex((option) => option.value === value);
  return selectedIndex >= 0 ? selectedIndex : 0;
}

export function getNextGridToggleValue(value: string, options: GridToggleOption[]): string {
  const selectedIndex = getSelectedGridToggleIndex(value, options);
  const nextIndex = (selectedIndex + 1) % options.length;
  return options[nextIndex]?.value ?? options[0]?.value ?? '';
}

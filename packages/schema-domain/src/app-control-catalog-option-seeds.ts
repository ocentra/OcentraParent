import { AppControlOptionIdSchema } from './app-control-catalog-schema';
import type {
  AppControlCatalogDefaultValue,
  AppControlCatalogOptionSeed,
  AppControlCatalogSettingSeed,
} from './app-control-catalog-data';
import type { AppControlCatalogOption } from './app-control-catalog-schema';
import { titleFromToken } from './app-control-catalog-string';

export function optionsFromSeeds(
  settingId: string,
  controlKind: string,
  optionSeeds: readonly AppControlCatalogOptionSeed[],
  defaultValue: AppControlCatalogDefaultValue
): AppControlCatalogOption[] {
  if (controlKind === 'toggle' && optionSeeds.length === 0) {
    return [
      optionFromSeed(
        settingId,
        { value: 'enabled', label: 'Enabled', meaning: 'This app control is enabled.' },
        defaultValue
      ),
      optionFromSeed(
        settingId,
        { value: 'disabled', label: 'Disabled', meaning: 'This app control is disabled.' },
        defaultValue
      ),
    ];
  }
  return optionSeeds.map((optionSeed) => optionFromSeed(settingId, optionSeed, defaultValue));
}

export function optionsFromGuideSeeds(
  settingId: string,
  optionSeeds: readonly AppControlCatalogOptionSeed[]
): AppControlCatalogOption[] {
  if (optionSeeds.length > 0) {
    return optionSeeds.map((optionSeed) => optionFromSeed(settingId, optionSeed, null));
  }
  return [
    optionFromSeed(
      settingId,
      { value: 'represented', label: 'Represented', meaning: 'This guide control is represented in the catalog.' },
      null
    ),
    optionFromSeed(
      settingId,
      { value: 'not-represented', label: 'Not represented', meaning: 'This guide control is not selected.' },
      null
    ),
  ];
}

export function optionFromSeed(
  settingId: string,
  optionSeed: AppControlCatalogOptionSeed,
  defaultValue: AppControlCatalogDefaultValue
): AppControlCatalogOption {
  const optionValue = typeof optionSeed === 'string' ? optionSeed : optionSeed.value;
  const label = typeof optionSeed === 'string' ? titleFromToken(optionSeed) : optionSeed.label;
  const meaning = typeof optionSeed === 'string' ? null : (optionSeed.meaning ?? null);
  return {
    optionId: AppControlOptionIdSchema.parse(`${settingId}.${optionValue}`),
    label,
    value: optionValue,
    originalSourceText: label,
    meaning,
    defaultSelected: isDefaultOption(defaultValue, optionValue),
  };
}

export function acceptedOptionCountForSeeds(seeds: readonly AppControlCatalogSettingSeed[]): number {
  return seeds.reduce((count, seed) => count + optionsFromSeeds(seed[5], seed[6], seed[15], seed[16]).length, 0);
}

function isDefaultOption(defaultValue: AppControlCatalogDefaultValue, optionValue: string): boolean {
  if (Array.isArray(defaultValue)) {
    return defaultValue.includes(optionValue);
  }
  if (typeof defaultValue === 'boolean') {
    return (defaultValue && optionValue === 'enabled') || (!defaultValue && optionValue === 'disabled');
  }
  if (defaultValue === null) {
    return false;
  }
  return String(defaultValue) === optionValue;
}

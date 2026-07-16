import {
  type TrackingControlCatalogDefaultValue,
  type TrackingControlCatalogOptionSeed,
} from './tracking-control-catalog-data';
import { slugToken, titleFromToken } from './tracking-control-catalog-metadata';
import {
  TrackingControlOptionIdSchema,
  TrackingControlSettingIdSchema,
  type TrackingControlKind,
  type TrackingControlOption,
} from './tracking-control-catalog-schema';

export function buildTrackingControlOptions(
  settingId: string,
  controlKind: TrackingControlKind,
  optionSeeds: readonly TrackingControlCatalogOptionSeed[],
  defaultValue: TrackingControlCatalogDefaultValue
): TrackingControlOption[] {
  if (controlKind === 'toggle' && optionSeeds.length === 0) {
    return [
      optionFromSeed(
        settingId,
        { value: 'enabled', label: 'Enabled', meaning: 'This control is enabled.' },
        defaultValue
      ),
      optionFromSeed(
        settingId,
        { value: 'disabled', label: 'Disabled', meaning: 'This control is disabled.' },
        defaultValue
      ),
    ];
  }

  if (optionSeeds.length === 0) {
    return [
      optionFromSeed(
        settingId,
        { value: 'represented', label: 'Represented', meaning: 'This Tracking source item is represented.' },
        defaultValue
      ),
      optionFromSeed(
        settingId,
        { value: 'not-represented', label: 'Not Represented', meaning: 'This Tracking source item is not selected.' },
        defaultValue
      ),
    ];
  }

  return optionSeeds.map((optionSeed) => optionFromSeed(settingId, optionSeed, defaultValue));
}

export function buildTrackingControlOptionLabels(settingId: string, labels: readonly string[]) {
  return labels.map((label) =>
    optionFromSeed(settingId, { value: slugToken(label), label, meaning: `${label} option.` }, null)
  );
}

export function buildTrackingControlVisibilityConditions(settingId: string) {
  if (settingId === 'location.enabled') {
    return [
      {
        ruleId: TrackingControlSettingIdSchema.parse(`${settingId}.always-visible`),
        description: 'Visible in the Tracking side-panel category.',
      },
    ];
  }

  return [
    {
      ruleId: TrackingControlSettingIdSchema.parse(`${settingId}.location-enabled`),
      description: 'Visible when device location features are enabled.',
    },
  ];
}

export function buildTrackingControlEnabledConditions(settingId: string, effectStatus: string, runtimeOwner: string) {
  return [
    {
      ruleId: TrackingControlSettingIdSchema.parse(`${settingId}.capability-state`),
      description: `Enabled state follows ${effectStatus} capability status.`,
    },
    {
      ruleId: TrackingControlSettingIdSchema.parse(`${settingId}.runtime-owner`),
      description: `Runtime owner remains ${runtimeOwner}; Portal does not execute tracking or policy evaluation.`,
    },
  ];
}

export function buildTrackingControlValidationRules(settingId: string, controlKind: TrackingControlKind) {
  return [
    {
      ruleId: TrackingControlSettingIdSchema.parse(`${settingId}.writes-to`),
      description: 'writesTo must target a known locationPolicy path.',
    },
    {
      ruleId: TrackingControlSettingIdSchema.parse(`${settingId}.value-shape`),
      description: `${controlKind} values must decode through the Tracking control schema.`,
    },
  ];
}

function optionFromSeed(
  settingId: string,
  optionSeed: TrackingControlCatalogOptionSeed,
  defaultValue: TrackingControlCatalogDefaultValue
): TrackingControlOption {
  const optionValue = typeof optionSeed === 'string' ? optionSeed : optionSeed.value;
  const label = typeof optionSeed === 'string' ? titleFromToken(slugToken(optionSeed)) : optionSeed.label;
  const meaning = typeof optionSeed === 'string' ? null : (optionSeed.meaning ?? null);

  return {
    optionId: TrackingControlOptionIdSchema.parse(`${settingId}.${optionValue}`),
    label,
    value: optionValue,
    originalSourceText: label,
    meaning,
    defaultSelected: isDefaultOption(defaultValue, optionValue),
  };
}

function isDefaultOption(defaultValue: TrackingControlCatalogDefaultValue, optionValue: string): boolean {
  if (Array.isArray(defaultValue)) {
    return defaultValue.includes(optionValue);
  }
  if (typeof defaultValue === 'boolean') {
    return defaultValue ? optionValue === 'enabled' : optionValue === 'disabled';
  }
  return defaultValue === optionValue;
}

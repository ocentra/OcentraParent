/* generated from crates/network-core/src/network_control_catalog_value_helpers.ts.txt */

import {
  NetworkControlCatalogEffectModeLabels,
  NetworkControlCatalogTargetScopeLabels,
  type NetworkControlCatalogDefaultValue,
  type NetworkControlCatalogOptionSeed,
} from './network-control-catalog-data';
import { slugToken, titleFromToken } from './network-control-catalog-metadata';
import {
  NetworkControlOptionIdSchema,
  NetworkControlSettingIdSchema,
  type NetworkControlKind,
  type NetworkControlOption,
} from './network-control-catalog-schema';

export const NetworkControlTargetScopeOptions = optionLabels('network-control.target-scope', [
  ...NetworkControlCatalogTargetScopeLabels,
]);

export const NetworkControlEffectModeOptions = optionLabels('network-control.effect-mode', [
  ...NetworkControlCatalogEffectModeLabels,
]);

export function optionsFromSeeds(
  settingId: string,
  controlKind: NetworkControlKind,
  optionSeeds: readonly NetworkControlCatalogOptionSeed[],
  defaultValue: NetworkControlCatalogDefaultValue
): NetworkControlOption[] {
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
        { value: 'represented', label: 'Represented', meaning: 'This Network source item is represented.' },
        defaultValue
      ),
      optionFromSeed(
        settingId,
        {
          value: 'not-represented',
          label: 'Not Represented',
          meaning: 'This Network source item is not selected.',
        },
        defaultValue
      ),
    ];
  }

  return optionSeeds.map((optionSeed) => optionFromSeed(settingId, optionSeed, defaultValue));
}

export function visibilityConditionsFor(settingId: string) {
  if (settingId === 'network.enabled') {
    return [
      {
        ruleId: NetworkControlSettingIdSchema.parse(`${settingId}.always-visible`),
        description: 'Visible in the Network side-panel category.',
      },
    ];
  }

  return [
    {
      ruleId: NetworkControlSettingIdSchema.parse(`${settingId}.network-enabled`),
      description: 'Visible when network management is enabled.',
    },
  ];
}

export function enabledConditionsFor(settingId: string, effectStatus: string, runtimeOwner: string) {
  return [
    {
      ruleId: NetworkControlSettingIdSchema.parse(`${settingId}.capability-state`),
      description: `Enabled state follows ${effectStatus} capability status.`,
    },
    {
      ruleId: NetworkControlSettingIdSchema.parse(`${settingId}.runtime-owner`),
      description: `Runtime owner remains ${runtimeOwner}; Portal does not execute network capture or enforcement.`,
    },
  ];
}

export function validationRulesFor(settingId: string, controlKind: NetworkControlKind) {
  return [
    {
      ruleId: NetworkControlSettingIdSchema.parse(`${settingId}.writes-to`),
      description: 'writesTo must target a known networkPolicy path.',
    },
    {
      ruleId: NetworkControlSettingIdSchema.parse(`${settingId}.value-shape`),
      description: `${controlKind} values must decode through the Network control schema.`,
    },
  ];
}

function optionFromSeed(
  settingId: string,
  optionSeed: NetworkControlCatalogOptionSeed,
  defaultValue: NetworkControlCatalogDefaultValue
): NetworkControlOption {
  const optionValue = typeof optionSeed === 'string' ? slugToken(optionSeed) : optionSeed.value;
  const label = typeof optionSeed === 'string' ? titleFromToken(slugToken(optionSeed)) : optionSeed.label;
  const meaning = typeof optionSeed === 'string' ? null : (optionSeed.meaning ?? null);
  return {
    optionId: NetworkControlOptionIdSchema.parse(`${settingId}.${optionValue}`),
    label,
    value: optionValue,
    originalSourceText: label,
    meaning,
    defaultSelected: isDefaultOption(defaultValue, optionValue),
  };
}

function optionLabels(settingId: string, labels: readonly string[]) {
  return labels.map((label) =>
    optionFromSeed(settingId, { value: slugToken(label), label, meaning: `${label} option.` }, null)
  );
}

function isDefaultOption(defaultValue: NetworkControlCatalogDefaultValue, optionValue: string) {
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

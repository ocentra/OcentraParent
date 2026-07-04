import { AppControlKindSchema, AppControlSettingIdSchema } from './app-control-catalog-schema';
import type {
  AppControlCatalogLayoutHints,
  AppControlCatalogOption,
  AppControlCatalogRule,
  AppControlCardKind,
} from './app-control-catalog-schema';

const AppControlCardKindByControlKind = {
  toggle: 'toggle',
  'single-choice': null,
  'multi-choice': null,
  number: 'number-card',
  duration: 'number-card',
  schedule: 'schedule-card',
  'rule-list': 'rule-list-card',
  'target-list': 'target-list-card',
  retention: 'retention-card',
  'action-list': null,
  'read-only-status': 'status-card',
} as const satisfies Record<string, AppControlCardKind | null>;

export function cardKindFor(controlKind: string, optionsForSetting: readonly AppControlCatalogOption[]): AppControlCardKind {
  const parsedControlKind = AppControlKindSchema.parse(controlKind);
  const cardKind = AppControlCardKindByControlKind[parsedControlKind];
  if (cardKind !== null) {
    return cardKind;
  }
  if (parsedControlKind === 'single-choice') {
    return optionsForSetting.length > 4 ? 'single-choice-many' : 'single-choice-compact';
  }
  return optionsForSetting.length > 4 ? 'multi-choice-many' : 'multi-choice-normal';
}

export function layoutHintsFor(
  controlKind: string,
  optionsForSetting: readonly AppControlCatalogOption[]
): AppControlCatalogLayoutHints {
  const parsedControlKind = AppControlKindSchema.parse(controlKind);
  const manyOptions = optionsForSetting.length > 4;
  const listLike = parsedControlKind === 'multi-choice' || parsedControlKind === 'action-list';
  return {
    preferredColumnSpan: manyOptions || parsedControlKind === 'retention' ? 2 : 1,
    collapsible: manyOptions || listLike,
    searchableOptions: manyOptions,
    optionGroupCount: manyOptions ? Math.ceil(optionsForSetting.length / 4) : 1,
    showAsMatrixWhenLarge: manyOptions && listLike,
    showSelectedCount: listLike,
  };
}

export function helperTextFor(
  effectStatus: string,
  capabilityRequirement: string | null,
  proofRequirement: string | null,
  unsafeOrUnsupportedFallback: string | null
): string {
  if (proofRequirement !== null) {
    return proofRequirement;
  }
  if (capabilityRequirement !== null) {
    return capabilityRequirement;
  }
  if (unsafeOrUnsupportedFallback !== null) {
    return unsafeOrUnsupportedFallback;
  }
  if (effectStatus === 'already-represented') {
    return 'Portal renders parent intent while child runtime owns evaluation and audit.';
  }
  return 'Capability state must be shown before claiming app-control behavior.';
}

export function visibilityConditionsFor(settingId: string): AppControlCatalogRule[] {
  if (settingId === 'app.enabled') {
    return [
      {
        ruleId: AppControlSettingIdSchema.parse(`${settingId}.always-visible`),
        description: 'Visible in the Apps side-panel category.',
      },
    ];
  }
  return [
    {
      ruleId: AppControlSettingIdSchema.parse(`${settingId}.app-enabled`),
      description: 'Visible when app management is enabled.',
    },
  ];
}

export function enabledConditionsFor(settingId: string, effectStatus: string, runtimeOwner: string): AppControlCatalogRule[] {
  const descriptions = [
    `Enabled state follows ${effectStatus} capability status.`,
    `Runtime owner remains ${runtimeOwner}; Portal does not execute enforcement.`,
  ];
  return descriptions.map((description, index) => ({
    ruleId: AppControlSettingIdSchema.parse(`${settingId}.enabled-${index + 1}`),
    description,
  }));
}

export function validationRulesFor(settingId: string, controlKind: string): AppControlCatalogRule[] {
  return [
    {
      ruleId: AppControlSettingIdSchema.parse(`${settingId}.writes-to`),
      description: 'writesTo must target a known appPolicy path.',
    },
    {
      ruleId: AppControlSettingIdSchema.parse(`${settingId}.value-shape`),
      description: `${AppControlKindSchema.parse(controlKind)} values must decode through the app-control schema.`,
    },
  ];
}

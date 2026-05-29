import {
  AppControlCatalogSettingSeeds,
  AppControlCatalogSourceDocuments,
  AppControlCapabilitySeeds,
  AppControlEffectModeSeeds,
  AppControlTargetScopeSeeds,
  type AppControlCatalogDefaultValue,
  type AppControlCatalogOptionSeed,
  type AppControlCatalogSettingSeed,
} from './app-control-catalog-data';
import {
  AppControlAuthoringCatalogSchema,
  AppControlCapabilityIdSchema,
  AppControlCapabilitySchema,
  AppControlCatalogIdSchema,
  AppControlCardKindSchema,
  AppControlEffectivePolicySchema,
  AppControlEffectModeSchema,
  AppControlEffectStatusSchema,
  AppControlGroupIdSchema,
  AppControlKindSchema,
  AppControlOptionIdSchema,
  AppControlPolicyValueSchema,
  AppControlRuntimeOwnerSchema,
  AppControlSectionIdSchema,
  AppControlSettingIdSchema,
  AppControlTargetScopeSchema,
  AppControlUiTabSchema,
  AppControlUpdateCommandSchema,
  AppControlWritesToPathSchema,
  type AppControlAuthoringCatalog,
  type AppControlCapability,
  type AppControlCapabilityState,
  type AppControlCardKind,
  type AppControlCatalogGroup,
  type AppControlCatalogLayoutHints,
  type AppControlCatalogOption,
  type AppControlCatalogRule,
  type AppControlCatalogSection,
  type AppControlCatalogSetting,
  type AppControlEffectivePolicy,
  type AppControlKind,
  type AppControlPolicyValue,
  type AppControlUpdateCommand,
} from './app-control-catalog-schema';
import { ParentContractSchemaVersion } from './reference-primitives';

export {
  AppControlAuthoringCatalogSchema,
  AppControlCapabilitySchema,
  AppControlEffectivePolicySchema,
  AppControlPolicyValueSchema,
  AppControlUpdateCommandSchema,
} from './app-control-catalog-schema';
export type {
  AppControlAuthoringCatalog,
  AppControlCapability,
  AppControlCapabilityState,
  AppControlCardKind,
  AppControlCatalogGroup,
  AppControlCatalogOption,
  AppControlCatalogSection,
  AppControlCatalogSetting,
  AppControlEffectivePolicy,
  AppControlKind,
  AppControlPolicyValue,
  AppControlUpdateCommand,
} from './app-control-catalog-schema';

type GroupDraft = Omit<AppControlCatalogGroup, 'settings'> & {
  readonly settings: AppControlCatalogSetting[];
};

type SectionDraft = Omit<AppControlCatalogSection, 'groups'> & {
  readonly groups: Map<string, GroupDraft>;
};

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
} as const satisfies Record<AppControlKind, AppControlCardKind | null>;

export const AppControlSourceOptionCount = AppControlCatalogSettingSeeds.reduce(
  (count, seed) => count + seed[15].length,
  0
);

export const AppControlCapabilities: readonly AppControlCapability[] = AppControlCapabilitySeeds.map((seed) =>
  AppControlCapabilitySchema.parse({
    capabilityId: AppControlCapabilityIdSchema.parse(seed[0]),
    state: seed[1],
    proof: seed[2],
    source: seed[3],
    affectsSettings: seed[4].map((settingId) => AppControlSettingIdSchema.parse(settingId)),
  })
);

export const BaselineAppControlAuthoringCatalog: AppControlAuthoringCatalog = AppControlAuthoringCatalogSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  catalogId: AppControlCatalogIdSchema.parse('app-control-authoring-v1'),
  sidePanelCategory: 'apps',
  sourceDocuments: [...AppControlCatalogSourceDocuments],
  settingCount: AppControlCatalogSettingSeeds.length,
  acceptedOptionCount: acceptedOptionCountForSeeds(AppControlCatalogSettingSeeds),
  targetScopeOptions: AppControlTargetScopeSeeds.map((scope) => AppControlTargetScopeSchema.parse(scope)),
  effectModeOptions: AppControlEffectModeSeeds.map((mode) => AppControlEffectModeSchema.parse(mode)),
  sections: buildSections(AppControlCatalogSettingSeeds),
});

export function appControlCatalogSettings(catalog = BaselineAppControlAuthoringCatalog) {
  return catalog.sections.flatMap((section) => section.groups.flatMap((group) => group.settings));
}

export function appControlCatalogSettingCount(catalog = BaselineAppControlAuthoringCatalog) {
  return appControlCatalogSettings(catalog).length;
}

export function appControlCatalogSectionCount(catalog = BaselineAppControlAuthoringCatalog) {
  return catalog.sections.length;
}

export function appControlCatalogGroupCount(catalog = BaselineAppControlAuthoringCatalog) {
  return catalog.sections.reduce((count, section) => count + section.groups.length, 0);
}

export function appControlCatalogAcceptedOptionCount(catalog = BaselineAppControlAuthoringCatalog) {
  return appControlCatalogSettings(catalog).reduce((count, setting) => count + setting.acceptedOptions.length, 0);
}

export function appControlCatalogSourceOptionCount() {
  return AppControlSourceOptionCount;
}

export function appControlCatalogCanRender(catalog = BaselineAppControlAuthoringCatalog) {
  if (catalog.sidePanelCategory !== 'apps' || catalog.sections.length === 0) {
    return false;
  }
  return catalog.sections.every((section) =>
    section.groups.every((group) =>
      group.settings.every(
        (setting) =>
          setting.policyLane.length > 0 &&
          setting.controlKind.length > 0 &&
          setting.cardKind.length > 0 &&
          setting.layoutHints.preferredColumnSpan > 0 &&
          setting.targetScopeOptions.length > 0 &&
          setting.effectModeOptions.length > 0
      )
    )
  );
}

export function appControlCatalogKnownSettingIds(catalog = BaselineAppControlAuthoringCatalog) {
  return new Set(appControlCatalogSettings(catalog).map((setting) => String(setting.settingId)));
}

export function decodeAppControlAuthoringCatalog(input: unknown) {
  return AppControlAuthoringCatalogSchema.parse(input);
}

export function decodeAppControlCapabilities(input: unknown) {
  return AppControlCapabilitySchema.parse(input);
}

export function decodeAppControlPolicyValue(input: unknown) {
  return AppControlPolicyValueSchema.parse(input);
}

export function decodeAppControlPolicyValueForCatalog(
  input: unknown,
  catalog = BaselineAppControlAuthoringCatalog
): AppControlPolicyValue {
  const parsed = decodeAppControlPolicyValue(input);
  const knownSettingIds = appControlCatalogKnownSettingIds(catalog);
  const seenSettingIds = new Set<string>();
  for (const setting of parsed.settings) {
    const settingId = String(setting.settingId);
    if (!knownSettingIds.has(settingId)) {
      throw new Error(`Unknown app control setting id: ${settingId}`);
    }
    if (seenSettingIds.has(settingId)) {
      throw new Error(`Duplicate app control setting id: ${settingId}`);
    }
    seenSettingIds.add(settingId);
  }
  return parsed;
}

export function decodeAppControlEffectivePolicy(input: unknown) {
  return AppControlEffectivePolicySchema.parse(input);
}

export function decodeAppControlUpdateCommand(input: unknown) {
  return AppControlUpdateCommandSchema.parse(input);
}

export function decodeAppControlUpdateCommandForCatalog(
  input: unknown,
  catalog = BaselineAppControlAuthoringCatalog
): AppControlUpdateCommand {
  const parsed = decodeAppControlUpdateCommand(input);
  const writesToPaths = new Set(appControlCatalogSettings(catalog).map((setting) => String(setting.writesTo)));
  for (const patch of parsed.patch) {
    const path = String(patch.path);
    if (!writesToPaths.has(path)) {
      throw new Error(`Unknown app control writesTo path: ${path}`);
    }
  }
  return parsed;
}

export function buildAppControlEffectivePolicyPlan(
  policy: AppControlPolicyValue,
  catalog = BaselineAppControlAuthoringCatalog
): AppControlEffectivePolicy['plans'] {
  const settingMetadata = new Map(
    appControlCatalogSettings(catalog).map((setting) => [String(setting.settingId), setting])
  );
  return policy.settings.map((policySetting) => {
    const setting = settingMetadata.get(String(policySetting.settingId));
    if (setting === undefined) {
      throw new Error(`Unknown app control setting id: ${String(policySetting.settingId)}`);
    }
    return {
      settingId: setting.settingId,
      effectStatus: setting.effectStatus,
      runtimeOwner: setting.runtimeOwner,
      fallback: setting.unsafeOrUnsupportedFallback ?? 'No fallback required for this app control setting.',
    };
  });
}

function buildSections(seeds: readonly AppControlCatalogSettingSeed[]): AppControlCatalogSection[] {
  const sections = new Map<string, SectionDraft>();
  seeds.forEach((seed, index) => {
    const sourceOrder = index + 1;
    const setting = buildSetting(seed, sourceOrder);
    const section = getSectionDraft(sections, seed, sourceOrder);
    const group = getGroupDraft(section, seed[3], seed[4], sourceOrder);
    group.settings.push(setting);
    section.groups.set(seed[3], group);
    sections.set(seed[0], section);
  });
  return [...sections.values()].sort(bySourceOrder).map(finalizeSection);
}

function buildSetting(seed: AppControlCatalogSettingSeed, sourceOrder: number): AppControlCatalogSetting {
  const [
    sectionId,
    sectionTitle,
    policyLane,
    groupId,
    groupTitle,
    settingId,
    controlKind,
    uiQuestionText,
    writesTo,
    effectStatus,
    runtimeOwner,
    capabilityState,
    capabilityRequirement,
    proofRequirement,
    unsafeOrUnsupportedFallback,
    optionSeeds,
    defaultValue,
  ] = seed;
  const optionsForSetting = optionsFromSeeds(settingId, controlKind, optionSeeds, defaultValue);
  return {
    sidePanelCategory: 'apps',
    policyLane: AppControlUiTabSchema.parse(policyLane),
    sectionId: AppControlSectionIdSchema.parse(sectionId),
    groupId: AppControlGroupIdSchema.parse(groupId),
    settingId: AppControlSettingIdSchema.parse(settingId),
    sourceDocument: 'docs/app-control-schema-proposal.md',
    sourceHeadingPath: [sectionTitle, groupTitle],
    sourceSection: AppControlSectionIdSchema.parse(sectionId),
    sourceGroup: AppControlGroupIdSchema.parse(groupId),
    sourceOrder,
    sourceText: uiQuestionText,
    originalSourceText: uiQuestionText,
    question: uiQuestionText,
    uiQuestionText,
    helperText: helperTextFor(effectStatus, capabilityRequirement, proofRequirement, unsafeOrUnsupportedFallback),
    displayOrder: sourceOrder,
    controlKind: AppControlKindSchema.parse(controlKind),
    cardKind: AppControlCardKindSchema.parse(cardKindFor(controlKind, optionsForSetting)),
    layoutHints: layoutHintsFor(controlKind, optionsForSetting),
    options: optionsForSetting,
    acceptedOptions: optionsForSetting,
    targetScopeOptions: AppControlTargetScopeSeeds.map((scope) => AppControlTargetScopeSchema.parse(scope)),
    effectModeOptions: AppControlEffectModeSeeds.map((mode) => AppControlEffectModeSchema.parse(mode)),
    writesTo: AppControlWritesToPathSchema.parse(writesTo),
    effectKey: AppControlSettingIdSchema.parse(settingId),
    effectStatus: AppControlEffectStatusSchema.parse(effectStatus),
    runtimeOwner: AppControlRuntimeOwnerSchema.parse(runtimeOwner),
    capabilityState: capabilityState as AppControlCapabilityState,
    capabilityRequirement,
    proofRequirement,
    visibilityConditions: visibilityConditionsFor(settingId),
    enabledConditions: enabledConditionsFor(settingId, effectStatus, runtimeOwner),
    validationRules: validationRulesFor(settingId, controlKind),
    unsafeOrUnsupportedFallback,
  };
}

function getSectionDraft(
  sections: Map<string, SectionDraft>,
  seed: AppControlCatalogSettingSeed,
  sourceOrder: number
): SectionDraft {
  const [sectionId, title, policyLane] = seed;
  const existing = sections.get(sectionId);
  if (existing !== undefined) {
    return existing;
  }
  return {
    sectionId: AppControlSectionIdSchema.parse(sectionId),
    title,
    sourceOrder,
    policyLane: AppControlUiTabSchema.parse(policyLane),
    groups: new Map(),
  };
}

function getGroupDraft(section: SectionDraft, groupId: string, title: string, sourceOrder: number): GroupDraft {
  const existing = section.groups.get(groupId);
  if (existing !== undefined) {
    return existing;
  }
  return {
    groupId: AppControlGroupIdSchema.parse(groupId),
    title,
    sourceOrder,
    settings: [],
  };
}

function finalizeSection(section: SectionDraft): AppControlCatalogSection {
  return {
    sectionId: section.sectionId,
    title: section.title,
    sourceOrder: section.sourceOrder,
    policyLane: section.policyLane,
    groups: [...section.groups.values()].sort(bySourceOrder).map(finalizeGroup),
  };
}

function finalizeGroup(group: GroupDraft): AppControlCatalogGroup {
  return {
    groupId: group.groupId,
    title: group.title,
    sourceOrder: group.sourceOrder,
    settings: group.settings,
  };
}

function optionsFromSeeds(
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

function optionFromSeed(
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

function acceptedOptionCountForSeeds(seeds: readonly AppControlCatalogSettingSeed[]): number {
  return seeds.reduce((count, seed) => count + optionsFromSeeds(seed[5], seed[6], seed[15], seed[16]).length, 0);
}

function cardKindFor(controlKind: string, optionsForSetting: readonly AppControlCatalogOption[]): AppControlCardKind {
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

function layoutHintsFor(
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

function helperTextFor(
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

function visibilityConditionsFor(settingId: string): AppControlCatalogRule[] {
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

function enabledConditionsFor(settingId: string, effectStatus: string, runtimeOwner: string): AppControlCatalogRule[] {
  const descriptions = [
    `Enabled state follows ${effectStatus} capability status.`,
    `Runtime owner remains ${runtimeOwner}; Portal does not execute enforcement.`,
  ];
  return descriptions.map((description, index) => ({
    ruleId: AppControlSettingIdSchema.parse(`${settingId}.enabled-${index + 1}`),
    description,
  }));
}

function validationRulesFor(settingId: string, controlKind: string): AppControlCatalogRule[] {
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

function titleFromToken(value: string): string {
  return value
    .split('-')
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
    .join(' ');
}

function bySourceOrder<T extends { readonly sourceOrder: number }>(left: T, right: T): number {
  return left.sourceOrder - right.sourceOrder;
}

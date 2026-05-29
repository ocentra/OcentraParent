import {
  TrackingControlCapabilitySeeds,
  TrackingControlCatalogEffectModeLabels,
  TrackingControlCatalogSettingSeeds,
  TrackingControlCatalogSourceDocuments,
  TrackingControlCatalogTargetScopeLabels,
  type TrackingControlCatalogDefaultValue,
  type TrackingControlCatalogOptionSeed,
  type TrackingControlCatalogSettingSeed,
} from './tracking-control-catalog-data';
import {
  capabilityRequirementFor,
  capabilityStateFor,
  capabilityStateFromSourceState,
  cardKindFor,
  controlKindFor,
  effectStatusFor,
  explicitOptionLabels,
  fallbackFor,
  helperTextFor,
  layoutHintsFor,
  policyLaneFor,
  proofRequirementFor,
  questionFromSourceText,
  runtimeOwnerFor,
  selectionModeFor,
  slugToken,
  titleFromToken,
} from './tracking-control-catalog-metadata';
import {
  TrackingControlCapabilityIdSchema,
  TrackingControlCapabilitySchema,
  TrackingControlCatalogIdSchema,
  TrackingControlCatalogSchema,
  TrackingControlCardKindSchema,
  TrackingControlEffectivePolicySchema,
  TrackingControlEffectStatusSchema,
  TrackingControlGroupIdSchema,
  TrackingControlKindSchema,
  TrackingControlOptionIdSchema,
  TrackingControlPolicyValueSchema,
  TrackingControlRuntimeOwnerSchema,
  TrackingControlSectionIdSchema,
  TrackingControlSettingIdSchema,
  TrackingControlUiTabSchema,
  TrackingControlUpdateCommandSchema,
  TrackingControlWritesToPathSchema,
  type TrackingControlCapability,
  type TrackingControlCatalog,
  type TrackingControlCatalogGroup,
  type TrackingControlCatalogSection,
  type TrackingControlCatalogSetting,
  type TrackingControlCatalogTab,
  type TrackingControlEffectivePolicy,
  type TrackingControlEffectStatus,
  type TrackingControlKind,
  type TrackingControlOption,
  type TrackingControlPolicyValue,
  type TrackingControlRuntimeOwner,
  type TrackingControlUpdateCommand,
} from './tracking-control-catalog-schema';
import { ParentContractSchemaVersion } from './reference-primitives';

export {
  TrackingControlCapabilitySchema,
  TrackingControlCatalogSchema,
  TrackingControlEffectivePolicySchema,
  TrackingControlPolicyValueSchema,
  TrackingControlUpdateCommandSchema,
} from './tracking-control-catalog-schema';
export type {
  TrackingControlCapability,
  TrackingControlCatalog,
  TrackingControlCatalogGroup,
  TrackingControlCatalogSection,
  TrackingControlCatalogSetting,
  TrackingControlCatalogTab,
  TrackingControlEffectivePolicy,
  TrackingControlOption,
  TrackingControlPolicyValue,
  TrackingControlUpdateCommand,
} from './tracking-control-catalog-schema';

type GroupDraft = Omit<TrackingControlCatalogGroup, 'settings'> & {
  readonly settings: TrackingControlCatalogSetting[];
};

type SectionDraft = Omit<TrackingControlCatalogSection, 'groups'> & {
  readonly groups: Map<string, GroupDraft>;
};

type TabDraft = Omit<TrackingControlCatalogTab, 'sections'> & {
  readonly sections: Map<string, SectionDraft>;
};

const TrackingControlTabOrder = [
  'rules',
  'evidence',
  'live',
  'places',
  'schedule',
  'approvals',
  'enforcement',
  'reports',
  'audit',
  'setup',
  'platform',
  'data',
] as const;

const TrackingControlTabTitles = {
  rules: 'Rules',
  evidence: 'Evidence',
  live: 'Live',
  places: 'Places',
  schedule: 'Schedule',
  approvals: 'Approvals',
  enforcement: 'Enforcement',
  reports: 'Reports',
  audit: 'Audit',
  setup: 'Setup',
  platform: 'Platform',
  data: 'Data',
} as const;

export const TrackingControlProposalSettingCount = TrackingControlCatalogSettingSeeds.filter(
  (seed) => seed[0] === TrackingControlCatalogSourceDocuments[1]
).length;
export const TrackingControlGuideSettingCount =
  TrackingControlCatalogSettingSeeds.length - TrackingControlProposalSettingCount;
export const TrackingControlSourceOptionCount = TrackingControlCatalogSettingSeeds.reduce(
  (count, seed) => count + seed[15].length,
  0
);

export const TrackingControlCapabilities: readonly TrackingControlCapability[] = TrackingControlCapabilitySeeds.map(
  (seed) =>
    TrackingControlCapabilitySchema.parse({
      capabilityId: TrackingControlCapabilityIdSchema.parse(seed[0]),
      state: capabilityStateFromSourceState(seed[1]),
      sourceState: seed[1],
      proof: seed[2],
      affectsSettings: seed[3].map((settingId) => TrackingControlSettingIdSchema.parse(settingId)),
    })
);

const TrackingControlTargetScopeOptions = optionLabels('tracking-control.target-scope', [
  ...TrackingControlCatalogTargetScopeLabels,
]);
const TrackingControlEffectModeOptions = optionLabels('tracking-control.effect-mode', [
  ...TrackingControlCatalogEffectModeLabels,
]);

export const BaselineTrackingControlCatalog: TrackingControlCatalog = TrackingControlCatalogSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  catalogId: TrackingControlCatalogIdSchema.parse('tracking-control-full-catalog-v1'),
  sidePanelCategory: 'tracking',
  sourceDocuments: [...TrackingControlCatalogSourceDocuments],
  settingCount: TrackingControlCatalogSettingSeeds.length,
  acceptedOptionCount: trackingControlCatalogAcceptedOptionCountFromSeeds(TrackingControlCatalogSettingSeeds),
  targetScopeOptions: TrackingControlTargetScopeOptions,
  effectModeOptions: TrackingControlEffectModeOptions,
  tabs: buildTabsFromSettings(TrackingControlCatalogSettingSeeds.map(buildSetting)),
});

export function trackingControlCatalogSettings(catalog = BaselineTrackingControlCatalog) {
  return catalog.tabs.flatMap((tab) =>
    tab.sections.flatMap((section) => section.groups.flatMap((group) => group.settings))
  );
}

export function trackingControlCatalogSettingCount(catalog = BaselineTrackingControlCatalog) {
  return trackingControlCatalogSettings(catalog).length;
}

export function trackingControlCatalogSectionCount(catalog = BaselineTrackingControlCatalog) {
  return catalog.tabs.reduce((count, tab) => count + tab.sections.length, 0);
}

export function trackingControlCatalogGroupCount(catalog = BaselineTrackingControlCatalog) {
  return catalog.tabs.reduce(
    (count, tab) => count + tab.sections.reduce((sectionCount, section) => sectionCount + section.groups.length, 0),
    0
  );
}

export function trackingControlCatalogAcceptedOptionCount(catalog = BaselineTrackingControlCatalog) {
  return trackingControlCatalogSettings(catalog).reduce((count, setting) => count + setting.acceptedOptions.length, 0);
}

export function trackingControlCatalogSourceOptionCount() {
  return TrackingControlSourceOptionCount;
}

export function trackingControlCatalogCanRender(catalog = BaselineTrackingControlCatalog) {
  if (catalog.sidePanelCategory !== 'tracking' || catalog.tabs.length === 0) {
    return false;
  }
  return trackingControlCatalogSettings(catalog).every(
    (setting) =>
      setting.policyLane.length > 0 &&
      setting.controlKind.length > 0 &&
      setting.cardKind.length > 0 &&
      setting.layoutHints.preferredColumnSpan > 0 &&
      setting.targetScopeOptions.length > 0 &&
      setting.effectModeOptions.length > 0 &&
      setting.visibilityConditions.length > 0 &&
      setting.enabledConditions.length > 0 &&
      setting.validationRules.length > 0
  );
}

export function trackingControlCatalogKnownSettingIds(catalog = BaselineTrackingControlCatalog) {
  return new Set(trackingControlCatalogSettings(catalog).map((setting) => String(setting.settingId)));
}

export function decodeTrackingControlCatalog(input: unknown) {
  return TrackingControlCatalogSchema.parse(input);
}

export function decodeTrackingControlPolicyValue(input: unknown) {
  return TrackingControlPolicyValueSchema.parse(input);
}

export function decodeTrackingControlPolicyValueForCatalog(
  input: unknown,
  catalog = BaselineTrackingControlCatalog
): TrackingControlPolicyValue {
  const parsed = decodeTrackingControlPolicyValue(input);
  const knownSettingIds = trackingControlCatalogKnownSettingIds(catalog);
  const seenSettingIds = new Set<string>();
  for (const setting of parsed.settings) {
    const settingId = String(setting.settingId);
    if (!knownSettingIds.has(settingId)) {
      throw new Error(`Unknown tracking control setting id: ${settingId}`);
    }
    if (seenSettingIds.has(settingId)) {
      throw new Error(`Duplicate tracking control setting id: ${settingId}`);
    }
    seenSettingIds.add(settingId);
  }
  return parsed;
}

export function decodeTrackingControlEffectivePolicy(input: unknown) {
  return TrackingControlEffectivePolicySchema.parse(input);
}

export function decodeTrackingControlUpdateCommand(input: unknown) {
  return TrackingControlUpdateCommandSchema.parse(input);
}

export function decodeTrackingControlUpdateCommandForCatalog(
  input: unknown,
  catalog = BaselineTrackingControlCatalog
): TrackingControlUpdateCommand {
  const parsed = decodeTrackingControlUpdateCommand(input);
  const writesToPaths = new Set(trackingControlCatalogSettings(catalog).map((setting) => String(setting.writesTo)));
  for (const patch of parsed.patch) {
    const path = String(patch.path);
    if (!writesToPaths.has(path)) {
      throw new Error(`Unknown tracking control writesTo path: ${path}`);
    }
  }
  return parsed;
}

export function buildTrackingControlEffectivePolicyPlan(
  policy: TrackingControlPolicyValue,
  catalog = BaselineTrackingControlCatalog
): TrackingControlEffectivePolicy['plans'] {
  const settingMetadata = new Map(
    trackingControlCatalogSettings(catalog).map((setting) => [String(setting.settingId), setting])
  );
  return policy.settings.map((policySetting) => {
    const setting = settingMetadata.get(String(policySetting.settingId));
    if (setting === undefined) {
      throw new Error(`Unknown tracking control setting id: ${String(policySetting.settingId)}`);
    }
    return {
      settingId: setting.settingId,
      writesTo: setting.writesTo,
      effectStatus: setting.effectStatus,
      runtimeOwner: setting.runtimeOwner,
      capabilityState: setting.capabilityState,
      fallback: setting.unsafeOrUnsupportedFallback,
    };
  });
}

function buildTabsFromSettings(settings: readonly TrackingControlCatalogSetting[]): TrackingControlCatalogTab[] {
  const tabs = new Map<string, TabDraft>();
  for (const setting of settings) {
    const tab = getTabDraft(tabs, String(setting.policyLane));
    const section = getSectionDraft(tab, setting);
    const group = getGroupDraft(section, setting);
    group.settings.push(setting);
  }
  return [...tabs.values()]
    .map((tab) => ({
      ...tab,
      sections: [...tab.sections.values()]
        .map((section) => ({
          ...section,
          groups: [...section.groups.values()].sort((left, right) => left.sourceOrder - right.sourceOrder),
        }))
        .sort((left, right) => left.sourceOrder - right.sourceOrder),
    }))
    .sort((left, right) => left.sourceOrder - right.sourceOrder);
}

function buildSetting(seed: TrackingControlCatalogSettingSeed): TrackingControlCatalogSetting {
  const [
    sourceDocument,
    sectionId,
    sectionTitle,
    ,
    groupId,
    groupTitle,
    ,
    settingId,
    sourceOrder,
    sourceLine,
    sourceText,
    explicitControlKind,
    explicitQuestion,
    explicitWritesTo,
    defaultValue,
    seedOptions,
  ] = seed;
  const policyLane = policyLaneFor(sectionTitle, groupTitle, sourceText);
  const optionSeeds = seedOptions.length > 0 ? seedOptions : explicitOptionLabels(sourceText);
  const controlKind = controlKindFor(sourceText, explicitControlKind);
  const optionsForSetting = optionsFromSeeds(settingId, controlKind, optionSeeds, defaultValue);
  const selectionMode = selectionModeFor(controlKind, optionsForSetting);
  const effectStatus = effectStatusFor(sectionTitle, groupTitle, sourceText);
  const runtimeOwner = runtimeOwnerFor(sectionTitle, groupTitle, sourceText);
  const capabilityRequirement = capabilityRequirementFor(sectionTitle, groupTitle, sourceText);
  return {
    sidePanelCategory: 'tracking',
    policyLane: TrackingControlUiTabSchema.parse(policyLane),
    sectionId: TrackingControlSectionIdSchema.parse(sectionId),
    groupId: TrackingControlGroupIdSchema.parse(groupId),
    settingId: TrackingControlSettingIdSchema.parse(settingId),
    sourceDocument,
    sourceHeadingPath: [sectionTitle, groupTitle],
    sourceSection: TrackingControlSectionIdSchema.parse(sectionId),
    sourceGroup: TrackingControlGroupIdSchema.parse(groupId),
    sourceOrder,
    sourceLine,
    sourceText,
    originalSourceText: sourceText,
    question: questionFromSourceText(sourceText, explicitQuestion),
    uiQuestionText: questionFromSourceText(sourceText, explicitQuestion),
    helperText: helperTextFor(effectStatus, capabilityRequirement),
    displayOrder: sourceOrder,
    selectionMode,
    controlKind: TrackingControlKindSchema.parse(controlKind),
    cardKind: TrackingControlCardKindSchema.parse(cardKindFor(controlKind, optionsForSetting.length)),
    layoutHints: layoutHintsFor(controlKind, optionsForSetting.length),
    options: optionsForSetting,
    acceptedOptions: optionsForSetting,
    targetScopeOptions: TrackingControlTargetScopeOptions,
    effectModeOptions: TrackingControlEffectModeOptions,
    writesTo: TrackingControlWritesToPathSchema.parse(explicitWritesTo ?? `/locationPolicy/catalogGuide/${settingId}`),
    effectKey: TrackingControlSettingIdSchema.parse(settingId),
    effectStatus: TrackingControlEffectStatusSchema.parse(effectStatus),
    runtimeOwner: TrackingControlRuntimeOwnerSchema.parse(runtimeOwner),
    capabilityState: capabilityStateFor(effectStatus),
    capabilityRequirement,
    proofRequirement: proofRequirementFor(sectionTitle, groupTitle, sourceText),
    visibilityConditions: visibilityConditionsFor(settingId),
    enabledConditions: enabledConditionsFor(settingId, effectStatus, runtimeOwner),
    validationRules: validationRulesFor(settingId, controlKind),
    unsafeOrUnsupportedFallback: fallbackFor(effectStatus, sourceText),
  };
}

function getTabDraft(tabs: Map<string, TabDraft>, tabId: string): TabDraft {
  const existing = tabs.get(tabId);
  if (existing !== undefined) {
    return existing;
  }
  const sourceOrder = TrackingControlTabOrder.indexOf(tabId as (typeof TrackingControlTabOrder)[number]);
  const tab = {
    tabId: TrackingControlUiTabSchema.parse(tabId),
    title: TrackingControlTabTitles[tabId as keyof typeof TrackingControlTabTitles],
    sourceOrder: sourceOrder < 0 ? TrackingControlTabOrder.length : sourceOrder + 1,
    sections: new Map<string, SectionDraft>(),
  };
  tabs.set(tabId, tab);
  return tab;
}

function getSectionDraft(tab: TabDraft, setting: TrackingControlCatalogSetting): SectionDraft {
  const key = String(setting.sectionId);
  const existing = tab.sections.get(key);
  if (existing !== undefined) {
    return existing;
  }
  const section = {
    sectionId: setting.sectionId,
    title: setting.sourceHeadingPath[0] ?? key,
    sourceOrder: setting.sourceOrder,
    policyLane: setting.policyLane,
    groups: new Map<string, GroupDraft>(),
  };
  tab.sections.set(key, section);
  return section;
}

function getGroupDraft(section: SectionDraft, setting: TrackingControlCatalogSetting): GroupDraft {
  const key = String(setting.groupId);
  const existing = section.groups.get(key);
  if (existing !== undefined) {
    return existing;
  }
  const group = {
    groupId: setting.groupId,
    title: setting.sourceHeadingPath[1] ?? key,
    sourceOrder: setting.sourceOrder,
    settings: [],
  };
  section.groups.set(key, group);
  return group;
}

function optionsFromSeeds(
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

function optionFromSeed(
  settingId: string,
  optionSeed: TrackingControlCatalogOptionSeed,
  defaultValue: TrackingControlCatalogDefaultValue
): TrackingControlOption {
  const optionValue = typeof optionSeed === 'string' ? slugToken(optionSeed) : optionSeed.value;
  const label = typeof optionSeed === 'string' ? titleFromToken(slugToken(optionSeed)) : optionSeed.label;
  const meaning = typeof optionSeed === 'string' ? null : (optionSeed.meaning ?? null);
  return {
    optionId: TrackingControlOptionIdSchema.parse(`${settingId}.${slugToken(optionValue)}`),
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

function trackingControlCatalogAcceptedOptionCountFromSeeds(seeds: readonly TrackingControlCatalogSettingSeed[]) {
  return seeds.reduce((count, seed) => count + buildSetting(seed).acceptedOptions.length, 0);
}

function visibilityConditionsFor(settingId: string) {
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

function enabledConditionsFor(
  settingId: string,
  effectStatus: TrackingControlEffectStatus,
  runtimeOwner: TrackingControlRuntimeOwner
) {
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

function validationRulesFor(settingId: string, controlKind: TrackingControlKind) {
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

function isDefaultOption(defaultValue: TrackingControlCatalogDefaultValue, optionValue: string) {
  if (Array.isArray(defaultValue)) {
    return defaultValue.includes(optionValue);
  }
  if (typeof defaultValue === 'boolean') {
    return (defaultValue && optionValue === 'enabled') || (!defaultValue && optionValue === 'disabled');
  }
  return defaultValue === optionValue;
}

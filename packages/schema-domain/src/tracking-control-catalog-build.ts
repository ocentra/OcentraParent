import { type TrackingControlCatalogSettingSeed } from './tracking-control-catalog-data';
import {
  capabilityRequirementFor,
  capabilityStateFor,
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
} from './tracking-control-catalog-metadata';
import {
  TrackingControlCardKindSchema,
  TrackingControlEffectStatusSchema,
  TrackingControlGroupIdSchema,
  TrackingControlKindSchema,
  TrackingControlRuntimeOwnerSchema,
  TrackingControlSectionIdSchema,
  TrackingControlSettingIdSchema,
  TrackingControlUiTabSchema,
  TrackingControlWritesToPathSchema,
  type TrackingControlCatalogGroup,
  type TrackingControlCatalogSection,
  type TrackingControlCatalogSetting,
  type TrackingControlCatalogTab,
  type TrackingControlOption,
} from './tracking-control-catalog-schema';
import {
  buildTrackingControlEnabledConditions,
  buildTrackingControlOptions,
  buildTrackingControlOptionLabels as buildTrackingControlOptionLabelsImpl,
  buildTrackingControlValidationRules,
  buildTrackingControlVisibilityConditions,
} from './tracking-control-catalog-options';

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
  'schedule',
  'approvals',
  'enforcement',
  'audit',
  'evidence',
  'setup',
  'reports',
  'platform',
  'data',
  'live',
  'places',
] as const;

const TrackingControlTabTitles = {
  rules: 'Rules',
  schedule: 'Schedule',
  approvals: 'Approvals',
  enforcement: 'Enforcement',
  audit: 'Audit',
  evidence: 'Evidence',
  setup: 'Setup',
  reports: 'Reports',
  platform: 'Platform',
  data: 'Data',
  live: 'Live',
  places: 'Places',
} as const;

export function buildTrackingControlCatalogTabs(
  seeds: readonly TrackingControlCatalogSettingSeed[],
  targetScopeOptions: readonly TrackingControlOption[],
  effectModeOptions: readonly TrackingControlOption[]
) {
  const tabs = new Map<string, TabDraft>();
  for (const seed of seeds) {
    const setting = buildTrackingControlSetting(seed, targetScopeOptions, effectModeOptions);
    const tab = getTabDraft(tabs, setting.policyLane);
    const section = getSectionDraft(tab, seed, setting);
    const group = getGroupDraft(section, seed, setting);
    group.settings.push(setting);
    section.groups.set(String(setting.groupId), group);
    tab.sections.set(String(setting.sectionId), section);
    tabs.set(setting.policyLane, tab);
  }

  return TrackingControlTabOrder.filter((tabId) => tabs.has(tabId)).map((tabId) =>
    finalizeTab(tabs.get(tabId) as TabDraft)
  );
}

export function buildTrackingControlOptionLabels(settingId: string, labels: readonly string[]) {
  return buildTrackingControlOptionLabelsImpl(settingId, labels);
}

export function countTrackingControlAcceptedOptionsFromSeeds(
  seeds: readonly TrackingControlCatalogSettingSeed[],
  targetScopeOptions: readonly TrackingControlOption[],
  effectModeOptions: readonly TrackingControlOption[]
) {
  return seeds.reduce(
    (count, seed) =>
      count + buildTrackingControlSetting(seed, targetScopeOptions, effectModeOptions).acceptedOptions.length,
    0
  );
}

function buildTrackingControlSetting(
  seed: TrackingControlCatalogSettingSeed,
  targetScopeOptions: readonly TrackingControlOption[],
  effectModeOptions: readonly TrackingControlOption[]
): TrackingControlCatalogSetting {
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
  const optionsForSetting = buildTrackingControlOptions(settingId, controlKind, optionSeeds, defaultValue);
  const selectionMode = selectionModeFor(controlKind, optionsForSetting);
  const effectStatus = effectStatusFor(sectionTitle, groupTitle, sourceText);
  const runtimeOwner = runtimeOwnerFor(sectionTitle, groupTitle, sourceText);
  const capabilityRequirement = capabilityRequirementFor(sectionTitle, groupTitle, sourceText);
  const question = questionFromSourceText(sourceText, explicitQuestion);

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
    question,
    uiQuestionText: question,
    helperText: helperTextFor(effectStatus, capabilityRequirement),
    displayOrder: sourceOrder,
    selectionMode,
    controlKind: TrackingControlKindSchema.parse(controlKind),
    cardKind: TrackingControlCardKindSchema.parse(cardKindFor(controlKind, optionsForSetting.length)),
    layoutHints: layoutHintsFor(controlKind, optionsForSetting.length),
    options: optionsForSetting,
    acceptedOptions: optionsForSetting,
    targetScopeOptions,
    effectModeOptions,
    writesTo: TrackingControlWritesToPathSchema.parse(explicitWritesTo ?? `/locationPolicy/catalogGuide/${settingId}`),
    effectKey: TrackingControlSettingIdSchema.parse(settingId),
    effectStatus: TrackingControlEffectStatusSchema.parse(effectStatus),
    runtimeOwner: TrackingControlRuntimeOwnerSchema.parse(runtimeOwner),
    capabilityState: capabilityStateFor(effectStatus),
    capabilityRequirement,
    proofRequirement: proofRequirementFor(sectionTitle, groupTitle, sourceText),
    visibilityConditions: buildTrackingControlVisibilityConditions(settingId),
    enabledConditions: buildTrackingControlEnabledConditions(settingId, effectStatus, runtimeOwner),
    validationRules: buildTrackingControlValidationRules(settingId, controlKind),
    unsafeOrUnsupportedFallback: fallbackFor(effectStatus, sourceText),
  };
}

function getTabDraft(tabs: Map<string, TabDraft>, tabId: string): TabDraft {
  const existing = tabs.get(tabId);
  if (existing !== undefined) {
    return existing;
  }

  const parsedTabId = TrackingControlUiTabSchema.parse(tabId);
  return {
    tabId: parsedTabId,
    title: TrackingControlTabTitles[parsedTabId],
    sourceOrder: TrackingControlTabOrder.indexOf(parsedTabId) + 1,
    sections: new Map(),
  };
}

function getSectionDraft(
  tab: TabDraft,
  seed: TrackingControlCatalogSettingSeed,
  setting: TrackingControlCatalogSetting
): SectionDraft {
  const existing = tab.sections.get(String(setting.sectionId));
  if (existing !== undefined) {
    return existing;
  }

  return {
    sectionId: setting.sectionId,
    title: seed[2],
    sourceOrder: seed[3],
    policyLane: setting.policyLane,
    groups: new Map(),
  };
}

function getGroupDraft(
  section: SectionDraft,
  seed: TrackingControlCatalogSettingSeed,
  setting: TrackingControlCatalogSetting
): GroupDraft {
  const groupId = String(setting.groupId);
  const existing = section.groups.get(groupId);
  if (existing !== undefined) {
    return existing;
  }

  return {
    groupId: setting.groupId,
    title: seed[5],
    sourceOrder: seed[6],
    settings: [],
  };
}

function finalizeTab(tab: TabDraft): TrackingControlCatalogTab {
  return {
    tabId: tab.tabId,
    title: tab.title,
    sourceOrder: tab.sourceOrder,
    sections: [...tab.sections.values()].sort(bySourceOrder).map(finalizeSection),
  };
}

function finalizeSection(section: SectionDraft): TrackingControlCatalogSection {
  return {
    sectionId: section.sectionId,
    title: section.title,
    sourceOrder: section.sourceOrder,
    policyLane: section.policyLane,
    groups: [...section.groups.values()].sort(bySourceOrder).map(finalizeGroup),
  };
}

function finalizeGroup(group: GroupDraft): TrackingControlCatalogGroup {
  return {
    groupId: group.groupId,
    title: group.title,
    sourceOrder: group.sourceOrder,
    settings: group.settings.sort(byDisplayOrder),
  };
}

function byDisplayOrder<T extends { readonly displayOrder: number }>(left: T, right: T): number {
  return left.displayOrder - right.displayOrder;
}

function bySourceOrder<T extends { readonly sourceOrder: number }>(left: T, right: T): number {
  return left.sourceOrder - right.sourceOrder;
}

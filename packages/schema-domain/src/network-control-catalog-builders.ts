/* generated from crates/network-core/src/network_control_catalog_builders.ts.txt */

import {
  NetworkControlCatalogSourceDocuments,
  type NetworkControlCatalogSettingSeed,
} from './network-control-catalog-data';
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
} from './network-control-catalog-metadata';
import {
  NetworkControlCardKindSchema,
  NetworkControlEffectStatusSchema,
  NetworkControlGroupIdSchema,
  NetworkControlKindSchema,
  NetworkControlRuntimeOwnerSchema,
  NetworkControlSectionIdSchema,
  NetworkControlSettingIdSchema,
  NetworkControlUiTabSchema,
  NetworkControlWritesToPathSchema,
  type NetworkControlCatalogGroup,
  type NetworkControlCatalogSection,
  type NetworkControlCatalogSetting,
  type NetworkControlCatalogTab,
} from './network-control-catalog-schema';
import {
  NetworkControlEffectModeOptions,
  NetworkControlTargetScopeOptions,
  enabledConditionsFor,
  optionsFromSeeds,
  validationRulesFor,
  visibilityConditionsFor,
} from './network-control-catalog-value-helpers';

type GroupDraft = Omit<NetworkControlCatalogGroup, 'settings'> & {
  readonly settings: NetworkControlCatalogSetting[];
};

type SectionDraft = Omit<NetworkControlCatalogSection, 'groups'> & {
  readonly groups: Map<string, GroupDraft>;
};

type TabDraft = Omit<NetworkControlCatalogTab, 'sections'> & {
  readonly sections: Map<string, SectionDraft>;
};

const NetworkControlTabOrder = [
  'rules',
  'evidence',
  'enforcement',
  'schedule',
  'approvals',
  'reports',
  'audit',
  'setup',
  'platform',
  'data',
  'ai',
] as const;

const NetworkControlTabTitles = {
  rules: 'Rules',
  evidence: 'Evidence',
  enforcement: 'Enforcement',
  schedule: 'Schedule',
  approvals: 'Approvals',
  reports: 'Reports',
  audit: 'Audit',
  setup: 'Setup',
  platform: 'Platform',
  data: 'Data',
  ai: 'AI',
} as const;

export { NetworkControlEffectModeOptions, NetworkControlTargetScopeOptions };

export function networkControlCatalogAcceptedOptionCountFromSeeds(seeds: readonly NetworkControlCatalogSettingSeed[]) {
  return seeds.reduce((count, seed) => count + buildSetting(seed).acceptedOptions.length, 0);
}

export function buildTabs(seeds: readonly NetworkControlCatalogSettingSeed[]) {
  const tabs = new Map<string, TabDraft>();
  for (const seed of seeds) {
    const setting = buildSetting(seed);
    const tab = getTabDraft(tabs, setting.policyLane);
    const section = getSectionDraft(tab, setting);
    const group = getGroupDraft(
      section,
      String(setting.groupId),
      setting.sourceHeadingPath[1] ?? section.title,
      seed[6]
    );
    group.settings.push(setting);
    section.groups.set(String(setting.groupId), group);
    tab.sections.set(String(setting.sectionId), section);
    tabs.set(setting.policyLane, tab);
  }
  return NetworkControlTabOrder.filter((tabId) => tabs.has(tabId)).map((tabId) =>
    finalizeTab(tabs.get(tabId) as TabDraft)
  );
}

export function buildSetting(seed: NetworkControlCatalogSettingSeed): NetworkControlCatalogSetting {
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
  const stableSettingId =
    sourceDocument === NetworkControlCatalogSourceDocuments[0]
      ? `${settingId}-${String(sourceOrder).padStart(3, '0')}`
      : settingId;
  const policyLane = policyLaneFor(sectionTitle, groupTitle, sourceText);
  const optionSeeds = seedOptions.length > 0 ? seedOptions : explicitOptionLabels(sourceText);
  const controlKind = controlKindFor(sourceText, explicitControlKind);
  const optionsForSetting = optionsFromSeeds(stableSettingId, controlKind, optionSeeds, defaultValue);
  const selectionMode = selectionModeFor(controlKind, optionsForSetting);
  const effectStatus = effectStatusFor(sectionTitle, groupTitle, sourceText);
  const runtimeOwner = runtimeOwnerFor(sectionTitle, groupTitle, sourceText);
  return {
    sidePanelCategory: 'network',
    policyLane: NetworkControlUiTabSchema.parse(policyLane),
    sectionId: NetworkControlSectionIdSchema.parse(sectionId),
    groupId: NetworkControlGroupIdSchema.parse(groupId),
    settingId: NetworkControlSettingIdSchema.parse(stableSettingId),
    sourceDocument,
    sourceHeadingPath: [sectionTitle, groupTitle],
    sourceSection: NetworkControlSectionIdSchema.parse(sectionId),
    sourceGroup: NetworkControlGroupIdSchema.parse(groupId),
    sourceOrder,
    sourceLine,
    sourceText,
    originalSourceText: sourceText,
    question: questionFromSourceText(sourceText, explicitQuestion),
    uiQuestionText: questionFromSourceText(sourceText, explicitQuestion),
    helperText: helperTextFor(sectionTitle, groupTitle, sourceText),
    displayOrder: sourceOrder,
    selectionMode,
    controlKind: NetworkControlKindSchema.parse(controlKind),
    cardKind: NetworkControlCardKindSchema.parse(cardKindFor(controlKind, selectionMode, optionsForSetting)),
    layoutHints: layoutHintsFor(controlKind, selectionMode, optionsForSetting),
    options: optionsForSetting,
    acceptedOptions: optionsForSetting,
    targetScopeOptions: NetworkControlTargetScopeOptions,
    effectModeOptions: NetworkControlEffectModeOptions,
    writesTo: NetworkControlWritesToPathSchema.parse(
      explicitWritesTo ?? `/networkPolicy/catalogGuide/${stableSettingId}`
    ),
    effectKey: NetworkControlSettingIdSchema.parse(stableSettingId),
    effectStatus: NetworkControlEffectStatusSchema.parse(effectStatus),
    runtimeOwner: NetworkControlRuntimeOwnerSchema.parse(runtimeOwner),
    capabilityState: capabilityStateFor(effectStatus),
    capabilityRequirement: capabilityRequirementFor(sectionTitle, groupTitle, sourceText),
    proofRequirement: proofRequirementFor(sectionTitle, groupTitle, sourceText),
    visibilityConditions: visibilityConditionsFor(stableSettingId),
    enabledConditions: enabledConditionsFor(stableSettingId, effectStatus, runtimeOwner),
    validationRules: validationRulesFor(stableSettingId, controlKind),
    unsafeOrUnsupportedFallback: fallbackFor(effectStatus, sourceText),
  };
}

function getTabDraft(tabs: Map<string, TabDraft>, tabId: string): TabDraft {
  const existing = tabs.get(tabId);
  if (existing !== undefined) {
    return existing;
  }
  const parsedTabId = NetworkControlUiTabSchema.parse(tabId);
  return {
    tabId: parsedTabId,
    title: NetworkControlTabTitles[parsedTabId],
    sourceOrder: NetworkControlTabOrder.indexOf(parsedTabId) + 1,
    sections: new Map(),
  };
}

function getSectionDraft(tab: TabDraft, setting: NetworkControlCatalogSetting): SectionDraft {
  const existing = tab.sections.get(String(setting.sectionId));
  if (existing !== undefined) {
    return existing;
  }
  return {
    sectionId: setting.sectionId,
    title: setting.sourceHeadingPath[0] ?? String(setting.sectionId),
    sourceOrder: setting.sourceOrder,
    policyLane: setting.policyLane,
    groups: new Map(),
  };
}

function getGroupDraft(section: SectionDraft, groupId: string, title: string, sourceOrder: number): GroupDraft {
  const existing = section.groups.get(groupId);
  if (existing !== undefined) {
    return existing;
  }
  return {
    groupId: NetworkControlGroupIdSchema.parse(groupId),
    title,
    sourceOrder,
    settings: [],
  };
}

function finalizeTab(tab: TabDraft): NetworkControlCatalogTab {
  return {
    tabId: tab.tabId,
    title: tab.title,
    sourceOrder: tab.sourceOrder,
    sections: [...tab.sections.values()].sort(bySourceOrder).map(finalizeSection),
  };
}

function finalizeSection(section: SectionDraft): NetworkControlCatalogSection {
  return {
    sectionId: section.sectionId,
    title: section.title,
    sourceOrder: section.sourceOrder,
    policyLane: section.policyLane,
    groups: [...section.groups.values()].sort(bySourceOrder).map(finalizeGroup),
  };
}

function finalizeGroup(group: GroupDraft): NetworkControlCatalogGroup {
  return {
    groupId: group.groupId,
    title: group.title,
    sourceOrder: group.sourceOrder,
    settings: group.settings,
  };
}

function bySourceOrder<T extends { readonly sourceOrder: number }>(left: T, right: T): number {
  return left.sourceOrder - right.sourceOrder;
}

import { ParentContractSchemaVersion } from '@ocentra-parent/family-domain/reference-primitives';
import { ScreenControlCatalogData0 } from './screen-control-catalog-data-0';
import { ScreenControlCatalogData1 } from './screen-control-catalog-data-1';
import { ScreenControlCatalogData2 } from './screen-control-catalog-data-2';
import {
  ScreenControlCatalogEffectModeOptions,
  ScreenControlCatalogSidePanelCategory,
  ScreenControlCatalogSourceDocuments,
  ScreenControlCatalogTargetScopeOptions,
  capabilityRequirementFor,
  capabilityStateFor,
  cardKindFor,
  effectStatusFor,
  enabledConditionsFor,
  fallbackFor,
  layoutHintsFor,
  optionsForSetting,
  proofRequirementFor,
  questionFromSeed,
  runtimeOwnerFor,
  selectionModeFor,
  uiTabFor,
  validationRulesFor,
  visibilityConditionsFor,
} from './screen-control-catalog-metadata';
import {
  ScreenControlCatalogIdSchema,
  ScreenControlCatalogSchema,
  ScreenControlSectionIdSchema,
  ScreenControlSettingIdSchema,
  type ScreenControlCatalog,
  type ScreenControlCatalogGroup,
  type ScreenControlCatalogSection,
  type ScreenControlCatalogSectionKind,
  type ScreenControlCatalogSetting,
  type ScreenControlCatalogSettingSeed,
  type ScreenControlCatalogTab,
  type ScreenControlCatalogUiTab,
} from './screen-control-catalog-schema';

type GroupDraft = Omit<ScreenControlCatalogGroup, 'settings'> & {
  readonly settings: ScreenControlCatalogSetting[];
};

type SectionDraft = Omit<ScreenControlCatalogSection, 'groups'> & {
  readonly groups: Map<string, GroupDraft>;
};

type TabDraft = Omit<ScreenControlCatalogTab, 'sections'> & {
  readonly sections: Map<string, SectionDraft>;
};

const ScreenControlCatalogTabOrder = [
  'evidence',
  'rules',
  'schedule',
  'approvals',
  'enforcement',
  'audit',
  'reports',
  'data',
  'ai',
  'setup',
  'platform',
] as const satisfies readonly ScreenControlCatalogUiTab[];

const ScreenControlCatalogTabTitles = {
  evidence: 'Evidence',
  rules: 'Rules',
  schedule: 'Schedule',
  approvals: 'Approvals',
  enforcement: 'Enforcement',
  audit: 'Audit',
  reports: 'Reports',
  data: 'Data',
  ai: 'AI',
  setup: 'Setup',
  platform: 'Platform',
} as const satisfies Record<ScreenControlCatalogUiTab, string>;

export const ScreenControlCatalogSettingSeeds: readonly ScreenControlCatalogSettingSeed[] = [
  ...ScreenControlCatalogData0,
  ...ScreenControlCatalogData1,
  ...ScreenControlCatalogData2,
];

export const BaselineScreenControlCatalog: ScreenControlCatalog = ScreenControlCatalogSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  catalogId: ScreenControlCatalogIdSchema.parse('screen-control-catalog-v1'),
  sidePanelCategory: ScreenControlCatalogSidePanelCategory,
  sourceDocuments: ScreenControlCatalogSourceDocuments,
  settingCount: ScreenControlCatalogSettingSeeds.length,
  targetScopeOptions: ScreenControlCatalogTargetScopeOptions,
  effectModeOptions: ScreenControlCatalogEffectModeOptions,
  tabs: buildTabs(ScreenControlCatalogSettingSeeds),
});

export function screenControlCatalogSettings(catalog = BaselineScreenControlCatalog) {
  return catalog.tabs.flatMap((tab) =>
    tab.sections.flatMap((section) => section.groups.flatMap((group) => group.settings))
  );
}

export function screenControlCatalogSettingCount(catalog = BaselineScreenControlCatalog) {
  return screenControlCatalogSettings(catalog).length;
}

function buildTabs(seeds: readonly ScreenControlCatalogSettingSeed[]): ScreenControlCatalogTab[] {
  const tabs = new Map<ScreenControlCatalogUiTab, TabDraft>();
  for (const seed of seeds) {
    const setting = buildSetting(seed);
    const tab = getTabDraft(tabs, setting.uiTab);
    const section = getSectionDraft(tab, seed, setting.uiTab);
    const group = getGroupDraft(section, setting.groupId, seed[6], seed[7]);
    group.settings.push(setting);
    section.groups.set(setting.groupId, group);
    tab.sections.set(setting.sectionId, section);
    tabs.set(setting.uiTab, tab);
  }
  return ScreenControlCatalogTabOrder.filter((tabId) => tabs.has(tabId)).map((tabId) =>
    finalizeTab(tabs.get(tabId) as TabDraft)
  );
}

function buildSetting(seed: ScreenControlCatalogSettingSeed): ScreenControlCatalogSetting {
  const [
    sourceKind,
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
    controlKind,
    optionLabels,
  ] = seed;
  const acceptedOptions = optionsForSetting(sourceText, controlKind, optionLabels);
  const selectionMode = selectionModeFor(controlKind, acceptedOptions);
  const uiTab = uiTabFor(sectionTitle, groupTitle, sourceText);
  const question = questionFromSeed(sourceKind, sourceText, controlKind);
  return {
    sidePanelCategory: ScreenControlCatalogSidePanelCategory,
    sourceKind,
    sectionId: ScreenControlSectionIdSchema.parse(sectionId),
    groupId: ScreenControlSectionIdSchema.parse(groupId),
    settingId: ScreenControlSettingIdSchema.parse(settingId),
    sourceDocument,
    sourceHeadingPath: [sectionTitle, groupTitle],
    sourceSection: ScreenControlSectionIdSchema.parse(sectionId),
    sourceGroup: ScreenControlSectionIdSchema.parse(groupId),
    sourceOrder,
    sourceLine,
    sourceText,
    originalSourceText: sourceText,
    question,
    uiQuestionText: question,
    helperText: helperTextFor(sectionTitle, sourceText),
    displayOrder: sourceOrder,
    uiTab,
    policyLane: uiTab,
    selectionMode,
    cardKind: cardKindFor(controlKind, selectionMode, acceptedOptions),
    controlKind,
    layoutHints: layoutHintsFor(selectionMode, acceptedOptions),
    acceptedOptions,
    targetScopeOptions: ScreenControlCatalogTargetScopeOptions,
    effectModeOptions: ScreenControlCatalogEffectModeOptions,
    effectKey: ScreenControlSettingIdSchema.parse(settingId),
    effectStatus: effectStatusFor(sourceKind, sectionTitle, sourceText),
    runtimeOwner: runtimeOwnerFor(sectionTitle, sourceText),
    capabilityState: capabilityStateFor(sourceKind, sectionTitle, sourceText),
    capabilityRequirement: capabilityRequirementFor(sectionTitle, sourceText),
    proofRequirement: proofRequirementFor(sectionTitle, sourceText),
    visibilityConditions: visibilityConditionsFor(),
    enabledConditions: enabledConditionsFor(sourceKind, sectionTitle, sourceText),
    validationRules: validationRulesFor(sourceKind, sectionTitle, sourceText),
    unsafeOrUnsupportedFallback: fallbackFor(sourceKind, sectionTitle, sourceText),
  };
}

function getTabDraft(tabs: Map<ScreenControlCatalogUiTab, TabDraft>, tabId: ScreenControlCatalogUiTab): TabDraft {
  const existing = tabs.get(tabId);
  if (existing !== undefined) {
    return existing;
  }
  return {
    tabId,
    title: ScreenControlCatalogTabTitles[tabId],
    sourceOrder: ScreenControlCatalogTabOrder.indexOf(tabId) + 1,
    sections: new Map(),
  };
}

function getSectionDraft(
  tab: TabDraft,
  seed: ScreenControlCatalogSettingSeed,
  uiTab: ScreenControlCatalogUiTab
): SectionDraft {
  const [, , sectionId, title, sourceOrder] = seed;
  const existing = tab.sections.get(sectionId);
  if (existing !== undefined) {
    return existing;
  }
  return {
    sectionId: ScreenControlSectionIdSchema.parse(sectionId),
    title,
    sourceOrder,
    uiTab,
    sectionKind: sectionKindFor(seed[0]),
    groups: new Map(),
  };
}

function getGroupDraft(section: SectionDraft, groupId: string, title: string, sourceOrder: number): GroupDraft {
  const existing = section.groups.get(groupId);
  if (existing !== undefined) {
    return existing;
  }
  return {
    groupId: ScreenControlSectionIdSchema.parse(groupId),
    title,
    sourceOrder,
    settings: [],
  };
}

function finalizeTab(tab: TabDraft): ScreenControlCatalogTab {
  return {
    tabId: tab.tabId,
    title: tab.title,
    sourceOrder: tab.sourceOrder,
    sections: [...tab.sections.values()].sort(bySourceOrder).map(finalizeSection),
  };
}

function finalizeSection(section: SectionDraft): ScreenControlCatalogSection {
  return {
    sectionId: section.sectionId,
    title: section.title,
    sourceOrder: section.sourceOrder,
    uiTab: section.uiTab,
    sectionKind: section.sectionKind,
    groups: [...section.groups.values()].sort(bySourceOrder).map(finalizeGroup),
  };
}

function finalizeGroup(group: GroupDraft): ScreenControlCatalogGroup {
  return {
    groupId: group.groupId,
    title: group.title,
    sourceOrder: group.sourceOrder,
    settings: group.settings.sort(bySourceOrder),
  };
}

function sectionKindFor(sourceKind: ScreenControlCatalogSettingSeed[0]): ScreenControlCatalogSectionKind {
  if (sourceKind === 'capability-guide-bullet' || sourceKind === 'capability-matrix-row') {
    return 'capability-guide-section';
  }
  if (sourceKind === 'authoring-field' || sourceKind === 'rendering-rule') {
    return 'proposal-authoring-section';
  }
  if (sourceKind === 'capability-registry-entry' || sourceKind === 'capability-state-meaning') {
    return 'capability-registry-section';
  }
  if (sourceKind === 'update-command' || sourceKind === 'agent-rule') {
    return 'update-protocol-section';
  }
  return 'proposal-runtime-section';
}

function helperTextFor(sectionTitle: string, sourceText: string): string {
  if (proofRequirementFor(sectionTitle, sourceText) !== null) {
    return 'Screen evidence is high-sensitivity and strict behavior needs validated summaries, refs, and deletion proof.';
  }
  return 'Portal renders authored intent and status; child runtime owns capture, local analysis, queue, policy use, and audit.';
}

function bySourceOrder<T extends { readonly sourceOrder: number }>(left: T, right: T): number {
  return left.sourceOrder - right.sourceOrder;
}

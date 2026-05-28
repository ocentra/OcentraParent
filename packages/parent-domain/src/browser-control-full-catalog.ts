import {
  BrowserControlFieldIdSchema,
  BrowserControlManifestIdSchema,
  BrowserControlSectionIdSchema,
} from './browser-control-identifiers';
import { BrowserControlFullCatalogData0 } from './browser-control-full-catalog-data-0';
import { BrowserControlFullCatalogData1 } from './browser-control-full-catalog-data-1';
import { BrowserControlFullCatalogData2 } from './browser-control-full-catalog-data-2';
import { BrowserControlFullCatalogData3 } from './browser-control-full-catalog-data-3';
import { BrowserControlFullCatalogData4 } from './browser-control-full-catalog-data-4';
import {
  BrowserControlFullCatalogEffectModeOptions,
  BrowserControlFullCatalogSidePanelCategory,
  BrowserControlFullCatalogSourceDocument,
  BrowserControlFullCatalogSourceDocuments,
  BrowserControlFullCatalogTargetScopeOptions,
  capabilityRequirementFor,
  capabilityStateForSection,
  cardKindFor,
  controlKindFor,
  effectStatusForSection,
  enabledConditionsFor,
  fallbackFor,
  helperTextFor,
  layoutHintsFor,
  optionsFromSourceText,
  proofRequirementFor,
  questionFromSourceText,
  runtimeOwnerForSection,
  selectionModeFor,
  validationRulesFor,
  visibilityConditionsFor,
} from './browser-control-full-catalog-metadata';
import {
  BrowserControlFullCatalogSchema,
  type BrowserControlFullCatalog,
  type BrowserControlFullCatalogGroup,
  type BrowserControlFullCatalogSection,
  type BrowserControlFullCatalogSectionKind,
  type BrowserControlFullCatalogSetting,
  type BrowserControlFullCatalogSettingSeed,
  type BrowserControlFullCatalogTab,
  type BrowserControlFullCatalogUiTab,
} from './browser-control-full-catalog-schema';
import { ParentContractSchemaVersion } from './reference-primitives';

type GroupDraft = Omit<BrowserControlFullCatalogGroup, 'settings'> & {
  readonly settings: BrowserControlFullCatalogSetting[];
};

type SectionDraft = Omit<BrowserControlFullCatalogSection, 'groups'> & {
  readonly groups: Map<string, GroupDraft>;
};

type TabDraft = Omit<BrowserControlFullCatalogTab, 'sections'> & {
  readonly sections: Map<string, SectionDraft>;
};

const BrowserControlFullCatalogTabOrder = [
  'enforcement',
  'rules',
  'schedule',
  'approvals',
  'evidence',
  'reports',
  'data',
  'audit',
  'ai',
  'setup',
  'platform',
] as const satisfies readonly BrowserControlFullCatalogUiTab[];

const BrowserControlFullCatalogTabTitles = {
  enforcement: 'Enforcement',
  rules: 'Rules',
  schedule: 'Schedule',
  approvals: 'Approvals',
  evidence: 'Evidence',
  reports: 'Reports',
  data: 'Data',
  audit: 'Audit',
  ai: 'AI',
  setup: 'Setup',
  platform: 'Platform',
} as const satisfies Record<BrowserControlFullCatalogUiTab, string>;

export const BrowserControlFullCatalogSettingSeeds: readonly BrowserControlFullCatalogSettingSeed[] = [
  ...BrowserControlFullCatalogData0,
  ...BrowserControlFullCatalogData1,
  ...BrowserControlFullCatalogData2,
  ...BrowserControlFullCatalogData3,
  ...BrowserControlFullCatalogData4,
];

export const BaselineBrowserControlFullCatalog: BrowserControlFullCatalog = BrowserControlFullCatalogSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  catalogId: BrowserControlManifestIdSchema.parse('browser-control-full-catalog-v1'),
  sidePanelCategory: BrowserControlFullCatalogSidePanelCategory,
  sourceDocument: BrowserControlFullCatalogSourceDocument,
  sourceDocuments: BrowserControlFullCatalogSourceDocuments,
  settingCount: BrowserControlFullCatalogSettingSeeds.length,
  targetScopeOptions: BrowserControlFullCatalogTargetScopeOptions,
  effectModeOptions: BrowserControlFullCatalogEffectModeOptions,
  tabs: buildTabs(BrowserControlFullCatalogSettingSeeds),
});

export function browserControlFullCatalogSettings(catalog = BaselineBrowserControlFullCatalog) {
  return catalog.tabs.flatMap((tab) =>
    tab.sections.flatMap((section) => section.groups.flatMap((group) => group.settings))
  );
}

export function browserControlFullCatalogSectionTitles(catalog = BaselineBrowserControlFullCatalog) {
  return catalog.tabs.flatMap((tab) => tab.sections.map((section) => section.title));
}

export function browserControlFullCatalogSettingCount(catalog = BaselineBrowserControlFullCatalog) {
  return browserControlFullCatalogSettings(catalog).length;
}

function buildTabs(seeds: readonly BrowserControlFullCatalogSettingSeed[]): BrowserControlFullCatalogTab[] {
  const tabs = new Map<BrowserControlFullCatalogUiTab, TabDraft>();
  for (const seed of seeds) {
    const setting = buildSetting(seed);
    const groupTitle = seed[4];
    const tab = getTabDraft(tabs, setting.uiTab);
    const section = getSectionDraft(tab, seed, setting.uiTab);
    const group = getGroupDraft(section, setting.groupId, groupTitle, seed[5]);
    group.settings.push(setting);
    section.groups.set(setting.groupId, group);
    tab.sections.set(setting.sectionId, section);
    tabs.set(setting.uiTab, tab);
  }
  return BrowserControlFullCatalogTabOrder.filter((tabId) => tabs.has(tabId)).map((tabId) =>
    finalizeTab(tabs.get(tabId) as TabDraft)
  );
}

function buildSetting(seed: BrowserControlFullCatalogSettingSeed): BrowserControlFullCatalogSetting {
  const [sectionId, sectionTitle, , groupId, groupTitle, , settingId, sourceOrder, sourceLine, sourceText] = seed;
  const optionsForSetting = optionsFromSourceText(sourceText);
  const selectionMode = selectionModeFor(sourceText, optionsForSetting);
  const question = questionFromSourceText(sourceText);
  const uiTab = uiTabForSection(sectionTitle);
  return {
    sidePanelCategory: BrowserControlFullCatalogSidePanelCategory,
    sectionId: BrowserControlSectionIdSchema.parse(sectionId),
    groupId: BrowserControlSectionIdSchema.parse(groupId),
    settingId: BrowserControlFieldIdSchema.parse(settingId),
    sourceDocument: BrowserControlFullCatalogSourceDocument,
    sourceHeadingPath: [sectionTitle, groupTitle],
    sourceSection: BrowserControlSectionIdSchema.parse(sectionId),
    sourceGroup: BrowserControlSectionIdSchema.parse(groupId),
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
    cardKind: cardKindFor(selectionMode, optionsForSetting),
    controlKind: controlKindFor(sourceText, selectionMode, optionsForSetting),
    layoutHints: layoutHintsFor(selectionMode, optionsForSetting),
    options: optionsForSetting,
    acceptedOptions: optionsForSetting,
    targetScopeOptions: BrowserControlFullCatalogTargetScopeOptions,
    effectModeOptions: BrowserControlFullCatalogEffectModeOptions,
    effectKey: BrowserControlFieldIdSchema.parse(settingId),
    effectStatus: effectStatusForSection(sectionTitle, sourceText),
    runtimeOwner: runtimeOwnerForSection(sectionTitle, sourceText),
    capabilityState: capabilityStateForSection(sectionTitle, sourceText),
    capabilityRequirement: capabilityRequirementFor(sectionTitle, sourceText),
    proofRequirement: proofRequirementFor(sectionTitle, sourceText),
    visibilityConditions: visibilityConditionsFor(),
    enabledConditions: enabledConditionsFor(sectionTitle, sourceText),
    validationRules: validationRulesFor(sectionTitle, sourceText),
    unsafeOrUnsupportedFallback: fallbackFor(sectionTitle, sourceText),
  };
}

function getTabDraft(
  tabs: Map<BrowserControlFullCatalogUiTab, TabDraft>,
  tabId: BrowserControlFullCatalogUiTab
): TabDraft {
  const existing = tabs.get(tabId);
  if (existing !== undefined) {
    return existing;
  }
  return {
    tabId,
    title: BrowserControlFullCatalogTabTitles[tabId],
    sourceOrder: BrowserControlFullCatalogTabOrder.indexOf(tabId) + 1,
    sections: new Map(),
  };
}

function getSectionDraft(
  tab: TabDraft,
  seed: BrowserControlFullCatalogSettingSeed,
  uiTab: BrowserControlFullCatalogUiTab
): SectionDraft {
  const [sectionId, title, sourceOrder] = seed;
  const existing = tab.sections.get(sectionId);
  if (existing !== undefined) {
    return existing;
  }
  return {
    sectionId: BrowserControlSectionIdSchema.parse(sectionId),
    title,
    sourceOrder,
    uiTab,
    sectionKind: sectionKindForTitle(title),
    groups: new Map(),
  };
}

function getGroupDraft(section: SectionDraft, groupId: string, title: string, sourceOrder: number): GroupDraft {
  const existing = section.groups.get(groupId);
  if (existing !== undefined) {
    return existing;
  }
  return {
    groupId: BrowserControlSectionIdSchema.parse(groupId),
    title,
    sourceOrder,
    settings: [],
  };
}

function finalizeTab(tab: TabDraft): BrowserControlFullCatalogTab {
  return {
    tabId: tab.tabId,
    title: tab.title,
    sourceOrder: tab.sourceOrder,
    sections: [...tab.sections.values()].sort(bySourceOrder).map(finalizeSection),
  };
}

function finalizeSection(section: SectionDraft): BrowserControlFullCatalogSection {
  return {
    sectionId: section.sectionId,
    title: section.title,
    sourceOrder: section.sourceOrder,
    uiTab: section.uiTab,
    sectionKind: section.sectionKind,
    groups: [...section.groups.values()].sort(bySourceOrder).map(finalizeGroup),
  };
}

function finalizeGroup(group: GroupDraft): BrowserControlFullCatalogGroup {
  return {
    groupId: group.groupId,
    title: group.title,
    sourceOrder: group.sourceOrder,
    settings: group.settings,
  };
}

function sectionKindForTitle(sectionTitle: string): BrowserControlFullCatalogSectionKind {
  if (sectionTitle === 'Global Rule Dimensions') {
    return 'rule-dimension-section';
  }
  if (sectionTitle === 'Candidate MVP Setting Set') {
    return 'candidate-mvp-section';
  }
  if (sectionTitle === 'Gaps To Decide Before UI Contracts') {
    return 'planning-gap-section';
  }
  return 'setting-section';
}

function uiTabForSection(sectionTitle: string): BrowserControlFullCatalogUiTab {
  if (/Rule|Search|Video|Conflict/u.test(sectionTitle)) {
    return 'rules';
  }
  if (/Schedule|Time Budget/u.test(sectionTitle)) {
    return 'schedule';
  }
  if (/Approval|Override|Notifications/u.test(sectionTitle)) {
    return 'approvals';
  }
  if (/Evidence|Never-Collect/u.test(sectionTitle)) {
    return 'evidence';
  }
  if (/Report|Portal Display|Child-Facing/u.test(sectionTitle)) {
    return 'reports';
  }
  if (/Custody|Retention/u.test(sectionTitle)) {
    return 'data';
  }
  if (/Audit/u.test(sectionTitle)) {
    return 'audit';
  }
  if (/AI/u.test(sectionTitle)) {
    return 'ai';
  }
  if (/Platform/u.test(sectionTitle)) {
    return 'platform';
  }
  if (/Setup|Provisioning/u.test(sectionTitle)) {
    return 'setup';
  }
  return 'enforcement';
}

function bySourceOrder<T extends { readonly sourceOrder: number }>(left: T, right: T): number {
  return left.sourceOrder - right.sourceOrder;
}

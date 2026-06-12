import {
  NetworkControlCapabilitySeeds,
  NetworkControlCatalogEffectModeLabels,
  NetworkControlCatalogSettingSeeds,
  NetworkControlCatalogSourceDocuments,
  NetworkControlCatalogTargetScopeLabels,
  type NetworkControlCatalogDefaultValue,
  type NetworkControlCatalogOptionSeed,
  type NetworkControlCatalogSettingSeed,
} from './network-control-catalog-data';
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
} from './network-control-catalog-metadata';
import {
  NetworkControlCapabilityIdSchema,
  NetworkControlCapabilitySchema,
  NetworkControlCatalogIdSchema,
  NetworkControlCatalogSchema,
  NetworkControlCardKindSchema,
  NetworkControlEffectivePolicySchema,
  NetworkControlEffectStatusSchema,
  NetworkControlGroupIdSchema,
  NetworkControlKindSchema,
  NetworkControlOptionIdSchema,
  NetworkControlPolicyValueSchema,
  NetworkControlRuntimeOwnerSchema,
  NetworkControlSectionIdSchema,
  NetworkControlSettingIdSchema,
  NetworkControlUiTabSchema,
  NetworkControlUpdateCommandSchema,
  NetworkControlWritesToPathSchema,
  type NetworkControlCapability,
  type NetworkControlCatalog,
  type NetworkControlCatalogGroup,
  type NetworkControlCatalogSection,
  type NetworkControlCatalogSetting,
  type NetworkControlCatalogTab,
  type NetworkControlEffectivePolicy,
  type NetworkControlEffectStatus,
  type NetworkControlKind,
  type NetworkControlOption,
  type NetworkControlPolicyValue,
  type NetworkControlRuntimeOwner,
  type NetworkControlUpdateCommand,
} from './network-control-catalog-schema';
import { ParentContractSchemaVersion } from '@ocentra-parent/family-domain/reference-primitives';

export {
  NetworkControlCapabilitySchema,
  NetworkControlCatalogSchema,
  NetworkControlEffectivePolicySchema,
  NetworkControlPolicyValueSchema,
  NetworkControlUpdateCommandSchema,
} from './network-control-catalog-schema';
export type {
  NetworkControlCapability,
  NetworkControlCatalog,
  NetworkControlCatalogGroup,
  NetworkControlCatalogSection,
  NetworkControlCatalogSetting,
  NetworkControlCatalogTab,
  NetworkControlEffectivePolicy,
  NetworkControlOption,
  NetworkControlPolicyValue,
  NetworkControlUpdateCommand,
} from './network-control-catalog-schema';

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

export const NetworkControlProposalSettingCount = NetworkControlCatalogSettingSeeds.filter(
  (seed) => seed[0] === 'docs/network-control-schema-proposal.md'
).length;

export const NetworkControlGuideSettingCount = NetworkControlCatalogSettingSeeds.filter(
  (seed) => seed[0] === 'docs/network-control-capability-guide.md'
).length;

export const NetworkControlSourceOptionCount = NetworkControlCatalogSettingSeeds.reduce(
  (count, seed) => count + seed[15].length,
  0
);

export const NetworkControlCapabilities: readonly NetworkControlCapability[] = NetworkControlCapabilitySeeds.map(
  (seed) =>
    NetworkControlCapabilitySchema.parse({
      capabilityId: NetworkControlCapabilityIdSchema.parse(seed[0]),
      state: capabilityStateFromSourceState(seed[1]),
      sourceState: seed[1],
      proof: seed[2],
      affectsSettings: seed[3].map((settingId) => NetworkControlSettingIdSchema.parse(settingId)),
    })
);

const NetworkControlTargetScopeOptions = optionLabels('network-control.target-scope', [
  ...NetworkControlCatalogTargetScopeLabels,
]);

const NetworkControlEffectModeOptions = optionLabels('network-control.effect-mode', [
  ...NetworkControlCatalogEffectModeLabels,
]);

export const BaselineNetworkControlCatalog: NetworkControlCatalog = NetworkControlCatalogSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  catalogId: NetworkControlCatalogIdSchema.parse('network-control-catalog-v1'),
  sidePanelCategory: 'network',
  sourceDocuments: [...NetworkControlCatalogSourceDocuments],
  settingCount: NetworkControlCatalogSettingSeeds.length,
  acceptedOptionCount: networkControlCatalogAcceptedOptionCountFromSeeds(NetworkControlCatalogSettingSeeds),
  targetScopeOptions: NetworkControlTargetScopeOptions,
  effectModeOptions: NetworkControlEffectModeOptions,
  tabs: buildTabs(NetworkControlCatalogSettingSeeds),
});

export function networkControlCatalogSettings(catalog = BaselineNetworkControlCatalog) {
  return catalog.tabs.flatMap((tab) =>
    tab.sections.flatMap((section) => section.groups.flatMap((group) => group.settings))
  );
}

export function networkControlCatalogSettingCount(catalog = BaselineNetworkControlCatalog) {
  return networkControlCatalogSettings(catalog).length;
}

export function networkControlCatalogSectionCount(catalog = BaselineNetworkControlCatalog) {
  return catalog.tabs.reduce((count, tab) => count + tab.sections.length, 0);
}

export function networkControlCatalogGroupCount(catalog = BaselineNetworkControlCatalog) {
  return catalog.tabs.reduce(
    (count, tab) => count + tab.sections.reduce((sectionCount, section) => sectionCount + section.groups.length, 0),
    0
  );
}

export function networkControlCatalogAcceptedOptionCount(catalog = BaselineNetworkControlCatalog) {
  return networkControlCatalogSettings(catalog).reduce((count, setting) => count + setting.acceptedOptions.length, 0);
}

export function networkControlCatalogSourceOptionCount() {
  return NetworkControlSourceOptionCount;
}

export function networkControlCatalogCanRender(catalog = BaselineNetworkControlCatalog) {
  if (catalog.sidePanelCategory !== 'network' || catalog.tabs.length === 0) {
    return false;
  }
  return catalog.tabs.every((tab) =>
    tab.sections.every((section) =>
      section.groups.every((group) =>
        group.settings.every(
          (setting) =>
            setting.policyLane.length > 0 &&
            setting.controlKind.length > 0 &&
            setting.cardKind.length > 0 &&
            setting.layoutHints.preferredColumnSpan > 0 &&
            setting.targetScopeOptions.length > 0 &&
            setting.effectModeOptions.length > 0 &&
            setting.capabilityRequirement.length > 0 &&
            setting.unsafeOrUnsupportedFallback.length > 0
        )
      )
    )
  );
}

export function decodeNetworkControlCatalog(input: unknown) {
  return NetworkControlCatalogSchema.parse(input);
}

export function decodeNetworkControlCapability(input: unknown) {
  return NetworkControlCapabilitySchema.parse(input);
}

export function decodeNetworkControlPolicyValue(input: unknown) {
  return NetworkControlPolicyValueSchema.parse(input);
}

export function decodeNetworkControlPolicyValueForCatalog(
  input: unknown,
  catalog = BaselineNetworkControlCatalog
): NetworkControlPolicyValue {
  const parsed = decodeNetworkControlPolicyValue(input);
  const knownSettingIds = new Set(networkControlCatalogSettings(catalog).map((setting) => String(setting.settingId)));
  const seenSettingIds = new Set<string>();
  for (const setting of parsed.settings) {
    const settingId = String(setting.settingId);
    if (!knownSettingIds.has(settingId)) {
      throw new Error(`Unknown network control setting id: ${settingId}`);
    }
    if (seenSettingIds.has(settingId)) {
      throw new Error(`Duplicate network control setting id: ${settingId}`);
    }
    seenSettingIds.add(settingId);
  }
  return parsed;
}

export function decodeNetworkControlEffectivePolicy(input: unknown) {
  return NetworkControlEffectivePolicySchema.parse(input);
}

export function decodeNetworkControlUpdateCommand(input: unknown) {
  return NetworkControlUpdateCommandSchema.parse(input);
}

export function decodeNetworkControlUpdateCommandForCatalog(
  input: unknown,
  catalog = BaselineNetworkControlCatalog
): NetworkControlUpdateCommand {
  const parsed = decodeNetworkControlUpdateCommand(input);
  const writesToPaths = new Set(networkControlCatalogSettings(catalog).map((setting) => String(setting.writesTo)));
  for (const patch of parsed.patch) {
    const path = String(patch.path);
    if (!writesToPaths.has(path)) {
      throw new Error(`Unknown network control writesTo path: ${path}`);
    }
  }
  return parsed;
}

export function buildNetworkControlEffectivePolicyPlan(
  policy: NetworkControlPolicyValue,
  catalog = BaselineNetworkControlCatalog
): NetworkControlEffectivePolicy['plans'] {
  const settingMetadata = new Map(
    networkControlCatalogSettings(catalog).map((setting) => [String(setting.settingId), setting])
  );
  return policy.settings.map((policySetting) => {
    const setting = settingMetadata.get(String(policySetting.settingId));
    if (setting === undefined) {
      throw new Error(`Unknown network control setting id: ${String(policySetting.settingId)}`);
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

function buildTabs(seeds: readonly NetworkControlCatalogSettingSeed[]) {
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

function buildSetting(seed: NetworkControlCatalogSettingSeed): NetworkControlCatalogSetting {
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

function optionsFromSeeds(
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
        { value: 'not-represented', label: 'Not Represented', meaning: 'This Network source item is not selected.' },
        defaultValue
      ),
    ];
  }
  return optionSeeds.map((optionSeed) => optionFromSeed(settingId, optionSeed, defaultValue));
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

function networkControlCatalogAcceptedOptionCountFromSeeds(seeds: readonly NetworkControlCatalogSettingSeed[]) {
  return seeds.reduce((count, seed) => count + buildSetting(seed).acceptedOptions.length, 0);
}

function visibilityConditionsFor(settingId: string) {
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

function enabledConditionsFor(
  settingId: string,
  effectStatus: NetworkControlEffectStatus,
  runtimeOwner: NetworkControlRuntimeOwner
) {
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

function validationRulesFor(settingId: string, controlKind: NetworkControlKind) {
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

function bySourceOrder<T extends { readonly sourceOrder: number }>(left: T, right: T): number {
  return left.sourceOrder - right.sourceOrder;
}

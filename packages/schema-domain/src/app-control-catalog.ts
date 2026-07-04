import {
  AppControlCatalogSettingSeeds,
  AppControlCatalogSourceDocuments,
  AppControlCapabilitySeeds,
  AppControlEffectModeSeeds,
  AppControlTargetScopeSeeds,
  type AppControlCatalogSettingSeed,
  type AppControlGuideCatalogSettingSeed,
} from './app-control-catalog-data';
import { AppControlGuideCatalogData } from './app-control-guide-catalog-data';
import { guideEffectStatusFor } from './app-control-catalog-guide-effect';
import { guideCapabilityStateFor } from './app-control-catalog-guide-capability';
import {
  guideCapabilityRequirementFor,
  guideFallbackFor,
  guideProofRequirementFor,
} from './app-control-catalog-guide-requirements';
import { guideRuntimeOwnerFor } from './app-control-catalog-guide-runtime';
import { guideControlKindFor, guideOptionsFor } from './app-control-catalog-guide-control-kind';
import { guidePolicyLaneFor } from './app-control-catalog-guide-policy';
import { guideHelperTextFor, questionFromGuideText } from './app-control-catalog-guide-text';
import { acceptedOptionCountForSeeds, optionsFromGuideSeeds, optionsFromSeeds } from './app-control-catalog-option-seeds';
import { cardKindFor, enabledConditionsFor, helperTextFor, layoutHintsFor, validationRulesFor, visibilityConditionsFor } from './app-control-catalog-options';
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
  type AppControlCatalogGroup,
  type AppControlCatalogSection,
  type AppControlCatalogSetting,
  type AppControlEffectivePolicy,
  type AppControlPolicyValue,
  type AppControlUpdateCommand,
} from './app-control-catalog-schema';
import { ParentContractSchemaVersion } from '@ocentra-parent/schema-domain/family-reference-primitives';

type GroupDraft = Omit<AppControlCatalogGroup, 'settings'> & {
  readonly settings: AppControlCatalogSetting[];
};

type SectionDraft = Omit<AppControlCatalogSection, 'groups'> & {
  readonly groups: Map<string, GroupDraft>;
};

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

export const AppControlGuideSettingCount = AppControlGuideCatalogData.length;
export const AppControlFullCatalogSettingCount = AppControlCatalogSettingSeeds.length + AppControlGuideSettingCount;

const AppControlManifestSettings = buildManifestSettings(AppControlCatalogSettingSeeds);
const AppControlFullCatalogSettings = [
  ...AppControlManifestSettings,
  ...buildGuideSettings(AppControlGuideCatalogData, AppControlCatalogSettingSeeds.length),
];

export const BaselineAppControlAuthoringCatalog: AppControlAuthoringCatalog = AppControlAuthoringCatalogSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  catalogId: AppControlCatalogIdSchema.parse('app-control-authoring-v1'),
  sidePanelCategory: 'apps',
  sourceDocuments: [...AppControlCatalogSourceDocuments],
  settingCount: AppControlCatalogSettingSeeds.length,
  acceptedOptionCount: acceptedOptionCountForSeeds(AppControlCatalogSettingSeeds),
  targetScopeOptions: AppControlTargetScopeSeeds.map((scope) => AppControlTargetScopeSchema.parse(scope)),
  effectModeOptions: AppControlEffectModeSeeds.map((mode) => AppControlEffectModeSchema.parse(mode)),
  sections: buildSectionsFromSettings(AppControlManifestSettings),
});

export const BaselineAppControlFullCatalog: AppControlAuthoringCatalog = AppControlAuthoringCatalogSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  catalogId: AppControlCatalogIdSchema.parse('app-control-full-catalog-v1'),
  sidePanelCategory: 'apps',
  sourceDocuments: [...AppControlCatalogSourceDocuments],
  settingCount: AppControlFullCatalogSettingCount,
  acceptedOptionCount: AppControlFullCatalogSettings.reduce(
    (count, setting) => count + setting.acceptedOptions.length,
    0
  ),
  targetScopeOptions: AppControlTargetScopeSeeds.map((scope) => AppControlTargetScopeSchema.parse(scope)),
  effectModeOptions: AppControlEffectModeSeeds.map((mode) => AppControlEffectModeSchema.parse(mode)),
  sections: buildSectionsFromSettings(AppControlFullCatalogSettings),
});

export function appControlCatalogSettings(catalog = BaselineAppControlAuthoringCatalog) {
  return catalog.sections.flatMap((section) => section.groups.flatMap((group) => group.settings));
}

export function appControlFullCatalogSettings(catalog = BaselineAppControlFullCatalog) {
  return appControlCatalogSettings(catalog);
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

function bySourceOrder<T extends { readonly sourceOrder: number }>(left: T, right: T): number {
  return left.sourceOrder - right.sourceOrder;
}

function buildManifestSettings(seeds: readonly AppControlCatalogSettingSeed[]): AppControlCatalogSetting[] {
  return seeds.map((seed, index) => buildSetting(seed, index + 1));
}

function buildSectionsFromSettings(settings: readonly AppControlCatalogSetting[]): AppControlCatalogSection[] {
  const sections = new Map<string, SectionDraft>();
  for (const setting of settings) {
    const sectionKey = String(setting.sectionId);
    const groupKey = String(setting.groupId);
    const section = getSectionDraftFromSetting(sections, setting);
    const group = getGroupDraft(section, groupKey, setting.sourceHeadingPath[1] ?? section.title, setting.sourceOrder);
    group.settings.push(setting);
    section.groups.set(groupKey, group);
    sections.set(sectionKey, section);
  }
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
    sourceLine: 0,
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

function buildGuideSettings(
  seeds: readonly AppControlGuideCatalogSettingSeed[],
  sourceOrderOffset: number
): AppControlCatalogSetting[] {
  return seeds.map((seed) => buildGuideSetting(seed, sourceOrderOffset + seed[7]));
}

function buildGuideSetting(seed: AppControlGuideCatalogSettingSeed, sourceOrder: number): AppControlCatalogSetting {
  const [sectionId, sectionTitle, , groupId, groupTitle, , settingId, , sourceLine, sourceText] = seed;
  const controlKind = guideControlKindFor(sectionTitle, groupTitle, sourceText);
  const optionSeeds = guideOptionsFor(sourceText);
  const effectStatus = guideEffectStatusFor(sectionTitle, groupTitle, sourceText);
  const runtimeOwner = guideRuntimeOwnerFor(sectionTitle, groupTitle, sourceText);
  const optionsForSetting = optionsFromGuideSeeds(settingId, optionSeeds);
  return {
    sidePanelCategory: 'apps',
    policyLane: AppControlUiTabSchema.parse(guidePolicyLaneFor(sectionTitle, groupTitle, sourceText)),
    sectionId: AppControlSectionIdSchema.parse(sectionId),
    groupId: AppControlGroupIdSchema.parse(groupId),
    settingId: AppControlSettingIdSchema.parse(settingId),
    sourceDocument: 'docs/app-control-capability-guide.md',
    sourceHeadingPath: [sectionTitle, groupTitle],
    sourceSection: AppControlSectionIdSchema.parse(sectionId),
    sourceGroup: AppControlGroupIdSchema.parse(groupId),
    sourceOrder,
    sourceLine,
    sourceText,
    originalSourceText: sourceText,
    question: questionFromGuideText(sourceText),
    uiQuestionText: questionFromGuideText(sourceText),
    helperText: guideHelperTextFor(sectionTitle, groupTitle, sourceText),
    displayOrder: sourceOrder,
    controlKind: AppControlKindSchema.parse(controlKind),
    cardKind: AppControlCardKindSchema.parse(cardKindFor(controlKind, optionsForSetting)),
    layoutHints: layoutHintsFor(controlKind, optionsForSetting),
    options: optionsForSetting,
    acceptedOptions: optionsForSetting,
    targetScopeOptions: AppControlTargetScopeSeeds.map((scope) => AppControlTargetScopeSchema.parse(scope)),
    effectModeOptions: AppControlEffectModeSeeds.map((mode) => AppControlEffectModeSchema.parse(mode)),
    writesTo: AppControlWritesToPathSchema.parse(`/appPolicy/catalogGuide/${settingId}`),
    effectKey: AppControlSettingIdSchema.parse(settingId),
    effectStatus: AppControlEffectStatusSchema.parse(effectStatus),
    runtimeOwner: AppControlRuntimeOwnerSchema.parse(runtimeOwner),
    capabilityState: guideCapabilityStateFor(effectStatus),
    capabilityRequirement: guideCapabilityRequirementFor(sectionTitle, groupTitle, sourceText),
    proofRequirement: guideProofRequirementFor(sectionTitle, groupTitle, sourceText),
    visibilityConditions: visibilityConditionsFor(settingId),
    enabledConditions: enabledConditionsFor(settingId, effectStatus, runtimeOwner),
    validationRules: validationRulesFor(settingId, controlKind),
    unsafeOrUnsupportedFallback: guideFallbackFor(effectStatus, sectionTitle, groupTitle, sourceText),
  };
}

function getSectionDraftFromSetting(
  sections: Map<string, SectionDraft>,
  setting: AppControlCatalogSetting
): SectionDraft {
  const sectionId = String(setting.sectionId);
  const existing = sections.get(sectionId);
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

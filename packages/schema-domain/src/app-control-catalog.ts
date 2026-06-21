import {
  AppControlCatalogSettingSeeds,
  AppControlCatalogSourceDocuments,
  AppControlCapabilitySeeds,
  AppControlEffectModeSeeds,
  AppControlTargetScopeSeeds,
  type AppControlCatalogDefaultValue,
  type AppControlCatalogOptionSeed,
  type AppControlCatalogSettingSeed,
  type AppControlGuideCatalogSettingSeed,
} from './app-control-catalog-data';
import { AppControlGuideCatalogData } from './app-control-guide-catalog-data';
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
} from '@ocentra-parent/schema-domain/app-control-catalog-schema';
import { ParentContractSchemaVersion } from '@ocentra-parent/schema-domain/family-reference-primitives';

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

function guidePolicyLaneFor(sectionTitle: string, groupTitle: string, sourceText: string) {
  const searchable = `${sectionTitle} ${groupTitle} ${sourceText}`;
  if (/^Capability matrix row \|/u.test(sourceText)) {
    return 'evidence';
  }
  if (/time|duration|budget|schedule|timer|foreground today|grace/iu.test(searchable)) {
    return 'schedule';
  }
  if (/approval|ask parent|approve|deny|extend|unanswered/iu.test(searchable)) {
    return 'approvals';
  }
  if (/audit|retention|custody|report|redact|storage|journal|visible|summary/iu.test(searchable)) {
    return /report|summary|visible|parent sees/iu.test(searchable) ? 'reports' : 'audit';
  }
  if (
    /install|uninstall|setup|mdm|managed-device|device owner|entitlement|supervised|custody model/iu.test(searchable)
  ) {
    return 'setup';
  }
  if (/enforce|block|shield|suspend|hide|terminate|strict action|adapter result|rollback|launch/iu.test(searchable)) {
    return 'enforcement';
  }
  if (
    /evidence|inventory|process|window|foreground|package|identity|category|session|running|unknown|proof/iu.test(
      searchable
    )
  ) {
    return 'evidence';
  }
  return 'rules';
}

function guideControlKindFor(sectionTitle: string, groupTitle: string, sourceText: string): AppControlKind {
  const searchable = `${sectionTitle} ${groupTitle} ${sourceText}`.toLowerCase();
  if (/^Capability matrix row \|/u.test(sourceText)) {
    return 'read-only-status';
  }
  if (/minutes|seconds|hours|days|duration|budget|timer/u.test(searchable)) {
    return 'number';
  }
  if (/audit|retention|custody|delete|redact/u.test(searchable)) {
    return 'retention';
  }
  if (/actions|terminate|block|shield|suspend|hide|install|uninstall|launch/u.test(searchable)) {
    return 'action-list';
  }
  if (/targets|identity|category|package|bundle|process|window|unknown app/u.test(searchable)) {
    return 'multi-choice';
  }
  return guideOptionsFor(sourceText).length > 0 ? 'single-choice' : 'toggle';
}

function guideOptionsFor(sourceText: string): readonly AppControlCatalogOptionSeed[] {
  const matrixOptions = matrixOptionSeedsFromSourceText(sourceText);
  if (matrixOptions.length > 0) {
    return matrixOptions;
  }
  const colonIndex = sourceText.indexOf(':');
  if (colonIndex === -1) {
    return [];
  }
  return sourceText
    .slice(colonIndex + 1)
    .replace(/\.$/u, '')
    .split(/,|;|\bor\b/iu)
    .map((part) => cleanOptionLabel(part))
    .filter((part) => part.length > 0);
}

function matrixOptionSeedsFromSourceText(sourceText: string): readonly AppControlCatalogOptionSeed[] {
  if (!/^Capability matrix row \|/u.test(sourceText)) {
    return [];
  }
  return sourceText
    .split(' | ')
    .slice(1)
    .map((part) => {
      const separatorIndex = part.indexOf('=');
      const heading = separatorIndex === -1 ? 'Cell' : part.slice(0, separatorIndex);
      const value = separatorIndex === -1 ? part : part.slice(separatorIndex + 1);
      return {
        value: `matrix-${slugToken(heading)}`,
        label: `${heading}: ${value}`,
        meaning: `Capability matrix answer for ${heading}.`,
      };
    });
}

function guideEffectStatusFor(sectionTitle: string, groupTitle: string, sourceText: string) {
  const searchable = `${sectionTitle} ${groupTitle} ${sourceText}`;
  if (/manual|required setup|mdm|device-owner|supervised|entitlement|custody model|AppLocker|WDAC/iu.test(searchable)) {
    return 'manual-required';
  }
  if (/permission|visibility-limited|privacy|protected|unreadable|uncontrollable/iu.test(searchable)) {
    return /limited|partial|varies/iu.test(searchable) ? 'permission-limited' : 'permission-required';
  }
  if (
    /proof|unknown|confidence|must not|does not prove|cannot prove|without proof|source\/confidence/iu.test(searchable)
  ) {
    return 'proof-required';
  }
  if (/future|later|not yet|planned|missing|gap/iu.test(sourceText)) {
    return 'future-gap';
  }
  if (/unsupported|unavailable|stale|degraded|fallback|adapter-error|varies|partial|miss/iu.test(searchable)) {
    return 'degraded';
  }
  if (/audit|retention|report|redact|local-first|parent-owned|never collect|show/iu.test(searchable)) {
    return 'already-represented';
  }
  return 'needs-effect-wiring';
}

function guideRuntimeOwnerFor(sectionTitle: string, groupTitle: string, sourceText: string) {
  const searchable = `${sectionTitle} ${groupTitle} ${sourceText}`;
  if (/audit|retention|custody|report|redact|local-first|parent-owned|journal|storage/iu.test(searchable)) {
    return 'parent-owned-storage';
  }
  if (
    /manual|mdm|device-owner|supervised|entitlement|AppLocker|WDAC|platform management|permission/iu.test(searchable)
  ) {
    return 'os-adapter';
  }
  if (/policy|decision|rule|fallback|compile|deterministic/iu.test(searchable)) {
    return 'parent-domain';
  }
  return 'child-agent';
}

function guideCapabilityStateFor(effectStatus: string): AppControlCapabilityState {
  if (effectStatus === 'manual-required') {
    return 'manual-required';
  }
  if (effectStatus === 'permission-required') {
    return 'permission-required';
  }
  if (effectStatus === 'permission-limited') {
    return 'permission-limited';
  }
  if (effectStatus === 'future-gap') {
    return 'future-gap';
  }
  if (effectStatus === 'degraded') {
    return 'degraded';
  }
  if (effectStatus === 'proof-required') {
    return 'protected';
  }
  return 'available';
}

function guideCapabilityRequirementFor(sectionTitle: string, groupTitle: string, sourceText: string): string {
  const searchable = `${sectionTitle} ${groupTitle} ${sourceText}`;
  if (/unknown/iu.test(searchable)) {
    return 'unknown-app-state-must-remain-explicit';
  }
  if (
    /block|shield|suspend|hide|terminate|install|uninstall|AppLocker|WDAC|MDM|device-owner|entitlement/iu.test(
      searchable
    )
  ) {
    return 'platform-adapter-proof-required-before-product-claim';
  }
  if (/inventory|package|bundle|identity|process|window|foreground|duration|session/iu.test(searchable)) {
    return 'typed-local-app-evidence-required';
  }
  if (/audit|retention|report|redact|custody/iu.test(searchable)) {
    return 'parent-owned-local-storage-and-redaction';
  }
  return 'app-control-capability-registry';
}

function guideProofRequirementFor(sectionTitle: string, groupTitle: string, sourceText: string): string | null {
  const searchable = `${sectionTitle} ${groupTitle} ${sourceText}`;
  if (
    /broad app blocking|block launch|AppLocker|WDAC|shield|suspend|hide|install|uninstall|MDM|device-owner|entitlement/iu.test(
      searchable
    )
  ) {
    return 'strict app control requires real platform adapter or managed-device proof.';
  }
  if (/unknown|confidence|identity|category|proof|evidence|foreground|duration|process|window/iu.test(searchable)) {
    return 'app claims require fresh evidence references with confidence and custody.';
  }
  return null;
}

function guideFallbackFor(effectStatus: string, sectionTitle: string, groupTitle: string, sourceText: string): string {
  if (/unknown/iu.test(`${sectionTitle} ${groupTitle} ${sourceText}`)) {
    return 'Keep unknown apps labeled unknown; do not promote to known, risky, game, or blocked without proof.';
  }
  if (effectStatus === 'manual-required') {
    return 'Disable strict action or show manual-required until platform setup and adapter proof exist.';
  }
  if (effectStatus === 'permission-required' || effectStatus === 'permission-limited') {
    return 'Show permission-limited state and compile observe, warn, or ask fallback instead of hidden enforcement.';
  }
  if (effectStatus === 'degraded') {
    return 'Render degraded capability and keep unsupported behavior out of compiled enforcement plans.';
  }
  if (effectStatus === 'proof-required') {
    return 'Require evidence proof before strict effect; otherwise fall back to observe, warn, ask, or unavailable.';
  }
  if (effectStatus === 'future-gap') {
    return 'Expose as future or planning-only; do not claim current runtime behavior.';
  }
  return 'Portal renders the control; child-agent/runtime ownership remains explicit.';
}

function guideHelperTextFor(sectionTitle: string, groupTitle: string, sourceText: string): string {
  const proof = guideProofRequirementFor(sectionTitle, groupTitle, sourceText);
  return proof ?? guideCapabilityRequirementFor(sectionTitle, groupTitle, sourceText);
}

function questionFromGuideText(sourceText: string): string {
  const trimmed = sourceText.replace(/\.$/u, '');
  if (trimmed.endsWith('?')) {
    return trimmed;
  }
  const colonIndex = trimmed.indexOf(':');
  if (colonIndex !== -1) {
    return `Configure ${trimmed.slice(0, colonIndex).toLowerCase()}.`;
  }
  return `Represent ${trimmed.charAt(0).toLowerCase()}${trimmed.slice(1)}?`;
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

function optionsFromGuideSeeds(
  settingId: string,
  optionSeeds: readonly AppControlCatalogOptionSeed[]
): AppControlCatalogOption[] {
  if (optionSeeds.length > 0) {
    return optionSeeds.map((optionSeed) => optionFromSeed(settingId, optionSeed, null));
  }
  return [
    optionFromSeed(
      settingId,
      { value: 'represented', label: 'Represented', meaning: 'This guide control is represented in the catalog.' },
      null
    ),
    optionFromSeed(
      settingId,
      { value: 'not-represented', label: 'Not represented', meaning: 'This guide control is not selected.' },
      null
    ),
  ];
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

function slugToken(value: string): string {
  const slugged = value
    .toLowerCase()
    .replace(/[^a-z0-9]+/gu, '-')
    .replace(/^-+|-+$/gu, '');
  return slugged.length > 0 ? slugged : 'item';
}

function cleanOptionLabel(value: string): string {
  return titleFromToken(value.trim().replace(/\.$/u, '').replace(/\s+/gu, '-'));
}

function bySourceOrder<T extends { readonly sourceOrder: number }>(left: T, right: T): number {
  return left.sourceOrder - right.sourceOrder;
}

import {
  GameControlCapabilitySeeds,
  GameControlCapabilityTruthSeeds,
  GameControlCatalogSettingSeeds,
  GameControlCatalogSourceDocuments,
  GameControlUpdateCommandSeeds,
} from './game-control-catalog-data';
import {
  GameControlAuthoringManifestSchema,
  GameControlCapabilityIdSchema,
  GameControlCapabilityRegistrySchema,
  GameControlCatalogIdSchema,
  GameControlCommandIdSchema,
  GameControlEffectivePolicyDocumentSchema,
  GameControlEffectKeySchema,
  GameControlGroupIdSchema,
  GameControlOptionIdSchema,
  GameControlPolicyDocumentIdSchema,
  GameControlPolicyHashSchema,
  GameControlPolicyRevisionSchema,
  GameControlPolicyUpdateCommandSchema,
  GameControlPolicyValueDocumentSchema,
  GameControlRuleIdSchema,
  GameControlSectionIdSchema,
  GameControlSettingIdSchema,
  GameControlSourceDocumentSchema,
  GameControlWritePathSchema,
  type GameControlAuthoringManifest,
  type GameControlCapabilityState,
  type GameControlCapabilityTruth,
  type GameControlCatalogOptionSeed,
  type GameControlCatalogSettingSeed,
  type GameControlControlType,
  type GameControlEffectivePolicyDocument,
  type GameControlEffectMode,
  type GameControlEffectStatus,
  type GameControlGroup,
  type GameControlLane,
  type GameControlOption,
  type GameControlPolicyLane,
  type GameControlPolicyValueDocument,
  type GameControlRuntimeOwner,
  type GameControlSection,
  type GameControlSetting,
  type GameControlTargetScope,
  type GameControlUiCardType,
} from './game-control-catalog-schema';
import { ParentContractSchemaVersion } from '@ocentra-parent/schema-domain/family-reference-primitives';

type GameControlGroupDraft = Omit<GameControlGroup, 'settings'> & {
  readonly settings: GameControlSetting[];
};

type GameControlSectionDraft = Omit<GameControlSection, 'groups'> & {
  readonly groups: Map<string, GameControlGroupDraft>;
};

type GameControlLaneDraft = Omit<GameControlLane, 'sections'> & {
  readonly sections: Map<string, GameControlSectionDraft>;
};

const GameControlCatalogManifestId = GameControlCatalogIdSchema.parse('game-control-authoring-v1');
const GameControlPolicyDocumentId = GameControlPolicyDocumentIdSchema.parse('game-control-policy-default-v1');
const GameControlEffectivePolicyDocumentId = GameControlPolicyDocumentIdSchema.parse(
  'game-control-effective-default-v1'
);
const GameControlPolicyRevision = GameControlPolicyRevisionSchema.parse('game-control-policy-revision-1');

export const GameControlSourceDocument = GameControlSourceDocumentSchema.parse('docs/game-control-schema-proposal.md');
export const GameControlCapabilityGuideDocument = GameControlSourceDocumentSchema.parse(
  'docs/game-control-capability-guide.md'
);

export const GameControlTargetScopeOptions = [
  'family',
  'per-child',
  'per-device',
  'per-platform',
  'per-app',
  'per-game',
  'per-browser',
  'per-network',
] as const satisfies readonly GameControlTargetScope[];

export const GameControlEffectModeOptions = [
  'off',
  'observe',
  'dry-run',
  'warn',
  'notify',
  'ask',
  'limit',
  'block',
  'enforce',
  'audit-only',
] as const satisfies readonly GameControlEffectMode[];

const GameControlLaneOrder = [
  'rules',
  'schedule',
  'approvals',
  'enforcement',
  'audit',
  'evidence',
  'reports',
  'data',
] as const satisfies readonly GameControlPolicyLane[];

const GameControlLaneTitles = {
  rules: 'Rules',
  schedule: 'Schedule',
  approvals: 'Approvals',
  enforcement: 'Enforcement',
  audit: 'Audit',
  evidence: 'Evidence',
  reports: 'Reports',
  data: 'Data',
} as const satisfies Record<GameControlPolicyLane, string>;

export const BaselineGameControlAuthoringManifest: GameControlAuthoringManifest =
  GameControlAuthoringManifestSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    manifestId: GameControlCatalogManifestId,
    policyKind: 'game-control',
    sidePanelCategory: 'games',
    sourceDocuments: GameControlCatalogSourceDocuments,
    settingCount: GameControlCatalogSettingSeeds.length,
    acceptedOptionCount: gameControlSourceOptionCount(),
    targetScopeOptions: GameControlTargetScopeOptions,
    effectModeOptions: GameControlEffectModeOptions,
    lanes: buildLanes(GameControlCatalogSettingSeeds),
    capabilityTruths: buildCapabilityTruths(),
    capabilityRegistry: buildCapabilityRegistry(),
  });

export const BaselineGameControlPolicyValueDocument = parseCompleteGameControlPolicyValueDocument({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  policyKind: 'game-control',
  documentId: GameControlPolicyDocumentId,
  revision: GameControlPolicyRevision,
  manifestId: GameControlCatalogManifestId,
  targetScopes: ['family', 'per-child', 'per-device'],
  settings: gameControlCatalogSettings().map(defaultPolicyValueSetting),
});

export const BaselineGameControlEffectivePolicyDocument: GameControlEffectivePolicyDocument =
  GameControlEffectivePolicyDocumentSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    policyKind: 'game-control',
    documentId: GameControlEffectivePolicyDocumentId,
    compiledFromDocumentId: GameControlPolicyDocumentId,
    compiledFromRevision: GameControlPolicyRevision,
    effectivePolicyHash: GameControlPolicyHashSchema.parse('game-control-effective-policy-hash-v1'),
    targetScopes: ['family', 'per-child', 'per-device'],
    settings: gameControlCatalogSettings().map((setting) => ({
      settingId: setting.settingId,
      effectKey: setting.effectKey,
      effectStatus: setting.effectStatus,
      runtimeOwner: setting.runtimeOwner,
      capabilityState: setting.capabilityState,
      proofRequirement: setting.proofRequirement,
      fallbackDecision: setting.unsafeOrUnsupportedFallback ?? 'Compile authored intent only after capability proof.',
    })),
  });

export const BaselineGameControlPolicyUpdateCommands = GameControlUpdateCommandSeeds.map((seed, index) =>
  GameControlPolicyUpdateCommandSchema.parse({
    commandId: GameControlCommandIdSchema.parse(`game-control-update-command-${index + 1}`),
    commandType: seed[0],
    policyKind: 'game-control',
    targetScopes: ['family', 'per-child', 'per-device'],
    expectedRevision: index === 0 ? null : GameControlPolicyRevision,
    purpose: seed[1],
  })
);

export function gameControlCatalogSettings(catalog = BaselineGameControlAuthoringManifest): GameControlSetting[] {
  return catalog.lanes.flatMap((lane) =>
    lane.sections.flatMap((section) => section.groups.flatMap((group) => group.settings))
  );
}

export function gameControlCatalogSections(catalog = BaselineGameControlAuthoringManifest): GameControlSection[] {
  return catalog.lanes.flatMap((lane) => lane.sections);
}

export function gameControlCatalogGroups(catalog = BaselineGameControlAuthoringManifest): GameControlGroup[] {
  return gameControlCatalogSections(catalog).flatMap((section) => section.groups);
}

export function gameControlCatalogAcceptedOptionCount(catalog = BaselineGameControlAuthoringManifest): number {
  return gameControlCatalogSettings(catalog).reduce((count, setting) => count + setting.acceptedOptions.length, 0);
}

export function gameControlCatalogSettingsByCardType(catalog = BaselineGameControlAuthoringManifest) {
  return countBy(gameControlCatalogSettings(catalog), (setting) => setting.uiCardType);
}

export function gameControlCatalogSettingsByEffectStatus(catalog = BaselineGameControlAuthoringManifest) {
  return countBy(gameControlCatalogSettings(catalog), (setting) => setting.effectStatus);
}

export function gameControlCapabilityStateCount(catalog = BaselineGameControlAuthoringManifest) {
  return countBy(catalog.capabilityRegistry.capabilities, (capability) => capability.state);
}

export function parseCompleteGameControlPolicyValueDocument(input: unknown): GameControlPolicyValueDocument {
  const document = GameControlPolicyValueDocumentSchema.parse(input);
  const settings = gameControlCatalogSettings();
  const expectedIds = new Set(settings.map((setting) => setting.settingId));
  const seenIds = new Set(document.settings.map((setting) => setting.settingId));
  if (seenIds.size !== document.settings.length) {
    throw new Error('Duplicate game policy setting value.');
  }
  if (seenIds.size !== expectedIds.size || [...expectedIds].some((settingId) => !seenIds.has(settingId))) {
    throw new Error('Game policy value document must include every authoring manifest setting.');
  }
  const optionIdsBySettingId = new Map(
    settings.map((setting) => [setting.settingId, new Set(setting.acceptedOptions.map((option) => option.optionId))])
  );
  for (const value of document.settings) {
    const allowedOptionIds = optionIdsBySettingId.get(value.settingId);
    if (allowedOptionIds === undefined) {
      throw new Error(`Unknown game policy setting ${value.settingId}.`);
    }
    for (const optionId of value.selectedOptionIds) {
      if (!allowedOptionIds.has(optionId)) {
        throw new Error(`Invalid game policy option ${optionId} for ${value.settingId}.`);
      }
    }
  }
  return document;
}

function buildLanes(seeds: readonly GameControlCatalogSettingSeed[]): GameControlLane[] {
  const laneDrafts = new Map<GameControlPolicyLane, GameControlLaneDraft>();
  for (const seed of seeds) {
    const setting = buildSetting(seed);
    const lane = getLaneDraft(laneDrafts, setting.policyLane);
    const section = getSectionDraft(lane, seed, setting.policyLane);
    const group = getGroupDraft(section, seed);
    group.settings.push(setting);
    section.groups.set(seed.groupId, group);
    lane.sections.set(seed.sectionId, section);
    laneDrafts.set(setting.policyLane, lane);
  }
  return GameControlLaneOrder.filter((laneId) => laneDrafts.has(laneId)).map((laneId) =>
    finalizeLane(laneDrafts.get(laneId) as GameControlLaneDraft)
  );
}

function buildSetting(seed: GameControlCatalogSettingSeed): GameControlSetting {
  const settingOptions = optionsForSeed(seed);
  const controlType = controlTypeForSeed(seed);
  const policyLane = laneForSeed(seed);
  return {
    sidePanelCategory: 'games',
    policyLane,
    sourceSection: GameControlSectionIdSchema.parse(seed.sectionId),
    sourceGroup: GameControlGroupIdSchema.parse(seed.groupId),
    sectionId: GameControlSectionIdSchema.parse(seed.sectionId),
    groupId: GameControlGroupIdSchema.parse(seed.groupId),
    settingId: GameControlSettingIdSchema.parse(seed.settingId),
    sourceDocument: GameControlSourceDocument,
    sourceHeadingPath: [seed.sectionTitle, seed.groupTitle],
    originalSourceText: seed.question,
    uiQuestionText: seed.question,
    helperText: helperTextForSeed(seed),
    displayOrder: seed.sectionOrder * 100 + seed.settingOrder,
    controlType,
    uiCardType: cardTypeFor(controlType, settingOptions),
    layoutHints: layoutHintsFor(controlType, settingOptions),
    acceptedOptions: settingOptions,
    targetScopeOptions: GameControlTargetScopeOptions,
    effectModeOptions: GameControlEffectModeOptions,
    writesTo: GameControlWritePathSchema.parse(seed.writesTo),
    effectKey: GameControlEffectKeySchema.parse(`game-control-effect-${slug(seed.settingId)}`),
    effectStatus: effectStatusForSeed(seed),
    runtimeOwner: runtimeOwnerForSeed(seed),
    capabilityState: capabilityStateForSeed(seed),
    capabilityRequirement: capabilityRequirementForSeed(seed),
    proofRequirement: proofRequirementForSeed(seed),
    visibilityConditions: visibilityConditionsForSeed(seed),
    enabledConditions: enabledConditionsForSeed(seed),
    validationRules: validationRulesForSeed(seed),
    unsafeOrUnsupportedFallback: fallbackForSeed(seed),
  };
}

function buildCapabilityTruths(): GameControlCapabilityTruth[] {
  return GameControlCapabilityTruthSeeds.map((truth) => ({
    truthId: GameControlRuleIdSchema.parse(truth.truthId),
    sourceDocument: GameControlCapabilityGuideDocument,
    sourceHeadingPath: truth.sourceHeadingPath,
    originalSourceText: truth.originalSourceText,
    appliesToSettingIds: truth.appliesToSettingIds.map((settingId) => GameControlSettingIdSchema.parse(settingId)),
    capabilityState: truth.capabilityState,
  }));
}

function buildCapabilityRegistry() {
  return GameControlCapabilityRegistrySchema.parse({
    registryId: GameControlCatalogIdSchema.parse('game-control-capability-registry-v1'),
    sidePanelCategory: 'games',
    sourceDocument: GameControlSourceDocument,
    capabilities: GameControlCapabilitySeeds.map((capability) => ({
      capabilityId: GameControlCapabilityIdSchema.parse(capability.capabilityId),
      state: capability.state,
      proofRequirement: capability.proofRequirement,
      affectsSettingIds: capability.affectsSettingIds.map((settingId) => GameControlSettingIdSchema.parse(settingId)),
    })),
  });
}

function optionsForSeed(seed: GameControlCatalogSettingSeed): GameControlOption[] {
  if (seed.controlType === 'boolean') {
    return [
      optionFromSeedValue(seed, ['enabled', 'Enabled', null]),
      optionFromSeedValue(seed, ['disabled', 'Disabled', null]),
    ];
  }
  return seed.options.map((optionSeed) => optionFromSeedValue(seed, optionSeed));
}

function optionFromSeedValue(seed: GameControlCatalogSettingSeed, optionSeed: GameControlCatalogOptionSeed) {
  return {
    optionId: GameControlOptionIdSchema.parse(`game-control-option-${slug(seed.settingId)}-${slug(optionSeed[0])}`),
    label: optionSeed[1],
    value: optionSeed[0],
    originalSourceText: optionSeed[0],
    meaning: optionSeed[2],
    defaultSelected: optionDefaultSelected(seed.defaultValue, optionSeed[0]),
  };
}

function optionDefaultSelected(defaultValue: unknown, optionValue: string): boolean {
  if (Array.isArray(defaultValue)) {
    return defaultValue.includes(optionValue);
  }
  if (typeof defaultValue === 'boolean') {
    return defaultValue ? optionValue === 'enabled' : optionValue === 'disabled';
  }
  return defaultValue === optionValue;
}

function defaultPolicyValueSetting(setting: GameControlSetting) {
  const seed = GameControlCatalogSettingSeeds.find((candidate) => candidate.settingId === setting.settingId);
  if (seed === undefined) {
    throw new Error(`Missing game control seed ${setting.settingId}`);
  }
  return {
    settingId: setting.settingId,
    writesTo: setting.writesTo,
    selectedOptionIds: setting.acceptedOptions
      .filter((option) => option.defaultSelected)
      .map((option) => option.optionId),
    booleanValue: typeof seed.defaultValue === 'boolean' ? seed.defaultValue : null,
    numericValue: typeof seed.defaultValue === 'number' ? seed.defaultValue : null,
    ruleItemCount: seed.controlType === 'rule-list' && Array.isArray(seed.defaultValue) ? seed.defaultValue.length : 0,
  };
}

function getLaneDraft(
  laneDrafts: Map<GameControlPolicyLane, GameControlLaneDraft>,
  laneId: GameControlPolicyLane
): GameControlLaneDraft {
  const existing = laneDrafts.get(laneId);
  if (existing !== undefined) {
    return existing;
  }
  return {
    laneId,
    title: GameControlLaneTitles[laneId],
    displayOrder: GameControlLaneOrder.indexOf(laneId) + 1,
    sections: new Map(),
  };
}

function getSectionDraft(
  lane: GameControlLaneDraft,
  seed: GameControlCatalogSettingSeed,
  policyLane: GameControlPolicyLane
): GameControlSectionDraft {
  const existing = lane.sections.get(seed.sectionId);
  if (existing !== undefined) {
    return existing;
  }
  return {
    sectionId: GameControlSectionIdSchema.parse(seed.sectionId),
    title: seed.sectionTitle,
    purpose: seed.sectionPurpose,
    displayOrder: seed.sectionOrder,
    policyLane,
    groups: new Map(),
  };
}

function getGroupDraft(section: GameControlSectionDraft, seed: GameControlCatalogSettingSeed): GameControlGroupDraft {
  const existing = section.groups.get(seed.groupId);
  if (existing !== undefined) {
    return existing;
  }
  return {
    groupId: GameControlGroupIdSchema.parse(seed.groupId),
    title: seed.groupTitle,
    displayOrder: seed.groupOrder,
    settings: [],
  };
}

function finalizeLane(lane: GameControlLaneDraft): GameControlLane {
  return {
    laneId: lane.laneId,
    title: lane.title,
    displayOrder: lane.displayOrder,
    sections: [...lane.sections.values()].sort(byDisplayOrder).map(finalizeSection),
  };
}

function finalizeSection(section: GameControlSectionDraft): GameControlSection {
  return {
    sectionId: section.sectionId,
    title: section.title,
    purpose: section.purpose,
    displayOrder: section.displayOrder,
    policyLane: section.policyLane,
    groups: [...section.groups.values()].sort(byDisplayOrder).map(finalizeGroup),
  };
}

function finalizeGroup(group: GameControlGroupDraft): GameControlGroup {
  return {
    groupId: group.groupId,
    title: group.title,
    displayOrder: group.displayOrder,
    settings: group.settings.sort(byDisplayOrder),
  };
}

function laneForSeed(seed: GameControlCatalogSettingSeed): GameControlPolicyLane {
  if (seed.sectionId === 'budgets') {
    return 'schedule';
  }
  if (seed.sectionId === 'approvals') {
    return 'approvals';
  }
  if (seed.sectionId === 'audit') {
    return 'audit';
  }
  if (seed.sectionId === 'session-evidence') {
    return 'evidence';
  }
  if (seed.sectionId === 'reports') {
    return 'reports';
  }
  if (['native-games', 'launcher-games', 'browser-cloud-games'].includes(seed.sectionId)) {
    return 'enforcement';
  }
  return 'rules';
}

function controlTypeForSeed(seed: GameControlCatalogSettingSeed): GameControlControlType {
  if (seed.controlType === 'boolean') {
    return 'toggle';
  }
  if (seed.controlType === 'single-choice') {
    return 'single-choice';
  }
  if (seed.controlType === 'multi-choice') {
    return 'multi-choice';
  }
  if (seed.controlType === 'number') {
    return 'number';
  }
  if (seed.controlType === 'retention') {
    return 'retention';
  }
  if (seed.controlType === 'rule-list') {
    return 'rule-list';
  }
  return 'read-only-status';
}

function cardTypeFor(
  controlType: GameControlControlType,
  options: readonly GameControlOption[]
): GameControlUiCardType {
  if (controlType === 'toggle') {
    return 'toggle-card';
  }
  if (controlType === 'rule-list') {
    return 'rule-list-card';
  }
  if (controlType === 'retention') {
    return 'retention-card';
  }
  if (controlType === 'number') {
    return 'status-card';
  }
  if (controlType === 'multi-choice') {
    return options.length > 4 ? 'many-option-multi-choice' : 'normal-multi-choice';
  }
  return options.length > 4 ? 'many-option-single-choice' : 'compact-single-choice';
}

function layoutHintsFor(controlType: GameControlControlType, options: readonly GameControlOption[]) {
  const manyOptions = options.length > 4;
  const groupedControl = controlType === 'multi-choice' || controlType === 'rule-list';
  return {
    preferredColumnSpan: manyOptions || groupedControl ? 2 : 1,
    collapsible: manyOptions || groupedControl,
    searchableOptions: manyOptions,
    optionGroupCount: manyOptions ? Math.ceil(options.length / 4) : 1,
    showAsMatrixWhenLarge: manyOptions && controlType === 'multi-choice',
    showSelectedCount: controlType === 'multi-choice',
  };
}

function effectStatusForSeed(seed: GameControlCatalogSettingSeed): GameControlEffectStatus {
  if (seed.settingId === 'game.enabled' || seed.settingId === 'budgets.enabled') {
    return 'needs-wiring';
  }
  if (/neverCollect|reports\.|retention\.|custody\.|audit\./u.test(seed.settingId)) {
    return 'already-represented';
  }
  if (
    /browserCloud|requiredProof|durationCountingMode|strictActions|allowedTargetTypes|allowedActions/u.test(
      seed.settingId
    )
  ) {
    return 'proof-required';
  }
  if (/managementMode|launchers\.supportedKinds|whenManifestUnavailable/u.test(seed.settingId)) {
    return 'manual-required';
  }
  if (/classificationStates|whenProofUnavailable|launcherOnlyHandling/u.test(seed.settingId)) {
    return 'degraded';
  }
  return 'needs-wiring';
}

function runtimeOwnerForSeed(seed: GameControlCatalogSettingSeed): GameControlRuntimeOwner {
  if (/reports\.|retention\.|custody\.|audit\.|neverCollect/u.test(seed.settingId)) {
    return 'parent-owned-storage';
  }
  if (/approvals\./u.test(seed.settingId)) {
    return 'agent-protocol';
  }
  if (/nativeGames|launchers|browserCloud/u.test(seed.settingId)) {
    return 'os-adapter';
  }
  if (/rules\.|budgets\.|evidence\./u.test(seed.settingId)) {
    return 'child-agent';
  }
  return 'parent-domain';
}

function capabilityStateForSeed(seed: GameControlCatalogSettingSeed): GameControlCapabilityState {
  const effectStatus = effectStatusForSeed(seed);
  if (effectStatus === 'manual-required') {
    return 'manual-required';
  }
  if (effectStatus === 'degraded') {
    return 'degraded';
  }
  if (effectStatus === 'proof-required') {
    return 'protected';
  }
  if (effectStatus === 'future-gap') {
    return 'future-gap';
  }
  if (effectStatus === 'permission-required') {
    return 'permission-required';
  }
  return 'available';
}

function capabilityRequirementForSeed(seed: GameControlCatalogSettingSeed): string {
  if (/browserCloud/u.test(seed.settingId)) {
    return 'managed-browser-boundary-or-cloud-client-surface-proof';
  }
  if (/launchers/u.test(seed.settingId)) {
    return 'launcher-manifest-or-child-process-attribution-proof';
  }
  if (/nativeGames/u.test(seed.settingId)) {
    return 'local-process-package-window-proof-plus-platform-adapter-capability';
  }
  if (/inventory|evidence/u.test(seed.settingId)) {
    return 'child-device-local-evidence-read-model-with-source-confidence';
  }
  if (/approvals/u.test(seed.settingId)) {
    return 'validated-parent-approval-protocol-with-offline-fallback';
  }
  if (/reports|retention|custody|audit/u.test(seed.settingId)) {
    return 'child-local-or-parent-owned-storage-with-custody-labels';
  }
  return 'game-control-authoring-manifest';
}

function proofRequirementForSeed(seed: GameControlCatalogSettingSeed): string | null {
  if (/browserCloud/u.test(seed.settingId)) {
    return 'Browser-game title proof requires managed browser URL/title evidence or explicit platform integration.';
  }
  if (/nativeGames\.strictActions|rules\.allowedActions/u.test(seed.settingId)) {
    return 'Strict action proof requires current target recheck, adapter capability, audit, and rollback path.';
  }
  if (/evidence\.requiredProof|evidence\.durationCountingMode|budgets/u.test(seed.settingId)) {
    return 'Duration proof requires session id, process/package identity, observation gaps, and evidence refs.';
  }
  if (/launchers/u.test(seed.settingId)) {
    return 'Launcher proof must not treat launcher-only activity as active gameplay.';
  }
  if (/inventory/u.test(seed.settingId)) {
    return 'Unknown and possible-game evidence must stay labeled until deterministic proof exists.';
  }
  return null;
}

function fallbackForSeed(seed: GameControlCatalogSettingSeed): string {
  const effectStatus = effectStatusForSeed(seed);
  if (effectStatus === 'manual-required') {
    return 'Disable strict behavior and show manual-required setup until platform proof exists.';
  }
  if (effectStatus === 'degraded') {
    return 'Keep lower-confidence or degraded state visible and compile observe, ask, or report-only behavior.';
  }
  if (effectStatus === 'proof-required') {
    return 'Require explicit proof before strict enforcement; otherwise use observe, ask, or audit-only fallback.';
  }
  if (effectStatus === 'already-represented') {
    return 'Render and validate parent intent without claiming new runtime enforcement.';
  }
  return 'Portal authors intent only; child-agent runtime owns compile, persistence, evaluation, and audit.';
}

function helperTextForSeed(seed: GameControlCatalogSettingSeed): string {
  if (/browserCloud/u.test(seed.settingId)) {
    return 'Browser and cloud games keep their surface-specific proof boundary; network hints are not exact title proof.';
  }
  if (/launchers/u.test(seed.settingId)) {
    return 'Launcher activity is not automatically game play; manifests and child-process attribution remain separate.';
  }
  if (/nativeGames/u.test(seed.settingId)) {
    return 'Native game controls depend on process, package, foreground, and protected-process capability proof.';
  }
  return 'Portal renders typed parent intent while the child-device runtime owns proof, compile, fallback, and audit.';
}

function visibilityConditionsForSeed(seed: GameControlCatalogSettingSeed) {
  const conditions = [condition(`Visible when Games side-panel category renders ${seed.sectionTitle}.`)];
  if (seed.settingId !== 'game.enabled') {
    conditions.push(condition('Visible when game management is enabled or when Portal previews disabled controls.'));
  }
  return conditions;
}

function enabledConditionsForSeed(seed: GameControlCatalogSettingSeed) {
  return [
    condition('A family, child, or device target must be selected before writing game policy intent.'),
    condition(`Capability state must support ${effectStatusForSeed(seed)} presentation for this control.`),
  ];
}

function validationRulesForSeed(seed: GameControlCatalogSettingSeed) {
  const rules = [
    condition('Selected option ids must belong to this setting acceptedOptions list.'),
    condition('Portal writes only authored intent; child runtime owns compile, persistence, evaluation, and audit.'),
  ];
  const proofRequirement = proofRequirementForSeed(seed);
  if (proofRequirement !== null) {
    rules.push(condition(proofRequirement));
  }
  return rules;
}

function condition(description: string) {
  return {
    ruleId: GameControlRuleIdSchema.parse(`game-control-rule-${slug(description)}`),
    description,
  };
}

function gameControlSourceOptionCount(): number {
  return GameControlCatalogSettingSeeds.reduce((count, seed) => count + seed.options.length, 0);
}

function countBy<T>(items: readonly T[], keyFor: (item: T) => string) {
  const counts = new Map<string, number>();
  for (const item of items) {
    const key = keyFor(item);
    counts.set(key, (counts.get(key) ?? 0) + 1);
  }
  return Object.fromEntries([...counts.entries()].sort(([left], [right]) => left.localeCompare(right)));
}

function byDisplayOrder<T extends { readonly displayOrder: number }>(left: T, right: T): number {
  return left.displayOrder - right.displayOrder;
}

function slug(value: string): string {
  const normalized = value
    .toLowerCase()
    .replace(/&/gu, ' and ')
    .replace(/[^a-z0-9]+/gu, '-')
    .replace(/^-+|-+$/gu, '')
    .replace(/-{2,}/gu, '-');
  return normalized.length > 0 ? normalized : 'value';
}

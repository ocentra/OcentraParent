import { GameControlCatalogSettingSeeds, GameControlCapabilitySeeds, GameControlCapabilityTruthSeeds } from './game-control-catalog-data';
import {
  GameControlCapabilityGuideDocument,
  GameControlLaneOrder,
  GameControlLaneTitles,
  GameControlSourceDocument,
  GameControlTargetScopeOptions,
  GameControlEffectModeOptions,
  GameControlGroupDraft,
  GameControlLaneDraft,
  GameControlSectionDraft,
  byDisplayOrder,
  slug,
} from './game-control-catalog-core';
import {
  GameControlCapabilityIdSchema,
  GameControlCapabilityRegistrySchema,
  GameControlEffectKeySchema,
  GameControlGroupIdSchema,
  GameControlOptionIdSchema,
  GameControlRuleIdSchema,
  GameControlSectionIdSchema,
  GameControlSettingIdSchema,
  GameControlWritePathSchema,
  type GameControlCatalogOptionSeed,
  type GameControlCatalogSettingSeed,
  type GameControlCapabilityTruth,
  type GameControlLane,
  type GameControlOption,
  type GameControlPolicyLane,
  type GameControlSetting,
} from './game-control-catalog-schema';
import { cardTypeFor, controlTypeForSeed, layoutHintsFor } from './game-control-catalog-ui';
import { capabilityRequirementForSeed, runtimeOwnerForSeed } from './game-control-catalog-owner';
import { capabilityStateForSeed, effectStatusForSeed } from './game-control-catalog-state';
import { helperTextForSeed, visibilityConditionsForSeed } from './game-control-catalog-display';
import { proofRequirementForSeed, validationRulesForSeed, enabledConditionsForSeed, fallbackForSeed } from './game-control-catalog-guidance';
import { laneForSeed } from './game-control-catalog-lane';

export function buildLanes(seeds: readonly GameControlCatalogSettingSeed[]): GameControlLane[] {
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

export function buildSetting(seed: GameControlCatalogSettingSeed): GameControlSetting {
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

export function buildCapabilityTruths(): GameControlCapabilityTruth[] {
  return GameControlCapabilityTruthSeeds.map((truth) => ({
    truthId: GameControlRuleIdSchema.parse(truth.truthId),
    sourceDocument: GameControlCapabilityGuideDocument,
    sourceHeadingPath: truth.sourceHeadingPath,
    originalSourceText: truth.originalSourceText,
    appliesToSettingIds: truth.appliesToSettingIds.map((settingId) => GameControlSettingIdSchema.parse(settingId)),
    capabilityState: truth.capabilityState,
  }));
}

export function buildCapabilityRegistry() {
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

export function optionsForSeed(seed: GameControlCatalogSettingSeed): GameControlOption[] {
  if (seed.controlType === 'boolean') {
    return [
      optionFromSeedValue(seed, ['enabled', 'Enabled', null]),
      optionFromSeedValue(seed, ['disabled', 'Disabled', null]),
    ];
  }
  return seed.options.map((optionSeed) => optionFromSeedValue(seed, optionSeed));
}

export function optionFromSeedValue(seed: GameControlCatalogSettingSeed, optionSeed: GameControlCatalogOptionSeed) {
  return {
    optionId: GameControlOptionIdSchema.parse(`game-control-option-${slug(seed.settingId)}-${slug(optionSeed[0])}`),
    label: optionSeed[1],
    value: optionSeed[0],
    originalSourceText: optionSeed[0],
    meaning: optionSeed[2],
    defaultSelected: optionDefaultSelected(seed.defaultValue, optionSeed[0]),
  };
}

export function optionDefaultSelected(defaultValue: unknown, optionValue: string): boolean {
  if (Array.isArray(defaultValue)) {
    return defaultValue.includes(optionValue);
  }
  if (typeof defaultValue === 'boolean') {
    return defaultValue ? optionValue === 'enabled' : optionValue === 'disabled';
  }
  return defaultValue === optionValue;
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

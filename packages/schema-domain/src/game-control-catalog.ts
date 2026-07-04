import {
  GameControlCapabilityGuideDocument as GameControlCapabilityGuideDocumentValue,
  GameControlEffectModeOptions as GameControlEffectModeOptionsValue,
  GameControlEffectivePolicyDocumentId,
  GameControlCatalogManifestId,
  GameControlPolicyDocumentId,
  GameControlPolicyRevision,
  GameControlSourceDocument as GameControlSourceDocumentValue,
  GameControlTargetScopeOptions as GameControlTargetScopeOptionsValue,
  countBy,
  gameControlSourceOptionCount,
} from './game-control-catalog-core';
import {
  buildCapabilityRegistry,
  buildCapabilityTruths,
  buildLanes,
} from './game-control-catalog-build';
import {
  defaultPolicyValueSetting,
  parseCompleteGameControlPolicyValueDocument as parseCompleteGameControlPolicyValueDocumentImpl,
} from './game-control-catalog-policy';
import {
  GameControlCatalogSourceDocuments,
  GameControlUpdateCommandSeeds,
} from './game-control-catalog-data';
import {
  type GameControlAuthoringManifest,
  type GameControlEffectivePolicyDocument,
  type GameControlGroup,
  type GameControlPolicyValueDocument,
  type GameControlSection,
  type GameControlSetting,
} from './game-control-catalog-schema';
import { GameControlAuthoringManifestSchema, GameControlEffectivePolicyDocumentSchema, GameControlPolicyHashSchema, GameControlPolicyUpdateCommandSchema, GameControlCommandIdSchema } from './game-control-catalog-schema';
import { ParentContractSchemaVersion } from './family-reference-primitives';

export const GameControlSourceDocument = GameControlSourceDocumentValue;
export const GameControlCapabilityGuideDocument = GameControlCapabilityGuideDocumentValue;
export const GameControlTargetScopeOptions = GameControlTargetScopeOptionsValue;
export const GameControlEffectModeOptions = GameControlEffectModeOptionsValue;

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

export const BaselineGameControlPolicyValueDocument = parseCompleteGameControlPolicyValueDocumentImpl({
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
  return parseCompleteGameControlPolicyValueDocumentImpl(input);
}

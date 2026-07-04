import type { GameControlCatalogSetting, GameControlPolicyValueDocument } from './game-control-catalog-schema';
import { GameControlPolicyValueDocumentSchema } from './game-control-catalog-schema';
import { GameControlCatalogSettingSeeds } from './game-control-catalog-data';
import { buildSetting } from './game-control-catalog-build';

export function parseCompleteGameControlPolicyValueDocument(input: unknown): GameControlPolicyValueDocument {
  const document = GameControlPolicyValueDocumentSchema.parse(input);
  const settings = GameControlCatalogSettingSeeds.map(buildSetting);
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

export function defaultPolicyValueSetting(setting: GameControlCatalogSetting) {
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

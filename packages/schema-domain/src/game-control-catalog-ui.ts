import type {
  GameControlCatalogSettingSeed,
  GameControlControlType,
  GameControlOption,
  GameControlUiCardType,
} from './game-control-catalog-schema';

const GameControlControlTypeBySeedType: Record<string, GameControlControlType> = {
  boolean: 'toggle',
  'single-choice': 'single-choice',
  'multi-choice': 'multi-choice',
  number: 'number',
  retention: 'retention',
  'rule-list': 'rule-list',
};

export function controlTypeForSeed(seed: GameControlCatalogSettingSeed): GameControlControlType {
  return GameControlControlTypeBySeedType[seed.controlType] ?? 'read-only-status';
}

export function cardTypeFor(controlType: GameControlControlType, options: readonly GameControlOption[]): GameControlUiCardType {
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

export function layoutHintsFor(controlType: GameControlControlType, options: readonly GameControlOption[]) {
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

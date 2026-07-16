/* generated from crates/network-core/src/network_control_catalog_metadata_layout.ts.txt */

import type {
  NetworkControlCardKind,
  NetworkControlKind,
  NetworkControlLayoutHints,
  NetworkControlOption,
  NetworkControlSelectionMode,
} from './network-control-catalog-schema';

const hasOwn = <T extends object>(value: T, key: PropertyKey): key is keyof T =>
  Object.prototype.hasOwnProperty.call(value, key);

const ExplicitKinds = {
  boolean: 'toggle',
  'single-choice': 'single-choice',
  'multi-choice': 'multi-choice',
} as const satisfies Partial<Record<string, NetworkControlKind>>;

const KindPatterns = [
  [/^Capability matrix row \|/u, 'read-only-status'],
  [/retention|custody|deletion|expiry/u, 'retention'],
  [/budget|bytes|count|minutes|duration|timer|threshold/u, 'number'],
  [/schedule|time window/u, 'schedule'],
  [/allow|block|warn|ask|enforce|terminate|force|route|adapter|actions/u, 'action-list'],
  [/target|domain|ip|port|protocol|process|exception|indicator|source|field|scope|category/u, 'multi-choice'],
] as const satisfies readonly (readonly [pattern: RegExp, value: NetworkControlKind])[];

const FixedCardKinds = {
  toggle: 'toggle',
  schedule: 'schedule-card',
  'rule-list': 'rule-list-card',
  'target-list': 'target-list-card',
  retention: 'retention-card',
  'read-only-status': 'status-card',
  number: 'number-card',
  duration: 'number-card',
} as const satisfies Partial<Record<NetworkControlKind, NetworkControlCardKind>>;

const MultiSelectionKinds = new Set<NetworkControlKind>(['multi-choice', 'action-list', 'target-list']);

export function networkControlKindFor(
  sourceText: string,
  explicitKind: string | null,
  explicitOptionLabels: (value: string) => readonly string[]
): NetworkControlKind {
  const mappedKind =
    explicitKind !== null && hasOwn(ExplicitKinds, explicitKind) ? ExplicitKinds[explicitKind] : undefined;
  if (mappedKind !== undefined) {
    return mappedKind;
  }
  const searchable = sourceText.toLowerCase();
  for (const [pattern, kind] of KindPatterns) {
    if (pattern.test(searchable)) {
      return kind;
    }
  }
  return explicitOptionLabels(sourceText).length > 0 ? 'single-choice' : 'toggle';
}

export function networkSelectionModeFor(
  controlKind: NetworkControlKind,
  optionsForSetting: readonly NetworkControlOption[]
): NetworkControlSelectionMode {
  return MultiSelectionKinds.has(controlKind) || (optionsForSetting.length > 4 && controlKind !== 'read-only-status')
    ? 'multi'
    : 'single';
}

export function networkCardKindFor(
  controlKind: NetworkControlKind,
  selectionMode: NetworkControlSelectionMode,
  optionsForSetting: readonly NetworkControlOption[]
): NetworkControlCardKind {
  const fixedCardKind = hasOwn(FixedCardKinds, controlKind) ? FixedCardKinds[controlKind] : undefined;
  if (fixedCardKind !== undefined) {
    return fixedCardKind;
  }
  return selectionMode === 'multi'
    ? optionsForSetting.length > 4
      ? 'multi-choice-many'
      : 'multi-choice-normal'
    : optionsForSetting.length > 4
      ? 'single-choice-many'
      : 'single-choice-compact';
}

export function networkLayoutHintsFor(
  controlKind: NetworkControlKind,
  selectionMode: NetworkControlSelectionMode,
  optionsForSetting: readonly NetworkControlOption[]
): NetworkControlLayoutHints {
  const manyOptions = optionsForSetting.length > 4;
  const listLike = selectionMode === 'multi' || controlKind === 'action-list' || controlKind === 'target-list';
  return {
    preferredColumnSpan: manyOptions || controlKind === 'retention' || controlKind === 'read-only-status' ? 2 : 1,
    collapsible: manyOptions || listLike || controlKind === 'read-only-status',
    searchableOptions: manyOptions,
    optionGroupCount: manyOptions ? Math.ceil(optionsForSetting.length / 4) : 1,
    showAsMatrixWhenLarge: manyOptions && listLike,
    showSelectedCount: listLike,
  };
}

export function networkHelperTextFor(
  sectionTitle: string,
  groupTitle: string,
  sourceText: string,
  proofRequirementFor: (section: string, group: string, source: string) => string | null,
  capabilityRequirementFor: (section: string, group: string, source: string) => string
): string {
  return (
    proofRequirementFor(sectionTitle, groupTitle, sourceText) ??
    capabilityRequirementFor(sectionTitle, groupTitle, sourceText)
  );
}

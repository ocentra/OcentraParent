import {
  GameControlCatalogSettingSeeds,
} from './game-control-catalog-data';
import {
  GameControlCatalogIdSchema,
  GameControlPolicyDocumentIdSchema,
  GameControlPolicyRevisionSchema,
  GameControlSourceDocumentSchema,
  type GameControlEffectMode,
  type GameControlLane,
  type GameControlPolicyLane,
  type GameControlSection,
  type GameControlGroup,
  type GameControlSetting,
  type GameControlTargetScope,
} from './game-control-catalog-schema';
import { ParentContractSchemaVersion } from './family-reference-primitives';

export type GameControlGroupDraft = Omit<GameControlGroup, 'settings'> & {
  readonly settings: GameControlSetting[];
};

export type GameControlSectionDraft = Omit<GameControlSection, 'groups'> & {
  readonly groups: Map<string, GameControlGroupDraft>;
};

export type GameControlLaneDraft = Omit<GameControlLane, 'sections'> & {
  readonly sections: Map<string, GameControlSectionDraft>;
};

export const GameControlCatalogManifestId = GameControlCatalogIdSchema.parse('game-control-authoring-v1');
export const GameControlPolicyDocumentId = GameControlPolicyDocumentIdSchema.parse('game-control-policy-default-v1');
export const GameControlEffectivePolicyDocumentId = GameControlPolicyDocumentIdSchema.parse(
  'game-control-effective-default-v1'
);
export const GameControlPolicyRevision = GameControlPolicyRevisionSchema.parse('game-control-policy-revision-1');

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

export const GameControlLaneOrder = [
  'rules',
  'schedule',
  'approvals',
  'enforcement',
  'audit',
  'evidence',
  'reports',
  'data',
] as const satisfies readonly GameControlPolicyLane[];

export const GameControlLaneTitles = {
  rules: 'Rules',
  schedule: 'Schedule',
  approvals: 'Approvals',
  enforcement: 'Enforcement',
  audit: 'Audit',
  evidence: 'Evidence',
  reports: 'Reports',
  data: 'Data',
} as const satisfies Record<GameControlPolicyLane, string>;

export function gameControlSourceOptionCount(): number {
  return GameControlCatalogSettingSeeds.reduce((count, seed) => count + seed.options.length, 0);
}

export function countBy<T>(items: readonly T[], keyFor: (item: T) => string) {
  const counts = new Map<string, number>();
  for (const item of items) {
    const key = keyFor(item);
    counts.set(key, (counts.get(key) ?? 0) + 1);
  }
  return Object.fromEntries([...counts.entries()].sort(([left], [right]) => left.localeCompare(right)));
}

export function byDisplayOrder<T extends { readonly displayOrder: number }>(left: T, right: T): number {
  return left.displayOrder - right.displayOrder;
}

export function slug(value: string): string {
  const normalized = value
    .toLowerCase()
    .replace(/&/gu, ' and ')
    .replace(/[^a-z0-9]+/gu, '-')
    .replace(/^-+|-+$/gu, '')
    .replace(/-{2,}/gu, '-');
  return normalized.length > 0 ? normalized : 'value';
}

export const GameControlPolicyDocumentSchemaVersion = ParentContractSchemaVersion.V0_6;

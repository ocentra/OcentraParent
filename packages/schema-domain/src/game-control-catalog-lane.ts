import type { GameControlCatalogSettingSeed, GameControlPolicyLane } from './game-control-catalog-schema';

const GameControlPolicyLaneBySectionId: Partial<Record<string, GameControlPolicyLane>> = {
  budgets: 'schedule',
  approvals: 'approvals',
  audit: 'audit',
  'session-evidence': 'evidence',
  reports: 'reports',
};

const GameControlEnforcementSectionIds = new Set(['native-games', 'launcher-games', 'browser-cloud-games']);

export function laneForSeed(seed: GameControlCatalogSettingSeed): GameControlPolicyLane {
  return GameControlPolicyLaneBySectionId[seed.sectionId] ?? (GameControlEnforcementSectionIds.has(seed.sectionId) ? 'enforcement' : 'rules');
}

import type { GameControlCatalogSettingSeed, GameControlRuntimeOwner } from './game-control-catalog-schema';

const GameControlRuntimeOwnerRules: ReadonlyArray<[RegExp, GameControlRuntimeOwner]> = [
  [/reports\.|retention\.|custody\.|audit\.|neverCollect/u, 'parent-owned-storage'],
  [/approvals\./u, 'agent-protocol'],
  [/nativeGames|launchers|browserCloud/u, 'os-adapter'],
  [/rules\.|budgets\.|evidence\./u, 'child-agent'],
];

const GameControlCapabilityRequirementRules: ReadonlyArray<[RegExp, string]> = [
  [/browserCloud/u, 'managed-browser-boundary-or-cloud-client-surface-proof'],
  [/launchers/u, 'launcher-manifest-or-child-process-attribution-proof'],
  [/nativeGames/u, 'local-process-package-window-proof-plus-platform-adapter-capability'],
  [/inventory|evidence/u, 'child-device-local-evidence-read-model-with-source-confidence'],
  [/approvals/u, 'validated-parent-approval-protocol-with-offline-fallback'],
  [/reports|retention|custody|audit/u, 'child-local-or-parent-owned-storage-with-custody-labels'],
];

export function runtimeOwnerForSeed(seed: GameControlCatalogSettingSeed): GameControlRuntimeOwner {
  for (const [pattern, owner] of GameControlRuntimeOwnerRules) {
    if (pattern.test(seed.settingId)) {
      return owner;
    }
  }
  return 'rust-parent-runtime';
}

export function capabilityRequirementForSeed(seed: GameControlCatalogSettingSeed): string {
  for (const [pattern, requirement] of GameControlCapabilityRequirementRules) {
    if (pattern.test(seed.settingId)) {
      return requirement;
    }
  }
  return 'game-control-authoring-manifest';
}

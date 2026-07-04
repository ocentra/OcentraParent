import type {
  GameControlCapabilityState,
  GameControlCatalogSettingSeed,
  GameControlEffectStatus,
} from './game-control-catalog-schema';

const GameControlCapabilityStateByEffectStatus: Record<GameControlEffectStatus, GameControlCapabilityState> = {
  'manual-required': 'manual-required',
  degraded: 'degraded',
  'proof-required': 'protected',
  'future-gap': 'future-gap',
  'permission-required': 'permission-required',
  'needs-wiring': 'available',
  'already-represented': 'available',
};

const GameControlEffectStatusRules: ReadonlyArray<[RegExp, GameControlEffectStatus]> = [
  [/neverCollect|reports\.|retention\.|custody\.|audit\./u, 'already-represented'],
  [/browserCloud|requiredProof|durationCountingMode|strictActions|allowedTargetTypes|allowedActions/u, 'proof-required'],
  [/managementMode|launchers\.supportedKinds|whenManifestUnavailable/u, 'manual-required'],
  [/classificationStates|whenProofUnavailable|launcherOnlyHandling/u, 'degraded'],
];

export function effectStatusForSeed(seed: GameControlCatalogSettingSeed): GameControlEffectStatus {
  for (const [pattern, status] of GameControlEffectStatusRules) {
    if (pattern.test(seed.settingId)) {
      return status;
    }
  }
  return 'needs-wiring';
}

export function capabilityStateForSeed(seed: GameControlCatalogSettingSeed): GameControlCapabilityState {
  return GameControlCapabilityStateByEffectStatus[effectStatusForSeed(seed)];
}

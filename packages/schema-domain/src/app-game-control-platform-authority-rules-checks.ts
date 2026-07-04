import { platformHardControlProofIsPresent } from './app-game-control-platform-authority-rules-platform-checks';
import type { AppGamePlatformAuthorityRowRuleInput } from './app-game-control-platform-authority-rules-types';

const hardControlActions: readonly AppGamePlatformAuthorityRowRuleInput['action'][] = [
  'time-limit',
  'terminate-process',
  'hide-app',
  'suspend-app',
  'shield-app',
  'block-launch',
  'enforce-allowlist',
];

const noExecutionAuthorityTiers = new Set<AppGamePlatformAuthorityRowRuleInput['authorityTier']>([
  'manual-required',
  'not-claimed',
]);

const observeOnlyHardControlActions = new Set<AppGamePlatformAuthorityRowRuleInput['action']>(hardControlActions);

const noExecutableCapabilityStates = new Set<AppGamePlatformAuthorityRowRuleInput['capabilityState']>([
  'manual-required',
  'observe-only',
]);

const noExecutionCapabilityStates = new Set<AppGamePlatformAuthorityRowRuleInput['capabilityState']>([
  'unavailable',
  'degraded',
]);

export function platformAuthorityRowIsConsistent(row: AppGamePlatformAuthorityRowRuleInput): boolean {
  return (
    nonExecutableStatesCannotExecute(row) &&
    proofedClaimsHaveArtifacts(row) &&
    platformHardControlProofIsPresent(row) &&
    parentVisibleLimitationIsSpecific(row)
  );
}

function nonExecutableStatesCannotExecute(row: AppGamePlatformAuthorityRowRuleInput): boolean {
  if (authorityTierRequiresNoExecution(row.authorityTier)) {
    return row.capabilityState === row.authorityTier && rowHasNoExecutableModes(row);
  }

  if (row.authorityTier === 'observe-only' && observeOnlyHardControlActions.has(row.action)) {
    return row.capabilityState === 'observe-only' && rowHasNoExecutableModes(row);
  }

  if (capabilityStateForbidsModes(row.capabilityState)) {
    return rowHasNoExecutableModes(row);
  }

  if (capabilityStateForbidsExecution(row.capabilityState)) {
    return !row.canExecuteAdapter;
  }

  return true;
}

function authorityTierRequiresNoExecution(
  authorityTier: AppGamePlatformAuthorityRowRuleInput['authorityTier']
): boolean {
  return noExecutionAuthorityTiers.has(authorityTier);
}

function capabilityStateForbidsModes(
  rowCapabilityState: AppGamePlatformAuthorityRowRuleInput['capabilityState']
): boolean {
  return noExecutableCapabilityStates.has(rowCapabilityState);
}

function capabilityStateForbidsExecution(
  rowCapabilityState: AppGamePlatformAuthorityRowRuleInput['capabilityState']
): boolean {
  return noExecutionCapabilityStates.has(rowCapabilityState);
}

function rowHasNoExecutableModes(row: AppGamePlatformAuthorityRowRuleInput): boolean {
  return row.supportedModes.length === 0 && !row.canExecuteAdapter;
}

function proofedClaimsHaveArtifacts(row: AppGamePlatformAuthorityRowRuleInput): boolean {
  if (!row.canExecuteAdapter && row.capabilityState !== 'supported') {
    return true;
  }

  return (
    row.proofReferences.length > 0 && (!hardControlActions.includes(row.action) || hasProof(row, 'rollback-proof'))
  );
}

function parentVisibleLimitationIsSpecific(row: AppGamePlatformAuthorityRowRuleInput): boolean {
  const normalized = String(row.parentVisibleLimitation).trim().toLowerCase();
  return normalized !== 'unsupported' && normalized !== 'not supported';
}

function hasProof(row: AppGamePlatformAuthorityRowRuleInput, proofKind: AppGamePlatformAuthorityRowRuleInput['proofReferences'][number]['proofKind']): boolean {
  return row.proofReferences.some((proofReference) => proofReference.proofKind === proofKind);
}

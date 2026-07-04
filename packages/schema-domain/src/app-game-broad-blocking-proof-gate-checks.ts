import { platformProofRequirementsAreNamed } from './app-game-broad-blocking-proof-gate-platform-checks';
import type { AppGameBroadBlockingGateRuleInput, AppGameBroadBlockingProofKindRuleInput } from './app-game-broad-blocking-proof-gate-types';

const supportedProofKinds: readonly AppGameBroadBlockingProofKindRuleInput[] = [
  'setup-proof',
  'authority-tier-proof',
  'rollback-proof',
  'audit-state-proof',
];

export function appGameBroadBlockingGateIsHonest(gate: AppGameBroadBlockingGateRuleInput): boolean {
  return (
    gateHasSpecificUiReason(gate) &&
    nonSupportedGateCannotDispatch(gate) &&
    supportedGateCarriesCompleteProof(gate) &&
    platformProofRequirementsAreNamed(gate)
  );
}

function gateHasSpecificUiReason(gate: AppGameBroadBlockingGateRuleInput): boolean {
  const normalized = String(gate.parentVisibleReason).trim().toLowerCase();
  return normalized !== 'unsupported' && normalized !== 'not supported' && normalized !== 'manual-required';
}

function nonSupportedGateCannotDispatch(gate: AppGameBroadBlockingGateRuleInput): boolean {
  return gate.outcomeState === 'supported' || (!gate.canCallAdapter && gate.supportedModes.length === 0 && gate.adapterDispatchState !== 'dispatch-eligible');
}

function supportedGateCarriesCompleteProof(gate: AppGameBroadBlockingGateRuleInput): boolean {
  return (
    gate.outcomeState !== 'supported' ||
    (gate.canCallAdapter &&
      gate.adapterDispatchState === 'dispatch-eligible' &&
      gate.rollbackState === 'rollback-proof-attached' &&
      gate.auditState === 'audit-proof-attached' &&
      gate.broadBlockingClaimed &&
      supportedProofKinds.every((proofKind) => gateHasProof(gate, proofKind)))
  );
}

function gateHasProof(
  gate: AppGameBroadBlockingGateRuleInput,
  proofKind: AppGameBroadBlockingProofKindRuleInput
): boolean {
  return gate.proofReferences.some((proofReference) => proofReference.proofKind === proofKind);
}

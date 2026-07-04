import type {
  AppGameBroadBlockingActionRuleInput,
  AppGameBroadBlockingGateRuleInput,
  AppGameBroadBlockingProofKindRuleInput,
} from './app-game-broad-blocking-proof-gate-types';

const desktopGateActions = new Set<AppGameBroadBlockingActionRuleInput>(['block-launch', 'enforce-allowlist']);

export function platformProofRequirementsAreNamed(gate: AppGameBroadBlockingGateRuleInput): boolean {
  if (desktopGateActions.has(gate.action)) {
    return desktopBroadBlockProofRequirementsAreNamed(gate);
  }

  if (gate.platform === 'android' && (gate.action === 'hide-app' || gate.action === 'suspend-app')) {
    return gateRequiresAny(gate, ['android-device-owner-proof', 'android-profile-owner-proof']);
  }

  if (gate.platform === 'ios' && gate.action === 'shield-app') {
    return gateRequiresAll(gate, ['ios-family-controls-proof', 'ios-managed-settings-proof']);
  }

  return true;
}

function desktopBroadBlockProofRequirementsAreNamed(gate: AppGameBroadBlockingGateRuleInput): boolean {
  if (!gateRequiresAll(gate, ['setup-proof', 'authority-tier-proof', 'rollback-proof', 'audit-state-proof'])) {
    return false;
  }

  const platformRequirementCheckers: Record<
    AppGameBroadBlockingGateRuleInput['platform'],
    (gate: AppGameBroadBlockingGateRuleInput) => boolean
  > = {
    windows: (innerGate) => gateRequiresAny(innerGate, ['windows-applocker-proof', 'windows-app-control-proof']),
    macos: (innerGate) =>
      gateRequiresAny(innerGate, [
        'macos-mdm-profile-proof',
        'macos-endpoint-security-proof',
        'macos-system-extension-proof',
      ]),
    linux: (innerGate) => gateRequiresAll(innerGate, ['linux-mechanism-proof', 'linux-distro-proof', 'linux-session-proof']),
    android: (innerGate) => gateRequiresAny(innerGate, ['android-device-owner-proof', 'android-profile-owner-proof']),
    ios: (innerGate) =>
      gateRequiresAny(innerGate, [
        'ios-family-controls-proof',
        'ios-managed-settings-proof',
        'ios-supervised-mdm-proof',
      ]),
  };

  return platformRequirementCheckers[gate.platform](gate);
}

function gateRequiresAll(
  gate: AppGameBroadBlockingGateRuleInput,
  proofKinds: readonly AppGameBroadBlockingProofKindRuleInput[]
): boolean {
  return proofKinds.every((proofKind) => gateHasNamedRequirementOrProof(gate, proofKind));
}

function gateRequiresAny(
  gate: AppGameBroadBlockingGateRuleInput,
  proofKinds: readonly AppGameBroadBlockingProofKindRuleInput[]
): boolean {
  return proofKinds.some((proofKind) => gateHasNamedRequirementOrProof(gate, proofKind));
}

function gateHasNamedRequirementOrProof(
  gate: AppGameBroadBlockingGateRuleInput,
  proofKind: AppGameBroadBlockingProofKindRuleInput
): boolean {
  return gate.outcomeState === 'supported' ? gateHasProof(gate, proofKind) : gate.requiredProofKinds.includes(proofKind) || gateHasProof(gate, proofKind);
}

function gateHasProof(
  gate: AppGameBroadBlockingGateRuleInput,
  proofKind: AppGameBroadBlockingProofKindRuleInput
): boolean {
  return gate.proofReferences.some((proofReference) => proofReference.proofKind === proofKind);
}

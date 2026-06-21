type AppGameBroadBlockingPlatformRuleInput = 'windows' | 'linux' | 'macos' | 'android' | 'ios';

type AppGameBroadBlockingActionRuleInput =
  | 'inventory'
  | 'runtime'
  | 'foreground'
  | 'warn'
  | 'ask-parent'
  | 'time-limit'
  | 'terminate-process'
  | 'hide-app'
  | 'suspend-app'
  | 'shield-app'
  | 'block-launch'
  | 'enforce-allowlist';

type AppGameBroadBlockingProofKindRuleInput =
  | 'setup-proof'
  | 'authority-tier-proof'
  | 'rollback-proof'
  | 'audit-state-proof'
  | 'windows-applocker-proof'
  | 'windows-applocker-audit-proof'
  | 'windows-app-control-proof'
  | 'windows-system-app-allowlist-proof'
  | 'macos-mdm-profile-proof'
  | 'macos-endpoint-security-proof'
  | 'macos-system-extension-proof'
  | 'linux-mechanism-proof'
  | 'linux-distro-proof'
  | 'linux-session-proof'
  | 'android-device-owner-proof'
  | 'android-profile-owner-proof'
  | 'ios-family-controls-proof'
  | 'ios-managed-settings-proof'
  | 'ios-supervised-mdm-proof';

interface AppGameBroadBlockingGateRuleInput {
  readonly platform: AppGameBroadBlockingPlatformRuleInput;
  readonly action: AppGameBroadBlockingActionRuleInput;
  readonly outcomeState: 'manual-required' | 'unavailable' | 'not-claimed' | 'supported';
  readonly adapterDispatchState:
    | 'blocked-before-adapter'
    | 'adapter-unavailable'
    | 'not-dispatched'
    | 'dispatch-eligible';
  readonly supportedModes: readonly unknown[];
  readonly canCallAdapter: boolean;
  readonly rollbackState: 'rollback-required' | 'rollback-proof-attached' | 'not-applicable';
  readonly auditState: 'audit-required' | 'audit-proof-attached' | 'not-applicable';
  readonly parentVisibleReason: unknown;
  readonly requiredProofKinds: readonly AppGameBroadBlockingProofKindRuleInput[];
  readonly proofReferences: readonly { readonly proofKind: AppGameBroadBlockingProofKindRuleInput }[];
  readonly broadBlockingClaimed: boolean;
}

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
  if (gate.outcomeState === 'supported') {
    return true;
  }

  return !gate.canCallAdapter && gate.supportedModes.length === 0 && gate.adapterDispatchState !== 'dispatch-eligible';
}

function supportedGateCarriesCompleteProof(gate: AppGameBroadBlockingGateRuleInput): boolean {
  if (gate.outcomeState !== 'supported') {
    return !gate.broadBlockingClaimed;
  }

  return (
    gate.canCallAdapter &&
    gate.adapterDispatchState === 'dispatch-eligible' &&
    gate.rollbackState === 'rollback-proof-attached' &&
    gate.auditState === 'audit-proof-attached' &&
    gate.broadBlockingClaimed &&
    gateHasProof(gate, 'setup-proof') &&
    gateHasProof(gate, 'authority-tier-proof') &&
    gateHasProof(gate, 'rollback-proof') &&
    gateHasProof(gate, 'audit-state-proof')
  );
}

function platformProofRequirementsAreNamed(gate: AppGameBroadBlockingGateRuleInput): boolean {
  if (gate.action === 'block-launch' || gate.action === 'enforce-allowlist') {
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

  switch (gate.platform) {
    case 'windows':
      return gateRequiresAny(gate, ['windows-applocker-proof', 'windows-app-control-proof']);
    case 'macos':
      return gateRequiresAny(gate, [
        'macos-mdm-profile-proof',
        'macos-endpoint-security-proof',
        'macos-system-extension-proof',
      ]);
    case 'linux':
      return gateRequiresAll(gate, ['linux-mechanism-proof', 'linux-distro-proof', 'linux-session-proof']);
    case 'android':
      return gateRequiresAny(gate, ['android-device-owner-proof', 'android-profile-owner-proof']);
    case 'ios':
      return gateRequiresAny(gate, [
        'ios-family-controls-proof',
        'ios-managed-settings-proof',
        'ios-supervised-mdm-proof',
      ]);
  }
}

function gateRequiresAll(
  gate: AppGameBroadBlockingGateRuleInput,
  proofKinds: readonly AppGameBroadBlockingProofKindRuleInput[]
) {
  return proofKinds.every((proofKind) => gateHasNamedRequirementOrProof(gate, proofKind));
}

function gateRequiresAny(
  gate: AppGameBroadBlockingGateRuleInput,
  proofKinds: readonly AppGameBroadBlockingProofKindRuleInput[]
) {
  return proofKinds.some((proofKind) => gateHasNamedRequirementOrProof(gate, proofKind));
}

function gateHasNamedRequirementOrProof(
  gate: AppGameBroadBlockingGateRuleInput,
  proofKind: AppGameBroadBlockingProofKindRuleInput
): boolean {
  if (gate.outcomeState === 'supported') {
    return gateHasProof(gate, proofKind);
  }

  return gate.requiredProofKinds.includes(proofKind) || gateHasProof(gate, proofKind);
}

function gateHasProof(
  gate: AppGameBroadBlockingGateRuleInput,
  proofKind: AppGameBroadBlockingProofKindRuleInput
): boolean {
  return gate.proofReferences.some((proofReference) => proofReference.proofKind === proofKind);
}

type AppGamePlatformActionRuleInput =
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

type AppGamePlatformAuthorityTierRuleInput =
  | 'observe-only'
  | 'user-approved-helper'
  | 'accessibility-assisted'
  | 'managed-profile'
  | 'device-owner'
  | 'mdm-enrolled'
  | 'supervised-device'
  | 'system-extension'
  | 'root-or-admin-service'
  | 'kiosk-or-single-app'
  | 'manual-required'
  | 'not-claimed';

type AppGamePlatformProofKindRuleInput =
  | 'contract-proof'
  | 'manual-host-proof'
  | 'rollback-proof'
  | 'windows-applocker-proof'
  | 'windows-app-control-proof'
  | 'device-owner-proof'
  | 'profile-owner-proof'
  | 'family-controls-authorization'
  | 'managed-settings-shield-proof'
  | 'mdm-profile-proof'
  | 'endpoint-security-proof'
  | 'system-extension-proof'
  | 'linux-mechanism-proof'
  | 'linux-distro-proof'
  | 'linux-session-proof'
  | 'accessibility-permission-proof'
  | 'usage-stats-proof';

interface AppGamePlatformAuthorityRowRuleInput {
  readonly platform: 'windows' | 'linux' | 'macos' | 'android' | 'ios';
  readonly action: AppGamePlatformActionRuleInput;
  readonly authorityTier: AppGamePlatformAuthorityTierRuleInput;
  readonly capabilityState: 'supported' | 'unavailable' | 'degraded' | 'dry-run' | 'observe-only' | 'manual-required';
  readonly canExecuteAdapter: boolean;
  readonly supportedModes: readonly unknown[];
  readonly proofReferences: readonly { readonly proofKind: AppGamePlatformProofKindRuleInput }[];
  readonly parentVisibleLimitation: unknown;
  readonly linuxMechanism: unknown | null;
  readonly linuxDistro: unknown | null;
  readonly linuxSession: unknown | null;
}

const hardControlActions: readonly AppGamePlatformActionRuleInput[] = [
  'time-limit',
  'terminate-process',
  'hide-app',
  'suspend-app',
  'shield-app',
  'block-launch',
  'enforce-allowlist',
];

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

  if (row.authorityTier === 'observe-only' && hardControlActions.includes(row.action)) {
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

function authorityTierRequiresNoExecution(authorityTier: AppGamePlatformAuthorityTierRuleInput): boolean {
  return authorityTier === 'manual-required' || authorityTier === 'not-claimed';
}

function capabilityStateForbidsModes(
  rowCapabilityState: AppGamePlatformAuthorityRowRuleInput['capabilityState']
): boolean {
  return rowCapabilityState === 'manual-required' || rowCapabilityState === 'observe-only';
}

function capabilityStateForbidsExecution(
  rowCapabilityState: AppGamePlatformAuthorityRowRuleInput['capabilityState']
): boolean {
  return rowCapabilityState === 'unavailable' || rowCapabilityState === 'degraded';
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

function platformHardControlProofIsPresent(row: AppGamePlatformAuthorityRowRuleInput): boolean {
  if (!row.canExecuteAdapter && row.capabilityState !== 'supported') {
    return true;
  }

  switch (row.platform) {
    case 'android':
      return androidProofIsPresent(row);
    case 'ios':
      return iosShieldProofIsPresent(row);
    case 'macos':
      return macosHardBlockProofIsPresent(row);
    case 'linux':
      return linuxHardBlockProofIsPresent(row);
    case 'windows':
      return windowsBroadBlockProofIsPresent(row);
  }
}

function androidProofIsPresent(row: AppGamePlatformAuthorityRowRuleInput): boolean {
  if (row.action !== 'hide-app' && row.action !== 'suspend-app') {
    return true;
  }

  const hasOwnerTier = row.authorityTier === 'device-owner' || row.authorityTier === 'managed-profile';
  return hasOwnerTier && (hasProof(row, 'device-owner-proof') || hasProof(row, 'profile-owner-proof'));
}

function iosShieldProofIsPresent(row: AppGamePlatformAuthorityRowRuleInput): boolean {
  if (row.action === 'terminate-process' || row.action === 'block-launch' || row.action === 'enforce-allowlist') {
    return false;
  }

  if (row.action !== 'shield-app') {
    return true;
  }

  return hasProof(row, 'family-controls-authorization') && hasProof(row, 'managed-settings-shield-proof');
}

function macosHardBlockProofIsPresent(row: AppGamePlatformAuthorityRowRuleInput): boolean {
  if (row.action !== 'block-launch' && row.action !== 'enforce-allowlist') {
    return true;
  }

  const hasAuthorityTier = row.authorityTier === 'mdm-enrolled' || row.authorityTier === 'system-extension';
  const hasHardBlockProof =
    hasProof(row, 'mdm-profile-proof') ||
    hasProof(row, 'endpoint-security-proof') ||
    hasProof(row, 'system-extension-proof');
  return hasAuthorityTier && hasHardBlockProof;
}

function linuxHardBlockProofIsPresent(row: AppGamePlatformAuthorityRowRuleInput): boolean {
  if (row.action !== 'block-launch' && row.action !== 'enforce-allowlist') {
    return true;
  }

  return (
    row.authorityTier === 'root-or-admin-service' &&
    row.linuxMechanism !== null &&
    row.linuxDistro !== null &&
    row.linuxSession !== null &&
    hasProof(row, 'linux-mechanism-proof') &&
    hasProof(row, 'linux-distro-proof') &&
    hasProof(row, 'linux-session-proof')
  );
}

function windowsBroadBlockProofIsPresent(row: AppGamePlatformAuthorityRowRuleInput): boolean {
  if (row.action !== 'block-launch' && row.action !== 'enforce-allowlist') {
    return true;
  }

  return hasProof(row, 'windows-applocker-proof') || hasProof(row, 'windows-app-control-proof');
}

function parentVisibleLimitationIsSpecific(row: AppGamePlatformAuthorityRowRuleInput): boolean {
  const normalized = String(row.parentVisibleLimitation).trim().toLowerCase();
  return normalized !== 'unsupported' && normalized !== 'not supported';
}

function hasProof(row: AppGamePlatformAuthorityRowRuleInput, proofKind: AppGamePlatformProofKindRuleInput): boolean {
  return row.proofReferences.some((proofReference) => proofReference.proofKind === proofKind);
}

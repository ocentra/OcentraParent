import type { AppGamePlatformAuthorityRowRuleInput } from './app-game-control-platform-authority-rules-types';

const hardBlockActions = new Set<AppGamePlatformAuthorityRowRuleInput['action']>([
  'block-launch',
  'enforce-allowlist',
]);

export function platformHardControlProofIsPresent(row: AppGamePlatformAuthorityRowRuleInput): boolean {
  if (!row.canExecuteAdapter && row.capabilityState !== 'supported') {
    return true;
  }

  return platformCheckers[row.platform](row);
}

const platformCheckers: Record<
  AppGamePlatformAuthorityRowRuleInput['platform'],
  (row: AppGamePlatformAuthorityRowRuleInput) => boolean
> = {
  android: androidProofIsPresent,
  ios: iosShieldProofIsPresent,
  macos: macosHardBlockProofIsPresent,
  linux: linuxHardBlockProofIsPresent,
  windows: windowsBroadBlockProofIsPresent,
};

function androidProofIsPresent(row: AppGamePlatformAuthorityRowRuleInput): boolean {
  if (!platformActionNeedsHardControlProof(row.action)) {
    return true;
  }

  return (
    (row.authorityTier === 'device-owner' || row.authorityTier === 'managed-profile') &&
    (hasProof(row, 'device-owner-proof') || hasProof(row, 'profile-owner-proof'))
  );
}

function iosShieldProofIsPresent(row: AppGamePlatformAuthorityRowRuleInput): boolean {
  if (row.action === 'terminate-process' || platformActionNeedsHardControlProof(row.action)) {
    return false;
  }

  if (row.action !== 'shield-app') {
    return true;
  }

  return hasProof(row, 'family-controls-authorization') && hasProof(row, 'managed-settings-shield-proof');
}

function macosHardBlockProofIsPresent(row: AppGamePlatformAuthorityRowRuleInput): boolean {
  if (!platformActionNeedsHardControlProof(row.action)) {
    return true;
  }

  return (
    (row.authorityTier === 'mdm-enrolled' || row.authorityTier === 'system-extension') &&
    (hasProof(row, 'mdm-profile-proof') ||
      hasProof(row, 'endpoint-security-proof') ||
      hasProof(row, 'system-extension-proof'))
  );
}

function linuxHardBlockProofIsPresent(row: AppGamePlatformAuthorityRowRuleInput): boolean {
  if (!platformActionNeedsHardControlProof(row.action)) {
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
  if (!platformActionNeedsHardControlProof(row.action)) {
    return true;
  }

  return hasProof(row, 'windows-applocker-proof') || hasProof(row, 'windows-app-control-proof');
}

function platformActionNeedsHardControlProof(
  action: AppGamePlatformAuthorityRowRuleInput['action']
): boolean {
  return hardBlockActions.has(action);
}

function hasProof(row: AppGamePlatformAuthorityRowRuleInput, proofKind: AppGamePlatformAuthorityRowRuleInput['proofReferences'][number]['proofKind']): boolean {
  return row.proofReferences.some((proofReference) => proofReference.proofKind === proofKind);
}

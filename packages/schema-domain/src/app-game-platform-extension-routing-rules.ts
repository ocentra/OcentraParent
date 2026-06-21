type PlatformExtensionPlatformRuleInput = 'windows' | 'linux' | 'macos' | 'android' | 'ios';

type PlatformExtensionAuthorityTierRuleInput =
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

type PlatformExtensionActionScopeRuleInput =
  | 'inventory'
  | 'runtime'
  | 'foreground'
  | 'identity'
  | 'policy-handoff'
  | 'warn-ask'
  | 'time-budget'
  | 'terminate-process'
  | 'hide-app'
  | 'suspend-app'
  | 'shield-app'
  | 'block-launch'
  | 'allowlist'
  | 'uninstall-block'
  | 'managed-configuration'
  | 'single-app-mode'
  | 'store-signing'
  | 'cross-plan-handoff';

type PlatformExtensionCapabilityStateRuleInput =
  | 'supported'
  | 'unavailable'
  | 'degraded'
  | 'dry-run'
  | 'observe-only'
  | 'manual-required';

type PlatformExtensionProofFileRuleInput =
  | '00-source-snapshot.md'
  | '01-contract-proof.log'
  | '02-rust-protocol-proof.log'
  | '03-runtime-evidence.json'
  | '04-journal-sqlite-proof.json'
  | '05-policy-action-proof.json'
  | '06-ui-snapshots/ui-not-applicable.md'
  | '07-playwright-ui-proof.log'
  | '08-security-negative-proof.log'
  | '09-manual-platform-proof.md'
  | '10-validation-commands.log'
  | '11-authority-tier-proof.md'
  | '12-permission-setup-proof.md'
  | '13-rollback-proof.md';

interface PlatformExtensionProofReferenceRuleInput {
  readonly proofFile: PlatformExtensionProofFileRuleInput;
  readonly artifactRef: unknown;
}

interface PlatformExtensionRoutingRowRuleInput {
  readonly rowId: string;
  readonly platform: PlatformExtensionPlatformRuleInput;
  readonly actionScope: PlatformExtensionActionScopeRuleInput;
  readonly authorityTier: PlatformExtensionAuthorityTierRuleInput;
  readonly capabilityState: PlatformExtensionCapabilityStateRuleInput;
  readonly parentVisibleLabel: unknown;
  readonly appPlanProofPackRef: unknown;
  readonly appGameProofPackRef: unknown;
  readonly crossPlanHandoff: unknown;
  readonly manualTags: readonly string[];
  readonly requiredProofFiles: readonly PlatformExtensionProofFileRuleInput[];
  readonly proofReferences: readonly PlatformExtensionProofReferenceRuleInput[];
  readonly canPromote: boolean;
  readonly promotionState: 'extension-checklist' | 'manual-required' | 'not-claimed' | 'promotion-ready';
}

const rowIdPrefixByPlatform = {
  android: 'ANDROID-',
  ios: 'IOS-',
  linux: 'LINUX-',
  macos: 'MAC-',
  windows: 'WIN-',
} as const satisfies Record<PlatformExtensionPlatformRuleInput, string>;

const manualTagByPlatform = {
  android: '@requires-android',
  ios: '@requires-ios',
  linux: '@requires-linux',
  macos: '@requires-macos',
  windows: '@requires-windows',
} as const satisfies Record<PlatformExtensionPlatformRuleInput, string>;

const strongActionScopes: readonly PlatformExtensionActionScopeRuleInput[] = [
  'time-budget',
  'terminate-process',
  'hide-app',
  'suspend-app',
  'shield-app',
  'block-launch',
  'allowlist',
  'uninstall-block',
  'managed-configuration',
  'single-app-mode',
];

const strongAuthorityTiers: readonly PlatformExtensionAuthorityTierRuleInput[] = [
  'managed-profile',
  'device-owner',
  'mdm-enrolled',
  'supervised-device',
  'system-extension',
  'root-or-admin-service',
  'kiosk-or-single-app',
];

const requiredStrongProofFiles: readonly PlatformExtensionProofFileRuleInput[] = [
  '11-authority-tier-proof.md',
  '12-permission-setup-proof.md',
  '13-rollback-proof.md',
];

export function platformExtensionRoutingRowIsHonest(row: PlatformExtensionRoutingRowRuleInput): boolean {
  return (
    rowIdMatchesPlatform(row) &&
    parentVisibleLabelIsSpecific(row.parentVisibleLabel) &&
    crossPlanHandoffIsLinked(row) &&
    manualRowsHaveManualTags(row) &&
    strongerClaimsNameCoreProof(row) &&
    missingProofCannotPromote(row) &&
    promotionRowsCarryAttachedProof(row)
  );
}

function rowIdMatchesPlatform(row: PlatformExtensionRoutingRowRuleInput): boolean {
  return row.rowId.startsWith(rowIdPrefixByPlatform[row.platform]);
}

function parentVisibleLabelIsSpecific(parentVisibleLabel: unknown): boolean {
  const normalized = String(parentVisibleLabel).trim().toLowerCase();
  return normalized !== 'unsupported' && normalized !== 'not supported';
}

function crossPlanHandoffIsLinked(row: PlatformExtensionRoutingRowRuleInput): boolean {
  const appPlanRef = String(row.appPlanProofPackRef);
  const appGameRef = String(row.appGameProofPackRef);
  return (
    String(row.crossPlanHandoff).trim().length > 0 && appPlanRef.includes(row.rowId) && appGameRef.includes(row.rowId)
  );
}

function manualRowsHaveManualTags(row: PlatformExtensionRoutingRowRuleInput): boolean {
  if (row.promotionState === 'promotion-ready') {
    return true;
  }

  return row.manualTags.includes('@manual') && row.manualTags.includes(manualTagByPlatform[row.platform]);
}

function strongerClaimsNameCoreProof(row: PlatformExtensionRoutingRowRuleInput): boolean {
  if (!rowIsStrongerThanObserve(row)) {
    return true;
  }

  return requiredStrongProofFiles.every((proofFile) => row.requiredProofFiles.includes(proofFile));
}

function rowIsStrongerThanObserve(row: PlatformExtensionRoutingRowRuleInput): boolean {
  return strongActionScopes.includes(row.actionScope) || strongAuthorityTiers.includes(row.authorityTier);
}

function missingProofCannotPromote(row: PlatformExtensionRoutingRowRuleInput): boolean {
  if (row.proofReferences.length > 0) {
    return true;
  }

  return !row.canPromote && row.promotionState !== 'promotion-ready' && row.capabilityState !== 'supported';
}

function promotionRowsCarryAttachedProof(row: PlatformExtensionRoutingRowRuleInput): boolean {
  if (!row.canPromote && row.promotionState !== 'promotion-ready') {
    return true;
  }

  const attachedProofFiles = new Set(row.proofReferences.map((proofReference) => proofReference.proofFile));
  return (
    row.requiredProofFiles.length > 0 && row.requiredProofFiles.every((proofFile) => attachedProofFiles.has(proofFile))
  );
}

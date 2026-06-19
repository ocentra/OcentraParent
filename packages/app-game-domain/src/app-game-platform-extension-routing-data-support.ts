import {
  type AppGamePlatformExtensionActionScope,
  type AppGamePlatformExtensionManualTag,
  type AppGamePlatformExtensionProductScope,
  type AppGamePlatformExtensionProofFile,
  type AppGamePlatformExtensionProofKind,
  type AppGamePlatformExtensionPromotionState,
  type AppGamePlatformExtensionRoutingRow,
  AppGamePlatformExtensionRoutingRowSchema,
} from './app-game-platform-extension-routing';
import {
  type AppGamePlatformAuthorityTier,
  type AppGamePlatformSetupState,
} from './app-game-control-platform-authority';
import type { EnforcementCapabilityStateSchema } from '@ocentra-parent/enforcement-domain/enforcement';
import {
  type ParentPlatform,
  ParentContractSchemaVersion,
  ParentPlatform as ParentPlatformValue,
} from '@ocentra-parent/schema-domain/family-reference-primitives';

type ExtensionRouteInput = {
  rowId: string;
  platform: ParentPlatform;
  title: string;
  actionScope: AppGamePlatformExtensionActionScope;
  authorityTier: AppGamePlatformAuthorityTier;
  setupState: AppGamePlatformSetupState;
  proofGate: string;
  parentVisibleLabel: string;
  appGameDifference: string;
  capabilityState?: typeof EnforcementCapabilityStateSchema.Type;
  productScope?: AppGamePlatformExtensionProductScope;
  promotionState?: AppGamePlatformExtensionPromotionState;
  manualTags?: readonly AppGamePlatformExtensionManualTag[];
  requiredProofFiles?: readonly AppGamePlatformExtensionProofFile[];
  requiredProofKinds?: readonly AppGamePlatformExtensionProofKind[];
  setupProofRequired?: boolean;
  actionProofRequired?: boolean;
  rollbackProofRequired?: boolean;
  storeSigningEntitlementRequired?: boolean;
  crossPlanHandoff?: string;
};

export const PlatformExtensionGeneratedAt = '2026-06-03T11:35:00.000Z';
const AppPlanWorkpack = 'docs/plans/app-plan/workpacks/24-platform-extension-checklist-and-proof-routing.md';
const AppGameWorkpack = 'docs/plans/app-game-plan/workpacks/25-platform-extension-checklist-and-proof-routing.md';

const baseProofFiles = [
  '00-source-snapshot.md',
  '01-contract-proof.log',
  '09-manual-platform-proof.md',
  '10-validation-commands.log',
] as const satisfies readonly AppGamePlatformExtensionProofFile[];

const hardControlProofFiles = [
  ...baseProofFiles,
  '05-policy-action-proof.json',
  '08-security-negative-proof.log',
  '11-authority-tier-proof.md',
  '12-permission-setup-proof.md',
  '13-rollback-proof.md',
] as const satisfies readonly AppGamePlatformExtensionProofFile[];

const platformManualTag = {
  android: '@requires-android',
  ios: '@requires-ios',
  linux: '@requires-linux',
  macos: '@requires-macos',
  windows: '@requires-windows',
} as const satisfies Record<ParentPlatform, AppGamePlatformExtensionManualTag>;

const hardActionScopes: readonly AppGamePlatformExtensionActionScope[] = [
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

const strongAuthorityTiers: readonly AppGamePlatformAuthorityTier[] = [
  'managed-profile',
  'device-owner',
  'mdm-enrolled',
  'supervised-device',
  'system-extension',
  'root-or-admin-service',
  'kiosk-or-single-app',
];

export function linuxRoute(
  rowId: string,
  title: string,
  actionScope: AppGamePlatformExtensionActionScope,
  authorityTier: AppGamePlatformAuthorityTier,
  setupState: AppGamePlatformSetupState,
  proofGate: string,
  extraTags: readonly AppGamePlatformExtensionManualTag[] = []
): AppGamePlatformExtensionRoutingRow {
  return route({
    rowId,
    platform: ParentPlatformValue.Linux,
    title,
    actionScope,
    authorityTier,
    setupState,
    proofGate,
    parentVisibleLabel: `${title} stays manual-required or observe-only until Linux distro/session proof is attached.`,
    appGameDifference: 'Native game Linux proof must name distro, desktop/session, mechanism, and rollback scope.',
    manualTags: extraTags,
  });
}

export function route(input: ExtensionRouteInput): AppGamePlatformExtensionRoutingRow {
  const requiredProofFiles = input.requiredProofFiles ?? requiredFilesFor(input.actionScope, input.authorityTier);
  const productScope = input.productScope ?? 'shared-app-game';

  return AppGamePlatformExtensionRoutingRowSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    rowId: input.rowId,
    platform: input.platform,
    title: input.title,
    sourceChecklistRef: `docs/plans/app-plan/v0-5-native-apps-platform-deep-dive.md#${input.rowId}`,
    appPlanWorkpackRef: AppPlanWorkpack,
    appGamePlanWorkpackRef: AppGameWorkpack,
    appPlanProofPackRef: `output/app-plan-proof/platform-extensions/${input.platform}/${input.rowId}`,
    appGameProofPackRef: `output/app-game-plan-proof/platform-extensions/${input.platform}/${input.rowId}`,
    productScope,
    actionScope: input.actionScope,
    authorityTier: input.authorityTier,
    setupState: input.setupState,
    capabilityState: input.capabilityState ?? capabilityFor(input.authorityTier),
    promotionState: input.promotionState ?? 'manual-required',
    parentVisibleLabel: input.parentVisibleLabel,
    proofGate: input.proofGate,
    manualTags: tagsFor(input.platform, input.manualTags ?? []),
    requiredProofFiles,
    requiredProofKinds: input.requiredProofKinds ?? requiredKindsFor(requiredProofFiles),
    proofReferences: [],
    setupProofRequired: input.setupProofRequired ?? input.setupState !== 'not-required',
    actionProofRequired: input.actionProofRequired ?? hardActionScopes.includes(input.actionScope),
    rollbackProofRequired: input.rollbackProofRequired ?? hardActionScopes.includes(input.actionScope),
    storeSigningEntitlementRequired: input.storeSigningEntitlementRequired ?? false,
    canPromote: false,
    crossPlanHandoff:
      input.crossPlanHandoff ??
      'Promotion must update app-plan WP24 and app-game WP25 proof packs before support status moves.',
    appGameDifference: input.appGameDifference,
    lastCheckedAt: PlatformExtensionGeneratedAt,
  });
}

function requiredFilesFor(
  actionScope: AppGamePlatformExtensionActionScope,
  authorityTier: AppGamePlatformAuthorityTier
): readonly AppGamePlatformExtensionProofFile[] {
  if (hardActionScopes.includes(actionScope) || strongAuthorityTiers.includes(authorityTier)) {
    return hardControlProofFiles;
  }

  return baseProofFiles;
}

function requiredKindsFor(
  proofFiles: readonly AppGamePlatformExtensionProofFile[]
): readonly AppGamePlatformExtensionProofKind[] {
  const proofKinds = new Set<AppGamePlatformExtensionProofKind>(['source-snapshot', 'contract-proof']);

  if (proofFiles.includes('05-policy-action-proof.json')) {
    proofKinds.add('policy-action-proof');
  }

  if (proofFiles.includes('08-security-negative-proof.log')) {
    proofKinds.add('security-negative-proof');
  }

  if (proofFiles.includes('09-manual-platform-proof.md')) {
    proofKinds.add('manual-platform-proof');
  }

  if (proofFiles.includes('11-authority-tier-proof.md')) {
    proofKinds.add('authority-tier-proof');
  }

  if (proofFiles.includes('12-permission-setup-proof.md')) {
    proofKinds.add('permission-setup-proof');
  }

  if (proofFiles.includes('13-rollback-proof.md')) {
    proofKinds.add('rollback-proof');
  }

  return Array.from(proofKinds);
}

function capabilityFor(authorityTier: AppGamePlatformAuthorityTier): typeof EnforcementCapabilityStateSchema.Type {
  if (authorityTier === 'observe-only') {
    return 'observe-only';
  }

  if (authorityTier === 'not-claimed') {
    return 'manual-required';
  }

  return 'manual-required';
}

function tagsFor(
  platform: ParentPlatform,
  extraTags: readonly AppGamePlatformExtensionManualTag[]
): readonly AppGamePlatformExtensionManualTag[] {
  return Array.from(new Set<AppGamePlatformExtensionManualTag>(['@manual', platformManualTag[platform], ...extraTags]));
}

import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import {
  AppGamePlatformAuthorityTierSchema,
  AppGamePlatformSetupStateSchema,
} from './app-game-control-platform-authority';
import { platformExtensionRoutingRowIsHonest } from '@ocentra-parent/schema-domain/app-game-platform-extension-routing-rules';
import { EnforcementCapabilityStateSchema } from '@ocentra-parent/schema-domain/enforcement';
import { ParentContractSchemaVersionSchema, ParentPlatformSchema, ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';

export const AppGamePlatformExtensionRoutingRowIdSchema = brandedNonEmptyStringSchema('AppGamePlatformExtensionRoutingRowId');
export const AppGamePlatformExtensionRoutingMatrixIdSchema = brandedNonEmptyStringSchema('AppGamePlatformExtensionRoutingMatrixId');
export const AppGamePlatformExtensionProofPackRefSchema = brandedNonEmptyStringSchema('AppGamePlatformExtensionProofPackRef');
export const AppGamePlatformExtensionSourceRefSchema = brandedNonEmptyStringSchema('AppGamePlatformExtensionSourceRef');
export const AppGamePlatformExtensionLabelSchema = brandedNonEmptyStringSchema('AppGamePlatformExtensionLabel');

export const AppGamePlatformExtensionProductScopeSchema = withParser(
  Schema.Literal('native-app', 'native-game', 'shared-app-game', 'platform-handoff')
);

export const AppGamePlatformExtensionActionScopeSchema = withParser(
  Schema.Literal(
    'inventory',
    'runtime',
    'foreground',
    'identity',
    'policy-handoff',
    'warn-ask',
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
    'store-signing',
    'cross-plan-handoff'
  )
);

export const AppGamePlatformExtensionPromotionStateSchema = withParser(
  Schema.Literal('extension-checklist', 'manual-required', 'not-claimed', 'promotion-ready')
);

export const AppGamePlatformExtensionProofFileSchema = withParser(
  Schema.Literal(
    '00-source-snapshot.md',
    '01-contract-proof.log',
    '02-rust-protocol-proof.log',
    '03-runtime-evidence.json',
    '04-journal-sqlite-proof.json',
    '05-policy-action-proof.json',
    '06-ui-snapshots/ui-not-applicable.md',
    '07-playwright-ui-proof.log',
    '08-security-negative-proof.log',
    '09-manual-platform-proof.md',
    '10-validation-commands.log',
    '11-authority-tier-proof.md',
    '12-permission-setup-proof.md',
    '13-rollback-proof.md'
  )
);

export const AppGamePlatformExtensionManualTagSchema = withParser(
  Schema.Literal(
    '@manual',
    '@requires-windows',
    '@requires-macos',
    '@requires-ios',
    '@requires-android',
    '@requires-linux',
    '@requires-mdm',
    '@requires-admin-root',
    '@requires-system-extension',
    '@requires-store-signing',
    '@requires-kiosk',
    '@requires-accessibility',
    '@requires-usage-stats',
    '@requires-device-owner',
    '@requires-profile-owner',
    '@requires-familycontrols',
    '@requires-managedsettings',
    '@requires-wayland',
    '@requires-x11'
  )
);

export const AppGamePlatformExtensionProofKindSchema = withParser(
  Schema.Literal(
    'source-snapshot',
    'contract-proof',
    'runtime-evidence-proof',
    'journal-sqlite-proof',
    'policy-action-proof',
    'manual-platform-proof',
    'authority-tier-proof',
    'permission-setup-proof',
    'rollback-proof',
    'ui-proof',
    'security-negative-proof',
    'store-signing-entitlement-proof',
    'cross-plan-handoff-proof'
  )
);

const AppGamePlatformExtensionProofReferenceSchema = Schema.Struct({
  proofKind: AppGamePlatformExtensionProofKindSchema,
  proofFile: AppGamePlatformExtensionProofFileSchema,
  artifactRef: brandedNonEmptyStringSchema('AppGamePlatformExtensionProofArtifactRef'),
});

const AppGamePlatformExtensionRoutingRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  rowId: AppGamePlatformExtensionRoutingRowIdSchema,
  platform: ParentPlatformSchema,
  title: AppGamePlatformExtensionLabelSchema,
  sourceChecklistRef: AppGamePlatformExtensionSourceRefSchema,
  appPlanWorkpackRef: AppGamePlatformExtensionSourceRefSchema,
  appGamePlanWorkpackRef: AppGamePlatformExtensionSourceRefSchema,
  appPlanProofPackRef: AppGamePlatformExtensionProofPackRefSchema,
  appGameProofPackRef: AppGamePlatformExtensionProofPackRefSchema,
  productScope: AppGamePlatformExtensionProductScopeSchema,
  actionScope: AppGamePlatformExtensionActionScopeSchema,
  authorityTier: AppGamePlatformAuthorityTierSchema,
  setupState: AppGamePlatformSetupStateSchema,
  capabilityState: EnforcementCapabilityStateSchema,
  promotionState: AppGamePlatformExtensionPromotionStateSchema,
  parentVisibleLabel: AppGamePlatformExtensionLabelSchema,
  proofGate: AppGamePlatformExtensionLabelSchema,
  manualTags: Schema.Array(AppGamePlatformExtensionManualTagSchema),
  requiredProofFiles: Schema.Array(AppGamePlatformExtensionProofFileSchema),
  requiredProofKinds: Schema.Array(AppGamePlatformExtensionProofKindSchema),
  proofReferences: Schema.Array(AppGamePlatformExtensionProofReferenceSchema),
  setupProofRequired: Schema.Boolean,
  actionProofRequired: Schema.Boolean,
  rollbackProofRequired: Schema.Boolean,
  storeSigningEntitlementRequired: Schema.Boolean,
  canPromote: Schema.Boolean,
  crossPlanHandoff: AppGamePlatformExtensionLabelSchema,
  appGameDifference: AppGamePlatformExtensionLabelSchema,
  lastCheckedAt: ParentTimestampSchema,
});

type AppGamePlatformExtensionRoutingRowCandidate = Infer<typeof AppGamePlatformExtensionRoutingRowBaseSchema>;

export const AppGamePlatformExtensionRoutingRowSchema = withParser(
  AppGamePlatformExtensionRoutingRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        platformExtensionRoutingRowIsHonest(row) ||
        'Expected platform extension routing rows to name authority, proof packs, manual tags, and no-promotion boundaries before support moves up'
    )
  )
);

export const AppGamePlatformExtensionRoutingMatrixSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    matrixId: AppGamePlatformExtensionRoutingMatrixIdSchema,
    generatedAt: ParentTimestampSchema,
    rows: Schema.Array(AppGamePlatformExtensionRoutingRowSchema),
  }).pipe(
    Schema.filter(
      (matrix) =>
        matrix.rows.length > 0 && matrixRowsAreUnique(matrix.rows) && matrixHasRequiredExtensionCoverage(matrix.rows)
    )
  )
);

function matrixRowsAreUnique(rows: readonly AppGamePlatformExtensionRoutingRowCandidate[]): boolean {
  const seen = new Set<string>();

  for (const row of rows) {
    if (seen.has(row.rowId)) {
      return false;
    }

    seen.add(row.rowId);
  }

  return true;
}

function matrixHasRequiredExtensionCoverage(rows: readonly AppGamePlatformExtensionRoutingRowCandidate[]): boolean {
  return (
    countRowsWithPrefix(rows, 'MAC-') === 12 &&
    countRowsWithPrefix(rows, 'IOS-') === 12 &&
    countRowsWithPrefix(rows, 'ANDROID-') === 14 &&
    countRowsWithPrefix(rows, 'LINUX-') === 14
  );
}

function countRowsWithPrefix(rows: readonly AppGamePlatformExtensionRoutingRowCandidate[], prefix: string): number {
  return rows.filter((row) => row.rowId.startsWith(prefix)).length;
}

export type AppGamePlatformExtensionProductScope = Infer<typeof AppGamePlatformExtensionProductScopeSchema>;
export type AppGamePlatformExtensionActionScope = Infer<typeof AppGamePlatformExtensionActionScopeSchema>;
export type AppGamePlatformExtensionPromotionState = Infer<typeof AppGamePlatformExtensionPromotionStateSchema>;
export type AppGamePlatformExtensionProofFile = Infer<typeof AppGamePlatformExtensionProofFileSchema>;
export type AppGamePlatformExtensionManualTag = Infer<typeof AppGamePlatformExtensionManualTagSchema>;
export type AppGamePlatformExtensionProofKind = Infer<typeof AppGamePlatformExtensionProofKindSchema>;
export type AppGamePlatformExtensionRoutingRow = Infer<typeof AppGamePlatformExtensionRoutingRowSchema>;
export type AppGamePlatformExtensionRoutingMatrix = Infer<typeof AppGamePlatformExtensionRoutingMatrixSchema>;

export const decodeAppGamePlatformExtensionRoutingRow = (input: unknown) =>
  AppGamePlatformExtensionRoutingRowSchema.parse(input);
export const decodeAppGamePlatformExtensionRoutingMatrix = (input: unknown) =>
  AppGamePlatformExtensionRoutingMatrixSchema.parse(input);


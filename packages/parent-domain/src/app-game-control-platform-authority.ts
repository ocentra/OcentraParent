import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { EnforcementCapabilityStateSchema, EnforcementModeSchema } from './enforcement';
import { ParentContractSchemaVersionSchema, ParentPlatformSchema, ParentTimestampSchema } from './reference-primitives';
import { platformAuthorityRowIsConsistent } from './app-game-control-platform-authority-rules';

const NonEmptyPlatformAuthorityText = Schema.String.pipe(Schema.minLength(1));

export const AppGamePlatformAuthorityTierSchema = withParser(
  Schema.Literal(
    'observe-only',
    'user-approved-helper',
    'accessibility-assisted',
    'managed-profile',
    'device-owner',
    'mdm-enrolled',
    'supervised-device',
    'system-extension',
    'root-or-admin-service',
    'kiosk-or-single-app',
    'manual-required',
    'not-claimed'
  )
);

export const AppGamePlatformActionSchema = withParser(
  Schema.Literal(
    'inventory',
    'runtime',
    'foreground',
    'warn',
    'ask-parent',
    'time-limit',
    'terminate-process',
    'hide-app',
    'suspend-app',
    'shield-app',
    'block-launch',
    'enforce-allowlist'
  )
);

export const AppGamePlatformSetupStateSchema = withParser(
  Schema.Literal(
    'not-required',
    'permission-required',
    'helper-required',
    'managed-profile-required',
    'device-owner-required',
    'mdm-required',
    'supervision-required',
    'system-extension-required',
    'admin-or-root-required',
    'kiosk-required',
    'manual-required',
    'not-claimed'
  )
);

export const AppGamePlatformProofStateSchema = withParser(
  Schema.Literal(
    'missing-proof',
    'fixture-proof',
    'manual-proof-attached',
    'runtime-proof-attached',
    'rollback-proof-attached',
    'manual-required',
    'not-claimed'
  )
);

export const AppGamePlatformParentVisibleStateSchema = withParser(
  Schema.Literal(
    'supported',
    'dry-run',
    'observe-only',
    'permission-required',
    'managed-device-required',
    'admin-or-root-required',
    'system-extension-required',
    'supervised-device-required',
    'manual-required',
    'not-claimed',
    'unavailable'
  )
);

export const AppGamePlatformProofKindSchema = withParser(
  Schema.Literal(
    'contract-proof',
    'manual-host-proof',
    'rollback-proof',
    'windows-applocker-proof',
    'windows-app-control-proof',
    'device-owner-proof',
    'profile-owner-proof',
    'family-controls-authorization',
    'managed-settings-shield-proof',
    'mdm-profile-proof',
    'endpoint-security-proof',
    'system-extension-proof',
    'linux-mechanism-proof',
    'linux-distro-proof',
    'linux-session-proof',
    'accessibility-permission-proof',
    'usage-stats-proof'
  )
);

const AppGamePlatformProofReferenceSchema = Schema.Struct({
  proofKind: AppGamePlatformProofKindSchema,
  artifactRef: NonEmptyPlatformAuthorityText.pipe(Schema.brand('AppGamePlatformAuthorityProofArtifactRef')),
});

const AppGamePlatformAuthorityRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  rowId: NonEmptyPlatformAuthorityText.pipe(Schema.brand('AppGamePlatformAuthorityRowId')),
  platform: ParentPlatformSchema,
  action: AppGamePlatformActionSchema,
  authorityTier: AppGamePlatformAuthorityTierSchema,
  setupState: AppGamePlatformSetupStateSchema,
  proofState: AppGamePlatformProofStateSchema,
  capabilityState: EnforcementCapabilityStateSchema,
  parentVisibleState: AppGamePlatformParentVisibleStateSchema,
  parentVisibleLimitation: NonEmptyPlatformAuthorityText.pipe(Schema.brand('AppGamePlatformParentVisibleLimitation')),
  canExecuteAdapter: Schema.Boolean,
  supportedModes: Schema.Array(EnforcementModeSchema),
  proofReferences: Schema.Array(AppGamePlatformProofReferenceSchema),
  proofNeededToClaim: Schema.Array(AppGamePlatformProofKindSchema),
  linuxMechanism: Schema.Union(
    NonEmptyPlatformAuthorityText.pipe(Schema.brand('LinuxAppGameControlMechanism')),
    Schema.Null
  ),
  linuxDistro: Schema.Union(NonEmptyPlatformAuthorityText.pipe(Schema.brand('LinuxAppGameControlDistro')), Schema.Null),
  linuxSession: Schema.Union(
    NonEmptyPlatformAuthorityText.pipe(Schema.brand('LinuxAppGameControlSession')),
    Schema.Null
  ),
  lastCheckedAt: ParentTimestampSchema,
});

type AppGamePlatformAuthorityRowCandidate = Infer<typeof AppGamePlatformAuthorityRowBaseSchema>;

export const AppGamePlatformAuthorityRowSchema = withParser(
  AppGamePlatformAuthorityRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        platformAuthorityRowIsConsistent(row) ||
        'Expected app/game platform authority row to keep hard-control claims proof-gated and parent-visible limits specific'
    )
  )
);

export const AppGamePlatformAuthorityMatrixSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    matrixId: NonEmptyPlatformAuthorityText.pipe(Schema.brand('AppGamePlatformAuthorityMatrixId')),
    rows: Schema.Array(AppGamePlatformAuthorityRowSchema),
    generatedAt: ParentTimestampSchema,
  }).pipe(
    Schema.filter(
      (matrix) =>
        (matrix.rows.length > 0 && matrixRowsAreUnique(matrix.rows)) ||
        'Expected app/game platform authority matrix rows to be non-empty and unique by platform/action'
    )
  )
);

function matrixRowsAreUnique(rows: readonly AppGamePlatformAuthorityRowCandidate[]): boolean {
  const seen = new Set<string>();

  for (const row of rows) {
    const key = `${row.platform}/${row.action}`;

    if (seen.has(key)) {
      return false;
    }

    seen.add(key);
  }

  return true;
}

export type AppGamePlatformAuthorityTier = Infer<typeof AppGamePlatformAuthorityTierSchema>;
export type AppGamePlatformAction = Infer<typeof AppGamePlatformActionSchema>;
export type AppGamePlatformSetupState = Infer<typeof AppGamePlatformSetupStateSchema>;
export type AppGamePlatformProofState = Infer<typeof AppGamePlatformProofStateSchema>;
export type AppGamePlatformParentVisibleState = Infer<typeof AppGamePlatformParentVisibleStateSchema>;
export type AppGamePlatformProofKind = Infer<typeof AppGamePlatformProofKindSchema>;
export type AppGamePlatformAuthorityRow = Infer<typeof AppGamePlatformAuthorityRowSchema>;
export type AppGamePlatformAuthorityMatrix = Infer<typeof AppGamePlatformAuthorityMatrixSchema>;

export const AppGamePlatformAuthorityTier = {
  ObserveOnly: AppGamePlatformAuthorityTierSchema.parse('observe-only'),
  UserApprovedHelper: AppGamePlatformAuthorityTierSchema.parse('user-approved-helper'),
  AccessibilityAssisted: AppGamePlatformAuthorityTierSchema.parse('accessibility-assisted'),
  ManagedProfile: AppGamePlatformAuthorityTierSchema.parse('managed-profile'),
  DeviceOwner: AppGamePlatformAuthorityTierSchema.parse('device-owner'),
  MdmEnrolled: AppGamePlatformAuthorityTierSchema.parse('mdm-enrolled'),
  SupervisedDevice: AppGamePlatformAuthorityTierSchema.parse('supervised-device'),
  SystemExtension: AppGamePlatformAuthorityTierSchema.parse('system-extension'),
  RootOrAdminService: AppGamePlatformAuthorityTierSchema.parse('root-or-admin-service'),
  KioskOrSingleApp: AppGamePlatformAuthorityTierSchema.parse('kiosk-or-single-app'),
  ManualRequired: AppGamePlatformAuthorityTierSchema.parse('manual-required'),
  NotClaimed: AppGamePlatformAuthorityTierSchema.parse('not-claimed'),
} as const;

export const AppGamePlatformAction = {
  Inventory: AppGamePlatformActionSchema.parse('inventory'),
  Runtime: AppGamePlatformActionSchema.parse('runtime'),
  Foreground: AppGamePlatformActionSchema.parse('foreground'),
  Warn: AppGamePlatformActionSchema.parse('warn'),
  AskParent: AppGamePlatformActionSchema.parse('ask-parent'),
  TimeLimit: AppGamePlatformActionSchema.parse('time-limit'),
  TerminateProcess: AppGamePlatformActionSchema.parse('terminate-process'),
  HideApp: AppGamePlatformActionSchema.parse('hide-app'),
  SuspendApp: AppGamePlatformActionSchema.parse('suspend-app'),
  ShieldApp: AppGamePlatformActionSchema.parse('shield-app'),
  BlockLaunch: AppGamePlatformActionSchema.parse('block-launch'),
  EnforceAllowlist: AppGamePlatformActionSchema.parse('enforce-allowlist'),
} as const;

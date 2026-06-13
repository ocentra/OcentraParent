import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { appGameBroadBlockingGateIsHonest } from './app-game-broad-blocking-proof-gate-rules';
import {
  AppGamePlatformActionSchema,
  AppGamePlatformAuthorityTierSchema,
  AppGamePlatformSetupStateSchema,
} from './app-game-control-platform-authority';
import { EnforcementCapabilityStateSchema, EnforcementModeSchema } from '@ocentra-parent/enforcement-domain/enforcement';
import { ParentContractSchemaVersionSchema, ParentPlatformSchema, ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';

export const AppGameBroadBlockingGateIdSchema = brandedNonEmptyStringSchema('AppGameBroadBlockingGateId');
export const AppGameBroadBlockingGateMatrixIdSchema = brandedNonEmptyStringSchema('AppGameBroadBlockingGateMatrixId');
export const AppGameBroadBlockingGateReasonSchema = brandedNonEmptyStringSchema('AppGameBroadBlockingGateReason');
export const AppGameBroadBlockingGateProofArtifactRefSchema = brandedNonEmptyStringSchema('AppGameBroadBlockingGateProofArtifactRef');

export const AppGameBroadBlockingGateOutcomeSchema = withParser(
  Schema.Literal('manual-required', 'unavailable', 'not-claimed', 'supported')
);

export const AppGameBroadBlockingAdapterDispatchStateSchema = withParser(
  Schema.Literal('blocked-before-adapter', 'adapter-unavailable', 'not-dispatched', 'dispatch-eligible')
);

export const AppGameBroadBlockingRollbackStateSchema = withParser(
  Schema.Literal('rollback-required', 'rollback-proof-attached', 'not-applicable')
);

export const AppGameBroadBlockingAuditStateSchema = withParser(
  Schema.Literal('audit-required', 'audit-proof-attached', 'not-applicable')
);

export const AppGameBroadBlockingProofKindSchema = withParser(
  Schema.Literal(
    'setup-proof',
    'authority-tier-proof',
    'rollback-proof',
    'audit-state-proof',
    'windows-applocker-proof',
    'windows-applocker-audit-proof',
    'windows-app-control-proof',
    'windows-system-app-allowlist-proof',
    'macos-mdm-profile-proof',
    'macos-endpoint-security-proof',
    'macos-system-extension-proof',
    'linux-mechanism-proof',
    'linux-distro-proof',
    'linux-session-proof',
    'android-device-owner-proof',
    'android-profile-owner-proof',
    'ios-family-controls-proof',
    'ios-managed-settings-proof',
    'ios-supervised-mdm-proof'
  )
);

const AppGameBroadBlockingGateProofReferenceSchema = Schema.Struct({
  proofKind: AppGameBroadBlockingProofKindSchema,
  artifactRef: AppGameBroadBlockingGateProofArtifactRefSchema,
});

const AppGameBroadBlockingGateBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  gateId: AppGameBroadBlockingGateIdSchema,
  platform: ParentPlatformSchema,
  action: AppGamePlatformActionSchema,
  outcomeState: AppGameBroadBlockingGateOutcomeSchema,
  adapterDispatchState: AppGameBroadBlockingAdapterDispatchStateSchema,
  authorityTier: AppGamePlatformAuthorityTierSchema,
  setupState: AppGamePlatformSetupStateSchema,
  capabilityState: EnforcementCapabilityStateSchema,
  supportedModes: Schema.Array(EnforcementModeSchema),
  canCallAdapter: Schema.Boolean,
  rollbackState: AppGameBroadBlockingRollbackStateSchema,
  auditState: AppGameBroadBlockingAuditStateSchema,
  parentVisibleReason: AppGameBroadBlockingGateReasonSchema,
  requiredProofKinds: Schema.Array(AppGameBroadBlockingProofKindSchema),
  proofReferences: Schema.Array(AppGameBroadBlockingGateProofReferenceSchema),
  sourceGateRefs: Schema.Array(AppGameBroadBlockingGateProofArtifactRefSchema),
  broadBlockingClaimed: Schema.Boolean,
  lastCheckedAt: ParentTimestampSchema,
});

export const AppGameBroadBlockingGateSchema = withParser(
  AppGameBroadBlockingGateBaseSchema.pipe(
    Schema.filter(
      (gate) =>
        appGameBroadBlockingGateIsHonest(gate) ||
        'Expected app/game broad blocking gates to keep unproved platform blocking manual-required, unavailable, or not-claimed before adapter dispatch'
    )
  )
);

export const AppGameBroadBlockingGateMatrixSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    matrixId: AppGameBroadBlockingGateMatrixIdSchema,
    generatedAt: ParentTimestampSchema,
    gates: Schema.Array(AppGameBroadBlockingGateSchema),
  }).pipe(
    Schema.filter(
      (matrix) =>
        matrix.gates.length > 0 && new Set(matrix.gates.map((gate) => gate.gateId)).size === matrix.gates.length
    )
  )
);

export type AppGameBroadBlockingGateOutcome = Infer<typeof AppGameBroadBlockingGateOutcomeSchema>;
export type AppGameBroadBlockingAdapterDispatchState = Infer<typeof AppGameBroadBlockingAdapterDispatchStateSchema>;
export type AppGameBroadBlockingRollbackState = Infer<typeof AppGameBroadBlockingRollbackStateSchema>;
export type AppGameBroadBlockingAuditState = Infer<typeof AppGameBroadBlockingAuditStateSchema>;
export type AppGameBroadBlockingProofKind = Infer<typeof AppGameBroadBlockingProofKindSchema>;
export type AppGameBroadBlockingGate = Infer<typeof AppGameBroadBlockingGateSchema>;
export type AppGameBroadBlockingGateMatrix = Infer<typeof AppGameBroadBlockingGateMatrixSchema>;

export const decodeAppGameBroadBlockingGate = Schema.decodeUnknownSync(AppGameBroadBlockingGateSchema);
export const decodeAppGameBroadBlockingGateMatrix = Schema.decodeUnknownSync(AppGameBroadBlockingGateMatrixSchema);


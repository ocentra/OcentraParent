import { type Infer, Schema, brandedNonEmptyStringSchema, withParser } from './effect';
import {
  ParentContractSchemaVersionSchema,
  ParentPlatformSchema,
  ParentTimestampSchema,
} from './family-reference-primitives';

export const AppGameAdapterExecutionReadinessReadModelIdSchema = brandedNonEmptyStringSchema(
  'AppGameAdapterExecutionReadinessReadModelId'
);
export const AppGameAdapterExecutionReadinessRowIdSchema = brandedNonEmptyStringSchema(
  'AppGameAdapterExecutionReadinessRowId'
);
export const AppGameAdapterExecutionReadinessReferenceSchema = brandedNonEmptyStringSchema(
  'AppGameAdapterExecutionReadinessReference'
);
export const AppGameAdapterExecutionReadinessBoundarySchema = brandedNonEmptyStringSchema(
  'AppGameAdapterExecutionReadinessBoundary'
);

export const AppGameAdapterProductMeaningSchema = withParser(Schema.Literal('native-app', 'native-game'));

export const AgentAppGameAdapterProductMeaning = {
  NativeApp: 'native-app',
  NativeGame: 'native-game',
} as const;

export const AppGameAdapterExecutionStateSchema = withParser(
  Schema.Literal('proved-scoped-execution', 'manual-required', 'unavailable', 'unsupported', 'degraded', 'not-claimed')
);

export const AgentAppGameAdapterExecutionState = {
  ProvedScopedExecution: 'proved-scoped-execution',
  ManualRequired: 'manual-required',
  Unavailable: 'unavailable',
  Unsupported: 'unsupported',
  Degraded: 'degraded',
  NotClaimed: 'not-claimed',
} as const;

export const AppGameAdapterExecutionDecisionSchema = withParser(
  Schema.Literal('execution-allowed', 'blocked-before-execution')
);

export const AgentAppGameAdapterExecutionDecision = {
  ExecutionAllowed: 'execution-allowed',
  BlockedBeforeExecution: 'blocked-before-execution',
} as const;

export const AppGameAdapterHostCapabilityStateSchema = withParser(
  Schema.Literal('available', 'not-detected', 'not-applicable')
);

export const AgentAppGameAdapterHostCapabilityState = {
  Available: 'available',
  NotDetected: 'not-detected',
  NotApplicable: 'not-applicable',
} as const;

const AppGameAdapterExecutionReadinessRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  rowId: AppGameAdapterExecutionReadinessRowIdSchema,
  sourceProofEntryId: AppGameAdapterExecutionReadinessReferenceSchema,
  platform: ParentPlatformSchema,
  productMeanings: Schema.Array(AppGameAdapterProductMeaningSchema),
  adapterCapability: AppGameAdapterExecutionReadinessReferenceSchema,
  adapterExecutionState: AppGameAdapterExecutionStateSchema,
  executionDecision: AppGameAdapterExecutionDecisionSchema,
  runtimeBoundary: AppGameAdapterExecutionReadinessReferenceSchema,
  targetIdentityState: AppGameAdapterExecutionReadinessReferenceSchema,
  rollbackReferenceState: AppGameAdapterExecutionReadinessReferenceSchema,
  auditReferenceState: AppGameAdapterExecutionReadinessReferenceSchema,
  evidenceRefs: Schema.Array(AppGameAdapterExecutionReadinessReferenceSchema),
  hostCapabilityState: AppGameAdapterHostCapabilityStateSchema,
  hostCapabilityEvidenceRefs: Schema.Array(AppGameAdapterExecutionReadinessReferenceSchema),
  hostCapabilityProbeRefs: Schema.Array(AppGameAdapterExecutionReadinessReferenceSchema),
  linkedProofArtifacts: Schema.Array(AppGameAdapterExecutionReadinessReferenceSchema),
  manualProofRequirements: Schema.Array(AppGameAdapterExecutionReadinessReferenceSchema),
  claimBoundary: AppGameAdapterExecutionReadinessBoundarySchema,
  fallbackBehavior: AppGameAdapterExecutionReadinessBoundarySchema,
  adapterExecutionClaimed: Schema.Boolean,
  broadInstalledAppBlockingClaimed: Schema.Boolean,
  childDeviceDeliveryClaimed: Schema.Boolean,
  platformEnforcementClaimed: Schema.Boolean,
  providerDeliveryClaimed: Schema.Boolean,
  privateDiagnosticsClaimed: Schema.Boolean,
  lastCheckedAt: ParentTimestampSchema,
});

type AppGameAdapterExecutionReadinessRowCandidate = Infer<typeof AppGameAdapterExecutionReadinessRowBaseSchema>;

export const AppGameAdapterExecutionReadinessRowSchema = withParser(
  AppGameAdapterExecutionReadinessRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        appGameAdapterExecutionReadinessRowIsHonest(row) ||
        'Expected app/game adapter execution readiness rows to allow execution only for the scoped Windows owned-process time-limit boundary and keep broad blocking, delivery, platform enforcement, and private diagnostics unclaimed'
    )
  )
);

export const AppGameAdapterExecutionReadinessReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    readModelId: AppGameAdapterExecutionReadinessReadModelIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceReadModelIds: Schema.Array(AppGameAdapterExecutionReadinessReferenceSchema),
    rows: Schema.Array(AppGameAdapterExecutionReadinessRowSchema),
  }).pipe(
    Schema.filter(
      (readModel) =>
        new Set(readModel.rows.map((row) => row.rowId)).size === readModel.rows.length ||
        'Expected app/game adapter execution readiness rows to have unique ids'
    )
  )
);

function appGameAdapterExecutionReadinessRowIsHonest(row: AppGameAdapterExecutionReadinessRowCandidate): boolean {
  if (appGameAdapterExecutionReadinessRowHasClaimUpgrade(row)) return false;

  return row.adapterExecutionState === 'proved-scoped-execution'
    ? provedScopedExecutionRowIsHonest(row)
    : blockedExecutionReadinessRowIsHonest(row);
}

function provedScopedExecutionRowIsHonest(row: AppGameAdapterExecutionReadinessRowCandidate): boolean {
  return (
    provedScopedExecutionRowMatchesBoundary(row) &&
    provedScopedExecutionRowTracksEvidence(row) &&
    provedScopedExecutionRowTracksHostCapability(row)
  );
}

function blockedExecutionReadinessRowIsHonest(row: AppGameAdapterExecutionReadinessRowCandidate): boolean {
  return (
    row.executionDecision === 'blocked-before-execution' &&
    !row.adapterExecutionClaimed &&
    appGameHostCapabilityStateMatchesEvidence(row) &&
    appGameHostCapabilityProbeRefsAreParentSafe(row) &&
    row.manualProofRequirements.length > 0
  );
}

function provedScopedExecutionRowMatchesBoundary(row: AppGameAdapterExecutionReadinessRowCandidate): boolean {
  return (
    row.platform === 'windows' &&
    row.executionDecision === 'execution-allowed' &&
    row.runtimeBoundary === 'windows-app-game-owned-process-time-limit' &&
    row.targetIdentityState === 'process-session-evidence-backed' &&
    row.rollbackReferenceState === 'timer-recovery-backed' &&
    row.auditReferenceState === 'audit-reference-backed'
  );
}

function provedScopedExecutionRowTracksEvidence(row: AppGameAdapterExecutionReadinessRowCandidate): boolean {
  return (
    row.adapterExecutionClaimed &&
    row.evidenceRefs.length > 0 &&
    row.linkedProofArtifacts.length > 0 &&
    row.manualProofRequirements.length === 0
  );
}

function provedScopedExecutionRowTracksHostCapability(row: AppGameAdapterExecutionReadinessRowCandidate): boolean {
  return (
    row.hostCapabilityState === 'available' &&
    row.hostCapabilityEvidenceRefs.length > 0 &&
    row.hostCapabilityProbeRefs.length > 0
  );
}

function appGameHostCapabilityStateMatchesEvidence(row: AppGameAdapterExecutionReadinessRowCandidate): boolean {
  if (row.hostCapabilityState === 'available') {
    return row.hostCapabilityEvidenceRefs.length > 0;
  }
  if (row.hostCapabilityState === 'not-applicable') {
    return row.hostCapabilityEvidenceRefs.length === 0;
  }
  return true;
}

function appGameHostCapabilityProbeRefsAreParentSafe(row: AppGameAdapterExecutionReadinessRowCandidate): boolean {
  if (row.hostCapabilityState === 'not-applicable') {
    return row.hostCapabilityProbeRefs.length === 0;
  }
  return row.hostCapabilityProbeRefs.every((ref) => String(ref).endsWith('-probe-ref'));
}

function appGameAdapterExecutionReadinessRowHasClaimUpgrade(
  row: AppGameAdapterExecutionReadinessRowCandidate
): boolean {
  return [
    row.broadInstalledAppBlockingClaimed,
    row.childDeviceDeliveryClaimed,
    row.platformEnforcementClaimed,
    row.providerDeliveryClaimed,
    row.privateDiagnosticsClaimed,
  ].some(Boolean);
}

export type AppGameAdapterExecutionReadinessReadModelId = typeof AppGameAdapterExecutionReadinessReadModelIdSchema.Type;
export type AppGameAdapterExecutionReadinessRowId = typeof AppGameAdapterExecutionReadinessRowIdSchema.Type;
export type AppGameAdapterExecutionReadinessReference = typeof AppGameAdapterExecutionReadinessReferenceSchema.Type;
export type AppGameAdapterExecutionReadinessBoundary = typeof AppGameAdapterExecutionReadinessBoundarySchema.Type;
export type AppGameAdapterProductMeaning = Infer<typeof AppGameAdapterProductMeaningSchema>;
export type AppGameAdapterExecutionState = Infer<typeof AppGameAdapterExecutionStateSchema>;
export type AppGameAdapterExecutionDecision = Infer<typeof AppGameAdapterExecutionDecisionSchema>;
export type AppGameAdapterHostCapabilityState = Infer<typeof AppGameAdapterHostCapabilityStateSchema>;
export type AppGameAdapterExecutionReadinessRow = Infer<typeof AppGameAdapterExecutionReadinessRowSchema>;
export type AppGameAdapterExecutionReadinessReadModel = Infer<typeof AppGameAdapterExecutionReadinessReadModelSchema>;

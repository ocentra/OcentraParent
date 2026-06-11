import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentPlatformSchema,
  ParentTimestampSchema,
} from './reference-primitives';
import {
  V08SupportedAdapterRuntimeProofReadModel,
  type V08SupportedAdapterRuntimeProofEntry,
  type V08SupportedAdapterRuntimeProofReadModel as V08SupportedAdapterRuntimeProofReadModelType,
} from './v0-8-supported-adapter-runtime-proof';

const NonEmptyAdapterReadinessText = Schema.String.pipe(Schema.minLength(1));

export const AppGameAdapterExecutionReadinessReadModelIdSchema = NonEmptyAdapterReadinessText.pipe(
  Schema.brand('AppGameAdapterExecutionReadinessReadModelId')
);
export const AppGameAdapterExecutionReadinessRowIdSchema = NonEmptyAdapterReadinessText.pipe(
  Schema.brand('AppGameAdapterExecutionReadinessRowId')
);
export const AppGameAdapterExecutionReadinessReferenceSchema = NonEmptyAdapterReadinessText.pipe(
  Schema.brand('AppGameAdapterExecutionReadinessReference')
);
export const AppGameAdapterExecutionReadinessBoundarySchema = NonEmptyAdapterReadinessText.pipe(
  Schema.brand('AppGameAdapterExecutionReadinessBoundary')
);

export const AppGameAdapterProductMeaningSchema = withParser(Schema.Literal('native-app', 'native-game'));

export const AppGameAdapterExecutionStateSchema = withParser(
  Schema.Literal('proved-scoped-execution', 'manual-required', 'unavailable', 'unsupported', 'degraded', 'not-claimed')
);

export const AppGameAdapterExecutionDecisionSchema = withParser(
  Schema.Literal('execution-allowed', 'blocked-before-execution')
);

export const AppGameAdapterHostCapabilityStateSchema = withParser(
  Schema.Literal('available', 'not-detected', 'not-applicable')
);

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

// eslint-disable-next-line complexity -- proof honesty predicates intentionally enumerate required evidence gates.
function appGameAdapterExecutionReadinessRowIsHonest(row: AppGameAdapterExecutionReadinessRowCandidate): boolean {
  if (appGameAdapterExecutionReadinessRowHasClaimUpgrade(row)) return false;

  if (row.adapterExecutionState === 'proved-scoped-execution') {
    return (
      row.platform === 'windows' &&
      row.executionDecision === 'execution-allowed' &&
      row.runtimeBoundary === 'windows-app-game-owned-process-time-limit' &&
      row.targetIdentityState === 'process-session-evidence-backed' &&
      row.rollbackReferenceState === 'timer-recovery-backed' &&
      row.auditReferenceState === 'audit-reference-backed' &&
      row.adapterExecutionClaimed &&
      row.evidenceRefs.length > 0 &&
      row.hostCapabilityState === 'available' &&
      row.hostCapabilityEvidenceRefs.length > 0 &&
      row.hostCapabilityProbeRefs.length > 0 &&
      row.linkedProofArtifacts.length > 0 &&
      row.manualProofRequirements.length === 0
    );
  }

  return (
    row.executionDecision === 'blocked-before-execution' &&
    !row.adapterExecutionClaimed &&
    appGameHostCapabilityStateMatchesEvidence(row) &&
    appGameHostCapabilityProbeRefsAreParentSafe(row) &&
    row.manualProofRequirements.length > 0
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

const generatedAt = '2026-06-08T09:17:00.000Z';

const AppGameAdapterRuntimeProofEntryIds = new Set([
  'windows-app-game-owned-process-time-limit',
  'windows-broad-installed-app-blocking-manual-gate',
  'windows-broad-installed-app-artifact-status',
  'windows-adapter-permission-dependency-degraded',
  'linux-host-adapter-unavailable',
  'macos-host-adapter-unsupported',
  'android-mobile-control-manual-gate',
  'ios-mobile-control-manual-gate',
]);
const decodeAppGameAdapterExecutionReadinessReference = Schema.decodeUnknownSync(
  AppGameAdapterExecutionReadinessReferenceSchema
);

export const AppGameAdapterExecutionReadinessReadModel = buildAppGameAdapterExecutionReadinessReadModel(
  V08SupportedAdapterRuntimeProofReadModel
);

export function buildAppGameAdapterExecutionReadinessReadModel(
  sourceReadModel: V08SupportedAdapterRuntimeProofReadModelType
): AppGameAdapterExecutionReadinessReadModel {
  return AppGameAdapterExecutionReadinessReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    readModelId: 'app-game-adapter-execution-readiness',
    generatedAt,
    sourceReadModelIds: [String(sourceReadModel.readModelId)],
    rows: sourceReadModel.entries
      .filter((entry) => AppGameAdapterRuntimeProofEntryIds.has(String(entry.proofEntryId)))
      .map(appGameAdapterExecutionReadinessRowFromEntry),
  });
}

function appGameAdapterExecutionReadinessRowFromEntry(
  entry: V08SupportedAdapterRuntimeProofEntry
): AppGameAdapterExecutionReadinessRow {
  const executionState = appGameAdapterExecutionState(entry);
  const executionAllowed = executionState === 'proved-scoped-execution';
  return AppGameAdapterExecutionReadinessRowSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    rowId: `app-game-adapter-execution-${entry.proofEntryId}`,
    sourceProofEntryId: entry.proofEntryId,
    platform: entry.platform,
    productMeanings: ['native-app', 'native-game'],
    adapterCapability: entry.adapterCapability,
    adapterExecutionState: executionState,
    executionDecision: executionAllowed ? 'execution-allowed' : 'blocked-before-execution',
    runtimeBoundary: entry.runtimeBoundary,
    targetIdentityState: entry.targetIdentityState,
    rollbackReferenceState: entry.rollbackReferenceState,
    auditReferenceState: entry.auditReferenceState,
    evidenceRefs: entry.evidenceRefs,
    hostCapabilityState: appGameHostCapabilityState(entry),
    hostCapabilityEvidenceRefs: appGameHostCapabilityEvidenceRefs(entry),
    hostCapabilityProbeRefs: appGameHostCapabilityProbeRefs(entry),
    linkedProofArtifacts: entry.linkedProofArtifacts,
    manualProofRequirements: executionAllowed ? [] : appGameManualProofRequirements(entry),
    claimBoundary: appGameClaimBoundary(entry),
    fallbackBehavior: entry.fallbackBehavior,
    adapterExecutionClaimed: executionAllowed,
    broadInstalledAppBlockingClaimed: false,
    childDeviceDeliveryClaimed: false,
    platformEnforcementClaimed: false,
    providerDeliveryClaimed: false,
    privateDiagnosticsClaimed: false,
    lastCheckedAt: generatedAt,
  });
}

function appGameHostCapabilityProbeRefs(
  entry: V08SupportedAdapterRuntimeProofEntry
): readonly AppGameAdapterExecutionReadinessReference[] {
  if (entry.platform === 'macos' || entry.platform === 'ios') {
    return appGameReadinessReferences([]);
  }
  if (entry.platform === 'windows') {
    return appGameReadinessReferences(['windows-host-local-probe-ref']);
  }
  return appGameReadinessReferences([]);
}

function appGameHostCapabilityState(entry: V08SupportedAdapterRuntimeProofEntry): AppGameAdapterHostCapabilityState {
  if (entry.platform === 'windows') {
    return 'available';
  }
  if (entry.platform === 'macos' || entry.platform === 'ios') {
    return 'not-applicable';
  }
  return 'not-detected';
}

function appGameHostCapabilityEvidenceRefs(
  entry: V08SupportedAdapterRuntimeProofEntry
): readonly AppGameAdapterExecutionReadinessReference[] {
  if (entry.platform === 'windows') {
    return appGameReadinessReferences(['adapter-capability-state-ref']);
  }
  return appGameReadinessReferences([]);
}

function appGameAdapterExecutionState(entry: V08SupportedAdapterRuntimeProofEntry): AppGameAdapterExecutionState {
  if (entry.proofEntryId === 'windows-app-game-owned-process-time-limit') {
    return 'proved-scoped-execution';
  }
  if (entry.runtimeState === 'manual-required') return 'manual-required';
  if (entry.runtimeState === 'unavailable') return 'unavailable';
  if (entry.runtimeState === 'unsupported') return 'unsupported';
  if (entry.runtimeState === 'degraded') return 'degraded';
  return 'not-claimed';
}

function appGameManualProofRequirements(
  entry: V08SupportedAdapterRuntimeProofEntry
): readonly AppGameAdapterExecutionReadinessReference[] {
  if (entry.manualProofRequirements.length > 0) {
    return appGameReadinessReferences(entry.manualProofRequirements);
  }
  return appGameReadinessReferences(['app-game adapter execution proof requirement']);
}

function appGameClaimBoundary(entry: V08SupportedAdapterRuntimeProofEntry): string {
  if (entry.proofEntryId === 'windows-app-game-owned-process-time-limit') {
    return `${entry.claimBoundary} Adapter execution is claimable only for scoped owned-process time-limit rows.`;
  }
  return `${entry.claimBoundary} Adapter execution is blocked before runtime for this app/game row.`;
}

function appGameReadinessReferences(values: readonly string[]): readonly AppGameAdapterExecutionReadinessReference[] {
  return values.map((value) => decodeAppGameAdapterExecutionReadinessReference(value));
}

export function summarizeAppGameAdapterExecutionReadiness(
  readModel: AppGameAdapterExecutionReadinessReadModel
): Record<string, number> {
  return {
    rows: readModel.rows.length,
    executionAllowed: readModel.rows.filter((row) => row.executionDecision === 'execution-allowed').length,
    blockedBeforeExecution: readModel.rows.filter((row) => row.executionDecision === 'blocked-before-execution').length,
    adapterExecutionClaimed: readModel.rows.filter((row) => row.adapterExecutionClaimed).length,
    broadInstalledAppBlockingClaimed: readModel.rows.filter((row) => row.broadInstalledAppBlockingClaimed).length,
    childDeviceDeliveryClaimed: readModel.rows.filter((row) => row.childDeviceDeliveryClaimed).length,
    platformEnforcementClaimed: readModel.rows.filter((row) => row.platformEnforcementClaimed).length,
    providerDeliveryClaimed: readModel.rows.filter((row) => row.providerDeliveryClaimed).length,
    privateDiagnosticsClaimed: readModel.rows.filter((row) => row.privateDiagnosticsClaimed).length,
  };
}

export const decodeAppGameAdapterExecutionReadinessRow = Schema.decodeUnknownSync(
  AppGameAdapterExecutionReadinessRowSchema
);
export const decodeAppGameAdapterExecutionReadinessReadModel = Schema.decodeUnknownSync(
  AppGameAdapterExecutionReadinessReadModelSchema
);

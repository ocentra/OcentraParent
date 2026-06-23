import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from './effect';
import { AppGamePolicyPreviewTargetDomainSchema } from './app-game-policy-preview-handoff';
import { AppGameSourceFreshnessEvidenceRefSchema } from './app-game-source-freshness-policy-consumption';
import { AppGameSourceGatedPolicyPreviewTimerProofRefSchema } from './app-game-source-gated-policy-preview-timer-status';
import {
  AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessIdSchema,
  AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessRowIdSchema,
  AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessSchema,
  type AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessRow,
} from './app-game-source-gated-policy-preview-timer-runtime-readiness';
import {
  AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceNoClaimFlags,
  AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceState,
  RequiredAppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceNonClaims,
  appGameSourceGatedPolicyPreviewTimerSchedulerPersistenceCountsMatch,
  appGameSourceGatedPolicyPreviewTimerSchedulerPersistenceHasNoRuntimeClaims,
  appGameSourceGatedPolicyPreviewTimerSchedulerPersistenceMatchesRuntimeReadiness,
} from './app-game-source-gated-policy-preview-timer-scheduler-persistence-rules';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './family-reference-primitives';

export const AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceIdSchema = brandedNonEmptyStringSchema(
  'AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceId'
);
export const AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceRowIdSchema = brandedNonEmptyStringSchema(
  'AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceRowId'
);
export const AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceContractRefSchema = brandedNonEmptyStringSchema(
  'AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceContractRef'
);

export const AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceStateSchema = withParser(
  Schema.Literal(...Object.values(AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceState))
);
export const AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceNonClaimSchema = withParser(
  Schema.Literal(...RequiredAppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceNonClaims)
);

export const AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceOptionsSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    persistenceId: AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceContractRefs: Schema.Array(AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceContractRefSchema),
    serviceTimerRuntimeProofRef: AppGameSourceGatedPolicyPreviewTimerProofRefSchema,
    schedulerPersistenceProofRef: AppGameSourceGatedPolicyPreviewTimerProofRefSchema,
    schedulerStateStoreProofRef: AppGameSourceGatedPolicyPreviewTimerProofRefSchema,
    auditProofRef: AppGameSourceGatedPolicyPreviewTimerProofRefSchema,
    rollbackProofRef: AppGameSourceGatedPolicyPreviewTimerProofRefSchema,
  }).pipe(
    Schema.filter(
      (options) =>
        options.sourceContractRefs.length > 0 ||
        'Expected source-gated policy preview timer scheduler persistence options to cite source contracts'
    )
  )
);

const AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  rowId: AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceRowIdSchema,
  sourceRuntimeReadinessRowId: AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessRowIdSchema,
  targetDomain: AppGamePolicyPreviewTargetDomainSchema,
  schedulerPersistenceState: AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceStateSchema,
  serviceTimerRuntimeProofRequired: Schema.Boolean,
  schedulerPersistenceProofRequired: Schema.Boolean,
  schedulerStateStoreProofRequired: Schema.Boolean,
  auditProofRequired: Schema.Boolean,
  rollbackProofRequired: Schema.Boolean,
  requiredProofRefs: Schema.Array(AppGameSourceGatedPolicyPreviewTimerProofRefSchema),
  sourceEvidenceRefs: Schema.Array(AppGameSourceFreshnessEvidenceRefSchema),
  serviceRuntimeEventClaimed: Schema.Literal(false),
  portalUiRendered: Schema.Literal(false),
  policyEvaluatorRuntimeClaimed: Schema.Literal(false),
  timerRuntimeClaimed: Schema.Literal(false),
  timerScheduled: Schema.Literal(false),
  schedulerPersistenceRuntimeClaimed: Schema.Literal(false),
  durableSchedulerStorageClaimed: Schema.Literal(false),
  auditRuntimeClaimed: Schema.Literal(false),
  rollbackRuntimeClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  childDeliveryClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  rawPrivateSourceRowsIncluded: Schema.Literal(false),
  generatedAt: ParentTimestampSchema,
});

export const AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceRowSchema = withParser(
  AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        row.requiredProofRefs.length > 0 ||
        'Expected source-gated policy preview timer scheduler persistence rows to name required proof refs'
    )
  )
);

const AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  persistenceId: AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceIdSchema,
  sourceRuntimeReadinessId: AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceContractRefs: Schema.Array(AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceContractRefSchema),
  rows: Schema.Array(AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceRowSchema),
  nativeAppRowCount: Schema.Number,
  nativeGameRowCount: Schema.Number,
  schedulerPersistenceProofRequiredCount: Schema.Number,
  blockedBySourceFreshnessCount: Schema.Number,
  blockedByCompilerDecisionCount: Schema.Number,
  schedulerPersistenceNonClaims: Schema.Array(AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceNonClaimSchema),
  serviceRuntimeEventClaimed: Schema.Literal(false),
  portalUiRendered: Schema.Literal(false),
  policyEvaluatorRuntimeClaimed: Schema.Literal(false),
  timerRuntimeClaimed: Schema.Literal(false),
  timerScheduled: Schema.Literal(false),
  schedulerPersistenceRuntimeClaimed: Schema.Literal(false),
  durableSchedulerStorageClaimed: Schema.Literal(false),
  auditRuntimeClaimed: Schema.Literal(false),
  rollbackRuntimeClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  childDeliveryClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  rawPrivateSourceRowsIncluded: Schema.Literal(false),
});

export const AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceSchema = withParser(
  AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceBaseSchema.pipe(
    Schema.filter(
      (persistence) =>
        appGameSourceGatedPolicyPreviewTimerSchedulerPersistenceCountsMatch(persistence) ||
        'Expected source-gated policy preview timer scheduler persistence counts to match row states'
    )
  ).pipe(
    Schema.filter(
      (persistence) =>
        appGameSourceGatedPolicyPreviewTimerSchedulerPersistenceHasNoRuntimeClaims(persistence) ||
        'Expected source-gated policy preview timer scheduler persistence to avoid runtime, UI, timer, scheduler, adapter, and raw-source claims'
    )
  )
);

export type AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceOptions = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceOptionsSchema
>;
export type AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceRow = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceRowSchema
>;
export type AppGameSourceGatedPolicyPreviewTimerSchedulerPersistence = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceSchema
>;

export function buildAppGameSourceGatedPolicyPreviewTimerSchedulerPersistence(
  optionsInput: unknown,
  runtimeReadinessInput: unknown
): AppGameSourceGatedPolicyPreviewTimerSchedulerPersistence {
  const options = AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceOptionsSchema.parse(optionsInput);
  const runtimeReadiness = AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessSchema.parse(runtimeReadinessInput);
  const rows = runtimeReadiness.rows.map((row) => buildSchedulerPersistenceRow(options, row));

  return AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceSchema.parse({
    schemaVersion: options.schemaVersion,
    persistenceId: options.persistenceId,
    sourceRuntimeReadinessId: runtimeReadiness.readinessId,
    generatedAt: options.generatedAt,
    sourceContractRefs: options.sourceContractRefs,
    rows,
    nativeAppRowCount: runtimeReadiness.nativeAppRowCount,
    nativeGameRowCount: runtimeReadiness.nativeGameRowCount,
    schedulerPersistenceProofRequiredCount: rows.filter(
      (row) =>
        row.schedulerPersistenceState ===
        AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceState.SchedulerPersistenceProofRequired
    ).length,
    blockedBySourceFreshnessCount: rows.filter(
      (row) =>
        row.schedulerPersistenceState ===
        AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceState.BlockedBySourceFreshness
    ).length,
    blockedByCompilerDecisionCount: rows.filter(
      (row) =>
        row.schedulerPersistenceState ===
        AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceState.BlockedByCompilerDecision
    ).length,
    schedulerPersistenceNonClaims: RequiredAppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceNonClaims,
    ...AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceNoClaimFlags,
  });
}

function buildSchedulerPersistenceRow(
  options: AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceOptions,
  runtimeReadinessRow: AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessRow
): AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceRow {
  const schedulerPersistenceState = schedulerPersistenceStateForRuntimeReadiness(runtimeReadinessRow);
  const schedulerProofRequired =
    schedulerPersistenceState ===
    AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceState.SchedulerPersistenceProofRequired;

  return AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceRowSchema.parse({
    schemaVersion: options.schemaVersion,
    rowId: `${runtimeReadinessRow.rowId}:scheduler-persistence`,
    sourceRuntimeReadinessRowId: runtimeReadinessRow.rowId,
    targetDomain: runtimeReadinessRow.targetDomain,
    schedulerPersistenceState,
    serviceTimerRuntimeProofRequired: schedulerProofRequired,
    schedulerPersistenceProofRequired: schedulerProofRequired,
    schedulerStateStoreProofRequired: schedulerProofRequired,
    auditProofRequired: schedulerProofRequired,
    rollbackProofRequired: schedulerProofRequired,
    requiredProofRefs: requiredProofRefsForSchedulerPersistence(
      options,
      schedulerPersistenceState,
      runtimeReadinessRow
    ),
    sourceEvidenceRefs: runtimeReadinessRow.sourceEvidenceRefs,
    ...AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceNoClaimFlags,
    generatedAt: options.generatedAt,
  });
}

function schedulerPersistenceStateForRuntimeReadiness(
  runtimeReadinessRow: AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessRow
) {
  for (const state of Object.values(AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceState)) {
    if (
      appGameSourceGatedPolicyPreviewTimerSchedulerPersistenceMatchesRuntimeReadiness(
        runtimeReadinessRow.runtimeReadinessState,
        state
      )
    ) {
      return state;
    }
  }
  return AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceState.BlockedByCompilerDecision;
}

function requiredProofRefsForSchedulerPersistence(
  options: AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceOptions,
  schedulerPersistenceState: string,
  runtimeReadinessRow: AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessRow
) {
  if (
    schedulerPersistenceState ===
    AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceState.SchedulerPersistenceProofRequired
  ) {
    return [
      options.serviceTimerRuntimeProofRef,
      options.schedulerPersistenceProofRef,
      options.schedulerStateStoreProofRef,
      options.auditProofRef,
      options.rollbackProofRef,
    ];
  }
  return runtimeReadinessRow.requiredProofRefs;
}

export const decodeAppGameSourceGatedPolicyPreviewTimerSchedulerPersistence = Schema.decodeUnknownSync(
  AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceSchema
);

export { AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceState };

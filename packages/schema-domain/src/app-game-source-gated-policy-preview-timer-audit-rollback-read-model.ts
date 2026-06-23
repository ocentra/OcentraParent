import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from './effect';
import { AppGamePolicyPreviewTargetDomainSchema } from './app-game-policy-preview-handoff';
import { AppGameSourceFreshnessEvidenceRefSchema } from './app-game-source-freshness-policy-consumption';
import {
  AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffIdSchema,
  AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffRowIdSchema,
  AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffSchema,
  type AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffRow,
} from './app-game-source-gated-policy-preview-timer-audit-rollback-handoff';
import { AppGameSourceGatedPolicyPreviewTimerProofRefSchema } from './app-game-source-gated-policy-preview-timer-status';
import { AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceRowIdSchema } from './app-game-source-gated-policy-preview-timer-scheduler-persistence';
import {
  AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelNoClaimFlags,
  AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelState,
  RequiredAppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelNonClaims,
  type AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelStateValue,
  appGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelCountsMatch,
  appGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelHasNoRuntimeClaims,
  appGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelMatchesHandoff,
} from './app-game-source-gated-policy-preview-timer-audit-rollback-read-model-rules';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './family-reference-primitives';

export const AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelIdSchema = brandedNonEmptyStringSchema(
  'AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelId'
);
export const AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelRowIdSchema = brandedNonEmptyStringSchema(
  'AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelRowId'
);
export const AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelContractRefSchema = brandedNonEmptyStringSchema(
  'AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelContractRef'
);
export const AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelSummaryRefSchema = brandedNonEmptyStringSchema(
  'AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelSummaryRef'
);

export const AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelStateSchema = withParser(
  Schema.Literal(...Object.values(AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelState))
);
export const AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelNonClaimSchema = withParser(
  Schema.Literal(...RequiredAppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelNonClaims)
);

export const AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelOptionsSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    readModelId: AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceContractRefs: Schema.Array(AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelContractRefSchema),
    parentVisibleSummaryRef: AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelSummaryRefSchema,
  }).pipe(
    Schema.filter(
      (options) =>
        options.sourceContractRefs.length > 0 ||
        'Expected source-gated policy preview timer audit rollback read model options to cite source contracts'
    )
  )
);

const AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  rowId: AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelRowIdSchema,
  sourceAuditRollbackHandoffRowId: AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffRowIdSchema,
  sourceSchedulerPersistenceRowId: AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceRowIdSchema,
  targetDomain: AppGamePolicyPreviewTargetDomainSchema,
  readModelState: AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelStateSchema,
  serviceTimerRuntimeProofRequired: Schema.Boolean,
  schedulerPersistenceProofRequired: Schema.Boolean,
  schedulerStateStoreProofRequired: Schema.Boolean,
  auditTrailProofRequired: Schema.Boolean,
  rollbackPlanProofRequired: Schema.Boolean,
  auditRollbackReadModelProofRequired: Schema.Boolean,
  requiredProofRefs: Schema.Array(AppGameSourceGatedPolicyPreviewTimerProofRefSchema),
  sourceEvidenceRefs: Schema.Array(AppGameSourceFreshnessEvidenceRefSchema),
  parentVisibleSummaryRef: AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelSummaryRefSchema,
  serviceRuntimeEventClaimed: Schema.Literal(false),
  portalUiRendered: Schema.Literal(false),
  policyEvaluatorRuntimeClaimed: Schema.Literal(false),
  timerRuntimeClaimed: Schema.Literal(false),
  timerScheduled: Schema.Literal(false),
  schedulerPersistenceRuntimeClaimed: Schema.Literal(false),
  durableSchedulerStorageClaimed: Schema.Literal(false),
  auditRuntimeClaimed: Schema.Literal(false),
  durableAuditLogClaimed: Schema.Literal(false),
  rollbackRuntimeClaimed: Schema.Literal(false),
  rollbackExecutionClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  childDeliveryClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  rawPrivateSourceRowsIncluded: Schema.Literal(false),
  generatedAt: ParentTimestampSchema,
});

export const AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelRowSchema = withParser(
  AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        row.requiredProofRefs.length > 0 ||
        'Expected source-gated policy preview timer audit rollback read-model rows to name required proof refs'
    )
  )
);

const AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  readModelId: AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelIdSchema,
  sourceAuditRollbackHandoffId: AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceContractRefs: Schema.Array(AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelContractRefSchema),
  parentVisibleSummaryRef: AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelSummaryRefSchema,
  rows: Schema.Array(AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelRowSchema),
  nativeAppRowCount: Schema.Number,
  nativeGameRowCount: Schema.Number,
  auditRollbackReadModelProofRequiredCount: Schema.Number,
  blockedBySourceFreshnessCount: Schema.Number,
  blockedByCompilerDecisionCount: Schema.Number,
  auditRollbackReadModelNonClaims: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelNonClaimSchema
  ),
  serviceRuntimeEventClaimed: Schema.Literal(false),
  portalUiRendered: Schema.Literal(false),
  policyEvaluatorRuntimeClaimed: Schema.Literal(false),
  timerRuntimeClaimed: Schema.Literal(false),
  timerScheduled: Schema.Literal(false),
  schedulerPersistenceRuntimeClaimed: Schema.Literal(false),
  durableSchedulerStorageClaimed: Schema.Literal(false),
  auditRuntimeClaimed: Schema.Literal(false),
  durableAuditLogClaimed: Schema.Literal(false),
  rollbackRuntimeClaimed: Schema.Literal(false),
  rollbackExecutionClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  childDeliveryClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  rawPrivateSourceRowsIncluded: Schema.Literal(false),
});

export const AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelSchema = withParser(
  AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        appGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelCountsMatch(readModel) ||
        'Expected source-gated policy preview timer audit rollback read model counts to match row states'
    )
  ).pipe(
    Schema.filter(
      (readModel) =>
        appGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelHasNoRuntimeClaims(readModel) ||
        'Expected source-gated policy preview timer audit rollback read model to avoid runtime, UI, timer, scheduler, audit, rollback, adapter, and raw-source claims'
    )
  )
);

export type AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelOptions = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelOptionsSchema
>;
export type AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelRow = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelRowSchema
>;
export type AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModel = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelSchema
>;

export function buildAppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModel(
  optionsInput: unknown,
  auditRollbackHandoffInput: unknown
): AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModel {
  const options = AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelOptionsSchema.parse(optionsInput);
  const handoff = AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffSchema.parse(auditRollbackHandoffInput);
  const rows = handoff.rows.map((row) => buildAuditRollbackReadModelRow(options, row));

  return AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelSchema.parse({
    schemaVersion: options.schemaVersion,
    readModelId: options.readModelId,
    sourceAuditRollbackHandoffId: handoff.handoffId,
    generatedAt: options.generatedAt,
    sourceContractRefs: options.sourceContractRefs,
    parentVisibleSummaryRef: options.parentVisibleSummaryRef,
    rows,
    nativeAppRowCount: handoff.nativeAppRowCount,
    nativeGameRowCount: handoff.nativeGameRowCount,
    auditRollbackReadModelProofRequiredCount: rows.filter(
      (row) =>
        row.readModelState ===
        AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelState.AuditRollbackReadModelProofRequired
    ).length,
    blockedBySourceFreshnessCount: rows.filter(
      (row) =>
        row.readModelState === AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelState.BlockedBySourceFreshness
    ).length,
    blockedByCompilerDecisionCount: rows.filter(
      (row) =>
        row.readModelState === AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelState.BlockedByCompilerDecision
    ).length,
    auditRollbackReadModelNonClaims: RequiredAppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelNonClaims,
    ...AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelNoClaimFlags,
  });
}

function buildAuditRollbackReadModelRow(
  options: AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelOptions,
  handoffRow: AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffRow
): AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelRow {
  const readModelState = readModelStateForAuditRollbackHandoff(handoffRow);

  return AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelRowSchema.parse({
    schemaVersion: options.schemaVersion,
    rowId: `${handoffRow.rowId}:audit-rollback-read-model`,
    sourceAuditRollbackHandoffRowId: handoffRow.rowId,
    sourceSchedulerPersistenceRowId: handoffRow.sourceSchedulerPersistenceRowId,
    targetDomain: handoffRow.targetDomain,
    readModelState,
    serviceTimerRuntimeProofRequired: handoffRow.serviceTimerRuntimeProofRequired,
    schedulerPersistenceProofRequired: handoffRow.schedulerPersistenceProofRequired,
    schedulerStateStoreProofRequired: handoffRow.schedulerStateStoreProofRequired,
    auditTrailProofRequired: handoffRow.auditTrailProofRequired,
    rollbackPlanProofRequired: handoffRow.rollbackPlanProofRequired,
    auditRollbackReadModelProofRequired: handoffRow.auditRollbackReadModelProofRequired,
    requiredProofRefs: handoffRow.requiredProofRefs,
    sourceEvidenceRefs: handoffRow.sourceEvidenceRefs,
    parentVisibleSummaryRef: options.parentVisibleSummaryRef,
    ...AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelNoClaimFlags,
    generatedAt: options.generatedAt,
  });
}

function readModelStateForAuditRollbackHandoff(
  handoffRow: AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffRow
): AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelStateValue {
  for (const state of Object.values(AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelState)) {
    if (
      appGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelMatchesHandoff(handoffRow.auditRollbackState, state)
    ) {
      return state;
    }
  }
  return AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelState.BlockedByCompilerDecision;
}

export const decodeAppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModel = Schema.decodeUnknownSync(
  AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelSchema
);

export { AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelState };

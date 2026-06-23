import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from './effect';
import { AppGamePolicyPreviewTargetDomainSchema } from './app-game-policy-preview-handoff';
import { AppGameSourceFreshnessEvidenceRefSchema } from './app-game-source-freshness-policy-consumption';
import { AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffRowIdSchema } from './app-game-source-gated-policy-preview-timer-audit-rollback-handoff';
import {
  AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelIdSchema,
  AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelRowIdSchema,
  AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelSchema,
  type AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelRow,
} from './app-game-source-gated-policy-preview-timer-audit-rollback-read-model';
import { AppGameSourceGatedPolicyPreviewTimerProofRefSchema } from './app-game-source-gated-policy-preview-timer-status';
import { AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceRowIdSchema } from './app-game-source-gated-policy-preview-timer-scheduler-persistence';
import {
  AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentNoClaimFlags,
  AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentState,
  RequiredAppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentNonClaims,
  type AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentStateValue,
  appGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentCountsMatch,
  appGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentHasNoRuntimeClaims,
  appGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentMatchesReadModel,
} from './app-game-source-gated-policy-preview-timer-audit-rollback-parent-surface-intent-rules';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './family-reference-primitives';

export const AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentIdSchema = brandedNonEmptyStringSchema(
  'AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentId'
);
export const AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentRowIdSchema =
  brandedNonEmptyStringSchema('AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentRowId');
export const AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentContractRefSchema =
  brandedNonEmptyStringSchema('AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentContractRef');
export const AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentDrillInRefSchema =
  brandedNonEmptyStringSchema('AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentDrillInRef');

export const AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentStateSchema = withParser(
  Schema.Literal(...Object.values(AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentState))
);
export const AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentNonClaimSchema = withParser(
  Schema.Literal(...RequiredAppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentNonClaims)
);

export const AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentOptionsSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    intentId: AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceContractRefs: Schema.Array(
      AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentContractRefSchema
    ),
    parentSurfaceDrillInRef: AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentDrillInRefSchema,
    parentSurfaceProofRef: AppGameSourceGatedPolicyPreviewTimerProofRefSchema,
  }).pipe(
    Schema.filter(
      (options) =>
        options.sourceContractRefs.length > 0 ||
        'Expected source-gated policy preview timer audit rollback parent-surface intent options to cite source contracts'
    )
  )
);

const AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  rowId: AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentRowIdSchema,
  sourceAuditRollbackReadModelRowId: AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelRowIdSchema,
  sourceAuditRollbackHandoffRowId: AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffRowIdSchema,
  sourceSchedulerPersistenceRowId: AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceRowIdSchema,
  targetDomain: AppGamePolicyPreviewTargetDomainSchema,
  parentSurfaceIntentState: AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentStateSchema,
  serviceTimerRuntimeProofRequired: Schema.Boolean,
  schedulerPersistenceProofRequired: Schema.Boolean,
  schedulerStateStoreProofRequired: Schema.Boolean,
  auditTrailProofRequired: Schema.Boolean,
  rollbackPlanProofRequired: Schema.Boolean,
  auditRollbackReadModelProofRequired: Schema.Boolean,
  parentSurfaceProofRequired: Schema.Boolean,
  requiredProofRefs: Schema.Array(AppGameSourceGatedPolicyPreviewTimerProofRefSchema),
  sourceEvidenceRefs: Schema.Array(AppGameSourceFreshnessEvidenceRefSchema),
  parentSurfaceDrillInRef: AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentDrillInRefSchema,
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

export const AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentRowSchema = withParser(
  AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        row.requiredProofRefs.length > 0 ||
        'Expected source-gated policy preview timer audit rollback parent-surface intent rows to name required proof refs'
    )
  )
);

const AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  intentId: AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentIdSchema,
  sourceAuditRollbackReadModelId: AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceContractRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentContractRefSchema
  ),
  rows: Schema.Array(AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentRowSchema),
  nativeAppRowCount: Schema.Number,
  nativeGameRowCount: Schema.Number,
  auditRollbackParentSurfaceProofRequiredCount: Schema.Number,
  blockedBySourceFreshnessCount: Schema.Number,
  blockedByCompilerDecisionCount: Schema.Number,
  parentSurfaceIntentNonClaims: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentNonClaimSchema
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

export const AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentSchema = withParser(
  AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentBaseSchema.pipe(
    Schema.filter(
      (intent) =>
        appGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentCountsMatch(intent) ||
        'Expected source-gated policy preview timer audit rollback parent-surface intent counts to match row states'
    )
  ).pipe(
    Schema.filter(
      (intent) =>
        appGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentHasNoRuntimeClaims(intent) ||
        'Expected source-gated policy preview timer audit rollback parent-surface intent to avoid runtime, UI, timer, scheduler, audit, rollback, adapter, and raw-source claims'
    )
  )
);

export type AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentOptions = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentOptionsSchema
>;
export type AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentRow = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentRowSchema
>;
export type AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntent = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentSchema
>;

export function buildAppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntent(
  optionsInput: unknown,
  auditRollbackReadModelInput: unknown
): AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntent {
  const options = AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentOptionsSchema.parse(optionsInput);
  const readModel = AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelSchema.parse(auditRollbackReadModelInput);
  const rows = readModel.rows.map((row) => buildParentSurfaceIntentRow(options, row));

  return AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentSchema.parse({
    schemaVersion: options.schemaVersion,
    intentId: options.intentId,
    sourceAuditRollbackReadModelId: readModel.readModelId,
    generatedAt: options.generatedAt,
    sourceContractRefs: options.sourceContractRefs,
    rows,
    nativeAppRowCount: readModel.nativeAppRowCount,
    nativeGameRowCount: readModel.nativeGameRowCount,
    auditRollbackParentSurfaceProofRequiredCount: rows.filter(
      (row) =>
        row.parentSurfaceIntentState ===
        AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentState.AuditRollbackParentSurfaceProofRequired
    ).length,
    blockedBySourceFreshnessCount: rows.filter(
      (row) =>
        row.parentSurfaceIntentState ===
        AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentState.BlockedBySourceFreshness
    ).length,
    blockedByCompilerDecisionCount: rows.filter(
      (row) =>
        row.parentSurfaceIntentState ===
        AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentState.BlockedByCompilerDecision
    ).length,
    parentSurfaceIntentNonClaims: RequiredAppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentNonClaims,
    ...AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentNoClaimFlags,
  });
}

function buildParentSurfaceIntentRow(
  options: AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentOptions,
  readModelRow: AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelRow
): AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentRow {
  const parentSurfaceIntentState = parentSurfaceIntentStateForReadModel(readModelRow);
  const parentSurfaceProofRequired =
    parentSurfaceIntentState ===
    AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentState.AuditRollbackParentSurfaceProofRequired;

  return AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentRowSchema.parse({
    schemaVersion: options.schemaVersion,
    rowId: `${readModelRow.rowId}:parent-surface-intent`,
    sourceAuditRollbackReadModelRowId: readModelRow.rowId,
    sourceAuditRollbackHandoffRowId: readModelRow.sourceAuditRollbackHandoffRowId,
    sourceSchedulerPersistenceRowId: readModelRow.sourceSchedulerPersistenceRowId,
    targetDomain: readModelRow.targetDomain,
    parentSurfaceIntentState,
    serviceTimerRuntimeProofRequired: readModelRow.serviceTimerRuntimeProofRequired,
    schedulerPersistenceProofRequired: readModelRow.schedulerPersistenceProofRequired,
    schedulerStateStoreProofRequired: readModelRow.schedulerStateStoreProofRequired,
    auditTrailProofRequired: readModelRow.auditTrailProofRequired,
    rollbackPlanProofRequired: readModelRow.rollbackPlanProofRequired,
    auditRollbackReadModelProofRequired: readModelRow.auditRollbackReadModelProofRequired,
    parentSurfaceProofRequired,
    requiredProofRefs: parentSurfaceProofRequired
      ? [...readModelRow.requiredProofRefs, options.parentSurfaceProofRef]
      : readModelRow.requiredProofRefs,
    sourceEvidenceRefs: readModelRow.sourceEvidenceRefs,
    parentSurfaceDrillInRef: options.parentSurfaceDrillInRef,
    ...AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentNoClaimFlags,
    generatedAt: options.generatedAt,
  });
}

function parentSurfaceIntentStateForReadModel(
  readModelRow: AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelRow
): AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentStateValue {
  for (const state of Object.values(AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentState)) {
    if (
      appGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentMatchesReadModel(
        readModelRow.readModelState,
        state
      )
    ) {
      return state;
    }
  }
  return AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentState.BlockedByCompilerDecision;
}

export const decodeAppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntent = Schema.decodeUnknownSync(
  AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentSchema
);

export { AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentState };

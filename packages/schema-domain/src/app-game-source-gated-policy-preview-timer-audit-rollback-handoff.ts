import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from './effect';
import { AppGamePolicyPreviewTargetDomainSchema } from './app-game-policy-preview-handoff';
import { AppGameSourceFreshnessEvidenceRefSchema } from './app-game-source-freshness-policy-consumption';
import { AppGameSourceGatedPolicyPreviewTimerProofRefSchema } from './app-game-source-gated-policy-preview-timer-status';
import {
  AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceIdSchema,
  AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceRowIdSchema,
  AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceSchema,
  type AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceRow,
} from './app-game-source-gated-policy-preview-timer-scheduler-persistence';
import {
  AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffNoClaimFlags,
  AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffState,
  RequiredAppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffNonClaims,
  type AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffStateValue,
  appGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffCountsMatch,
  appGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffHasNoRuntimeClaims,
  appGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffMatchesSchedulerPersistence,
} from './app-game-source-gated-policy-preview-timer-audit-rollback-handoff-rules';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './family-reference-primitives';

export const AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffIdSchema =
  brandedNonEmptyStringSchema('AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffId');
export const AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffRowIdSchema =
  brandedNonEmptyStringSchema('AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffRowId');
export const AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffContractRefSchema =
  brandedNonEmptyStringSchema('AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffContractRef');

export const AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffStateSchema = withParser(
  Schema.Literal(...Object.values(AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffState))
);
export const AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffNonClaimSchema = withParser(
  Schema.Literal(...RequiredAppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffNonClaims)
);

export const AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffOptionsSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    handoffId: AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceContractRefs: Schema.Array(AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffContractRefSchema),
    serviceTimerRuntimeProofRef: AppGameSourceGatedPolicyPreviewTimerProofRefSchema,
    schedulerPersistenceProofRef: AppGameSourceGatedPolicyPreviewTimerProofRefSchema,
    schedulerStateStoreProofRef: AppGameSourceGatedPolicyPreviewTimerProofRefSchema,
    auditTrailProofRef: AppGameSourceGatedPolicyPreviewTimerProofRefSchema,
    rollbackPlanProofRef: AppGameSourceGatedPolicyPreviewTimerProofRefSchema,
    auditRollbackReadModelProofRef: AppGameSourceGatedPolicyPreviewTimerProofRefSchema,
  }).pipe(
    Schema.filter(
      (options) =>
        options.sourceContractRefs.length > 0 ||
        'Expected source-gated policy preview timer audit rollback handoff options to cite source contracts'
    )
  )
);

const AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  rowId: AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffRowIdSchema,
  sourceSchedulerPersistenceRowId: AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceRowIdSchema,
  targetDomain: AppGamePolicyPreviewTargetDomainSchema,
  auditRollbackState: AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffStateSchema,
  serviceTimerRuntimeProofRequired: Schema.Boolean,
  schedulerPersistenceProofRequired: Schema.Boolean,
  schedulerStateStoreProofRequired: Schema.Boolean,
  auditTrailProofRequired: Schema.Boolean,
  rollbackPlanProofRequired: Schema.Boolean,
  auditRollbackReadModelProofRequired: Schema.Boolean,
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
  durableAuditLogClaimed: Schema.Literal(false),
  rollbackRuntimeClaimed: Schema.Literal(false),
  rollbackExecutionClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  childDeliveryClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  rawPrivateSourceRowsIncluded: Schema.Literal(false),
  generatedAt: ParentTimestampSchema,
});

export const AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffRowSchema = withParser(
  AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        row.requiredProofRefs.length > 0 ||
        'Expected source-gated policy preview timer audit rollback handoff rows to name required proof refs'
    )
  )
);

const AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  handoffId: AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffIdSchema,
  sourceSchedulerPersistenceId: AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceContractRefs: Schema.Array(AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffContractRefSchema),
  rows: Schema.Array(AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffRowSchema),
  nativeAppRowCount: Schema.Number,
  nativeGameRowCount: Schema.Number,
  auditRollbackProofRequiredCount: Schema.Number,
  blockedBySourceFreshnessCount: Schema.Number,
  blockedByCompilerDecisionCount: Schema.Number,
  auditRollbackNonClaims: Schema.Array(AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffNonClaimSchema),
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

export const AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffSchema = withParser(
  AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffBaseSchema.pipe(
    Schema.filter(
      (handoff) =>
        appGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffCountsMatch(handoff) ||
        'Expected source-gated policy preview timer audit rollback handoff counts to match row states'
    )
  ).pipe(
    Schema.filter(
      (handoff) =>
        appGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffHasNoRuntimeClaims(handoff) ||
        'Expected source-gated policy preview timer audit rollback handoff to avoid runtime, UI, timer, scheduler, audit, rollback, adapter, and raw-source claims'
    )
  )
);

export type AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffOptions = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffOptionsSchema
>;
export type AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffRow = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffRowSchema
>;
export type AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoff = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffSchema
>;

export function buildAppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoff(
  optionsInput: unknown,
  schedulerPersistenceInput: unknown
): AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoff {
  const options = AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffOptionsSchema.parse(optionsInput);
  const schedulerPersistence =
    AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceSchema.parse(schedulerPersistenceInput);
  const rows = schedulerPersistence.rows.map((row) => buildAuditRollbackHandoffRow(options, row));

  return AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffSchema.parse({
    schemaVersion: options.schemaVersion,
    handoffId: options.handoffId,
    sourceSchedulerPersistenceId: schedulerPersistence.persistenceId,
    generatedAt: options.generatedAt,
    sourceContractRefs: options.sourceContractRefs,
    rows,
    nativeAppRowCount: schedulerPersistence.nativeAppRowCount,
    nativeGameRowCount: schedulerPersistence.nativeGameRowCount,
    auditRollbackProofRequiredCount: rows.filter(
      (row) =>
        row.auditRollbackState ===
        AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffState.AuditRollbackProofRequired
    ).length,
    blockedBySourceFreshnessCount: rows.filter(
      (row) =>
        row.auditRollbackState ===
        AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffState.BlockedBySourceFreshness
    ).length,
    blockedByCompilerDecisionCount: rows.filter(
      (row) =>
        row.auditRollbackState ===
        AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffState.BlockedByCompilerDecision
    ).length,
    auditRollbackNonClaims: RequiredAppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffNonClaims,
    ...AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffNoClaimFlags,
  });
}

function buildAuditRollbackHandoffRow(
  options: AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffOptions,
  schedulerPersistenceRow: AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceRow
): AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffRow {
  const auditRollbackState = auditRollbackStateForSchedulerPersistence(schedulerPersistenceRow);
  const proofRequired =
    auditRollbackState === AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffState.AuditRollbackProofRequired;

  return AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffRowSchema.parse({
    schemaVersion: options.schemaVersion,
    rowId: `${schedulerPersistenceRow.rowId}:audit-rollback-handoff`,
    sourceSchedulerPersistenceRowId: schedulerPersistenceRow.rowId,
    targetDomain: schedulerPersistenceRow.targetDomain,
    auditRollbackState,
    serviceTimerRuntimeProofRequired: proofRequired,
    schedulerPersistenceProofRequired: proofRequired,
    schedulerStateStoreProofRequired: proofRequired,
    auditTrailProofRequired: proofRequired,
    rollbackPlanProofRequired: proofRequired,
    auditRollbackReadModelProofRequired: proofRequired,
    requiredProofRefs: requiredProofRefsForAuditRollback(options, auditRollbackState, schedulerPersistenceRow),
    sourceEvidenceRefs: schedulerPersistenceRow.sourceEvidenceRefs,
    ...AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffNoClaimFlags,
    generatedAt: options.generatedAt,
  });
}

function auditRollbackStateForSchedulerPersistence(
  schedulerPersistenceRow: AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceRow
) {
  for (const state of Object.values(AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffState)) {
    if (
      appGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffMatchesSchedulerPersistence(
        schedulerPersistenceRow.schedulerPersistenceState,
        state
      )
    ) {
      return state;
    }
  }
  return AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffState.BlockedByCompilerDecision;
}

function requiredProofRefsForAuditRollback(
  options: AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffOptions,
  auditRollbackState: AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffStateValue,
  schedulerPersistenceRow: AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceRow
) {
  if (auditRollbackState === AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffState.AuditRollbackProofRequired) {
    return [
      options.serviceTimerRuntimeProofRef,
      options.schedulerPersistenceProofRef,
      options.schedulerStateStoreProofRef,
      options.auditTrailProofRef,
      options.rollbackPlanProofRef,
      options.auditRollbackReadModelProofRef,
    ];
  }
  return schedulerPersistenceRow.requiredProofRefs;
}

export const decodeAppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoff = Schema.decodeUnknownSync(
  AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffSchema
);

export { AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffState };



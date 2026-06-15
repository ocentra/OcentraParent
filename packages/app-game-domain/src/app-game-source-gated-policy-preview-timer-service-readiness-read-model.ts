import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AppGamePolicyPreviewTargetDomainSchema } from './app-game-policy-preview-handoff';
import { AppGameSourceFreshnessEvidenceRefSchema } from './app-game-source-freshness-policy-consumption';
import { AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffRowIdSchema } from './app-game-source-gated-policy-preview-timer-audit-rollback-handoff';
import { AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentRowIdSchema } from './app-game-source-gated-policy-preview-timer-audit-rollback-parent-surface-intent';
import { AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelRowIdSchema } from './app-game-source-gated-policy-preview-timer-audit-rollback-read-model';
import { AppGameSourceGatedPolicyPreviewTimerProofRefSchema } from './app-game-source-gated-policy-preview-timer-status';
import { AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceRowIdSchema } from './app-game-source-gated-policy-preview-timer-scheduler-persistence';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffIdSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffReadApiRefSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffRowIdSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffSchema,
  type AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffRow,
} from './app-game-source-gated-policy-preview-timer-service-readiness-handoff';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelNoClaimFlags,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelState,
  RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelNonClaims,
  type AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelStateValue,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelCountsMatch,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelHasNoRuntimeClaims,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelMatchesHandoff,
} from './app-game-source-gated-policy-preview-timer-service-readiness-read-model-rules';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';

const ServiceReadinessReadModelText = Schema.String.pipe(Schema.minLength(1));

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelIdSchema = ServiceReadinessReadModelText.pipe(
  Schema.brand('AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelId')
);
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelRowIdSchema =
  ServiceReadinessReadModelText.pipe(
    Schema.brand('AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelRowId')
  );
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelContractRefSchema =
  ServiceReadinessReadModelText.pipe(
    Schema.brand('AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelContractRef')
  );
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelSummaryRefSchema =
  ServiceReadinessReadModelText.pipe(
    Schema.brand('AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelSummaryRef')
  );

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelStateSchema = withParser(
  Schema.Literal(...Object.values(AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelState))
);
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelNonClaimSchema = withParser(
  Schema.Literal(...RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelNonClaims)
);

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelOptionsSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    readModelId: AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceContractRefs: Schema.Array(AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelContractRefSchema),
    serviceReadinessSummaryRef: AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelSummaryRefSchema,
  }).pipe(
    Schema.filter(
      (options) =>
        options.sourceContractRefs.length > 0 ||
        'Expected source-gated policy preview timer service-readiness read model options to cite source contracts'
    )
  )
);

const AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  rowId: AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelRowIdSchema,
  sourceServiceReadinessHandoffRowId: AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffRowIdSchema,
  sourceParentSurfaceIntentRowId: AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentRowIdSchema,
  sourceAuditRollbackReadModelRowId: AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelRowIdSchema,
  sourceAuditRollbackHandoffRowId: AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffRowIdSchema,
  sourceSchedulerPersistenceRowId: AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceRowIdSchema,
  targetDomain: AppGamePolicyPreviewTargetDomainSchema,
  serviceReadinessReadModelState: AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelStateSchema,
  parentSurfaceProofRequired: Schema.Boolean,
  serviceReadinessProofRequired: Schema.Boolean,
  serviceReadApiProofRequired: Schema.Boolean,
  requiredProofRefs: Schema.Array(AppGameSourceGatedPolicyPreviewTimerProofRefSchema),
  sourceEvidenceRefs: Schema.Array(AppGameSourceFreshnessEvidenceRefSchema),
  serviceReadApiRef: AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffReadApiRefSchema,
  serviceReadinessSummaryRef: AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelSummaryRefSchema,
  serviceRuntimeEventClaimed: Schema.Literal(false),
  serviceReadApiImplemented: Schema.Literal(false),
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

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelRowSchema = withParser(
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        row.requiredProofRefs.length > 0 ||
        'Expected source-gated policy preview timer service-readiness read-model rows to name required proof refs'
    )
  )
);

const AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  readModelId: AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelIdSchema,
  sourceServiceReadinessHandoffId: AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceContractRefs: Schema.Array(AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelContractRefSchema),
  serviceReadinessSummaryRef: AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelSummaryRefSchema,
  rows: Schema.Array(AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelRowSchema),
  nativeAppRowCount: Schema.Number,
  nativeGameRowCount: Schema.Number,
  serviceReadModelProofRequiredCount: Schema.Number,
  blockedBySourceFreshnessCount: Schema.Number,
  blockedByCompilerDecisionCount: Schema.Number,
  serviceReadinessReadModelNonClaims: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelNonClaimSchema
  ),
  serviceRuntimeEventClaimed: Schema.Literal(false),
  serviceReadApiImplemented: Schema.Literal(false),
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

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelSchema = withParser(
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        appGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelCountsMatch(readModel) ||
        'Expected source-gated policy preview timer service-readiness read model counts to match row states'
    )
  ).pipe(
    Schema.filter(
      (readModel) =>
        appGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelHasNoRuntimeClaims(readModel) ||
        'Expected source-gated policy preview timer service-readiness read model to avoid service, UI, timer, scheduler, audit, rollback, adapter, and raw-source claims'
    )
  )
);

export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelOptions = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelOptionsSchema
>;
export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelRow = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelRowSchema
>;
export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModel = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelSchema
>;

export function buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModel(
  optionsInput: unknown,
  handoffInput: unknown
): AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModel {
  const options = AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelOptionsSchema.parse(optionsInput);
  const handoff = AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffSchema.parse(handoffInput);
  const rows = handoff.rows.map((row) => buildServiceReadinessReadModelRow(options, row));

  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelSchema.parse({
    schemaVersion: options.schemaVersion,
    readModelId: options.readModelId,
    sourceServiceReadinessHandoffId: handoff.handoffId,
    generatedAt: options.generatedAt,
    sourceContractRefs: options.sourceContractRefs,
    serviceReadinessSummaryRef: options.serviceReadinessSummaryRef,
    rows,
    nativeAppRowCount: handoff.nativeAppRowCount,
    nativeGameRowCount: handoff.nativeGameRowCount,
    serviceReadModelProofRequiredCount: rows.filter(
      (row) =>
        row.serviceReadinessReadModelState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelState.ServiceReadModelProofRequired
    ).length,
    blockedBySourceFreshnessCount: rows.filter(
      (row) =>
        row.serviceReadinessReadModelState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelState.BlockedBySourceFreshness
    ).length,
    blockedByCompilerDecisionCount: rows.filter(
      (row) =>
        row.serviceReadinessReadModelState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelState.BlockedByCompilerDecision
    ).length,
    serviceReadinessReadModelNonClaims: RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelNonClaims,
    ...AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelNoClaimFlags,
  });
}

function buildServiceReadinessReadModelRow(
  options: AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelOptions,
  handoffRow: AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffRow
): AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelRow {
  const serviceReadinessReadModelState = readModelStateForHandoff(handoffRow);

  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelRowSchema.parse({
    schemaVersion: options.schemaVersion,
    rowId: `${handoffRow.rowId}:service-readiness-read-model`,
    sourceServiceReadinessHandoffRowId: handoffRow.rowId,
    sourceParentSurfaceIntentRowId: handoffRow.sourceParentSurfaceIntentRowId,
    sourceAuditRollbackReadModelRowId: handoffRow.sourceAuditRollbackReadModelRowId,
    sourceAuditRollbackHandoffRowId: handoffRow.sourceAuditRollbackHandoffRowId,
    sourceSchedulerPersistenceRowId: handoffRow.sourceSchedulerPersistenceRowId,
    targetDomain: handoffRow.targetDomain,
    serviceReadinessReadModelState,
    parentSurfaceProofRequired: handoffRow.parentSurfaceProofRequired,
    serviceReadinessProofRequired: handoffRow.serviceReadinessProofRequired,
    serviceReadApiProofRequired: handoffRow.serviceReadApiProofRequired,
    requiredProofRefs: handoffRow.requiredProofRefs,
    sourceEvidenceRefs: handoffRow.sourceEvidenceRefs,
    serviceReadApiRef: handoffRow.serviceReadApiRef,
    serviceReadinessSummaryRef: options.serviceReadinessSummaryRef,
    ...AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelNoClaimFlags,
    generatedAt: options.generatedAt,
  });
}

function readModelStateForHandoff(
  handoffRow: AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffRow
): AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelStateValue {
  for (const state of Object.values(AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelState)) {
    if (
      appGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelMatchesHandoff(
        handoffRow.serviceReadinessHandoffState,
        state
      )
    ) {
      return state;
    }
  }
  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelState.BlockedByCompilerDecision;
}

export const decodeAppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModel = Schema.decodeUnknownSync(
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelSchema
);

export { AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelState };

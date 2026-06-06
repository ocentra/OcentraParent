import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentIdSchema,
  AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentRowIdSchema,
  AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentSchema,
  type AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentRow,
} from './app-game-source-gated-policy-preview-timer-audit-rollback-parent-surface-intent';
import { AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffRowIdSchema } from './app-game-source-gated-policy-preview-timer-audit-rollback-handoff';
import { AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelRowIdSchema } from './app-game-source-gated-policy-preview-timer-audit-rollback-read-model';
import { AppGameSourceGatedPolicyPreviewTimerProofRefSchema } from './app-game-source-gated-policy-preview-timer-status';
import { AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceRowIdSchema } from './app-game-source-gated-policy-preview-timer-scheduler-persistence';
import { AppGamePolicyPreviewTargetDomainSchema } from './app-game-policy-preview-handoff';
import { AppGameSourceFreshnessEvidenceRefSchema } from './app-game-source-freshness-policy-consumption';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffNoClaimFlags,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffState,
  RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffNonClaims,
  type AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffStateValue,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffCountsMatch,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffHasNoRuntimeClaims,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffMatchesParentSurfaceIntent,
} from './app-game-source-gated-policy-preview-timer-service-readiness-handoff-rules';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './reference-primitives';

const ServiceReadinessHandoffText = Schema.String.pipe(Schema.minLength(1));

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffIdSchema = ServiceReadinessHandoffText.pipe(
  Schema.brand('AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffId')
);
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffRowIdSchema = ServiceReadinessHandoffText.pipe(
  Schema.brand('AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffRowId')
);
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffContractRefSchema =
  ServiceReadinessHandoffText.pipe(
    Schema.brand('AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffContractRef')
  );
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffReadApiRefSchema =
  ServiceReadinessHandoffText.pipe(
    Schema.brand('AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffReadApiRef')
  );

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffStateSchema = withParser(
  Schema.Literal(...Object.values(AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffState))
);
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffNonClaimSchema = withParser(
  Schema.Literal(...RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffNonClaims)
);

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffOptionsSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    handoffId: AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceContractRefs: Schema.Array(AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffContractRefSchema),
    serviceReadinessProofRef: AppGameSourceGatedPolicyPreviewTimerProofRefSchema,
    serviceReadApiProofRef: AppGameSourceGatedPolicyPreviewTimerProofRefSchema,
    serviceReadApiRef: AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffReadApiRefSchema,
  }).pipe(
    Schema.filter(
      (options) =>
        options.sourceContractRefs.length > 0 ||
        'Expected source-gated policy preview timer service-readiness handoff options to cite source contracts'
    )
  )
);

const AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  rowId: AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffRowIdSchema,
  sourceParentSurfaceIntentRowId: AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentRowIdSchema,
  sourceAuditRollbackReadModelRowId: AppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModelRowIdSchema,
  sourceAuditRollbackHandoffRowId: AppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoffRowIdSchema,
  sourceSchedulerPersistenceRowId: AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceRowIdSchema,
  targetDomain: AppGamePolicyPreviewTargetDomainSchema,
  serviceReadinessHandoffState: AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffStateSchema,
  parentSurfaceProofRequired: Schema.Boolean,
  serviceReadinessProofRequired: Schema.Boolean,
  serviceReadApiProofRequired: Schema.Boolean,
  requiredProofRefs: Schema.Array(AppGameSourceGatedPolicyPreviewTimerProofRefSchema),
  sourceEvidenceRefs: Schema.Array(AppGameSourceFreshnessEvidenceRefSchema),
  serviceReadApiRef: AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffReadApiRefSchema,
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

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffRowSchema = withParser(
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        row.requiredProofRefs.length > 0 ||
        'Expected source-gated policy preview timer service-readiness handoff rows to name required proof refs'
    )
  )
);

const AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  handoffId: AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffIdSchema,
  sourceParentSurfaceIntentId: AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceContractRefs: Schema.Array(AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffContractRefSchema),
  rows: Schema.Array(AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffRowSchema),
  nativeAppRowCount: Schema.Number,
  nativeGameRowCount: Schema.Number,
  serviceReadApiProofRequiredCount: Schema.Number,
  blockedBySourceFreshnessCount: Schema.Number,
  blockedByCompilerDecisionCount: Schema.Number,
  serviceReadinessHandoffNonClaims: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffNonClaimSchema
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

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffSchema = withParser(
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffBaseSchema.pipe(
    Schema.filter(
      (handoff) =>
        appGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffCountsMatch(handoff) ||
        'Expected source-gated policy preview timer service-readiness handoff counts to match row states'
    )
  ).pipe(
    Schema.filter(
      (handoff) =>
        appGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffHasNoRuntimeClaims(handoff) ||
        'Expected source-gated policy preview timer service-readiness handoff to avoid service, UI, timer, scheduler, audit, rollback, adapter, and raw-source claims'
    )
  )
);

export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffOptions = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffOptionsSchema
>;
export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffRow = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffRowSchema
>;
export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoff = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffSchema
>;

export function buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoff(
  optionsInput: unknown,
  parentSurfaceIntentInput: unknown
): AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoff {
  const options = AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffOptionsSchema.parse(optionsInput);
  const intent =
    AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentSchema.parse(parentSurfaceIntentInput);
  const rows = intent.rows.map((row) => buildServiceReadinessRow(options, row));

  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffSchema.parse({
    schemaVersion: options.schemaVersion,
    handoffId: options.handoffId,
    sourceParentSurfaceIntentId: intent.intentId,
    generatedAt: options.generatedAt,
    sourceContractRefs: options.sourceContractRefs,
    rows,
    nativeAppRowCount: intent.nativeAppRowCount,
    nativeGameRowCount: intent.nativeGameRowCount,
    serviceReadApiProofRequiredCount: rows.filter(
      (row) =>
        row.serviceReadinessHandoffState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffState.ServiceReadApiProofRequired
    ).length,
    blockedBySourceFreshnessCount: rows.filter(
      (row) =>
        row.serviceReadinessHandoffState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffState.BlockedBySourceFreshness
    ).length,
    blockedByCompilerDecisionCount: rows.filter(
      (row) =>
        row.serviceReadinessHandoffState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffState.BlockedByCompilerDecision
    ).length,
    serviceReadinessHandoffNonClaims: RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffNonClaims,
    ...AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffNoClaimFlags,
  });
}

function buildServiceReadinessRow(
  options: AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffOptions,
  intentRow: AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentRow
): AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffRow {
  const serviceReadinessHandoffState = serviceReadinessStateForParentSurfaceIntent(intentRow);
  const serviceReadApiProofRequired =
    serviceReadinessHandoffState ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffState.ServiceReadApiProofRequired;

  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffRowSchema.parse({
    schemaVersion: options.schemaVersion,
    rowId: `${intentRow.rowId}:service-readiness-handoff`,
    sourceParentSurfaceIntentRowId: intentRow.rowId,
    sourceAuditRollbackReadModelRowId: intentRow.sourceAuditRollbackReadModelRowId,
    sourceAuditRollbackHandoffRowId: intentRow.sourceAuditRollbackHandoffRowId,
    sourceSchedulerPersistenceRowId: intentRow.sourceSchedulerPersistenceRowId,
    targetDomain: intentRow.targetDomain,
    serviceReadinessHandoffState,
    parentSurfaceProofRequired: intentRow.parentSurfaceProofRequired,
    serviceReadinessProofRequired: serviceReadApiProofRequired,
    serviceReadApiProofRequired,
    requiredProofRefs: serviceReadApiProofRequired
      ? [...intentRow.requiredProofRefs, options.serviceReadinessProofRef, options.serviceReadApiProofRef]
      : intentRow.requiredProofRefs,
    sourceEvidenceRefs: intentRow.sourceEvidenceRefs,
    serviceReadApiRef: options.serviceReadApiRef,
    ...AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffNoClaimFlags,
    generatedAt: options.generatedAt,
  });
}

function serviceReadinessStateForParentSurfaceIntent(
  intentRow: AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentRow
): AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffStateValue {
  for (const state of Object.values(AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffState)) {
    if (
      appGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffMatchesParentSurfaceIntent(
        intentRow.parentSurfaceIntentState,
        state
      )
    ) {
      return state;
    }
  }
  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffState.BlockedByCompilerDecision;
}

export const decodeAppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoff = Schema.decodeUnknownSync(
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffSchema
);

export { AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffState };

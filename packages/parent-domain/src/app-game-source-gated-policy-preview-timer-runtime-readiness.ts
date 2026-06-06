import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AppGamePolicyPreviewTargetDomainSchema } from './app-game-policy-preview-handoff';
import { AppGameSourceFreshnessEvidenceRefSchema } from './app-game-source-freshness-policy-consumption';
import {
  AppGameSourceGatedPolicyPreviewTimerProofRefSchema,
  AppGameSourceGatedPolicyPreviewTimerStatusIdSchema,
  AppGameSourceGatedPolicyPreviewTimerStatusRowIdSchema,
  AppGameSourceGatedPolicyPreviewTimerStatusSchema,
  type AppGameSourceGatedPolicyPreviewTimerStatusRow,
} from './app-game-source-gated-policy-preview-timer-status';
import {
  AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessNoClaimFlags,
  AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessState,
  RequiredAppGameSourceGatedPolicyPreviewTimerRuntimeReadinessNonClaims,
  appGameSourceGatedPolicyPreviewTimerRuntimeReadinessCountsMatch,
  appGameSourceGatedPolicyPreviewTimerRuntimeReadinessHasNoRuntimeClaims,
  appGameSourceGatedPolicyPreviewTimerRuntimeReadinessMatchesStatus,
} from './app-game-source-gated-policy-preview-timer-runtime-readiness-rules';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './reference-primitives';

const SourceGatedPolicyPreviewTimerRuntimeReadinessText = Schema.String.pipe(Schema.minLength(1));

export const AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessIdSchema =
  SourceGatedPolicyPreviewTimerRuntimeReadinessText.pipe(
    Schema.brand('AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessId')
  );
export const AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessRowIdSchema =
  SourceGatedPolicyPreviewTimerRuntimeReadinessText.pipe(
    Schema.brand('AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessRowId')
  );
export const AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessContractRefSchema =
  SourceGatedPolicyPreviewTimerRuntimeReadinessText.pipe(
    Schema.brand('AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessContractRef')
  );

export const AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessStateSchema = withParser(
  Schema.Literal(...Object.values(AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessState))
);
export const AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessNonClaimSchema = withParser(
  Schema.Literal(...RequiredAppGameSourceGatedPolicyPreviewTimerRuntimeReadinessNonClaims)
);

export const AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessOptionsSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    readinessId: AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceContractRefs: Schema.Array(AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessContractRefSchema),
    timerRuntimeProofRef: AppGameSourceGatedPolicyPreviewTimerProofRefSchema,
    schedulerPersistenceProofRef: AppGameSourceGatedPolicyPreviewTimerProofRefSchema,
    auditProofRef: AppGameSourceGatedPolicyPreviewTimerProofRefSchema,
    rollbackProofRef: AppGameSourceGatedPolicyPreviewTimerProofRefSchema,
  }).pipe(
    Schema.filter(
      (options) =>
        options.sourceContractRefs.length > 0 ||
        'Expected source-gated policy preview timer runtime readiness options to cite source contracts'
    )
  )
);

const AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  rowId: AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessRowIdSchema,
  sourceTimerStatusRowId: AppGameSourceGatedPolicyPreviewTimerStatusRowIdSchema,
  targetDomain: AppGamePolicyPreviewTargetDomainSchema,
  runtimeReadinessState: AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessStateSchema,
  timerRuntimeProofRequired: Schema.Boolean,
  schedulerPersistenceProofRequired: Schema.Boolean,
  auditProofRequired: Schema.Boolean,
  rollbackProofRequired: Schema.Boolean,
  requiredProofRefs: Schema.Array(AppGameSourceGatedPolicyPreviewTimerProofRefSchema),
  sourceEvidenceRefs: Schema.Array(AppGameSourceFreshnessEvidenceRefSchema),
  serviceRuntimeEventClaimed: Schema.Literal(false),
  portalUiRendered: Schema.Literal(false),
  policyEvaluatorRuntimeClaimed: Schema.Literal(false),
  timerRuntimeClaimed: Schema.Literal(false),
  timerScheduled: Schema.Literal(false),
  schedulerPersistenceClaimed: Schema.Literal(false),
  auditRuntimeClaimed: Schema.Literal(false),
  rollbackRuntimeClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  childDeliveryClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  rawPrivateSourceRowsIncluded: Schema.Literal(false),
  generatedAt: ParentTimestampSchema,
});

export const AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessRowSchema = withParser(
  AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        row.requiredProofRefs.length > 0 ||
        'Expected source-gated policy preview timer runtime readiness rows to name required proof refs'
    )
  )
);

const AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  readinessId: AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessIdSchema,
  sourceTimerStatusId: AppGameSourceGatedPolicyPreviewTimerStatusIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceContractRefs: Schema.Array(AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessContractRefSchema),
  rows: Schema.Array(AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessRowSchema),
  nativeAppRowCount: Schema.Number,
  nativeGameRowCount: Schema.Number,
  runtimeProofRequiredCount: Schema.Number,
  blockedBySourceFreshnessCount: Schema.Number,
  blockedByCompilerDecisionCount: Schema.Number,
  runtimeReadinessNonClaims: Schema.Array(AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessNonClaimSchema),
  serviceRuntimeEventClaimed: Schema.Literal(false),
  portalUiRendered: Schema.Literal(false),
  policyEvaluatorRuntimeClaimed: Schema.Literal(false),
  timerRuntimeClaimed: Schema.Literal(false),
  timerScheduled: Schema.Literal(false),
  schedulerPersistenceClaimed: Schema.Literal(false),
  auditRuntimeClaimed: Schema.Literal(false),
  rollbackRuntimeClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  childDeliveryClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  rawPrivateSourceRowsIncluded: Schema.Literal(false),
});

export const AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessSchema = withParser(
  AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessBaseSchema.pipe(
    Schema.filter(
      (readiness) =>
        appGameSourceGatedPolicyPreviewTimerRuntimeReadinessCountsMatch(readiness) ||
        'Expected source-gated policy preview timer runtime readiness counts to match row states'
    )
  ).pipe(
    Schema.filter(
      (readiness) =>
        appGameSourceGatedPolicyPreviewTimerRuntimeReadinessHasNoRuntimeClaims(readiness) ||
        'Expected source-gated policy preview timer runtime readiness to avoid runtime, UI, timer, adapter, and raw-source claims'
    )
  )
);

export type AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessOptions = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessOptionsSchema
>;
export type AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessRow = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessRowSchema
>;
export type AppGameSourceGatedPolicyPreviewTimerRuntimeReadiness = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessSchema
>;

export function buildAppGameSourceGatedPolicyPreviewTimerRuntimeReadiness(
  optionsInput: unknown,
  timerStatusInput: unknown
): AppGameSourceGatedPolicyPreviewTimerRuntimeReadiness {
  const options = AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessOptionsSchema.parse(optionsInput);
  const timerStatus = AppGameSourceGatedPolicyPreviewTimerStatusSchema.parse(timerStatusInput);
  const rows = timerStatus.rows.map((row) => buildRuntimeReadinessRow(options, row));

  return AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessSchema.parse({
    schemaVersion: options.schemaVersion,
    readinessId: options.readinessId,
    sourceTimerStatusId: timerStatus.statusId,
    generatedAt: options.generatedAt,
    sourceContractRefs: options.sourceContractRefs,
    rows,
    nativeAppRowCount: timerStatus.nativeAppRowCount,
    nativeGameRowCount: timerStatus.nativeGameRowCount,
    runtimeProofRequiredCount: rows.filter(
      (row) =>
        row.runtimeReadinessState === AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessState.RuntimeProofRequired
    ).length,
    blockedBySourceFreshnessCount: rows.filter(
      (row) =>
        row.runtimeReadinessState === AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessState.BlockedBySourceFreshness
    ).length,
    blockedByCompilerDecisionCount: rows.filter(
      (row) =>
        row.runtimeReadinessState ===
        AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessState.BlockedByCompilerDecision
    ).length,
    runtimeReadinessNonClaims: RequiredAppGameSourceGatedPolicyPreviewTimerRuntimeReadinessNonClaims,
    ...AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessNoClaimFlags,
  });
}

function buildRuntimeReadinessRow(
  options: AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessOptions,
  timerStatusRow: AppGameSourceGatedPolicyPreviewTimerStatusRow
): AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessRow {
  const runtimeReadinessState = runtimeReadinessStateForTimerStatus(timerStatusRow);
  const runtimeProofRequired =
    runtimeReadinessState === AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessState.RuntimeProofRequired;

  return AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessRowSchema.parse({
    schemaVersion: options.schemaVersion,
    rowId: `${timerStatusRow.rowId}:runtime-readiness`,
    sourceTimerStatusRowId: timerStatusRow.rowId,
    targetDomain: timerStatusRow.targetDomain,
    runtimeReadinessState,
    timerRuntimeProofRequired: runtimeProofRequired,
    schedulerPersistenceProofRequired: runtimeProofRequired,
    auditProofRequired: runtimeProofRequired,
    rollbackProofRequired: runtimeProofRequired,
    requiredProofRefs: requiredProofRefsForReadiness(options, runtimeReadinessState, timerStatusRow),
    sourceEvidenceRefs: timerStatusRow.sourceEvidenceRefs,
    ...AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessNoClaimFlags,
    generatedAt: options.generatedAt,
  });
}

function runtimeReadinessStateForTimerStatus(timerStatusRow: AppGameSourceGatedPolicyPreviewTimerStatusRow) {
  for (const state of Object.values(AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessState)) {
    if (appGameSourceGatedPolicyPreviewTimerRuntimeReadinessMatchesStatus(timerStatusRow.timerStatusState, state)) {
      return state;
    }
  }
  return AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessState.BlockedByCompilerDecision;
}

function requiredProofRefsForReadiness(
  options: AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessOptions,
  runtimeReadinessState: string,
  timerStatusRow: AppGameSourceGatedPolicyPreviewTimerStatusRow
) {
  if (runtimeReadinessState === AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessState.RuntimeProofRequired) {
    return [
      options.timerRuntimeProofRef,
      options.schedulerPersistenceProofRef,
      options.auditProofRef,
      options.rollbackProofRef,
    ];
  }
  return timerStatusRow.requiredProofRefs;
}

export const decodeAppGameSourceGatedPolicyPreviewTimerRuntimeReadiness = Schema.decodeUnknownSync(
  AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessSchema
);

export { AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessState };

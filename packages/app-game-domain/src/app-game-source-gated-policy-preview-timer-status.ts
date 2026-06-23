import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { AppGamePolicyPreviewTargetDomainSchema } from '@ocentra-parent/schema-domain/app-game-policy-preview-handoff';
import { AppGameSourceFreshnessEvidenceRefSchema } from '@ocentra-parent/schema-domain/app-game-source-freshness-policy-consumption';
import {
  AppGameSourceGatedPolicyPreviewTimerHandoffIdSchema,
  AppGameSourceGatedPolicyPreviewTimerHandoffRowIdSchema,
  AppGameSourceGatedPolicyPreviewTimerHandoffSchema,
  type AppGameSourceGatedPolicyPreviewTimerHandoffRow,
} from '@ocentra-parent/schema-domain/app-game-source-gated-policy-preview-timer-handoff';
import {
  AppGameSourceGatedPolicyPreviewTimerStatusNoClaimFlags,
  AppGameSourceGatedPolicyPreviewTimerStatusState,
  RequiredAppGameSourceGatedPolicyPreviewTimerStatusNonClaims,
  appGameSourceGatedPolicyPreviewTimerStatusCountsMatch,
  appGameSourceGatedPolicyPreviewTimerStatusHasNoRuntimeClaims,
  appGameSourceGatedPolicyPreviewTimerStatusMatchesHandoff,
} from '@ocentra-parent/schema-domain/app-game-source-gated-policy-preview-timer-status-rules';
import {
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/schema-domain/family-reference-primitives';

const AppGameSourceGatedPolicyPreviewTimerStatusIdSchema = brandedNonEmptyStringSchema(
  'AppGameSourceGatedPolicyPreviewTimerStatusId'
);
const AppGameSourceGatedPolicyPreviewTimerStatusRowIdSchema = brandedNonEmptyStringSchema(
  'AppGameSourceGatedPolicyPreviewTimerStatusRowId'
);
const AppGameSourceGatedPolicyPreviewTimerStatusContractRefSchema = brandedNonEmptyStringSchema(
  'AppGameSourceGatedPolicyPreviewTimerStatusContractRef'
);
export const AppGameSourceGatedPolicyPreviewTimerProofRefSchema = brandedNonEmptyStringSchema(
  'AppGameSourceGatedPolicyPreviewTimerProofRef'
);

const AppGameSourceGatedPolicyPreviewTimerStatusStateSchema = withParser(
  Schema.Literal(...Object.values(AppGameSourceGatedPolicyPreviewTimerStatusState))
);
const AppGameSourceGatedPolicyPreviewTimerStatusNonClaimSchema = withParser(
  Schema.Literal(...RequiredAppGameSourceGatedPolicyPreviewTimerStatusNonClaims)
);

const AppGameSourceGatedPolicyPreviewTimerStatusOptionsSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    statusId: AppGameSourceGatedPolicyPreviewTimerStatusIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceContractRefs: Schema.Array(AppGameSourceGatedPolicyPreviewTimerStatusContractRefSchema),
    timerRuntimeProofRef: AppGameSourceGatedPolicyPreviewTimerProofRefSchema,
    sourceFreshnessProofRef: AppGameSourceGatedPolicyPreviewTimerProofRefSchema,
    compilerDecisionProofRef: AppGameSourceGatedPolicyPreviewTimerProofRefSchema,
  }).pipe(
    Schema.filter(
      (options) =>
        options.sourceContractRefs.length > 0 ||
        'Expected source-gated policy preview timer status options to cite source contracts'
    )
  )
);

const AppGameSourceGatedPolicyPreviewTimerStatusRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  rowId: AppGameSourceGatedPolicyPreviewTimerStatusRowIdSchema,
  sourceTimerHandoffRowId: AppGameSourceGatedPolicyPreviewTimerHandoffRowIdSchema,
  targetDomain: AppGamePolicyPreviewTargetDomainSchema,
  timerStatusState: AppGameSourceGatedPolicyPreviewTimerStatusStateSchema,
  timerRuntimeProofRequired: Schema.Boolean,
  sourceFreshnessProofRequired: Schema.Boolean,
  compilerDecisionProofRequired: Schema.Boolean,
  requiredProofRefs: Schema.Array(AppGameSourceGatedPolicyPreviewTimerProofRefSchema),
  sourceEvidenceRefs: Schema.Array(AppGameSourceFreshnessEvidenceRefSchema),
  serviceRuntimeEventClaimed: Schema.Literal(false),
  portalUiRendered: Schema.Literal(false),
  policyEvaluatorRuntimeClaimed: Schema.Literal(false),
  timerRuntimeClaimed: Schema.Literal(false),
  timerScheduled: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  childDeliveryClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  rawPrivateSourceRowsIncluded: Schema.Literal(false),
  generatedAt: ParentTimestampSchema,
});

const AppGameSourceGatedPolicyPreviewTimerStatusRowSchema = withParser(
  AppGameSourceGatedPolicyPreviewTimerStatusRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        row.requiredProofRefs.length > 0 ||
        'Expected source-gated policy preview timer status rows to name required proof refs'
    )
  )
);

const AppGameSourceGatedPolicyPreviewTimerStatusBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  statusId: AppGameSourceGatedPolicyPreviewTimerStatusIdSchema,
  sourceTimerHandoffId: AppGameSourceGatedPolicyPreviewTimerHandoffIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceContractRefs: Schema.Array(AppGameSourceGatedPolicyPreviewTimerStatusContractRefSchema),
  rows: Schema.Array(AppGameSourceGatedPolicyPreviewTimerStatusRowSchema),
  nativeAppRowCount: Schema.Number,
  nativeGameRowCount: Schema.Number,
  timerRuntimeProofRequiredCount: Schema.Number,
  sourceFreshnessProofRequiredCount: Schema.Number,
  compilerDecisionProofRequiredCount: Schema.Number,
  timerStatusNonClaims: Schema.Array(AppGameSourceGatedPolicyPreviewTimerStatusNonClaimSchema),
  serviceRuntimeEventClaimed: Schema.Literal(false),
  portalUiRendered: Schema.Literal(false),
  policyEvaluatorRuntimeClaimed: Schema.Literal(false),
  timerRuntimeClaimed: Schema.Literal(false),
  timerScheduled: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  childDeliveryClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  rawPrivateSourceRowsIncluded: Schema.Literal(false),
});

const AppGameSourceGatedPolicyPreviewTimerStatusSchema = withParser(
  AppGameSourceGatedPolicyPreviewTimerStatusBaseSchema.pipe(
    Schema.filter(
      (status) =>
        appGameSourceGatedPolicyPreviewTimerStatusCountsMatch(status) ||
        'Expected source-gated policy preview timer status counts to match required proof rows'
    )
  ).pipe(
    Schema.filter(
      (status) =>
        appGameSourceGatedPolicyPreviewTimerStatusHasNoRuntimeClaims(status) ||
        'Expected source-gated policy preview timer status to avoid runtime, UI, timer, adapter, and raw-source claims'
    )
  )
);

type AppGameSourceGatedPolicyPreviewTimerStatusOptions = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerStatusOptionsSchema
>;
type AppGameSourceGatedPolicyPreviewTimerStatusRow = Infer<typeof AppGameSourceGatedPolicyPreviewTimerStatusRowSchema>;
type AppGameSourceGatedPolicyPreviewTimerStatus = Infer<typeof AppGameSourceGatedPolicyPreviewTimerStatusSchema>;

export function buildAppGameSourceGatedPolicyPreviewTimerStatus(
  optionsInput: unknown,
  timerHandoffInput: unknown
): AppGameSourceGatedPolicyPreviewTimerStatus {
  const options = AppGameSourceGatedPolicyPreviewTimerStatusOptionsSchema.parse(optionsInput);
  const timerHandoff = AppGameSourceGatedPolicyPreviewTimerHandoffSchema.parse(timerHandoffInput);
  const rows = timerHandoff.rows.map((row) => buildTimerStatusRow(options, row));

  return AppGameSourceGatedPolicyPreviewTimerStatusSchema.parse({
    schemaVersion: options.schemaVersion,
    statusId: options.statusId,
    sourceTimerHandoffId: timerHandoff.handoffId,
    generatedAt: options.generatedAt,
    sourceContractRefs: options.sourceContractRefs,
    rows,
    nativeAppRowCount: timerHandoff.nativeAppRowCount,
    nativeGameRowCount: timerHandoff.nativeGameRowCount,
    timerRuntimeProofRequiredCount: rows.filter(
      (row) => row.timerStatusState === AppGameSourceGatedPolicyPreviewTimerStatusState.TimerRuntimeProofRequired
    ).length,
    sourceFreshnessProofRequiredCount: rows.filter(
      (row) => row.timerStatusState === AppGameSourceGatedPolicyPreviewTimerStatusState.SourceFreshnessProofRequired
    ).length,
    compilerDecisionProofRequiredCount: rows.filter(
      (row) => row.timerStatusState === AppGameSourceGatedPolicyPreviewTimerStatusState.CompilerDecisionProofRequired
    ).length,
    timerStatusNonClaims: RequiredAppGameSourceGatedPolicyPreviewTimerStatusNonClaims,
    ...AppGameSourceGatedPolicyPreviewTimerStatusNoClaimFlags,
  });
}

function buildTimerStatusRow(
  options: AppGameSourceGatedPolicyPreviewTimerStatusOptions,
  handoffRow: AppGameSourceGatedPolicyPreviewTimerHandoffRow
): AppGameSourceGatedPolicyPreviewTimerStatusRow {
  const timerStatusState = timerStatusStateForHandoff(handoffRow);

  return AppGameSourceGatedPolicyPreviewTimerStatusRowSchema.parse({
    schemaVersion: options.schemaVersion,
    rowId: `${handoffRow.rowId}:timer-status`,
    sourceTimerHandoffRowId: handoffRow.rowId,
    targetDomain: handoffRow.targetDomain,
    timerStatusState,
    timerRuntimeProofRequired:
      timerStatusState === AppGameSourceGatedPolicyPreviewTimerStatusState.TimerRuntimeProofRequired,
    sourceFreshnessProofRequired:
      timerStatusState === AppGameSourceGatedPolicyPreviewTimerStatusState.SourceFreshnessProofRequired,
    compilerDecisionProofRequired:
      timerStatusState === AppGameSourceGatedPolicyPreviewTimerStatusState.CompilerDecisionProofRequired,
    requiredProofRefs: requiredProofRefsForStatus(options, timerStatusState),
    sourceEvidenceRefs: handoffRow.sourceEvidenceRefs,
    ...AppGameSourceGatedPolicyPreviewTimerStatusNoClaimFlags,
    generatedAt: options.generatedAt,
  });
}

function timerStatusStateForHandoff(handoffRow: AppGameSourceGatedPolicyPreviewTimerHandoffRow) {
  for (const state of Object.values(AppGameSourceGatedPolicyPreviewTimerStatusState)) {
    if (appGameSourceGatedPolicyPreviewTimerStatusMatchesHandoff(handoffRow.timerHandoffState, state)) {
      return state;
    }
  }
  return AppGameSourceGatedPolicyPreviewTimerStatusState.CompilerDecisionProofRequired;
}

function requiredProofRefsForStatus(
  options: AppGameSourceGatedPolicyPreviewTimerStatusOptions,
  timerStatusState: string
) {
  switch (timerStatusState) {
    case AppGameSourceGatedPolicyPreviewTimerStatusState.TimerRuntimeProofRequired:
      return [options.timerRuntimeProofRef];
    case AppGameSourceGatedPolicyPreviewTimerStatusState.SourceFreshnessProofRequired:
      return [options.sourceFreshnessProofRef];
    case AppGameSourceGatedPolicyPreviewTimerStatusState.CompilerDecisionProofRequired:
      return [options.compilerDecisionProofRef];
    default:
      return [options.compilerDecisionProofRef];
  }
}

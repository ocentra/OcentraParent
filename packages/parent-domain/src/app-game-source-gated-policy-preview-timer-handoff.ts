import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  AppGameSourceGatedPolicyPreviewReadModelIdSchema,
  AppGameSourceGatedPolicyPreviewReadModelRowIdSchema,
  AppGameSourceGatedPolicyPreviewReadModelSchema,
  type AppGameSourceGatedPolicyPreviewReadModelRow,
} from './app-game-source-gated-policy-preview-read-model';
import {
  AppGameSourceGatedPolicyPreviewTimerHandoffNoClaimFlags,
  AppGameSourceGatedPolicyPreviewTimerHandoffState,
  RequiredAppGameSourceGatedPolicyPreviewTimerHandoffNonClaims,
  appGameSourceGatedPolicyPreviewTimerHandoffCountsMatch,
  appGameSourceGatedPolicyPreviewTimerHandoffHasNoRuntimeClaims,
  appGameSourceGatedPolicyPreviewTimerStateMatchesProjection,
} from './app-game-source-gated-policy-preview-timer-handoff-rules';
import { AppGameSourceFreshnessEvidenceRefSchema } from './app-game-source-freshness-policy-consumption';
import {
  AppGameSourceGatedPolicyPreviewDecisionRefSchema,
  AppGameSourceGatedPolicyPreviewReadModelContractRefSchema,
} from './app-game-source-gated-policy-preview-read-model';
import { AppGamePolicyPreviewTargetDomainSchema } from './app-game-policy-preview-handoff';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './reference-primitives';

const SourceGatedPolicyPreviewTimerText = Schema.String.pipe(Schema.minLength(1));

export const AppGameSourceGatedPolicyPreviewTimerHandoffIdSchema = SourceGatedPolicyPreviewTimerText.pipe(
  Schema.brand('AppGameSourceGatedPolicyPreviewTimerHandoffId')
);
export const AppGameSourceGatedPolicyPreviewTimerHandoffRowIdSchema = SourceGatedPolicyPreviewTimerText.pipe(
  Schema.brand('AppGameSourceGatedPolicyPreviewTimerHandoffRowId')
);

export const AppGameSourceGatedPolicyPreviewTimerHandoffStateSchema = withParser(
  Schema.Literal(...Object.values(AppGameSourceGatedPolicyPreviewTimerHandoffState))
);
export const AppGameSourceGatedPolicyPreviewTimerHandoffNonClaimSchema = withParser(
  Schema.Literal(...RequiredAppGameSourceGatedPolicyPreviewTimerHandoffNonClaims)
);

export const AppGameSourceGatedPolicyPreviewTimerHandoffOptionsSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    handoffId: AppGameSourceGatedPolicyPreviewTimerHandoffIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceContractRefs: Schema.Array(AppGameSourceGatedPolicyPreviewReadModelContractRefSchema),
  }).pipe(
    Schema.filter(
      (options) =>
        options.sourceContractRefs.length > 0 ||
        'Expected source-gated policy preview timer handoff options to cite source contracts'
    )
  )
);

const AppGameSourceGatedPolicyPreviewTimerHandoffRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  rowId: AppGameSourceGatedPolicyPreviewTimerHandoffRowIdSchema,
  sourceReadModelRowId: AppGameSourceGatedPolicyPreviewReadModelRowIdSchema,
  targetDomain: AppGamePolicyPreviewTargetDomainSchema,
  timerHandoffState: AppGameSourceGatedPolicyPreviewTimerHandoffStateSchema,
  timerRuntimeRequired: Schema.Boolean,
  manualProofRequired: Schema.Boolean,
  sourceEvidenceRefs: Schema.Array(AppGameSourceFreshnessEvidenceRefSchema),
  previewDecisionRef: Schema.Union(AppGameSourceGatedPolicyPreviewDecisionRefSchema, Schema.Null),
  serviceRuntimeEventClaimed: Schema.Literal(false),
  portalUiRendered: Schema.Literal(false),
  policyEvaluatorRuntimeClaimed: Schema.Literal(false),
  timerRuntimeClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  childDeliveryClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  rawPrivateSourceRowsIncluded: Schema.Literal(false),
  generatedAt: ParentTimestampSchema,
});

export const AppGameSourceGatedPolicyPreviewTimerHandoffRowSchema = withParser(
  AppGameSourceGatedPolicyPreviewTimerHandoffRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        (row.timerHandoffState === AppGameSourceGatedPolicyPreviewTimerHandoffState.ReadyForTimerSequencing &&
          row.timerRuntimeRequired &&
          !row.manualProofRequired) ||
        (row.timerHandoffState !== AppGameSourceGatedPolicyPreviewTimerHandoffState.ReadyForTimerSequencing &&
          !row.timerRuntimeRequired &&
          row.manualProofRequired) ||
        'Expected timer handoff rows to require future timer runtime only for preview-ready rows'
    )
  )
);

const AppGameSourceGatedPolicyPreviewTimerHandoffBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  handoffId: AppGameSourceGatedPolicyPreviewTimerHandoffIdSchema,
  sourceReadModelId: AppGameSourceGatedPolicyPreviewReadModelIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceContractRefs: Schema.Array(AppGameSourceGatedPolicyPreviewReadModelContractRefSchema),
  rows: Schema.Array(AppGameSourceGatedPolicyPreviewTimerHandoffRowSchema),
  nativeAppRowCount: Schema.Number,
  nativeGameRowCount: Schema.Number,
  timerSequenceCandidateCount: Schema.Number,
  sourceManualBlockedCount: Schema.Number,
  compilerManualBlockedCount: Schema.Number,
  timerHandoffNonClaims: Schema.Array(AppGameSourceGatedPolicyPreviewTimerHandoffNonClaimSchema),
  serviceRuntimeEventClaimed: Schema.Literal(false),
  portalUiRendered: Schema.Literal(false),
  policyEvaluatorRuntimeClaimed: Schema.Literal(false),
  timerRuntimeClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  childDeliveryClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  rawPrivateSourceRowsIncluded: Schema.Literal(false),
});

export const AppGameSourceGatedPolicyPreviewTimerHandoffSchema = withParser(
  AppGameSourceGatedPolicyPreviewTimerHandoffBaseSchema.pipe(
    Schema.filter(
      (handoff) =>
        appGameSourceGatedPolicyPreviewTimerHandoffCountsMatch(handoff) ||
        'Expected source-gated policy preview timer handoff counts to match timer-ready and manual-blocked rows'
    )
  ).pipe(
    Schema.filter(
      (handoff) =>
        appGameSourceGatedPolicyPreviewTimerHandoffHasNoRuntimeClaims(handoff) ||
        'Expected source-gated policy preview timer handoff to avoid runtime, UI, adapter, and raw-source claims'
    )
  )
);

export type AppGameSourceGatedPolicyPreviewTimerHandoffOptions = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerHandoffOptionsSchema
>;
export type AppGameSourceGatedPolicyPreviewTimerHandoffRow = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerHandoffRowSchema
>;
export type AppGameSourceGatedPolicyPreviewTimerHandoff = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerHandoffSchema
>;

export function buildAppGameSourceGatedPolicyPreviewTimerHandoff(
  optionsInput: unknown,
  readModelInput: unknown
): AppGameSourceGatedPolicyPreviewTimerHandoff {
  const options = AppGameSourceGatedPolicyPreviewTimerHandoffOptionsSchema.parse(optionsInput);
  const readModel = AppGameSourceGatedPolicyPreviewReadModelSchema.parse(readModelInput);
  const rows = readModel.rows.map((row) => buildTimerHandoffRow(options, row));

  return AppGameSourceGatedPolicyPreviewTimerHandoffSchema.parse({
    schemaVersion: options.schemaVersion,
    handoffId: options.handoffId,
    sourceReadModelId: readModel.readModelId,
    generatedAt: options.generatedAt,
    sourceContractRefs: options.sourceContractRefs,
    rows,
    nativeAppRowCount: readModel.nativeAppRowCount,
    nativeGameRowCount: readModel.nativeGameRowCount,
    timerSequenceCandidateCount: rows.filter(
      (row) => row.timerHandoffState === AppGameSourceGatedPolicyPreviewTimerHandoffState.ReadyForTimerSequencing
    ).length,
    sourceManualBlockedCount: rows.filter(
      (row) =>
        row.timerHandoffState === AppGameSourceGatedPolicyPreviewTimerHandoffState.SourceManualRequiredBeforeTimer
    ).length,
    compilerManualBlockedCount: rows.filter(
      (row) =>
        row.timerHandoffState === AppGameSourceGatedPolicyPreviewTimerHandoffState.CompilerManualRequiredBeforeTimer
    ).length,
    timerHandoffNonClaims: RequiredAppGameSourceGatedPolicyPreviewTimerHandoffNonClaims,
    ...AppGameSourceGatedPolicyPreviewTimerHandoffNoClaimFlags,
  });
}

function buildTimerHandoffRow(
  options: AppGameSourceGatedPolicyPreviewTimerHandoffOptions,
  sourceRow: AppGameSourceGatedPolicyPreviewReadModelRow
): AppGameSourceGatedPolicyPreviewTimerHandoffRow {
  const timerHandoffState = timerHandoffStateForProjection(sourceRow);

  return AppGameSourceGatedPolicyPreviewTimerHandoffRowSchema.parse({
    schemaVersion: options.schemaVersion,
    rowId: `${sourceRow.rowId}:timer-handoff`,
    sourceReadModelRowId: sourceRow.rowId,
    targetDomain: sourceRow.targetDomain,
    timerHandoffState,
    timerRuntimeRequired:
      timerHandoffState === AppGameSourceGatedPolicyPreviewTimerHandoffState.ReadyForTimerSequencing,
    manualProofRequired: timerHandoffState !== AppGameSourceGatedPolicyPreviewTimerHandoffState.ReadyForTimerSequencing,
    sourceEvidenceRefs: sourceRow.sourceEvidenceRefs,
    previewDecisionRef: sourceRow.previewDecisionRef,
    ...AppGameSourceGatedPolicyPreviewTimerHandoffNoClaimFlags,
    generatedAt: options.generatedAt,
  });
}

function timerHandoffStateForProjection(sourceRow: AppGameSourceGatedPolicyPreviewReadModelRow) {
  for (const state of Object.values(AppGameSourceGatedPolicyPreviewTimerHandoffState)) {
    if (appGameSourceGatedPolicyPreviewTimerStateMatchesProjection(sourceRow.projectionState, state)) {
      return state;
    }
  }
  return AppGameSourceGatedPolicyPreviewTimerHandoffState.CompilerManualRequiredBeforeTimer;
}

export const decodeAppGameSourceGatedPolicyPreviewTimerHandoff = Schema.decodeUnknownSync(
  AppGameSourceGatedPolicyPreviewTimerHandoffSchema
);

export { AppGameSourceGatedPolicyPreviewTimerHandoffState };

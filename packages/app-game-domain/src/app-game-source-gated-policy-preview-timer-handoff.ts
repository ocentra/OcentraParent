import {
  AppGameSourceGatedPolicyPreviewReadModelSchema,
  type AppGameSourceGatedPolicyPreviewReadModelRow,
} from '@ocentra-parent/schema-domain/app-game-source-gated-policy-preview-read-model';
import {
  AppGameSourceGatedPolicyPreviewTimerHandoffOptionsSchema,
  AppGameSourceGatedPolicyPreviewTimerHandoffRowSchema,
  AppGameSourceGatedPolicyPreviewTimerHandoffSchema,
  type AppGameSourceGatedPolicyPreviewTimerHandoff,
  type AppGameSourceGatedPolicyPreviewTimerHandoffOptions,
  type AppGameSourceGatedPolicyPreviewTimerHandoffRow,
} from '@ocentra-parent/schema-domain/app-game-source-gated-policy-preview-timer-handoff';
import {
  AppGameSourceGatedPolicyPreviewTimerHandoffNoClaimFlags,
  AppGameSourceGatedPolicyPreviewTimerHandoffState,
  RequiredAppGameSourceGatedPolicyPreviewTimerHandoffNonClaims,
  appGameSourceGatedPolicyPreviewTimerStateMatchesProjection,
} from '@ocentra-parent/schema-domain/app-game-source-gated-policy-preview-timer-handoff-rules';

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

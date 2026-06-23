import {
  AppGameSourceGatedPolicyPreviewReadModelOptionsSchema as SchemaDomainAppGameSourceGatedPolicyPreviewReadModelOptionsSchema,
  AppGameSourceGatedPolicyPreviewReadModelRowSchema as SchemaDomainAppGameSourceGatedPolicyPreviewReadModelRowSchema,
  AppGameSourceGatedPolicyPreviewReadModelSchema as SchemaDomainAppGameSourceGatedPolicyPreviewReadModelSchema,
  type AppGameSourceGatedPolicyPreviewReadModel as SchemaDomainAppGameSourceGatedPolicyPreviewReadModel,
  type AppGameSourceGatedPolicyPreviewReadModelOptions as SchemaDomainAppGameSourceGatedPolicyPreviewReadModelOptions,
  type AppGameSourceGatedPolicyPreviewReadModelRow as SchemaDomainAppGameSourceGatedPolicyPreviewReadModelRow,
} from '@ocentra-parent/schema-domain/app-game-source-gated-policy-preview-read-model';
import {
  AppGameSourceFreshnessPreviewGateReadModelSchema,
  type AppGameSourceFreshnessPreviewGateReadModel,
  type AppGameSourceFreshnessPreviewGateRow,
} from '@ocentra-parent/schema-domain/app-game-source-freshness-preview-gate';
import {
  AppGameSourceGatedPolicyPreviewReadModelNoClaimFlags,
  AppGameSourceGatedPolicyPreviewReadModelProjectionState,
  AppGameSourceGatedPolicyPreviewReadModelSensitiveBoundary,
  RequiredAppGameSourceGatedPolicyPreviewReadModelNonClaims,
  countAppGameSourceGatedPolicyPreviewReadModelRows,
} from '@ocentra-parent/schema-domain/app-game-source-gated-policy-preview-read-model-rules';
const AppGameSourceGatedPolicyPreviewReadModelSchema = SchemaDomainAppGameSourceGatedPolicyPreviewReadModelSchema;

const AppGameSourceGatedPolicyPreviewReadModelOptionsSchema =
  SchemaDomainAppGameSourceGatedPolicyPreviewReadModelOptionsSchema;
const AppGameSourceGatedPolicyPreviewReadModelRowSchema = SchemaDomainAppGameSourceGatedPolicyPreviewReadModelRowSchema;

type AppGameSourceGatedPolicyPreviewReadModelOptions = SchemaDomainAppGameSourceGatedPolicyPreviewReadModelOptions;
type AppGameSourceGatedPolicyPreviewReadModelRow = SchemaDomainAppGameSourceGatedPolicyPreviewReadModelRow;
type AppGameSourceGatedPolicyPreviewReadModel = SchemaDomainAppGameSourceGatedPolicyPreviewReadModel;

export function buildAppGameSourceGatedPolicyPreviewReadModel(
  optionsInput: unknown,
  sourceGateReadModelInput: unknown
): AppGameSourceGatedPolicyPreviewReadModel {
  const options = AppGameSourceGatedPolicyPreviewReadModelOptionsSchema.parse(optionsInput);
  const sourceGateReadModel = AppGameSourceFreshnessPreviewGateReadModelSchema.parse(sourceGateReadModelInput);
  const rows = sourceGateReadModel.rows.map((row) =>
    buildAppGameSourceGatedPolicyPreviewReadModelRow(options, sourceGateReadModel, row)
  );

  return AppGameSourceGatedPolicyPreviewReadModelSchema.parse({
    schemaVersion: options.schemaVersion,
    readModelId: options.readModelId,
    sourceGateId: sourceGateReadModel.gateId,
    generatedAt: options.generatedAt,
    sourceContractRefs: options.sourceContractRefs,
    sourceGateContractRefs: sourceGateReadModel.sourceContractRefs,
    rows,
    ...countAppGameSourceGatedPolicyPreviewReadModelRows(rows),
    readModelNonClaims: RequiredAppGameSourceGatedPolicyPreviewReadModelNonClaims,
    ...AppGameSourceGatedPolicyPreviewReadModelNoClaimFlags,
  });
}

function buildAppGameSourceGatedPolicyPreviewReadModelRow(
  options: AppGameSourceGatedPolicyPreviewReadModelOptions,
  sourceGateReadModel: AppGameSourceFreshnessPreviewGateReadModel,
  row: AppGameSourceFreshnessPreviewGateRow
): AppGameSourceGatedPolicyPreviewReadModelRow {
  return AppGameSourceGatedPolicyPreviewReadModelRowSchema.parse({
    schemaVersion: options.schemaVersion,
    rowId: `${row.rowId}:source-gated-preview-read-model`,
    sourceGateRowId: row.rowId,
    sourceGateId: sourceGateReadModel.gateId,
    targetDomain: row.targetDomain,
    sourceReadinessId: row.sourceReadinessId,
    sourcePolicyRequestId: row.sourcePolicyRequestId,
    sourceReadinessState: row.sourceReadinessState,
    sourceRequirementStates: row.sourceRequirementStates,
    sourcePolicyCompileAllowed: row.sourcePolicyCompileAllowed,
    sourceEvidenceRefs: row.sourceEvidenceRefs,
    gateState: row.gateState,
    projectionState: projectionStateForGateRow(row),
    previewStatus: row.previewStatus,
    previewDecisionRef: row.previewRow?.policyDecisionId ?? null,
    previewCompilerStatus: row.previewRow?.previewStatus ?? null,
    sensitiveDetailBoundary: AppGameSourceGatedPolicyPreviewReadModelSensitiveBoundary.RedactedEvidenceRefsOnly,
    ...AppGameSourceGatedPolicyPreviewReadModelNoClaimFlags,
    generatedAt: options.generatedAt,
  });
}

function projectionStateForGateRow(row: AppGameSourceFreshnessPreviewGateRow) {
  switch (row.gateState) {
    case 'source-fresh':
      return AppGameSourceGatedPolicyPreviewReadModelProjectionState.PreviewReadyVisible;
    case 'source-manual-required':
      return AppGameSourceGatedPolicyPreviewReadModelProjectionState.SourceManualRequiredVisible;
    case 'compiler-manual-required':
      return AppGameSourceGatedPolicyPreviewReadModelProjectionState.CompilerManualRequiredVisible;
  }
}

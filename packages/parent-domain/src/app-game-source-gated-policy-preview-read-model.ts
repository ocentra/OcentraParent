import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  AppGamePolicyPreviewStatusSchema,
  AppGamePolicyPreviewTargetDomainSchema,
} from './app-game-policy-preview-handoff';
import {
  AppGameSourceFreshnessPreviewGateIdSchema,
  AppGameSourceFreshnessPreviewGateReadModelSchema,
  AppGameSourceFreshnessPreviewGateRowIdSchema,
  AppGameSourceFreshnessPreviewGateStateSchema,
  AppGameSourceFreshnessPreviewGateStatusSchema,
  type AppGameSourceFreshnessPreviewGateReadModel,
  type AppGameSourceFreshnessPreviewGateRow,
} from './app-game-source-freshness-preview-gate';
import {
  AppGameSourceFreshnessEvidenceRefSchema,
  AppGameSourceFreshnessPolicyReadinessIdSchema,
  AppGameSourceFreshnessPolicyReadinessStateSchema,
  AppGameSourceFreshnessPolicyRequestIdSchema,
  AppGameSourceFreshnessRequirementStateSchema,
} from './app-game-source-freshness-policy-consumption';
import {
  AppGameSourceGatedPolicyPreviewReadModelNoClaimFlags,
  AppGameSourceGatedPolicyPreviewReadModelProjectionState,
  AppGameSourceGatedPolicyPreviewReadModelSensitiveBoundary,
  RequiredAppGameSourceGatedPolicyPreviewReadModelNonClaims,
  appGameSourceGatedPolicyPreviewReadModelCountsMatchRows,
  appGameSourceGatedPolicyPreviewReadModelHasNoRuntimeClaims,
  appGameSourceGatedPolicyPreviewReadModelRowHasNoRuntimeClaims,
  appGameSourceGatedPolicyPreviewReadModelRowMatchesGateState,
  countAppGameSourceGatedPolicyPreviewReadModelRows,
} from './app-game-source-gated-policy-preview-read-model-rules';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './reference-primitives';

const SourceGatedPolicyPreviewText = Schema.String.pipe(Schema.minLength(1));

export const AppGameSourceGatedPolicyPreviewReadModelIdSchema = SourceGatedPolicyPreviewText.pipe(
  Schema.brand('AppGameSourceGatedPolicyPreviewReadModelId')
);
export const AppGameSourceGatedPolicyPreviewReadModelRowIdSchema = SourceGatedPolicyPreviewText.pipe(
  Schema.brand('AppGameSourceGatedPolicyPreviewReadModelRowId')
);
export const AppGameSourceGatedPolicyPreviewReadModelContractRefSchema = SourceGatedPolicyPreviewText.pipe(
  Schema.brand('AppGameSourceGatedPolicyPreviewReadModelContractRef')
);
export const AppGameSourceGatedPolicyPreviewDecisionRefSchema = SourceGatedPolicyPreviewText.pipe(
  Schema.brand('AppGameSourceGatedPolicyPreviewDecisionRef')
);

export const AppGameSourceGatedPolicyPreviewReadModelProjectionStateSchema = withParser(
  Schema.Literal(...Object.values(AppGameSourceGatedPolicyPreviewReadModelProjectionState))
);
export const AppGameSourceGatedPolicyPreviewReadModelSensitiveBoundarySchema = withParser(
  Schema.Literal(...Object.values(AppGameSourceGatedPolicyPreviewReadModelSensitiveBoundary))
);
export const AppGameSourceGatedPolicyPreviewReadModelNonClaimSchema = withParser(
  Schema.Literal(...RequiredAppGameSourceGatedPolicyPreviewReadModelNonClaims)
);

export const AppGameSourceGatedPolicyPreviewReadModelOptionsSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    readModelId: AppGameSourceGatedPolicyPreviewReadModelIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceContractRefs: Schema.Array(AppGameSourceGatedPolicyPreviewReadModelContractRefSchema),
  }).pipe(
    Schema.filter(
      (options) =>
        options.sourceContractRefs.length > 0 ||
        'Expected source-gated policy preview read model options to cite source contracts'
    )
  )
);

const AppGameSourceGatedPolicyPreviewReadModelRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  rowId: AppGameSourceGatedPolicyPreviewReadModelRowIdSchema,
  sourceGateRowId: AppGameSourceFreshnessPreviewGateRowIdSchema,
  sourceGateId: AppGameSourceFreshnessPreviewGateIdSchema,
  targetDomain: AppGamePolicyPreviewTargetDomainSchema,
  sourceReadinessId: AppGameSourceFreshnessPolicyReadinessIdSchema,
  sourcePolicyRequestId: AppGameSourceFreshnessPolicyRequestIdSchema,
  sourceReadinessState: AppGameSourceFreshnessPolicyReadinessStateSchema,
  sourceRequirementStates: Schema.Array(AppGameSourceFreshnessRequirementStateSchema),
  sourcePolicyCompileAllowed: Schema.Boolean,
  sourceEvidenceRefs: Schema.Array(AppGameSourceFreshnessEvidenceRefSchema),
  gateState: AppGameSourceFreshnessPreviewGateStateSchema,
  projectionState: AppGameSourceGatedPolicyPreviewReadModelProjectionStateSchema,
  previewStatus: AppGameSourceFreshnessPreviewGateStatusSchema,
  previewDecisionRef: Schema.Union(AppGameSourceGatedPolicyPreviewDecisionRefSchema, Schema.Null),
  previewCompilerStatus: Schema.Union(AppGamePolicyPreviewStatusSchema, Schema.Null),
  sensitiveDetailBoundary: AppGameSourceGatedPolicyPreviewReadModelSensitiveBoundarySchema,
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

export const AppGameSourceGatedPolicyPreviewReadModelRowSchema = withParser(
  AppGameSourceGatedPolicyPreviewReadModelRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        appGameSourceGatedPolicyPreviewReadModelRowMatchesGateState(row) ||
        'Expected source-gated policy preview read-model rows to preserve source, compiler, and preview gate state'
    )
  ).pipe(
    Schema.filter(
      (row) =>
        appGameSourceGatedPolicyPreviewReadModelRowHasNoRuntimeClaims(row) ||
        'Expected source-gated policy preview read-model rows to avoid runtime, UI, adapter, and raw-source claims'
    )
  )
);

const AppGameSourceGatedPolicyPreviewReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  readModelId: AppGameSourceGatedPolicyPreviewReadModelIdSchema,
  sourceGateId: AppGameSourceFreshnessPreviewGateIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceContractRefs: Schema.Array(AppGameSourceGatedPolicyPreviewReadModelContractRefSchema),
  sourceGateContractRefs: Schema.Array(AppGameSourceGatedPolicyPreviewReadModelContractRefSchema),
  rows: Schema.Array(AppGameSourceGatedPolicyPreviewReadModelRowSchema),
  nativeAppRowCount: Schema.Number,
  nativeGameRowCount: Schema.Number,
  previewReadyVisibleCount: Schema.Number,
  sourceManualRequiredVisibleCount: Schema.Number,
  compilerManualRequiredVisibleCount: Schema.Number,
  readModelNonClaims: Schema.Array(AppGameSourceGatedPolicyPreviewReadModelNonClaimSchema),
  serviceRuntimeEventClaimed: Schema.Literal(false),
  portalUiRendered: Schema.Literal(false),
  policyEvaluatorRuntimeClaimed: Schema.Literal(false),
  timerRuntimeClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  childDeliveryClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  rawPrivateSourceRowsIncluded: Schema.Literal(false),
});

export const AppGameSourceGatedPolicyPreviewReadModelSchema = withParser(
  AppGameSourceGatedPolicyPreviewReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        appGameSourceGatedPolicyPreviewReadModelCountsMatchRows(readModel) ||
        'Expected source-gated policy preview read-model counts to match rows'
    )
  )
    .pipe(
      Schema.filter(
        (readModel) =>
          new Set(readModel.rows.map((row) => row.rowId)).size === readModel.rows.length ||
          'Expected source-gated policy preview read-model row ids to be unique'
      )
    )
    .pipe(
      Schema.filter(
        (readModel) =>
          appGameSourceGatedPolicyPreviewReadModelHasNoRuntimeClaims(readModel) ||
          'Expected source-gated policy preview read model to avoid runtime, UI, adapter, and raw-source claims'
      )
    )
);

export type AppGameSourceGatedPolicyPreviewReadModelOptions = Infer<
  typeof AppGameSourceGatedPolicyPreviewReadModelOptionsSchema
>;
export type AppGameSourceGatedPolicyPreviewReadModelRow = Infer<
  typeof AppGameSourceGatedPolicyPreviewReadModelRowSchema
>;
export type AppGameSourceGatedPolicyPreviewReadModel = Infer<typeof AppGameSourceGatedPolicyPreviewReadModelSchema>;

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

export const decodeAppGameSourceGatedPolicyPreviewReadModel = Schema.decodeUnknownSync(
  AppGameSourceGatedPolicyPreviewReadModelSchema
);

export {
  AppGameSourceGatedPolicyPreviewReadModelProjectionState,
  AppGameSourceGatedPolicyPreviewReadModelSensitiveBoundary,
};

import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from './effect';
import {
  AppGamePolicyPreviewStatusSchema,
  AppGamePolicyPreviewTargetDomainSchema,
} from './app-game-policy-preview-handoff';
import {
  AppGameSourceFreshnessPreviewGateIdSchema,
  AppGameSourceFreshnessPreviewGateRowIdSchema,
  AppGameSourceFreshnessPreviewGateStateSchema,
  AppGameSourceFreshnessPreviewGateStatusSchema,
} from './generated-app-game-source-freshness-preview-gate';
import {
  AppGameSourceFreshnessEvidenceRefSchema,
  AppGameSourceFreshnessPolicyReadinessIdSchema,
  AppGameSourceFreshnessPolicyReadinessStateSchema,
  AppGameSourceFreshnessPolicyRequestIdSchema,
  AppGameSourceFreshnessRequirementStateSchema,
} from './app-game-source-freshness-policy-consumption';
import {
  AppGameSourceGatedPolicyPreviewReadModelProjectionState,
  AppGameSourceGatedPolicyPreviewReadModelSensitiveBoundary,
  RequiredAppGameSourceGatedPolicyPreviewReadModelNonClaims,
  appGameSourceGatedPolicyPreviewReadModelCountsMatchRows,
  appGameSourceGatedPolicyPreviewReadModelHasNoRuntimeClaims,
  appGameSourceGatedPolicyPreviewReadModelRowHasNoRuntimeClaims,
  appGameSourceGatedPolicyPreviewReadModelRowMatchesGateState,
} from './app-game-source-gated-policy-preview-read-model-rules';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './family-reference-primitives';

export const AppGameSourceGatedPolicyPreviewReadModelIdSchema = brandedNonEmptyStringSchema(
  'AppGameSourceGatedPolicyPreviewReadModelId'
);
export const AppGameSourceGatedPolicyPreviewReadModelRowIdSchema = brandedNonEmptyStringSchema(
  'AppGameSourceGatedPolicyPreviewReadModelRowId'
);
export const AppGameSourceGatedPolicyPreviewReadModelContractRefSchema = brandedNonEmptyStringSchema(
  'AppGameSourceGatedPolicyPreviewReadModelContractRef'
);
export const AppGameSourceGatedPolicyPreviewDecisionRefSchema = brandedNonEmptyStringSchema(
  'AppGameSourceGatedPolicyPreviewDecisionRef'
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

export const decodeAppGameSourceGatedPolicyPreviewReadModel = Schema.decodeUnknownSync(
  AppGameSourceGatedPolicyPreviewReadModelSchema
);

export {
  AppGameSourceGatedPolicyPreviewReadModelProjectionState,
  AppGameSourceGatedPolicyPreviewReadModelSensitiveBoundary,
};

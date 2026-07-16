import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from './effect';
import { AppGamePolicyCompiledDecisionSchema } from './app-game-policy-target-compiler';
import {
  AppGamePolicyPreviewHandoffOptionsSchema,
  AppGamePolicyPreviewHandoffRowSchema,
} from './app-game-policy-preview-handoff';
import { AppGamePolicyPreviewTargetDomain } from './app-game-policy-preview-handoff-rules';
import {
  AppGameSourceFreshnessEvidenceRefSchema,
  AppGameSourceFreshnessPolicyReadinessSchema,
  AppGameSourceFreshnessPolicyReadinessIdSchema,
  AppGameSourceFreshnessPolicyReadinessStateSchema,
  AppGameSourceFreshnessPolicyRequestIdSchema,
  AppGameSourceFreshnessReasonCodeSchema,
  AppGameSourceFreshnessRequirementStateSchema,
} from './app-game-source-freshness-policy-consumption';
import {
  AppGameSourceFreshnessPreviewGateState,
  AppGameSourceFreshnessPreviewGateStatus,
  appGameSourceFreshnessPreviewGateReadModelCountsMatchRows,
  appGameSourceFreshnessPreviewGateReadModelHasNoRuntimeClaims,
  appGameSourceFreshnessPreviewGateRowHasNoRuntimeClaims,
  appGameSourceFreshnessPreviewGateRowMatchesPreviewState,
  appGameSourceFreshnessPreviewGateRowMatchesSourceState,
} from './app-game-source-freshness-preview-gate-rules';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './family-reference-primitives';

export const AppGameSourceFreshnessPreviewGateIdSchema = brandedNonEmptyStringSchema(
  'AppGameSourceFreshnessPreviewGateId'
);
export const AppGameSourceFreshnessPreviewGateRowIdSchema = brandedNonEmptyStringSchema(
  'AppGameSourceFreshnessPreviewGateRowId'
);
export const AppGameSourceFreshnessPreviewGateSourceContractRefSchema = brandedNonEmptyStringSchema(
  'AppGameSourceFreshnessPreviewGateSourceContractRef'
);

export const AppGameSourceFreshnessPreviewGateStatusSchema = withParser(
  Schema.Literal(...Object.values(AppGameSourceFreshnessPreviewGateStatus))
);
export const AppGameSourceFreshnessPreviewGateStateSchema = withParser(
  Schema.Literal(...Object.values(AppGameSourceFreshnessPreviewGateState))
);
export const AppGameSourceFreshnessPreviewGateTargetDomainSchema = withParser(
  Schema.Literal(...Object.values(AppGamePolicyPreviewTargetDomain))
);

export const AppGameSourceFreshnessPreviewGateOptionsSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    gateId: AppGameSourceFreshnessPreviewGateIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceContractRefs: Schema.Array(AppGameSourceFreshnessPreviewGateSourceContractRefSchema),
    policyPreviewOptions: AppGamePolicyPreviewHandoffOptionsSchema,
  }).pipe(
    Schema.filter(
      (options) =>
        options.sourceContractRefs.length > 0 ||
        'Expected source freshness preview gate options to cite source contracts'
    )
  )
);

export const AppGameSourceFreshnessPreviewGateEntrySchema = withParser(
  Schema.Struct({
    rowId: AppGameSourceFreshnessPreviewGateRowIdSchema,
    sourceReadiness: AppGameSourceFreshnessPolicyReadinessSchema,
    compiledDecision: Schema.Union(AppGamePolicyCompiledDecisionSchema, Schema.Null),
  })
);

const AppGameSourceFreshnessPreviewGateRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  rowId: AppGameSourceFreshnessPreviewGateRowIdSchema,
  targetDomain: AppGameSourceFreshnessPreviewGateTargetDomainSchema,
  sourceReadinessId: AppGameSourceFreshnessPolicyReadinessIdSchema,
  sourcePolicyRequestId: AppGameSourceFreshnessPolicyRequestIdSchema,
  sourceReadinessState: AppGameSourceFreshnessPolicyReadinessStateSchema,
  sourcePolicyCompileAllowed: Schema.Boolean,
  sourceRequirementStates: Schema.Array(AppGameSourceFreshnessRequirementStateSchema),
  sourceReasonCodes: Schema.Array(Schema.Union(AppGameSourceFreshnessReasonCodeSchema, Schema.Null)),
  sourceEvidenceRefs: Schema.Array(AppGameSourceFreshnessEvidenceRefSchema),
  compiledDecisionProvided: Schema.Boolean,
  previewStatus: AppGameSourceFreshnessPreviewGateStatusSchema,
  gateState: AppGameSourceFreshnessPreviewGateStateSchema,
  previewRow: Schema.Union(AppGamePolicyPreviewHandoffRowSchema, Schema.Null),
  policyEvaluatorRuntimeClaimed: Schema.Boolean,
  timerRuntimeClaimed: Schema.Boolean,
  adapterDispatchClaimed: Schema.Boolean,
  childDeliveryClaimed: Schema.Boolean,
  platformEnforcementClaimed: Schema.Boolean,
  generatedAt: ParentTimestampSchema,
});

export const AppGameSourceFreshnessPreviewGateRowSchema = withParser(
  AppGameSourceFreshnessPreviewGateRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        appGameSourceFreshnessPreviewGateRowMatchesSourceState(row) ||
        'Expected source freshness preview gate rows to block stale/manual source states before preview'
    )
  )
    .pipe(
      Schema.filter(
        (row) =>
          appGameSourceFreshnessPreviewGateRowMatchesPreviewState(row) ||
          'Expected source freshness preview gate rows to preserve policy preview status'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          appGameSourceFreshnessPreviewGateRowHasNoRuntimeClaims(row) ||
          'Expected source freshness preview gate rows to avoid evaluator, timer, adapter, child-delivery, and platform claims'
      )
    )
);

const AppGameSourceFreshnessPreviewGateReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  gateId: AppGameSourceFreshnessPreviewGateIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceContractRefs: Schema.Array(AppGameSourceFreshnessPreviewGateSourceContractRefSchema),
  rows: Schema.Array(AppGameSourceFreshnessPreviewGateRowSchema),
  nativeAppRowCount: Schema.Number,
  nativeGameRowCount: Schema.Number,
  previewReadyCount: Schema.Number,
  manualRequiredCount: Schema.Number,
  sourceManualRequiredCount: Schema.Number,
  compilerManualRequiredCount: Schema.Number,
  policyEvaluatorRuntimeClaimed: Schema.Boolean,
  timerRuntimeClaimed: Schema.Boolean,
  adapterDispatchClaimed: Schema.Boolean,
  childDeliveryClaimed: Schema.Boolean,
  platformEnforcementClaimed: Schema.Boolean,
});

export const AppGameSourceFreshnessPreviewGateReadModelSchema = withParser(
  AppGameSourceFreshnessPreviewGateReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        appGameSourceFreshnessPreviewGateReadModelCountsMatchRows(readModel) ||
        'Expected source freshness preview gate counts to match rows'
    )
  )
    .pipe(
      Schema.filter(
        (readModel) =>
          new Set(readModel.rows.map((row) => row.rowId)).size === readModel.rows.length ||
          'Expected source freshness preview gate row ids to be unique'
      )
    )
    .pipe(
      Schema.filter(
        (readModel) =>
          appGameSourceFreshnessPreviewGateReadModelHasNoRuntimeClaims(readModel) ||
          'Expected source freshness preview gate read model to avoid runtime and enforcement claims'
      )
    )
);

export type AppGameSourceFreshnessPreviewGateOptions = Infer<typeof AppGameSourceFreshnessPreviewGateOptionsSchema>;
export type AppGameSourceFreshnessPreviewGateEntry = Infer<typeof AppGameSourceFreshnessPreviewGateEntrySchema>;
export type AppGameSourceFreshnessPreviewGateRow = Infer<typeof AppGameSourceFreshnessPreviewGateRowSchema>;
export type AppGameSourceFreshnessPreviewGateReadModel = Infer<typeof AppGameSourceFreshnessPreviewGateReadModelSchema>;

export const decodeAppGameSourceFreshnessPreviewGateReadModel = (input: unknown) =>
  AppGameSourceFreshnessPreviewGateReadModelSchema.parse(input);

export { AppGameSourceFreshnessPreviewGateState, AppGameSourceFreshnessPreviewGateStatus };

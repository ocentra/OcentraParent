import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import {
  AppGamePolicyCompiledDecisionSchema,
  type AppGamePolicyCompiledDecision,
} from './app-game-policy-target-compiler';
import {
  AppGamePolicyPreviewHandoffOptionsSchema,
  AppGamePolicyPreviewHandoffRowSchema,
  type AppGamePolicyPreviewHandoffOptions,
  buildAppGamePolicyPreviewHandoffRow,
} from './app-game-policy-preview-handoff';
import {
  AppGamePolicyPreviewTargetDomain,
  appGamePolicyPreviewTargetDomainForKind,
} from './app-game-policy-preview-handoff-rules';
import {
  AppGameSourceFreshnessEvidenceRefSchema,
  AppGameSourceFreshnessPolicyReadinessSchema,
  AppGameSourceFreshnessPolicyReadinessIdSchema,
  AppGameSourceFreshnessPolicyReadinessStateSchema,
  AppGameSourceFreshnessPolicyRequestIdSchema,
  AppGameSourceFreshnessReasonCodeSchema,
  AppGameSourceFreshnessRequirementStateSchema,
  type AppGameSourceFreshnessPolicyReadiness,
} from './app-game-source-freshness-policy-consumption';
import {
  AppGameSourceFreshnessPolicyReadinessState,
  AppGameSourceFreshnessPolicyTargetKind,
} from './app-game-source-freshness-policy-consumption-values';
import {
  AppGameSourceFreshnessPreviewGateNoRuntimeClaimFlags,
  AppGameSourceFreshnessPreviewGateState,
  AppGameSourceFreshnessPreviewGateStatus,
  appGameSourceFreshnessPreviewGateReadModelCountsMatchRows,
  appGameSourceFreshnessPreviewGateReadModelHasNoRuntimeClaims,
  appGameSourceFreshnessPreviewGateRowHasNoRuntimeClaims,
  appGameSourceFreshnessPreviewGateRowMatchesPreviewState,
  appGameSourceFreshnessPreviewGateRowMatchesSourceState,
  countAppGameSourceFreshnessPreviewGateReadModelRows,
} from './app-game-source-freshness-preview-gate-rules';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';

export const AppGameSourceFreshnessPreviewGateIdSchema = brandedNonEmptyStringSchema('AppGameSourceFreshnessPreviewGateId');
export const AppGameSourceFreshnessPreviewGateRowIdSchema = brandedNonEmptyStringSchema('AppGameSourceFreshnessPreviewGateRowId');
export const AppGameSourceFreshnessPreviewGateSourceContractRefSchema = brandedNonEmptyStringSchema('AppGameSourceFreshnessPreviewGateSourceContractRef');

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

export function buildAppGameSourceFreshnessPreviewGateReadModel(
  optionsInput: unknown,
  entriesInput: readonly unknown[]
): AppGameSourceFreshnessPreviewGateReadModel {
  const options = AppGameSourceFreshnessPreviewGateOptionsSchema.parse(optionsInput);
  const rows = entriesInput.map((entry) => buildAppGameSourceFreshnessPreviewGateRow(options, entry));
  return AppGameSourceFreshnessPreviewGateReadModelSchema.parse({
    schemaVersion: options.schemaVersion,
    gateId: options.gateId,
    generatedAt: options.generatedAt,
    sourceContractRefs: options.sourceContractRefs,
    rows,
    ...countAppGameSourceFreshnessPreviewGateReadModelRows(rows),
    ...AppGameSourceFreshnessPreviewGateNoRuntimeClaimFlags,
  });
}

export function buildAppGameSourceFreshnessPreviewGateRow(
  optionsInput: unknown,
  entryInput: unknown
): AppGameSourceFreshnessPreviewGateRow {
  const options = AppGameSourceFreshnessPreviewGateOptionsSchema.parse(optionsInput);
  const entry = AppGameSourceFreshnessPreviewGateEntrySchema.parse(entryInput);
  const previewRow = buildPreviewRow(options.policyPreviewOptions, entry.sourceReadiness, entry.compiledDecision);
  const targetDomain = sourceFreshnessTargetDomain(entry.sourceReadiness);
  return AppGameSourceFreshnessPreviewGateRowSchema.parse({
    schemaVersion: options.schemaVersion,
    rowId: entry.rowId,
    targetDomain,
    sourceReadinessId: entry.sourceReadiness.readinessId,
    sourcePolicyRequestId: entry.sourceReadiness.request.policyRequestId,
    sourceReadinessState: entry.sourceReadiness.readinessState,
    sourcePolicyCompileAllowed: entry.sourceReadiness.policyCompileAllowed,
    sourceRequirementStates: entry.sourceReadiness.requirementResults.map((result) => result.requirementState),
    sourceReasonCodes: entry.sourceReadiness.requirementResults.map((result) => result.reasonCode),
    sourceEvidenceRefs: entry.sourceReadiness.policyEvidenceRefs,
    compiledDecisionProvided: entry.compiledDecision !== null,
    previewStatus:
      previewRow === null ? AppGameSourceFreshnessPreviewGateStatus.ManualRequired : previewRow.previewStatus,
    gateState: previewGateState(entry.sourceReadiness, previewRow),
    previewRow,
    ...AppGameSourceFreshnessPreviewGateNoRuntimeClaimFlags,
    generatedAt: options.generatedAt,
  });
}

function buildPreviewRow(
  options: AppGamePolicyPreviewHandoffOptions,
  sourceReadiness: AppGameSourceFreshnessPolicyReadiness,
  compiledDecision: AppGamePolicyCompiledDecision | null
) {
  if (sourceReadiness.readinessState === AppGameSourceFreshnessPolicyReadinessState.ManualRequired) {
    return null;
  }

  if (compiledDecision === null) {
    throw new Error('Expected policy-ready source freshness rows to include a compiled preview decision');
  }

  const previewRow = buildAppGamePolicyPreviewHandoffRow(options, compiledDecision);
  if (previewRow.targetDomain !== sourceFreshnessTargetDomain(sourceReadiness)) {
    throw new Error('Expected source freshness target domain to match compiled policy preview target domain');
  }
  return previewRow;
}

function previewGateState(
  sourceReadiness: AppGameSourceFreshnessPolicyReadiness,
  previewRow: ReturnType<typeof buildAppGamePolicyPreviewHandoffRow> | null
) {
  if (sourceReadiness.readinessState === AppGameSourceFreshnessPolicyReadinessState.ManualRequired) {
    return AppGameSourceFreshnessPreviewGateState.SourceManualRequired;
  }

  return previewRow?.previewStatus === AppGameSourceFreshnessPreviewGateStatus.PreviewReady
    ? AppGameSourceFreshnessPreviewGateState.SourceFresh
    : AppGameSourceFreshnessPreviewGateState.CompilerManualRequired;
}

function sourceFreshnessTargetDomain(readiness: AppGameSourceFreshnessPolicyReadiness) {
  switch (readiness.request.target.targetKind) {
    case AppGameSourceFreshnessPolicyTargetKind.NativeApp:
      return AppGamePolicyPreviewTargetDomain.NativeApp;
    case AppGameSourceFreshnessPolicyTargetKind.NativeGame:
      return AppGamePolicyPreviewTargetDomain.NativeGame;
    case AppGameSourceFreshnessPolicyTargetKind.AllNativeApps:
      return AppGamePolicyPreviewTargetDomain.NativeApp;
    case AppGameSourceFreshnessPolicyTargetKind.AllNativeGames:
      return AppGamePolicyPreviewTargetDomain.NativeGame;
  }
}

export const appGameSourceFreshnessPreviewGateCompiledDecisionDomain = (decisionInput: unknown) => {
  const decision = AppGamePolicyCompiledDecisionSchema.parse(decisionInput);
  return appGamePolicyPreviewTargetDomainForKind(decision.request.target.targetKind);
};

export const decodeAppGameSourceFreshnessPreviewGateReadModel = Schema.decodeUnknownSync(
  AppGameSourceFreshnessPreviewGateReadModelSchema
);

export { AppGameSourceFreshnessPreviewGateState, AppGameSourceFreshnessPreviewGateStatus };


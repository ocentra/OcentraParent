/* generated from crates/app-game-core/src/app_game_policy_preview_handoff_generated_ts.rs */

import { type Infer, NonEmptyStringSchema, Schema, withParser, brandedNonEmptyStringSchema } from './effect';
import {
  AppGamePolicyCompiledDecisionIdSchema,
  AppGamePolicyCompileRequestIdSchema,
  AppGamePolicyCompilerOutcomeStateSchema,
  AppGamePolicyCompilerRejectionReasonSchema,
  AppGamePolicyTargetKindSchema,
} from './app-game-policy-target-compiler';
import {
  AppGamePolicyPreviewStatus as AppGamePolicyPreviewStatusGeneratedValue,
  AppGamePolicyPreviewTargetDomain as AppGamePolicyPreviewTargetDomainGeneratedValue,
  appGamePolicyPreviewReadModelCountsMatchRows,
  appGamePolicyPreviewRowHasNoRuntimeClaims,
  appGamePolicyPreviewRowHasProofRefs,
  appGamePolicyPreviewRowIsDryRunOnly,
  appGamePolicyPreviewStatusMatchesOutcome,
} from './app-game-policy-preview-handoff-rules';
import {
  PolicyActionSchema,
  PolicyDecisionHandoffStateSchema,
  PolicyDecisionIdSchema,
  PolicyRuleIdSchema,
  PolicyTargetSchema,
} from './policy-contracts';
import { ParentDeviceReferenceSchema, ParentEvidenceReferenceSchema } from './family-references';
import {
  ParentContractSchemaVersionSchema,
  ParentPolicyVersionSchema,
  ParentTimestampSchema,
} from './family-reference-primitives';
import {
  AppGamePolicyPreviewHandoffAdapterDispatchStateGenerated,
  AppGamePolicyPreviewHandoffRuntimeClaimStateGenerated,
} from './generated-app-game-policy-preview-handoff-values';

export const AppGamePolicyPreviewHandoffIdSchema = brandedNonEmptyStringSchema('AppGamePolicyPreviewHandoffId');
export const AppGamePolicyPreviewHandoffRowIdSchema = brandedNonEmptyStringSchema('AppGamePolicyPreviewHandoffRowId');
export const AppGamePolicyPreviewHandoffSourceContractRefSchema = brandedNonEmptyStringSchema(
  'AppGamePolicyPreviewHandoffSourceContractRef'
);
export const AppGamePolicyPreviewHandoffRuntimeClaimStateSchema = withParser(
  Schema.Literal(AppGamePolicyPreviewHandoffRuntimeClaimStateGenerated.NotClaimed)
);
export const AppGamePolicyPreviewHandoffAdapterDispatchStateSchema = withParser(
  Schema.Literal(AppGamePolicyPreviewHandoffAdapterDispatchStateGenerated.NotDispatched)
);

export const AppGamePolicyPreviewTargetDomainSchema = withParser(
  Schema.Literal(...Object.values(AppGamePolicyPreviewTargetDomainGeneratedValue))
);
export const AppGamePolicyPreviewStatusSchema = withParser(
  Schema.Literal(...Object.values(AppGamePolicyPreviewStatusGeneratedValue))
);

export const AppGamePolicyPreviewHandoffOptionsSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    handoffId: AppGamePolicyPreviewHandoffIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceContractRefs: Schema.Array(AppGamePolicyPreviewHandoffSourceContractRefSchema),
  }).pipe(
    Schema.filter(
      (options) =>
        options.sourceContractRefs.length > 0 ||
        'Expected app/game policy preview handoff options to cite source contracts'
    )
  )
);

const AppGamePolicyPreviewHandoffRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  rowId: AppGamePolicyPreviewHandoffRowIdSchema,
  targetDomain: AppGamePolicyPreviewTargetDomainSchema,
  sourceCompiledDecisionId: AppGamePolicyCompiledDecisionIdSchema,
  sourceCompileRequestId: AppGamePolicyCompileRequestIdSchema,
  sourceTargetKind: AppGamePolicyTargetKindSchema,
  device: ParentDeviceReferenceSchema,
  policyVersion: ParentPolicyVersionSchema,
  policyTarget: PolicyTargetSchema,
  policyDecisionId: PolicyDecisionIdSchema,
  policyAction: PolicyActionSchema,
  outcomeState: AppGamePolicyCompilerOutcomeStateSchema,
  previewStatus: AppGamePolicyPreviewStatusSchema,
  rejectionReason: AppGamePolicyCompilerRejectionReasonSchema,
  ruleRefs: Schema.Array(PolicyRuleIdSchema),
  evidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
  capabilityRefs: Schema.Array(NonEmptyStringSchema),
  authorityRefs: Schema.Array(NonEmptyStringSchema),
  auditRefs: Schema.Array(NonEmptyStringSchema),
  dryRun: Schema.Boolean,
  enforcementHandoffState: PolicyDecisionHandoffStateSchema,
  policyEvaluatorRuntimeClaimState: AppGamePolicyPreviewHandoffRuntimeClaimStateSchema,
  timerRuntimeClaimState: AppGamePolicyPreviewHandoffRuntimeClaimStateSchema,
  adapterDispatchState: AppGamePolicyPreviewHandoffAdapterDispatchStateSchema,
  childDeliveryClaimState: AppGamePolicyPreviewHandoffRuntimeClaimStateSchema,
  platformEnforcementClaimState: AppGamePolicyPreviewHandoffRuntimeClaimStateSchema,
  policyEvaluatorRuntimeClaimed: Schema.Boolean,
  timerRuntimeClaimed: Schema.Boolean,
  adapterDispatchClaimed: Schema.Boolean,
  childDeliveryClaimed: Schema.Boolean,
  platformEnforcementClaimed: Schema.Boolean,
  generatedAt: ParentTimestampSchema,
});

export const AppGamePolicyPreviewHandoffRowSchema = withParser(
  AppGamePolicyPreviewHandoffRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        appGamePolicyPreviewStatusMatchesOutcome(row) ||
        'Expected app/game policy preview rows to preserve compiler outcome state'
    )
  )
    .pipe(
      Schema.filter(
        (row) =>
          appGamePolicyPreviewRowIsDryRunOnly(row) ||
          'Expected app/game policy preview rows to remain dry-run with disabled enforcement handoff'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          appGamePolicyPreviewRowHasNoRuntimeClaims(row) ||
          'Expected app/game policy preview rows to avoid evaluator, timer, adapter, child-delivery, and platform enforcement claims'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          appGamePolicyPreviewRowHasProofRefs(row) ||
          'Expected app/game policy preview rows to retain evidence, rule, capability, and audit refs'
      )
    )
);

const AppGamePolicyPreviewHandoffReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  handoffId: AppGamePolicyPreviewHandoffIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceContractRefs: Schema.Array(AppGamePolicyPreviewHandoffSourceContractRefSchema),
  rows: Schema.Array(AppGamePolicyPreviewHandoffRowSchema),
  nativeAppRowCount: Schema.Number,
  nativeGameRowCount: Schema.Number,
  previewReadyCount: Schema.Number,
  manualRequiredCount: Schema.Number,
  rejectedCount: Schema.Number,
  policyEvaluatorRuntimeClaimed: Schema.Boolean,
  timerRuntimeClaimed: Schema.Boolean,
  adapterDispatchClaimed: Schema.Boolean,
  childDeliveryClaimed: Schema.Boolean,
  platformEnforcementClaimed: Schema.Boolean,
});

export const AppGamePolicyPreviewHandoffReadModelSchema = withParser(
  AppGamePolicyPreviewHandoffReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        appGamePolicyPreviewReadModelCountsMatchRows(readModel) ||
        'Expected app/game policy preview handoff counts to match rows'
    )
  )
    .pipe(
      Schema.filter(
        (readModel) =>
          new Set(readModel.rows.map((row) => row.rowId)).size === readModel.rows.length ||
          'Expected app/game policy preview handoff row ids to be unique'
      )
    )
    .pipe(
      Schema.filter(
        (readModel) =>
          (!readModel.policyEvaluatorRuntimeClaimed &&
            !readModel.timerRuntimeClaimed &&
            !readModel.adapterDispatchClaimed &&
            !readModel.childDeliveryClaimed &&
            !readModel.platformEnforcementClaimed) ||
          'Expected app/game policy preview handoff read model to avoid runtime and enforcement claims'
      )
    )
);

export type AppGamePolicyPreviewHandoffOptions = Infer<typeof AppGamePolicyPreviewHandoffOptionsSchema>;
export type AppGamePolicyPreviewHandoffRow = Infer<typeof AppGamePolicyPreviewHandoffRowSchema>;
export type AppGamePolicyPreviewHandoffReadModel = Infer<typeof AppGamePolicyPreviewHandoffReadModelSchema>;
export type AppGamePolicyPreviewTargetDomain = Infer<typeof AppGamePolicyPreviewTargetDomainSchema>;
export type AppGamePolicyPreviewStatus = Infer<typeof AppGamePolicyPreviewStatusSchema>;

export const decodeAppGamePolicyPreviewHandoffReadModel = (input: unknown) =>
  AppGamePolicyPreviewHandoffReadModelSchema.parse(input);

export const AppGamePolicyPreviewStatus = AppGamePolicyPreviewStatusGeneratedValue;
export const AppGamePolicyPreviewTargetDomain = AppGamePolicyPreviewTargetDomainGeneratedValue;

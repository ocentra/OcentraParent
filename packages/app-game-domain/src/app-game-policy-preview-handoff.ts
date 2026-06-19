import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema,
  NonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import {
  AppGamePolicyCompiledDecisionSchema,
  AppGamePolicyCompiledDecisionIdSchema,
  AppGamePolicyCompileRequestIdSchema,
  AppGamePolicyCompilerOutcomeStateSchema,
  AppGamePolicyCompilerRejectionReasonSchema,
  AppGamePolicyTargetKindSchema,
} from './app-game-policy-target-compiler';
import {
  AppGamePolicyPreviewStatus,
  AppGamePolicyPreviewTargetDomain,
  AppGamePolicyPreviewNoRuntimeClaimFlags,
  AppGamePolicyPreviewNoRuntimeClaimStates,
  countAppGamePolicyPreviewReadModelRows,
  appGamePolicyPreviewReadModelCountsMatchRows,
  appGamePolicyPreviewRowHasNoRuntimeClaims,
  appGamePolicyPreviewRowHasProofRefs,
  appGamePolicyPreviewRowIsDryRunOnly,
  appGamePolicyPreviewStatusForOutcome,
  appGamePolicyPreviewStatusMatchesOutcome,
  appGamePolicyPreviewTargetDomainForKind,
} from './app-game-policy-preview-handoff-rules';
import {
  PolicyActionSchema,
  PolicyDecisionHandoffStateSchema,
  PolicyDecisionIdSchema,
  PolicyRuleIdSchema,
  PolicyTargetSchema,
} from '@ocentra-parent/policy-domain/policy';
import { ParentDeviceReferenceSchema, ParentEvidenceReferenceSchema } from '@ocentra-parent/family-domain/references';
import {
  ParentContractSchemaVersionSchema,
  ParentPolicyVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/schema-domain/family-reference-primitives';

export const AppGamePolicyPreviewHandoffIdSchema = brandedNonEmptyStringSchema('AppGamePolicyPreviewHandoffId');
export const AppGamePolicyPreviewHandoffRowIdSchema = brandedNonEmptyStringSchema('AppGamePolicyPreviewHandoffRowId');
export const AppGamePolicyPreviewHandoffSourceContractRefSchema = brandedNonEmptyStringSchema('AppGamePolicyPreviewHandoffSourceContractRef');
export const AppGamePolicyPreviewHandoffRuntimeClaimStateSchema = withParser(Schema.Literal('not-claimed'));
export const AppGamePolicyPreviewHandoffAdapterDispatchStateSchema = withParser(Schema.Literal('not-dispatched'));

export const AppGamePolicyPreviewTargetDomainSchema = withParser(
  Schema.Literal(...Object.values(AppGamePolicyPreviewTargetDomain))
);
export const AppGamePolicyPreviewStatusSchema = withParser(
  Schema.Literal(...Object.values(AppGamePolicyPreviewStatus))
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

export function buildAppGamePolicyPreviewHandoffReadModel(
  optionsInput: unknown,
  decisionsInput: readonly unknown[]
): AppGamePolicyPreviewHandoffReadModel {
  const options = AppGamePolicyPreviewHandoffOptionsSchema.parse(optionsInput);
  const rows = decisionsInput.map((decision) => buildAppGamePolicyPreviewHandoffRow(options, decision));
  return AppGamePolicyPreviewHandoffReadModelSchema.parse({
    schemaVersion: options.schemaVersion,
    handoffId: options.handoffId,
    generatedAt: options.generatedAt,
    sourceContractRefs: options.sourceContractRefs,
    rows,
    ...countAppGamePolicyPreviewReadModelRows(rows),
    ...AppGamePolicyPreviewNoRuntimeClaimFlags,
  });
}

export function buildAppGamePolicyPreviewHandoffRow(
  optionsInput: unknown,
  decisionInput: unknown
): AppGamePolicyPreviewHandoffRow {
  const options = AppGamePolicyPreviewHandoffOptionsSchema.parse(optionsInput);
  const decision = AppGamePolicyCompiledDecisionSchema.parse(decisionInput);
  return AppGamePolicyPreviewHandoffRowSchema.parse({
    schemaVersion: decision.schemaVersion,
    rowId: `${decision.compiledDecisionId}:preview`,
    targetDomain: appGamePolicyPreviewTargetDomainForKind(decision.request.target.targetKind),
    sourceCompiledDecisionId: decision.compiledDecisionId,
    sourceCompileRequestId: decision.request.compileRequestId,
    sourceTargetKind: decision.request.target.targetKind,
    device: decision.request.device,
    policyVersion: decision.request.policyVersion,
    policyTarget: decision.policyTarget,
    policyDecisionId: decision.policyDecision.decisionId,
    policyAction: decision.policyDecision.action,
    outcomeState: decision.outcomeState,
    previewStatus: appGamePolicyPreviewStatusForOutcome(decision.outcomeState),
    rejectionReason: decision.rejectionReason,
    ruleRefs: decision.policyDecision.ruleIds,
    evidenceReferences: decision.policyDecision.evidenceReferences,
    capabilityRefs: decision.capabilityRefs,
    authorityRefs: decision.authorityRefs,
    auditRefs: decision.auditRefs,
    dryRun: decision.policyDecision.dryRun,
    enforcementHandoffState: decision.policyDecision.enforcementHandoffState,
    ...AppGamePolicyPreviewNoRuntimeClaimStates,
    ...AppGamePolicyPreviewNoRuntimeClaimFlags,
    generatedAt: options.generatedAt,
  });
}

export const decodeAppGamePolicyPreviewHandoffReadModel = Schema.decodeUnknownSync(
  AppGamePolicyPreviewHandoffReadModelSchema
);

export { AppGamePolicyPreviewStatus, AppGamePolicyPreviewTargetDomain };


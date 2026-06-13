import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema,
  NonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import {
  PolicyActionSchema,
  PolicyDecisionSchema,
  PolicyRuleSchema,
  comparePolicyActionStrictness,
  selectStricterPolicyAction,
} from '@ocentra-parent/policy-domain/policy';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
const ScreenAiStricterParentRuleProofIdSchema = brandedNonEmptyStringSchema('ScreenAiStricterParentRuleProofId');

export const ScreenAiStricterParentRuleClaimBoundarySchema = withParser(
  Schema.Struct({
    localAiAuthorityClaimed: Schema.Literal(false),
    remoteAiUsed: Schema.Literal(false),
    apiAiUsed: Schema.Literal(false),
    rawImageRetained: Schema.Literal(false),
    enforcementClaimed: Schema.Literal(false),
  })
);

const ScreenAiStricterParentRuleInputBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  proofId: ScreenAiStricterParentRuleProofIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceProof: NonEmptyStringSchema,
  sourceDecision: PolicyDecisionSchema,
  stricterParentRule: PolicyRuleSchema,
  expectedFinalAction: PolicyActionSchema,
  claimBoundaries: ScreenAiStricterParentRuleClaimBoundarySchema,
});

type ScreenAiStricterParentRuleInputCandidate = Infer<typeof ScreenAiStricterParentRuleInputBaseSchema>;

export const ScreenAiStricterParentRuleInputSchema = withParser(
  ScreenAiStricterParentRuleInputBaseSchema.pipe(
    Schema.filter(
      (input) =>
        screenAiStricterParentRuleInputIsReady(input) ||
        'Expected stricter parent rule proof input to use a stricter enabled parent rule and dry-run local AI decision'
    )
  )
);

const ScreenAiStricterParentRuleProofBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  proofId: ScreenAiStricterParentRuleProofIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceProof: NonEmptyStringSchema,
  sourceLocalAiAction: PolicyActionSchema,
  stricterParentRuleAction: PolicyActionSchema,
  finalAction: PolicyActionSchema,
  sourceDecision: PolicyDecisionSchema,
  stricterParentRule: PolicyRuleSchema,
  finalDecision: PolicyDecisionSchema,
  claimBoundaries: ScreenAiStricterParentRuleClaimBoundarySchema,
});

type ScreenAiStricterParentRuleProofCandidate = Infer<typeof ScreenAiStricterParentRuleProofBaseSchema>;

export const ScreenAiStricterParentRuleProofSchema = withParser(
  ScreenAiStricterParentRuleProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        screenAiStricterParentRuleProofIsHonest(proof) ||
        'Expected screen AI policy proof to preserve stricter parent rule over local AI output'
    )
  )
);

export type ScreenAiStricterParentRuleProof = Infer<typeof ScreenAiStricterParentRuleProofSchema>;

export function buildScreenAiStricterParentRuleProof(input: unknown): ScreenAiStricterParentRuleProof {
  const parsed = ScreenAiStricterParentRuleInputSchema.parse(input);
  const finalAction = selectStricterPolicyAction(parsed.stricterParentRule.action, parsed.sourceDecision.action);
  return ScreenAiStricterParentRuleProofSchema.parse({
    schemaVersion: parsed.schemaVersion,
    proofId: parsed.proofId,
    generatedAt: parsed.generatedAt,
    sourceProof: parsed.sourceProof,
    sourceLocalAiAction: parsed.sourceDecision.action,
    stricterParentRuleAction: parsed.stricterParentRule.action,
    finalAction,
    sourceDecision: parsed.sourceDecision,
    stricterParentRule: parsed.stricterParentRule,
    finalDecision: PolicyDecisionSchema.parse({
      ...parsed.sourceDecision,
      decisionId: `${parsed.sourceDecision.decisionId}-stricter-parent-rule`,
      action: finalAction,
      reasonCodes: [parsed.stricterParentRule.reasonCode, ...parsed.sourceDecision.reasonCodes],
      ruleIds: [parsed.stricterParentRule.ruleId, ...parsed.sourceDecision.ruleIds],
      dryRun: true,
      enforcementHandoffState: 'disabled',
    }),
    claimBoundaries: parsed.claimBoundaries,
  });
}

function screenAiStricterParentRuleInputIsReady(input: ScreenAiStricterParentRuleInputCandidate): boolean {
  return (
    input.sourceDecision.dryRun &&
    input.sourceDecision.enforcementHandoffState !== 'handed-off' &&
    input.sourceDecision.localAiResultId !== null &&
    input.stricterParentRule.enabled &&
    comparePolicyActionStrictness(input.stricterParentRule.action, input.sourceDecision.action) > 0 &&
    input.expectedFinalAction ===
      selectStricterPolicyAction(input.stricterParentRule.action, input.sourceDecision.action)
  );
}

function screenAiStricterParentRuleProofIsHonest(proof: ScreenAiStricterParentRuleProofCandidate): boolean {
  return (
    proof.finalAction === proof.stricterParentRule.action &&
    proof.finalDecision.action === proof.stricterParentRule.action &&
    proof.finalDecision.localAiResultId === proof.sourceDecision.localAiResultId &&
    proof.finalDecision.evidenceReferences.length === proof.sourceDecision.evidenceReferences.length &&
    proof.finalDecision.ruleIds.includes(proof.stricterParentRule.ruleId) &&
    proof.finalDecision.dryRun &&
    proof.finalDecision.enforcementHandoffState !== 'handed-off' &&
    Object.values(proof.claimBoundaries).every((claim) => claim === false)
  );
}


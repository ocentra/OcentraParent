import { type Infer, Schema, withParser, brandedNonEmptyStringSchema, NonEmptyStringSchema } from './effect';
import {
  generatedScreenAiStricterParentRuleInputIsReady,
  generatedScreenAiStricterParentRuleProofIsHonest,
  selectGeneratedStricterPolicyAction,
} from './generated-policy-control-helpers';
import { PolicyActionSchema, PolicyDecisionSchema, PolicyRuleSchema } from './policy-contracts';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './family-reference-primitives';
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
  const finalAction = selectGeneratedStricterPolicyAction(parsed.stricterParentRule.action, parsed.sourceDecision.action);
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
  return generatedScreenAiStricterParentRuleInputIsReady(input);
}

function screenAiStricterParentRuleProofIsHonest(proof: ScreenAiStricterParentRuleProofCandidate): boolean {
  return generatedScreenAiStricterParentRuleProofIsHonest({
    ...proof,
    stricterParentRuleAction: proof.stricterParentRule.action,
  });
}

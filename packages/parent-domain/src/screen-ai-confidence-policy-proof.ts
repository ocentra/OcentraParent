import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { LocalAiSafetyResultSchema, type LocalAiSafetyResult } from './local-ai';
import { LocalAiConfidenceSchema, LocalAiDegradedState, LocalAiUnknownState } from './local-ai-primitives';
import {
  PolicyAction,
  PolicyActionSchema,
  PolicyDecisionHandoffState,
  type PolicyDecisionHandoffStateSchema,
  PolicyDecisionSchema,
  PolicyRuleSchema,
  comparePolicyActionStrictness,
  type PolicyAction as PolicyActionType,
  type PolicyDecision,
} from './policy';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './reference-primitives';

const NonEmptyConfidencePolicyText = Schema.String.pipe(Schema.minLength(1));

export const ScreenAiConfidencePolicyProofIdSchema = NonEmptyConfidencePolicyText.pipe(
  Schema.brand('ScreenAiConfidencePolicyProofId')
);

export const ScreenAiConfidencePolicyStateSchema = withParser(
  Schema.Literal('trusted-confidence', 'low-confidence-fallback', 'degraded-fallback')
);

export const ScreenAiConfidencePolicyReasonSchema = withParser(
  Schema.Literal(
    'local-ai-result-schema-valid',
    'confidence-threshold-applied',
    'parent-rule-cited',
    'stricter-parent-rule-preserved',
    'low-confidence-cannot-allow',
    'degraded-output-cannot-enforce',
    'dry-run-policy-only',
    'remote-ai-not-used',
    'raw-evidence-not-embedded'
  )
);

const requiredProofReasons = [
  'local-ai-result-schema-valid',
  'confidence-threshold-applied',
  'parent-rule-cited',
  'dry-run-policy-only',
  'remote-ai-not-used',
  'raw-evidence-not-embedded',
] as const satisfies ReadonlyArray<Infer<typeof ScreenAiConfidencePolicyReasonSchema>>;

const ScreenAiConfidencePolicyReasonsSchema = Schema.Array(ScreenAiConfidencePolicyReasonSchema).pipe(
  Schema.filter((reasons) => requiredProofReasons.every((reason) => reasons.includes(reason)))
);

export const ScreenAiConfidencePolicyClaimBoundarySchema = withParser(
  Schema.Struct({
    remoteAiUsed: Schema.Literal(false),
    apiAiUsed: Schema.Literal(false),
    policyAuthorityClaimed: Schema.Literal(false),
    enforcementClaimed: Schema.Literal(false),
    rawEvidenceEmbedded: Schema.Literal(false),
    modelQualityClaimed: Schema.Literal(false),
  })
);

const ScreenAiConfidencePolicyInputBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  proofId: ScreenAiConfidencePolicyProofIdSchema,
  evaluatedAt: ParentTimestampSchema,
  localAiResult: LocalAiSafetyResultSchema,
  parentRule: PolicyRuleSchema,
  sourcePolicyDecision: PolicyDecisionSchema,
  minimumConfidence: LocalAiConfidenceSchema,
  claimBoundaries: ScreenAiConfidencePolicyClaimBoundarySchema,
});

type ScreenAiConfidencePolicyInputCandidate = Infer<typeof ScreenAiConfidencePolicyInputBaseSchema>;

export const ScreenAiConfidencePolicyInputSchema = withParser(
  ScreenAiConfidencePolicyInputBaseSchema.pipe(
    Schema.filter(
      (input) =>
        screenAiConfidencePolicyInputIsReady(input) ||
        'Expected evidence-cited local AI result, matching parent rule, dry-run source policy, and no remote/enforcement claims'
    )
  )
);

const ScreenAiConfidencePolicyProofBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  proofId: ScreenAiConfidencePolicyProofIdSchema,
  evaluatedAt: ParentTimestampSchema,
  confidenceState: ScreenAiConfidencePolicyStateSchema,
  minimumConfidence: LocalAiConfidenceSchema,
  observedConfidence: LocalAiConfidenceSchema,
  aiSuggestedAction: PolicyActionSchema,
  parentRuleAction: PolicyActionSchema,
  selectedPolicyAction: PolicyActionSchema,
  policyDecision: PolicyDecisionSchema,
  proofReasons: ScreenAiConfidencePolicyReasonsSchema,
  claimBoundaries: ScreenAiConfidencePolicyClaimBoundarySchema,
});

type ScreenAiConfidencePolicyProofCandidate = Infer<typeof ScreenAiConfidencePolicyProofBaseSchema>;

export const ScreenAiConfidencePolicyProofSchema = withParser(
  ScreenAiConfidencePolicyProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        screenAiConfidencePolicyProofIsHonest(proof) ||
        'Expected dry-run screen AI confidence policy proof to preserve stricter parent rules and avoid AI/enforcement overclaims'
    )
  )
);

export type ScreenAiConfidencePolicyInput = Infer<typeof ScreenAiConfidencePolicyInputSchema>;
export type ScreenAiConfidencePolicyProof = Infer<typeof ScreenAiConfidencePolicyProofSchema>;

export function buildScreenAiConfidencePolicyProof(input: unknown): ScreenAiConfidencePolicyProof {
  const parsed = ScreenAiConfidencePolicyInputSchema.parse(input);
  const confidenceState = classifyConfidenceState(parsed.localAiResult, parsed.minimumConfidence);
  const selectedPolicyAction = selectConfidenceBoundPolicyAction(parsed.parentRule.action, parsed.localAiResult);
  const policyDecision = buildPolicyDecision(parsed, selectedPolicyAction, confidenceState);
  return ScreenAiConfidencePolicyProofSchema.parse({
    schemaVersion: parsed.schemaVersion,
    proofId: parsed.proofId,
    evaluatedAt: parsed.evaluatedAt,
    confidenceState,
    minimumConfidence: parsed.minimumConfidence,
    observedConfidence: parsed.localAiResult.confidence,
    aiSuggestedAction: parsed.localAiResult.action,
    parentRuleAction: parsed.parentRule.action,
    selectedPolicyAction,
    policyDecision,
    proofReasons: buildProofReasons(parsed, confidenceState, selectedPolicyAction),
    claimBoundaries: parsed.claimBoundaries,
  });
}

function screenAiConfidencePolicyInputIsReady(input: ScreenAiConfidencePolicyInputCandidate): boolean {
  return (
    input.localAiResult.evidenceReferences.length > 0 &&
    input.sourcePolicyDecision.evidenceReferences.length > 0 &&
    input.localAiResult.parentRuleReferences.includes(input.parentRule.ruleId) &&
    input.sourcePolicyDecision.ruleIds.includes(input.parentRule.ruleId) &&
    policyDecisionHandoffIsNonEnforcing(input.sourcePolicyDecision.enforcementHandoffState) &&
    input.sourcePolicyDecision.dryRun
  );
}

function classifyConfidenceState(
  localAiResult: LocalAiSafetyResult,
  minimumConfidence: number
): Infer<typeof ScreenAiConfidencePolicyStateSchema> {
  if (localAiResult.degradedState !== LocalAiDegradedState.None) {
    return 'degraded-fallback';
  }
  if (
    localAiResult.confidence < minimumConfidence ||
    localAiResult.unknownState === LocalAiUnknownState.LowConfidence
  ) {
    return 'low-confidence-fallback';
  }
  return 'trusted-confidence';
}

function selectConfidenceBoundPolicyAction(
  parentRuleAction: PolicyActionType,
  localAiResult: LocalAiSafetyResult
): PolicyActionType {
  const aiAction = localAiResultIsPolicyUsable(localAiResult) ? localAiResult.action : PolicyAction.Unknown;
  return comparePolicyActionStrictness(parentRuleAction, aiAction) >= 0 ? parentRuleAction : aiAction;
}

function localAiResultIsPolicyUsable(localAiResult: LocalAiSafetyResult): boolean {
  return (
    localAiResult.degradedState === LocalAiDegradedState.None &&
    localAiResult.unknownState !== LocalAiUnknownState.LowConfidence &&
    localAiResult.confidence > 0
  );
}

function buildPolicyDecision(
  input: ScreenAiConfidencePolicyInput,
  selectedPolicyAction: PolicyActionType,
  confidenceState: Infer<typeof ScreenAiConfidencePolicyStateSchema>
): PolicyDecision {
  return PolicyDecisionSchema.parse({
    schemaVersion: input.schemaVersion,
    decisionId: `${input.proofId}-policy-dry-run`,
    action: selectedPolicyAction,
    reasonCodes: buildReasonCodes(input, confidenceState),
    evidenceReferences: input.localAiResult.evidenceReferences,
    ruleIds: [input.parentRule.ruleId],
    localAiResultId: input.sourcePolicyDecision.localAiResultId,
    dryRun: true,
    enforcementHandoffState: PolicyDecisionHandoffState.Disabled,
    expiresAt: input.localAiResult.expiresAt,
  });
}

function buildReasonCodes(
  input: ScreenAiConfidencePolicyInput,
  confidenceState: Infer<typeof ScreenAiConfidencePolicyStateSchema>
) {
  const reasonCodes: unknown[] = [input.parentRule.reasonCode];
  if (confidenceState === 'low-confidence-fallback') {
    reasonCodes.push('screen-ai-low-confidence-fallback');
  }
  if (confidenceState === 'degraded-fallback') {
    reasonCodes.push('screen-ai-degraded-fallback');
  }
  return reasonCodes;
}

function buildProofReasons(
  input: ScreenAiConfidencePolicyInput,
  confidenceState: Infer<typeof ScreenAiConfidencePolicyStateSchema>,
  selectedPolicyAction: PolicyActionType
) {
  const reasons: Array<Infer<typeof ScreenAiConfidencePolicyReasonSchema>> = [
    'local-ai-result-schema-valid',
    'confidence-threshold-applied',
    'parent-rule-cited',
    'dry-run-policy-only',
    'remote-ai-not-used',
    'raw-evidence-not-embedded',
  ];
  if (comparePolicyActionStrictness(input.parentRule.action, input.localAiResult.action) > 0) {
    reasons.push('stricter-parent-rule-preserved');
  }
  if (confidenceState === 'low-confidence-fallback' && selectedPolicyAction !== PolicyAction.Allow) {
    reasons.push('low-confidence-cannot-allow');
  }
  if (confidenceState === 'degraded-fallback') {
    reasons.push('degraded-output-cannot-enforce');
  }
  return reasons;
}

function screenAiConfidencePolicyProofIsHonest(proof: ScreenAiConfidencePolicyProofCandidate): boolean {
  return (
    proof.policyDecision.dryRun &&
    policyDecisionHandoffIsNonEnforcing(proof.policyDecision.enforcementHandoffState) &&
    proof.claimBoundaries.remoteAiUsed === false &&
    proof.claimBoundaries.apiAiUsed === false &&
    proof.claimBoundaries.policyAuthorityClaimed === false &&
    proof.claimBoundaries.enforcementClaimed === false &&
    proof.claimBoundaries.rawEvidenceEmbedded === false &&
    proof.claimBoundaries.modelQualityClaimed === false &&
    lowConfidenceDoesNotAllow(proof)
  );
}

function lowConfidenceDoesNotAllow(proof: ScreenAiConfidencePolicyProofCandidate): boolean {
  return proof.confidenceState !== 'low-confidence-fallback' || proof.selectedPolicyAction !== PolicyAction.Allow;
}

function policyDecisionHandoffIsNonEnforcing(state: Infer<typeof PolicyDecisionHandoffStateSchema>): boolean {
  return state === PolicyDecisionHandoffState.Disabled || state === PolicyDecisionHandoffState.NotRequested;
}

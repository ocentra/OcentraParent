import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import { PolicyCompilerCapabilityState } from '@ocentra-parent/schema-domain/policy-compiler';
import {
  type SocialParentPolicyCompilerInput,
  SocialParentPolicyCompilerInputSchema,
  type SocialParentPolicyDecisionCandidate,
  SocialParentPolicyDecisionCandidateSchema,
} from '@ocentra-parent/schema-domain/social-policy-compiler';
import {
  SocialParentPolicyActionCandidateSchema,
  SocialParentPolicyConfidenceSchema,
  SocialParentPolicyDecisionCandidateIdSchema,
  SocialParentPolicyReasonCodesSchema,
} from '@ocentra-parent/schema-domain/social-policy-compiler-values';

const SocialParentPolicyCompileRequestSchema = withParser(
  Schema.Struct({
    input: SocialParentPolicyCompilerInputSchema,
    decisionCandidateId: SocialParentPolicyDecisionCandidateIdSchema,
    decidedAt: ParentTimestampSchema,
    expiresAt: Schema.Union(ParentTimestampSchema, Schema.Null),
    actionCandidate: SocialParentPolicyActionCandidateSchema,
    reasonCodes: SocialParentPolicyReasonCodesSchema,
    confidence: SocialParentPolicyConfidenceSchema,
    fallbackUsed: Schema.Boolean,
    parentApprovalRequired: Schema.Boolean,
  })
);

type SocialParentPolicyCompileRequest = Infer<typeof SocialParentPolicyCompileRequestSchema>;

export function compileSocialParentPolicyCandidate(
  request: SocialParentPolicyCompileRequest
): SocialParentPolicyDecisionCandidate {
  const parsed = SocialParentPolicyCompileRequestSchema.parse(request);
  const input = parsed.input;

  return SocialParentPolicyDecisionCandidateSchema.parse({
    schemaVersion: input.schemaVersion,
    decisionCandidateId: parsed.decisionCandidateId,
    compileRequestId: input.compileRequestId,
    decidedAt: parsed.decidedAt,
    expiresAt: parsed.expiresAt,
    policyVersionRef: input.policyVersionRef,
    targetKind: input.targetKind,
    sourceEvidenceRefs: input.sourceEvidenceRefs,
    signalSetRefs: input.signalSetRefs,
    parentRuleRefs: input.parentRuleRefs,
    scheduleContextRefs: input.scheduleContextRefs,
    timeBudgetContextRefs: input.timeBudgetContextRefs,
    scheduleState: input.scheduleState,
    timeBudgetState: input.timeBudgetState,
    actionCandidate: parsed.actionCandidate,
    reasonCodes: parsed.reasonCodes,
    confidence: parsed.confidence,
    compilerMode: input.compilerMode,
    fallbackUsed: parsed.fallbackUsed,
    parentApprovalRequired: parsed.parentApprovalRequired,
    compilerCapabilityState: compilerCapabilityStateForMode(input.compilerMode),
    finalPolicyDecisionClaimed: false,
    runtimeGateExecutedClaimed: false,
    uiRenderedClaimed: false,
    enforcementClaimed: false,
    nativeAppControlClaimed: false,
    platformConnectorClaimed: false,
    rawSignalPayloadStored: false,
    rawModelTextUsed: false,
  });
}

function compilerCapabilityStateForMode(mode: SocialParentPolicyCompilerInput['compilerMode']) {
  switch (mode) {
    case 'contract-only':
      return PolicyCompilerCapabilityState.Supported;
    case 'manual-required':
      return PolicyCompilerCapabilityState.ManualRequired;
    case 'unavailable':
      return PolicyCompilerCapabilityState.Unsupported;
  }
}

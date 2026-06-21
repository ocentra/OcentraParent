import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  PolicyCompilerCapabilityState,
} from '@ocentra-parent/schema-domain/policy-compiler';
import {
  type BrowserGamePolicyCompilerInput,
  BrowserGamePolicyCompilerInputSchema,
  type BrowserGamePolicyDecisionCandidate,
  BrowserGamePolicyDecisionCandidateSchema,
} from '@ocentra-parent/schema-domain/browser-game-policy-compiler';
import {
  BrowserGamePolicyActionCandidateSchema,
  BrowserGamePolicyConfidenceSchema,
  BrowserGamePolicyDecisionCandidateIdSchema,
  BrowserGamePolicyReasonCodesSchema,
} from '@ocentra-parent/schema-domain/browser-game-policy-compiler-values';

const BrowserGamePolicyCompileRequestSchema = withParser(
  Schema.Struct({
    input: BrowserGamePolicyCompilerInputSchema,
    decisionCandidateId: BrowserGamePolicyDecisionCandidateIdSchema,
    decidedAt: ParentTimestampSchema,
    expiresAt: Schema.Union(ParentTimestampSchema, Schema.Null),
    actionCandidate: BrowserGamePolicyActionCandidateSchema,
    reasonCodes: BrowserGamePolicyReasonCodesSchema,
    confidence: BrowserGamePolicyConfidenceSchema,
    fallbackUsed: Schema.Boolean,
    parentApprovalRequired: Schema.Boolean,
  })
);

type BrowserGamePolicyCompileRequest = Infer<typeof BrowserGamePolicyCompileRequestSchema>;

export function compileBrowserGamePolicyCandidate(
  request: BrowserGamePolicyCompileRequest
): BrowserGamePolicyDecisionCandidate {
  const parsed = BrowserGamePolicyCompileRequestSchema.parse(request);
  const input = parsed.input;

  return BrowserGamePolicyDecisionCandidateSchema.parse({
    schemaVersion: input.schemaVersion,
    decisionCandidateId: parsed.decisionCandidateId,
    compileRequestId: input.compileRequestId,
    decidedAt: parsed.decidedAt,
    expiresAt: parsed.expiresAt,
    policyVersionRef: input.policyVersionRef,
    targetKind: input.targetKind,
    sourceEvidenceRefs: input.sourceEvidenceRefs,
    analysisRefs: input.analysisRefs,
    mobileCapabilityRefs: input.mobileCapabilityRefs,
    parentRuleRefs: input.parentRuleRefs,
    scheduleContextRefs: input.scheduleContextRefs,
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
    nativeGameControlClaimed: false,
    cloudFrameAnalysisClaimed: false,
    rawGamePayloadStored: false,
    rawModelTextUsed: false,
  });
}

function compilerCapabilityStateForMode(mode: BrowserGamePolicyCompilerInput['compilerMode']) {
  switch (mode) {
    case 'contract-only':
      return PolicyCompilerCapabilityState.Supported;
    case 'manual-required':
      return PolicyCompilerCapabilityState.ManualRequired;
    case 'unavailable':
      return PolicyCompilerCapabilityState.Unsupported;
  }
}

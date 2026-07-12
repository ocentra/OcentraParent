/* generated from crates/browser-core/src/social_policy_candidate_compiler.rs */

import type {
  SocialParentPolicyCompilerInput,
  SocialParentPolicyDecisionCandidate,
} from './social_policy_compiler_contract';

export const SocialPolicyDecisionTemplates = {
  'contract-only': {
    compilerCapabilityState: 'supported',
    finalPolicyDecisionClaimed: false,
    runtimeGateExecutedClaimed: false,
    uiRenderedClaimed: false,
    enforcementClaimed: false,
    nativeAppControlClaimed: false,
    platformConnectorClaimed: false,
    rawSignalPayloadStored: false,
    rawModelTextUsed: false,
  },
  'manual-required': {
    compilerCapabilityState: 'manual-required',
    finalPolicyDecisionClaimed: false,
    runtimeGateExecutedClaimed: false,
    uiRenderedClaimed: false,
    enforcementClaimed: false,
    nativeAppControlClaimed: false,
    platformConnectorClaimed: false,
    rawSignalPayloadStored: false,
    rawModelTextUsed: false,
  },
  unavailable: {
    compilerCapabilityState: 'unsupported',
    finalPolicyDecisionClaimed: false,
    runtimeGateExecutedClaimed: false,
    uiRenderedClaimed: false,
    enforcementClaimed: false,
    nativeAppControlClaimed: false,
    platformConnectorClaimed: false,
    rawSignalPayloadStored: false,
    rawModelTextUsed: false,
  },
} as const;

export type SocialPolicyCompilerMode = 'contract-only' | 'manual-required' | 'unavailable';

export type SocialPolicyDecisionTemplate =
  (typeof SocialPolicyDecisionTemplates)[keyof typeof SocialPolicyDecisionTemplates];

type SocialParentPolicyCompileRequest = {
  readonly input: SocialParentPolicyCompilerInput;
  readonly decisionCandidateId: string;
  readonly decidedAt: string;
  readonly expiresAt: string | null;
  readonly actionCandidate: SocialParentPolicyDecisionCandidate['actionCandidate'];
  readonly reasonCodes: readonly string[];
  readonly confidence: SocialParentPolicyDecisionCandidate['confidence'];
  readonly fallbackUsed: boolean;
  readonly parentApprovalRequired: boolean;
};

export function socialPolicyDecisionTemplate(mode: SocialPolicyCompilerMode): SocialPolicyDecisionTemplate {
  return SocialPolicyDecisionTemplates[mode];
}

export function buildGeneratedSocialPolicyDecisionCandidate(request: SocialParentPolicyCompileRequest) {
  const input = request.input;
  const decisionTemplate = socialPolicyDecisionTemplate(input.compilerMode);

  return {
    schemaVersion: input.schemaVersion,
    decisionCandidateId: request.decisionCandidateId,
    compileRequestId: input.compileRequestId,
    decidedAt: request.decidedAt,
    expiresAt: request.expiresAt,
    policyVersionRef: input.policyVersionRef,
    targetKind: input.targetKind,
    sourceEvidenceRefs: [...input.sourceEvidenceRefs],
    signalSetRefs: [...input.signalSetRefs],
    parentRuleRefs: [...input.parentRuleRefs],
    scheduleContextRefs: [...input.scheduleContextRefs],
    timeBudgetContextRefs: [...input.timeBudgetContextRefs],
    scheduleState: input.scheduleState,
    timeBudgetState: input.timeBudgetState,
    actionCandidate: request.actionCandidate,
    reasonCodes: [...request.reasonCodes],
    confidence: request.confidence,
    compilerMode: input.compilerMode,
    fallbackUsed: request.fallbackUsed,
    parentApprovalRequired: request.parentApprovalRequired,
    ...decisionTemplate,
  };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SocialPolicyCompilerMode {
    ContractOnly,
    ManualRequired,
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SocialPolicyDecisionTemplate {
    pub compiler_capability_state: &'static str,
    pub final_policy_decision_claimed: bool,
    pub runtime_gate_executed_claimed: bool,
    pub ui_rendered_claimed: bool,
    pub enforcement_claimed: bool,
    pub native_app_control_claimed: bool,
    pub platform_connector_claimed: bool,
    pub raw_signal_payload_stored: bool,
    pub raw_model_text_used: bool,
}

const CONTRACT_ONLY_TEMPLATE: SocialPolicyDecisionTemplate = SocialPolicyDecisionTemplate {
    compiler_capability_state: "supported",
    final_policy_decision_claimed: false,
    runtime_gate_executed_claimed: false,
    ui_rendered_claimed: false,
    enforcement_claimed: false,
    native_app_control_claimed: false,
    platform_connector_claimed: false,
    raw_signal_payload_stored: false,
    raw_model_text_used: false,
};

const MANUAL_REQUIRED_TEMPLATE: SocialPolicyDecisionTemplate = SocialPolicyDecisionTemplate {
    compiler_capability_state: "manual-required",
    ..CONTRACT_ONLY_TEMPLATE
};

const UNAVAILABLE_TEMPLATE: SocialPolicyDecisionTemplate = SocialPolicyDecisionTemplate {
    compiler_capability_state: "unsupported",
    ..CONTRACT_ONLY_TEMPLATE
};

pub fn evaluate_social_policy_candidate(
    mode: SocialPolicyCompilerMode,
) -> SocialPolicyDecisionTemplate {
    match mode {
        SocialPolicyCompilerMode::ContractOnly => CONTRACT_ONLY_TEMPLATE,
        SocialPolicyCompilerMode::ManualRequired => MANUAL_REQUIRED_TEMPLATE,
        SocialPolicyCompilerMode::Unavailable => UNAVAILABLE_TEMPLATE,
    }
}

pub fn social_policy_candidate_compiler_typescript() -> String {
    SOCIAL_POLICY_CANDIDATE_COMPILER_TYPESCRIPT.to_string()
}

const SOCIAL_POLICY_CANDIDATE_COMPILER_TYPESCRIPT: &str = r#"/* generated from crates/browser-core/src/social_policy_candidate_compiler.rs */

import type {
  SocialParentPolicyCompilerInput,
  SocialParentPolicyDecisionCandidate,
} from '@ocentra-parent/schema-domain/social-policy-compiler';

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

export type SocialPolicyCompilerMode =
  | 'contract-only'
  | 'manual-required'
  | 'unavailable';

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

export function socialPolicyDecisionTemplate(
  mode: SocialPolicyCompilerMode
): SocialPolicyDecisionTemplate {
  return SocialPolicyDecisionTemplates[mode];
}

export function buildGeneratedSocialPolicyDecisionCandidate(
  request: SocialParentPolicyCompileRequest
) {
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
"#;

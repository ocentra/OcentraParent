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
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../browser-core-generated/social_policy_candidate_compiler.ts"
    ))
    .to_string()
}

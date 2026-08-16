#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrowserGamePolicyCompilerMode {
    ContractOnly,
    ManualRequired,
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BrowserGamePolicyDecisionTemplate {
    pub compiler_capability_state: &'static str,
    pub final_policy_decision_claimed: bool,
    pub runtime_gate_executed_claimed: bool,
    pub ui_rendered_claimed: bool,
    pub enforcement_claimed: bool,
    pub native_game_control_claimed: bool,
    pub cloud_frame_analysis_claimed: bool,
    pub raw_game_payload_stored: bool,
    pub raw_model_text_used: bool,
}

const CONTRACT_ONLY_TEMPLATE: BrowserGamePolicyDecisionTemplate =
    BrowserGamePolicyDecisionTemplate {
        compiler_capability_state: "supported",
        final_policy_decision_claimed: false,
        runtime_gate_executed_claimed: false,
        ui_rendered_claimed: false,
        enforcement_claimed: false,
        native_game_control_claimed: false,
        cloud_frame_analysis_claimed: false,
        raw_game_payload_stored: false,
        raw_model_text_used: false,
    };

const MANUAL_REQUIRED_TEMPLATE: BrowserGamePolicyDecisionTemplate =
    BrowserGamePolicyDecisionTemplate {
        compiler_capability_state: "manual-required",
        ..CONTRACT_ONLY_TEMPLATE
    };

const UNAVAILABLE_TEMPLATE: BrowserGamePolicyDecisionTemplate = BrowserGamePolicyDecisionTemplate {
    compiler_capability_state: "unsupported",
    ..CONTRACT_ONLY_TEMPLATE
};

pub fn evaluate_browser_game_policy_candidate(
    mode: BrowserGamePolicyCompilerMode,
) -> BrowserGamePolicyDecisionTemplate {
    match mode {
        BrowserGamePolicyCompilerMode::ContractOnly => CONTRACT_ONLY_TEMPLATE,
        BrowserGamePolicyCompilerMode::ManualRequired => MANUAL_REQUIRED_TEMPLATE,
        BrowserGamePolicyCompilerMode::Unavailable => UNAVAILABLE_TEMPLATE,
    }
}

pub fn browser_game_policy_candidate_compiler_typescript() -> String {
    [
        "/* generated from crates/browser-core/src/browser_game_policy_candidate_compiler.rs */",
        "",
        "export const BrowserGamePolicyDecisionTemplates = {",
        "  'contract-only': {",
        "    compilerCapabilityState: 'supported',",
        "    finalPolicyDecisionClaimed: false,",
        "    runtimeGateExecutedClaimed: false,",
        "    uiRenderedClaimed: false,",
        "    enforcementClaimed: false,",
        "    nativeGameControlClaimed: false,",
        "    cloudFrameAnalysisClaimed: false,",
        "    rawGamePayloadStored: false,",
        "    rawModelTextUsed: false,",
        "  },",
        "  'manual-required': {",
        "    compilerCapabilityState: 'manual-required',",
        "    finalPolicyDecisionClaimed: false,",
        "    runtimeGateExecutedClaimed: false,",
        "    uiRenderedClaimed: false,",
        "    enforcementClaimed: false,",
        "    nativeGameControlClaimed: false,",
        "    cloudFrameAnalysisClaimed: false,",
        "    rawGamePayloadStored: false,",
        "    rawModelTextUsed: false,",
        "  },",
        "  unavailable: {",
        "    compilerCapabilityState: 'unsupported',",
        "    finalPolicyDecisionClaimed: false,",
        "    runtimeGateExecutedClaimed: false,",
        "    uiRenderedClaimed: false,",
        "    enforcementClaimed: false,",
        "    nativeGameControlClaimed: false,",
        "    cloudFrameAnalysisClaimed: false,",
        "    rawGamePayloadStored: false,",
        "    rawModelTextUsed: false,",
        "  },",
        "} as const;",
        "",
        "export type BrowserGamePolicyCompilerMode =",
        "  | 'contract-only'",
        "  | 'manual-required'",
        "  | 'unavailable';",
        "",
        "export type BrowserGamePolicyDecisionTemplate =",
        "  (typeof BrowserGamePolicyDecisionTemplates)[keyof typeof BrowserGamePolicyDecisionTemplates];",
        "",
        "export function browserGamePolicyDecisionTemplate(",
        "  mode: BrowserGamePolicyCompilerMode",
        "): BrowserGamePolicyDecisionTemplate {",
        "  return BrowserGamePolicyDecisionTemplates[mode];",
        "}",
        "",
    ]
    .join("\n")
}

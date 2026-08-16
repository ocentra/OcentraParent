#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrowserGameHiddenAnalysisCapabilityState {
    Available,
    DisabledByPolicy,
    ProfileProofMissing,
    ManualRequired,
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BrowserGameHiddenAnalysisDecisionTemplate {
    pub state: &'static str,
    pub confidence: &'static str,
    pub reason_codes: &'static [&'static str],
}

const SAFE_LOADING_REASONS: &[&str] = &[
    "ocentra-owned-profile",
    "separate-from-child-profile",
    "bounded-retention",
];

const POLICY_DISABLED_DECISION: BrowserGameHiddenAnalysisDecisionTemplate =
    BrowserGameHiddenAnalysisDecisionTemplate {
        state: "disabled-by-policy",
        confidence: "low",
        reason_codes: &["policy-disabled"],
    };

const AVAILABLE_DECISION: BrowserGameHiddenAnalysisDecisionTemplate =
    BrowserGameHiddenAnalysisDecisionTemplate {
        state: "loading",
        confidence: "medium",
        reason_codes: SAFE_LOADING_REASONS,
    };

const PROFILE_PROOF_MISSING_DECISION: BrowserGameHiddenAnalysisDecisionTemplate =
    BrowserGameHiddenAnalysisDecisionTemplate {
        state: "profile-proof-missing",
        confidence: "low",
        reason_codes: &["profile-proof-missing"],
    };

const MANUAL_REQUIRED_DECISION: BrowserGameHiddenAnalysisDecisionTemplate =
    BrowserGameHiddenAnalysisDecisionTemplate {
        state: "manual-required",
        confidence: "low",
        reason_codes: &["manual-required"],
    };

const UNAVAILABLE_DECISION: BrowserGameHiddenAnalysisDecisionTemplate =
    BrowserGameHiddenAnalysisDecisionTemplate {
        state: "unavailable",
        confidence: "low",
        reason_codes: &["unavailable"],
    };

pub fn evaluate_browser_game_hidden_analysis_loader(
    policy_allows_hidden_analysis: bool,
    capability_state: BrowserGameHiddenAnalysisCapabilityState,
) -> BrowserGameHiddenAnalysisDecisionTemplate {
    if !policy_allows_hidden_analysis {
        return POLICY_DISABLED_DECISION;
    }

    match capability_state {
        BrowserGameHiddenAnalysisCapabilityState::Available => AVAILABLE_DECISION,
        BrowserGameHiddenAnalysisCapabilityState::DisabledByPolicy => POLICY_DISABLED_DECISION,
        BrowserGameHiddenAnalysisCapabilityState::ProfileProofMissing => {
            PROFILE_PROOF_MISSING_DECISION
        }
        BrowserGameHiddenAnalysisCapabilityState::ManualRequired => MANUAL_REQUIRED_DECISION,
        BrowserGameHiddenAnalysisCapabilityState::Unavailable => UNAVAILABLE_DECISION,
    }
}

pub fn browser_game_hidden_analysis_loader_plan_typescript() -> String {
    [
        "/* generated from crates/browser-core/src/browser_game_hidden_analysis_loader.rs */",
        "",
        "export const BrowserGameHiddenAnalysisDecisionTemplates = {",
        "  'policy-disabled': {",
        "    state: 'disabled-by-policy',",
        "    confidence: 'low',",
        "    reasonCodes: ['policy-disabled'],",
        "  },",
        "  available: {",
        "    state: 'loading',",
        "    confidence: 'medium',",
        "    reasonCodes: ['ocentra-owned-profile', 'separate-from-child-profile', 'bounded-retention'],",
        "  },",
        "  'profile-proof-missing': {",
        "    state: 'profile-proof-missing',",
        "    confidence: 'low',",
        "    reasonCodes: ['profile-proof-missing'],",
        "  },",
        "  'manual-required': {",
        "    state: 'manual-required',",
        "    confidence: 'low',",
        "    reasonCodes: ['manual-required'],",
        "  },",
        "  unavailable: {",
        "    state: 'unavailable',",
        "    confidence: 'low',",
        "    reasonCodes: ['unavailable'],",
        "  },",
        "} as const;",
        "",
        "export type BrowserGameHiddenAnalysisCapabilityState =",
        "  | 'available'",
        "  | 'disabled-by-policy'",
        "  | 'profile-proof-missing'",
        "  | 'manual-required'",
        "  | 'unavailable';",
        "",
        "export type BrowserGameHiddenAnalysisDecisionTemplate =",
        "  (typeof BrowserGameHiddenAnalysisDecisionTemplates)[keyof typeof BrowserGameHiddenAnalysisDecisionTemplates];",
        "",
        "export function browserGameHiddenAnalysisDecisionTemplate(",
        "  policyAllowsHiddenAnalysis: boolean,",
        "  capabilityState: BrowserGameHiddenAnalysisCapabilityState",
        "): BrowserGameHiddenAnalysisDecisionTemplate {",
        "  if (!policyAllowsHiddenAnalysis || capabilityState === 'disabled-by-policy') {",
        "    return BrowserGameHiddenAnalysisDecisionTemplates['policy-disabled'];",
        "  }",
        "",
        "  return BrowserGameHiddenAnalysisDecisionTemplates[capabilityState];",
        "}",
        "",
    ]
    .join("\n")
}

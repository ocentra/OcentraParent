#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrowserHiddenAnalysisLoaderCapabilityState {
    Available,
    DisabledByPolicy,
    ManualRequired,
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BrowserHiddenAnalysisLoaderDecisionTemplate {
    pub state: &'static str,
    pub degraded_reasons: &'static [&'static str],
    pub loaded_by_hidden_adapter: bool,
    pub loader_proof_ref: Option<&'static str>,
    pub page_body_captured: bool,
    pub transcript_text_captured: bool,
}

const LOADING_TEMPLATE: BrowserHiddenAnalysisLoaderDecisionTemplate =
    BrowserHiddenAnalysisLoaderDecisionTemplate {
        state: "loading",
        degraded_reasons: &[],
        loaded_by_hidden_adapter: false,
        loader_proof_ref: None,
        page_body_captured: false,
        transcript_text_captured: false,
    };

const DISABLED_BY_POLICY_TEMPLATE: BrowserHiddenAnalysisLoaderDecisionTemplate =
    BrowserHiddenAnalysisLoaderDecisionTemplate {
        state: "manual-required",
        degraded_reasons: &["disabled-by-policy"],
        loaded_by_hidden_adapter: false,
        loader_proof_ref: None,
        page_body_captured: false,
        transcript_text_captured: false,
    };

const MANUAL_REQUIRED_TEMPLATE: BrowserHiddenAnalysisLoaderDecisionTemplate =
    BrowserHiddenAnalysisLoaderDecisionTemplate {
        state: "manual-required",
        degraded_reasons: &["manual-required"],
        loaded_by_hidden_adapter: false,
        loader_proof_ref: None,
        page_body_captured: false,
        transcript_text_captured: false,
    };

const UNAVAILABLE_TEMPLATE: BrowserHiddenAnalysisLoaderDecisionTemplate =
    BrowserHiddenAnalysisLoaderDecisionTemplate {
        state: "manual-required",
        degraded_reasons: &["loader-unavailable"],
        loaded_by_hidden_adapter: false,
        loader_proof_ref: None,
        page_body_captured: false,
        transcript_text_captured: false,
    };

pub fn evaluate_browser_hidden_analysis_loader(
    policy_allows_hidden_analysis: bool,
    capability_state: BrowserHiddenAnalysisLoaderCapabilityState,
) -> BrowserHiddenAnalysisLoaderDecisionTemplate {
    if !policy_allows_hidden_analysis {
        return DISABLED_BY_POLICY_TEMPLATE;
    }

    match capability_state {
        BrowserHiddenAnalysisLoaderCapabilityState::Available => LOADING_TEMPLATE,
        BrowserHiddenAnalysisLoaderCapabilityState::DisabledByPolicy => DISABLED_BY_POLICY_TEMPLATE,
        BrowserHiddenAnalysisLoaderCapabilityState::ManualRequired => MANUAL_REQUIRED_TEMPLATE,
        BrowserHiddenAnalysisLoaderCapabilityState::Unavailable => UNAVAILABLE_TEMPLATE,
    }
}

pub fn browser_hidden_analysis_loader_typescript() -> String {
    [
        "/* generated from crates/browser-core/src/browser_hidden_analysis_loader.rs */",
        "",
        "export const BrowserHiddenAnalysisLoaderDecisionTemplates = {",
        "  available: {",
        "    state: 'loading',",
        "    degradedReasons: [],",
        "    loadedByHiddenAdapter: false,",
        "    loaderProofRef: null,",
        "    pageBodyCaptured: false,",
        "    transcriptTextCaptured: false,",
        "  },",
        "  'disabled-by-policy': {",
        "    state: 'manual-required',",
        "    degradedReasons: ['disabled-by-policy'],",
        "    loadedByHiddenAdapter: false,",
        "    loaderProofRef: null,",
        "    pageBodyCaptured: false,",
        "    transcriptTextCaptured: false,",
        "  },",
        "  'manual-required': {",
        "    state: 'manual-required',",
        "    degradedReasons: ['manual-required'],",
        "    loadedByHiddenAdapter: false,",
        "    loaderProofRef: null,",
        "    pageBodyCaptured: false,",
        "    transcriptTextCaptured: false,",
        "  },",
        "  unavailable: {",
        "    state: 'manual-required',",
        "    degradedReasons: ['loader-unavailable'],",
        "    loadedByHiddenAdapter: false,",
        "    loaderProofRef: null,",
        "    pageBodyCaptured: false,",
        "    transcriptTextCaptured: false,",
        "  },",
        "} as const;",
        "",
        "export type BrowserHiddenAnalysisLoaderCapabilityState =",
        "  | 'available'",
        "  | 'disabled-by-policy'",
        "  | 'manual-required'",
        "  | 'unavailable';",
        "",
        "export type BrowserHiddenAnalysisLoaderDecisionTemplate =",
        "  (typeof BrowserHiddenAnalysisLoaderDecisionTemplates)[keyof typeof BrowserHiddenAnalysisLoaderDecisionTemplates];",
        "",
        "export function browserHiddenAnalysisLoaderDecisionTemplate(",
        "  policyAllowsHiddenAnalysis: boolean,",
        "  capabilityState: BrowserHiddenAnalysisLoaderCapabilityState",
        "): BrowserHiddenAnalysisLoaderDecisionTemplate {",
        "  if (!policyAllowsHiddenAnalysis) {",
        "    return BrowserHiddenAnalysisLoaderDecisionTemplates['disabled-by-policy'];",
        "  }",
        "",
        "  return BrowserHiddenAnalysisLoaderDecisionTemplates[capabilityState];",
        "}",
        "",
    ]
    .join("\n")
}

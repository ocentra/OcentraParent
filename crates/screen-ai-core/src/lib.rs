#![forbid(unsafe_code)]

//! Screen AI pipeline ownership.
//!
//! Browser/app/game/screen sources produce triggers and evidence refs. This
//! crate decides when a screen-analysis work item may be requested and keeps
//! model output advisory until policy consumes validated evidence.

use ocentra_evidence::EvidenceReferenceState;

pub const CRATE_NAME: &str = "ocentra-screen-ai-core";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenAiTriggerSource {
    App,
    Browser,
    AppGame,
    ScreenCapture,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RawPrivateFrameState {
    Allowed,
    Blocked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenAiPolicyNeedState {
    Required,
    NotRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenAiAnalysisRequestState {
    Required,
    NotRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenAiRawFrameInclusionState {
    Include,
    Exclude,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenAiPolicyAuthorityState {
    EvidenceOnly,
    ClaimsAuthority,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScreenAiPipelineInput {
    pub trigger_source: ScreenAiTriggerSource,
    pub evidence_reference_state: EvidenceReferenceState,
    pub raw_private_frame_state: RawPrivateFrameState,
    pub policy_need_state: ScreenAiPolicyNeedState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScreenAiPipelineDecision {
    pub analysis_request_state: ScreenAiAnalysisRequestState,
    pub raw_frame_inclusion_state: ScreenAiRawFrameInclusionState,
    pub policy_authority_state: ScreenAiPolicyAuthorityState,
}

pub fn evaluate_screen_ai_pipeline(
    input: ScreenAiPipelineInput,
) -> ScreenAiPipelineDecision {
    let request_analysis = input.evidence_reference_state == EvidenceReferenceState::Stable
        && input.policy_need_state == ScreenAiPolicyNeedState::Required;

    ScreenAiPipelineDecision {
        analysis_request_state: if request_analysis {
            ScreenAiAnalysisRequestState::Required
        } else {
            ScreenAiAnalysisRequestState::NotRequired
        },
        raw_frame_inclusion_state: if request_analysis
            && input.raw_private_frame_state == RawPrivateFrameState::Allowed
        {
            ScreenAiRawFrameInclusionState::Include
        } else {
            ScreenAiRawFrameInclusionState::Exclude
        },
        policy_authority_state: ScreenAiPolicyAuthorityState::EvidenceOnly,
    }
}

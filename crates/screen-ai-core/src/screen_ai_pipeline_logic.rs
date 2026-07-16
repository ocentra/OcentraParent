use crate::screen_ai_pipeline::{
    screen_ai_decision_id, screen_ai_decision_ref, RawPrivateFrameState,
    ScreenAiAnalysisRequestState, ScreenAiPipelineDecision, ScreenAiPipelineDecisionRecordedEvent,
    ScreenAiPipelineEvaluationRequestedEvent, ScreenAiPipelineInput, ScreenAiPolicyAuthorityState,
    ScreenAiPolicyNeedState, ScreenAiRawFrameInclusionState,
};
use ocentra_evidence::EvidenceReferenceState;

pub(crate) fn evaluate_screen_ai_pipeline(
    input: ScreenAiPipelineInput,
) -> ScreenAiPipelineDecision {
    let request_analysis = screen_ai_pipeline_request_analysis_is_required(&input);

    ScreenAiPipelineDecision {
        analysis_request_state: screen_ai_analysis_request_state_for(request_analysis),
        raw_frame_inclusion_state: screen_ai_raw_frame_inclusion_state_for(
            request_analysis,
            &input,
        ),
        policy_authority_state: screen_ai_policy_authority_state_for(),
    }
}

pub(crate) fn record_screen_ai_pipeline_decision(
    event: &ScreenAiPipelineEvaluationRequestedEvent,
) -> ScreenAiPipelineDecisionRecordedEvent {
    let decision_id = screen_ai_decision_id(screen_ai_decision_ref(&event.evaluation_id));

    ScreenAiPipelineDecisionRecordedEvent {
        aggregate_id: event.aggregate_id.clone(),
        decision_id,
        source_evaluation_id: event.evaluation_id.clone(),
        decision: evaluate_screen_ai_pipeline(event.input),
    }
}

fn screen_ai_pipeline_request_analysis_is_required(input: &ScreenAiPipelineInput) -> bool {
    input.evidence_reference_state == EvidenceReferenceState::Stable
        && input.policy_need_state == ScreenAiPolicyNeedState::Required
}

fn screen_ai_analysis_request_state_for(request_analysis: bool) -> ScreenAiAnalysisRequestState {
    if request_analysis {
        ScreenAiAnalysisRequestState::Required
    } else {
        ScreenAiAnalysisRequestState::NotRequired
    }
}

fn screen_ai_raw_frame_inclusion_state_for(
    request_analysis: bool,
    input: &ScreenAiPipelineInput,
) -> ScreenAiRawFrameInclusionState {
    if request_analysis && input.raw_private_frame_state == RawPrivateFrameState::Allowed {
        ScreenAiRawFrameInclusionState::Include
    } else {
        ScreenAiRawFrameInclusionState::Exclude
    }
}

fn screen_ai_policy_authority_state_for() -> ScreenAiPolicyAuthorityState {
    ScreenAiPolicyAuthorityState::EvidenceOnly
}

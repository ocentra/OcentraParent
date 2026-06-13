use ocentra_evidence::EvidenceReferenceState;
use ocentra_screen_ai_core::{
    evaluate_screen_ai_pipeline, RawPrivateFrameState, ScreenAiAnalysisRequestState,
    ScreenAiPipelineInput, ScreenAiPolicyAuthorityState, ScreenAiPolicyNeedState,
    ScreenAiRawFrameInclusionState, ScreenAiTriggerSource,
};

#[test]
fn screen_ai_request_requires_evidence_refs_and_policy_need() {
    let decision = evaluate_screen_ai_pipeline(ScreenAiPipelineInput {
        trigger_source: ScreenAiTriggerSource::Browser,
        evidence_reference_state: EvidenceReferenceState::Stable,
        raw_private_frame_state: RawPrivateFrameState::Blocked,
        policy_need_state: ScreenAiPolicyNeedState::Required,
    });

    assert_eq!(
        decision.analysis_request_state,
        ScreenAiAnalysisRequestState::Required
    );
    assert_eq!(
        decision.raw_frame_inclusion_state,
        ScreenAiRawFrameInclusionState::Exclude
    );
    assert_eq!(
        decision.policy_authority_state,
        ScreenAiPolicyAuthorityState::EvidenceOnly
    );
}

#[test]
fn screen_ai_accepts_app_trigger_as_evidence_source_without_policy_authority() {
    let decision = evaluate_screen_ai_pipeline(ScreenAiPipelineInput {
        trigger_source: ScreenAiTriggerSource::App,
        evidence_reference_state: EvidenceReferenceState::Stable,
        raw_private_frame_state: RawPrivateFrameState::Blocked,
        policy_need_state: ScreenAiPolicyNeedState::Required,
    });

    assert_eq!(
        decision.analysis_request_state,
        ScreenAiAnalysisRequestState::Required
    );
    assert_eq!(
        decision.policy_authority_state,
        ScreenAiPolicyAuthorityState::EvidenceOnly
    );
}

#[test]
fn screen_ai_does_not_request_analysis_without_evidence_refs() {
    let decision = evaluate_screen_ai_pipeline(ScreenAiPipelineInput {
        trigger_source: ScreenAiTriggerSource::ScreenCapture,
        evidence_reference_state: EvidenceReferenceState::Missing,
        raw_private_frame_state: RawPrivateFrameState::Allowed,
        policy_need_state: ScreenAiPolicyNeedState::Required,
    });

    assert_eq!(
        decision.analysis_request_state,
        ScreenAiAnalysisRequestState::NotRequired
    );
    assert_eq!(
        decision.raw_frame_inclusion_state,
        ScreenAiRawFrameInclusionState::Exclude
    );
    assert_eq!(
        decision.policy_authority_state,
        ScreenAiPolicyAuthorityState::EvidenceOnly
    );
}

#[test]
fn screen_ai_does_not_request_analysis_when_policy_does_not_need_it() {
    let decision = evaluate_screen_ai_pipeline(ScreenAiPipelineInput {
        trigger_source: ScreenAiTriggerSource::AppGame,
        evidence_reference_state: EvidenceReferenceState::Stable,
        raw_private_frame_state: RawPrivateFrameState::Allowed,
        policy_need_state: ScreenAiPolicyNeedState::NotRequired,
    });

    assert_eq!(
        decision.analysis_request_state,
        ScreenAiAnalysisRequestState::NotRequired
    );
    assert_eq!(
        decision.raw_frame_inclusion_state,
        ScreenAiRawFrameInclusionState::Exclude
    );
    assert_eq!(
        decision.policy_authority_state,
        ScreenAiPolicyAuthorityState::EvidenceOnly
    );
}

#[test]
fn screen_ai_includes_raw_frame_only_when_evidence_policy_need_and_privacy_allow_it() {
    let decision = evaluate_screen_ai_pipeline(ScreenAiPipelineInput {
        trigger_source: ScreenAiTriggerSource::ScreenCapture,
        evidence_reference_state: EvidenceReferenceState::Stable,
        raw_private_frame_state: RawPrivateFrameState::Allowed,
        policy_need_state: ScreenAiPolicyNeedState::Required,
    });

    assert_eq!(
        decision.analysis_request_state,
        ScreenAiAnalysisRequestState::Required
    );
    assert_eq!(
        decision.raw_frame_inclusion_state,
        ScreenAiRawFrameInclusionState::Include
    );
    assert_eq!(
        decision.policy_authority_state,
        ScreenAiPolicyAuthorityState::EvidenceOnly
    );
}

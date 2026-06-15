use ocentra_eventing::envelope::DomainEvent;
use ocentra_evidence::EvidenceReferenceState;
use ocentra_screen_ai_core::{
    evaluate_screen_ai_pipeline, record_screen_ai_pipeline_decision, RawPrivateFrameState,
    ScreenAiAggregateId, ScreenAiAnalysisRequestState, ScreenAiPipelineEvaluationId,
    ScreenAiPipelineEvaluationRequestedEvent, ScreenAiPipelineInput, ScreenAiPolicyAuthorityState,
    ScreenAiPolicyNeedState, ScreenAiRawFrameInclusionState, ScreenAiTriggerSource,
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

#[test]
fn screen_ai_pipeline_request_records_typed_decision_event() {
    let request = ScreenAiPipelineEvaluationRequestedEvent {
        aggregate_id: ScreenAiAggregateId::parse("screen-ai-family-default")
            .expect("screen ai aggregate"),
        evaluation_id: ScreenAiPipelineEvaluationId::parse("screen-ai-evaluation-default")
            .expect("screen ai evaluation"),
        input: ScreenAiPipelineInput {
            trigger_source: ScreenAiTriggerSource::Browser,
            evidence_reference_state: EvidenceReferenceState::Stable,
            raw_private_frame_state: RawPrivateFrameState::Blocked,
            policy_need_state: ScreenAiPolicyNeedState::Required,
        },
    };

    let decision = record_screen_ai_pipeline_decision(&request);

    assert_eq!(decision.aggregate_id, request.aggregate_id);
    assert_eq!(decision.source_evaluation_id, request.evaluation_id);
    assert_eq!(
        decision.decision.analysis_request_state,
        ScreenAiAnalysisRequestState::Required
    );
    assert_eq!(
        request
            .contract()
            .expect("screen ai request contract")
            .event_type
            .as_str(),
        "screen-ai.pipeline-evaluation.requested"
    );
    assert_eq!(
        decision
            .contract()
            .expect("screen ai decision contract")
            .event_type
            .as_str(),
        "screen-ai.pipeline-decision.recorded"
    );
}

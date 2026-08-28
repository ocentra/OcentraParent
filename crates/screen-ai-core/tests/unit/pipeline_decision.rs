use ocentra_eventing::envelope::DomainEvent;
use ocentra_evidence::EvidenceReferenceState;
use ocentra_screen_ai_core::screen_ai_pipeline::{
    evaluate_screen_ai_pipeline, record_screen_ai_pipeline_decision, RawPrivateFrameState,
    ScreenAiAggregateId, ScreenAiAnalysisRequestState, ScreenAiPipelineEvaluationId,
    ScreenAiPipelineEvaluationRequestedEvent, ScreenAiPipelineInput, ScreenAiPolicyAuthorityState,
    ScreenAiPolicyNeedState, ScreenAiRawFrameInclusionState, ScreenAiTriggerSource,
};
use ocentra_screen_ai_core::screen_intelligence_router::extraction::owner::handoff::ScreenManagedBrowserStructuredExtractionHandoffError;
use ocentra_screen_ai_core::screen_intelligence_router::extraction::owner::{
    ManagedBrowserStructuredExtractionObservation, ManagedBrowserStructuredExtractionOwner,
};
use ocentra_screen_ai_core::screen_intelligence_router::{
    plan_screen_intelligence_route, screen_intelligence_route_decision_is_consistent,
    screen_intelligence_route_request_is_consistent,
    screen_managed_browser_structured_extraction_is_consistent, ActivityEvidenceRef,
    ScreenCaptureScope, ScreenEvidenceCustodyState, ScreenIntelligencePolicySensitivity,
    ScreenIntelligenceRouteKind, ScreenIntelligenceRouteRequest, ScreenIntelligenceSourceKind,
    ScreenManagedBrowserStructuredExtraction, ScreenStructuredExtractionFallbackState,
};

#[derive(Debug)]
struct TestError(String);

impl std::fmt::Display for TestError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for TestError {}

type TestResult = Result<(), TestError>;

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
fn screen_ai_pipeline_request_records_typed_decision_event() -> TestResult {
    let request = ScreenAiPipelineEvaluationRequestedEvent {
        aggregate_id: ScreenAiAggregateId::parse("screen-ai-family-default")
            .map_err(|error| TestError(format!("screen ai aggregate: {error:?}")))?,
        evaluation_id: ScreenAiPipelineEvaluationId::parse("screen-ai-evaluation-default")
            .map_err(|error| TestError(format!("screen ai evaluation: {error:?}")))?,
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
            .map_err(|error| { TestError(format!("screen ai request contract: {error:?}")) })?
            .event_type
            .as_str(),
        "screen-ai.pipeline-evaluation.requested"
    );
    assert_eq!(
        decision
            .contract()
            .map_err(|error| { TestError(format!("screen ai decision contract: {error:?}")) })?
            .event_type
            .as_str(),
        "screen-ai.pipeline-decision.recorded"
    );

    Ok(())
}

const TEST_DIGEST: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

struct ContractObservationOwner {
    observation: std::sync::Mutex<Option<ManagedBrowserStructuredExtractionObservation>>,
}

/// Contract-only adapter for an untrusted observation boundary. It does not
/// represent a live browser provider or authorize a route.
impl ManagedBrowserStructuredExtractionOwner for ContractObservationOwner {
    fn observation(&self) -> ManagedBrowserStructuredExtractionObservation {
        self.observation
            .lock()
            .expect("contract observation owner mutex should not be poisoned")
            .take()
            .expect("contract observation owner should be queried once")
    }
}

fn route_request(
    source_kind: ScreenIntelligenceSourceKind,
    parent_allows_screen_capture: bool,
    allowed_capture_scopes: Vec<ScreenCaptureScope>,
) -> ScreenIntelligenceRouteRequest {
    ScreenIntelligenceRouteRequest {
        schema_version: 1,
        request_id: String::from("screen-route-request-test"),
        requested_at: String::from("2026-08-28T12:00:00Z"),
        device_ref: String::from("child-device-test"),
        source_kind,
        capture_reason: String::from("policy-evidence-review"),
        policy_question: String::from("category-review"),
        policy_sensitivity: ScreenIntelligencePolicySensitivity::Ordinary,
        existing_evidence_refs: Vec::new(),
        structured_extraction: None,
        parent_allows_managed_browser_structured_extraction: false,
        parent_allows_screen_capture,
        allowed_capture_scopes,
        protected_surface_suspected: false,
        credential_prompt_suspected: false,
    }
}

fn managed_browser_request(
    extraction: Option<ScreenManagedBrowserStructuredExtraction>,
) -> ScreenIntelligenceRouteRequest {
    let mut request = route_request(
        ScreenIntelligenceSourceKind::ManagedBrowser,
        true,
        vec![ScreenCaptureScope::ManagedBrowserWindow],
    );
    request.parent_allows_managed_browser_structured_extraction = true;
    request.structured_extraction = extraction;
    request
}

fn managed_browser_observation() -> ManagedBrowserStructuredExtractionObservation {
    let target_ref = String::from("browser-target-test");
    let evidence_refs = vec![
        ActivityEvidenceRef {
            evidence_id: target_ref.clone(),
            kind: String::from("managed-browser-target"),
            digest: String::from(TEST_DIGEST),
            uri: None,
        },
        ActivityEvidenceRef {
            evidence_id: String::from("browser-url-test"),
            kind: String::from("managed-browser-url"),
            digest: String::from(TEST_DIGEST),
            uri: None,
        },
        ActivityEvidenceRef {
            evidence_id: String::from("browser-title-test"),
            kind: String::from("managed-browser-title"),
            digest: String::from(TEST_DIGEST),
            uri: None,
        },
    ];

    ManagedBrowserStructuredExtractionObservation {
        source_id: String::from("managed-browser-cdp"),
        extraction_id: format!("browser-extraction-{TEST_DIGEST}"),
        captured_at: String::from("2026-08-28T12:00:01Z"),
        managed_browser_session_ref: String::from("managed-browser-session-test"),
        target_ref,
        evidence_refs,
        structured_evidence_digest: String::from(TEST_DIGEST),
        structured_signal_digest: String::from(TEST_DIGEST),
        structured_body_digest: format!("managed-browser-body-sha256-v1-{TEST_DIGEST}"),
        structured_sensitivity_digest: String::from(
            "managed-browser-sensitivity-structural-safe-v1",
        ),
        document_frame_id: Some(String::from("frame-test")),
        document_loader_id: Some(String::from("loader-test")),
        document_url_digest: Some(String::from(TEST_DIGEST)),
        authority_digest: String::from(TEST_DIGEST),
        dom_overflow_redacted: false,
        private_content_redacted: false,
        protected_content_skipped: false,
        fresh: true,
        unavailable: false,
        custody_state: ScreenEvidenceCustodyState::ChildDeviceQueryStore,
    }
}

fn managed_browser_extraction(
    observation: ManagedBrowserStructuredExtractionObservation,
) -> Result<
    ScreenManagedBrowserStructuredExtraction,
    ScreenManagedBrowserStructuredExtractionHandoffError,
> {
    ScreenManagedBrowserStructuredExtraction::from_untrusted_observation(Box::new(
        ContractObservationOwner {
            observation: std::sync::Mutex::new(Some(observation)),
        },
    ))
}

#[test]
fn native_sources_route_to_scoped_capture_after_evidence_check() {
    let cases = [
        (
            ScreenIntelligenceSourceKind::NativeApp,
            ScreenCaptureScope::ActiveWindow,
            ScreenIntelligenceRouteKind::ScreenCaptureActiveWindow,
        ),
        (
            ScreenIntelligenceSourceKind::NativeGame,
            ScreenCaptureScope::SelectedWindow,
            ScreenIntelligenceRouteKind::ScreenCaptureSelectedWindow,
        ),
        (
            ScreenIntelligenceSourceKind::Launcher,
            ScreenCaptureScope::ActiveWindow,
            ScreenIntelligenceRouteKind::ScreenCaptureActiveWindow,
        ),
        (
            ScreenIntelligenceSourceKind::UnknownProcess,
            ScreenCaptureScope::SelectedWindow,
            ScreenIntelligenceRouteKind::ScreenCaptureSelectedWindow,
        ),
    ];

    for (source_kind, scope, expected_route) in cases {
        let request = route_request(source_kind, true, vec![scope.clone()]);
        assert_eq!(
            screen_intelligence_route_request_is_consistent(&request),
            true
        );

        let decision = plan_screen_intelligence_route(&request, "route-native-test");

        assert_eq!(decision.route_kind, expected_route);
        assert_eq!(decision.capture_scope, Some(scope));
        assert_eq!(decision.screenshot_skipped, false);
        assert_eq!(decision.checked_existing_evidence_first, true);
        assert_eq!(decision.managed_browser_structured_extraction_first, false);
        assert_eq!(
            decision.structured_extraction_fallback_state,
            ScreenStructuredExtractionFallbackState::NotAttempted
        );
        assert_eq!(
            decision.custody_state,
            ScreenEvidenceCustodyState::ChildDeviceQueryStore
        );
        assert_eq!(decision.manual_required_reason, None);
        assert_eq!(decision.unavailable_reason, None);
        assert_eq!(decision.policy_question, "category-review");
        assert_eq!(
            decision.policy_sensitivity,
            ScreenIntelligencePolicySensitivity::Ordinary
        );
        assert_eq!(decision.remote_ai_allowed, false);
        assert_eq!(decision.raw_screenshot_retained, false);
        assert_eq!(
            screen_intelligence_route_decision_is_consistent(&decision),
            true
        );
    }
}

#[test]
fn evidence_only_sources_require_manual_review_without_capture() {
    for source_kind in [
        ScreenIntelligenceSourceKind::NetworkOrSessionSummary,
        ScreenIntelligenceSourceKind::ScreenAdjacentEvidence,
    ] {
        let request = route_request(source_kind, true, vec![ScreenCaptureScope::ActiveWindow]);
        let decision = plan_screen_intelligence_route(&request, "route-evidence-only-test");

        assert_eq!(
            decision.route_kind,
            ScreenIntelligenceRouteKind::ManualRequired
        );
        assert_eq!(decision.capture_scope, None);
        assert_eq!(decision.screenshot_skipped, true);
        assert_eq!(
            decision.manual_required_reason.as_deref(),
            Some("existing evidence requires an owner-backed answer before screen capture")
        );
        assert_eq!(decision.unavailable_reason, None);
        assert_eq!(
            screen_intelligence_route_decision_is_consistent(&decision),
            true
        );
    }
}

#[test]
fn managed_browser_without_owner_receipt_never_falls_back_to_screenshot() {
    let request = managed_browser_request(None);
    let decision = plan_screen_intelligence_route(&request, "route-browser-missing-owner-test");

    assert_eq!(
        decision.route_kind,
        ScreenIntelligenceRouteKind::Unavailable
    );
    assert_eq!(decision.capture_scope, None);
    assert_eq!(decision.screenshot_skipped, true);
    assert_eq!(decision.structured_extraction_id, None);
    assert_eq!(decision.managed_browser_structured_extraction_first, false);
    assert_eq!(
        decision.structured_extraction_fallback_state,
        ScreenStructuredExtractionFallbackState::AuthorityUnavailable
    );
    assert_eq!(
        decision.unavailable_reason.as_deref(),
        Some("managed-browser structured extraction producer authority is unavailable")
    );
    assert_eq!(
        decision.custody_state,
        ScreenEvidenceCustodyState::Unavailable
    );
    assert_eq!(decision.evidence_refs, Vec::<ActivityEvidenceRef>::new());
    assert_eq!(
        screen_intelligence_route_decision_is_consistent(&decision),
        true
    );
}

#[test]
fn untrusted_managed_browser_observation_is_validated_but_not_promoted_to_authority() {
    let extraction = managed_browser_extraction(managed_browser_observation())
        .expect("the bounded redacted observation should satisfy the handoff shape");
    assert_eq!(
        screen_managed_browser_structured_extraction_is_consistent(&extraction),
        true
    );

    let request = managed_browser_request(Some(extraction));
    assert_eq!(
        screen_intelligence_route_request_is_consistent(&request),
        true
    );
    let decision = plan_screen_intelligence_route(&request, "route-browser-unvalidated-owner-test");

    assert_eq!(
        decision.route_kind,
        ScreenIntelligenceRouteKind::Unavailable
    );
    assert_eq!(decision.capture_scope, None);
    assert_eq!(decision.screenshot_skipped, true);
    assert_eq!(decision.structured_extraction_id, None);
    assert_eq!(
        decision.structured_extraction_fallback_state,
        ScreenStructuredExtractionFallbackState::AuthorityUnavailable
    );
    assert_eq!(decision.evidence_refs, Vec::<ActivityEvidenceRef>::new());
    assert_eq!(
        screen_intelligence_route_decision_is_consistent(&decision),
        true
    );
}

#[test]
fn unavailable_managed_browser_observation_is_normalized_to_a_fail_closed_boundary() {
    let mut observation = managed_browser_observation();
    observation.extraction_id = String::from("caller-controlled-extraction");
    observation.managed_browser_session_ref = String::from("caller-controlled-session");
    observation.target_ref = String::from("caller-controlled-target");
    observation.authority_digest = String::from("caller-controlled-authority");
    observation.structured_evidence_digest = String::from("caller-controlled-evidence");
    observation.structured_signal_digest = String::from("caller-controlled-signal");
    observation.structured_body_digest = String::from("caller-controlled-body");
    observation.structured_sensitivity_digest = String::from("caller-controlled-sensitivity");
    observation.unavailable = true;

    let extraction = managed_browser_extraction(observation)
        .expect("unavailable owner output should normalize to fixed redacted identities");
    assert_eq!(
        screen_managed_browser_structured_extraction_is_consistent(&extraction),
        false
    );

    let request = managed_browser_request(Some(extraction));
    assert_eq!(
        screen_intelligence_route_request_is_consistent(&request),
        false
    );
    let decision = plan_screen_intelligence_route(&request, "route-browser-unavailable-test");
    assert_eq!(
        decision.route_kind,
        ScreenIntelligenceRouteKind::Unavailable
    );
    assert_eq!(decision.capture_scope, None);
    assert_eq!(decision.screenshot_skipped, true);
    assert_eq!(
        decision.structured_extraction_fallback_state,
        ScreenStructuredExtractionFallbackState::AuthorityUnavailable
    );
    assert_eq!(decision.evidence_refs, Vec::<ActivityEvidenceRef>::new());
    assert_eq!(
        decision.unavailable_reason.as_deref(),
        Some("screen intelligence route request is inconsistent or unsupported")
    );
}

#[test]
fn malformed_managed_browser_observation_is_rejected_before_routing() {
    let mut observation = managed_browser_observation();
    observation.structured_body_digest = String::from("raw-structured-body");

    let handoff = managed_browser_extraction(observation);
    assert_eq!(
        handoff.err(),
        Some(ScreenManagedBrowserStructuredExtractionHandoffError::InvalidOwnerHandoff)
    );
}

#[test]
fn sensitive_or_disabled_capture_never_queues_a_screenshot() {
    let mut protected_request = route_request(
        ScreenIntelligenceSourceKind::NativeApp,
        true,
        vec![ScreenCaptureScope::ActiveWindow],
    );
    protected_request.policy_sensitivity = ScreenIntelligencePolicySensitivity::ProtectedSurface;
    let protected_decision =
        plan_screen_intelligence_route(&protected_request, "route-protected-test");

    assert_eq!(
        protected_decision.route_kind,
        ScreenIntelligenceRouteKind::Unavailable
    );
    assert_eq!(protected_decision.capture_scope, None);
    assert_eq!(protected_decision.screenshot_skipped, true);
    assert_eq!(
        protected_decision.unavailable_reason.as_deref(),
        Some("protected surface is not eligible for screen capture or model analysis")
    );

    let mut credential_request = route_request(
        ScreenIntelligenceSourceKind::NativeGame,
        true,
        vec![ScreenCaptureScope::SelectedWindow],
    );
    credential_request.credential_prompt_suspected = true;
    let credential_decision =
        plan_screen_intelligence_route(&credential_request, "route-credential-test");

    assert_eq!(
        credential_decision.route_kind,
        ScreenIntelligenceRouteKind::Unavailable
    );
    assert_eq!(credential_decision.capture_scope, None);
    assert_eq!(credential_decision.screenshot_skipped, true);
    assert_eq!(
        credential_decision.unavailable_reason.as_deref(),
        Some("credential prompt risk is not eligible for screen capture or model analysis")
    );

    let disabled_request = route_request(
        ScreenIntelligenceSourceKind::NativeApp,
        false,
        vec![ScreenCaptureScope::ActiveWindow],
    );
    let disabled_decision =
        plan_screen_intelligence_route(&disabled_request, "route-disabled-test");

    assert_eq!(
        disabled_decision.route_kind,
        ScreenIntelligenceRouteKind::ManualRequired
    );
    assert_eq!(disabled_decision.capture_scope, None);
    assert_eq!(disabled_decision.screenshot_skipped, true);
    assert_eq!(
        disabled_decision.manual_required_reason.as_deref(),
        Some("parent setting requires manual review before screen capture")
    );
}

#[test]
fn unsupported_scope_requires_manual_review() {
    let request = route_request(
        ScreenIntelligenceSourceKind::Launcher,
        true,
        vec![ScreenCaptureScope::PrimaryDisplay],
    );
    let decision = plan_screen_intelligence_route(&request, "route-unsupported-scope-test");

    assert_eq!(
        decision.route_kind,
        ScreenIntelligenceRouteKind::ManualRequired
    );
    assert_eq!(decision.capture_scope, None);
    assert_eq!(decision.screenshot_skipped, true);
    assert_eq!(
        decision.manual_required_reason.as_deref(),
        Some("no allowed active-window or selected-window capture scope is available")
    );
}

#[test]
fn inconsistent_requests_fail_closed_without_returning_existing_private_refs() {
    let mut request = route_request(
        ScreenIntelligenceSourceKind::NativeApp,
        true,
        vec![
            ScreenCaptureScope::FullScreen,
            ScreenCaptureScope::ActiveWindow,
        ],
    );
    request.existing_evidence_refs = vec![ActivityEvidenceRef {
        evidence_id: String::from("evidence-private"),
        kind: String::from("screen-evidence"),
        digest: String::from(TEST_DIGEST),
        uri: Some(String::from("private-uri")),
    }];

    assert_eq!(
        screen_intelligence_route_request_is_consistent(&request),
        false
    );
    let decision = plan_screen_intelligence_route(&request, "route-inconsistent-test");

    assert_eq!(
        decision.route_kind,
        ScreenIntelligenceRouteKind::Unavailable
    );
    assert_eq!(decision.capture_scope, None);
    assert_eq!(decision.screenshot_skipped, true);
    assert_eq!(decision.evidence_refs, Vec::<ActivityEvidenceRef>::new());
    assert_eq!(
        decision.unavailable_reason.as_deref(),
        Some("screen intelligence route request is inconsistent or unsupported")
    );
    assert_eq!(
        screen_intelligence_route_decision_is_consistent(&decision),
        true
    );
}

use ocentra_eventing::error::EventingError;
use ocentra_eventing::expect_value::ExpectValue;

use crate::{
    child_domain_runtime::{
        child_domain_ai_analysis_completed_event, child_domain_ai_analysis_requested_event,
        child_domain_ai_request_id, child_domain_ai_request_id_from_evidence_ref,
        child_domain_analysis_purpose, child_domain_evidence_recorded_event,
        child_domain_evidence_ref, child_domain_evidence_ref_from_observation_id,
        child_domain_observation_id_from_subject_ref, child_domain_observed_event,
        child_domain_policy_evaluation_requested_from_ai_result_event_if_required,
        child_domain_policy_request_id_from_fact_ref, child_domain_policy_rule_ref,
        child_domain_policy_severity, child_domain_policy_violation_id,
        ChildDomainAiAnalysisRequirement, ChildDomainAnalysisPurposeKind, ChildDomainEventType,
        ChildDomainObservedSignal, ChildDomainPolicyEvaluationRequirement,
        ChildDomainPolicyRuleKind, ChildDomainPolicySeverityKind, ChildDomainRefSuffix,
        ChildRuntimeDomain,
    },
    constants,
};

#[test]
fn child_domain_event_type_rejects_unknown_event_name() {
    let result = ChildDomainEventType::parse(
        constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED,
    );

    assert!(matches!(result, Err(EventingError::InvalidValue { .. })));
}

#[test]
fn child_domain_event_type_accepts_known_event_name() {
    let event_type = ChildDomainEventType::parse(
        constants::child_domain_runtime::BROWSER_AI_ANALYSIS_REQUESTED_EVENT_TYPE,
    )
    .expect_value(constants::child_domain_runtime::BROWSER_AI_ANALYSIS_REQUESTED_EVENT_TYPE);

    assert_eq!(
        event_type,
        ChildRuntimeDomain::Browser.ai_analysis_requested_event_type()
    );
}

#[test]
fn child_domain_default_profile_uses_typed_contract_selectors() {
    let app_event = child_domain_observed_event(ChildRuntimeDomain::App.default_observed_profile());
    let event = child_domain_observed_event(ChildRuntimeDomain::Browser.default_observed_profile());
    let evidence = child_domain_evidence_recorded_event(&event);

    assert_eq!(
        app_event.event_type,
        ChildRuntimeDomain::App.observed_event_type()
    );
    assert!(app_event
        .subject_ref
        .as_str()
        .contains(constants::child_domain_runtime::APP_SUBJECT_REF_SUFFIX));
    assert_eq!(
        event.event_type,
        ChildRuntimeDomain::Browser.observed_event_type()
    );
    assert_eq!(
        event.observed_state,
        ChildDomainObservedSignal::RequiresAi.into_observed_state()
    );
    assert_eq!(
        event.observation_id,
        child_domain_observation_id_from_subject_ref(
            ChildRuntimeDomain::Browser,
            &event.subject_ref,
            &event.observed_state
        )
    );
    assert_eq!(
        evidence.ai_analysis_requirement,
        ChildDomainAiAnalysisRequirement::Required
    );
    assert_eq!(
        evidence.policy_evaluation_requirement,
        ChildDomainPolicyEvaluationRequirement::Required
    );
    assert_eq!(
        evidence.evidence_ref,
        child_domain_evidence_ref_from_observation_id(
            ChildRuntimeDomain::Browser,
            &event.observation_id
        )
    );
    assert_eq!(evidence.source_observed_at, event.observed_at);
}

#[test]
fn child_domain_ref_and_policy_constructors_use_typed_selectors() {
    let evidence = child_domain_evidence_ref(
        ChildRuntimeDomain::Browser,
        ChildDomainRefSuffix::DefaultEvidence,
    );
    let ai_request = child_domain_ai_request_id(
        ChildRuntimeDomain::Browser,
        ChildDomainRefSuffix::DefaultAiRequest,
    );
    let purpose = child_domain_analysis_purpose(ChildDomainAnalysisPurposeKind::Classification);
    let violation = child_domain_policy_violation_id(
        ChildRuntimeDomain::Browser,
        ChildDomainRefSuffix::DefaultPolicyViolation,
    );
    let rule = child_domain_policy_rule_ref(ChildDomainPolicyRuleKind::Default);
    let severity = child_domain_policy_severity(ChildDomainPolicySeverityKind::Review);

    assert!(evidence
        .as_str()
        .contains(ChildRuntimeDomain::Browser.as_contract_text()));
    assert!(ai_request
        .as_str()
        .contains(ChildRuntimeDomain::Browser.as_contract_text()));
    assert_eq!(
        purpose.as_str(),
        constants::child_domain_runtime::AI_PURPOSE_CLASSIFICATION
    );
    assert!(violation
        .as_str()
        .contains(ChildRuntimeDomain::Browser.as_contract_text()));
    assert_eq!(
        rule.as_str(),
        constants::child_domain_runtime::POLICY_RULE_DEFAULT
    );
    assert_eq!(
        severity.as_str(),
        constants::child_domain_runtime::POLICY_SEVERITY_REVIEW
    );
}

#[test]
fn child_domain_ai_completion_is_a_named_boundary_event_before_policy() {
    let observed =
        child_domain_observed_event(ChildRuntimeDomain::Browser.default_observed_profile());
    let evidence = child_domain_evidence_recorded_event(&observed);
    let ai_request = child_domain_ai_analysis_requested_event(&evidence);
    let ai_completed = child_domain_ai_analysis_completed_event(&ai_request);
    let policy_request =
        child_domain_policy_evaluation_requested_from_ai_result_event_if_required(&ai_completed)
            .expect_value(constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED);

    assert_eq!(
        ai_completed.event_type,
        ChildDomainEventType::ai_analysis_completed()
    );
    assert_eq!(ai_completed.source_ai_request_id, ai_request.ai_request_id);
    assert_eq!(ai_completed.evidence_refs, ai_request.evidence_refs);
    assert_eq!(ai_request.source_observed_at, evidence.source_observed_at);
    assert_eq!(
        ai_completed.source_observed_at,
        ai_request.source_observed_at
    );
    assert_eq!(
        policy_request.source_observed_at,
        ai_completed.source_observed_at
    );
    assert_eq!(policy_request.source_fact_ref, ai_completed.result_fact_ref);
    assert_eq!(
        ai_request.ai_request_id,
        child_domain_ai_request_id_from_evidence_ref(
            ChildRuntimeDomain::Browser,
            &evidence.evidence_ref
        )
    );
    assert_eq!(
        policy_request.policy_request_id,
        child_domain_policy_request_id_from_fact_ref(
            ChildRuntimeDomain::Browser,
            &ai_completed.result_fact_ref
        )
    );
}

use ocentra_child_runtime::child_domain_runtime_flow as ocentra_child_runtime;
use ocentra_network_core::network_runtime::{network_observed_event, NetworkObservationIntent};
use ocentra_parent_agent_protocol::child_domain_runtime::{
    child_domain_observed_event, ChildDomainAiAnalysisRequirement, ChildDomainEventType,
    ChildDomainObservedEventProfile, ChildDomainObservedSignal,
    ChildDomainPolicyEvaluationRequirement, ChildDomainRefSuffix, ChildRuntimeDomain,
};
use ocentra_parent_agent_protocol::constants;

trait OptionRequiredExt<T> {
    fn required(self, context: impl std::fmt::Display) -> T;
}

impl<T> OptionRequiredExt<T> for Option<T> {
    fn required(self, context: impl std::fmt::Display) -> T {
        let _ = context;
        self.unwrap_or_else(|| std::process::abort())
    }
}

trait ResultRequiredExt<T, E> {
    fn required(self, context: impl std::fmt::Display) -> T;
}

impl<T, E: std::fmt::Debug> ResultRequiredExt<T, E> for Result<T, E> {
    fn required(self, context: impl std::fmt::Display) -> T {
        let _ = context;
        self.unwrap_or_else(|_| std::process::abort())
    }
}

#[tokio::test]
async fn child_domain_runtime_flow_keeps_feature_ai_policy_and_notification_decoupled_by_events() {
    let reports = ocentra_child_runtime::publish_default_child_domain_runtime_flows()
        .await
        .required(constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED);

    assert_eq!(reports.len(), 7);
    assert_eq!(reports[0].domain, ChildRuntimeDomain::App);
    assert_eq!(reports[1].domain, ChildRuntimeDomain::AppGame);
    assert_eq!(reports[2].domain, ChildRuntimeDomain::Browser);
    assert_eq!(reports[3].domain, ChildRuntimeDomain::Lan);
    assert_eq!(reports[4].domain, ChildRuntimeDomain::Network);
    assert_eq!(reports[5].domain, ChildRuntimeDomain::Screen);
    assert_eq!(reports[6].domain, ChildRuntimeDomain::ScreenLiveView);

    for report in &reports {
        let policy_evaluation_requested = report
            .policy_evaluation_requested
            .as_ref()
            .required(constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED);
        let policy_violation_detected = report
            .policy_violation_detected
            .as_ref()
            .required(constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED);
        let notification_requested = report
            .notification_requested
            .as_ref()
            .required(constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED);
        assert_eq!(report.evidence_recorded.domain, report.domain);
        assert_eq!(policy_evaluation_requested.domain, report.domain);
        assert_eq!(policy_violation_detected.domain, report.domain);
        assert_eq!(notification_requested.domain, report.domain);
        assert_eq!(
            policy_violation_detected.event_type,
            ChildDomainEventType::policy_violation_detected()
        );
        assert_eq!(
            notification_requested.event_type,
            ChildDomainEventType::notification_requested()
        );
        assert_eq!(
            policy_violation_detected.evidence_refs,
            policy_evaluation_requested.evidence_refs
        );
        assert_eq!(
            policy_violation_detected.detected_at,
            policy_evaluation_requested.source_observed_at
        );
        assert_eq!(
            notification_requested.source_policy_violation_id,
            policy_violation_detected.violation_id
        );
        assert_eq!(
            notification_requested.requested_at,
            policy_violation_detected.detected_at
        );
        if let Some(ai_analysis_requested) = &report.ai_analysis_requested {
            let ai_analysis_completed = report
                .ai_analysis_completed
                .as_ref()
                .required(constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED);
            assert_eq!(
                ai_analysis_completed.source_ai_request_id,
                ai_analysis_requested.ai_request_id
            );
            assert_eq!(
                ai_analysis_completed.event_type,
                ChildDomainEventType::ai_analysis_completed()
            );
        }
    }

    let ai_requested_domains: Vec<ChildRuntimeDomain> = reports
        .iter()
        .filter_map(|report| report.ai_analysis_requested.as_ref().map(|_| report.domain))
        .collect();
    assert_eq!(
        ai_requested_domains,
        vec![ChildRuntimeDomain::Browser, ChildRuntimeDomain::Screen]
    );
}

#[tokio::test]
async fn child_domain_ai_only_flow_does_not_publish_policy_or_notification() {
    let event = child_domain_observed_event(ChildDomainObservedEventProfile {
        domain: ChildRuntimeDomain::Browser,
        subject_ref_suffix: ChildDomainRefSuffix::BrowserSubject,
        observed_state: ChildDomainObservedSignal::RequiresAi,
        ai_analysis_requirement: ChildDomainAiAnalysisRequirement::Required,
        policy_evaluation_requirement: ChildDomainPolicyEvaluationRequirement::NotRequired,
    });

    let report = ocentra_child_runtime::publish_child_domain_observed_event(event)
        .await
        .required(constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED);

    let ai_analysis_requested = report
        .ai_analysis_requested
        .required(constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED);
    let ai_analysis_completed = report
        .ai_analysis_completed
        .required(constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED);
    assert_eq!(
        ai_analysis_completed.source_ai_request_id,
        ai_analysis_requested.ai_request_id
    );
    assert_eq!(
        ai_analysis_requested.source_observed_at,
        report.evidence_recorded.source_observed_at
    );
    assert_eq!(
        ai_analysis_completed.source_observed_at,
        ai_analysis_requested.source_observed_at
    );
    assert_eq!(report.policy_evaluation_requested, None);
    assert_eq!(report.policy_violation_detected, None);
    assert_eq!(report.notification_requested, None);
}

#[tokio::test]
async fn child_domain_observe_only_intent_records_evidence_without_side_effects() {
    let event = network_observed_event(NetworkObservationIntent::TelemetryObservationOnly);

    let report = ocentra_child_runtime::publish_child_domain_observed_event(event)
        .await
        .required(constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED);

    assert_eq!(report.domain, ChildRuntimeDomain::Network);
    assert_eq!(report.evidence_recorded.domain, ChildRuntimeDomain::Network);
    assert_eq!(
        report.evidence_recorded.ai_analysis_requirement,
        ChildDomainAiAnalysisRequirement::NotRequired
    );
    assert_eq!(
        report.evidence_recorded.policy_evaluation_requirement,
        ChildDomainPolicyEvaluationRequirement::NotRequired
    );
    assert_eq!(report.ai_analysis_requested, None);
    assert_eq!(report.ai_analysis_completed, None);
    assert_eq!(report.policy_evaluation_requested, None);
    assert_eq!(report.policy_violation_detected, None);
    assert_eq!(report.notification_requested, None);
}

#[tokio::test]
async fn child_domain_ambiguous_feature_intent_routes_ai_then_policy_then_notification() {
    let event = ocentra_app_game_core::app_game_observed_event(
        ocentra_app_game_core::AppGameObservationIntent::AmbiguousUsageRequiresAi,
    );

    let report = ocentra_child_runtime::publish_child_domain_observed_event(event)
        .await
        .required(constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED);

    assert_eq!(report.domain, ChildRuntimeDomain::AppGame);
    assert_eq!(
        report.evidence_recorded.ai_analysis_requirement,
        ChildDomainAiAnalysisRequirement::Required
    );
    assert_ai_policy_notification_chain(&report);
}

#[tokio::test]
async fn child_domain_unknown_app_intent_routes_ai_then_policy_then_notification() {
    let event = ocentra_app_core::app_observed_event(
        ocentra_app_core::AppObservationIntent::UnknownAppRequiresAi,
    );

    let report = ocentra_child_runtime::publish_child_domain_observed_event(event)
        .await
        .required(constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED);

    assert_eq!(report.domain, ChildRuntimeDomain::App);
    assert_eq!(
        report.evidence_recorded.ai_analysis_requirement,
        ChildDomainAiAnalysisRequirement::Required
    );
    assert_ai_policy_notification_chain(&report);
}

#[tokio::test]
async fn child_domain_runtime_flow_can_attach_once_for_domain_event_family() {
    let event = ocentra_browser_core::browser_observed_event(
        ocentra_browser_core::BrowserObservationIntent::AmbiguousNavigationRequiresAi,
    );
    let runtime_flow = ocentra_child_runtime::ChildDomainRuntimeEventFlow::for_event(&event)
        .await
        .required(constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED);
    let metrics_before = runtime_flow.metrics_snapshot().await;

    let report = runtime_flow
        .publish_observed(event)
        .await
        .required(constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED);
    let metrics_after = runtime_flow.metrics_snapshot().await;

    assert_eq!(metrics_before.subscription_count, 6);
    assert_eq!(metrics_after.subscription_count, 6);
    assert_eq!(report.domain, ChildRuntimeDomain::Browser);
    assert_ai_policy_notification_chain(&report);
}

fn assert_ai_policy_notification_chain(
    report: &ocentra_child_runtime::ChildDomainRuntimeFlowReport,
) {
    let ai_analysis_requested = report
        .ai_analysis_requested
        .as_ref()
        .required(constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED);
    let ai_analysis_completed = report
        .ai_analysis_completed
        .as_ref()
        .required(constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED);
    let policy_evaluation_requested = report
        .policy_evaluation_requested
        .as_ref()
        .required(constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED);
    let policy_violation_detected = report
        .policy_violation_detected
        .as_ref()
        .required(constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED);
    let notification_requested = report
        .notification_requested
        .as_ref()
        .required(constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED);

    assert_eq!(
        ai_analysis_completed.source_ai_request_id,
        ai_analysis_requested.ai_request_id
    );
    assert_eq!(
        policy_evaluation_requested.evidence_refs,
        ai_analysis_completed.evidence_refs
    );
    assert_eq!(
        policy_evaluation_requested.source_observed_at,
        ai_analysis_completed.source_observed_at
    );
    assert_eq!(
        policy_violation_detected.evidence_refs,
        policy_evaluation_requested.evidence_refs
    );
    assert_eq!(
        policy_violation_detected.detected_at,
        policy_evaluation_requested.source_observed_at
    );
    assert_eq!(
        notification_requested.source_policy_violation_id,
        policy_violation_detected.violation_id
    );
    assert_eq!(
        notification_requested.requested_at,
        policy_violation_detected.detected_at
    );
}

use ocentra_child_runtime::{
    publish_child_domain_observed_event, publish_default_child_domain_runtime_flows,
};
use ocentra_parent_agent_protocol::{ChildDomainObservedSignal, ChildRuntimeDomain};

#[tokio::test]
async fn default_child_domain_runtime_flows_cover_child_owned_domains() {
    let reports = publish_default_child_domain_runtime_flows()
        .await
        .expect("default child domain runtime flows");
    let domains = reports
        .iter()
        .map(|report| report.domain)
        .collect::<Vec<_>>();

    assert_eq!(
        domains,
        vec![
            ChildRuntimeDomain::App,
            ChildRuntimeDomain::AppGame,
            ChildRuntimeDomain::Browser,
            ChildRuntimeDomain::Lan,
            ChildRuntimeDomain::Network,
            ChildRuntimeDomain::Screen,
            ChildRuntimeDomain::ScreenLiveView,
        ]
    );
    assert!(
        reports
            .iter()
            .all(|report| report.policy_violation_detected.is_some())
    );
    assert!(
        reports
            .iter()
            .all(|report| report.notification_requested.is_some())
    );
}

#[tokio::test]
async fn app_inventory_observation_records_evidence_without_side_effect_requests() {
    let report = publish_child_domain_observed_event(ocentra_app_core::app_observed_event(
        ocentra_app_core::AppObservationIntent::InventoryObservationOnly,
    ))
    .await
    .expect("app inventory flow");

    assert_eq!(report.domain, ChildRuntimeDomain::App);
    assert_eq!(
        report.evidence_recorded.signal,
        ChildDomainObservedSignal::ObserveOnly
            .into_observed_state()
    );
    assert_eq!(report.ai_analysis_requested, None);
    assert_eq!(report.ai_analysis_completed, None);
    assert_eq!(report.policy_evaluation_requested, None);
    assert_eq!(report.policy_violation_detected, None);
    assert_eq!(report.notification_requested, None);
}

#[tokio::test]
async fn browser_ambiguous_navigation_runs_ai_policy_notification_chain() {
    let report = publish_child_domain_observed_event(ocentra_browser_core::browser_observed_event(
        ocentra_browser_core::BrowserObservationIntent::AmbiguousNavigationRequiresAi,
    ))
    .await
    .expect("browser ambiguous navigation flow");

    let ai_request = report.ai_analysis_requested.expect("browser ai request");
    let ai_completed = report.ai_analysis_completed.expect("browser ai completed");
    let policy_request = report
        .policy_evaluation_requested
        .expect("browser policy request");
    let violation = report
        .policy_violation_detected
        .expect("browser policy violation");
    let notification = report
        .notification_requested
        .expect("browser notification request");

    assert_eq!(report.domain, ChildRuntimeDomain::Browser);
    assert_eq!(ai_request.evidence_refs, vec![report.evidence_recorded.evidence_ref]);
    assert_eq!(ai_completed.source_ai_request_id, ai_request.ai_request_id);
    assert_eq!(policy_request.evidence_refs, ai_completed.evidence_refs);
    assert_eq!(violation.evidence_refs, policy_request.evidence_refs);
    assert_eq!(notification.source_policy_violation_id, violation.violation_id);
}

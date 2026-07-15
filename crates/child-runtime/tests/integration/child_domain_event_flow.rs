use ocentra_child_runtime::child_domain_runtime_flow::{
    publish_child_domain_observed_event, publish_default_child_domain_runtime_flows,
};
use ocentra_parent_agent_protocol::child_domain_runtime::{
    ChildDomainObservedSignal, ChildRuntimeDomain,
};

trait OptionRequiredExt<T> {
    fn required(self, context: impl std::fmt::Display) -> T;
}

impl<T> OptionRequiredExt<T> for Option<T> {
    fn required(self, context: impl std::fmt::Display) -> T {
        let context = context.to_string();
        let _ = context;
        self.unwrap_or_else(|| std::process::abort())
    }
}

trait ResultRequiredExt<T, E> {
    fn required(self, context: impl std::fmt::Display) -> T;
}

impl<T, E: std::fmt::Debug> ResultRequiredExt<T, E> for Result<T, E> {
    fn required(self, context: impl std::fmt::Display) -> T {
        let context = context.to_string();
        let _ = context;
        self.unwrap_or_else(|_| std::process::abort())
    }
}

#[tokio::test]
async fn default_child_domain_runtime_flows_cover_child_owned_domains() {
    let reports = publish_default_child_domain_runtime_flows()
        .await
        .required("default child domain runtime flows");
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
    assert!(reports
        .iter()
        .all(|report| report.policy_violation_detected.is_some()));
    assert!(reports
        .iter()
        .all(|report| report.notification_requested.is_some()));
}

#[tokio::test]
async fn app_inventory_observation_records_evidence_without_side_effect_requests() {
    let report = publish_child_domain_observed_event(ocentra_app_core::app_observed_event(
        ocentra_app_core::AppObservationIntent::InventoryObservationOnly,
    ))
    .await
    .required("app inventory flow");

    assert_eq!(report.domain, ChildRuntimeDomain::App);
    assert_eq!(
        report.evidence_recorded.signal,
        ChildDomainObservedSignal::ObserveOnly.into_observed_state()
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
    .required("browser ambiguous navigation flow");

    let ai_request = report.ai_analysis_requested.required("browser ai request");
    let ai_completed = report
        .ai_analysis_completed
        .required("browser ai completed");
    let policy_request = report
        .policy_evaluation_requested
        .required("browser policy request");
    let violation = report
        .policy_violation_detected
        .required("browser policy violation");
    let notification = report
        .notification_requested
        .required("browser notification request");

    assert_eq!(report.domain, ChildRuntimeDomain::Browser);
    assert_eq!(
        ai_request.evidence_refs,
        vec![report.evidence_recorded.evidence_ref]
    );
    assert_eq!(ai_completed.source_ai_request_id, ai_request.ai_request_id);
    assert_eq!(policy_request.evidence_refs, ai_completed.evidence_refs);
    assert_eq!(violation.evidence_refs, policy_request.evidence_refs);
    assert_eq!(
        notification.source_policy_violation_id,
        violation.violation_id
    );
}

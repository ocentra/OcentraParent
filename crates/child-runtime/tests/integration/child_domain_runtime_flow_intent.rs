use ocentra_parent_agent_protocol::{
    child_domain_observed_event, constants, ChildDomainAiAnalysisRequirement, ChildDomainEventType,
    ChildDomainObservedEventProfile, ChildDomainObservedSignal,
    ChildDomainPolicyEvaluationRequirement, ChildDomainRefSuffix, ChildRuntimeDomain,
};

#[tokio::test]
async fn child_domain_runtime_flow_keeps_feature_ai_policy_and_notification_decoupled_by_events() {
    let reports = ocentra_child_runtime::publish_default_child_domain_runtime_flows()
        .await
        .expect(constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED);

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
            .expect(constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED);
        let policy_violation_detected = report
            .policy_violation_detected
            .as_ref()
            .expect(constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED);
        let notification_requested = report
            .notification_requested
            .as_ref()
            .expect(constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED);
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
            notification_requested.source_policy_violation_id,
            policy_violation_detected.violation_id
        );
        if let Some(ai_analysis_requested) = &report.ai_analysis_requested {
            let ai_analysis_completed = report
                .ai_analysis_completed
                .as_ref()
                .expect(constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED);
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

    assert!(reports
        .iter()
        .filter(|report| report.ai_analysis_requested.is_some())
        .all(|report| matches!(
            report.domain,
            ChildRuntimeDomain::Browser | ChildRuntimeDomain::Screen
        )));
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
        .expect(constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED);

    assert!(report.ai_analysis_requested.is_some());
    assert!(report.ai_analysis_completed.is_some());
    assert!(report.policy_evaluation_requested.is_none());
    assert!(report.policy_violation_detected.is_none());
    assert!(report.notification_requested.is_none());
}

#[tokio::test]
async fn child_domain_observe_only_intent_records_evidence_without_side_effects() {
    let event = ocentra_network_core::network_observed_event(
        ocentra_network_core::NetworkObservationIntent::TelemetryObservationOnly,
    );

    let report = ocentra_child_runtime::publish_child_domain_observed_event(event)
        .await
        .expect(constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED);

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
    assert!(report.ai_analysis_requested.is_none());
    assert!(report.ai_analysis_completed.is_none());
    assert!(report.policy_evaluation_requested.is_none());
    assert!(report.policy_violation_detected.is_none());
    assert!(report.notification_requested.is_none());
}

#[tokio::test]
async fn child_domain_ambiguous_feature_intent_routes_ai_then_policy_then_notification() {
    let event = ocentra_app_game_core::app_game_observed_event(
        ocentra_app_game_core::AppGameObservationIntent::AmbiguousUsageRequiresAi,
    );

    let report = ocentra_child_runtime::publish_child_domain_observed_event(event)
        .await
        .expect(constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED);

    assert_eq!(report.domain, ChildRuntimeDomain::AppGame);
    assert_eq!(
        report.evidence_recorded.ai_analysis_requirement,
        ChildDomainAiAnalysisRequirement::Required
    );
    assert!(report.ai_analysis_requested.is_some());
    assert!(report.ai_analysis_completed.is_some());
    assert!(report.policy_evaluation_requested.is_some());
    assert!(report.policy_violation_detected.is_some());
    assert!(report.notification_requested.is_some());
}

#[tokio::test]
async fn child_domain_unknown_app_intent_routes_ai_then_policy_then_notification() {
    let event = ocentra_app_core::app_observed_event(
        ocentra_app_core::AppObservationIntent::UnknownAppRequiresAi,
    );

    let report = ocentra_child_runtime::publish_child_domain_observed_event(event)
        .await
        .expect(constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED);

    assert_eq!(report.domain, ChildRuntimeDomain::App);
    assert_eq!(
        report.evidence_recorded.ai_analysis_requirement,
        ChildDomainAiAnalysisRequirement::Required
    );
    assert!(report.ai_analysis_requested.is_some());
    assert!(report.ai_analysis_completed.is_some());
    assert!(report.policy_evaluation_requested.is_some());
    assert!(report.policy_violation_detected.is_some());
    assert!(report.notification_requested.is_some());
}

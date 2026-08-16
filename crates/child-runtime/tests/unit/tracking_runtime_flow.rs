use ocentra_child_runtime::tracking_runtime_flow as child_runtime_tracking;
use ocentra_evidence::PrivatePayloadState;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::identifiers::{
    tracking_ai_request_id_from_evidence_ref, tracking_evidence_ref_from_observation_id,
    tracking_notification_id_from_violation_id, tracking_violation_id_from_ai_request_and_rule_ref,
    TrackingNotificationChannel, TrackingPlaceCategory, TrackingPolicyRuleRef, TrackingTimestamp,
};
use ocentra_parent_agent_protocol::tracking::runtime_event::{
    TrackingAiBoundaryMode, TrackingNotificationMode, TrackingParentActionRequirement,
    TrackingRuntimeEnabledState, TrackingRuntimeMode,
};
use ocentra_tracking_core::runtime_flow::TrackingPortalNotificationCandidateState;

mod support;

use support::ResultRequiredExt;

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

#[tokio::test]
async fn child_runtime_routes_tracking_observation_through_ai_policy_and_notification_boundaries() {
    let event = ocentra_tracking_core::runtime_flow::default_location_observed_event();
    let flow_report = child_runtime_tracking::publish_child_tracking_location_observed_event(event)
        .await
        .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);
    assert_tracking_runtime_subscription_reports(&flow_report);
    assert_tracking_runtime_observation_branches(&flow_report);
    assert_tracking_runtime_policy_branches(&flow_report);
}

#[tokio::test]
async fn child_runtime_keeps_observe_only_tracking_flow_out_of_policy_and_notification() {
    let mut event = ocentra_tracking_core::runtime_flow::default_location_observed_event();
    event.config.tracking_enabled_state = TrackingRuntimeEnabledState::Enabled;
    event.config.tracking_mode = TrackingRuntimeMode::ObserveOnly;
    let flow_report = child_runtime_tracking::publish_child_tracking_location_observed_event(event)
        .await
        .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);

    assert_eq!(
        flow_report.evidence_recorded.parent_action_requirement,
        TrackingParentActionRequirement::NotRequired
    );
    assert!(flow_report.policy_violation_detected.is_none());
    assert!(flow_report.parent_notification_requested.is_none());
}

#[tokio::test]
async fn child_runtime_keeps_ai_disabled_tracking_flow_out_of_ai_policy_and_notification() {
    let mut event = ocentra_tracking_core::runtime_flow::default_location_observed_event();
    event.config.ai_boundary_mode = TrackingAiBoundaryMode::Disabled;
    let flow_report = child_runtime_tracking::publish_child_tracking_location_observed_event(event)
        .await
        .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);

    assert!(flow_report.ai_analysis_requested.is_none());
    assert!(flow_report.nearby_place_classified.is_none());
    assert!(flow_report.policy_violation_detected.is_none());
    assert!(flow_report.parent_notification_requested.is_none());
}

#[tokio::test]
async fn child_runtime_honors_disabled_notification_mode_without_suppressing_policy_detection() {
    let mut event = ocentra_tracking_core::runtime_flow::default_location_observed_event();
    event.config.notification_mode = TrackingNotificationMode::Disabled;
    let flow_report = child_runtime_tracking::publish_child_tracking_location_observed_event(event)
        .await
        .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);

    assert_eq!(
        flow_report
            .policy_violation_detected
            .as_ref()
            .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .policy_rule_ref,
        TrackingPolicyRuleRef::parse(constants::tracking_runtime::POLICY_RULE_EXPECTED_PLACE)
            .required(constants::tracking_runtime::POLICY_RULE_EXPECTED_PLACE)
    );
    assert!(flow_report.alert_decision.is_none());
    assert!(flow_report.parent_notification_requested.is_none());
}

fn assert_tracking_runtime_subscription_reports(
    flow_report: &child_runtime_tracking::TrackingRuntimeEventFlowReport,
) {
    assert_eq!(
        flow_report
            .tracking_subscription_report
            .subscriber_id
            .as_str(),
        constants::tracking_runtime::SUBSCRIBER_CHILD_TRACKING_OBSERVER
    );
    assert_eq!(
        flow_report
            .child_ai_subscription_report
            .subscriber_id
            .as_str(),
        constants::tracking_runtime::SUBSCRIBER_CHILD_AI_TRACKING_ANALYZER
    );
    assert_eq!(
        flow_report
            .child_policy_subscription_report
            .subscriber_id
            .as_str(),
        constants::tracking_runtime::SUBSCRIBER_CHILD_POLICY_TRACKING_ANALYZER
    );
    assert_eq!(
        flow_report
            .child_notification_subscription_report
            .subscriber_id
            .as_str(),
        constants::tracking_runtime::SUBSCRIBER_CHILD_NOTIFICATION_POLICY_BRIDGE
    );
}

fn assert_tracking_runtime_observation_branches(
    flow_report: &child_runtime_tracking::TrackingRuntimeEventFlowReport,
) {
    let expected_evidence_ref = tracking_evidence_ref_from_observation_id(
        &flow_report.evidence_recorded.source_observation_id,
    );
    let ai_request = flow_report
        .ai_analysis_requested
        .as_ref()
        .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);
    let nearby_place = flow_report
        .nearby_place_classified
        .as_ref()
        .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);

    assert_eq!(
        flow_report.evidence_recorded.evidence_ref,
        expected_evidence_ref
    );
    assert_eq!(
        flow_report.evidence_recorded.source_observed_at,
        TrackingTimestamp::parse(constants::tracking_runtime::DEFAULT_OBSERVED_AT)
            .required(constants::tracking_runtime::DEFAULT_OBSERVED_AT)
    );
    assert_eq!(
        flow_report.evidence_recorded.parent_action_requirement,
        TrackingParentActionRequirement::Required
    );
    assert_eq!(
        ai_request.evidence_refs,
        vec![flow_report.evidence_recorded.evidence_ref.clone()]
    );
    assert_eq!(
        ai_request.source_observed_at,
        flow_report.evidence_recorded.source_observed_at
    );
    assert_eq!(
        ai_request.ai_request_id,
        tracking_ai_request_id_from_evidence_ref(&flow_report.evidence_recorded.evidence_ref)
    );
    assert_eq!(
        ai_request.private_payload_state,
        PrivatePayloadState::Excluded
    );
    assert_eq!(
        nearby_place.place_category,
        TrackingPlaceCategory::parse(constants::tracking_runtime::PLACE_CATEGORY_HOSPITAL)
            .required(constants::tracking_runtime::PLACE_CATEGORY_HOSPITAL)
    );
    assert_eq!(
        nearby_place.source_location_evidence_ref,
        flow_report.evidence_recorded.evidence_ref
    );
    assert_eq!(
        nearby_place.source_observed_at,
        flow_report.evidence_recorded.source_observed_at
    );
    assert_eq!(
        nearby_place.provider_kind,
        constants::tracking_runtime::NEARBY_PROVIDER_KIND_LOCAL_CACHE
    );
    assert_eq!(
        nearby_place.ambiguity_state,
        constants::tracking_runtime::NEARBY_PLACE_AMBIGUITY_CLEAR
    );
}

fn assert_tracking_runtime_policy_branches(
    flow_report: &child_runtime_tracking::TrackingRuntimeEventFlowReport,
) {
    let ai_request = flow_report
        .ai_analysis_requested
        .as_ref()
        .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);
    let ai_boundary_decision = flow_report
        .ai_boundary_decision
        .as_ref()
        .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);
    let policy_violation = flow_report
        .policy_violation_detected
        .as_ref()
        .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);
    let parent_notification = flow_report
        .parent_notification_requested
        .as_ref()
        .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);

    assert_eq!(
        ai_boundary_decision.decision_state,
        constants::tracking_runtime::AI_RESULT_ACCEPTED_AS_EVIDENCE
    );
    assert_eq!(
        policy_violation.policy_rule_ref,
        TrackingPolicyRuleRef::parse(constants::tracking_runtime::POLICY_RULE_EXPECTED_PLACE)
            .required(constants::tracking_runtime::POLICY_RULE_EXPECTED_PLACE)
    );
    assert_eq!(
        policy_violation.violation_id,
        tracking_violation_id_from_ai_request_and_rule_ref(
            &ai_request.ai_request_id,
            &policy_violation.policy_rule_ref,
        )
    );
    assert_eq!(
        parent_notification.channel,
        TrackingNotificationChannel::parse(
            constants::tracking_runtime::NOTIFICATION_CHANNEL_PARENT_PORTAL,
        )
        .required(constants::tracking_runtime::NOTIFICATION_CHANNEL_PARENT_PORTAL)
    );
    assert_eq!(
        parent_notification.notification_id,
        tracking_notification_id_from_violation_id(&policy_violation.violation_id)
    );
    assert_eq!(
        ocentra_tracking_core::runtime_flow::tracking_observation_portal_notification_candidate_state(
            parent_notification
        ),
        TrackingPortalNotificationCandidateState::Candidate
    );
}

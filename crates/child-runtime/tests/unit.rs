use ocentra_evidence::PrivatePayloadState;
use ocentra_parent_agent_protocol::{
    constants, default_tracking_retention_settings_write_request, AgentCommandEnvelope,
    AgentCommandName, AgentMessageTarget, AgentPeer, AgentPeerRole, AgentRoute, LogFields,
    TrackingAiBoundaryMode, TrackingConfigEffectiveState, TrackingConfigUpdateEventName,
    TrackingConfigUpdateResponseState, TrackingDurableSettingsPersistenceState,
    TrackingEvidenceRef, TrackingNotificationChannel, TrackingParentActionRequirement,
    TrackingPlaceCategory, TrackingPolicyRuleRef, TrackingRetentionSettingsWriteRequest,
    TrackingRuntimeEnabledState, TrackingRuntimeMode, AGENT_PROTOCOL_SCHEMA_VERSION,
};
use ocentra_tracking_core::TrackingPortalNotificationCandidateState;

#[path = "unit/runtime_gate.rs"]
mod runtime_gate;

#[test]
fn child_runtime_declares_tracking_core_dependency() {
    assert_eq!(ocentra_child_runtime::CRATE_NAME, "ocentra-child-runtime");
    assert_eq!(
        ocentra_child_runtime::tracking_runtime_crate_name(),
        ocentra_tracking_core::CRATE_NAME
    );
}

#[tokio::test]
async fn child_runtime_routes_parent_config_event_through_named_subscribers_to_child_tracking_core()
{
    let request: TrackingRetentionSettingsWriteRequest =
        default_tracking_retention_settings_write_request();
    let command = command_envelope(request.clone());
    let parent_event =
        ocentra_child_runtime::parent_tracking_config_updated_event_from_command(&command, request);
    let child_event =
        ocentra_child_runtime::child_tracking_config_updated_event_from_parent(&parent_event);
    let flow_report =
        ocentra_child_runtime::publish_parent_tracking_config_updated_event(&parent_event)
            .await
            .expect(constants::tracking_config_update::ERROR_PARENT_CONFIG_EVENT_APPLIED);
    let applied_report = flow_report.applied_report;

    assert_eq!(
        applied_report.parent_event_type,
        TrackingConfigUpdateEventName::Parent
    );
    assert_eq!(
        applied_report.child_event_type,
        TrackingConfigUpdateEventName::Child
    );
    assert_eq!(
        child_event.parent_event_type,
        TrackingConfigUpdateEventName::Parent
    );
    assert_eq!(
        flow_report
            .parent_subscription_report
            .subscriber_id
            .as_str(),
        constants::tracking_config_update::SUBSCRIBER_PARENT_TRACKING_CONFIG_RELAY
    );
    assert_eq!(
        flow_report.child_subscription_report.subscriber_id.as_str(),
        constants::tracking_config_update::SUBSCRIBER_CHILD_TRACKING_CONFIG_APPLIER
    );
    assert_eq!(
        flow_report
            .applied_subscription_report
            .subscriber_id
            .as_str(),
        constants::tracking_config_update::SUBSCRIBER_CHILD_TRACKING_CONFIG_APPLIED_RECORDER
    );
    assert_eq!(
        flow_report.parent_request_report.response.response_state,
        TrackingConfigUpdateResponseState::Applied
    );
    assert_eq!(
        flow_report
            .parent_request_report
            .response
            .effective_tracking_state,
        TrackingConfigEffectiveState::Enabled
    );
    assert_eq!(
        flow_report
            .parent_request_report
            .publish_report
            .subscriber_count,
        1
    );
    assert_eq!(
        flow_report
            .parent_request_report
            .publish_report
            .handled_count,
        1
    );
    assert_eq!(
        applied_report.applied_event_type,
        TrackingConfigUpdateEventName::Applied
    );
    assert_eq!(
        applied_report.response_state,
        TrackingConfigUpdateResponseState::Applied
    );
    assert_eq!(
        applied_report.effective_tracking_state,
        TrackingConfigEffectiveState::Enabled
    );
    assert!(applied_report.applied_state.local_service_state_revision > 0);
    assert_eq!(
        flow_report
            .parent_request_report
            .response
            .durable_settings_persistence_state,
        TrackingDurableSettingsPersistenceState::Persisted
    );
    assert!(ocentra_child_runtime::tracking_retention_settings_durable_store_path().exists());
}

#[tokio::test]
async fn parent_tracking_config_flow_can_attach_once_to_runtime_owned_bus() {
    let request: TrackingRetentionSettingsWriteRequest =
        default_tracking_retention_settings_write_request();
    let command = command_envelope(request.clone());
    let parent_event =
        ocentra_child_runtime::parent_tracking_config_updated_event_from_command(&command, request);
    let runtime_flow = ocentra_child_runtime::TrackingConfigUpdateEventFlow::new()
        .await
        .expect(constants::tracking_config_update::ERROR_PARENT_CONFIG_EVENT_APPLIED);
    let metrics_before = runtime_flow.metrics_snapshot().await;

    let flow_report = runtime_flow
        .publish_parent_config_updated(&parent_event)
        .await
        .expect(constants::tracking_config_update::ERROR_PARENT_CONFIG_EVENT_APPLIED);
    let metrics_after = runtime_flow.metrics_snapshot().await;
    let journal = runtime_flow.journal_snapshot().await;

    assert_eq!(metrics_before.subscription_count, 3);
    assert_eq!(metrics_after.subscription_count, 3);
    assert_eq!(
        flow_report.parent_request_report.response.response_state,
        TrackingConfigUpdateResponseState::Applied
    );
    assert_eq!(
        flow_report.applied_report.child_event_type,
        TrackingConfigUpdateEventName::Child
    );
    assert_eq!(journal.len(), 3);
    assert_eq!(
        journal[0].contract.event_type.as_str(),
        constants::tracking_config_update::PARENT_EVENT_TYPE
    );
    assert_eq!(
        journal[1].contract.event_type.as_str(),
        constants::tracking_config_update::CHILD_EVENT_TYPE
    );
    assert_eq!(
        journal[2].contract.event_type.as_str(),
        constants::tracking_config_update::APPLIED_EVENT_TYPE
    );
}

#[tokio::test]
async fn child_runtime_routes_tracking_observation_through_ai_policy_and_notification_boundaries() {
    let event = ocentra_tracking_core::default_location_observed_event();
    let flow_report = ocentra_child_runtime::publish_child_tracking_location_observed_event(event)
        .await
        .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);

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
    assert_eq!(
        flow_report.evidence_recorded.evidence_ref,
        TrackingEvidenceRef::parse(constants::tracking_runtime::DEFAULT_EVIDENCE_REF)
            .expect(constants::tracking_runtime::DEFAULT_EVIDENCE_REF)
    );
    assert_eq!(
        flow_report.evidence_recorded.parent_action_requirement,
        TrackingParentActionRequirement::Required
    );
    assert_eq!(
        flow_report
            .ai_analysis_requested
            .as_ref()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .evidence_refs,
        vec![
            TrackingEvidenceRef::parse(constants::tracking_runtime::DEFAULT_EVIDENCE_REF)
                .expect(constants::tracking_runtime::DEFAULT_EVIDENCE_REF)
        ]
    );
    assert_eq!(
        flow_report
            .ai_analysis_requested
            .as_ref()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .private_payload_state,
        PrivatePayloadState::Excluded
    );
    assert_eq!(
        flow_report
            .nearby_place_classified
            .as_ref()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .place_category,
        TrackingPlaceCategory::parse(constants::tracking_runtime::PLACE_CATEGORY_HOSPITAL)
            .expect(constants::tracking_runtime::PLACE_CATEGORY_HOSPITAL)
    );
    assert_eq!(
        flow_report
            .ai_boundary_decision
            .as_ref()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .decision_state,
        constants::tracking_runtime::AI_RESULT_ACCEPTED_AS_EVIDENCE
    );
    let policy_violation = flow_report
        .policy_violation_detected
        .as_ref()
        .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);
    let parent_notification = flow_report
        .parent_notification_requested
        .as_ref()
        .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);
    assert_eq!(
        policy_violation.policy_rule_ref,
        TrackingPolicyRuleRef::parse(constants::tracking_runtime::POLICY_RULE_EXPECTED_PLACE)
            .expect(constants::tracking_runtime::POLICY_RULE_EXPECTED_PLACE)
    );
    assert_eq!(
        parent_notification.channel,
        TrackingNotificationChannel::parse(
            constants::tracking_runtime::NOTIFICATION_CHANNEL_PARENT_PORTAL,
        )
        .expect(constants::tracking_runtime::NOTIFICATION_CHANNEL_PARENT_PORTAL)
    );
    assert_eq!(
        ocentra_tracking_core::tracking_observation_portal_notification_candidate_state(
            parent_notification
        ),
        TrackingPortalNotificationCandidateState::Candidate
    );
}

#[tokio::test]
async fn child_runtime_keeps_observe_only_tracking_flow_out_of_policy_and_notification() {
    let mut event = ocentra_tracking_core::default_location_observed_event();
    event.config.tracking_enabled_state = TrackingRuntimeEnabledState::Enabled;
    event.config.tracking_mode = TrackingRuntimeMode::ObserveOnly;
    let flow_report = ocentra_child_runtime::publish_child_tracking_location_observed_event(event)
        .await
        .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);

    assert_eq!(
        flow_report.evidence_recorded.parent_action_requirement,
        TrackingParentActionRequirement::NotRequired
    );
    assert!(flow_report.policy_violation_detected.is_none());
    assert!(flow_report.parent_notification_requested.is_none());
}

#[tokio::test]
async fn child_runtime_keeps_ai_disabled_tracking_flow_out_of_ai_policy_and_notification() {
    let mut event = ocentra_tracking_core::default_location_observed_event();
    event.config.ai_boundary_mode = TrackingAiBoundaryMode::Disabled;
    let flow_report = ocentra_child_runtime::publish_child_tracking_location_observed_event(event)
        .await
        .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);

    assert!(flow_report.ai_analysis_requested.is_none());
    assert!(flow_report.nearby_place_classified.is_none());
    assert!(flow_report.policy_violation_detected.is_none());
    assert!(flow_report.parent_notification_requested.is_none());
}

fn command_envelope(request: TrackingRetentionSettingsWriteRequest) -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: String::from(request.command_id),
        sent_at: constants::tracking_retention_settings_write::ACCEPTED_AT.to_string(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform:
                ocentra_parent_agent_protocol::policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS
                    .to_string(),
            route: AgentRoute::Localhost,
        },
        command: AgentCommandName::AgentActivityTrackingRetentionSettingsWrite,
        payload: LogFields::new(),
    }
}

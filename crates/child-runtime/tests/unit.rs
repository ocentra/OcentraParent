use ocentra_parent_agent_protocol::{
    constants, default_tracking_retention_settings_write_request, AgentCommandEnvelope,
    AgentCommandName, AgentMessageTarget, AgentPeer, AgentPeerRole, AgentRoute, LogFields,
    TrackingRetentionSettingsWriteRequest, AGENT_PROTOCOL_SCHEMA_VERSION,
};

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
        constants::tracking_config_update::PARENT_EVENT_TYPE
    );
    assert_eq!(
        applied_report.child_event_type,
        constants::tracking_config_update::CHILD_EVENT_TYPE
    );
    assert_eq!(
        child_event.parent_event_type,
        constants::tracking_config_update::PARENT_EVENT_TYPE
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
        flow_report.parent_request_report.response.response_state,
        constants::tracking_config_update::RESPONSE_STATE_APPLIED
    );
    assert_eq!(
        flow_report
            .parent_request_report
            .response
            .effective_tracking_state,
        constants::tracking_config_update::EFFECTIVE_STATE_ENABLED
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
    assert!(applied_report.applied_state.local_service_state_revision > 0);
    assert!(ocentra_child_runtime::tracking_retention_settings_durable_store_path().exists());
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
        constants::tracking_runtime::DEFAULT_EVIDENCE_REF
    );
    assert_eq!(
        flow_report.ai_analysis_requested.evidence_refs,
        vec![constants::tracking_runtime::DEFAULT_EVIDENCE_REF.to_string()]
    );
    assert!(
        !flow_report
            .ai_analysis_requested
            .raw_private_payload_included
    );
    assert_eq!(
        flow_report.nearby_place_classified.place_category,
        constants::tracking_runtime::PLACE_CATEGORY_HOSPITAL
    );
    assert_eq!(
        flow_report.policy_violation_detected.policy_rule_ref,
        constants::tracking_runtime::POLICY_RULE_EXPECTED_PLACE
    );
    assert_eq!(
        flow_report.parent_notification_requested.channel,
        constants::tracking_runtime::NOTIFICATION_CHANNEL_PARENT_PORTAL
    );
    assert!(
        ocentra_tracking_core::tracking_observation_is_portal_notification_candidate(
            &flow_report.parent_notification_requested
        )
    );
}

fn command_envelope(request: TrackingRetentionSettingsWriteRequest) -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: request.command_id,
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

use ocentra_parent_agent_protocol::{
    constants, default_tracking_config_update_request, AgentRoute,
    ParentTrackingConfigUpdatedEvent, TrackingConfigPolicyDecisionState,
    TrackingConfigPortalUpdateKind, TrackingConfigUpdateResponseState,
    TrackingConfigUpdateTarget, TrackingConfigUpdateTargetScope, TrackingSourceMessageId,
    TrackingSourcePeerId, TrackingTargetDeviceId, TrackingTargetPlatform,
};
use ocentra_parent_runtime_core::{
    publish_parent_tracking_config_updated_event_flow, ChildAcknowledgementState,
    ChildRuntimePublishState, ParentRuntimeOriginState,
};

#[tokio::test]
async fn parent_runtime_tracking_config_flow_publishes_approved_chain_and_child_runtime_ack() {
    let event = parent_tracking_config_event(TrackingConfigUpdateTargetScope::ChildDevice);

    let flow_report = publish_parent_tracking_config_updated_event_flow(
        "event.parent-controller.parent-action.received.1",
        &event,
        ChildAcknowledgementState::Required,
        ParentRuntimeOriginState::TrustedLocalUi,
    )
    .await
    .expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        flow_report.change_requested_event.previous_event_ref,
        "event.parent-controller.parent-action.received.1"
    );
    assert_eq!(
        flow_report.policy_decision_event.decision_state,
        TrackingConfigPolicyDecisionState::Approved
    );
    assert_eq!(
        flow_report.dispatch_event.decision.child_runtime_publish_state,
        ChildRuntimePublishState::Publish
    );
    assert!(flow_report.change_approved_event.is_some());
    assert!(flow_report.change_rejected_event.is_none());
    assert_eq!(
        flow_report.audit_event.audit_outcome,
        ocentra_parent_agent_protocol::TrackingConfigAuditOutcome::Committed
    );
    assert_eq!(
        flow_report.portal_event.update_kind,
        TrackingConfigPortalUpdateKind::TrackingConfigState
    );
    assert_eq!(
        flow_report.parent_request_report.response.response_state,
        TrackingConfigUpdateResponseState::Applied
    );
    assert!(flow_report.child_runtime_flow.is_some());
}

#[tokio::test]
async fn parent_runtime_tracking_config_flow_rejects_untrusted_origin_without_child_dispatch() {
    let event = parent_tracking_config_event(TrackingConfigUpdateTargetScope::ChildDevice);

    let flow_report = publish_parent_tracking_config_updated_event_flow(
        "event.parent-controller.parent-action.received.2",
        &event,
        ChildAcknowledgementState::Required,
        ParentRuntimeOriginState::Untrusted,
    )
    .await
    .expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        flow_report.policy_decision_event.decision_state,
        TrackingConfigPolicyDecisionState::Rejected
    );
    assert_eq!(
        flow_report.dispatch_event.decision.child_runtime_publish_state,
        ChildRuntimePublishState::DoNotPublish
    );
    assert!(flow_report.change_approved_event.is_none());
    assert_eq!(
        flow_report
            .change_rejected_event
            .as_ref()
            .expect(constants::error::AGENT_EVENT_SERIALIZES)
            .rejection_reason_code,
        constants::tracking_config_update::REJECTION_REASON_CHILD_RUNTIME_DISPATCH_BLOCKED
    );
    assert_eq!(
        flow_report.portal_event.update_kind,
        TrackingConfigPortalUpdateKind::ManualRequiredState
    );
    assert_eq!(flow_report.portal_event.visible_manual_required, true);
    assert_eq!(flow_report.portal_event.visible_unavailable, true);
    assert_eq!(
        flow_report.parent_request_report.response.response_state,
        TrackingConfigUpdateResponseState::Rejected
    );
    assert!(flow_report.child_runtime_flow.is_none());
}

fn parent_tracking_config_event(
    scope: TrackingConfigUpdateTargetScope,
) -> ParentTrackingConfigUpdatedEvent {
    let request = default_tracking_config_update_request();
    ParentTrackingConfigUpdatedEvent {
        source_command_id: request.command_id.clone(),
        source_message_id: TrackingSourceMessageId::parse(
            constants::tracking_retention_settings_write::COMMAND_ID,
        )
        .expect(constants::tracking_retention_settings_write::COMMAND_ID),
        source_peer_id: TrackingSourcePeerId::parse(constants::peer::PORTAL_DEV)
            .expect(constants::peer::PORTAL_DEV),
        target: TrackingConfigUpdateTarget {
            scope,
            device_id: TrackingTargetDeviceId::parse(constants::peer::LOCAL_DEV_AGENT)
                .expect(constants::peer::LOCAL_DEV_AGENT),
            platform: TrackingTargetPlatform::parse(
                ocentra_parent_agent_protocol::policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS,
            )
            .expect(
                ocentra_parent_agent_protocol::policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS,
            ),
            route: AgentRoute::Localhost,
        },
        config: request,
    }
}

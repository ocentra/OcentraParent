use ocentra_child_runtime::tracking_config_update_flow as child_runtime_config;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::tracking::config_update_event::{
    child_tracking_config_updated_event_from_parent, default_tracking_config_update_request,
    parent_tracking_config_updated_event_from_command, TrackingConfigEffectiveState,
    TrackingConfigUpdateEventName, TrackingConfigUpdateRequest, TrackingConfigUpdateResponseState,
    TrackingConfigUpdateTargetScope,
};
use ocentra_parent_agent_protocol::tracking::retention_settings_write_command::TrackingDurableSettingsPersistenceState;
use ocentra_parent_agent_protocol::tracking::runtime_event::TrackingRuntimeEnabledState;
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentMessageTarget, AgentPeer, AgentPeerRole,
    AgentRoute,
};
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;

mod support;

use support::ResultRequiredExt;

#[tokio::test]
async fn child_runtime_routes_parent_config_event_through_named_subscribers_to_child_tracking_core()
{
    let request: TrackingConfigUpdateRequest = default_tracking_config_update_request();
    let command = command_envelope(request.clone());
    let parent_event = parent_tracking_config_updated_event_from_command(&command, request);
    let child_event = child_tracking_config_updated_event_from_parent(&parent_event);
    let flow_report =
        child_runtime_config::publish_parent_tracking_config_updated_event(&parent_event)
            .await
            .required(constants::tracking_config_update::ERROR_PARENT_CONFIG_EVENT_APPLIED);
    assert_tracking_config_flow_types(&flow_report, &child_event);
    assert_tracking_config_flow_subscribers(&flow_report);
    assert_tracking_config_flow_state(&flow_report);
}

#[tokio::test]
async fn parent_tracking_config_flow_can_attach_once_to_runtime_owned_bus() {
    let request: TrackingConfigUpdateRequest = default_tracking_config_update_request();
    let command = command_envelope(request.clone());
    let parent_event = parent_tracking_config_updated_event_from_command(&command, request);
    let runtime_flow = child_runtime_config::TrackingConfigUpdateEventFlow::new()
        .await
        .required(constants::tracking_config_update::ERROR_PARENT_CONFIG_EVENT_APPLIED);
    let metrics_before = runtime_flow.metrics_snapshot().await;

    let flow_report = runtime_flow
        .publish_parent_config_updated(&parent_event)
        .await
        .required(constants::tracking_config_update::ERROR_PARENT_CONFIG_EVENT_APPLIED);
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
    assert_eq!(
        flow_report.child_event.target.scope,
        TrackingConfigUpdateTargetScope::ChildDevice
    );
    assert_eq!(
        flow_report.applied_event.target.scope,
        TrackingConfigUpdateTargetScope::ChildDevice
    );
    assert_eq!(journal.len(), 3);
    assert_eq!(
        journal[0].contract.event_type.as_str(),
        constants::tracking_config_update::PARENT_EVENT_TYPE
    );
    assert_eq!(
        journal[1].contract.event_type.as_str(),
        constants::tracking_config_update::APPLIED_EVENT_TYPE
    );
    assert_eq!(
        journal[2].contract.event_type.as_str(),
        constants::tracking_config_update::CHILD_EVENT_TYPE
    );
}

#[tokio::test]
async fn child_runtime_applies_disabled_tracking_runtime_config_without_rejecting_request() {
    let mut request: TrackingConfigUpdateRequest = default_tracking_config_update_request();
    request.runtime_config.tracking_enabled_state = TrackingRuntimeEnabledState::Disabled;
    let command = command_envelope(request.clone());
    let parent_event = parent_tracking_config_updated_event_from_command(&command, request);

    let flow_report =
        child_runtime_config::publish_parent_tracking_config_updated_event(&parent_event)
            .await
            .required(constants::tracking_config_update::ERROR_PARENT_CONFIG_EVENT_APPLIED);

    assert_eq!(
        flow_report
            .parent_request_report
            .response
            .effective_tracking_state,
        TrackingConfigEffectiveState::Disabled
    );
    assert_eq!(
        flow_report.applied_report.effective_tracking_state,
        TrackingConfigEffectiveState::Disabled
    );
}

fn command_envelope(request: TrackingConfigUpdateRequest) -> AgentCommandEnvelope {
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

fn assert_tracking_config_flow_types(
    flow_report: &child_runtime_config::TrackingConfigUpdateEventFlowReport,
    child_event: &ocentra_parent_agent_protocol::tracking::config_update_event::ChildTrackingConfigUpdatedEvent,
) {
    assert_eq!(
        flow_report.applied_report.parent_event_type,
        TrackingConfigUpdateEventName::Parent
    );
    assert_eq!(
        flow_report.applied_report.child_event_type,
        TrackingConfigUpdateEventName::Child
    );
    assert_eq!(
        child_event.parent_event_type,
        TrackingConfigUpdateEventName::Parent
    );
    assert_eq!(
        flow_report.child_event.parent_event_type,
        TrackingConfigUpdateEventName::Parent
    );
    assert_eq!(
        flow_report.applied_event.child_event_type,
        TrackingConfigUpdateEventName::Child
    );
    assert_eq!(
        flow_report.applied_report.applied_event_type,
        TrackingConfigUpdateEventName::Applied
    );
}

fn assert_tracking_config_flow_subscribers(
    flow_report: &child_runtime_config::TrackingConfigUpdateEventFlowReport,
) {
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
}

fn assert_tracking_config_flow_state(
    flow_report: &child_runtime_config::TrackingConfigUpdateEventFlowReport,
) {
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
        flow_report.applied_report.response_state,
        TrackingConfigUpdateResponseState::Applied
    );
    assert_eq!(
        flow_report.applied_report.effective_tracking_state,
        TrackingConfigEffectiveState::Enabled
    );
    assert_eq!(
        flow_report.applied_event.response_state,
        TrackingConfigUpdateResponseState::Applied
    );
    assert!(
        flow_report
            .applied_report
            .applied_state
            .local_service_state_revision
            > 0
    );
    assert_eq!(
        flow_report
            .parent_request_report
            .response
            .durable_settings_persistence_state,
        TrackingDurableSettingsPersistenceState::Persisted
    );
    assert!(child_runtime_config::tracking_retention_settings_durable_store_path().exists());
}

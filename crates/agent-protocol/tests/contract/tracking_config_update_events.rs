use ocentra_eventing::DomainEvent;
use ocentra_parent_agent_protocol::{
    child_tracking_config_updated_event_from_parent, constants,
    default_tracking_retention_settings_write_request,
    parent_tracking_config_updated_event_from_command,
    tracking_config_update_applied_event_from_child, AgentCommandEnvelope, AgentCommandName,
    AgentMessageTarget, AgentPeer, AgentPeerRole, AgentRoute, LogFields,
    TrackingConfigEffectiveState, TrackingConfigUpdateEventName,
    TrackingConfigUpdateResponseState,
    TrackingDurableSettingsPersistenceState,
};

#[test]
fn tracking_config_update_event_names_serialize_exact_contract_text() {
    assert_eq!(
        serde_json::to_value(TrackingConfigUpdateEventName::Parent)
            .expect(constants::error::AGENT_EVENT_SERIALIZES),
        constants::tracking_config_update::PARENT_EVENT_TYPE
    );
    assert_eq!(
        serde_json::to_value(TrackingConfigUpdateEventName::Child)
            .expect(constants::error::AGENT_EVENT_SERIALIZES),
        constants::tracking_config_update::CHILD_EVENT_TYPE
    );
    assert_eq!(
        serde_json::to_value(TrackingConfigUpdateEventName::Applied)
            .expect(constants::error::AGENT_EVENT_SERIALIZES),
        constants::tracking_config_update::APPLIED_EVENT_TYPE
    );
}

#[test]
fn tracking_config_update_applied_event_serializes_durable_child_runtime_result() {
    let request = default_tracking_retention_settings_write_request();
    let command = command_envelope(&request.command_id);
    let parent_event = parent_tracking_config_updated_event_from_command(&command, request);
    let child_event = child_tracking_config_updated_event_from_parent(&parent_event);
    let applied_event = tracking_config_update_applied_event_from_child(
        &child_event,
        TrackingConfigUpdateResponseState::Applied,
        TrackingConfigEffectiveState::Enabled,
        7,
        TrackingDurableSettingsPersistenceState::Persisted,
    );
    let serialized =
        serde_json::to_value(&applied_event).expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        applied_event
            .contract()
            .expect(constants::error::AGENT_EVENT_SERIALIZES)
            .event_type
            .as_str(),
        constants::tracking_config_update::APPLIED_EVENT_TYPE
    );
    assert_eq!(
        serialized["parentEventType"],
        constants::tracking_config_update::PARENT_EVENT_TYPE
    );
    assert_eq!(
        serialized["childEventType"],
        constants::tracking_config_update::CHILD_EVENT_TYPE
    );
    assert_eq!(
        serialized["responseState"],
        constants::tracking_config_update::RESPONSE_STATE_APPLIED
    );
    assert_eq!(
        serialized["effectiveTrackingState"],
        constants::tracking_config_update::EFFECTIVE_STATE_ENABLED
    );
    assert_eq!(serialized["localServiceStateRevision"], 7);
    assert_eq!(serialized["durableSettingsPersistenceState"], "persisted");
}

fn command_envelope(command_id: &str) -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: command_id.to_owned(),
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

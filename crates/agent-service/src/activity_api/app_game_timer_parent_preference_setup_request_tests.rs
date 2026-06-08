use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentCommandName, AgentEventName, AgentMessageTarget,
    AgentPeer, AgentPeerRole, AgentRoute, AppGameTimerParentPreferenceSetupRequest,
    AppGameTimerParentPreferenceSetupRequestResult, LogFieldValue, LogFields,
    AGENT_PROTOCOL_SCHEMA_VERSION,
};

use crate::{lan_pairing::LanPairingRuntime, websocket::handle_command_text_for_test};

#[tokio::test]
async fn app_game_timer_parent_preference_setup_request_command_returns_accepted_boundary_result() {
    let body =
        serde_json::to_string(&command_envelope()).expect(constants::error::AGENT_EVENT_SERIALIZES);
    let event = handle_command_text_for_test(&body, LanPairingRuntime::empty(), None).await;
    let result = request_payload(
        &event.payload[constants::field::APP_GAME_TIMER_PARENT_PREFERENCE_SETUP_REQUEST],
    );

    assert_eq!(
        event.event,
        AgentEventName::AgentActivityAppGameTimerParentPreferenceSetupRequested
    );
    assert_eq!(
        result.schema_version,
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_REQUEST_SCHEMA_VERSION
    );
    assert_eq!(
        result.request_status,
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_REQUEST_ACCEPTED
    );
    assert!(result.command_boundary_claimed);
    assert!(!result.parent_preference_mutation_claimed);
    assert!(!result.notification_rule_mutation_claimed);
    assert!(!result.provider_delivery_claimed);
    assert!(!result.durable_outbox_claimed);
    assert!(!result.adapter_dispatch_claimed);
    assert!(!result.platform_enforcement_claimed);
}

fn command_envelope() -> AgentCommandEnvelope {
    let request = AppGameTimerParentPreferenceSetupRequest {
        request_id: constants::event_id::ACTIVITY_APP_GAME_TIMER_PARENT_PREFERENCE_SETUP_REQUESTED
            .to_string(),
        requested_at: constants::activity_store::TEST_TRACKING_RETENTION_DELETE_OBSERVED_AT
            .to_string(),
        parent_surface_intent_reference_id:
            constants::value::APP_GAME_CHILD_UX_PARENT_SURFACE_INTENT_PREFIX.to_string(),
        parent_preference_setup_reference_id:
            constants::value::APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_PREFIX.to_string(),
        request_reference_ids: vec![
            constants::value::APP_GAME_CHILD_UX_PARENT_SURFACE_INTENT_PREFIX.to_string(),
            constants::value::APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_PREFIX.to_string(),
        ],
    };
    let mut payload = LogFields::new();
    payload.insert(
        constants::field::APP_GAME_TIMER_PARENT_PREFERENCE_SETUP_REQUEST.to_string(),
        LogFieldValue::String(
            serde_json::to_string(&request).expect(constants::error::AGENT_EVENT_SERIALIZES),
        ),
    );
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::event_id::ACTIVITY_APP_GAME_TIMER_PARENT_PREFERENCE_SETUP_REQUESTED
            .to_string(),
        sent_at: constants::activity_store::TEST_TRACKING_RETENTION_DELETE_OBSERVED_AT.to_string(),
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
        command: AgentCommandName::AgentActivityAppGameTimerParentPreferenceSetupRequest,
        payload,
    }
}

fn request_payload(value: &LogFieldValue) -> AppGameTimerParentPreferenceSetupRequestResult {
    match value {
        LogFieldValue::String(text) => {
            serde_json::from_str(text).expect(constants::error::AGENT_EVENT_SERIALIZES)
        }
        _ => std::panic::panic_any(constants::error::AGENT_EVENT_SERIALIZES),
    }
}

use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentCommandName, AgentEventName, AgentMessageTarget,
    AgentPeer, AgentPeerRole, AgentRoute, LogFieldValue, LogFields, SocialAuditExplanationSnapshot,
    AGENT_PROTOCOL_SCHEMA_VERSION, SOCIAL_AUDIT_EXPLANATION_SUBJECT_FEED_VIDEO_GATE,
};

use crate::{lan_pairing::LanPairingRuntime, websocket::handle_command_text_for_test};

#[tokio::test]
async fn social_audit_explanation_command_reports_service_backed_snapshot_rows() {
    let body =
        serde_json::to_string(&command_envelope()).expect(constants::error::AGENT_EVENT_SERIALIZES);
    let event = handle_command_text_for_test(&body, LanPairingRuntime::empty(), None).await;
    let read_model = social_audit_explanation_payload(
        &event.payload[constants::field::BROWSER_SOCIAL_AUDIT_EXPLANATION_READ_MODEL],
    );

    assert_eq!(
        event.event,
        AgentEventName::AgentBrowserSocialAuditExplanationReadModelReported
    );
    assert_eq!(read_model.entries.len(), 6);
    assert_eq!(
        read_model.entries[1].subject_kind,
        SOCIAL_AUDIT_EXPLANATION_SUBJECT_FEED_VIDEO_GATE
    );
    assert!(!read_model.entries[1].runtime_audit_store_claimed);
    assert!(!read_model.entries[1].final_policy_decision_claimed);
    assert!(!read_model.entries[1].enforcement_claimed);
}

fn command_envelope() -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::event_id::BROWSER_SOCIAL_AUDIT_EXPLANATION_READ_MODEL_REPORTED
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
        command: AgentCommandName::AgentBrowserSocialAuditExplanationReadModelGet,
        payload: LogFields::new(),
    }
}

fn social_audit_explanation_payload(value: &LogFieldValue) -> SocialAuditExplanationSnapshot {
    match value {
        LogFieldValue::String(text) => {
            serde_json::from_str(text).expect(constants::error::AGENT_EVENT_SERIALIZES)
        }
        _ => std::panic::panic_any(constants::error::AGENT_EVENT_SERIALIZES),
    }
}

use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventName, AgentMessageTarget, AgentPeer,
    AgentPeerRole, AgentRoute,
};
use ocentra_parent_agent_protocol::SocialSourceCustodyMutationSnapshot;
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;

use ocentra_parent_agent_service::test_support::handle_local_command_text_for_test;

#[tokio::test]
async fn social_source_custody_mutation_command_returns_applied_service_snapshot() {
    let body = serde_json::to_string(&command_envelope())
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    let event =
        handle_local_command_text_for_test(crate::test_text::TestText::from_display(body)).await;
    let mutation = mutation_payload(&crate::test_invariants::log_field(
        &event.payload,
        constants::field::BROWSER_SOCIAL_SOURCE_CUSTODY_MUTATION,
        constants::error::AGENT_EVENT_SERIALIZES,
    ));

    assert_eq!(
        event.event,
        AgentEventName::AgentBrowserSocialSourceCustodyMutationApplied
    );
    assert!(mutation.service_mutation_executed);
    assert!(mutation.runtime_custody_mutation_applied);
    assert!(!mutation.settings.runtime_custody_mutation_claimed);
    assert!(!mutation.raw_content_custody_claimed);
    assert!(!mutation.connector_api_called);
    assert!(!mutation.final_policy_decision_claimed);
    assert!(!mutation.enforcement_claimed);
    assert!(!mutation.product_claim_ready);
}

fn command_envelope() -> AgentCommandEnvelope {
    let mut payload = LogFields::new();
    payload.insert(
        constants::field::REQUESTED_AT.to_string(),
        LogFieldValue::String(
            constants::activity_store::TEST_TRACKING_RETENTION_DELETE_OBSERVED_AT.to_string(),
        ),
    );
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::event_id::BROWSER_SOCIAL_SOURCE_CUSTODY_MUTATION_APPLIED.to_string(),
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
        command: AgentCommandName::AgentBrowserSocialSourceCustodyMutationApply,
        payload,
    }
}

fn mutation_payload(value: &LogFieldValue) -> SocialSourceCustodyMutationSnapshot {
    match value {
        LogFieldValue::String(text) => {
            serde_json::from_str(text).expect_value(constants::error::AGENT_EVENT_SERIALIZES)
        }
        _ => std::process::abort(),
    }
}

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentMessageTarget, AgentPeer, AgentPeerRole,
    AgentRoute,
};
use ocentra_parent_agent_protocol::SocialSourceCustodyMutationSnapshot;
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::SOCIAL_SOURCE_CUSTODY_MUTATION_STATE_APPLIED;

use super::social_source_custody_mutation_payload::{
    social_source_custody_mutation_from_command, social_source_custody_mutation_payload,
};
use crate::log_payload::payload_json;

#[test]
fn social_source_custody_mutation_payload_reports_applied_ref_only_snapshot() {
    let mutation = social_source_custody_mutation_from_command(&command_envelope());
    let payload = social_source_custody_mutation_payload(&mutation);
    let decoded: SocialSourceCustodyMutationSnapshot = payload_json(
        &payload,
        constants::field::BROWSER_SOCIAL_SOURCE_CUSTODY_MUTATION,
    );

    assert_eq!(
        decoded.mutation_state,
        SOCIAL_SOURCE_CUSTODY_MUTATION_STATE_APPLIED
    );
    assert!(decoded.service_mutation_executed);
    assert!(decoded.runtime_custody_mutation_applied);
    assert!(!decoded.settings.runtime_custody_mutation_claimed);
    assert!(!decoded.connector_api_called);
    assert!(!decoded.final_policy_decision_claimed);
    assert!(!decoded.enforcement_claimed);
    assert!(!decoded.product_claim_ready);
}

fn command_envelope() -> AgentCommandEnvelope {
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
        payload: LogFields::new(),
    }
}

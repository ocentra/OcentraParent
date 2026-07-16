use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::transport::{AgentCommandEnvelope, AgentCommandName};

use crate::lan_pairing_test_commands::{
    command_for_target, local_network_target, proof_payload_for_pairing,
};

pub(crate) fn route_revoke_command(payload: LogFields) -> AgentCommandEnvelope {
    command_for_target(
        AgentCommandName::AgentLanPairingRouteRevoke,
        local_network_target(constants::lan_pairing::CHILD_DEVICE_ID),
        payload,
    )
}

pub(crate) fn second_proof_payload() -> LogFields {
    proof_payload_for_pairing(
        constants::lan_pairing::SECOND_PAIRING_ID,
        constants::lan_pairing::SECOND_CHALLENGE_ID,
        constants::lan_pairing::SECOND_CHILD_DEVICE_ID,
        constants::lan_pairing::ROUTE_ID_SECOND_LOCAL_NETWORK,
        constants::lan_pairing::SECOND_PROOF_DIGEST,
    )
}

use ocentra_parent_agent_protocol::transport::{AgentCommandEnvelope, AgentCommandName};

use crate::lan_pairing::{
    command_entrypoints::{
        lan_pairing_route_revoke, lan_pairing_route_select, lan_pairing_status_get,
        submit_pairing_proof,
    },
    controller_lease::{
        controller_lease_release, controller_lease_renew, controller_lease_takeover,
    },
    lan_ai_job::lan_ai_provider_status_get,
};
use crate::lan_runtime_stream_api::build_lan_runtime_event_chain_stream_report;

use super::{LanCommandDecision, LanCommandOrigin, LanPairingRuntime};

pub(super) async fn owned_lan_response(
    runtime: LanPairingRuntime,
    origin: LanCommandOrigin,
    command: AgentCommandEnvelope,
) -> Option<LanCommandDecision> {
    let origin_text = origin.0.clone();
    let event = match command.command.clone() {
        AgentCommandName::AgentLanRuntimeEventChainStreamGet => {
            build_lan_runtime_event_chain_stream_report(&runtime, command).await
        }
        AgentCommandName::AgentLanPairingProofSubmit => {
            submit_pairing_proof(runtime, origin.0.clone(), command).await
        }
        AgentCommandName::AgentLanPairingRouteSelect => {
            lan_pairing_route_select(runtime, origin.0.clone(), command)
        }
        AgentCommandName::AgentLanPairingRouteRevoke => {
            lan_pairing_route_revoke(runtime, origin.0.clone(), command)
        }
        AgentCommandName::AgentLanPairingStatusGet => {
            lan_pairing_status_get(runtime, origin.0.clone(), command)
        }
        AgentCommandName::AgentLanPairingControllerLeaseRenew => {
            controller_lease_renew(runtime, origin_text.clone(), command)
        }
        AgentCommandName::AgentLanPairingControllerLeaseRelease => {
            controller_lease_release(runtime, origin_text.clone(), command)
        }
        AgentCommandName::AgentLanPairingControllerLeaseTakeover => {
            controller_lease_takeover(runtime, origin_text.clone(), command)
        }
        AgentCommandName::AgentLanAiProviderStatusGet => {
            lan_ai_provider_status_get(runtime, origin_text, command)
        }
        _ => return None,
    };

    Some(LanCommandDecision::Respond(event))
}

use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingOptionalText, LanPairingRejectionReason,
};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventEnvelope,
};

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
    let event = match command.command.clone() {
        AgentCommandName::AgentLanRuntimeEventChainStreamGet => {
            build_lan_runtime_event_chain_stream_report(&runtime, command).await
        }
        AgentCommandName::AgentLanPairingProofSubmit => {
            submit_pairing_proof_blocking(runtime, origin.0.clone(), command).await
        }
        AgentCommandName::AgentLanPairingRouteSelect => {
            run_blocking_event(runtime, origin.0.clone(), command, lan_pairing_route_select).await
        }
        AgentCommandName::AgentLanPairingRouteRevoke => {
            run_blocking_event(runtime, origin.0.clone(), command, lan_pairing_route_revoke).await
        }
        AgentCommandName::AgentLanPairingStatusGet => {
            run_blocking_event(runtime, origin.0.clone(), command, lan_pairing_status_get).await
        }
        AgentCommandName::AgentLanPairingControllerLeaseRenew => {
            run_blocking_event(runtime, origin.0.clone(), command, controller_lease_renew).await
        }
        AgentCommandName::AgentLanPairingControllerLeaseRelease => {
            run_blocking_event(runtime, origin.0.clone(), command, controller_lease_release).await
        }
        AgentCommandName::AgentLanPairingControllerLeaseTakeover => {
            run_blocking_event(
                runtime,
                origin.0.clone(),
                command,
                controller_lease_takeover,
            )
            .await
        }
        AgentCommandName::AgentLanAiProviderStatusGet => {
            run_blocking_event(runtime, origin.0, command, lan_ai_provider_status_get).await
        }
        _ => return None,
    };

    Some(LanCommandDecision::Respond(event))
}

type BlockingLanEvent =
    fn(LanPairingRuntime, LanPairingOptionalText, AgentCommandEnvelope) -> AgentEventEnvelope;

async fn run_blocking_event(
    runtime: LanPairingRuntime,
    origin: LanPairingOptionalText,
    command: AgentCommandEnvelope,
    handler: BlockingLanEvent,
) -> AgentEventEnvelope {
    let fallback_command = command.clone();
    let fallback_origin = origin.clone();
    tokio::task::spawn_blocking(move || handler(runtime, origin, command))
        .await
        .unwrap_or_else(|_| {
            crate::lan_pairing::rejection_event(
                fallback_command,
                &LanPairingRejectionReason::Malformed,
                None,
                &fallback_origin,
            )
        })
}

async fn submit_pairing_proof_blocking(
    runtime: LanPairingRuntime,
    origin: LanPairingOptionalText,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let fallback_command = command.clone();
    let fallback_origin = origin.clone();
    tokio::task::spawn_blocking(move || {
        tokio::runtime::Handle::current().block_on(submit_pairing_proof(runtime, origin, command))
    })
    .await
    .unwrap_or_else(|_| {
        crate::lan_pairing::rejection_event(
            fallback_command,
            &LanPairingRejectionReason::Malformed,
            None,
            &fallback_origin,
        )
    })
}

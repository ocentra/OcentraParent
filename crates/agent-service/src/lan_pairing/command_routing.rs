use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentRoute,
};

use crate::{
    lan_pairing_browser_runtime::{browser_add_device_request_event, browser_discovery_scan_event},
    lan_pairing_payload::parse_intent,
};

use super::{
    controller_lease::{
        controller_lease_release, controller_lease_renew, controller_lease_takeover,
    },
    lan_ai_job::{lan_ai_job_submit, lan_ai_provider_status_get},
    lan_pairing_route_revoke, lan_pairing_route_select, lan_pairing_status_get, rejection_event,
    signed_child_agent_observed, submit_pairing_proof, validate_control_intent, LanCommandDecision,
    LanPairingRuntime,
};

use ocentra_parent_agent_protocol::lan_pairing::LanPairingOptionalText;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LanCommandOrigin(pub(crate) LanPairingOptionalText);

pub(crate) async fn route_lan_command<O>(
    runtime: LanPairingRuntime,
    origin: O,
    command: AgentCommandEnvelope,
) -> LanCommandDecision
where
    O: Into<LanCommandOrigin>,
{
    let origin = origin.into();
    route_lan_command_inner(runtime, origin, command).await
}

async fn route_lan_command_inner(
    runtime: LanPairingRuntime,
    origin: LanCommandOrigin,
    command: AgentCommandEnvelope,
) -> LanCommandDecision {
    if should_continue_without_lan_handling(&command) {
        return LanCommandDecision::Continue {
            command,
            audit_fields: None,
        };
    }

    if let Some(decision) = direct_pairing_response(&runtime, Some(&origin), &command) {
        return decision;
    }

    if let Some(decision) =
        owned_lan_response(runtime.clone(), origin.clone(), command.clone()).await
    {
        return decision;
    }

    validate_control_command(runtime, Some(&origin), command)
}

fn should_continue_without_lan_handling(command: &AgentCommandEnvelope) -> bool {
    command.target.route != AgentRoute::LocalNetwork
        || command.command == AgentCommandName::AgentLanRuntimeEventChainStreamGet
}

fn direct_pairing_response(
    runtime: &LanPairingRuntime,
    origin: Option<&LanCommandOrigin>,
    command: &AgentCommandEnvelope,
) -> Option<LanCommandDecision> {
    let origin_text = origin.and_then(|origin| origin.0 .0.as_deref());
    let origin_wrapper = origin
        .cloned()
        .map(|origin| origin.0)
        .unwrap_or(LanPairingOptionalText(None));
    match command.command.clone() {
        AgentCommandName::AgentLanPairingBrowserDiscoveryScan => Some(LanCommandDecision::Respond(
            browser_discovery_scan_event(runtime, command.clone()),
        )),
        AgentCommandName::AgentLanPairingAddDeviceRequest => Some(LanCommandDecision::Respond(
            browser_add_device_request_event(runtime, origin_text, command.clone()),
        )),
        AgentCommandName::AgentLanPairingSignedChildAgentObserve => {
            Some(LanCommandDecision::Respond(signed_child_agent_observed(
                runtime,
                &origin_wrapper,
                command.clone(),
            )))
        }
        AgentCommandName::AgentLanAiJobSubmit => Some(LanCommandDecision::Respond(
            lan_ai_job_submit(runtime, origin_wrapper.clone(), command.clone()),
        )),
        _ => None,
    }
}

async fn owned_lan_response(
    runtime: LanPairingRuntime,
    origin: LanCommandOrigin,
    command: AgentCommandEnvelope,
) -> Option<LanCommandDecision> {
    let observed_origin = origin.0 .0.as_deref();
    let origin_text = origin.0.clone();
    let event = match command.command.clone() {
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
    let _ = observed_origin;
    Some(LanCommandDecision::Respond(event))
}

fn validate_control_command(
    runtime: LanPairingRuntime,
    origin: Option<&LanCommandOrigin>,
    command: AgentCommandEnvelope,
) -> LanCommandDecision {
    let origin = origin
        .cloned()
        .map(|origin| origin.0)
        .unwrap_or(LanPairingOptionalText(None));
    match parse_intent(&command.payload) {
        Ok(intent) => validate_control_intent(runtime, &origin, command, intent),
        Err(reason) => {
            LanCommandDecision::Respond(rejection_event(command, &reason, None, &origin))
        }
    }
}

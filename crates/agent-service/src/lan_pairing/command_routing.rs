use ocentra_parent_agent_protocol::transport::{AgentCommandEnvelope, AgentRoute};

use crate::lan_pairing_payload::parse_intent;

#[path = "command_routing/direct.rs"]
mod direct;
#[path = "command_routing/owned.rs"]
mod owned;

use super::{
    command_entrypoints::validate_control_intent, rejection_event, LanCommandDecision,
    LanPairingRuntime,
};

use ocentra_parent_agent_protocol::lan_pairing::LanPairingOptionalText;

use self::direct::direct_pairing_response;
use self::owned::owned_lan_response;

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

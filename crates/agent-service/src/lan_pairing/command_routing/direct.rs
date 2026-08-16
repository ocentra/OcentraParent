use ocentra_parent_agent_protocol::transport::{AgentCommandEnvelope, AgentCommandName};

use crate::lan_pairing_browser_runtime::{
    browser_add_device_request_event, browser_discovery_scan_event,
};

use super::{LanCommandDecision, LanCommandOrigin, LanPairingOptionalText, LanPairingRuntime};
use crate::lan_pairing::command_entrypoints::signed_child_agent_observed;
use crate::lan_pairing::lan_ai_job::lan_ai_job_submit;

pub(super) fn direct_pairing_response(
    runtime: &LanPairingRuntime,
    origin: Option<&LanCommandOrigin>,
    command: &AgentCommandEnvelope,
) -> Option<LanCommandDecision> {
    match command.command.clone() {
        AgentCommandName::AgentLanPairingBrowserDiscoveryScan => Some(LanCommandDecision::Respond(
            browser_discovery_scan_event(runtime, command.clone()),
        )),
        AgentCommandName::AgentLanPairingAddDeviceRequest => {
            let origin_wrapper = origin_wrapper(origin);
            Some(LanCommandDecision::Respond(
                browser_add_device_request_event(runtime, origin_wrapper, command.clone()),
            ))
        }
        AgentCommandName::AgentLanPairingSignedChildAgentObserve => {
            let origin_wrapper = origin_wrapper(origin);
            Some(LanCommandDecision::Respond(signed_child_agent_observed(
                runtime,
                &origin_wrapper,
                command.clone(),
            )))
        }
        AgentCommandName::AgentLanAiJobSubmit => {
            let origin_wrapper = origin_wrapper(origin);
            Some(LanCommandDecision::Respond(lan_ai_job_submit(
                runtime,
                origin_wrapper,
                command.clone(),
            )))
        }
        _ => None,
    }
}

fn origin_wrapper(origin: Option<&LanCommandOrigin>) -> LanPairingOptionalText {
    origin
        .cloned()
        .map(|origin| origin.0)
        .unwrap_or(LanPairingOptionalText(None))
}

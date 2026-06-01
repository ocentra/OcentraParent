use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentEventEnvelope, AgentEventName, LogLevel,
};

use crate::{
    lan_pairing::LanPairingRuntime,
    lan_pairing_status::{pairing_challenge_status_event, pairing_status_event},
};

pub(crate) fn browser_discovery_scan_event(
    runtime: &LanPairingRuntime,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    retag_lan_pairing_event(
        pairing_status_event(runtime, command),
        constants::lan_pairing::EVENT_BROWSER_DISCOVERY_REPORTED,
        AgentEventName::AgentLanPairingBrowserDiscoveryReported,
        LogLevel::Info,
    )
}

pub(crate) fn browser_add_device_request_event(
    runtime: &LanPairingRuntime,
    origin: Option<&str>,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let event = pairing_challenge_status_event(runtime, origin, command);
    if event.event == AgentEventName::AgentLanPairingStatusReported {
        retag_lan_pairing_event(
            event,
            constants::lan_pairing::EVENT_ADD_DEVICE_REPORTED,
            AgentEventName::AgentLanPairingAddDeviceReported,
            LogLevel::Info,
        )
    } else {
        event
    }
}

fn retag_lan_pairing_event(
    mut event: AgentEventEnvelope,
    event_id: &str,
    event_name: AgentEventName,
    severity: LogLevel,
) -> AgentEventEnvelope {
    event.event_id = event_id.to_string();
    event.event = event_name;
    event.severity = severity;
    event
}

use std::thread;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingOptionalText;
use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentCommandName;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;
use ocentra_parent_agent_protocol::transport::AgentRoute;

use crate::{
    lan_pairing::{rejection_event, LanPairingRuntime},
    lan_pairing_browser_add_device_state::refresh_browser_discovery_scan_history,
    lan_pairing_payload::parse_household_device_decision,
    lan_pairing_status::{pairing_challenge_status_event, pairing_status_event},
    time::timestamp_now,
};

const LAN_BROWSER_DISCOVERY_SCAN_THREAD_NAME: &str = "lan-browser-discovery-scan";

struct LanPairingEventIdRef(&'static str);

pub(crate) fn browser_discovery_scan_event(
    runtime: &LanPairingRuntime,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let scan_command = command.clone();
    let event = retag_lan_pairing_event(
        pairing_status_event(runtime, browser_discovery_scan_ack_command(command)),
        &LanPairingEventIdRef(constants::lan_pairing::EVENT_BROWSER_DISCOVERY_REPORTED),
        AgentEventName::AgentLanPairingBrowserDiscoveryReported,
        LogLevel::Info,
    );
    start_background_browser_discovery_scan(runtime, scan_command);
    event
}

fn start_background_browser_discovery_scan(
    runtime: &LanPairingRuntime,
    command: AgentCommandEnvelope,
) {
    let runtime = runtime.clone();
    let _ = thread::Builder::new()
        .name(LAN_BROWSER_DISCOVERY_SCAN_THREAD_NAME.to_string())
        .spawn(move || refresh_browser_discovery_scan_history(&runtime, &command));
}

fn browser_discovery_scan_ack_command(mut command: AgentCommandEnvelope) -> AgentCommandEnvelope {
    command.command = AgentCommandName::AgentLanPairingStatusGet;
    command.target.route = AgentRoute::Localhost;
    command
}

pub(crate) fn browser_add_device_request_event(
    runtime: &LanPairingRuntime,
    origin: LanPairingOptionalText,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    match parse_household_device_decision(&command.payload, timestamp_now::<String>()) {
        Some(Ok(decision)) => {
            runtime
                .registry
                .lock()
                .map(|mut registry| {
                    registry.apply_household_device_decision(decision);
                    runtime.persist_registry(&registry);
                })
                .ok();
            return retag_lan_pairing_event(
                pairing_status_event(runtime, command),
                &LanPairingEventIdRef(constants::lan_pairing::EVENT_ADD_DEVICE_REPORTED),
                AgentEventName::AgentLanPairingAddDeviceReported,
                LogLevel::Info,
            );
        }
        Some(Err(reason)) => {
            return rejection_event(command, &reason, None, &origin);
        }
        None => {}
    }

    let event = pairing_challenge_status_event(runtime, origin, command);
    if event.event == AgentEventName::AgentLanPairingStatusReported {
        retag_lan_pairing_event(
            event,
            &LanPairingEventIdRef(constants::lan_pairing::EVENT_ADD_DEVICE_REPORTED),
            AgentEventName::AgentLanPairingAddDeviceReported,
            LogLevel::Info,
        )
    } else {
        event
    }
}

fn retag_lan_pairing_event(
    mut event: AgentEventEnvelope,
    event_id: &LanPairingEventIdRef,
    event_name: AgentEventName,
    severity: LogLevel,
) -> AgentEventEnvelope {
    event.event_id = event_id.0.to_string();
    event.event = event_name;
    event.severity = severity;
    event
}

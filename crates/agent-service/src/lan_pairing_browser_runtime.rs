use std::{
    sync::{atomic::AtomicBool, Arc},
    thread,
};

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingOptionalText;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingText;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanHouseholdDeviceDecision;
use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentCommandName;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;
use ocentra_parent_agent_protocol::transport::AgentRoute;

use crate::{
    lan_pairing::{
        rejection_event,
        runtime_validation::household_decision::validate_household_device_decision,
        LanBrowserDiscoveryScanWorker, LanPairingRuntime,
    },
    lan_pairing_browser_add_device_state::physical_lan_scan::cancellation::refresh_network_device_scan_history_with_cancellation,
    lan_pairing_payload::{parse_household_device_decision, parse_intent},
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
    if let Err(reason) = start_background_browser_discovery_scan(runtime, scan_command) {
        return rejection_event(command, &reason, None, &LanPairingOptionalText(None));
    }
    retag_lan_pairing_event(
        pairing_status_event(runtime, browser_discovery_scan_ack_command(command)),
        &LanPairingEventIdRef(constants::lan_pairing::EVENT_BROWSER_DISCOVERY_REPORTED),
        AgentEventName::AgentLanPairingBrowserDiscoveryReported,
        LogLevel::Info,
    )
}

fn start_background_browser_discovery_scan(
    runtime: &LanPairingRuntime,
    command: AgentCommandEnvelope,
) -> Result<(), LanPairingRejectionReason> {
    let cancellation = Arc::new(AtomicBool::new(false));
    let worker_slot = Arc::clone(&runtime.browser_discovery_scan_worker);
    let mut worker_slot_guard = worker_slot
        .lock()
        .map_err(|_error| LanPairingRejectionReason::SignedChildAgentContextUnavailable)?;
    worker_slot_guard
        .take()
        .map(LanBrowserDiscoveryScanWorker::cancel_and_join)
        .transpose()
        .map_err(|worker| {
            worker_slot_guard.replace(worker);
            LanPairingRejectionReason::SignedChildAgentContextUnavailable
        })?;
    let worker_cancellation = Arc::clone(&cancellation);
    let worker_runtime = runtime.clone_for_background_scan();
    let join = thread::Builder::new()
        .name(LAN_BROWSER_DISCOVERY_SCAN_THREAD_NAME.to_string())
        .spawn(move || {
            let _ = refresh_network_device_scan_history_with_cancellation(
                &worker_runtime,
                &command,
                &worker_cancellation,
            );
        })
        .map_err(|_error| LanPairingRejectionReason::SignedChildAgentContextUnavailable)?;
    worker_slot_guard.replace(LanBrowserDiscoveryScanWorker {
        cancellation,
        join: Some(join),
    });
    Ok(())
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
            let intent = match parse_intent(&command.payload) {
                Ok(intent) => intent,
                Err(reason) => return rejection_event(command, &reason, None, &origin),
            };
            if let Err(reason) =
                validate_household_device_decision(runtime, &command, &origin, &intent, &decision)
            {
                return rejection_event(command, &reason, Some(&intent), &origin);
            }
            if let Err(reason) =
                apply_household_device_decision(runtime, &origin, &intent, decision)
            {
                return rejection_event(command, &reason, Some(&intent), &origin);
            }
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

fn apply_household_device_decision(
    runtime: &LanPairingRuntime,
    origin: &LanPairingOptionalText,
    intent: &ocentra_parent_agent_protocol::lan_pairing::LanParentIntentEnvelope,
    decision: LanHouseholdDeviceDecision,
) -> Result<(), LanPairingRejectionReason> {
    let mut registry = runtime
        .registry
        .lock()
        .map_err(|_error| LanPairingRejectionReason::SignedChildAgentContextUnavailable)?;
    let observed_at = LanPairingText(timestamp_now());
    runtime.apply_household_device_decision(&mut registry, intent, origin, &observed_at, decision)
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

use std::time::Duration;

use ocentra_lan_core::network_inventory::passive_discovery::{
    collect_raw_socket_protocol_passive_updates, collect_udp_multicast_passive_packets,
    ingest_allowed_snmp_response_packet, LanPassiveDiscoveryListenerState,
    LanPassiveDiscoveryPacketIngestOutcome, LanPassiveDiscoveryRawSocketProtocol,
    LanPassiveDiscoverySource, LanPassiveDiscoveryTriggerReason,
};
use ocentra_lan_core::network_inventory::{
    passive_runtime_local_network_identity, LanPassiveRuntimeLocalNetworkIdentity,
};
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;

use crate::{lan_pairing::LanPairingRuntime, time::timestamp_now};

const PASSIVE_DISCOVERY_INTERVAL: Duration = Duration::from_secs(180);
const PASSIVE_DISCOVERY_READ_TIMEOUT: Duration = Duration::from_millis(50);
const PASSIVE_DISCOVERY_MAX_DATAGRAMS_PER_SOURCE: usize = 8;

const PASSIVE_DISCOVERY_UDP_SOURCES: [LanPassiveDiscoverySource; 6] = [
    LanPassiveDiscoverySource::Dhcp,
    LanPassiveDiscoverySource::Mdns,
    LanPassiveDiscoverySource::Ssdp,
    LanPassiveDiscoverySource::WsDiscovery,
    LanPassiveDiscoverySource::Llmnr,
    LanPassiveDiscoverySource::Netbios,
];

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct LanPassiveDiscoveryRuntimeObservedState {
    heartbeat_loss_recorded: bool,
    last_local_network_identity: Option<LanPassiveRuntimeLocalNetworkIdentity>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LanPassiveDiscoveryLocalNetworkChangeTrigger {
    pub(crate) reason: LanPassiveDiscoveryTriggerReason,
    pub(crate) summary: String,
}

pub(crate) fn spawn_lan_passive_discovery_runtime(runtime: LanPairingRuntime) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(PASSIVE_DISCOVERY_INTERVAL);
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        let mut observed_state = LanPassiveDiscoveryRuntimeObservedState::default();
        runtime.record_passive_rescan_trigger(
            LanPassiveDiscoveryTriggerReason::AppResumed,
            "passive discovery runtime started",
        );
        if !runtime.collect_passive_discovery_runtime_slice(&mut observed_state) {
            return;
        }
        loop {
            interval.tick().await;
            if !runtime.collect_passive_discovery_runtime_slice(&mut observed_state) {
                break;
            }
        }
    });
}

impl LanPairingRuntime {
    pub(crate) fn record_passive_rescan_trigger(
        &self,
        trigger_reason: LanPassiveDiscoveryTriggerReason,
        summary: &str,
    ) {
        let observed_at = timestamp_now();
        if let Ok(mut state) = self.passive_discovery_listener_state.lock() {
            let _ = state.record_rescan_trigger(trigger_reason, &observed_at, summary);
        }
    }

    pub(crate) fn collect_passive_discovery_runtime_slice(
        &self,
        observed_state: &mut LanPassiveDiscoveryRuntimeObservedState,
    ) -> bool {
        if let Ok(mut state) = self.passive_discovery_listener_state.lock() {
            if !state.is_running() {
                return false;
            }
            self.record_local_network_change_triggers_if_needed(
                &mut state,
                observed_state,
                &timestamp_now(),
                &passive_runtime_local_network_identity(),
            );
            collect_passive_discovery_inputs(&mut state);
            self.record_heartbeat_loss_trigger_if_needed(
                &mut state,
                observed_state,
                &timestamp_now(),
            );
            return state.is_running();
        }
        false
    }

    pub(crate) fn record_local_network_change_triggers_if_needed(
        &self,
        state: &mut LanPassiveDiscoveryListenerState,
        observed_state: &mut LanPassiveDiscoveryRuntimeObservedState,
        observed_at: &str,
        current_identity: &LanPassiveRuntimeLocalNetworkIdentity,
    ) {
        for trigger in local_network_change_triggers(
            observed_state.last_local_network_identity.as_ref(),
            current_identity,
        ) {
            let _ = state.record_rescan_trigger(trigger.reason, observed_at, trigger.summary);
        }
        observed_state.last_local_network_identity = Some(current_identity.clone());
    }

    pub(crate) fn record_heartbeat_loss_trigger_if_needed(
        &self,
        state: &mut LanPassiveDiscoveryListenerState,
        observed_state: &mut LanPassiveDiscoveryRuntimeObservedState,
        observed_at: &str,
    ) {
        let lost = matches!(
            self.lan_ai_provider_heartbeat_reachability(),
            Some(LanPairingDeviceReachability::Offline | LanPairingDeviceReachability::Stale)
        );
        if lost {
            if !observed_state.heartbeat_loss_recorded {
                let _ = state.record_rescan_trigger(
                    LanPassiveDiscoveryTriggerReason::HeartbeatLost,
                    observed_at,
                    "provider heartbeat lost",
                );
                observed_state.heartbeat_loss_recorded = true;
            }
            return;
        }
        observed_state.heartbeat_loss_recorded = false;
    }

    pub(crate) fn record_allowed_snmp_probe_response_packet(&self, payload: &[u8]) -> bool {
        if let Ok(mut state) = self.passive_discovery_listener_state.lock() {
            if !state.is_running() {
                return false;
            }
            return matches!(
                ingest_allowed_snmp_response_packet(&mut state, payload),
                LanPassiveDiscoveryPacketIngestOutcome::Recorded
                    | LanPassiveDiscoveryPacketIngestOutcome::Deduplicated
            );
        }
        false
    }
}

pub(crate) fn local_network_change_triggers(
    previous_identity: Option<&LanPassiveRuntimeLocalNetworkIdentity>,
    current_identity: &LanPassiveRuntimeLocalNetworkIdentity,
) -> Vec<LanPassiveDiscoveryLocalNetworkChangeTrigger> {
    let Some(previous_identity) = previous_identity else {
        return Vec::new();
    };

    let mut triggers = Vec::new();

    if previous_identity.network_interface != current_identity.network_interface {
        if let Some(previous_interface) = previous_identity.network_interface.as_deref() {
            triggers.push(LanPassiveDiscoveryLocalNetworkChangeTrigger {
                reason: LanPassiveDiscoveryTriggerReason::InterfaceDown,
                summary: format!("network interface down: {previous_interface}"),
            });
        }
        if let Some(current_interface) = current_identity.network_interface.as_deref() {
            triggers.push(LanPassiveDiscoveryLocalNetworkChangeTrigger {
                reason: LanPassiveDiscoveryTriggerReason::InterfaceUp,
                summary: format!("network interface up: {current_interface}"),
            });
        }
    }

    if previous_identity.network_interface == current_identity.network_interface
        && previous_identity.wifi_ssid != current_identity.wifi_ssid
        && (previous_identity.wifi_ssid.is_some() || current_identity.wifi_ssid.is_some())
    {
        triggers.push(LanPassiveDiscoveryLocalNetworkChangeTrigger {
            reason: LanPassiveDiscoveryTriggerReason::WifiSsidChanged,
            summary: format!(
                "wifi ssid changed: {} -> {}",
                optional_identity_value(previous_identity.wifi_ssid.as_deref()),
                optional_identity_value(current_identity.wifi_ssid.as_deref())
            ),
        });
    }

    if previous_identity.ip_address != current_identity.ip_address {
        triggers.push(LanPassiveDiscoveryLocalNetworkChangeTrigger {
            reason: LanPassiveDiscoveryTriggerReason::IpAddressChanged,
            summary: format!(
                "ip address changed: {} -> {}",
                optional_identity_value(previous_identity.ip_address.as_deref()),
                optional_identity_value(current_identity.ip_address.as_deref())
            ),
        });
    }

    if previous_identity.default_gateway != current_identity.default_gateway {
        triggers.push(LanPassiveDiscoveryLocalNetworkChangeTrigger {
            reason: LanPassiveDiscoveryTriggerReason::DefaultGatewayChanged,
            summary: format!(
                "default gateway changed: {} -> {}",
                optional_identity_value(previous_identity.default_gateway.as_deref()),
                optional_identity_value(current_identity.default_gateway.as_deref())
            ),
        });
    }

    triggers
}

fn collect_passive_discovery_inputs(state: &mut LanPassiveDiscoveryListenerState) {
    let _ = collect_raw_socket_protocol_passive_updates(
        state,
        LanPassiveDiscoveryRawSocketProtocol::Arp,
        PASSIVE_DISCOVERY_READ_TIMEOUT,
    );
    for source in passive_discovery_udp_sources() {
        let _ = collect_udp_multicast_passive_packets(
            state,
            *source,
            PASSIVE_DISCOVERY_MAX_DATAGRAMS_PER_SOURCE,
            PASSIVE_DISCOVERY_READ_TIMEOUT,
        );
    }
}

pub(crate) fn passive_discovery_udp_sources() -> &'static [LanPassiveDiscoverySource] {
    &PASSIVE_DISCOVERY_UDP_SOURCES
}

fn optional_identity_value(value: Option<&str>) -> &str {
    value.unwrap_or("none")
}

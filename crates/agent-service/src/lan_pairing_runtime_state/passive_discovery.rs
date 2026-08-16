#[path = "passive_discovery/change_triggers.rs"]
mod change_triggers;
#[path = "passive_discovery/runtime_slice.rs"]
mod runtime_slice;

use std::time::Duration;

use ocentra_lan_core::network_inventory::passive_discovery::{
    LanPassiveDiscoverySource, LanPassiveDiscoveryTriggerReason,
};
use ocentra_lan_core::network_inventory::LanPassiveRuntimeLocalNetworkIdentity;
use ocentra_parent_agent_protocol::constants::lan_pairing as lan_pairing_constants;

use crate::lan_pairing::LanPairingRuntime;

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
            lan_pairing_constants::PASSIVE_DISCOVERY_RUNTIME_STARTED_SUMMARY,
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

pub(crate) fn local_network_change_triggers(
    previous_identity: Option<&LanPassiveRuntimeLocalNetworkIdentity>,
    current_identity: &LanPassiveRuntimeLocalNetworkIdentity,
) -> Vec<LanPassiveDiscoveryLocalNetworkChangeTrigger> {
    change_triggers::local_network_change_triggers(previous_identity, current_identity)
}

pub(crate) fn passive_discovery_udp_sources() -> &'static [LanPassiveDiscoverySource] {
    &PASSIVE_DISCOVERY_UDP_SOURCES
}

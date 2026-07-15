use ocentra_lan_core::network_inventory::passive_discovery::{
    collect_raw_socket_protocol_passive_updates, collect_udp_multicast_passive_packets,
    ingest_allowed_snmp_response_packet, LanPassiveDiscoveryListenerState,
    LanPassiveDiscoveryPacketIngestOutcome, LanPassiveDiscoveryRawSocketProtocol,
    LanPassiveDiscoveryTriggerReason,
};
use ocentra_lan_core::network_inventory::{
    passive_runtime_local_network_identity, LanPassiveRuntimeLocalNetworkIdentity,
};
use ocentra_parent_agent_protocol::constants::lan_pairing as lan_pairing_constants;
use ocentra_parent_agent_protocol::lan_pairing::{LanPairingDeviceReachability, LanPairingText};

use crate::{lan_pairing::LanPairingRuntime, time::timestamp_now};

use super::{
    local_network_change_triggers, passive_discovery_udp_sources,
    LanPassiveDiscoveryRuntimeObservedState, PASSIVE_DISCOVERY_MAX_DATAGRAMS_PER_SOURCE,
    PASSIVE_DISCOVERY_READ_TIMEOUT,
};

impl LanPairingRuntime {
    pub(crate) fn record_passive_rescan_trigger(
        &self,
        trigger_reason: LanPassiveDiscoveryTriggerReason,
        summary: impl Into<LanPairingText>,
    ) {
        let observed_at: LanPairingText = timestamp_now::<String>().into();
        let summary = summary.into();
        if let Ok(mut state) = self.passive_discovery_listener_state.lock() {
            let _ = state.record_rescan_trigger(trigger_reason, &observed_at.0, &summary.0);
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
            let observed_at: LanPairingText = timestamp_now::<String>().into();
            let current_identity = passive_runtime_local_network_identity();
            self.record_local_network_change_triggers_if_needed(
                &mut state,
                observed_state,
                observed_at.clone(),
                &current_identity,
            );
            collect_passive_discovery_inputs(&mut state);
            self.record_heartbeat_loss_trigger_if_needed(&mut state, observed_state, observed_at);
            return state.is_running();
        }
        false
    }

    pub(crate) fn record_local_network_change_triggers_if_needed(
        &self,
        state: &mut LanPassiveDiscoveryListenerState,
        observed_state: &mut LanPassiveDiscoveryRuntimeObservedState,
        observed_at: impl Into<LanPairingText>,
        current_identity: &LanPassiveRuntimeLocalNetworkIdentity,
    ) {
        let observed_at = observed_at.into();
        for trigger in local_network_change_triggers(
            observed_state.last_local_network_identity.as_ref(),
            current_identity,
        ) {
            let _ = state.record_rescan_trigger(trigger.reason, &observed_at.0, &trigger.summary);
        }
        observed_state.last_local_network_identity = Some(current_identity.clone());
    }

    pub(crate) fn record_heartbeat_loss_trigger_if_needed(
        &self,
        state: &mut LanPassiveDiscoveryListenerState,
        observed_state: &mut LanPassiveDiscoveryRuntimeObservedState,
        observed_at: impl Into<LanPairingText>,
    ) {
        let observed_at = observed_at.into();
        let lost = matches!(
            self.lan_ai_provider_heartbeat_reachability(),
            Some(LanPairingDeviceReachability::Offline | LanPairingDeviceReachability::Stale)
        );
        if lost {
            if !observed_state.heartbeat_loss_recorded {
                let _ = state.record_rescan_trigger(
                    LanPassiveDiscoveryTriggerReason::HeartbeatLost,
                    &observed_at.0,
                    lan_pairing_constants::PASSIVE_DISCOVERY_HEARTBEAT_LOST_SUMMARY,
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

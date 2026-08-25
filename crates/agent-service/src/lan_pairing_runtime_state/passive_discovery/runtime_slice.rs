use std::sync::{Arc, Mutex};

use ocentra_lan_core::network_inventory::passive_discovery::{
    collect_raw_socket_protocol_passive_updates, ingest_allowed_snmp_response_packet,
    LanPassiveDiscoveryEventKind, LanPassiveDiscoveryEventRow, LanPassiveDiscoveryListenerState,
    LanPassiveDiscoveryPacketIngestOutcome, LanPassiveDiscoveryRawSocketProtocol,
    LanPassiveDiscoveryRecordOutcome, LanPassiveDiscoveryTriggerReason,
};
use ocentra_lan_core::network_inventory::{
    passive_runtime_local_network_identity, LanPassiveRuntimeLocalNetworkIdentity,
};
use ocentra_parent_agent_protocol::constants::lan_pairing as lan_pairing_constants;
use ocentra_parent_agent_protocol::lan_pairing::{LanPairingDeviceReachability, LanPairingText};

use crate::{lan_pairing::LanPairingRuntime, time::timestamp_now};

use super::{
    local_network_change_triggers, LanPassiveDiscoveryRefreshSignal,
    LanPassiveDiscoveryRuntimeObservedState, LanPassiveDiscoveryRuntimeSliceOutcome,
    PASSIVE_DISCOVERY_READ_TIMEOUT,
};

impl LanPairingRuntime {
    pub(crate) fn record_passive_rescan_trigger(
        &self,
        trigger_reason: LanPassiveDiscoveryTriggerReason,
        summary: impl Into<LanPairingText>,
    ) -> Option<LanPassiveDiscoveryRefreshSignal> {
        let observed_at: LanPairingText = timestamp_now::<String>().into();
        let summary = summary.into();
        if let Ok(mut state) = self.passive_discovery_listener_state.lock() {
            return (state.record_rescan_trigger(
                trigger_reason.clone(),
                &observed_at.0,
                &summary.0,
            ) == LanPassiveDiscoveryRecordOutcome::Recorded)
                .then(|| LanPassiveDiscoveryRefreshSignal {
                    sequence: 0,
                    source: None,
                    trigger_reason,
                    observed_at: observed_at.0,
                });
        }
        None
    }

    pub(crate) fn collect_passive_discovery_runtime_slice(
        &self,
        observed_state: &mut LanPassiveDiscoveryRuntimeObservedState,
    ) -> bool {
        collect_runtime_slice(
            &self.passive_discovery_listener_state,
            self.lan_ai_provider_heartbeat_reachability(),
            observed_state,
        )
        .running
    }

    pub(crate) fn record_local_network_change_triggers_if_needed(
        &self,
        state: &mut LanPassiveDiscoveryListenerState,
        observed_state: &mut LanPassiveDiscoveryRuntimeObservedState,
        observed_at: impl Into<LanPairingText>,
        current_identity: &LanPassiveRuntimeLocalNetworkIdentity,
    ) {
        let observed_at = observed_at.into();
        let _ = record_local_network_change_triggers(
            state,
            observed_state,
            &observed_at,
            current_identity,
        );
    }

    pub(crate) fn record_heartbeat_loss_trigger_if_needed(
        &self,
        state: &mut LanPassiveDiscoveryListenerState,
        observed_state: &mut LanPassiveDiscoveryRuntimeObservedState,
        observed_at: impl Into<LanPairingText>,
    ) {
        let observed_at = observed_at.into();
        let _ = record_heartbeat_loss_trigger(
            state,
            observed_state,
            &observed_at,
            self.lan_ai_provider_heartbeat_reachability(),
        );
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

pub(super) fn collect_runtime_slice(
    listener_state: &Arc<Mutex<LanPassiveDiscoveryListenerState>>,
    heartbeat_reachability: Option<LanPairingDeviceReachability>,
    observed_state: &mut LanPassiveDiscoveryRuntimeObservedState,
) -> LanPassiveDiscoveryRuntimeSliceOutcome {
    if !listener_is_running(listener_state) {
        return stopped_slice();
    }

    let observed_at: LanPairingText = timestamp_now::<String>().into();
    let current_identity = passive_runtime_local_network_identity();
    let arp_rows = collect_arp_rows(&observed_at);
    let Ok(mut state) = listener_state.lock() else {
        return stopped_slice();
    };
    if !state.is_running() {
        return stopped_slice();
    }

    let (network_changed, mut refresh_signals) = record_local_network_change_triggers(
        &mut state,
        observed_state,
        &observed_at,
        &current_identity,
    );
    refresh_signals.extend(record_arp_rows(&mut state, arp_rows));
    if let Some(signal) = record_heartbeat_loss_trigger(
        &mut state,
        observed_state,
        &observed_at,
        heartbeat_reachability,
    ) {
        refresh_signals.push(signal);
    }
    LanPassiveDiscoveryRuntimeSliceOutcome {
        running: state.is_running(),
        network_changed,
        refresh_signals,
    }
}

fn listener_is_running(listener_state: &Arc<Mutex<LanPassiveDiscoveryListenerState>>) -> bool {
    listener_state
        .lock()
        .map(|state| state.is_running())
        .unwrap_or(false)
}

fn collect_arp_rows(observed_at: &LanPairingText) -> Vec<LanPassiveDiscoveryEventRow> {
    let mut temporary_state = LanPassiveDiscoveryListenerState::running(observed_at.0.clone());
    let _ = collect_raw_socket_protocol_passive_updates(
        &mut temporary_state,
        LanPassiveDiscoveryRawSocketProtocol::Arp,
        PASSIVE_DISCOVERY_READ_TIMEOUT,
    );
    temporary_state.rows()
}

fn record_arp_rows(
    state: &mut LanPassiveDiscoveryListenerState,
    rows: Vec<LanPassiveDiscoveryEventRow>,
) -> Vec<LanPassiveDiscoveryRefreshSignal> {
    let mut refresh_signals = Vec::new();
    for row in rows {
        if row.event_kind != LanPassiveDiscoveryEventKind::PassiveUpdate {
            continue;
        }
        let Some(source) = row.source else {
            continue;
        };
        if state.record_passive_update(
            source,
            row.trigger_reason.clone(),
            &row.observed_at,
            row.device_id.as_ref().map(|value| value.as_str()),
            row.scan_session_id.as_ref().map(|value| value.as_str()),
            row.summary,
        ) == LanPassiveDiscoveryRecordOutcome::Recorded
        {
            refresh_signals.push(LanPassiveDiscoveryRefreshSignal {
                sequence: 0,
                source: Some(source),
                trigger_reason: row.trigger_reason,
                observed_at: row.observed_at,
            });
        }
    }
    refresh_signals
}

fn record_local_network_change_triggers(
    state: &mut LanPassiveDiscoveryListenerState,
    observed_state: &mut LanPassiveDiscoveryRuntimeObservedState,
    observed_at: &LanPairingText,
    current_identity: &LanPassiveRuntimeLocalNetworkIdentity,
) -> (bool, Vec<LanPassiveDiscoveryRefreshSignal>) {
    let triggers = local_network_change_triggers(
        observed_state.last_local_network_identity.as_ref(),
        current_identity,
    );
    let network_changed = !triggers.is_empty();
    let refresh_signals = triggers
        .into_iter()
        .filter_map(|trigger| {
            (state.record_rescan_trigger(trigger.reason.clone(), &observed_at.0, &trigger.summary)
                == LanPassiveDiscoveryRecordOutcome::Recorded)
                .then(|| LanPassiveDiscoveryRefreshSignal {
                    sequence: 0,
                    source: None,
                    trigger_reason: trigger.reason,
                    observed_at: observed_at.0.clone(),
                })
        })
        .collect();
    observed_state.last_local_network_identity = Some(current_identity.clone());
    (network_changed, refresh_signals)
}

fn record_heartbeat_loss_trigger(
    state: &mut LanPassiveDiscoveryListenerState,
    observed_state: &mut LanPassiveDiscoveryRuntimeObservedState,
    observed_at: &LanPairingText,
    heartbeat_reachability: Option<LanPairingDeviceReachability>,
) -> Option<LanPassiveDiscoveryRefreshSignal> {
    let lost = matches!(
        heartbeat_reachability,
        Some(LanPairingDeviceReachability::Offline | LanPairingDeviceReachability::Stale)
    );
    if !lost {
        observed_state.heartbeat_loss_recorded = false;
        return None;
    }
    if observed_state.heartbeat_loss_recorded {
        return None;
    }
    observed_state.heartbeat_loss_recorded = true;
    (state.record_rescan_trigger(
        LanPassiveDiscoveryTriggerReason::HeartbeatLost,
        &observed_at.0,
        lan_pairing_constants::PASSIVE_DISCOVERY_HEARTBEAT_LOST_SUMMARY,
    ) == LanPassiveDiscoveryRecordOutcome::Recorded)
        .then(|| LanPassiveDiscoveryRefreshSignal {
            sequence: 0,
            source: None,
            trigger_reason: LanPassiveDiscoveryTriggerReason::HeartbeatLost,
            observed_at: observed_at.0.clone(),
        })
}

fn stopped_slice() -> LanPassiveDiscoveryRuntimeSliceOutcome {
    LanPassiveDiscoveryRuntimeSliceOutcome {
        running: false,
        network_changed: false,
        refresh_signals: Vec::new(),
    }
}

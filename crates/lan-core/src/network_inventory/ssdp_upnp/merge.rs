use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanServiceIdentityProbeEvidence, LanServiceIdentityProbeEvidenceKind,
};

use super::{LanNetworkInventoryDevice, SsdpDiscoveryRecord};

mod evidence;

pub(super) fn merge_ssdp_network_inventory_device(
    existing: &mut LanNetworkInventoryDevice,
    incoming: &mut LanNetworkInventoryDevice,
) {
    if existing.label.is_empty()
        || existing
            .label
            .starts_with(constants::lan_pairing::NETWORK_NEIGHBOR_LABEL_PREFIX)
    {
        existing.label = incoming.label.clone();
    }
    if existing.platform == constants::lan_pairing::PLATFORM_UNKNOWN {
        existing.platform = incoming.platform.clone();
    }
    if existing.hostname.is_none() {
        existing.hostname = incoming.hostname.take();
    }
    if existing.network_interface.is_none() {
        existing.network_interface = incoming.network_interface.take();
    }
    if existing.reachability != LanPairingDeviceReachability::Online {
        existing.reachability = LanPairingDeviceReachability::Online;
    }
    if existing.agent_status.is_none() {
        existing.agent_status = incoming.agent_status.take();
    }
    merge_service_identity_probe_evidence(
        &mut existing.service_identity_probe_evidence,
        incoming.service_identity_probe_evidence.drain(..),
    );
    for source in incoming.scan_sources.drain(..) {
        if !existing
            .scan_sources
            .iter()
            .any(|existing_source| existing_source == &source)
        {
            existing.scan_sources.push(source);
        }
    }
}

pub(super) fn ssdp_hint_evidence(
    record: &SsdpDiscoveryRecord,
    selected_interface: Option<&str>,
) -> Vec<LanServiceIdentityProbeEvidence> {
    evidence::ssdp_hint_evidence(record, selected_interface)
}

pub(super) fn merge_service_identity_probe_evidence(
    existing: &mut Vec<LanServiceIdentityProbeEvidence>,
    incoming: impl Iterator<Item = LanServiceIdentityProbeEvidence>,
) {
    for record in incoming {
        if let Some(current) = existing.iter_mut().find(|entry| {
            entry.evidence_kind == record.evidence_kind
                && entry.value.eq_ignore_ascii_case(&record.value)
        }) {
            if current.selected_interface.is_none() {
                current.selected_interface = record.selected_interface.clone();
            }
            continue;
        }
        existing.push(record);
    }
}

pub(super) fn push_ssdp_hint(
    records: &mut Vec<LanServiceIdentityProbeEvidence>,
    evidence_kind: LanServiceIdentityProbeEvidenceKind,
    value: &str,
    selected_interface: Option<String>,
) {
    evidence::push_ssdp_hint(records, evidence_kind, value, selected_interface)
}

use std::collections::HashSet;

use ocentra_parent_agent_protocol::lan_pairing::LanPairingTrustState;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDevice, LanCanonicalHouseholdDeviceSource,
    LanCanonicalHouseholdNetworkIdentity, LanDiscoveryEvidenceKind,
};

use super::collections::merge_strings;
use super::confidence::merged_confidence;
use super::merge_evidence::merge_evidence_records;

pub(super) fn merge_network_identity(
    mut existing: LanCanonicalHouseholdNetworkIdentity,
    incoming: LanCanonicalHouseholdNetworkIdentity,
    trust_state: &LanPairingTrustState,
    existing_sources: &[LanCanonicalHouseholdDeviceSource],
    incoming_sources: &[LanCanonicalHouseholdDeviceSource],
) -> LanCanonicalHouseholdNetworkIdentity {
    if existing.hostname.is_none() {
        existing.hostname = incoming.hostname;
    }
    if existing.mac_address.is_none() {
        existing.mac_address = incoming.mac_address;
    }
    if existing.mac_vendor.is_none() {
        existing.mac_vendor = incoming.mac_vendor;
    }
    merge_strings(&mut existing.ip_addresses, incoming.ip_addresses);
    merge_strings(
        &mut existing.network_interfaces,
        incoming.network_interfaces,
    );
    merge_evidence_records(&mut existing.evidence_records, incoming.evidence_records);
    existing.confidence = merged_confidence(
        trust_state,
        existing_sources,
        incoming_sources,
        &existing.evidence_records,
    );
    existing
}

pub(super) fn conflicting_source_identity(
    existing: &LanCanonicalHouseholdDevice,
    incoming: &LanCanonicalHouseholdDevice,
) -> bool {
    let existing_source_ids = source_device_ids(existing);
    let incoming_source_ids = source_device_ids(incoming);

    !existing_source_ids.is_empty()
        && !incoming_source_ids.is_empty()
        && existing_source_ids.is_disjoint(&incoming_source_ids)
}

fn source_device_ids(device: &LanCanonicalHouseholdDevice) -> HashSet<String> {
    let non_parent_ids = device
        .network_identity
        .evidence_records
        .iter()
        .filter(|record| record.evidence_kind != LanDiscoveryEvidenceKind::ParentDecision)
        .filter_map(|record| normalized_source_device_id(&record.device_id))
        .collect::<HashSet<_>>();

    if non_parent_ids.is_empty() {
        device
            .network_identity
            .evidence_records
            .iter()
            .filter_map(|record| normalized_source_device_id(&record.device_id))
            .collect()
    } else {
        non_parent_ids
    }
}

fn normalized_source_device_id(device_id: &str) -> Option<String> {
    let trimmed = device_id.trim();
    (!trimmed.is_empty()).then(|| trimmed.to_string())
}

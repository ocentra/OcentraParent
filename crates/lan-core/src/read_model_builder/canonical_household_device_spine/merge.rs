#[path = "merge_evidence.rs"]
mod merge_evidence;
#[path = "merge_ranking.rs"]
mod merge_ranking;

use std::collections::HashSet;

use super::values::surfaces_for;
use merge_evidence::merge_evidence_records;
use merge_ranking::{
    stronger_classification, stronger_discovery_state, stronger_route_state, stronger_trust_state,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingTrustState;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDevice;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceConfidence;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceSource;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdNetworkIdentity;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdSurface;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceKind;

pub(super) fn merge_device(
    existing: &mut LanCanonicalHouseholdDevice,
    incoming: LanCanonicalHouseholdDevice,
) {
    existing.classification = stronger_classification(
        existing.classification.clone(),
        incoming.classification.clone(),
    );
    existing.enrollable = existing.enrollable || incoming.enrollable;
    let merged_discovery_state = stronger_discovery_state(
        existing.discovery_state.clone(),
        incoming.discovery_state.clone(),
    );
    existing.discovery_state = merged_discovery_state;
    let merged_trust_state =
        stronger_trust_state(existing.trust_state.clone(), incoming.trust_state);
    existing.trust_state = merged_trust_state.clone();
    existing.route_id = existing.route_id.clone().or(incoming.route_id);
    existing.route_state = stronger_route_state(existing.route_state.clone(), incoming.route_state);
    existing.display_name = preferred_display_name(&existing.display_name, &incoming.display_name);
    existing.network_identity = merge_network_identity(
        existing.network_identity.clone(),
        incoming.network_identity,
        &merged_trust_state,
        &existing.source_labels,
        &incoming.source_labels,
    );
    merge_sources(&mut existing.source_labels, incoming.source_labels);
    existing.policy_target_surfaces =
        merged_surfaces(existing.enrollable, incoming.policy_target_surfaces);
    merge_roles(&mut existing.role_badges, incoming.role_badges);
    if incoming.child_agent_inventory.is_some()
        && (merged_trust_state == LanPairingTrustState::Paired
            || existing.child_agent_inventory.is_none())
    {
        existing.child_agent_inventory = incoming.child_agent_inventory;
    }
}

fn preferred_display_name(existing: &str, incoming: &str) -> String {
    if existing.starts_with(constants::lan_pairing::NETWORK_NEIGHBOR_LABEL_PREFIX) {
        incoming.to_string()
    } else {
        existing.to_string()
    }
}

fn merge_network_identity(
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

fn merged_confidence(
    trust_state: &LanPairingTrustState,
    existing_sources: &[LanCanonicalHouseholdDeviceSource],
    incoming_sources: &[LanCanonicalHouseholdDeviceSource],
    evidence_records: &[ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceRecord],
) -> LanCanonicalHouseholdDeviceConfidence {
    if *trust_state == LanPairingTrustState::Paired {
        return LanCanonicalHouseholdDeviceConfidence::AgentConfirmed;
    }
    let has_local = source_present(
        existing_sources,
        &LanCanonicalHouseholdDeviceSource::LocalService,
    ) || source_present(
        incoming_sources,
        &LanCanonicalHouseholdDeviceSource::LocalService,
    );
    let has_neighbor = source_present(
        existing_sources,
        &LanCanonicalHouseholdDeviceSource::NetworkNeighbor,
    ) || source_present(
        incoming_sources,
        &LanCanonicalHouseholdDeviceSource::NetworkNeighbor,
    );
    let has_mac_identity_warning = evidence_records.iter().any(|record| {
        record.evidence_kind
            == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceKind::Vendor
            && matches!(
                record.confidence,
                ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceConfidence::ManualRequired
                    | ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceConfidence::Rejected
            )
    });
    if has_local && has_neighbor {
        if has_mac_identity_warning {
            LanCanonicalHouseholdDeviceConfidence::AgentConfirmed
        } else {
            LanCanonicalHouseholdDeviceConfidence::MacIpMatch
        }
    } else if has_local {
        LanCanonicalHouseholdDeviceConfidence::AgentConfirmed
    } else if has_neighbor {
        if has_mac_identity_warning {
            LanCanonicalHouseholdDeviceConfidence::ManualRequired
        } else {
            LanCanonicalHouseholdDeviceConfidence::NetworkNeighbor
        }
    } else {
        LanCanonicalHouseholdDeviceConfidence::ManualRequired
    }
}

fn source_present(
    sources: &[LanCanonicalHouseholdDeviceSource],
    source: &LanCanonicalHouseholdDeviceSource,
) -> bool {
    sources.iter().any(|entry| entry == source)
}

fn merge_roles(
    existing: &mut Vec<ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceRole>,
    incoming: Vec<ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceRole>,
) {
    for role in incoming {
        if !existing.contains(&role) {
            existing.push(role);
        }
    }
}

fn merge_sources(
    existing: &mut Vec<LanCanonicalHouseholdDeviceSource>,
    incoming: Vec<LanCanonicalHouseholdDeviceSource>,
) {
    for source in incoming {
        if !existing.contains(&source) {
            existing.push(source);
        }
    }
}

fn merged_surfaces(
    is_child_agent: bool,
    incoming: Vec<LanCanonicalHouseholdSurface>,
) -> Vec<LanCanonicalHouseholdSurface> {
    let mut surfaces = surfaces_for(is_child_agent);
    for surface in incoming {
        if !surfaces.contains(&surface) {
            surfaces.push(surface);
        }
    }
    surfaces
}

fn merge_strings(existing: &mut Vec<String>, incoming: Vec<String>) {
    for value in incoming {
        if !existing
            .iter()
            .any(|entry| entry.eq_ignore_ascii_case(&value))
        {
            existing.push(value);
        }
    }
}

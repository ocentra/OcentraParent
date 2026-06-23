use super::values::surfaces_for;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingProductionDiscoveryState;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingTrustState;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDevice;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceConfidence;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceSource;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdNetworkIdentity;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdRouteState;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdSurface;

pub(super) fn merge_device(
    existing: &mut LanCanonicalHouseholdDevice,
    incoming: LanCanonicalHouseholdDevice,
) {
    if existing.classification
        != ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification::ChildAgent
    {
        existing.classification = incoming.classification.clone();
    }
    existing.enrollable = existing.enrollable || incoming.enrollable;
    existing.discovery_state = stronger_discovery_state(
        existing.discovery_state.clone(),
        incoming.discovery_state.clone(),
    );
    existing.trust_state = stronger_trust_state(existing.trust_state.clone(), incoming.trust_state);
    existing.route_id = existing.route_id.clone().or(incoming.route_id);
    existing.route_state = stronger_route_state(existing.route_state.clone(), incoming.route_state);
    existing.display_name = preferred_display_name(&existing.display_name, &incoming.display_name);
    existing.network_identity = merge_network_identity(
        existing.network_identity.clone(),
        incoming.network_identity,
        &existing.source_labels,
        &incoming.source_labels,
    );
    merge_sources(&mut existing.source_labels, incoming.source_labels);
    existing.policy_target_surfaces =
        merged_surfaces(existing.enrollable, incoming.policy_target_surfaces);
    merge_roles(&mut existing.role_badges, incoming.role_badges);
    if existing.child_agent_inventory.is_none() {
        existing.child_agent_inventory = incoming.child_agent_inventory;
    }
}

fn stronger_discovery_state(
    existing: LanPairingProductionDiscoveryState,
    incoming: LanPairingProductionDiscoveryState,
) -> LanPairingProductionDiscoveryState {
    if incoming == LanPairingProductionDiscoveryState::Paired {
        incoming
    } else {
        existing
    }
}

fn stronger_trust_state(
    existing: LanPairingTrustState,
    incoming: LanPairingTrustState,
) -> LanPairingTrustState {
    if incoming == LanPairingTrustState::Paired {
        incoming
    } else {
        existing
    }
}

fn stronger_route_state(
    existing: LanCanonicalHouseholdRouteState,
    incoming: LanCanonicalHouseholdRouteState,
) -> LanCanonicalHouseholdRouteState {
    if incoming == LanCanonicalHouseholdRouteState::LocalNetwork {
        incoming
    } else {
        existing
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
    existing_sources: &[LanCanonicalHouseholdDeviceSource],
    incoming_sources: &[LanCanonicalHouseholdDeviceSource],
) -> LanCanonicalHouseholdNetworkIdentity {
    if existing.hostname.is_none() {
        existing.hostname = incoming.hostname;
    }
    if existing.mac_address.is_none() {
        existing.mac_address = incoming.mac_address;
    }
    merge_strings(&mut existing.ip_addresses, incoming.ip_addresses);
    merge_strings(
        &mut existing.network_interfaces,
        incoming.network_interfaces,
    );
    merge_evidence_records(&mut existing.evidence_records, incoming.evidence_records);
    existing.confidence = merged_confidence(existing_sources, incoming_sources);
    existing
}

fn merged_confidence(
    existing_sources: &[LanCanonicalHouseholdDeviceSource],
    incoming_sources: &[LanCanonicalHouseholdDeviceSource],
) -> LanCanonicalHouseholdDeviceConfidence {
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
    if has_local && has_neighbor {
        LanCanonicalHouseholdDeviceConfidence::MacIpMatch
    } else if has_local {
        LanCanonicalHouseholdDeviceConfidence::AgentConfirmed
    } else if has_neighbor {
        LanCanonicalHouseholdDeviceConfidence::NetworkNeighbor
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

fn merge_evidence_records(
    existing: &mut Vec<ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceRecord>,
    incoming: Vec<ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceRecord>,
) {
    for record in incoming {
        if !existing
            .iter()
            .any(|entry| entry.merge_key.eq_ignore_ascii_case(&record.merge_key))
        {
            existing.push(record);
        }
    }
}

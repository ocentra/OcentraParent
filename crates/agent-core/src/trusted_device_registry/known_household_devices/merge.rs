use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingTrustState;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDevice, LanCanonicalHouseholdDeviceClassification,
    LanCanonicalHouseholdDeviceConfidence, LanCanonicalHouseholdDeviceSource,
    LanCanonicalHouseholdNetworkIdentity, LanDiscoveryEvidenceConfidence, LanDiscoveryEvidenceKind,
    LanDiscoveryEvidenceRecord,
};

mod rank;
use self::rank::{stronger_discovery_state, stronger_route_state, stronger_trust_state};

pub(super) fn same_known_household_device(
    existing: &LanCanonicalHouseholdDevice,
    incoming: &LanCanonicalHouseholdDevice,
) -> bool {
    existing.canonical_device_id == incoming.canonical_device_id
        || existing
            .network_identity
            .mac_address
            .as_deref()
            .zip(incoming.network_identity.mac_address.as_deref())
            .map(|(left, right)| left.eq_ignore_ascii_case(right))
            .unwrap_or(false)
}

pub(super) fn merge_known_household_device(
    existing: &mut LanCanonicalHouseholdDevice,
    incoming: LanCanonicalHouseholdDevice,
) {
    existing.display_name = preferred_display_name(
        &existing.display_name,
        &incoming.display_name,
        &incoming.network_identity.evidence_records,
    );
    existing.classification = preferred_classification(
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
        stronger_trust_state(existing.trust_state.clone(), incoming.trust_state.clone());
    existing.trust_state = merged_trust_state.clone();
    existing.route_id = existing.route_id.clone().or(incoming.route_id);
    existing.route_state =
        stronger_route_state(existing.route_state.clone(), incoming.route_state.clone());
    existing.network_mode = incoming.network_mode;
    existing.network_identity = merge_network_identity(
        existing.network_identity.clone(),
        incoming.network_identity,
        &merged_trust_state,
        &existing.source_labels,
        &incoming.source_labels,
    );
    merge_source_labels(&mut existing.source_labels, incoming.source_labels);
    merge_surfaces(
        &mut existing.policy_target_surfaces,
        incoming.policy_target_surfaces,
    );
    merge_roles(&mut existing.role_badges, incoming.role_badges);
    if incoming.child_agent_inventory.is_some()
        && (merged_trust_state == LanPairingTrustState::Paired
            || existing.child_agent_inventory.is_none())
    {
        existing.child_agent_inventory = incoming.child_agent_inventory;
    }
}

fn preferred_display_name(
    existing: &str,
    incoming: &str,
    incoming_evidence: &[LanDiscoveryEvidenceRecord],
) -> String {
    if incoming_has_parent_rename_evidence(incoming_evidence) && !incoming.trim().is_empty() {
        return incoming.to_string();
    }
    if existing.starts_with(constants::lan_pairing::NETWORK_NEIGHBOR_LABEL_PREFIX)
        && !incoming.starts_with(constants::lan_pairing::NETWORK_NEIGHBOR_LABEL_PREFIX)
    {
        incoming.to_string()
    } else {
        existing.to_string()
    }
}

fn incoming_has_parent_rename_evidence(incoming_evidence: &[LanDiscoveryEvidenceRecord]) -> bool {
    incoming_evidence.iter().any(|record| {
        record.evidence_kind == LanDiscoveryEvidenceKind::ParentDecision
            && record.value == constants::lan_pairing::HOUSEHOLD_ACTION_RENAME
    })
}

fn preferred_classification(
    existing: LanCanonicalHouseholdDeviceClassification,
    incoming: LanCanonicalHouseholdDeviceClassification,
) -> LanCanonicalHouseholdDeviceClassification {
    use LanCanonicalHouseholdDeviceClassification::{
        ChildAgent, NetworkInfrastructure, UnknownLanDevice, UnsupportedLanDevice,
    };

    match (existing, incoming) {
        (ChildAgent, _) | (_, ChildAgent) => ChildAgent,
        (NetworkInfrastructure, _) | (_, NetworkInfrastructure) => NetworkInfrastructure,
        (UnknownLanDevice, other) => other,
        (existing, UnsupportedLanDevice) => existing,
        (_, incoming) => incoming,
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
    merge_string_values(&mut existing.ip_addresses, incoming.ip_addresses);
    merge_string_values(
        &mut existing.network_interfaces,
        incoming.network_interfaces,
    );
    existing.reachability = incoming.reachability;
    existing.stale_at = incoming.stale_at.or(existing.stale_at);
    existing.offline_at = incoming.offline_at.or(existing.offline_at);
    merge_evidence_records(&mut existing.evidence_records, incoming.evidence_records);
    existing.confidence = merged_confidence(
        trust_state,
        existing_sources,
        incoming_sources,
        &existing.evidence_records,
    );
    existing
}

fn merged_confidence(
    trust_state: &LanPairingTrustState,
    existing_sources: &[LanCanonicalHouseholdDeviceSource],
    incoming_sources: &[LanCanonicalHouseholdDeviceSource],
    evidence_records: &[LanDiscoveryEvidenceRecord],
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
        record.evidence_kind == LanDiscoveryEvidenceKind::Vendor
            && matches!(
                record.confidence,
                LanDiscoveryEvidenceConfidence::ManualRequired
                    | LanDiscoveryEvidenceConfidence::Rejected
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
    target: &LanCanonicalHouseholdDeviceSource,
) -> bool {
    sources.contains(target)
}

fn merge_string_values(existing: &mut Vec<String>, incoming: Vec<String>) {
    for value in incoming {
        if !existing
            .iter()
            .any(|entry| entry.eq_ignore_ascii_case(&value))
        {
            existing.push(value);
        }
    }
}

fn merge_source_labels(
    existing: &mut Vec<LanCanonicalHouseholdDeviceSource>,
    incoming: Vec<LanCanonicalHouseholdDeviceSource>,
) {
    for source in incoming {
        if !existing.contains(&source) {
            existing.push(source);
        }
    }
}

fn merge_surfaces(
    existing: &mut Vec<
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdSurface,
    >,
    incoming: Vec<
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdSurface,
    >,
) {
    for surface in incoming {
        if !existing.contains(&surface) {
            existing.push(surface);
        }
    }
}

fn merge_roles(
    existing: &mut Vec<
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceRole,
    >,
    incoming: Vec<
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceRole,
    >,
) {
    for role in incoming {
        if !existing.contains(&role) {
            existing.push(role);
        }
    }
}

fn merge_evidence_records(
    existing: &mut Vec<LanDiscoveryEvidenceRecord>,
    incoming: Vec<LanDiscoveryEvidenceRecord>,
) {
    for record in incoming {
        if let Some(existing_record) = existing
            .iter_mut()
            .find(|entry| same_evidence_record_identity(entry, &record))
        {
            if record.first_seen_at < existing_record.first_seen_at {
                existing_record.first_seen_at = record.first_seen_at.clone();
            }
            if record.last_seen_at > existing_record.last_seen_at {
                existing_record.last_seen_at = record.last_seen_at.clone();
            }
            if evidence_confidence_rank(&record.confidence)
                > evidence_confidence_rank(&existing_record.confidence)
            {
                existing_record.confidence = record.confidence.clone();
            }
            if existing_record.note.is_none() {
                existing_record.note = record.note.clone();
            }
            if record.expires_at.is_some() {
                existing_record.expires_at = record.expires_at.clone();
            }
            continue;
        }
        existing.push(record);
    }
}

fn same_evidence_record_identity(
    existing: &LanDiscoveryEvidenceRecord,
    incoming: &LanDiscoveryEvidenceRecord,
) -> bool {
    existing.source == incoming.source
        && existing.evidence_kind == incoming.evidence_kind
        && existing.merge_key.eq_ignore_ascii_case(&incoming.merge_key)
        && existing.device_id.eq_ignore_ascii_case(&incoming.device_id)
}

fn evidence_confidence_rank(confidence: &LanDiscoveryEvidenceConfidence) -> u8 {
    match confidence {
        LanDiscoveryEvidenceConfidence::Confirmed => 5,
        LanDiscoveryEvidenceConfidence::Strong => 4,
        LanDiscoveryEvidenceConfidence::Weak => 3,
        LanDiscoveryEvidenceConfidence::ManualRequired => 2,
        LanDiscoveryEvidenceConfidence::Rejected => 1,
    }
}

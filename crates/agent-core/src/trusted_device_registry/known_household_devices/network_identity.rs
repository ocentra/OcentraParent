use ocentra_parent_agent_protocol::lan_pairing::LanPairingTrustState;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDeviceConfidence, LanCanonicalHouseholdDeviceSource,
    LanCanonicalHouseholdNetworkIdentity, LanDiscoveryEvidenceConfidence,
    LanDiscoveryEvidenceRecord,
};

use super::evidence::merge_evidence_records;
use super::strings::merge_string_values;

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
            record.evidence_kind == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceKind::Vendor
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

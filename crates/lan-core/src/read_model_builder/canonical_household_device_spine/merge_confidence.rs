use ocentra_parent_agent_protocol::lan_pairing::LanPairingTrustState;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDeviceConfidence, LanCanonicalHouseholdDeviceSource,
    LanDiscoveryEvidenceConfidence, LanDiscoveryEvidenceKind, LanDiscoveryEvidenceRecord,
};

pub(super) fn merged_confidence(
    trust_state: &LanPairingTrustState,
    existing_sources: &[LanCanonicalHouseholdDeviceSource],
    incoming_sources: &[LanCanonicalHouseholdDeviceSource],
    evidence_records: &[LanDiscoveryEvidenceRecord],
) -> LanCanonicalHouseholdDeviceConfidence {
    if *trust_state == LanPairingTrustState::Paired {
        return LanCanonicalHouseholdDeviceConfidence::AgentConfirmed;
    }
    confidence_from_sources(
        source_present(
            existing_sources,
            &LanCanonicalHouseholdDeviceSource::LocalService,
        ) || source_present(
            incoming_sources,
            &LanCanonicalHouseholdDeviceSource::LocalService,
        ),
        source_present(
            existing_sources,
            &LanCanonicalHouseholdDeviceSource::NetworkNeighbor,
        ) || source_present(
            incoming_sources,
            &LanCanonicalHouseholdDeviceSource::NetworkNeighbor,
        ),
        has_mac_identity_warning(evidence_records),
    )
}

fn confidence_from_sources(
    has_local: bool,
    has_neighbor: bool,
    has_mac_identity_warning: bool,
) -> LanCanonicalHouseholdDeviceConfidence {
    match (has_local, has_neighbor, has_mac_identity_warning) {
        (true, true, true) => LanCanonicalHouseholdDeviceConfidence::AgentConfirmed,
        (true, true, false) => LanCanonicalHouseholdDeviceConfidence::MacIpMatch,
        (true, false, _) => LanCanonicalHouseholdDeviceConfidence::AgentConfirmed,
        (false, true, true) => LanCanonicalHouseholdDeviceConfidence::ManualRequired,
        (false, true, false) => LanCanonicalHouseholdDeviceConfidence::NetworkNeighbor,
        (false, false, _) => LanCanonicalHouseholdDeviceConfidence::ManualRequired,
    }
}

fn source_present(
    sources: &[LanCanonicalHouseholdDeviceSource],
    source: &LanCanonicalHouseholdDeviceSource,
) -> bool {
    sources.iter().any(|entry| entry == source)
}

fn has_mac_identity_warning(evidence_records: &[LanDiscoveryEvidenceRecord]) -> bool {
    evidence_records.iter().any(|record| {
        record.evidence_kind == LanDiscoveryEvidenceKind::Vendor
            && matches!(
                record.confidence,
                LanDiscoveryEvidenceConfidence::ManualRequired
                    | LanDiscoveryEvidenceConfidence::Rejected
            )
    })
}

use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDevice, LanDiscoveryEvidenceRecord,
};

pub(super) fn reachability_observed_at(device: &LanCanonicalHouseholdDevice) -> Option<String> {
    match device.network_identity.reachability {
        LanPairingDeviceReachability::Online => {
            latest_evidence_last_seen(&device.network_identity.evidence_records)
                .or_else(|| super::device::device_discovered_at(device))
        }
        LanPairingDeviceReachability::Offline => device
            .network_identity
            .offline_at
            .clone()
            .or_else(|| latest_evidence_last_seen(&device.network_identity.evidence_records))
            .or_else(|| super::device::device_discovered_at(device)),
        LanPairingDeviceReachability::Stale => device
            .network_identity
            .stale_at
            .clone()
            .or_else(|| latest_evidence_last_seen(&device.network_identity.evidence_records))
            .or_else(|| super::device::device_discovered_at(device)),
    }
}

fn latest_evidence_last_seen(evidence_records: &[LanDiscoveryEvidenceRecord]) -> Option<String> {
    let mut latest: Option<String> = None;
    for evidence in evidence_records {
        latest = super::compare::latest_timestamp(
            latest.as_deref(),
            latest_evidence_observed_at(evidence).as_deref(),
        );
    }
    latest
}

pub(super) fn latest_evidence_observed_at(evidence: &LanDiscoveryEvidenceRecord) -> Option<String> {
    if !evidence.last_seen_at.is_empty() {
        Some(evidence.last_seen_at.clone())
    } else {
        super::device::evidence_observed_at(evidence)
    }
}

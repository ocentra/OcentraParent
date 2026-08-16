use ocentra_parent_agent_protocol::lan_pairing::LanPairingDiscoveryRuntimeStatus;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDeviceConfidence, LanCanonicalHouseholdDeviceSource,
    LanDiscoveryEvidenceSource,
};

use super::neighbor_sources::is_network_neighbor_evidence_source;

pub(super) fn confidence_for_discovery(
    status: &LanPairingDiscoveryRuntimeStatus,
) -> LanCanonicalHouseholdDeviceConfidence {
    match status {
        LanPairingDiscoveryRuntimeStatus::WebsocketDirect => {
            LanCanonicalHouseholdDeviceConfidence::AgentConfirmed
        }
        LanPairingDiscoveryRuntimeStatus::NetworkNeighbor => {
            LanCanonicalHouseholdDeviceConfidence::NetworkNeighbor
        }
        LanPairingDiscoveryRuntimeStatus::PlannedUnsupported => {
            LanCanonicalHouseholdDeviceConfidence::ManualRequired
        }
    }
}

pub(super) fn source_for_discovery(
    status: &LanPairingDiscoveryRuntimeStatus,
    evidence_sources: &[LanDiscoveryEvidenceSource],
) -> LanCanonicalHouseholdDeviceSource {
    if evidence_sources.contains(&LanDiscoveryEvidenceSource::LocalService) {
        return LanCanonicalHouseholdDeviceSource::LocalService;
    }
    if evidence_sources
        .iter()
        .any(is_network_neighbor_evidence_source)
    {
        return LanCanonicalHouseholdDeviceSource::NetworkNeighbor;
    }
    match status {
        LanPairingDiscoveryRuntimeStatus::WebsocketDirect => {
            LanCanonicalHouseholdDeviceSource::LocalService
        }
        LanPairingDiscoveryRuntimeStatus::NetworkNeighbor => {
            LanCanonicalHouseholdDeviceSource::NetworkNeighbor
        }
        LanPairingDiscoveryRuntimeStatus::PlannedUnsupported => {
            LanCanonicalHouseholdDeviceSource::TrustedRegistry
        }
    }
}

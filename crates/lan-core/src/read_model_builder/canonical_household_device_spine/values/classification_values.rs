use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDeviceClassification, LanDiscoveryEvidenceSource,
    LanServiceIdentityProbeEvidence,
};

use super::neighbor_sources::is_network_neighbor_evidence_source;
use super::value_support::{
    has_child_agent_evidence, inferred_household_device_classification, known_hostname,
};
use crate::network_inventory::api::is_confirmed_agent_status;

pub(super) fn classification_for_discovery(
    device: &LanPairingDeviceRef,
    evidence_sources: &[LanDiscoveryEvidenceSource],
    service_identity_probe_evidence: &[LanServiceIdentityProbeEvidence],
) -> LanCanonicalHouseholdDeviceClassification {
    if device.platform == constants::lan_pairing::PLATFORM_ROUTER {
        return LanCanonicalHouseholdDeviceClassification::NetworkInfrastructure;
    }
    if device.child_profile_id.is_some()
        || is_confirmed_agent_status(device.agent_status.as_deref())
        || device.hardware_profile.is_some()
        || has_child_agent_evidence(evidence_sources)
    {
        return LanCanonicalHouseholdDeviceClassification::ChildAgent;
    }
    if let Some(classification) =
        inferred_household_device_classification(device, service_identity_probe_evidence)
    {
        return classification;
    }
    if device.platform != constants::lan_pairing::PLATFORM_UNKNOWN
        || has_lan_identity_evidence(device, evidence_sources, service_identity_probe_evidence)
    {
        return LanCanonicalHouseholdDeviceClassification::UnknownLanDevice;
    }
    LanCanonicalHouseholdDeviceClassification::UnsupportedLanDevice
}

fn has_lan_identity_evidence(
    device: &LanPairingDeviceRef,
    evidence_sources: &[LanDiscoveryEvidenceSource],
    service_identity_probe_evidence: &[LanServiceIdentityProbeEvidence],
) -> bool {
    device.ip_address.is_some()
        || device.mac_address.is_some()
        || known_hostname(device).is_some()
        || !service_identity_probe_evidence.is_empty()
        || evidence_sources
            .iter()
            .any(is_network_neighbor_evidence_source)
}

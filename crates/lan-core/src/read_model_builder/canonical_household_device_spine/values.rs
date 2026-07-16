pub(super) mod child_profile;
mod classification_values;
mod device_identity;
mod discovery_status;
mod evidence;
mod evidence_overlap;
mod inventory_values;
mod neighbor_sources;
mod network_identity;
mod route_values;
mod trust_state;
#[path = "value_support.rs"]
mod value_support;

use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDiscoveryRuntimeStatus;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingProductionDiscoveryState;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingTrustState;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDevice;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceConfidence;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceRole;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceSource;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdNetworkIdentity;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdSurface;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanChildAgentInventoryPacket;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceKind;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanServiceIdentityProbeEvidence;

pub fn canonical_device_id(device: &LanPairingDeviceRef) -> String {
    device_identity::canonical_device_id(device)
}

pub fn classification_for_discovery(
    device: &LanPairingDeviceRef,
    evidence_sources: &[LanDiscoveryEvidenceSource],
    service_identity_probe_evidence: &[LanServiceIdentityProbeEvidence],
) -> LanCanonicalHouseholdDeviceClassification {
    classification_values::classification_for_discovery(
        device,
        evidence_sources,
        service_identity_probe_evidence,
    )
}

pub fn display_name_for(device: &LanPairingDeviceRef) -> String {
    device_identity::display_name_for(device)
}

pub struct NetworkIdentityInput<'a> {
    pub device: &'a LanPairingDeviceRef,
    pub pairing_id: Option<&'a str>,
    pub reachability: LanPairingDeviceReachability,
    pub confidence: LanCanonicalHouseholdDeviceConfidence,
    pub source: &'a LanCanonicalHouseholdDeviceSource,
    pub evidence_sources: &'a [LanDiscoveryEvidenceSource],
    pub hint_sources: &'a [LanDiscoveryEvidenceSource],
    pub service_identity_probe_evidence: &'a [LanServiceIdentityProbeEvidence],
    pub observed_at: &'a str,
}

pub fn network_identity_for(
    input: NetworkIdentityInput<'_>,
) -> LanCanonicalHouseholdNetworkIdentity {
    network_identity::network_identity_for(input)
}

pub fn confidence_for_discovery(
    status: &LanPairingDiscoveryRuntimeStatus,
) -> LanCanonicalHouseholdDeviceConfidence {
    discovery_status::confidence_for_discovery(status)
}

pub fn source_for_discovery(
    status: &LanPairingDiscoveryRuntimeStatus,
    evidence_sources: &[LanDiscoveryEvidenceSource],
) -> LanCanonicalHouseholdDeviceSource {
    discovery_status::source_for_discovery(status, evidence_sources)
}

pub fn role_badges_for(
    is_child_agent: bool,
    status: &LanPairingDiscoveryRuntimeStatus,
) -> Vec<LanCanonicalHouseholdDeviceRole> {
    route_values::role_badges_for(is_child_agent, status)
}

pub fn route_id_for(is_child_agent: bool, route_id: Option<String>) -> Option<String> {
    route_values::route_id_for(is_child_agent, route_id)
}

pub fn route_state_for(
    is_child_agent: bool,
    status: &LanPairingDiscoveryRuntimeStatus,
) -> ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdRouteState
{
    route_values::route_state_for(is_child_agent, status)
}

pub fn child_agent_inventory_for(
    is_child_agent: bool,
    device: &LanPairingDeviceRef,
    trust_state: LanPairingTrustState,
    route_state: ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdRouteState,
) -> Option<LanChildAgentInventoryPacket> {
    inventory_values::child_agent_inventory_for(is_child_agent, device, trust_state, route_state)
}

pub fn surfaces_for(is_child_agent: bool) -> Vec<LanCanonicalHouseholdSurface> {
    inventory_values::surfaces_for(is_child_agent)
}

pub fn state_from_trust(trust_state: &LanPairingTrustState) -> LanPairingProductionDiscoveryState {
    trust_state::state_from_trust(trust_state)
}

pub fn option_overlaps(first: Option<&String>, second: Option<&String>) -> bool {
    value_support::option_overlaps(first, second)
}

pub fn evidence_kind_overlaps(
    existing: &LanCanonicalHouseholdDevice,
    incoming: &LanCanonicalHouseholdDevice,
    kinds: &[LanDiscoveryEvidenceKind],
) -> bool {
    evidence_overlap::evidence_kind_overlaps(existing, incoming, kinds)
}

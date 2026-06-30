mod evidence;
#[path = "value_support.rs"]
mod value_support;

use self::evidence::{evidence_records_for, EvidenceRecordsInput};
use self::value_support::{
    child_agent_capabilities, compact_identifier, confidence_for_mac_identity,
    has_child_agent_evidence, inferred_household_device_classification, known_hostname,
    offline_at_for, option_overlaps as option_overlaps_impl, preferred_mac_identity, stale_at_for,
};
use ocentra_parent_agent_protocol::constants;
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
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdRoleState;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdRouteState;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdSurface;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanChildAgentInventoryPacket;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceConfidence;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceKind;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanServiceIdentityProbeEvidence;

use crate::mac_identity::assess_mac_address;
use crate::network_inventory::api::is_confirmed_agent_status;

const CHILD_PROFILE_DEVICE_PREFIX: &str = "lan-child-profile-";

pub fn canonical_device_id(device: &LanPairingDeviceRef) -> String {
    if let Some(child_profile_id) = child_profile_device_id(device).as_deref() {
        let mut id = String::from(CHILD_PROFILE_DEVICE_PREFIX);
        id.push_str(child_profile_id);
        return id;
    }
    if let Some(mac) = preferred_mac_identity(device).as_deref() {
        let mut id = String::from(constants::lan_pairing::CANONICAL_DEVICE_MAC_PREFIX);
        id.push_str(&compact_identifier(mac));
        let device_suffix = compact_identifier(&device.device_id);
        if !device_suffix.is_empty() {
            id.push('-');
            id.push_str(&device_suffix);
        }
        return id;
    }
    let mut id = String::from(constants::lan_pairing::CANONICAL_DEVICE_ID_PREFIX);
    id.push_str(&compact_identifier(&device.device_id));
    id
}

pub fn classification_for_discovery(
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

pub fn display_name_for(device: &LanPairingDeviceRef) -> String {
    known_hostname(device).unwrap_or_else(|| device.label.clone())
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
    let NetworkIdentityInput {
        device,
        pairing_id,
        reachability,
        confidence,
        source,
        evidence_sources,
        hint_sources,
        service_identity_probe_evidence,
        observed_at,
    } = input;
    let mac_assessment = assess_mac_address(device.mac_address.as_deref());
    let stable_mac_assessment = mac_assessment
        .as_ref()
        .filter(|assessment| assessment.stable_identity_key_allowed());
    let effective_confidence = confidence_for_mac_identity(confidence, mac_assessment.as_ref());
    LanCanonicalHouseholdNetworkIdentity {
        hostname: known_hostname(device),
        ip_addresses: device.ip_address.clone().into_iter().collect(),
        mac_address: stable_mac_assessment
            .as_ref()
            .and_then(|assessment| assessment.normalized_owned()),
        mac_vendor: stable_mac_assessment
            .as_ref()
            .and_then(|assessment| assessment.vendor_name())
            .map(str::to_string),
        network_interfaces: device.network_interface.clone().into_iter().collect(),
        stale_at: stale_at_for(&reachability, observed_at),
        offline_at: offline_at_for(&reachability, observed_at),
        reachability,
        confidence: effective_confidence,
        evidence_records: evidence_records_for(&EvidenceRecordsInput {
            device,
            pairing_id,
            source,
            evidence_sources,
            hint_sources,
            service_identity_probe_evidence,
            observed_at,
            mac_assessment: mac_assessment.as_ref(),
        }),
    }
}

pub fn confidence_for_discovery(
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

pub fn source_for_discovery(
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

fn is_network_neighbor_evidence_source(source: &LanDiscoveryEvidenceSource) -> bool {
    matches!(
        source,
        LanDiscoveryEvidenceSource::WindowsNeighborTable
            | LanDiscoveryEvidenceSource::LinuxProcNetArp
            | LanDiscoveryEvidenceSource::LinuxIpNeigh
            | LanDiscoveryEvidenceSource::MacosArp
            | LanDiscoveryEvidenceSource::MdnsDnsSdQuery
            | LanDiscoveryEvidenceSource::SsdpUpnpQuery
            | LanDiscoveryEvidenceSource::DnsCache
            | LanDiscoveryEvidenceSource::Netbios
            | LanDiscoveryEvidenceSource::Llmnr
            | LanDiscoveryEvidenceSource::ServiceIdentityProbe
    )
}

pub fn role_badges_for(
    is_child_agent: bool,
    status: &LanPairingDiscoveryRuntimeStatus,
) -> Vec<LanCanonicalHouseholdDeviceRole> {
    if !is_child_agent {
        return Vec::new();
    }
    let mut roles = vec![LanCanonicalHouseholdDeviceRole::ChildAgent];
    if *status == LanPairingDiscoveryRuntimeStatus::WebsocketDirect {
        roles.push(LanCanonicalHouseholdDeviceRole::Portal);
        roles.push(LanCanonicalHouseholdDeviceRole::ParentController);
    }
    roles
}

pub fn route_id_for(is_child_agent: bool, route_id: Option<String>) -> Option<String> {
    if is_child_agent {
        route_id
    } else {
        None
    }
}

pub fn route_state_for(
    is_child_agent: bool,
    status: &LanPairingDiscoveryRuntimeStatus,
) -> LanCanonicalHouseholdRouteState {
    if !is_child_agent {
        return LanCanonicalHouseholdRouteState::Unavailable;
    }
    match status {
        LanPairingDiscoveryRuntimeStatus::WebsocketDirect => {
            LanCanonicalHouseholdRouteState::LocalNetwork
        }
        LanPairingDiscoveryRuntimeStatus::NetworkNeighbor => {
            LanCanonicalHouseholdRouteState::ManualRequired
        }
        LanPairingDiscoveryRuntimeStatus::PlannedUnsupported => {
            LanCanonicalHouseholdRouteState::Unavailable
        }
    }
}

pub fn child_agent_inventory_for(
    is_child_agent: bool,
    device: &LanPairingDeviceRef,
    trust_state: LanPairingTrustState,
    route_state: LanCanonicalHouseholdRouteState,
) -> Option<LanChildAgentInventoryPacket> {
    let has_child_agent_truth = device.child_profile_id.is_some()
        || is_confirmed_agent_status(device.agent_status.as_deref())
        || device.hardware_profile.is_some()
        || trust_state == LanPairingTrustState::Paired;
    if !is_child_agent || !has_child_agent_truth {
        return None;
    }
    let hardware = device.hardware_profile.clone().unwrap_or_default();
    Some(LanChildAgentInventoryPacket {
        device_name: display_name_for(device),
        platform: device.platform.clone(),
        os: device.platform.clone(),
        cpu_model: hardware.cpu_model,
        cpu_cores: hardware.cpu_cores,
        memory_total: hardware.memory_total,
        gpu_model: hardware.gpu_model,
        gpu_driver: hardware.gpu_driver,
        gpu_memory: hardware.gpu_memory,
        nvidia_smi: hardware.nvidia_smi,
        network_interfaces: device.network_interface.clone().into_iter().collect(),
        capabilities: child_agent_capabilities(),
        role_state: LanCanonicalHouseholdRoleState::Implemented,
        route_state,
        pairing_trust_state: trust_state,
    })
}

pub fn surfaces_for(is_child_agent: bool) -> Vec<LanCanonicalHouseholdSurface> {
    if !is_child_agent {
        return vec![
            LanCanonicalHouseholdSurface::Devices,
            LanCanonicalHouseholdSurface::Network,
        ];
    }
    vec![
        LanCanonicalHouseholdSurface::Devices,
        LanCanonicalHouseholdSurface::Policy,
        LanCanonicalHouseholdSurface::Browser,
        LanCanonicalHouseholdSurface::App,
        LanCanonicalHouseholdSurface::Screen,
        LanCanonicalHouseholdSurface::Network,
        LanCanonicalHouseholdSurface::Activity,
        LanCanonicalHouseholdSurface::Tracking,
        LanCanonicalHouseholdSurface::Ai,
    ]
}

pub fn state_from_trust(trust_state: &LanPairingTrustState) -> LanPairingProductionDiscoveryState {
    match trust_state {
        LanPairingTrustState::Paired => LanPairingProductionDiscoveryState::Paired,
        LanPairingTrustState::Pairing => LanPairingProductionDiscoveryState::Pending,
        LanPairingTrustState::Revoked => LanPairingProductionDiscoveryState::Revoked,
        LanPairingTrustState::Expired => LanPairingProductionDiscoveryState::Expired,
        LanPairingTrustState::Unpaired => LanPairingProductionDiscoveryState::Discovered,
    }
}

pub fn option_overlaps(first: Option<&String>, second: Option<&String>) -> bool {
    option_overlaps_impl(first, second)
}

pub fn evidence_kind_overlaps(
    existing: &LanCanonicalHouseholdDevice,
    incoming: &LanCanonicalHouseholdDevice,
    kinds: &[LanDiscoveryEvidenceKind],
) -> bool {
    existing
        .network_identity
        .evidence_records
        .iter()
        .filter(|record| trusted_merge_evidence(record, kinds))
        .any(|existing_record| {
            incoming
                .network_identity
                .evidence_records
                .iter()
                .filter(|record| trusted_merge_evidence(record, kinds))
                .any(|incoming_record| {
                    existing_record.evidence_kind == incoming_record.evidence_kind
                        && existing_record
                            .normalized_value
                            .eq_ignore_ascii_case(&incoming_record.normalized_value)
                })
        })
}

fn trusted_merge_evidence(
    record: &ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceRecord,
    kinds: &[LanDiscoveryEvidenceKind],
) -> bool {
    if !kinds.contains(&record.evidence_kind) {
        return false;
    }
    if record.evidence_kind == LanDiscoveryEvidenceKind::MacAddress
        && !stable_mac_evidence_value(&record.normalized_value)
    {
        return false;
    }
    matches!(
        record.confidence,
        LanDiscoveryEvidenceConfidence::Confirmed | LanDiscoveryEvidenceConfidence::Strong
    )
}

fn stable_mac_evidence_value(value: &str) -> bool {
    assess_mac_address(Some(value))
        .map(|assessment| assessment.stable_identity_key_allowed())
        .unwrap_or(false)
}

pub fn child_profile_device_id(device: &LanPairingDeviceRef) -> Option<String> {
    device
        .child_profile_id
        .as_deref()
        .map(str::trim)
        .filter(|child_profile_id| !child_profile_id.is_empty())
        .map(compact_identifier)
}

pub fn child_profile_identity_from_canonical(canonical_device_id: &str) -> Option<&str> {
    canonical_device_id.strip_prefix(CHILD_PROFILE_DEVICE_PREFIX)
}

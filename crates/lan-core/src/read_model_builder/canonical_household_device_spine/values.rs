mod evidence;

use self::evidence::evidence_records_for;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDiscoveryRuntimeStatus;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingProductionDiscoveryState;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingTrustState;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceConfidence;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceRole;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceSource;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdNetworkIdentity;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdRoleState;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdRouteState;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdSurface;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanChildAgentInventoryPacket;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource;

use crate::mac_identity::{assess_mac_address, LanMacIdentityAssessment};
use crate::network_inventory::is_confirmed_agent_status;

pub(super) fn canonical_device_id(device: &LanPairingDeviceRef) -> String {
    if let Some(mac) = preferred_mac_identity(device).as_deref() {
        let mut id = String::from(constants::lan_pairing::CANONICAL_DEVICE_MAC_PREFIX);
        id.push_str(&compact_identifier(mac));
        return id;
    }
    let mut id = String::from(constants::lan_pairing::CANONICAL_DEVICE_ID_PREFIX);
    id.push_str(&compact_identifier(&device.device_id));
    id
}

pub(super) fn classification_for(
    device: &LanPairingDeviceRef,
    trusted: bool,
) -> LanCanonicalHouseholdDeviceClassification {
    if device.platform == constants::lan_pairing::PLATFORM_ROUTER {
        return LanCanonicalHouseholdDeviceClassification::NetworkInfrastructure;
    }
    if trusted
        || is_confirmed_agent_status(device.agent_status.as_deref())
        || device.hardware_profile.is_some()
    {
        return LanCanonicalHouseholdDeviceClassification::ChildAgent;
    }
    if device.platform == constants::lan_pairing::PLATFORM_UNKNOWN {
        return LanCanonicalHouseholdDeviceClassification::UnsupportedLanDevice;
    }
    LanCanonicalHouseholdDeviceClassification::UnknownLanDevice
}

pub(super) fn display_name_for(device: &LanPairingDeviceRef) -> String {
    known_hostname(device).unwrap_or_else(|| device.label.clone())
}

pub(super) fn network_identity_for(
    device: &LanPairingDeviceRef,
    reachability: LanPairingDeviceReachability,
    confidence: LanCanonicalHouseholdDeviceConfidence,
    source: &LanCanonicalHouseholdDeviceSource,
    evidence_sources: &[LanDiscoveryEvidenceSource],
    hint_sources: &[LanDiscoveryEvidenceSource],
    observed_at: &str,
) -> LanCanonicalHouseholdNetworkIdentity {
    let mac_assessment = assess_mac_address(device.mac_address.as_deref());
    let effective_confidence = confidence_for_mac_identity(confidence, mac_assessment.as_ref());
    LanCanonicalHouseholdNetworkIdentity {
        hostname: known_hostname(device),
        ip_addresses: device.ip_address.clone().into_iter().collect(),
        mac_address: mac_assessment
            .as_ref()
            .and_then(LanMacIdentityAssessment::normalized_owned),
        mac_vendor: mac_assessment
            .as_ref()
            .and_then(LanMacIdentityAssessment::vendor_name)
            .map(str::to_string),
        network_interfaces: device.network_interface.clone().into_iter().collect(),
        stale_at: stale_at_for(&reachability, observed_at),
        offline_at: offline_at_for(&reachability, observed_at),
        reachability,
        confidence: effective_confidence,
        evidence_records: evidence_records_for(
            device,
            source,
            evidence_sources,
            hint_sources,
            observed_at,
            mac_assessment.as_ref(),
        ),
    }
}

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
    if evidence_sources
        .iter()
        .any(|source| *source == LanDiscoveryEvidenceSource::LocalService)
    {
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
    )
}

pub(super) fn role_badges_for(
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

pub(super) fn route_id_for(is_child_agent: bool, route_id: Option<String>) -> Option<String> {
    if is_child_agent {
        route_id
    } else {
        None
    }
}

pub(super) fn route_state_for(
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

pub(super) fn child_agent_inventory_for(
    is_child_agent: bool,
    device: &LanPairingDeviceRef,
    trust_state: LanPairingTrustState,
    route_state: LanCanonicalHouseholdRouteState,
) -> Option<LanChildAgentInventoryPacket> {
    if !is_child_agent
        || (!is_confirmed_agent_status(device.agent_status.as_deref())
            && device.hardware_profile.is_none())
    {
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

pub(super) fn surfaces_for(is_child_agent: bool) -> Vec<LanCanonicalHouseholdSurface> {
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

pub(super) fn state_from_trust(
    trust_state: &LanPairingTrustState,
) -> LanPairingProductionDiscoveryState {
    match trust_state {
        LanPairingTrustState::Paired => LanPairingProductionDiscoveryState::Paired,
        LanPairingTrustState::Pairing => LanPairingProductionDiscoveryState::Pending,
        LanPairingTrustState::Revoked => LanPairingProductionDiscoveryState::Revoked,
        LanPairingTrustState::Expired => LanPairingProductionDiscoveryState::Expired,
        LanPairingTrustState::Unpaired => LanPairingProductionDiscoveryState::Discovered,
    }
}

pub(super) fn option_overlaps(first: Option<&String>, second: Option<&String>) -> bool {
    first
        .zip(second)
        .and_then(|(left, right)| {
            let left = assess_mac_address(Some(left.as_str()))?;
            let right = assess_mac_address(Some(right.as_str()))?;
            (left.identity_key_allowed() && right.identity_key_allowed()).then_some((left, right))
        })
        .map(|(left, right)| left.normalized() == right.normalized())
        .unwrap_or(false)
}

pub(super) fn compact_identifier(value: &str) -> String {
    value
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

pub(super) fn known_hostname(device: &LanPairingDeviceRef) -> Option<String> {
    device
        .hostname
        .as_ref()
        .filter(|hostname| *hostname != constants::lan_pairing::NETWORK_NEIGHBOR_UNKNOWN_HOSTNAME)
        .cloned()
}

fn stale_at_for(reachability: &LanPairingDeviceReachability, observed_at: &str) -> Option<String> {
    if *reachability == LanPairingDeviceReachability::Stale {
        Some(observed_at.to_string())
    } else {
        None
    }
}

fn offline_at_for(
    reachability: &LanPairingDeviceReachability,
    observed_at: &str,
) -> Option<String> {
    if *reachability == LanPairingDeviceReachability::Offline {
        Some(observed_at.to_string())
    } else {
        None
    }
}

fn child_agent_capabilities() -> Vec<String> {
    vec![
        constants::lan_pairing::CHILD_AGENT_CAPABILITY_DIRECT_WEBSOCKET.to_string(),
        constants::lan_pairing::CHILD_AGENT_CAPABILITY_DEVICE_INVENTORY.to_string(),
        constants::lan_pairing::CHILD_AGENT_CAPABILITY_PAIRING_ROUTE.to_string(),
    ]
}

fn preferred_mac_identity(device: &LanPairingDeviceRef) -> Option<String> {
    let assessment = assess_mac_address(device.mac_address.as_deref())?;
    assessment
        .identity_key_allowed()
        .then_some(assessment.normalized_owned())
        .flatten()
}

fn confidence_for_mac_identity(
    confidence: LanCanonicalHouseholdDeviceConfidence,
    mac_assessment: Option<&LanMacIdentityAssessment>,
) -> LanCanonicalHouseholdDeviceConfidence {
    if matches!(
        confidence,
        LanCanonicalHouseholdDeviceConfidence::NetworkNeighbor
    ) && mac_assessment.is_some_and(|assessment| {
        !assessment.identity_key_allowed()
            || assessment.disposition()
                == crate::mac_identity::LanMacIdentityDisposition::LocallyAdministered
    }) {
        LanCanonicalHouseholdDeviceConfidence::ManualRequired
    } else {
        confidence
    }
}

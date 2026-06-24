use chrono::{SecondsFormat, Utc};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceReachability, LanPairingDiscoveryRuntimeStatus, LanPairingNetworkMode,
    LanPairingProductionDiscoveryState, LanPairingTrustState,
};
use ocentra_parent_agent_protocol::lan_pairing_authority::LanPairingParentAuthority;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::{
    LanDiscoverySourceAuthority, LanDiscoverySourceKind, LanDiscoverySourceMatrix,
    LanDiscoverySourceRow, LanDiscoverySourceRuntimePath, LanDiscoverySourceStatus,
    LanDiscoverySourceUiSurface, LanPlanWorkpackId, LanPlanWorkpackStatusRow,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanBrowserAddDeviceDiscoveryDevice, LanBrowserAddDeviceReadModel,
    LanBrowserAddDeviceScanSummary, LanDiscoveryEvidenceSource, LanPairingDiscoverySource,
    LanSelectedDeviceReadiness,
};

use crate::network_inventory::{
    discover_lan_network_devices, discovery_evidence_sources_for_network_device,
    LanNetworkInventoryDevice,
};
use crate::read_model_builder::{build_lan_add_device_read_model, LanAddDeviceReadModelInput};

pub fn current_lan_add_device_read_model() -> LanBrowserAddDeviceReadModel {
    lan_add_device_read_model_from_inventory(discover_lan_network_devices(), generated_at())
}

pub fn discovered_devices_from_network_inventory(
    network_devices: &[LanNetworkInventoryDevice],
    generated_at: &str,
) -> Vec<LanBrowserAddDeviceDiscoveryDevice> {
    discovered_devices(network_devices, generated_at)
}

fn lan_add_device_read_model_from_inventory(
    network_devices: Vec<LanNetworkInventoryDevice>,
    generated_at: String,
) -> LanBrowserAddDeviceReadModel {
    let discovered_devices = discovered_devices(&network_devices, &generated_at);
    let physical_household_lan_state = if discovered_devices.is_empty() {
        LanPairingProductionDiscoveryState::ManualRequired
    } else {
        LanPairingProductionDiscoveryState::Discovered
    };

    build_lan_add_device_read_model(LanAddDeviceReadModelInput {
        generated_at,
        discovery_source: if discovered_devices.is_empty() {
            LanPairingDiscoverySource::LocalService
        } else {
            LanPairingDiscoverySource::PhysicalHouseholdLan
        },
        add_device_state: physical_household_lan_state.clone(),
        local_service_discovery_state: LanPairingProductionDiscoveryState::Discovered,
        physical_household_lan_state,
        cloud_relay_state: LanPairingProductionDiscoveryState::Unavailable,
        discovered_devices,
        pairing_requests: Vec::new(),
        trusted_device_registry: Vec::new(),
        household_device_decisions: Vec::new(),
        trusted_device_ids: Vec::new(),
        revoked_device_ids: Vec::new(),
        selected_device_readiness: LanSelectedDeviceReadiness {
            schema_version: constants::lan_pairing::SCHEMA_VERSION,
            selected_child_device_id: None,
            route_id: None,
            pairing_id: None,
            trust_state: LanPairingTrustState::Unpaired,
            reachability: LanPairingDeviceReachability::Offline,
            ready_for_control: false,
            stale_at: None,
            offline_at: None,
        },
        controller_authority: LanPairingParentAuthority::ActiveController,
        observer_authority: LanPairingParentAuthority::Observer,
    })
}

fn discovered_devices(
    network_devices: &[LanNetworkInventoryDevice],
    generated_at: &str,
) -> Vec<LanBrowserAddDeviceDiscoveryDevice> {
    network_devices
        .iter()
        .map(|network_device| LanBrowserAddDeviceDiscoveryDevice {
            schema_version: constants::lan_pairing::SCHEMA_VERSION,
            discovered_at: generated_at.to_string(),
            child_device: discovered_child_device_ref(network_device),
            agent_peer_id: network_device.device_id.clone(),
            route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
            network_mode: LanPairingNetworkMode::LocalNetwork,
            reachability: network_device.reachability.clone(),
            address_ref: constants::lan_pairing::ADDRESS_REF_NETWORK_NEIGHBOR.to_string(),
            discovery_status: LanPairingDiscoveryRuntimeStatus::NetworkNeighbor,
            discovery_state: discovery_state_for_reachability(&network_device.reachability),
            evidence_sources: discovery_evidence_sources_for_network_device(network_device),
            hint_sources: discovery_hint_sources(network_device),
        })
        .collect()
}

fn discovered_child_device_ref(
    network_device: &LanNetworkInventoryDevice,
) -> ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef {
    let mut child_device = ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef::new(
        network_device.device_id.clone(),
        None,
        network_device.label.clone(),
        network_device.platform.clone(),
    );
    child_device.ip_address = Some(network_device.ip_address.clone());
    child_device.mac_address = Some(network_device.mac_address.clone());
    child_device.hostname = network_device.hostname.clone();
    child_device.network_interface = network_device.network_interface.clone();
    child_device.agent_status = network_device.agent_status.clone();
    child_device
}

fn discovery_hint_sources(
    network_device: &LanNetworkInventoryDevice,
) -> Vec<LanDiscoveryEvidenceSource> {
    if network_device.used_previous_scan_hint {
        vec![LanDiscoveryEvidenceSource::PreviousScanSnapshot]
    } else {
        Vec::new()
    }
}

fn discovery_state_for_reachability(
    reachability: &LanPairingDeviceReachability,
) -> LanPairingProductionDiscoveryState {
    match reachability {
        LanPairingDeviceReachability::Online => LanPairingProductionDiscoveryState::Discovered,
        LanPairingDeviceReachability::Stale => LanPairingProductionDiscoveryState::Stale,
        LanPairingDeviceReachability::Offline => LanPairingProductionDiscoveryState::Offline,
    }
}

pub(crate) fn audit_check_labels() -> Vec<String> {
    [
        constants::value::LAN_REASON_ANONYMOUS,
        constants::value::LAN_REASON_WRONG_ORIGIN,
        constants::value::LAN_REASON_WRONG_DEVICE,
        constants::value::LAN_REASON_REPLAYED,
        constants::value::LAN_REASON_STALE,
        constants::value::LAN_REASON_REVOKED,
        constants::value::LAN_REASON_OFFLINE,
        constants::value::LAN_REASON_EXPIRED,
    ]
    .iter()
    .map(|label| (*label).to_string())
    .collect()
}

pub(crate) fn honest_non_claims() -> Vec<String> {
    [
        constants::value::LAN_NON_CLAIM_PHYSICAL_HOUSEHOLD_MANUAL_REQUIRED,
        constants::value::LAN_NON_CLAIM_CLOUD_RELAY_NOT_IMPLEMENTED,
        constants::value::LAN_NON_CLAIM_REMOTE_DESKTOP_NOT_IMPLEMENTED,
    ]
    .iter()
    .map(|claim| (*claim).to_string())
    .collect()
}

fn generated_at() -> String {
    Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true)
}

pub(crate) fn lan_discovery_source_matrix(
    generated_at: &str,
    scan_summary: &LanBrowserAddDeviceScanSummary,
) -> LanDiscoverySourceMatrix {
    LanDiscoverySourceMatrix {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        generated_at: generated_at.to_string(),
        workpack_rows: workpack_rows(scan_summary),
        source_rows: source_rows(),
        claims_proved: vec![
            constants::lan_pairing::LAN_SOURCE_MATRIX_CLAIM_READ_MODEL.to_string(),
            constants::lan_pairing::LAN_SOURCE_MATRIX_CLAIM_WEAK_SOURCES.to_string(),
        ],
        claims_not_proved: vec![
            constants::lan_pairing::LAN_SOURCE_MATRIX_NON_CLAIM_PACKET_MODE.to_string(),
            constants::lan_pairing::LAN_SOURCE_MATRIX_NON_CLAIM_PHYSICAL.to_string(),
            constants::lan_pairing::LAN_SOURCE_MATRIX_NON_CLAIM_MDNS_SSDP.to_string(),
        ],
    }
}

fn workpack_rows(scan_summary: &LanBrowserAddDeviceScanSummary) -> Vec<LanPlanWorkpackStatusRow> {
    let mut rows = Vec::new();
    rows.extend(core_workpack_rows(scan_summary));
    rows.extend(packet_boundary_workpack_rows());
    rows.extend(enrichment_workpack_rows());
    rows.extend(closeout_workpack_rows());
    rows
}

fn core_workpack_rows(
    scan_summary: &LanBrowserAddDeviceScanSummary,
) -> Vec<LanPlanWorkpackStatusRow> {
    vec![
        ci_workpack(
            LanPlanWorkpackId::W01,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_01,
        ),
        partial_workpack(
            LanPlanWorkpackId::W02,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_02,
        ),
        partial_workpack(
            LanPlanWorkpackId::W03,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_03,
        ),
        workpack(
            LanPlanWorkpackId::W04,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_04,
            WorkpackDetails {
                discovery_state: neighbor_state(scan_summary),
                proof_state: ocentra_parent_agent_protocol::lan_pairing::V09ProductionDiscoveryHouseholdProofState::CiMechanicalProof,
                runtime_owner: ocentra_parent_agent_protocol::lan_pairing::V09ProductionDiscoveryHouseholdRuntimeOwner::RustServiceReadModel,
                source_status: LanDiscoverySourceStatus::Partial,
                read_model_visible: true,
                required_artifact_summary: None,
            },
        ),
    ]
}

fn packet_boundary_workpack_rows() -> Vec<LanPlanWorkpackStatusRow> {
    vec![
        not_implemented_workpack(
            LanPlanWorkpackId::W05,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_05,
            constants::lan_pairing::LAN_SOURCE_MATRIX_ARTIFACT_PACKET_MODE,
        ),
        not_implemented_workpack(
            LanPlanWorkpackId::W06,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_06,
            constants::lan_pairing::LAN_SOURCE_MATRIX_ARTIFACT_PACKET_MODE,
        ),
        manual_workpack(
            LanPlanWorkpackId::W07,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_07,
            constants::lan_pairing::LAN_SOURCE_MATRIX_ARTIFACT_PACKET_MODE,
        ),
    ]
}

fn enrichment_workpack_rows() -> Vec<LanPlanWorkpackStatusRow> {
    vec![
        manual_workpack(
            LanPlanWorkpackId::W08,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_08,
            constants::lan_pairing::LAN_SOURCE_MATRIX_ARTIFACT_MDNS_SSDP,
        ),
        manual_workpack(
            LanPlanWorkpackId::W09,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_09,
            constants::lan_pairing::LAN_SOURCE_MATRIX_ARTIFACT_MDNS_SSDP,
        ),
        partial_workpack(
            LanPlanWorkpackId::W10,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_10,
        ),
        partial_workpack(
            LanPlanWorkpackId::W11,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_11,
        ),
        partial_workpack(
            LanPlanWorkpackId::W12,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_12,
        ),
        partial_workpack(
            LanPlanWorkpackId::W13,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_13,
        ),
        partial_workpack(
            LanPlanWorkpackId::W14,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_14,
        ),
        partial_workpack(
            LanPlanWorkpackId::W15,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_15,
        ),
    ]
}

fn closeout_workpack_rows() -> Vec<LanPlanWorkpackStatusRow> {
    vec![
        ci_workpack(
            LanPlanWorkpackId::W16,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_16,
        ),
        manual_workpack(
            LanPlanWorkpackId::W17,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_17,
            constants::lan_pairing::LAN_SOURCE_MATRIX_ARTIFACT_MDNS_SSDP,
        ),
        manual_workpack(
            LanPlanWorkpackId::W18,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_18,
            constants::lan_pairing::LAN_SOURCE_MATRIX_ARTIFACT_SIGNED_CHILD,
        ),
        partial_workpack(
            LanPlanWorkpackId::W19,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_19,
        ),
        partial_workpack(
            LanPlanWorkpackId::W20,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_20,
        ),
    ]
}

fn ci_workpack(workpack_id: LanPlanWorkpackId, workpack_title: &str) -> LanPlanWorkpackStatusRow {
    workpack(
        workpack_id,
        workpack_title,
        WorkpackDetails {
            discovery_state: LanPairingProductionDiscoveryState::Discovered,
            proof_state: ocentra_parent_agent_protocol::lan_pairing::V09ProductionDiscoveryHouseholdProofState::CiMechanicalProof,
            runtime_owner: ocentra_parent_agent_protocol::lan_pairing::V09ProductionDiscoveryHouseholdRuntimeOwner::RustServiceReadModel,
            source_status: LanDiscoverySourceStatus::Implemented,
            read_model_visible: true,
            required_artifact_summary: None,
        },
    )
}

fn partial_workpack(
    workpack_id: LanPlanWorkpackId,
    workpack_title: &str,
) -> LanPlanWorkpackStatusRow {
    workpack(
        workpack_id,
        workpack_title,
        WorkpackDetails {
            discovery_state: LanPairingProductionDiscoveryState::Pending,
            proof_state: ocentra_parent_agent_protocol::lan_pairing::V09ProductionDiscoveryHouseholdProofState::CiMechanicalProof,
            runtime_owner: ocentra_parent_agent_protocol::lan_pairing::V09ProductionDiscoveryHouseholdRuntimeOwner::RustServiceReadModel,
            source_status: LanDiscoverySourceStatus::Partial,
            read_model_visible: true,
            required_artifact_summary: None,
        },
    )
}

fn manual_workpack(
    workpack_id: LanPlanWorkpackId,
    workpack_title: &str,
    artifact: &str,
) -> LanPlanWorkpackStatusRow {
    workpack(
        workpack_id,
        workpack_title,
        WorkpackDetails {
            discovery_state: LanPairingProductionDiscoveryState::ManualRequired,
            proof_state: ocentra_parent_agent_protocol::lan_pairing::V09ProductionDiscoveryHouseholdProofState::ManualRequired,
            runtime_owner: ocentra_parent_agent_protocol::lan_pairing::V09ProductionDiscoveryHouseholdRuntimeOwner::ManualProof,
            source_status: LanDiscoverySourceStatus::ManualRequired,
            read_model_visible: true,
            required_artifact_summary: Some(artifact.to_string()),
        },
    )
}

fn not_implemented_workpack(
    workpack_id: LanPlanWorkpackId,
    workpack_title: &str,
    artifact: &str,
) -> LanPlanWorkpackStatusRow {
    workpack(
        workpack_id,
        workpack_title,
        WorkpackDetails {
            discovery_state: LanPairingProductionDiscoveryState::Unavailable,
            proof_state: ocentra_parent_agent_protocol::lan_pairing::V09ProductionDiscoveryHouseholdProofState::NotImplemented,
            runtime_owner: ocentra_parent_agent_protocol::lan_pairing::V09ProductionDiscoveryHouseholdRuntimeOwner::ManualProof,
            source_status: LanDiscoverySourceStatus::NotImplemented,
            read_model_visible: true,
            required_artifact_summary: Some(artifact.to_string()),
        },
    )
}

struct WorkpackDetails {
    discovery_state: LanPairingProductionDiscoveryState,
    proof_state:
        ocentra_parent_agent_protocol::lan_pairing::V09ProductionDiscoveryHouseholdProofState,
    runtime_owner:
        ocentra_parent_agent_protocol::lan_pairing::V09ProductionDiscoveryHouseholdRuntimeOwner,
    source_status: LanDiscoverySourceStatus,
    read_model_visible: bool,
    required_artifact_summary: Option<String>,
}

fn workpack(
    workpack_id: LanPlanWorkpackId,
    workpack_title: &str,
    details: WorkpackDetails,
) -> LanPlanWorkpackStatusRow {
    LanPlanWorkpackStatusRow {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        workpack_id,
        title: workpack_title.to_string(),
        discovery_state: details.discovery_state,
        proof_state: details.proof_state,
        runtime_owner: details.runtime_owner,
        status: details.source_status,
        read_model_visible: details.read_model_visible,
        required_artifact_summary: details.required_artifact_summary,
    }
}

fn neighbor_state(
    scan_summary: &LanBrowserAddDeviceScanSummary,
) -> LanPairingProductionDiscoveryState {
    if scan_summary.passive_device_count > 0 || scan_summary.infrastructure_device_count > 0 {
        LanPairingProductionDiscoveryState::Discovered
    } else {
        LanPairingProductionDiscoveryState::Pending
    }
}

fn source_rows() -> Vec<LanDiscoverySourceRow> {
    let mut rows = Vec::new();
    rows.extend(implemented_source_rows());
    rows.extend(weak_name_source_rows());
    rows.extend(unavailable_source_rows());
    rows.extend(signed_child_source_rows());
    rows
}

struct SourceRowDetails {
    status: LanDiscoverySourceStatus,
    authority: LanDiscoverySourceAuthority,
    runtime_path: LanDiscoverySourceRuntimePath,
    ui_surface: LanDiscoverySourceUiSurface,
    can_confirm_child_agent: bool,
    can_assign_child_profile: bool,
    can_control_route: bool,
    requires_selected_interface: bool,
    persists_across_restart: bool,
    evidence_label: &'static str,
    required_artifact_summary: Option<String>,
}

fn implemented_source_rows() -> Vec<LanDiscoverySourceRow> {
    vec![
        source_row(
            LanDiscoverySourceKind::ContractBoundary,
            LanPlanWorkpackId::W01,
            SourceRowDetails {
                status: LanDiscoverySourceStatus::Implemented,
                authority: LanDiscoverySourceAuthority::ProofGate,
                runtime_path: LanDiscoverySourceRuntimePath::TypescriptContract,
                ui_surface: LanDiscoverySourceUiSurface::ProofReport,
                can_confirm_child_agent: false,
                can_assign_child_profile: false,
                can_control_route: false,
                requires_selected_interface: false,
                persists_across_restart: false,
                evidence_label: constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_01,
                required_artifact_summary: None,
            },
        ),
        source_row(
            LanDiscoverySourceKind::WindowsNeighborTable,
            LanPlanWorkpackId::W04,
            SourceRowDetails {
                status: LanDiscoverySourceStatus::Implemented,
                authority: LanDiscoverySourceAuthority::WeakIdentity,
                runtime_path: LanDiscoverySourceRuntimePath::RustServiceReadModel,
                ui_surface: LanDiscoverySourceUiSurface::DevicesLan,
                can_confirm_child_agent: false,
                can_assign_child_profile: false,
                can_control_route: false,
                requires_selected_interface: true,
                persists_across_restart: false,
                evidence_label: constants::lan_pairing::PRODUCTION_PROOF_LABEL_PASSIVE_NEIGHBOR,
                required_artifact_summary: None,
            },
        ),
        source_row(
            LanDiscoverySourceKind::LinuxProcNetArp,
            LanPlanWorkpackId::W04,
            SourceRowDetails {
                status: LanDiscoverySourceStatus::Implemented,
                authority: LanDiscoverySourceAuthority::WeakIdentity,
                runtime_path: LanDiscoverySourceRuntimePath::RustServiceReadModel,
                ui_surface: LanDiscoverySourceUiSurface::DevicesLan,
                can_confirm_child_agent: false,
                can_assign_child_profile: false,
                can_control_route: false,
                requires_selected_interface: true,
                persists_across_restart: false,
                evidence_label: constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_PROC_NET_ARP,
                required_artifact_summary: None,
            },
        ),
        source_row(
            LanDiscoverySourceKind::LinuxIpNeigh,
            LanPlanWorkpackId::W04,
            SourceRowDetails {
                status: LanDiscoverySourceStatus::Implemented,
                authority: LanDiscoverySourceAuthority::WeakIdentity,
                runtime_path: LanDiscoverySourceRuntimePath::RustServiceReadModel,
                ui_surface: LanDiscoverySourceUiSurface::DevicesLan,
                can_confirm_child_agent: false,
                can_assign_child_profile: false,
                can_control_route: false,
                requires_selected_interface: true,
                persists_across_restart: false,
                evidence_label: constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_IP_NEIGH,
                required_artifact_summary: None,
            },
        ),
        source_row(
            LanDiscoverySourceKind::PreviousScanSnapshot,
            LanPlanWorkpackId::W15,
            SourceRowDetails {
                status: LanDiscoverySourceStatus::Implemented,
                authority: LanDiscoverySourceAuthority::WeakIdentity,
                runtime_path: LanDiscoverySourceRuntimePath::RustServiceReadModel,
                ui_surface: LanDiscoverySourceUiSurface::DevicesLan,
                can_confirm_child_agent: false,
                can_assign_child_profile: false,
                can_control_route: false,
                requires_selected_interface: false,
                persists_across_restart: true,
                evidence_label: constants::lan_pairing::LAN_SCAN_SOURCE_PREVIOUS_SCAN_SNAPSHOT,
                required_artifact_summary: None,
            },
        ),
    ]
}

fn weak_name_source_rows() -> Vec<LanDiscoverySourceRow> {
    vec![
        weak_source(
            LanDiscoverySourceKind::NetbiosNameCache,
            LanPlanWorkpackId::W10,
        ),
        weak_source(
            LanDiscoverySourceKind::LlmnrNameQuery,
            LanPlanWorkpackId::W10,
        ),
        weak_source(
            LanDiscoverySourceKind::ReverseDnsQuery,
            LanPlanWorkpackId::W10,
        ),
        weak_source(
            LanDiscoverySourceKind::MdnsDnsSdQuery,
            LanPlanWorkpackId::W08,
        ),
        weak_source(
            LanDiscoverySourceKind::SsdpUpnpQuery,
            LanPlanWorkpackId::W09,
        ),
        service_identity_probe_source(),
        oui_vendor_lookup_source(),
    ]
}

fn unavailable_source_rows() -> Vec<LanDiscoverySourceRow> {
    vec![
        not_implemented_source(
            LanDiscoverySourceKind::TargetedArpRefresh,
            LanPlanWorkpackId::W05,
        ),
        not_implemented_source(
            LanDiscoverySourceKind::BoundedArpSweep,
            LanPlanWorkpackId::W06,
        ),
        not_implemented_source(
            LanDiscoverySourceKind::PassiveArpListener,
            LanPlanWorkpackId::W07,
        ),
        not_implemented_source(
            LanDiscoverySourceKind::PassiveMdnsListener,
            LanPlanWorkpackId::W07,
        ),
        not_implemented_source(
            LanDiscoverySourceKind::PassiveSsdpListener,
            LanPlanWorkpackId::W07,
        ),
        not_implemented_source(
            LanDiscoverySourceKind::PassiveLlmnrListener,
            LanPlanWorkpackId::W07,
        ),
        not_implemented_source(
            LanDiscoverySourceKind::PassiveNetbiosListener,
            LanPlanWorkpackId::W07,
        ),
        not_implemented_source(
            LanDiscoverySourceKind::ParentMdnsAdvertisement,
            LanPlanWorkpackId::W17,
        ),
        not_implemented_source(
            LanDiscoverySourceKind::ChildMdnsAdvertisement,
            LanPlanWorkpackId::W17,
        ),
    ]
}

fn signed_child_source_rows() -> Vec<LanDiscoverySourceRow> {
    vec![
        source_row(
            LanDiscoverySourceKind::SignedChildAgentHello,
            LanPlanWorkpackId::W18,
            SourceRowDetails {
                status: LanDiscoverySourceStatus::ManualRequired,
                authority: LanDiscoverySourceAuthority::StrongIdentity,
                runtime_path: LanDiscoverySourceRuntimePath::ManualArtifact,
                ui_surface: LanDiscoverySourceUiSurface::ProofReport,
                can_confirm_child_agent: true,
                can_assign_child_profile: false,
                can_control_route: true,
                requires_selected_interface: true,
                persists_across_restart: true,
                evidence_label: constants::lan_pairing::PRODUCTION_PROOF_LABEL_SIGNED_HELLO,
                required_artifact_summary: Some(
                    constants::lan_pairing::LAN_SOURCE_MATRIX_ARTIFACT_SIGNED_CHILD.to_string(),
                ),
            },
        ),
        source_row(
            LanDiscoverySourceKind::SignedChildAgentHeartbeat,
            LanPlanWorkpackId::W18,
            SourceRowDetails {
                status: LanDiscoverySourceStatus::ManualRequired,
                authority: LanDiscoverySourceAuthority::StrongIdentity,
                runtime_path: LanDiscoverySourceRuntimePath::ManualArtifact,
                ui_surface: LanDiscoverySourceUiSurface::ProofReport,
                can_confirm_child_agent: true,
                can_assign_child_profile: false,
                can_control_route: true,
                requires_selected_interface: true,
                persists_across_restart: true,
                evidence_label: constants::lan_pairing::PRODUCTION_PROOF_LABEL_SIGNED_HEARTBEAT,
                required_artifact_summary: Some(
                    constants::lan_pairing::LAN_SOURCE_MATRIX_ARTIFACT_SIGNED_CHILD.to_string(),
                ),
            },
        ),
    ]
}

fn weak_source(
    source: LanDiscoverySourceKind,
    workpack_id: LanPlanWorkpackId,
) -> LanDiscoverySourceRow {
    source_row(
        source,
        workpack_id,
        SourceRowDetails {
            status: LanDiscoverySourceStatus::ManualRequired,
            authority: LanDiscoverySourceAuthority::NameOnly,
            runtime_path: LanDiscoverySourceRuntimePath::ManualArtifact,
            ui_surface: LanDiscoverySourceUiSurface::ProofReport,
            can_confirm_child_agent: false,
            can_assign_child_profile: false,
            can_control_route: false,
            requires_selected_interface: true,
            persists_across_restart: false,
            evidence_label: constants::lan_pairing::LAN_SOURCE_MATRIX_ARTIFACT_MDNS_SSDP,
            required_artifact_summary: Some(
                constants::lan_pairing::LAN_SOURCE_MATRIX_ARTIFACT_MDNS_SSDP.to_string(),
            ),
        },
    )
}

fn oui_vendor_lookup_source() -> LanDiscoverySourceRow {
    source_row(
        LanDiscoverySourceKind::OuiVendorLookup,
        LanPlanWorkpackId::W12,
        SourceRowDetails {
            status: LanDiscoverySourceStatus::Partial,
            authority: LanDiscoverySourceAuthority::ClassificationOnly,
            runtime_path: LanDiscoverySourceRuntimePath::RustServiceReadModel,
            ui_surface: LanDiscoverySourceUiSurface::DevicesLan,
            can_confirm_child_agent: false,
            can_assign_child_profile: false,
            can_control_route: false,
            requires_selected_interface: false,
            persists_across_restart: false,
            evidence_label: constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_12,
            required_artifact_summary: None,
        },
    )
}

fn service_identity_probe_source() -> LanDiscoverySourceRow {
    source_row(
        LanDiscoverySourceKind::ServiceIdentityProbe,
        LanPlanWorkpackId::W11,
        SourceRowDetails {
            status: LanDiscoverySourceStatus::Partial,
            authority: LanDiscoverySourceAuthority::PresenceOnly,
            runtime_path: LanDiscoverySourceRuntimePath::RustServiceReadModel,
            ui_surface: LanDiscoverySourceUiSurface::DevicesLan,
            can_confirm_child_agent: false,
            can_assign_child_profile: false,
            can_control_route: false,
            requires_selected_interface: false,
            persists_across_restart: false,
            evidence_label: constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_11,
            required_artifact_summary: None,
        },
    )
}

fn not_implemented_source(
    source: LanDiscoverySourceKind,
    workpack_id: LanPlanWorkpackId,
) -> LanDiscoverySourceRow {
    source_row(
        source,
        workpack_id,
        SourceRowDetails {
            status: LanDiscoverySourceStatus::NotImplemented,
            authority: LanDiscoverySourceAuthority::NoChildConfirmation,
            runtime_path: LanDiscoverySourceRuntimePath::ManualArtifact,
            ui_surface: LanDiscoverySourceUiSurface::ProofReport,
            can_confirm_child_agent: false,
            can_assign_child_profile: false,
            can_control_route: false,
            requires_selected_interface: true,
            persists_across_restart: false,
            evidence_label: constants::lan_pairing::LAN_SOURCE_MATRIX_ARTIFACT_PACKET_MODE,
            required_artifact_summary: Some(
                constants::lan_pairing::LAN_SOURCE_MATRIX_ARTIFACT_PACKET_MODE.to_string(),
            ),
        },
    )
}

fn source_row(
    source: LanDiscoverySourceKind,
    workpack_id: LanPlanWorkpackId,
    details: SourceRowDetails,
) -> LanDiscoverySourceRow {
    LanDiscoverySourceRow {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        source,
        workpack_id,
        status: details.status,
        authority: details.authority,
        runtime_path: details.runtime_path,
        ui_surface: details.ui_surface,
        can_confirm_child_agent: details.can_confirm_child_agent,
        can_assign_child_profile: details.can_assign_child_profile,
        can_control_route: details.can_control_route,
        requires_selected_interface: details.requires_selected_interface,
        persists_across_restart: details.persists_across_restart,
        evidence_label: details.evidence_label.to_string(),
        required_artifact_summary: details.required_artifact_summary,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn neighbor(
        label: &str,
        hostname: Option<&str>,
        reachability: LanPairingDeviceReachability,
    ) -> LanNetworkInventoryDevice {
        neighbor_with_mac(
            label,
            hostname,
            reachability,
            constants::lan_pairing::TEST_LAN_MAC,
        )
    }

    fn neighbor_with_mac(
        label: &str,
        hostname: Option<&str>,
        reachability: LanPairingDeviceReachability,
        mac_address: &str,
    ) -> LanNetworkInventoryDevice {
        LanNetworkInventoryDevice {
            device_id: format!("network-neighbor-{label}"),
            label: label.to_string(),
            platform: constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
            ip_address: constants::lan_pairing::TEST_LAN_IP.to_string(),
            mac_address: mac_address.to_string(),
            hostname: hostname.map(str::to_string),
            network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
            reachability,
            agent_status: None,
            scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
            used_previous_scan_hint: false,
        }
    }

    #[test]
    fn empty_inventory_stays_honest_about_manual_required_lan_state() {
        let model = lan_add_device_read_model_from_inventory(
            Vec::new(),
            "2026-06-23T00:00:00Z".to_string(),
        );

        assert_eq!(
            model.physical_household_lan_state,
            LanPairingProductionDiscoveryState::ManualRequired
        );
        assert!(model.canonical_household_devices.is_empty());
        assert!(model.production_household_proof.is_some());
        assert!(model.signed_discovery_relay_spine.is_some());
        assert!(model.lan_discovery_source_matrix.is_some());
        assert!(model
            .honest_non_claims
            .iter()
            .any(|claim| claim == constants::value::LAN_NON_CLAIM_REMOTE_DESKTOP_NOT_IMPLEMENTED));
    }

    #[test]
    fn inventory_backed_read_model_preserves_real_neighbor_rows() {
        let model = lan_add_device_read_model_from_inventory(
            vec![neighbor(
                constants::lan_pairing::TEST_HOSTNAME,
                Some(constants::lan_pairing::TEST_HOSTNAME),
                LanPairingDeviceReachability::Online,
            )],
            "2026-06-23T00:00:00Z".to_string(),
        );

        assert_eq!(model.scan_summary.scanned_device_count, 1);
        assert_eq!(
            model.scan_summary.source_labels,
            vec![
                constants::lan_pairing::LAN_SCAN_SOURCE_LOCAL_SERVICE.to_string(),
                constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string(),
            ]
        );
        assert_eq!(
            model.canonical_household_devices[0].display_name,
            constants::lan_pairing::TEST_HOSTNAME
        );
        assert_eq!(
            model.canonical_household_devices[0].source_labels,
            vec![ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceSource::NetworkNeighbor]
        );
    }

    #[test]
    fn service_probe_presence_stays_non_enrollable_but_records_probe_source() {
        let mut discovered = neighbor(
            constants::lan_pairing::TEST_HOSTNAME,
            Some(constants::lan_pairing::TEST_HOSTNAME),
            LanPairingDeviceReachability::Online,
        );
        discovered.agent_status =
            Some(constants::lan_pairing::SERVICE_IDENTITY_PROBE_AGENT_STATUS.to_string());

        let model = lan_add_device_read_model_from_inventory(
            vec![discovered],
            "2026-06-23T00:00:00Z".to_string(),
        );

        assert_eq!(
            model.scan_summary.source_labels,
            vec![
                constants::lan_pairing::LAN_SCAN_SOURCE_LOCAL_SERVICE.to_string(),
                constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string(),
                "service-identity-probe".to_string(),
            ]
        );
        assert_eq!(model.scan_summary.agent_device_count, 0);
        let canonical = &model.canonical_household_devices[0];
        assert!(!canonical.enrollable);
        assert!(canonical.child_agent_inventory.is_none());
        assert!(canonical
            .network_identity
            .evidence_records
            .iter()
            .any(|record| {
                record.evidence_kind
                    == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceKind::ChildAgentPresence
                    && record.confidence
                        == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceConfidence::Weak
            }));
    }

    #[test]
    fn previous_scan_hint_stays_weak_but_visible_in_scan_summary_and_evidence() {
        let mut discovered = neighbor(
            constants::lan_pairing::TEST_HOSTNAME,
            Some(constants::lan_pairing::TEST_HOSTNAME),
            LanPairingDeviceReachability::Online,
        );
        discovered.used_previous_scan_hint = true;

        let model = lan_add_device_read_model_from_inventory(
            vec![discovered],
            "2026-06-23T00:00:00Z".to_string(),
        );

        assert_eq!(
            model.scan_summary.source_labels,
            vec![
                constants::lan_pairing::LAN_SCAN_SOURCE_LOCAL_SERVICE.to_string(),
                constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string(),
                constants::lan_pairing::LAN_SCAN_SOURCE_PREVIOUS_SCAN_SNAPSHOT.to_string(),
            ]
        );
        assert!(model.canonical_household_devices[0]
            .network_identity
            .evidence_records
            .iter()
            .any(|record| {
                record.source
                    == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource::PreviousScanSnapshot
                    && record.evidence_kind
                        == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceKind::HistoricalIdentityHint
                    && record.confidence
                        == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceConfidence::Weak
                    && record.note.as_deref()
                        == Some(constants::lan_pairing::LAN_PREVIOUS_SCAN_CONTINUITY_NOTE)
            }));
        assert!(model
            .lan_discovery_source_matrix
            .as_ref()
            .is_some_and(|matrix| matrix.source_rows.iter().any(|row| {
                row.source
                    == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceKind::PreviousScanSnapshot
                    && row.workpack_id
                        == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanPlanWorkpackId::W15
                    && row.status
                        == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceStatus::Implemented
                    && row.persists_across_restart
            })));
    }

    #[test]
    fn inventory_backed_read_model_keeps_linux_neighbor_sources_truthful() {
        let mut discovered = neighbor(
            constants::lan_pairing::TEST_HOSTNAME,
            Some(constants::lan_pairing::TEST_HOSTNAME),
            LanPairingDeviceReachability::Online,
        );
        discovered.scan_sources = vec![
            constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_IP_NEIGH.to_string(),
            constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_PROC_NET_ARP.to_string(),
        ];

        let model = lan_add_device_read_model_from_inventory(
            vec![discovered],
            "2026-06-23T00:00:00Z".to_string(),
        );

        assert_eq!(
            model.scan_summary.source_labels,
            vec![
                constants::lan_pairing::LAN_SCAN_SOURCE_LOCAL_SERVICE.to_string(),
                constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_IP_NEIGH.to_string(),
                constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_PROC_NET_ARP.to_string(),
            ]
        );
        assert!(model.canonical_household_devices[0]
            .network_identity
            .evidence_records
            .iter()
            .any(|record| {
                record.source
                    == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource::LinuxIpNeigh
                    && record.evidence_kind
                        == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceKind::IpAddress
            }));
    }

    #[test]
    fn oui_vendor_lookup_is_visible_in_read_model_truth() {
        let model = lan_add_device_read_model_from_inventory(
            vec![neighbor(
                constants::lan_pairing::TEST_HOSTNAME,
                Some(constants::lan_pairing::TEST_HOSTNAME),
                LanPairingDeviceReachability::Online,
            )],
            "2026-06-23T00:00:00Z".to_string(),
        );

        let canonical = &model.canonical_household_devices[0];
        assert_eq!(
            canonical.network_identity.mac_vendor.as_deref(),
            Some("AzureWave Technology Inc.")
        );
        assert!(canonical
            .network_identity
            .evidence_records
            .iter()
            .any(|record| {
                record.evidence_kind
                    == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceKind::Vendor
                    && record.confidence
                        == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceConfidence::Strong
                    && record.value == "AzureWave Technology Inc."
            }));
        assert!(model
            .lan_discovery_source_matrix
            .as_ref()
            .is_some_and(|matrix| matrix.source_rows.iter().any(|row| {
                row.source
                    == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceKind::OuiVendorLookup
                    && row.status
                        == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceStatus::Partial
                    && row.authority
                        == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceAuthority::ClassificationOnly
                    && row.runtime_path
                        == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceRuntimePath::RustServiceReadModel
            })));
    }

    #[test]
    fn locally_administered_mac_downgrades_neighbor_confidence_and_emits_warning() {
        let model = lan_add_device_read_model_from_inventory(
            vec![neighbor_with_mac(
                "phone-private-mac",
                Some(constants::lan_pairing::TEST_HOSTNAME),
                LanPairingDeviceReachability::Online,
                "02-aa-bb-cc-dd-ee",
            )],
            "2026-06-23T00:00:00Z".to_string(),
        );

        let canonical = &model.canonical_household_devices[0];
        assert_eq!(
            canonical.network_identity.confidence,
            ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceConfidence::ManualRequired
        );
        assert_eq!(
            canonical.network_identity.mac_address.as_deref(),
            Some("02-aa-bb-cc-dd-ee")
        );
        assert!(canonical
            .network_identity
            .evidence_records
            .iter()
            .any(|record| {
                record.evidence_kind
                    == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceKind::Vendor
                    && record.confidence
                        == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceConfidence::ManualRequired
                    && record.note.as_deref()
                        == Some(constants::lan_pairing::LAN_VENDOR_LOCAL_ADMINISTERED_NOTE)
            }));
    }
}

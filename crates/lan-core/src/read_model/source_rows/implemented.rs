use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::{
    LanDiscoverySourceAuthority, LanDiscoverySourceKind, LanDiscoverySourceRow,
    LanDiscoverySourceRuntimePath, LanDiscoverySourceStatus, LanDiscoverySourceUiSurface,
    LanPlanWorkpackId,
};

use super::{source_row, SourceRowDetails};

pub(super) fn implemented_source_rows() -> Vec<LanDiscoverySourceRow> {
    let mut rows = vec![
        implemented_contract_boundary_source(),
        implemented_evidence_model_source(),
        implemented_interface_selection_source(),
    ];
    rows.extend(implemented_neighbor_source_rows());
    rows.extend(partial_active_refresh_source_rows());
    rows.push(implemented_previous_scan_source());
    rows
}

fn implemented_contract_boundary_source() -> LanDiscoverySourceRow {
    source_row(
        LanDiscoverySourceKind::ContractBoundary,
        LanPlanWorkpackId::W01,
        SourceRowDetails {
            status: LanDiscoverySourceStatus::Implemented,
            authority: LanDiscoverySourceAuthority::ProofGate,
            runtime_path: LanDiscoverySourceRuntimePath::AgentProtocol,
            ui_surface: LanDiscoverySourceUiSurface::ProofReport,
            can_confirm_child_agent: false,
            can_assign_child_profile: false,
            can_control_route: false,
            requires_selected_interface: false,
            persists_across_restart: false,
            evidence_label: constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_01,
            required_artifact_summary: None,
        },
    )
}

fn implemented_evidence_model_source() -> LanDiscoverySourceRow {
    source_row(
        LanDiscoverySourceKind::EvidenceModel,
        LanPlanWorkpackId::W02,
        SourceRowDetails {
            status: LanDiscoverySourceStatus::Implemented,
            authority: LanDiscoverySourceAuthority::ProofGate,
            runtime_path: LanDiscoverySourceRuntimePath::RustServiceReadModel,
            ui_surface: LanDiscoverySourceUiSurface::DevicesLan,
            can_confirm_child_agent: false,
            can_assign_child_profile: false,
            can_control_route: false,
            requires_selected_interface: false,
            persists_across_restart: false,
            evidence_label: constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_02,
            required_artifact_summary: None,
        },
    )
}

fn implemented_interface_selection_source() -> LanDiscoverySourceRow {
    source_row(
        LanDiscoverySourceKind::InterfaceSelection,
        LanPlanWorkpackId::W03,
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
            evidence_label: constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_03,
            required_artifact_summary: None,
        },
    )
}

fn implemented_neighbor_source_rows() -> Vec<LanDiscoverySourceRow> {
    vec![
        implemented_neighbor_source(
            LanDiscoverySourceKind::WindowsNeighborTable,
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_PASSIVE_NEIGHBOR,
        ),
        implemented_neighbor_source(
            LanDiscoverySourceKind::LinuxProcNetArp,
            constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_PROC_NET_ARP,
        ),
        implemented_neighbor_source(
            LanDiscoverySourceKind::LinuxIpNeigh,
            constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_IP_NEIGH,
        ),
        partial_macos_arp_source(),
    ]
}

fn implemented_neighbor_source(
    source: LanDiscoverySourceKind,
    evidence_label: &'static str,
) -> LanDiscoverySourceRow {
    source_row(
        source,
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
            evidence_label,
            required_artifact_summary: None,
        },
    )
}

fn partial_macos_arp_source() -> LanDiscoverySourceRow {
    source_row(
        LanDiscoverySourceKind::MacosArp,
        LanPlanWorkpackId::W04,
        SourceRowDetails {
            status: LanDiscoverySourceStatus::Partial,
            authority: LanDiscoverySourceAuthority::WeakIdentity,
            runtime_path: LanDiscoverySourceRuntimePath::RustServiceReadModel,
            ui_surface: LanDiscoverySourceUiSurface::DevicesLan,
            can_confirm_child_agent: false,
            can_assign_child_profile: false,
            can_control_route: false,
            requires_selected_interface: true,
            persists_across_restart: false,
            evidence_label: constants::lan_pairing::LAN_SCAN_SOURCE_MACOS_ARP,
            required_artifact_summary: Some(
                constants::lan_pairing::LAN_SOURCE_MATRIX_ARTIFACT_PHYSICAL.to_string(),
            ),
        },
    )
}

fn partial_active_refresh_source_rows() -> Vec<LanDiscoverySourceRow> {
    vec![
        implemented_presence_source(
            LanDiscoverySourceKind::TargetedArpRefresh,
            LanPlanWorkpackId::W05,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_05,
        ),
        implemented_presence_source(
            LanDiscoverySourceKind::BoundedArpSweep,
            LanPlanWorkpackId::W06,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_06,
        ),
    ]
}

fn implemented_presence_source(
    source: LanDiscoverySourceKind,
    workpack_id: LanPlanWorkpackId,
    evidence_label: &'static str,
) -> LanDiscoverySourceRow {
    source_row(
        source,
        workpack_id,
        SourceRowDetails {
            status: LanDiscoverySourceStatus::Implemented,
            authority: LanDiscoverySourceAuthority::PresenceOnly,
            runtime_path: LanDiscoverySourceRuntimePath::RustServiceReadModel,
            ui_surface: LanDiscoverySourceUiSurface::DevicesLan,
            can_confirm_child_agent: false,
            can_assign_child_profile: false,
            can_control_route: false,
            requires_selected_interface: true,
            persists_across_restart: false,
            evidence_label,
            required_artifact_summary: None,
        },
    )
}

fn implemented_previous_scan_source() -> LanDiscoverySourceRow {
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
    )
}

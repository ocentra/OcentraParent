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
    rows.extend(unavailable_command_source_rows());
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
    vec![implemented_neighbor_source(
        LanDiscoverySourceKind::LinuxProcNetArp,
        constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_PROC_NET_ARP,
    )]
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

fn unavailable_command_source_rows() -> Vec<LanDiscoverySourceRow> {
    vec![
        implemented_command_source(
            LanDiscoverySourceKind::WindowsNeighborTable,
            LanPlanWorkpackId::W04,
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_PASSIVE_NEIGHBOR,
            LanDiscoverySourceAuthority::WeakIdentity,
        ),
        implemented_command_source(
            LanDiscoverySourceKind::LinuxIpNeigh,
            LanPlanWorkpackId::W04,
            constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_IP_NEIGH,
            LanDiscoverySourceAuthority::WeakIdentity,
        ),
        partial_command_source(
            LanDiscoverySourceKind::MacosArp,
            LanPlanWorkpackId::W04,
            constants::lan_pairing::LAN_SCAN_SOURCE_MACOS_ARP,
            LanDiscoverySourceAuthority::WeakIdentity,
        ),
        implemented_command_source(
            LanDiscoverySourceKind::TargetedArpRefresh,
            LanPlanWorkpackId::W05,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_05,
            LanDiscoverySourceAuthority::PresenceOnly,
        ),
        implemented_command_source(
            LanDiscoverySourceKind::BoundedArpSweep,
            LanPlanWorkpackId::W06,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_06,
            LanDiscoverySourceAuthority::PresenceOnly,
        ),
    ]
}

fn unavailable_command_source(
    source: LanDiscoverySourceKind,
    workpack_id: LanPlanWorkpackId,
    evidence_label: &'static str,
    authority: LanDiscoverySourceAuthority,
) -> LanDiscoverySourceRow {
    source_row(
        source,
        workpack_id,
        SourceRowDetails {
            status: LanDiscoverySourceStatus::NotImplemented,
            authority,
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

fn implemented_command_source(
    source: LanDiscoverySourceKind,
    workpack_id: LanPlanWorkpackId,
    evidence_label: &'static str,
    authority: LanDiscoverySourceAuthority,
) -> LanDiscoverySourceRow {
    let mut row = unavailable_command_source(source, workpack_id, evidence_label, authority);
    row.status = LanDiscoverySourceStatus::Implemented;
    row
}

fn partial_command_source(
    source: LanDiscoverySourceKind,
    workpack_id: LanPlanWorkpackId,
    evidence_label: &'static str,
    authority: LanDiscoverySourceAuthority,
) -> LanDiscoverySourceRow {
    let mut row = unavailable_command_source(source, workpack_id, evidence_label, authority);
    row.status = LanDiscoverySourceStatus::Partial;
    row.required_artifact_summary =
        Some(constants::lan_pairing::LAN_SOURCE_MATRIX_ARTIFACT_PHYSICAL.to_string());
    row
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

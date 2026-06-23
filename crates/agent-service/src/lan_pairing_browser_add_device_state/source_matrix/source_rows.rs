use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceAuthority;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceKind;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceRow;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceRuntimePath;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceStatus;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceUiSurface;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanPlanWorkpackId;

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

pub(super) fn source_rows() -> Vec<LanDiscoverySourceRow> {
    let mut rows = Vec::new();
    rows.extend(implemented_source_rows());
    rows.extend(weak_name_source_rows());
    rows.extend(unavailable_source_rows());
    rows.extend(signed_child_source_rows());
    rows
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
        weak_source(
            LanDiscoverySourceKind::ServiceIdentityProbe,
            LanPlanWorkpackId::W11,
        ),
        weak_source(
            LanDiscoverySourceKind::OuiVendorLookup,
            LanPlanWorkpackId::W12,
        ),
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

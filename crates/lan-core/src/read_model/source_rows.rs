use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::{
    LanDiscoverySourceAuthority, LanDiscoverySourceKind, LanDiscoverySourceRow,
    LanDiscoverySourceRuntimePath, LanDiscoverySourceStatus, LanDiscoverySourceUiSurface,
    LanPlanWorkpackId,
};

pub(super) fn source_rows() -> Vec<LanDiscoverySourceRow> {
    let mut rows = Vec::new();
    rows.extend(implemented_source_rows());
    rows.extend(weak_name_source_rows());
    rows.extend(unavailable_source_rows());
    rows.extend(canonical_spine_source_rows());
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
        ),
    ]
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

fn weak_name_source_rows() -> Vec<LanDiscoverySourceRow> {
    vec![
        partial_name_source(
            LanDiscoverySourceKind::NetbiosNameCache,
            LanPlanWorkpackId::W10,
        ),
        partial_name_source(
            LanDiscoverySourceKind::LlmnrNameQuery,
            LanPlanWorkpackId::W10,
        ),
        partial_name_source(
            LanDiscoverySourceKind::ReverseDnsQuery,
            LanPlanWorkpackId::W10,
        ),
        partial_presence_source(
            LanDiscoverySourceKind::MdnsDnsSdQuery,
            LanPlanWorkpackId::W08,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_08,
        ),
        partial_presence_source(
            LanDiscoverySourceKind::SsdpUpnpQuery,
            LanPlanWorkpackId::W09,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_09,
        ),
        service_identity_probe_source(),
        implemented_oui_vendor_lookup_source(),
    ]
}

fn unavailable_source_rows() -> Vec<LanDiscoverySourceRow> {
    vec![
        partial_presence_source(
            LanDiscoverySourceKind::PassiveArpListener,
            LanPlanWorkpackId::W07,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_07,
        ),
        partial_presence_source(
            LanDiscoverySourceKind::PassiveDhcpListener,
            LanPlanWorkpackId::W07,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_07,
        ),
        partial_presence_source(
            LanDiscoverySourceKind::PassiveMdnsListener,
            LanPlanWorkpackId::W07,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_07,
        ),
        partial_presence_source(
            LanDiscoverySourceKind::PassiveSsdpListener,
            LanPlanWorkpackId::W07,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_07,
        ),
        partial_presence_source(
            LanDiscoverySourceKind::PassiveWsDiscoveryListener,
            LanPlanWorkpackId::W07,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_07,
        ),
        partial_presence_source(
            LanDiscoverySourceKind::PassiveLlmnrListener,
            LanPlanWorkpackId::W07,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_07,
        ),
        partial_presence_source(
            LanDiscoverySourceKind::PassiveNetbiosListener,
            LanPlanWorkpackId::W07,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_07,
        ),
        partial_presence_source(
            LanDiscoverySourceKind::PassiveSnmpResponseListener,
            LanPlanWorkpackId::W07,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_07,
        ),
        parent_mdns_advertisement_source(),
        child_mdns_advertisement_source(),
    ]
}

fn canonical_spine_source_rows() -> Vec<LanDiscoverySourceRow> {
    vec![
        merge_deduplication_source(),
        explainable_classification_source(),
        household_device_store_source(),
        read_model_event_stream_source(),
        assignment_revocation_audit_source(),
        proof_gate_rollout_source(),
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

fn partial_name_source(
    source: LanDiscoverySourceKind,
    workpack_id: LanPlanWorkpackId,
) -> LanDiscoverySourceRow {
    source_row(
        source,
        workpack_id,
        SourceRowDetails {
            status: LanDiscoverySourceStatus::Partial,
            authority: LanDiscoverySourceAuthority::NameOnly,
            runtime_path: LanDiscoverySourceRuntimePath::RustServiceReadModel,
            ui_surface: LanDiscoverySourceUiSurface::DevicesLan,
            can_confirm_child_agent: false,
            can_assign_child_profile: false,
            can_control_route: false,
            requires_selected_interface: true,
            persists_across_restart: false,
            evidence_label: constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_10,
            required_artifact_summary: None,
        },
    )
}

fn partial_presence_source(
    source: LanDiscoverySourceKind,
    workpack_id: LanPlanWorkpackId,
    evidence_label: &'static str,
) -> LanDiscoverySourceRow {
    source_row(
        source,
        workpack_id,
        SourceRowDetails {
            status: LanDiscoverySourceStatus::Partial,
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

fn parent_mdns_advertisement_source() -> LanDiscoverySourceRow {
    source_row(
        LanDiscoverySourceKind::ParentMdnsAdvertisement,
        LanPlanWorkpackId::W17,
        SourceRowDetails {
            status: LanDiscoverySourceStatus::Partial,
            authority: LanDiscoverySourceAuthority::PresenceOnly,
            runtime_path: LanDiscoverySourceRuntimePath::AgentProtocol,
            ui_surface: LanDiscoverySourceUiSurface::ProofReport,
            can_confirm_child_agent: false,
            can_assign_child_profile: false,
            can_control_route: false,
            requires_selected_interface: true,
            persists_across_restart: false,
            evidence_label: constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_17,
            required_artifact_summary: Some(
                constants::lan_pairing::LAN_SOURCE_MATRIX_ARTIFACT_MDNS_SSDP.to_string(),
            ),
        },
    )
}

fn child_mdns_advertisement_source() -> LanDiscoverySourceRow {
    source_row(
        LanDiscoverySourceKind::ChildMdnsAdvertisement,
        LanPlanWorkpackId::W17,
        SourceRowDetails {
            status: LanDiscoverySourceStatus::Partial,
            authority: LanDiscoverySourceAuthority::PresenceOnly,
            runtime_path: LanDiscoverySourceRuntimePath::AgentProtocol,
            ui_surface: LanDiscoverySourceUiSurface::ProofReport,
            can_confirm_child_agent: false,
            can_assign_child_profile: false,
            can_control_route: false,
            requires_selected_interface: true,
            persists_across_restart: false,
            evidence_label: constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_17,
            required_artifact_summary: Some(
                constants::lan_pairing::LAN_SOURCE_MATRIX_ARTIFACT_MDNS_SSDP.to_string(),
            ),
        },
    )
}

fn implemented_oui_vendor_lookup_source() -> LanDiscoverySourceRow {
    source_row(
        LanDiscoverySourceKind::OuiVendorLookup,
        LanPlanWorkpackId::W12,
        SourceRowDetails {
            status: LanDiscoverySourceStatus::Implemented,
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

fn merge_deduplication_source() -> LanDiscoverySourceRow {
    source_row(
        LanDiscoverySourceKind::MergeDeduplication,
        LanPlanWorkpackId::W13,
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
            evidence_label: constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_13,
            required_artifact_summary: None,
        },
    )
}

fn explainable_classification_source() -> LanDiscoverySourceRow {
    source_row(
        LanDiscoverySourceKind::ExplainableClassification,
        LanPlanWorkpackId::W14,
        SourceRowDetails {
            status: LanDiscoverySourceStatus::Implemented,
            authority: LanDiscoverySourceAuthority::ClassificationOnly,
            runtime_path: LanDiscoverySourceRuntimePath::RustServiceReadModel,
            ui_surface: LanDiscoverySourceUiSurface::DevicesLan,
            can_confirm_child_agent: false,
            can_assign_child_profile: false,
            can_control_route: false,
            requires_selected_interface: false,
            persists_across_restart: false,
            evidence_label: constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_14,
            required_artifact_summary: None,
        },
    )
}

fn household_device_store_source() -> LanDiscoverySourceRow {
    source_row(
        LanDiscoverySourceKind::HouseholdDeviceStore,
        LanPlanWorkpackId::W15,
        SourceRowDetails {
            status: LanDiscoverySourceStatus::Implemented,
            authority: LanDiscoverySourceAuthority::ManualParentDecision,
            runtime_path: LanDiscoverySourceRuntimePath::RustServiceReadModel,
            ui_surface: LanDiscoverySourceUiSurface::DevicesLan,
            can_confirm_child_agent: false,
            can_assign_child_profile: true,
            can_control_route: false,
            requires_selected_interface: false,
            persists_across_restart: true,
            evidence_label: constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_15,
            required_artifact_summary: None,
        },
    )
}

fn read_model_event_stream_source() -> LanDiscoverySourceRow {
    source_row(
        LanDiscoverySourceKind::ReadModelEventStream,
        LanPlanWorkpackId::W16,
        SourceRowDetails {
            status: LanDiscoverySourceStatus::Implemented,
            authority: LanDiscoverySourceAuthority::ProofGate,
            runtime_path: LanDiscoverySourceRuntimePath::RustServiceReadModel,
            ui_surface: LanDiscoverySourceUiSurface::ActivityNetwork,
            can_confirm_child_agent: false,
            can_assign_child_profile: false,
            can_control_route: false,
            requires_selected_interface: false,
            persists_across_restart: false,
            evidence_label: constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_16,
            required_artifact_summary: None,
        },
    )
}

fn assignment_revocation_audit_source() -> LanDiscoverySourceRow {
    source_row(
        LanDiscoverySourceKind::AssignmentRevocationAudit,
        LanPlanWorkpackId::W19,
        SourceRowDetails {
            status: LanDiscoverySourceStatus::Implemented,
            authority: LanDiscoverySourceAuthority::ManualParentDecision,
            runtime_path: LanDiscoverySourceRuntimePath::RustServiceReadModel,
            ui_surface: LanDiscoverySourceUiSurface::DevicesLan,
            can_confirm_child_agent: false,
            can_assign_child_profile: true,
            can_control_route: false,
            requires_selected_interface: false,
            persists_across_restart: true,
            evidence_label: constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_19,
            required_artifact_summary: None,
        },
    )
}

fn proof_gate_rollout_source() -> LanDiscoverySourceRow {
    source_row(
        LanDiscoverySourceKind::ProofGateRollout,
        LanPlanWorkpackId::W20,
        SourceRowDetails {
            status: LanDiscoverySourceStatus::Partial,
            authority: LanDiscoverySourceAuthority::ProofGate,
            runtime_path: LanDiscoverySourceRuntimePath::ProofHarness,
            ui_surface: LanDiscoverySourceUiSurface::ProofReport,
            can_confirm_child_agent: false,
            can_assign_child_profile: false,
            can_control_route: false,
            requires_selected_interface: false,
            persists_across_restart: false,
            evidence_label: constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_20,
            required_artifact_summary: Some(
                constants::lan_pairing::LAN_SOURCE_MATRIX_ARTIFACT_PHYSICAL.to_string(),
            ),
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

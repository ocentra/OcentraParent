use ocentra_parent_agent_protocol::{
    constants, LanDiscoverySourceAuthority, LanDiscoverySourceKind, LanDiscoverySourceRow,
    LanDiscoverySourceRuntimePath, LanDiscoverySourceStatus, LanDiscoverySourceUiSurface,
    LanPlanWorkpackId,
};

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
            LanDiscoverySourceStatus::Implemented,
            LanDiscoverySourceAuthority::ProofGate,
            LanDiscoverySourceRuntimePath::TypescriptContract,
            LanDiscoverySourceUiSurface::ProofReport,
            false,
            false,
            false,
            false,
            false,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_01,
            None,
        ),
        source_row(
            LanDiscoverySourceKind::WindowsNeighborTable,
            LanPlanWorkpackId::W04,
            LanDiscoverySourceStatus::Implemented,
            LanDiscoverySourceAuthority::WeakIdentity,
            LanDiscoverySourceRuntimePath::RustServiceReadModel,
            LanDiscoverySourceUiSurface::DevicesLan,
            false,
            false,
            false,
            true,
            false,
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_PASSIVE_NEIGHBOR,
            None,
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
            LanDiscoverySourceStatus::ManualRequired,
            LanDiscoverySourceAuthority::StrongIdentity,
            LanDiscoverySourceRuntimePath::ManualArtifact,
            LanDiscoverySourceUiSurface::ProofReport,
            true,
            false,
            true,
            true,
            true,
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_SIGNED_HELLO,
            Some(constants::lan_pairing::LAN_SOURCE_MATRIX_ARTIFACT_SIGNED_CHILD.to_string()),
        ),
        source_row(
            LanDiscoverySourceKind::SignedChildAgentHeartbeat,
            LanPlanWorkpackId::W18,
            LanDiscoverySourceStatus::ManualRequired,
            LanDiscoverySourceAuthority::StrongIdentity,
            LanDiscoverySourceRuntimePath::ManualArtifact,
            LanDiscoverySourceUiSurface::ProofReport,
            true,
            false,
            true,
            true,
            true,
            constants::lan_pairing::PRODUCTION_PROOF_LABEL_SIGNED_HEARTBEAT,
            Some(constants::lan_pairing::LAN_SOURCE_MATRIX_ARTIFACT_SIGNED_CHILD.to_string()),
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
        LanDiscoverySourceStatus::ManualRequired,
        LanDiscoverySourceAuthority::NameOnly,
        LanDiscoverySourceRuntimePath::ManualArtifact,
        LanDiscoverySourceUiSurface::ProofReport,
        false,
        false,
        false,
        true,
        false,
        constants::lan_pairing::LAN_SOURCE_MATRIX_ARTIFACT_MDNS_SSDP,
        Some(constants::lan_pairing::LAN_SOURCE_MATRIX_ARTIFACT_MDNS_SSDP.to_string()),
    )
}

fn not_implemented_source(
    source: LanDiscoverySourceKind,
    workpack_id: LanPlanWorkpackId,
) -> LanDiscoverySourceRow {
    source_row(
        source,
        workpack_id,
        LanDiscoverySourceStatus::NotImplemented,
        LanDiscoverySourceAuthority::NoChildConfirmation,
        LanDiscoverySourceRuntimePath::ManualArtifact,
        LanDiscoverySourceUiSurface::ProofReport,
        false,
        false,
        false,
        true,
        false,
        constants::lan_pairing::LAN_SOURCE_MATRIX_ARTIFACT_PACKET_MODE,
        Some(constants::lan_pairing::LAN_SOURCE_MATRIX_ARTIFACT_PACKET_MODE.to_string()),
    )
}

#[allow(clippy::too_many_arguments)]
fn source_row(
    source: LanDiscoverySourceKind,
    workpack_id: LanPlanWorkpackId,
    status: LanDiscoverySourceStatus,
    authority: LanDiscoverySourceAuthority,
    runtime_path: LanDiscoverySourceRuntimePath,
    ui_surface: LanDiscoverySourceUiSurface,
    can_confirm_child_agent: bool,
    can_assign_child_profile: bool,
    can_control_route: bool,
    requires_selected_interface: bool,
    persists_across_restart: bool,
    evidence_label: &str,
    required_artifact_summary: Option<String>,
) -> LanDiscoverySourceRow {
    LanDiscoverySourceRow {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        source,
        workpack_id,
        status,
        authority,
        runtime_path,
        ui_surface,
        can_confirm_child_agent,
        can_assign_child_profile,
        can_control_route,
        requires_selected_interface,
        persists_across_restart,
        evidence_label: evidence_label.to_string(),
        required_artifact_summary,
    }
}

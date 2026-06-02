use ocentra_parent_agent_protocol::{
    constants, LanBrowserAddDeviceScanSummary, LanDiscoverySourceAuthority,
    LanDiscoverySourceKind, LanDiscoverySourceMatrix, LanDiscoverySourceRow,
    LanDiscoverySourceRuntimePath, LanDiscoverySourceStatus, LanDiscoverySourceUiSurface,
    LanPairingProductionDiscoveryState, LanPlanWorkpackId, LanPlanWorkpackStatusRow,
    V09ProductionDiscoveryHouseholdProofState, V09ProductionDiscoveryHouseholdRuntimeOwner,
};

pub(super) fn lan_discovery_source_matrix(
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
    vec![
        ci_workpack(LanPlanWorkpackId::W01, constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_01),
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
            neighbor_state(scan_summary),
            V09ProductionDiscoveryHouseholdProofState::CiMechanicalProof,
            V09ProductionDiscoveryHouseholdRuntimeOwner::RustServiceReadModel,
            LanDiscoverySourceStatus::Partial,
            true,
            None,
        ),
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
        not_implemented_workpack(
            LanPlanWorkpackId::W11,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_11,
            constants::lan_pairing::LAN_SOURCE_MATRIX_ARTIFACT_PACKET_MODE,
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
        ci_workpack(LanPlanWorkpackId::W16, constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_16),
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

fn source_rows() -> Vec<LanDiscoverySourceRow> {
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
        weak_source(LanDiscoverySourceKind::NetbiosNameCache, LanPlanWorkpackId::W10),
        weak_source(LanDiscoverySourceKind::LlmnrNameQuery, LanPlanWorkpackId::W10),
        weak_source(LanDiscoverySourceKind::ReverseDnsQuery, LanPlanWorkpackId::W10),
        weak_source(LanDiscoverySourceKind::MdnsDnsSdQuery, LanPlanWorkpackId::W08),
        weak_source(LanDiscoverySourceKind::SsdpUpnpQuery, LanPlanWorkpackId::W09),
        weak_source(LanDiscoverySourceKind::ServiceIdentityProbe, LanPlanWorkpackId::W11),
        weak_source(LanDiscoverySourceKind::OuiVendorLookup, LanPlanWorkpackId::W12),
        not_implemented_source(LanDiscoverySourceKind::TargetedArpRefresh, LanPlanWorkpackId::W05),
        not_implemented_source(LanDiscoverySourceKind::BoundedArpSweep, LanPlanWorkpackId::W06),
        not_implemented_source(LanDiscoverySourceKind::PassiveArpListener, LanPlanWorkpackId::W07),
        not_implemented_source(LanDiscoverySourceKind::PassiveMdnsListener, LanPlanWorkpackId::W07),
        not_implemented_source(LanDiscoverySourceKind::PassiveSsdpListener, LanPlanWorkpackId::W07),
        not_implemented_source(LanDiscoverySourceKind::PassiveLlmnrListener, LanPlanWorkpackId::W07),
        not_implemented_source(LanDiscoverySourceKind::PassiveNetbiosListener, LanPlanWorkpackId::W07),
        not_implemented_source(LanDiscoverySourceKind::ParentMdnsAdvertisement, LanPlanWorkpackId::W17),
        not_implemented_source(LanDiscoverySourceKind::ChildMdnsAdvertisement, LanPlanWorkpackId::W17),
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

fn ci_workpack(workpack_id: LanPlanWorkpackId, title: &str) -> LanPlanWorkpackStatusRow {
    workpack(
        workpack_id,
        title,
        LanPairingProductionDiscoveryState::Discovered,
        V09ProductionDiscoveryHouseholdProofState::CiMechanicalProof,
        V09ProductionDiscoveryHouseholdRuntimeOwner::RustServiceReadModel,
        LanDiscoverySourceStatus::Implemented,
        true,
        None,
    )
}

fn partial_workpack(workpack_id: LanPlanWorkpackId, title: &str) -> LanPlanWorkpackStatusRow {
    workpack(
        workpack_id,
        title,
        LanPairingProductionDiscoveryState::Pending,
        V09ProductionDiscoveryHouseholdProofState::CiMechanicalProof,
        V09ProductionDiscoveryHouseholdRuntimeOwner::RustServiceReadModel,
        LanDiscoverySourceStatus::Partial,
        true,
        None,
    )
}

fn manual_workpack(
    workpack_id: LanPlanWorkpackId,
    title: &str,
    artifact: &str,
) -> LanPlanWorkpackStatusRow {
    workpack(
        workpack_id,
        title,
        LanPairingProductionDiscoveryState::ManualRequired,
        V09ProductionDiscoveryHouseholdProofState::ManualRequired,
        V09ProductionDiscoveryHouseholdRuntimeOwner::ManualProof,
        LanDiscoverySourceStatus::ManualRequired,
        true,
        Some(artifact.to_string()),
    )
}

fn not_implemented_workpack(
    workpack_id: LanPlanWorkpackId,
    title: &str,
    artifact: &str,
) -> LanPlanWorkpackStatusRow {
    workpack(
        workpack_id,
        title,
        LanPairingProductionDiscoveryState::Unavailable,
        V09ProductionDiscoveryHouseholdProofState::NotImplemented,
        V09ProductionDiscoveryHouseholdRuntimeOwner::ManualProof,
        LanDiscoverySourceStatus::NotImplemented,
        true,
        Some(artifact.to_string()),
    )
}

fn workpack(
    workpack_id: LanPlanWorkpackId,
    title: &str,
    discovery_state: LanPairingProductionDiscoveryState,
    proof_state: V09ProductionDiscoveryHouseholdProofState,
    runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner,
    status: LanDiscoverySourceStatus,
    read_model_visible: bool,
    required_artifact_summary: Option<String>,
) -> LanPlanWorkpackStatusRow {
    LanPlanWorkpackStatusRow {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        workpack_id,
        title: title.to_string(),
        discovery_state,
        proof_state,
        runtime_owner,
        status,
        read_model_visible,
        required_artifact_summary,
    }
}

fn weak_source(source: LanDiscoverySourceKind, workpack_id: LanPlanWorkpackId) -> LanDiscoverySourceRow {
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

fn neighbor_state(scan_summary: &LanBrowserAddDeviceScanSummary) -> LanPairingProductionDiscoveryState {
    if scan_summary.passive_device_count > 0 || scan_summary.infrastructure_device_count > 0 {
        LanPairingProductionDiscoveryState::Discovered
    } else {
        LanPairingProductionDiscoveryState::Pending
    }
}

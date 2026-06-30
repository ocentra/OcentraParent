use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::LanDiscoverySourceAuthority;
use ocentra_parent_agent_protocol::LanDiscoverySourceKind;
use ocentra_parent_agent_protocol::LanDiscoverySourceMatrix;
use ocentra_parent_agent_protocol::LanDiscoverySourceRow;
use ocentra_parent_agent_protocol::LanDiscoverySourceRuntimePath;
use ocentra_parent_agent_protocol::LanDiscoverySourceStatus;
use ocentra_parent_agent_protocol::LanDiscoverySourceUiSurface;
use ocentra_parent_agent_protocol::LanPairingProductionDiscoveryState;
use ocentra_parent_agent_protocol::LanPlanWorkpackId;
use ocentra_parent_agent_protocol::LanPlanWorkpackStatusRow;
use ocentra_parent_agent_protocol::V09ProductionDiscoveryHouseholdProofState;
use ocentra_parent_agent_protocol::V09ProductionDiscoveryHouseholdRuntimeOwner;
use ocentra_parent_agent_protocol::LAN_PAIRING_SCHEMA_VERSION;

pub(super) fn source_matrix_fixture() -> LanDiscoverySourceMatrix {
    LanDiscoverySourceMatrix {
        schema_version: LAN_PAIRING_SCHEMA_VERSION,
        generated_at: constants::lan_pairing::OBSERVED_AT.to_string(),
        workpack_rows: workpack_rows(),
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

pub(super) fn assert_source_matrix_json(value: &serde_json::Value) {
    assert_eq!(
        value[constants::lan_pairing::LAN_SOURCE_MATRIX_FIELD_WORKPACK_ROWS]
            .as_array()
            .unwrap_or_else(|| unreachable!(
                "{}",
                constants::value::LAN_READ_MODEL_JSON_EXPECTATION
            ))
            .len(),
        25
    );
    assert_eq!(
        value[constants::lan_pairing::LAN_SOURCE_MATRIX_FIELD_WORKPACK_ROWS][17]
            [constants::lan_pairing::LAN_SOURCE_MATRIX_FIELD_WORKPACK_ID],
        serde_json::json!(constants::lan_pairing::LAN_SOURCE_MATRIX_WORKPACK_ID_SIGNED_CHILD_HELLO)
    );
    assert_eq!(
        value[constants::lan_pairing::LAN_SOURCE_MATRIX_FIELD_SOURCE_ROWS][0]
            [constants::lan_pairing::LAN_SOURCE_MATRIX_FIELD_SOURCE],
        serde_json::json!("windows-neighbor-table")
    );
    assert_eq!(
        value[constants::lan_pairing::LAN_SOURCE_MATRIX_FIELD_SOURCE_ROWS][0]
            [constants::lan_pairing::LAN_SOURCE_MATRIX_FIELD_CAN_CONFIRM],
        serde_json::json!(false)
    );
    assert!(
        value[constants::lan_pairing::LAN_SOURCE_MATRIX_FIELD_SOURCE_ROWS]
            .as_array()
            .unwrap_or_else(|| unreachable!(
                "{}",
                constants::value::LAN_READ_MODEL_JSON_EXPECTATION
            ))
            .iter()
            .any(|row| {
                row[constants::lan_pairing::LAN_SOURCE_MATRIX_FIELD_SOURCE]
                    == serde_json::json!("previous-scan-snapshot")
                    && row[constants::lan_pairing::LAN_SOURCE_MATRIX_FIELD_WORKPACK_ID]
                        == serde_json::json!("15")
                    && row["persistsAcrossRestart"] == serde_json::json!(true)
            })
    );
}

fn workpack_rows() -> Vec<LanPlanWorkpackStatusRow> {
    let mut rows = Vec::new();
    rows.extend(workpack_rows_01_to_10());
    rows.extend(workpack_rows_11_to_25());
    rows
}

fn workpack_rows_01_to_10() -> Vec<LanPlanWorkpackStatusRow> {
    vec![
        workpack(
            LanPlanWorkpackId::W01,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_01,
        ),
        workpack(
            LanPlanWorkpackId::W02,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_02,
        ),
        workpack(
            LanPlanWorkpackId::W03,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_03,
        ),
        workpack(
            LanPlanWorkpackId::W04,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_04,
        ),
        workpack(
            LanPlanWorkpackId::W05,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_05,
        ),
        workpack(
            LanPlanWorkpackId::W06,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_06,
        ),
        workpack(
            LanPlanWorkpackId::W07,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_07,
        ),
        workpack(
            LanPlanWorkpackId::W08,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_08,
        ),
        workpack(
            LanPlanWorkpackId::W09,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_09,
        ),
        workpack(
            LanPlanWorkpackId::W10,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_10,
        ),
    ]
}

fn workpack_rows_11_to_25() -> Vec<LanPlanWorkpackStatusRow> {
    vec![
        workpack(
            LanPlanWorkpackId::W11,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_11,
        ),
        workpack(
            LanPlanWorkpackId::W12,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_12,
        ),
        workpack(
            LanPlanWorkpackId::W13,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_13,
        ),
        workpack(
            LanPlanWorkpackId::W14,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_14,
        ),
        workpack(
            LanPlanWorkpackId::W15,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_15,
        ),
        workpack(
            LanPlanWorkpackId::W16,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_16,
        ),
        workpack(
            LanPlanWorkpackId::W17,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_17,
        ),
        workpack(
            LanPlanWorkpackId::W18,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_18,
        ),
        workpack(
            LanPlanWorkpackId::W19,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_19,
        ),
        workpack(
            LanPlanWorkpackId::W20,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_20,
        ),
        workpack(
            LanPlanWorkpackId::W21,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_21,
        ),
        workpack(
            LanPlanWorkpackId::W22,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_22,
        ),
        workpack(
            LanPlanWorkpackId::W23,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_23,
        ),
        workpack(
            LanPlanWorkpackId::W24,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_24,
        ),
        workpack(
            LanPlanWorkpackId::W25,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_25,
        ),
    ]
}

fn source_rows() -> Vec<LanDiscoverySourceRow> {
    vec![
        source(
            LanDiscoverySourceKind::WindowsNeighborTable,
            LanPlanWorkpackId::W04,
            false,
            None,
        ),
        implemented_source(
            LanDiscoverySourceKind::PreviousScanSnapshot,
            LanPlanWorkpackId::W15,
            constants::lan_pairing::LAN_SCAN_SOURCE_PREVIOUS_SCAN_SNAPSHOT,
        ),
        partial_weak_identity_source(
            LanDiscoverySourceKind::MdnsDnsSdQuery,
            LanPlanWorkpackId::W08,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_08,
        ),
        partial_weak_identity_source(
            LanDiscoverySourceKind::SsdpUpnpQuery,
            LanPlanWorkpackId::W09,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_09,
        ),
        source(
            LanDiscoverySourceKind::SignedChildAgentHello,
            LanPlanWorkpackId::W18,
            true,
            Some(constants::lan_pairing::LAN_SOURCE_MATRIX_ARTIFACT_SIGNED_CHILD),
        ),
        source(
            LanDiscoverySourceKind::SignedChildAgentHeartbeat,
            LanPlanWorkpackId::W18,
            true,
            Some(constants::lan_pairing::LAN_SOURCE_MATRIX_ARTIFACT_SIGNED_CHILD),
        ),
    ]
}

fn workpack(workpack_id: LanPlanWorkpackId, title: &str) -> LanPlanWorkpackStatusRow {
    LanPlanWorkpackStatusRow {
        schema_version: LAN_PAIRING_SCHEMA_VERSION,
        workpack_id,
        title: title.to_string(),
        discovery_state: LanPairingProductionDiscoveryState::Pending,
        proof_state: V09ProductionDiscoveryHouseholdProofState::CiMechanicalProof,
        runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner::RustServiceReadModel,
        status: LanDiscoverySourceStatus::Partial,
        read_model_visible: true,
        required_artifact_summary: None,
    }
}

fn implemented_source(
    source: LanDiscoverySourceKind,
    workpack_id: LanPlanWorkpackId,
    evidence_label: &str,
) -> LanDiscoverySourceRow {
    LanDiscoverySourceRow {
        schema_version: LAN_PAIRING_SCHEMA_VERSION,
        source,
        workpack_id,
        status: LanDiscoverySourceStatus::Implemented,
        authority: LanDiscoverySourceAuthority::WeakIdentity,
        runtime_path: LanDiscoverySourceRuntimePath::RustServiceReadModel,
        ui_surface: LanDiscoverySourceUiSurface::DevicesLan,
        can_confirm_child_agent: false,
        can_assign_child_profile: false,
        can_control_route: false,
        requires_selected_interface: false,
        persists_across_restart: true,
        evidence_label: evidence_label.to_string(),
        required_artifact_summary: None,
    }
}

fn source(
    source: LanDiscoverySourceKind,
    workpack_id: LanPlanWorkpackId,
    can_confirm_child_agent: bool,
    required_artifact_summary: Option<&str>,
) -> LanDiscoverySourceRow {
    LanDiscoverySourceRow {
        schema_version: LAN_PAIRING_SCHEMA_VERSION,
        source,
        workpack_id,
        status: LanDiscoverySourceStatus::ManualRequired,
        authority: if can_confirm_child_agent {
            LanDiscoverySourceAuthority::StrongIdentity
        } else {
            LanDiscoverySourceAuthority::NameOnly
        },
        runtime_path: LanDiscoverySourceRuntimePath::ManualArtifact,
        ui_surface: LanDiscoverySourceUiSurface::ProofReport,
        can_confirm_child_agent,
        can_assign_child_profile: false,
        can_control_route: can_confirm_child_agent,
        requires_selected_interface: true,
        persists_across_restart: can_confirm_child_agent,
        evidence_label: constants::lan_pairing::LAN_SOURCE_MATRIX_CLAIM_WEAK_SOURCES.to_string(),
        required_artifact_summary: required_artifact_summary.map(str::to_string),
    }
}

fn partial_weak_identity_source(
    source: LanDiscoverySourceKind,
    workpack_id: LanPlanWorkpackId,
    evidence_label: &str,
) -> LanDiscoverySourceRow {
    LanDiscoverySourceRow {
        schema_version: LAN_PAIRING_SCHEMA_VERSION,
        source,
        workpack_id,
        status: LanDiscoverySourceStatus::Partial,
        authority: LanDiscoverySourceAuthority::PresenceOnly,
        runtime_path: LanDiscoverySourceRuntimePath::RustServiceReadModel,
        ui_surface: LanDiscoverySourceUiSurface::DevicesLan,
        can_confirm_child_agent: false,
        can_assign_child_profile: false,
        can_control_route: false,
        requires_selected_interface: false,
        persists_across_restart: false,
        evidence_label: evidence_label.to_string(),
        required_artifact_summary: None,
    }
}

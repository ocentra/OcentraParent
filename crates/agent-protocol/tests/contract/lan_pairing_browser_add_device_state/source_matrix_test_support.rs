use ocentra_eventing::expect_value::ExpectValue;
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

macro_rules! workpack_row {
    ($workpack_id:expr, $title:expr) => {
        LanPlanWorkpackStatusRow {
            schema_version: LAN_PAIRING_SCHEMA_VERSION,
            workpack_id: $workpack_id,
            title: $title.to_string(),
            discovery_state: LanPairingProductionDiscoveryState::Pending,
            proof_state: V09ProductionDiscoveryHouseholdProofState::CiMechanicalProof,
            runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner::RustServiceReadModel,
            status: LanDiscoverySourceStatus::Partial,
            read_model_visible: true,
            required_artifact_summary: None,
        }
    };
}

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
    let workpack_rows = value[constants::lan_pairing::LAN_SOURCE_MATRIX_FIELD_WORKPACK_ROWS]
        .as_array()
        .expect_value(constants::value::LAN_READ_MODEL_JSON_EXPECTATION);
    let source_rows = value[constants::lan_pairing::LAN_SOURCE_MATRIX_FIELD_SOURCE_ROWS]
        .as_array()
        .expect_value(constants::value::LAN_READ_MODEL_JSON_EXPECTATION);

    assert_eq!(workpack_rows.len(), 25);
    for workpack_id in 1..=25 {
        assert!(
            workpack_rows.iter().any(|row| {
                row[constants::lan_pairing::LAN_SOURCE_MATRIX_FIELD_WORKPACK_ID]
                    == serde_json::json!(format!("{workpack_id:02}"))
            }),
            "missing LAN plan workpack {workpack_id:02}"
        );
    }
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
    assert!(source_rows.iter().any(|row| {
        row[constants::lan_pairing::LAN_SOURCE_MATRIX_FIELD_SOURCE]
            == serde_json::json!("previous-scan-snapshot")
            && row[constants::lan_pairing::LAN_SOURCE_MATRIX_FIELD_WORKPACK_ID]
                == serde_json::json!("15")
            && row["persistsAcrossRestart"] == serde_json::json!(true)
    }));
    for weak_source in [
        "windows-neighbor-table",
        "previous-scan-snapshot",
        "mdns-dns-sd-query",
        "ssdp-upnp-query",
    ] {
        assert!(
            source_rows.iter().any(|row| {
                row[constants::lan_pairing::LAN_SOURCE_MATRIX_FIELD_SOURCE]
                    == serde_json::json!(weak_source)
                    && row[constants::lan_pairing::LAN_SOURCE_MATRIX_FIELD_CAN_CONFIRM]
                        == serde_json::json!(false)
                    && row["canAssignChildProfile"] == serde_json::json!(false)
            }),
            "weak LAN source must not confirm child-agent identity or assign child profiles: {weak_source}"
        );
    }
    for signed_source in ["signed-child-agent-hello", "signed-child-agent-heartbeat"] {
        assert!(
            source_rows.iter().any(|row| {
                row[constants::lan_pairing::LAN_SOURCE_MATRIX_FIELD_SOURCE]
                    == serde_json::json!(signed_source)
                    && row[constants::lan_pairing::LAN_SOURCE_MATRIX_FIELD_CAN_CONFIRM]
                        == serde_json::json!(true)
                    && row["requiredArtifactSummary"].is_string()
            }),
            "signed LAN source must require an artifact and explicitly confirm child-agent identity: {signed_source}"
        );
    }
    for non_claim in [
        constants::lan_pairing::LAN_SOURCE_MATRIX_NON_CLAIM_PACKET_MODE,
        constants::lan_pairing::LAN_SOURCE_MATRIX_NON_CLAIM_PHYSICAL,
        constants::lan_pairing::LAN_SOURCE_MATRIX_NON_CLAIM_MDNS_SSDP,
    ] {
        assert!(
            value["claimsNotProved"]
                .as_array()
                .expect_value(constants::value::LAN_READ_MODEL_JSON_EXPECTATION)
                .iter()
                .any(|claim| claim == non_claim),
            "missing LAN source matrix non-claim: {non_claim}"
        );
    }
}

fn workpack_rows() -> Vec<LanPlanWorkpackStatusRow> {
    let mut rows = Vec::new();
    rows.extend(workpack_rows_01_to_10());
    rows.extend(workpack_rows_11_to_25());
    rows
}

fn workpack_rows_01_to_10() -> Vec<LanPlanWorkpackStatusRow> {
    vec![
        workpack_row!(
            LanPlanWorkpackId::W01,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_01
        ),
        workpack_row!(
            LanPlanWorkpackId::W02,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_02
        ),
        workpack_row!(
            LanPlanWorkpackId::W03,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_03
        ),
        workpack_row!(
            LanPlanWorkpackId::W04,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_04
        ),
        workpack_row!(
            LanPlanWorkpackId::W05,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_05
        ),
        workpack_row!(
            LanPlanWorkpackId::W06,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_06
        ),
        workpack_row!(
            LanPlanWorkpackId::W07,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_07
        ),
        workpack_row!(
            LanPlanWorkpackId::W08,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_08
        ),
        workpack_row!(
            LanPlanWorkpackId::W09,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_09
        ),
        workpack_row!(
            LanPlanWorkpackId::W10,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_10
        ),
    ]
}

fn workpack_rows_11_to_25() -> Vec<LanPlanWorkpackStatusRow> {
    vec![
        workpack_row!(
            LanPlanWorkpackId::W11,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_11
        ),
        workpack_row!(
            LanPlanWorkpackId::W12,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_12
        ),
        workpack_row!(
            LanPlanWorkpackId::W13,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_13
        ),
        workpack_row!(
            LanPlanWorkpackId::W14,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_14
        ),
        workpack_row!(
            LanPlanWorkpackId::W15,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_15
        ),
        workpack_row!(
            LanPlanWorkpackId::W16,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_16
        ),
        workpack_row!(
            LanPlanWorkpackId::W17,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_17
        ),
        workpack_row!(
            LanPlanWorkpackId::W18,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_18
        ),
        workpack_row!(
            LanPlanWorkpackId::W19,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_19
        ),
        workpack_row!(
            LanPlanWorkpackId::W20,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_20
        ),
        workpack_row!(
            LanPlanWorkpackId::W21,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_21
        ),
        workpack_row!(
            LanPlanWorkpackId::W22,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_22
        ),
        workpack_row!(
            LanPlanWorkpackId::W23,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_23
        ),
        workpack_row!(
            LanPlanWorkpackId::W24,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_24
        ),
        workpack_row!(
            LanPlanWorkpackId::W25,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_25
        ),
    ]
}

fn source_rows() -> Vec<LanDiscoverySourceRow> {
    vec![
        LanDiscoverySourceRow {
            schema_version: LAN_PAIRING_SCHEMA_VERSION,
            source: LanDiscoverySourceKind::WindowsNeighborTable,
            workpack_id: LanPlanWorkpackId::W04,
            status: LanDiscoverySourceStatus::ManualRequired,
            authority: LanDiscoverySourceAuthority::NameOnly,
            runtime_path: LanDiscoverySourceRuntimePath::ManualArtifact,
            ui_surface: LanDiscoverySourceUiSurface::ProofReport,
            can_confirm_child_agent: false,
            can_assign_child_profile: false,
            can_control_route: false,
            requires_selected_interface: true,
            persists_across_restart: false,
            evidence_label: constants::lan_pairing::LAN_SOURCE_MATRIX_CLAIM_WEAK_SOURCES
                .to_string(),
            required_artifact_summary: None,
        },
        LanDiscoverySourceRow {
            schema_version: LAN_PAIRING_SCHEMA_VERSION,
            source: LanDiscoverySourceKind::PreviousScanSnapshot,
            workpack_id: LanPlanWorkpackId::W15,
            status: LanDiscoverySourceStatus::Implemented,
            authority: LanDiscoverySourceAuthority::WeakIdentity,
            runtime_path: LanDiscoverySourceRuntimePath::RustServiceReadModel,
            ui_surface: LanDiscoverySourceUiSurface::DevicesLan,
            can_confirm_child_agent: false,
            can_assign_child_profile: false,
            can_control_route: false,
            requires_selected_interface: false,
            persists_across_restart: true,
            evidence_label: constants::lan_pairing::LAN_SCAN_SOURCE_PREVIOUS_SCAN_SNAPSHOT
                .to_string(),
            required_artifact_summary: None,
        },
        LanDiscoverySourceRow {
            schema_version: LAN_PAIRING_SCHEMA_VERSION,
            source: LanDiscoverySourceKind::MdnsDnsSdQuery,
            workpack_id: LanPlanWorkpackId::W08,
            status: LanDiscoverySourceStatus::Partial,
            authority: LanDiscoverySourceAuthority::PresenceOnly,
            runtime_path: LanDiscoverySourceRuntimePath::RustServiceReadModel,
            ui_surface: LanDiscoverySourceUiSurface::DevicesLan,
            can_confirm_child_agent: false,
            can_assign_child_profile: false,
            can_control_route: false,
            requires_selected_interface: false,
            persists_across_restart: false,
            evidence_label: constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_08.to_string(),
            required_artifact_summary: None,
        },
        LanDiscoverySourceRow {
            schema_version: LAN_PAIRING_SCHEMA_VERSION,
            source: LanDiscoverySourceKind::SsdpUpnpQuery,
            workpack_id: LanPlanWorkpackId::W09,
            status: LanDiscoverySourceStatus::Partial,
            authority: LanDiscoverySourceAuthority::PresenceOnly,
            runtime_path: LanDiscoverySourceRuntimePath::RustServiceReadModel,
            ui_surface: LanDiscoverySourceUiSurface::DevicesLan,
            can_confirm_child_agent: false,
            can_assign_child_profile: false,
            can_control_route: false,
            requires_selected_interface: false,
            persists_across_restart: false,
            evidence_label: constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_09.to_string(),
            required_artifact_summary: None,
        },
        LanDiscoverySourceRow {
            schema_version: LAN_PAIRING_SCHEMA_VERSION,
            source: LanDiscoverySourceKind::SignedChildAgentHello,
            workpack_id: LanPlanWorkpackId::W18,
            status: LanDiscoverySourceStatus::ManualRequired,
            authority: LanDiscoverySourceAuthority::StrongIdentity,
            runtime_path: LanDiscoverySourceRuntimePath::ManualArtifact,
            ui_surface: LanDiscoverySourceUiSurface::ProofReport,
            can_confirm_child_agent: true,
            can_assign_child_profile: false,
            can_control_route: true,
            requires_selected_interface: true,
            persists_across_restart: true,
            evidence_label: constants::lan_pairing::LAN_SOURCE_MATRIX_CLAIM_WEAK_SOURCES
                .to_string(),
            required_artifact_summary: Some(
                constants::lan_pairing::LAN_SOURCE_MATRIX_ARTIFACT_SIGNED_CHILD.to_string(),
            ),
        },
        LanDiscoverySourceRow {
            schema_version: LAN_PAIRING_SCHEMA_VERSION,
            source: LanDiscoverySourceKind::SignedChildAgentHeartbeat,
            workpack_id: LanPlanWorkpackId::W18,
            status: LanDiscoverySourceStatus::ManualRequired,
            authority: LanDiscoverySourceAuthority::StrongIdentity,
            runtime_path: LanDiscoverySourceRuntimePath::ManualArtifact,
            ui_surface: LanDiscoverySourceUiSurface::ProofReport,
            can_confirm_child_agent: true,
            can_assign_child_profile: false,
            can_control_route: true,
            requires_selected_interface: true,
            persists_across_restart: true,
            evidence_label: constants::lan_pairing::LAN_SOURCE_MATRIX_CLAIM_WEAK_SOURCES
                .to_string(),
            required_artifact_summary: Some(
                constants::lan_pairing::LAN_SOURCE_MATRIX_ARTIFACT_SIGNED_CHILD.to_string(),
            ),
        },
    ]
}

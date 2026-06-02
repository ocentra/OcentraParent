use ocentra_parent_agent_protocol::{
    constants, LanBrowserAddDeviceScanSummary, LanDiscoverySourceMatrix, LanDiscoverySourceStatus,
    LanPairingProductionDiscoveryState, LanPlanWorkpackId, LanPlanWorkpackStatusRow,
    V09ProductionDiscoveryHouseholdProofState, V09ProductionDiscoveryHouseholdRuntimeOwner,
};

mod source_rows;
use source_rows::source_rows;

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
                proof_state: V09ProductionDiscoveryHouseholdProofState::CiMechanicalProof,
                runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner::RustServiceReadModel,
                status: LanDiscoverySourceStatus::Partial,
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

fn ci_workpack(workpack_id: LanPlanWorkpackId, title: &str) -> LanPlanWorkpackStatusRow {
    workpack(
        workpack_id,
        title,
        WorkpackDetails {
            discovery_state: LanPairingProductionDiscoveryState::Discovered,
            proof_state: V09ProductionDiscoveryHouseholdProofState::CiMechanicalProof,
            runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner::RustServiceReadModel,
            status: LanDiscoverySourceStatus::Implemented,
            read_model_visible: true,
            required_artifact_summary: None,
        },
    )
}

fn partial_workpack(workpack_id: LanPlanWorkpackId, title: &str) -> LanPlanWorkpackStatusRow {
    workpack(
        workpack_id,
        title,
        WorkpackDetails {
            discovery_state: LanPairingProductionDiscoveryState::Pending,
            proof_state: V09ProductionDiscoveryHouseholdProofState::CiMechanicalProof,
            runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner::RustServiceReadModel,
            status: LanDiscoverySourceStatus::Partial,
            read_model_visible: true,
            required_artifact_summary: None,
        },
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
        WorkpackDetails {
            discovery_state: LanPairingProductionDiscoveryState::ManualRequired,
            proof_state: V09ProductionDiscoveryHouseholdProofState::ManualRequired,
            runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner::ManualProof,
            status: LanDiscoverySourceStatus::ManualRequired,
            read_model_visible: true,
            required_artifact_summary: Some(artifact.to_string()),
        },
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
        WorkpackDetails {
            discovery_state: LanPairingProductionDiscoveryState::Unavailable,
            proof_state: V09ProductionDiscoveryHouseholdProofState::NotImplemented,
            runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner::ManualProof,
            status: LanDiscoverySourceStatus::NotImplemented,
            read_model_visible: true,
            required_artifact_summary: Some(artifact.to_string()),
        },
    )
}

struct WorkpackDetails {
    discovery_state: LanPairingProductionDiscoveryState,
    proof_state: V09ProductionDiscoveryHouseholdProofState,
    runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner,
    status: LanDiscoverySourceStatus,
    read_model_visible: bool,
    required_artifact_summary: Option<String>,
}

fn workpack(
    workpack_id: LanPlanWorkpackId,
    title: &str,
    details: WorkpackDetails,
) -> LanPlanWorkpackStatusRow {
    LanPlanWorkpackStatusRow {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        workpack_id,
        title: title.to_string(),
        discovery_state: details.discovery_state,
        proof_state: details.proof_state,
        runtime_owner: details.runtime_owner,
        status: details.status,
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

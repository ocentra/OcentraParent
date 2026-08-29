use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingProductionDiscoveryState, V09ProductionDiscoveryHouseholdProofState,
    V09ProductionDiscoveryHouseholdRuntimeOwner,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::{
    LanDiscoverySourceStatus, LanPlanWorkpackId, LanPlanWorkpackStatusRow,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceScanSummary;

pub(super) fn workpack_rows(
    scan_summary: &LanBrowserAddDeviceScanSummary,
) -> Vec<LanPlanWorkpackStatusRow> {
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
        ci_workpack(
            LanPlanWorkpackId::W02,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_02,
        ),
        ci_workpack(
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
                source_status: LanDiscoverySourceStatus::Partial,
                read_model_visible: true,
                required_artifact_summary: None,
            },
        ),
    ]
}

fn packet_boundary_workpack_rows() -> Vec<LanPlanWorkpackStatusRow> {
    vec![
        implemented_workpack(
            LanPlanWorkpackId::W05,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_05,
        ),
        implemented_workpack(
            LanPlanWorkpackId::W06,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_06,
        ),
        partial_workpack_with_artifact(
            LanPlanWorkpackId::W07,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_07,
            constants::lan_pairing::LAN_SOURCE_MATRIX_ARTIFACT_PACKET_MODE,
        ),
    ]
}

fn enrichment_workpack_rows() -> Vec<LanPlanWorkpackStatusRow> {
    vec![
        partial_workpack(
            LanPlanWorkpackId::W08,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_08,
        ),
        partial_workpack(
            LanPlanWorkpackId::W09,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_09,
        ),
        partial_workpack(
            LanPlanWorkpackId::W10,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_10,
        ),
        partial_workpack(
            LanPlanWorkpackId::W11,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_11,
        ),
        ci_workpack(
            LanPlanWorkpackId::W12,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_12,
        ),
        ci_workpack(
            LanPlanWorkpackId::W13,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_13,
        ),
        ci_workpack(
            LanPlanWorkpackId::W14,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_14,
        ),
        ci_workpack(
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
        partial_workpack_with_artifact(
            LanPlanWorkpackId::W17,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_17,
            constants::lan_pairing::LAN_SOURCE_MATRIX_ARTIFACT_MDNS_SSDP,
        ),
        manual_workpack(
            LanPlanWorkpackId::W18,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_18,
            constants::lan_pairing::LAN_SOURCE_MATRIX_ARTIFACT_SIGNED_CHILD,
        ),
        ci_workpack(
            LanPlanWorkpackId::W19,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_19,
        ),
        partial_workpack(
            LanPlanWorkpackId::W20,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_20,
        ),
        ci_workpack(
            LanPlanWorkpackId::W21,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_21,
        ),
        ci_workpack(
            LanPlanWorkpackId::W22,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_22,
        ),
        partial_workpack_with_artifact(
            LanPlanWorkpackId::W23,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_23,
            constants::lan_pairing::LAN_SOURCE_MATRIX_ARTIFACT_PHYSICAL,
        ),
        ci_workpack(
            LanPlanWorkpackId::W24,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_24,
        ),
        partial_workpack_with_artifact(
            LanPlanWorkpackId::W25,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_25,
            constants::lan_pairing::LAN_SOURCE_MATRIX_ARTIFACT_PHYSICAL,
        ),
    ]
}

fn ci_workpack(workpack_id: LanPlanWorkpackId, workpack_title: &str) -> LanPlanWorkpackStatusRow {
    workpack(
        workpack_id,
        workpack_title,
        WorkpackDetails {
            discovery_state: LanPairingProductionDiscoveryState::Discovered,
            proof_state: V09ProductionDiscoveryHouseholdProofState::CiMechanicalProof,
            runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner::RustServiceReadModel,
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
            proof_state: V09ProductionDiscoveryHouseholdProofState::CiMechanicalProof,
            runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner::RustServiceReadModel,
            source_status: LanDiscoverySourceStatus::Partial,
            read_model_visible: true,
            required_artifact_summary: None,
        },
    )
}

fn unavailable_workpack(
    workpack_id: LanPlanWorkpackId,
    workpack_title: &str,
) -> LanPlanWorkpackStatusRow {
    workpack(
        workpack_id,
        workpack_title,
        WorkpackDetails {
            discovery_state: LanPairingProductionDiscoveryState::Unavailable,
            proof_state: V09ProductionDiscoveryHouseholdProofState::NotImplemented,
            runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner::RustServiceReadModel,
            source_status: LanDiscoverySourceStatus::NotImplemented,
            read_model_visible: true,
            required_artifact_summary: None,
        },
    )
}

fn implemented_workpack(
    workpack_id: LanPlanWorkpackId,
    workpack_title: &str,
) -> LanPlanWorkpackStatusRow {
    workpack(
        workpack_id,
        workpack_title,
        WorkpackDetails {
            discovery_state: LanPairingProductionDiscoveryState::Discovered,
            proof_state: V09ProductionDiscoveryHouseholdProofState::CiMechanicalProof,
            runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner::RustServiceReadModel,
            source_status: LanDiscoverySourceStatus::Implemented,
            read_model_visible: true,
            required_artifact_summary: None,
        },
    )
}

fn partial_workpack_with_artifact(
    workpack_id: LanPlanWorkpackId,
    workpack_title: &str,
    artifact: &str,
) -> LanPlanWorkpackStatusRow {
    workpack(
        workpack_id,
        workpack_title,
        WorkpackDetails {
            discovery_state: LanPairingProductionDiscoveryState::Pending,
            proof_state: V09ProductionDiscoveryHouseholdProofState::CiMechanicalProof,
            runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner::RustServiceReadModel,
            source_status: LanDiscoverySourceStatus::Partial,
            read_model_visible: true,
            required_artifact_summary: Some(artifact.to_string()),
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
            proof_state: V09ProductionDiscoveryHouseholdProofState::ManualRequired,
            runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner::ManualProof,
            source_status: LanDiscoverySourceStatus::ManualRequired,
            read_model_visible: true,
            required_artifact_summary: Some(artifact.to_string()),
        },
    )
}

struct WorkpackDetails {
    discovery_state: LanPairingProductionDiscoveryState,
    proof_state: V09ProductionDiscoveryHouseholdProofState,
    runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner,
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

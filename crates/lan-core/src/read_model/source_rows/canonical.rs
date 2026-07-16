use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::{
    LanDiscoverySourceAuthority, LanDiscoverySourceKind, LanDiscoverySourceRow,
    LanDiscoverySourceRuntimePath, LanDiscoverySourceStatus, LanDiscoverySourceUiSurface,
    LanPlanWorkpackId,
};

use super::{source_row, SourceRowDetails};

pub(super) fn canonical_spine_source_rows() -> Vec<LanDiscoverySourceRow> {
    vec![
        merge_deduplication_source(),
        explainable_classification_source(),
        household_device_store_source(),
        read_model_event_stream_source(),
        assignment_revocation_audit_source(),
        proof_gate_rollout_source(),
    ]
}

fn merge_deduplication_source() -> LanDiscoverySourceRow {
    source_row(
        LanDiscoverySourceKind::MergeDeduplication,
        LanPlanWorkpackId::W13,
        canonical_source_details(
            LanDiscoverySourceStatus::Implemented,
            LanDiscoverySourceAuthority::ProofGate,
            LanDiscoverySourceUiSurface::DevicesLan,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_13,
            None,
        ),
    )
}

fn explainable_classification_source() -> LanDiscoverySourceRow {
    source_row(
        LanDiscoverySourceKind::ExplainableClassification,
        LanPlanWorkpackId::W14,
        canonical_source_details(
            LanDiscoverySourceStatus::Implemented,
            LanDiscoverySourceAuthority::ClassificationOnly,
            LanDiscoverySourceUiSurface::DevicesLan,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_14,
            None,
        ),
    )
}

fn household_device_store_source() -> LanDiscoverySourceRow {
    let mut details = canonical_source_details(
        LanDiscoverySourceStatus::Implemented,
        LanDiscoverySourceAuthority::ManualParentDecision,
        LanDiscoverySourceUiSurface::DevicesLan,
        constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_15,
        None,
    );
    details.can_assign_child_profile = true;
    details.persists_across_restart = true;
    source_row(
        LanDiscoverySourceKind::HouseholdDeviceStore,
        LanPlanWorkpackId::W15,
        details,
    )
}

fn read_model_event_stream_source() -> LanDiscoverySourceRow {
    source_row(
        LanDiscoverySourceKind::ReadModelEventStream,
        LanPlanWorkpackId::W16,
        canonical_source_details(
            LanDiscoverySourceStatus::Implemented,
            LanDiscoverySourceAuthority::ProofGate,
            LanDiscoverySourceUiSurface::ActivityNetwork,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_16,
            None,
        ),
    )
}

fn assignment_revocation_audit_source() -> LanDiscoverySourceRow {
    let mut details = canonical_source_details(
        LanDiscoverySourceStatus::Implemented,
        LanDiscoverySourceAuthority::ManualParentDecision,
        LanDiscoverySourceUiSurface::DevicesLan,
        constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_19,
        None,
    );
    details.can_assign_child_profile = true;
    details.persists_across_restart = true;
    source_row(
        LanDiscoverySourceKind::AssignmentRevocationAudit,
        LanPlanWorkpackId::W19,
        details,
    )
}

fn proof_gate_rollout_source() -> LanDiscoverySourceRow {
    source_row(
        LanDiscoverySourceKind::ProofGateRollout,
        LanPlanWorkpackId::W20,
        canonical_source_details(
            LanDiscoverySourceStatus::Partial,
            LanDiscoverySourceAuthority::ProofGate,
            LanDiscoverySourceUiSurface::ProofReport,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_20,
            Some(constants::lan_pairing::LAN_SOURCE_MATRIX_ARTIFACT_PHYSICAL.to_string()),
        ),
    )
}

fn canonical_source_details(
    status: LanDiscoverySourceStatus,
    authority: LanDiscoverySourceAuthority,
    ui_surface: LanDiscoverySourceUiSurface,
    evidence_label: &'static str,
    required_artifact_summary: Option<String>,
) -> SourceRowDetails {
    SourceRowDetails {
        status,
        authority,
        runtime_path: LanDiscoverySourceRuntimePath::RustServiceReadModel,
        ui_surface,
        can_confirm_child_agent: false,
        can_assign_child_profile: false,
        can_control_route: false,
        requires_selected_interface: false,
        persists_across_restart: false,
        evidence_label,
        required_artifact_summary,
    }
}

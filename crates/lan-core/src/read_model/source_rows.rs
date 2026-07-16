mod canonical;
mod implemented;
mod unavailable;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::{
    LanDiscoverySourceAuthority, LanDiscoverySourceKind, LanDiscoverySourceRow,
    LanDiscoverySourceRuntimePath, LanDiscoverySourceStatus, LanDiscoverySourceUiSurface,
    LanPlanWorkpackId,
};

pub(super) fn source_rows() -> Vec<LanDiscoverySourceRow> {
    let mut rows = Vec::new();
    rows.extend(implemented::implemented_source_rows());
    rows.extend(weak_name_source_rows());
    rows.extend(unavailable::unavailable_source_rows());
    rows.extend(canonical::canonical_spine_source_rows());
    rows.extend(signed_child_source_rows());
    rows
}

pub(super) struct SourceRowDetails {
    pub(super) status: LanDiscoverySourceStatus,
    pub(super) authority: LanDiscoverySourceAuthority,
    pub(super) runtime_path: LanDiscoverySourceRuntimePath,
    pub(super) ui_surface: LanDiscoverySourceUiSurface,
    pub(super) can_confirm_child_agent: bool,
    pub(super) can_assign_child_profile: bool,
    pub(super) can_control_route: bool,
    pub(super) requires_selected_interface: bool,
    pub(super) persists_across_restart: bool,
    pub(super) evidence_label: &'static str,
    pub(super) required_artifact_summary: Option<String>,
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

pub(super) fn partial_presence_source(
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

pub(super) fn source_row(
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

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::{
    LanDiscoverySourceAuthority, LanDiscoverySourceKind, LanDiscoverySourceRow,
    LanDiscoverySourceRuntimePath, LanDiscoverySourceStatus, LanDiscoverySourceUiSurface,
    LanPlanWorkpackId,
};

use super::{partial_presence_source, source_row, SourceRowDetails};

pub(super) fn unavailable_source_rows() -> Vec<LanDiscoverySourceRow> {
    let mut rows = passive_source_rows();
    rows.push(parent_mdns_advertisement_source());
    rows.push(child_mdns_advertisement_source());
    rows
}

fn passive_source_rows() -> Vec<LanDiscoverySourceRow> {
    [
        LanDiscoverySourceKind::PassiveArpListener,
        LanDiscoverySourceKind::PassiveDhcpListener,
        LanDiscoverySourceKind::PassiveMdnsListener,
        LanDiscoverySourceKind::PassiveSsdpListener,
        LanDiscoverySourceKind::PassiveWsDiscoveryListener,
        LanDiscoverySourceKind::PassiveLlmnrListener,
        LanDiscoverySourceKind::PassiveNetbiosListener,
        LanDiscoverySourceKind::PassiveSnmpResponseListener,
    ]
    .into_iter()
    .map(|source| {
        partial_presence_source(
            source,
            LanPlanWorkpackId::W07,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_07,
        )
    })
    .collect()
}

fn parent_mdns_advertisement_source() -> LanDiscoverySourceRow {
    mdns_advertisement_source(LanDiscoverySourceKind::ParentMdnsAdvertisement)
}

fn child_mdns_advertisement_source() -> LanDiscoverySourceRow {
    mdns_advertisement_source(LanDiscoverySourceKind::ChildMdnsAdvertisement)
}

fn mdns_advertisement_source(source: LanDiscoverySourceKind) -> LanDiscoverySourceRow {
    source_row(
        source,
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

use ocentra_parent_agent_protocol::activity_surface::{
    ActivityBrowserReadModel, ActivityBrowserReadModelRow, ActivityNetworkReadModel,
    ActivityNetworkReadModelRow, ActivityReadModelState, ActivityScreenReadModel,
    ActivityScreenReadModelRow, ActivitySurfaceRequest,
};
use ocentra_parent_agent_protocol::browser_read_model::BrowserEvidenceReadModel;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::screen_evidence::ScreenEvidenceRecentSummary;
#[path = "activity_surface_read_models/app_use.rs"]
pub(crate) mod app_use;
#[path = "activity_surface_read_models/browser.rs"]
mod browser;
#[path = "activity_surface_read_models/games.rs"]
pub(crate) mod games;
#[path = "activity_surface_read_models/network.rs"]
mod network;
#[path = "activity_surface_read_models/screen.rs"]
mod screen;
#[path = "activity_surface_read_models/shared.rs"]
mod shared;

use shared::row_device_id;

pub(crate) fn screen_read_model(
    request: ActivitySurfaceRequest,
    summary: Option<ScreenEvidenceRecentSummary>,
) -> ActivityScreenReadModel {
    screen::screen_read_model(request, summary)
}

pub(crate) fn browser_read_model(
    request: ActivitySurfaceRequest,
    model: Option<BrowserEvidenceReadModel>,
) -> ActivityBrowserReadModel {
    browser::browser_read_model(request, model)
}

pub(crate) fn network_read_model(
    request: ActivitySurfaceRequest,
    model: Option<ocentra_parent_agent_protocol::network_flow::ActivityNetworkFlowReadModel>,
) -> ActivityNetworkReadModel {
    network::network_read_model(request, model)
}

pub(crate) fn activity_screen_row_from_result(
    result: ocentra_parent_agent_protocol::screen_evidence::ScreenAnalysisResult,
) -> ActivityScreenReadModelRow {
    screen::activity_screen_row_from_result(result)
}

fn browser_row(
    row: ocentra_parent_agent_protocol::browser_read_model::BrowserTabEvidence,
) -> ActivityBrowserReadModelRow {
    ActivityBrowserReadModelRow {
        row_id: row.browser_evidence_id,
        domain_label: row.domain,
        device_id: row.device_id,
        state: ActivityReadModelState::Ready,
        visit_count: 1,
        total_ms: 0,
        evidence_digest: None,
    }
}

fn network_row(
    request: &ActivitySurfaceRequest,
    row: ocentra_parent_agent_protocol::network_flow::ActivityNetworkFlowObservation,
) -> ActivityNetworkReadModelRow {
    ActivityNetworkReadModelRow {
        row_id: row.event_id,
        destination_label: row
            .destination_domain
            .or(row.destination_endpoint.ip)
            .unwrap_or_else(|| constants::activity_surface::SECTION_NETWORK.to_string()),
        device_id: row_device_id(request).0,
        state: ActivityReadModelState::Ready,
        connection_count: row.counters.connection_count,
        total_bytes: row.counters.bytes_sent.unwrap_or_default()
            + row.counters.bytes_received.unwrap_or_default(),
        evidence_digest: row
            .evidence
            .first()
            .and_then(|evidence| evidence.digest.clone()),
    }
}

use ocentra_parent_agent_protocol::{
    constants, ActivityAppUseReadModel, ActivityAppUseReadModelRow, ActivityBrowserReadModel,
    ActivityBrowserReadModelRow, ActivityGamesReadModel, ActivityGamesReadModelRow,
    ActivityNetworkReadModel, ActivityNetworkReadModelRow, ActivityReadModelState,
    ActivityRecentSummary, ActivityScreenReadModel, ActivityScreenReadModelRow,
    ActivitySurfaceRequest, AppGameSessionReport, BrowserEvidenceReadModel,
    ScreenEvidenceRecentSummary, ACTIVITY_SURFACE_SCHEMA_VERSION,
};

use crate::activity_surface_read_model_states::{
    empty_app_use_read_model, empty_games_read_model, empty_screen_read_model,
    offline_app_use_read_model, offline_browser_read_model, offline_games_read_model,
    offline_network_read_model, offline_screen_read_model, request_targets_remote_device,
    unavailable_app_use_read_model, unavailable_browser_read_model, unavailable_games_read_model,
    unavailable_network_read_model, unavailable_screen_read_model,
};
use crate::time::timestamp_now;

pub(crate) fn screen_read_model(
    request: ActivitySurfaceRequest,
    summary: Option<ScreenEvidenceRecentSummary>,
) -> ActivityScreenReadModel {
    if request_targets_remote_device(&request) {
        return offline_screen_read_model(request);
    }

    match summary {
        Some(summary) if summary.returned > 0 => ActivityScreenReadModel {
            schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
            request,
            state: ActivityReadModelState::Ready,
            generated_at: summary.generated_at,
            summary: summary
                .latest_summary
                .unwrap_or_else(|| constants::activity_surface::SUMMARY_READY.to_string()),
            rows: summary.results.into_iter().map(screen_row).collect(),
        },
        Some(summary) => empty_screen_read_model(request, summary.generated_at),
        None => unavailable_screen_read_model(request),
    }
}

pub(crate) fn app_use_read_model(
    request: ActivitySurfaceRequest,
    summary: Option<ActivityRecentSummary>,
) -> ActivityAppUseReadModel {
    if request_targets_remote_device(&request) {
        return offline_app_use_read_model(request);
    }

    match summary {
        Some(summary) if summary.returned > 0 => {
            let rows = vec![app_use_row(&request, summary)];
            ActivityAppUseReadModel {
                schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
                request,
                state: ActivityReadModelState::Ready,
                generated_at: timestamp_now(),
                summary: constants::activity_surface::SUMMARY_READY.to_string(),
                rows,
            }
        }
        Some(_) => empty_app_use_read_model(request),
        None => unavailable_app_use_read_model(request),
    }
}

pub(crate) fn browser_read_model(
    request: ActivitySurfaceRequest,
    model: Option<BrowserEvidenceReadModel>,
) -> ActivityBrowserReadModel {
    if request_targets_remote_device(&request) {
        return offline_browser_read_model(request);
    }

    match model {
        Some(model) if model.returned > 0 => ActivityBrowserReadModel {
            schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
            request,
            state: ActivityReadModelState::Ready,
            generated_at: model.generated_at,
            summary: constants::activity_surface::SUMMARY_READY.to_string(),
            rows: model.rows.into_iter().map(browser_row).collect(),
        },
        Some(model) => ActivityBrowserReadModel {
            schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
            request,
            state: ActivityReadModelState::Empty,
            generated_at: model.generated_at,
            summary: constants::activity_surface::SUMMARY_EMPTY.to_string(),
            rows: Vec::new(),
        },
        None => unavailable_browser_read_model(request),
    }
}

pub(crate) fn games_read_model(
    request: ActivitySurfaceRequest,
    report: Option<AppGameSessionReport>,
) -> ActivityGamesReadModel {
    if request_targets_remote_device(&request) {
        return offline_games_read_model(request);
    }

    match report {
        Some(report) if report.returned > 0 => {
            let rows = vec![games_row(&request, report)];
            ActivityGamesReadModel {
                schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
                request,
                state: ActivityReadModelState::Ready,
                generated_at: timestamp_now(),
                summary: constants::activity_surface::SUMMARY_READY.to_string(),
                rows,
            }
        }
        Some(_) => empty_games_read_model(request),
        None => unavailable_games_read_model(request),
    }
}

pub(crate) fn network_read_model(
    request: ActivitySurfaceRequest,
    model: Option<ocentra_parent_agent_protocol::ActivityNetworkFlowReadModel>,
) -> ActivityNetworkReadModel {
    if request_targets_remote_device(&request) {
        return offline_network_read_model(request);
    }

    match model {
        Some(model) if model.returned > 0 => {
            let rows = model
                .rows
                .into_iter()
                .map(|row| network_row(&request, row))
                .collect();
            ActivityNetworkReadModel {
                schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
                request,
                state: ActivityReadModelState::Ready,
                generated_at: model.generated_at,
                summary: constants::activity_surface::SUMMARY_READY.to_string(),
                rows,
            }
        }
        Some(model) => ActivityNetworkReadModel {
            schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
            request,
            state: ActivityReadModelState::Empty,
            generated_at: model.generated_at,
            summary: constants::activity_surface::SUMMARY_EMPTY.to_string(),
            rows: Vec::new(),
        },
        None => unavailable_network_read_model(request),
    }
}

fn screen_row(
    result: ocentra_parent_agent_protocol::ScreenAnalysisResult,
) -> ActivityScreenReadModelRow {
    ActivityScreenReadModelRow {
        row_id: result.screen_analysis_result_id,
        label: result.summary,
        device_id: constants::activity_surface::DEFAULT_DEVICE_ID.to_string(),
        state: ActivityReadModelState::Ready,
        total_ms: 0,
        foreground_ms: 0,
        background_ms: 0,
        evidence: result.source_evidence_refs,
    }
}

fn app_use_row(
    request: &ActivitySurfaceRequest,
    summary: ActivityRecentSummary,
) -> ActivityAppUseReadModelRow {
    ActivityAppUseReadModelRow {
        row_id: summary
            .last_event_id
            .unwrap_or_else(|| constants::activity_surface::READ_MODEL_APP_USE.to_string()),
        app_name: summary
            .most_recent_subject_name
            .unwrap_or_else(|| constants::activity_surface::SECTION_APP_USE.to_string()),
        device_id: row_device_id(request),
        state: ActivityReadModelState::Ready,
        total_ms: 0,
        launch_count: summary.returned,
        evidence: Vec::new(),
    }
}

fn browser_row(
    row: ocentra_parent_agent_protocol::BrowserTabEvidence,
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

fn games_row(
    request: &ActivitySurfaceRequest,
    report: AppGameSessionReport,
) -> ActivityGamesReadModelRow {
    ActivityGamesReadModelRow {
        row_id: report
            .most_recent_session_id
            .unwrap_or_else(|| constants::activity_surface::READ_MODEL_GAMES.to_string()),
        display_name: report
            .most_recent_display_name
            .unwrap_or_else(|| constants::activity_surface::SECTION_GAMES.to_string()),
        device_id: row_device_id(request),
        state: ActivityReadModelState::Ready,
        total_ms: report.most_recent_running_duration_ms.unwrap_or_default(),
        session_count: report.returned,
        evidence: Vec::new(),
    }
}

fn network_row(
    request: &ActivitySurfaceRequest,
    row: ocentra_parent_agent_protocol::ActivityNetworkFlowObservation,
) -> ActivityNetworkReadModelRow {
    ActivityNetworkReadModelRow {
        row_id: row.event_id,
        destination_label: row
            .destination_domain
            .or(row.destination_endpoint.ip)
            .unwrap_or_else(|| constants::activity_surface::SECTION_NETWORK.to_string()),
        device_id: row_device_id(request),
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

fn row_device_id(request: &ActivitySurfaceRequest) -> String {
    request
        .scope
        .device_id
        .clone()
        .unwrap_or_else(|| constants::activity_surface::DEFAULT_DEVICE_ID.to_string())
}

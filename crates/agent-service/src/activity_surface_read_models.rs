use ocentra_parent_agent_protocol::{
    constants, ActivityBrowserReadModel, ActivityBrowserReadModelRow, ActivityNetworkReadModel,
    ActivityNetworkReadModelRow, ActivityReadModelState, ActivityScreenReadModel,
    ActivityScreenReadModelRow, ActivitySurfaceRequest, BrowserEvidenceReadModel,
    ScreenEvidenceRecentSummary, ACTIVITY_SURFACE_SCHEMA_VERSION,
};

use crate::activity_surface_read_model_states::{
    empty_screen_read_model, offline_browser_read_model, offline_network_read_model,
    offline_screen_read_model, request_targets_remote_device, unavailable_browser_read_model,
    unavailable_network_read_model, unavailable_screen_read_model,
};

#[cfg(test)]
mod app_game_boundary_evidence_tests;
#[cfg(test)]
mod app_game_source_status_tests;
mod app_use;
mod games;
mod shared;

pub(crate) use app_use::app_use_read_model;
pub(crate) use games::games_read_model;
use shared::row_device_id;

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
            rows: summary
                .results
                .into_iter()
                .map(activity_screen_row_from_result)
                .collect(),
        },
        Some(summary) => empty_screen_read_model(request, summary.generated_at),
        None => unavailable_screen_read_model(request),
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

pub(crate) fn activity_screen_row_from_result(
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
        capture_reason: result.capture_reason,
        capture_scope: result.capture_scope,
        capability_status: result.capability_status,
        queue_job_id: result.queue_job_id,
        model_runtime_ref: result.model_runtime_ref,
        model_id: result.model_id,
        provider_kind: result.provider_kind,
        prompt_or_template_version: result.prompt_or_template_version,
        primary_category: result.primary_category,
        confidence: result.confidence,
        image_deletion_state: result.image_deletion_state,
        raw_image_retained: result.raw_image_retained,
        policy_eligible: result.policy_eligible,
        image_digest: result.image_digest,
        custody_state: result.custody_state,
        evidence: result.source_evidence_refs,
        policy_decision_ref: result.policy_decision_ref,
        policy_action: result.policy_action,
        policy_reason_codes: result.policy_reason_codes,
        parent_rule_refs: result.parent_rule_refs,
        local_model_runtime_refs: result.local_model_runtime_refs,
        parent_explanation_refs: result.parent_explanation_refs,
        explanation_reasons: result.explanation_reasons,
        deletion_reasons: result.deletion_reasons,
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

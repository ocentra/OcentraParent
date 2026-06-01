use ocentra_parent_agent_protocol::{
    constants, ActivityHistoricalReportList, ActivityHistoricalReportListItem,
    ActivityReadModelState, ActivityReportSourceStateSummary, ActivitySavedReportState,
    ActivitySurfaceRequest, ActivitySurfaceScope, ActivitySurfaceScopeKind,
    ParentEvidenceReferenceKind, ACTIVITY_SURFACE_SCHEMA_VERSION,
};

use crate::{
    fields::fields_from_pairs, local_ai_runtime_config::LocalAiRuntimeConfigSnapshot,
    parent_assistant_runtime::request_from_command,
};

#[test]
fn parent_assistant_request_cites_saved_activity_report_history_when_supplied() {
    let request = request_from_command(
        &history_context_command(),
        &LocalAiRuntimeConfigSnapshot::unconfigured(),
        None,
    );

    let report_context = request
        .evidence_context
        .iter()
        .find(|context| {
            context.citation_label == constants::parent_assistant::ACTIVITY_REPORT_CITATION_LABEL
        })
        .expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        report_context.evidence.evidence_reference_id,
        constants::activity_surface::REPORT_ID_DAILY
    );
    assert_eq!(
        report_context.evidence.kind,
        ParentEvidenceReferenceKind::QueryStoreSummary
    );
    assert!(report_context
        .allowed_summary
        .contains(constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_FILE_LABEL));
    assert!(report_context
        .allowed_summary
        .contains(constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_OFFLINE_SOURCE_IDS_LABEL));
    assert!(report_context
        .allowed_summary
        .contains(constants::activity_surface::FAMILY_SOURCE_OFFLINE_ID));
}

fn history_context_command() -> ocentra_parent_agent_protocol::AgentCommandEnvelope {
    super::command_with_payload(fields_from_pairs(vec![(
        constants::field::ACTIVITY_REPORTS,
        ocentra_parent_agent_protocol::LogFieldValue::String(
            serde_json::to_string(&history_list()).expect(constants::error::AGENT_EVENT_SERIALIZES),
        ),
    )]))
}

fn history_list() -> ActivityHistoricalReportList {
    let report = super::saved_report_document();
    ActivityHistoricalReportList {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        request: ActivitySurfaceRequest {
            schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
            scope: ActivitySurfaceScope {
                scope_kind: ActivitySurfaceScopeKind::Family,
                family_id: Some(constants::activity_surface::DEFAULT_FAMILY_ID.to_string()),
                device_id: None,
            },
            requested_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
            range_start: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
            range_end: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        },
        state: ActivityReadModelState::Ready,
        storage_state: ActivitySavedReportState::Saved,
        storage_reason: None,
        reports: vec![ActivityHistoricalReportListItem {
            schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
            report_id: constants::activity_surface::REPORT_ID_DAILY.to_string(),
            file_name: constants::activity_surface::REPORT_FILE_DAILY.to_string(),
            report_date: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
            range_start: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
            range_end: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
            summary: constants::activity_surface::SUMMARY_READY.to_string(),
            saved_state: ActivitySavedReportState::Saved,
            saved_at: Some(constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string()),
            source_state_summary: ActivityReportSourceStateSummary {
                total_sources: 4,
                ready_sources: 1,
                offline_sources: 1,
                stale_sources: 1,
                unavailable_sources: 1,
                unreachable_sources: 1,
                error_sources: 1,
            },
            parsed_report: report,
        }],
    }
}

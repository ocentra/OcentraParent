use std::io::Error as IoError;
use std::path::PathBuf as TestPathBuf;
use std::string::String as TestString;

use ocentra_parent_agent_protocol::activity::policy::ParentEvidenceReferenceKind;
use ocentra_parent_agent_protocol::activity_surface::ActivityHistoricalReportList;
use ocentra_parent_agent_protocol::activity_surface::ActivityHistoricalReportListItem;
use ocentra_parent_agent_protocol::activity_surface::ActivityReadModelState;
use ocentra_parent_agent_protocol::activity_surface::ActivityReportCustodyLabel;
use ocentra_parent_agent_protocol::activity_surface::ActivityReportSourceLabel;
use ocentra_parent_agent_protocol::activity_surface::ActivityReportSourceStateSummary;
use ocentra_parent_agent_protocol::activity_surface::ActivitySavedReportState;
use ocentra_parent_agent_protocol::activity_surface::ActivitySurfaceRequest;
use ocentra_parent_agent_protocol::activity_surface::ActivitySurfaceScope;
use ocentra_parent_agent_protocol::activity_surface::ActivitySurfaceScopeKind;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::ACTIVITY_SURFACE_SCHEMA_VERSION;

use crate::{
    activity_surface_report_store::save_report_document, fields::fields_from_pairs,
    local_ai_runtime_config::LocalAiRuntimeConfigSnapshot,
    parent_assistant_report_history::activity_report_history_from_command,
    parent_assistant_runtime::request_from_command,
};

#[test]
fn parent_assistant_request_cites_saved_activity_report_history_when_supplied() -> super::TestResult
{
    let command = history_context_command()?;
    let request = request_from_command(
        &command,
        &LocalAiRuntimeConfigSnapshot::unconfigured(),
        None,
        None,
    );

    let report_context = request
        .evidence_context
        .iter()
        .find(|context| {
            context.citation_label == constants::parent_assistant::ACTIVITY_REPORT_CITATION_LABEL
        })
        .ok_or_else(|| IoError::other(constants::error::AGENT_EVENT_SERIALIZES))?;

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
    assert_eq!(
        report_context.custody_label,
        constants::parent_assistant::EVIDENCE_CUSTODY_ACTIVITY_REPORT
    );
    assert_eq!(
        report_context.source_label,
        constants::parent_assistant::EVIDENCE_SOURCE_SAVED_ACTIVITY_REPORT_HISTORY
    );
    assert!(!report_context.raw_child_evidence_included);
    assert!(!report_context.direct_enforcement_allowed);

    Ok(())
}

#[tokio::test]
async fn parent_assistant_runtime_loads_saved_history_without_report_payload() -> super::TestResult
{
    let _guard = crate::activity_report_env_lock::REPORT_ENV_LOCK
        .lock()
        .await;
    let report_root = temp_report_root();
    cleanup_report_root(&report_root);
    std::env::set_var(constants::env_var::DEV_LOG_DIR, &report_root);
    save_report_document(super::saved_report_document());

    let stored_history =
        activity_report_history_from_command(&super::command_with_payload(Default::default()))
            .await
            .ok_or_else(|| IoError::other(constants::error::AGENT_EVENT_SERIALIZES))?;
    let request = request_from_command(
        &super::command_with_payload(Default::default()),
        &LocalAiRuntimeConfigSnapshot::unconfigured(),
        None,
        Some(stored_history),
    );

    std::env::remove_var(constants::env_var::DEV_LOG_DIR);
    cleanup_report_root(&report_root);

    let report_context = request
        .evidence_context
        .iter()
        .find(|context| {
            context.citation_label == constants::parent_assistant::ACTIVITY_REPORT_CITATION_LABEL
        })
        .ok_or_else(|| IoError::other(constants::error::AGENT_EVENT_SERIALIZES))?;

    assert_eq!(
        report_context.evidence.evidence_reference_id,
        constants::activity_surface::REPORT_ID_DAILY
    );
    assert!(report_context
        .allowed_summary
        .contains(constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_STORAGE_REASON_LABEL));
    assert!(report_context
        .allowed_summary
        .contains(constants::activity_surface::SUMMARY_STORAGE_SAVED));
    assert!(!report_context.raw_child_evidence_included);
    assert!(!report_context.direct_enforcement_allowed);

    Ok(())
}

fn history_context_command(
) -> Result<ocentra_parent_agent_protocol::transport::AgentCommandEnvelope, IoError> {
    let history = serde_json::to_string(&history_list()).map_err(|error| {
        IoError::other(format!(
            "{}: {error:?}",
            constants::error::AGENT_EVENT_SERIALIZES
        ))
    })?;

    Ok(super::command_with_payload(fields_from_pairs(vec![(
        constants::field::ACTIVITY_REPORTS,
        ocentra_parent_agent_protocol::logging::LogFieldValue::String(history),
    )])))
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
            custody_label: ActivityReportCustodyLabel::ParentDeviceLocalHistory,
            source_label: ActivityReportSourceLabel::SavedReportHistory,
            raw_child_evidence_included: false,
        }],
    }
}

fn temp_report_root() -> TestPathBuf {
    let mut path = std::env::temp_dir();
    let mut name = TestString::from(constants::activity_store::TEST_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(&nanos_now().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(constants::dev_log::DEFAULT_DIR);
    path.push(name);
    path
}

fn cleanup_report_root(path: &TestPathBuf) {
    let _ = std::fs::remove_dir_all(path);
}

fn nanos_now() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |duration| duration.as_nanos())
}

use std::{
    collections::BTreeMap,
    env,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use crate::test_text::TestText;
use ocentra_parent_agent_protocol::activity_surface::{
    ActivityHistoricalReportList, ActivityReadModelState, ActivityReportDocument,
    ActivityReportFrequency, ActivitySavedReportState,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields, LogLevel};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventEnvelope, AgentEventName, AgentMessageTarget,
    AgentPeer, AgentPeerRole, AgentRoute,
};
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;
use ocentra_parent_agent_service::test_support::{
    build_activity_report_document_for_test, handle_local_command_text_for_test,
    history_list_from_dir_for_test, lock_activity_report_env_for_test,
    save_activity_report_document_for_test, save_activity_report_document_to_dir_for_test,
};
use serde::Serialize;

#[tokio::test]
async fn activity_report_save_and_history_commands_round_trip_real_json_storage(
) -> Result<(), TestText> {
    let _guard = lock_activity_report_env_for_test().await;
    let report_root = temp_report_root();
    crate::test_support::cleanup_report_dir(&report_root);
    env::set_var(constants::env_var::DEV_LOG_DIR, report_root.path());
    let report = build_activity_report_document_for_test(report_request());

    let save_event = send_activity_surface_command(
        AgentCommandName::AgentActivityReportSave,
        save_payload(&report)?,
    )
    .await?;
    let history_event = send_activity_surface_command(
        AgentCommandName::AgentActivityReportHistoryList,
        surface_command_payload(),
    )
    .await?;

    env::remove_var(constants::env_var::DEV_LOG_DIR);
    crate::test_support::cleanup_report_dir(&report_root);

    let saved_report = report_from_event(&save_event)?;
    let history = history_from_event(&history_event)?;

    assert_eq!(save_event.event, AgentEventName::AgentActivityReportSaved);
    assert_eq!(save_event.severity, LogLevel::Info);
    assert_eq!(
        saved_report
            .saved_metadata
            .as_ref()
            .map(|metadata| metadata.saved_state),
        Some(ActivitySavedReportState::Saved)
    );
    assert_eq!(
        history_event.event,
        AgentEventName::AgentActivityReportHistoryReported
    );
    assert_eq!(history.reports.len(), 1);
    assert_eq!(history.storage_state, ActivitySavedReportState::Saved);
    assert_eq!(
        history.reports[0].parsed_report.report_id,
        saved_report.report_id
    );
    Ok(())
}

#[tokio::test]
async fn activity_surface_helper_modules_remain_linked_without_panics() -> Result<(), TestText> {
    let _guard = lock_activity_report_env_for_test().await;
    let report_root = temp_report_root();
    crate::test_support::cleanup_report_dir(&report_root);
    env::set_var(constants::env_var::DEV_LOG_DIR, report_root.path());

    let report = build_activity_report_document_for_test(report_request());
    activity_surface_helper_modules_are_linked_request_and_store(&report, &report_root)?;
    activity_surface_helper_modules_are_linked_payload_and_models(&report, &report_root).await?;
    activity_surface_helper_test_support_is_linked(&report_root).await;
    activity_surface_test_support_helpers_are_linked()?;
    activity_surface_time_helpers_are_linked();

    env::remove_var(constants::env_var::DEV_LOG_DIR);
    crate::test_support::cleanup_report_dir(&report_root);

    Ok(())
}

async fn activity_surface_helper_test_support_is_linked(report_root: &TempReportRoot) {
    let summary = crate::test_support::load_activity_recent_summary_from_store_path_for_test(
        report_root.path(),
    )
    .await;
    assert!(summary.is_none());
}

fn activity_surface_test_support_helpers_are_linked() -> Result<(), TestText> {
    let mut payload = LogFields::new();
    payload.insert(
        constants::field::ACTIVITY_REPORT_ID.to_string(),
        LogFieldValue::String(constants::activity_surface::DEFAULT_FAMILY_ID.to_string()),
    );

    assert_eq!(
        crate::test_invariants::require_ok(Ok::<usize, TestText>(1), "result"),
        1
    );
    assert_eq!(crate::test_invariants::require_some(Some(2), "value"), 2);
    assert_eq!(
        crate::test_invariants::require_json_decode::<usize>("3", "json"),
        3
    );
    assert_eq!(
        crate::test_invariants::require_log_string_field(
            payload.get(constants::field::ACTIVITY_REPORT_ID),
            "payload",
        ),
        constants::activity_surface::DEFAULT_FAMILY_ID
    );
    assert_eq!(
        crate::test_invariants::log_field(
            &payload,
            constants::field::ACTIVITY_REPORT_ID,
            "payload",
        ),
        LogFieldValue::String(constants::activity_surface::DEFAULT_FAMILY_ID.to_string())
    );
    assert_eq!(crate::test_invariants::serialize_test_json(&4), "4");

    let text = TestText::from_display("text");
    let mut counts = BTreeMap::new();
    let mut payload = LogFields::new();
    counts.insert(text.clone(), 1);
    payload.insert(
        constants::field::ACTIVITY_REPORT_ID.to_string(),
        LogFieldValue::String(text.to_string()),
    );

    let _: crate::test_text::TestResult = Ok(());
    assert_eq!(text.as_bytes(), b"text");
    assert_eq!(text.as_str(), "text");
    assert_eq!(
        crate::test_text::test_ok(Ok::<usize, TestText>(1), "result")?,
        1
    );
    assert_eq!(crate::test_text::test_some(Some(2), "value")?, 2);
    assert_eq!(crate::test_text::count_for_display(&counts, "text"), 1);
    assert_eq!(
        crate::test_text::optional_log_string(&payload, constants::field::ACTIVITY_REPORT_ID),
        Some(text)
    );

    Ok(())
}

fn activity_surface_time_helpers_are_linked() {
    let now: String = crate::time::timestamp_now();
    let after_epoch: String = crate::time::timestamp_after_epoch_seconds(0, 1);
    let from_epoch: String = crate::time::timestamp_from_epoch_seconds(0);

    assert!(now.ends_with('Z'));
    assert_eq!(after_epoch, "1970-01-01T00:00:01.000Z");
    assert_eq!(from_epoch, "1970-01-01T00:00:00.000Z");
}

fn activity_surface_helper_modules_are_linked_request_and_store(
    report: &ActivityReportDocument,
    report_root: &TempReportRoot,
) -> Result<(), TestText> {
    let save_command = command_envelope(
        AgentCommandName::AgentActivityReportSave,
        save_payload(report)?,
    );
    let parsed_request = crate::activity_surface_request::report_request_from_command(
        &save_command,
        ActivityReportFrequency::Daily,
    );
    let parsed_surface_request =
        crate::activity_surface_request::surface_request_from_command(&save_command);
    let parsed_document =
        crate::activity_surface_request::report_document_from_command(&save_command)
            .ok_or_else(|| TestText::from_display(constants::error::AGENT_EVENT_SERIALIZES))?;
    let saved_report =
        save_activity_report_document_to_dir_for_test(report.clone(), report_root.path());
    let saved_report_default = save_activity_report_document_for_test(report.clone());
    let saved_report_via_module =
        crate::activity_surface_report_store::save_report_document(report.clone());
    let history =
        history_list_from_dir_for_test(parsed_surface_request.clone(), report_root.path());
    let history_default =
        crate::activity_surface_report_store::history_list(parsed_surface_request.clone());
    let history_default_saved_report_count = history_default
        .reports
        .iter()
        .filter(|saved_report| saved_report.parsed_report.report_id == report.report_id)
        .count();
    let _report_metadata = crate::activity_surface_report_store::draft_metadata_for_report(report);
    let _report_path = crate::activity_store_path::activity_db_path();
    let _journal_path = crate::activity_store_path::activity_journal_path();
    let _journal_key_path = crate::activity_store_path::activity_journal_key_path();
    assert_eq!(
        parsed_request.scope.scope_kind,
        ocentra_parent_agent_protocol::activity_surface::ActivitySurfaceScopeKind::Family
    );
    assert_eq!(
        parsed_surface_request.scope.scope_kind,
        ocentra_parent_agent_protocol::activity_surface::ActivitySurfaceScopeKind::Family
    );
    assert_eq!(parsed_document.report_id, report.report_id);
    assert_eq!(
        saved_report
            .saved_metadata
            .as_ref()
            .map(|metadata| metadata.saved_state),
        Some(ActivitySavedReportState::Saved)
    );
    assert_eq!(
        saved_report_default
            .saved_metadata
            .as_ref()
            .map(|metadata| metadata.saved_state),
        Some(ActivitySavedReportState::Saved)
    );
    assert_eq!(saved_report_default.report_id, report.report_id);
    assert_eq!(saved_report_via_module.report_id, report.report_id);
    assert_eq!(history.reports.len(), 1);
    assert_eq!(history.reports[0].parsed_report.report_id, report.report_id);
    assert_eq!(history_default_saved_report_count, 1);
    assert_eq!(
        history_default.storage_state,
        ActivitySavedReportState::Saved
    );
    assert_eq!(
        history_default.reports[0].parsed_report.report_id,
        report.report_id
    );
    activity_surface_helper_modules_are_linked_family_sources()?;
    activity_surface_helper_modules_are_linked_read_models(&parsed_surface_request);

    Ok(())
}

#[tokio::test]
async fn activity_surface_helper_modules_remain_linked_without_panics_read_models(
) -> Result<(), TestText> {
    let _guard = lock_activity_report_env_for_test().await;
    let report_root = temp_report_root();
    crate::test_support::cleanup_report_dir(&report_root);
    let report = build_activity_report_document_for_test(report_request());
    activity_surface_helper_modules_are_linked_payload_and_models(&report, &report_root).await?;
    env::remove_var(constants::env_var::DEV_LOG_DIR);
    crate::test_support::cleanup_report_dir(&report_root);
    Ok(())
}

async fn activity_surface_helper_modules_are_linked_payload_and_models(
    report: &ActivityReportDocument,
    report_root: &TempReportRoot,
) -> Result<(), TestText> {
    let save_command = command_envelope(
        AgentCommandName::AgentActivityReportSave,
        save_payload(report)?,
    );
    let parsed_surface_request =
        crate::activity_surface_request::surface_request_from_command(&save_command);
    let _ = save_activity_report_document_for_test(report.clone());
    let history =
        history_list_from_dir_for_test(parsed_surface_request.clone(), report_root.path());
    let history_payload = crate::activity_surface_payload::activity_history_payload(&history);
    let report_payload = crate::activity_surface_payload::activity_report_document_payload(report);
    let screen_model = crate::activity_surface_read_models::screen_read_model(
        parsed_surface_request.clone(),
        None,
    );
    let browser_model = crate::activity_surface_read_models::browser_read_model(
        parsed_surface_request.clone(),
        None,
    );
    let network_model = crate::activity_surface_read_models::network_read_model(
        parsed_surface_request.clone(),
        None,
    );
    let screen_json = serde_json::to_string(&screen_model)
        .map_err(|error| TestText::from_display(format!("{error:?}")))?;
    let screen_payload = crate::activity_surface_payload::activity_read_model_payload(
        crate::activity_surface_payload::ReadModelKind("screen".to_string()),
        screen_model.state,
        screen_model.rows.len(),
        crate::activity_surface_payload::ReadModelJson(screen_json.clone()),
    );
    let history_json = serde_json::to_string(&history)
        .map_err(|error| TestText::from_display(format!("{error:?}")))?;
    let snapshot_none = crate::activity_surface_store::local_store_snapshot_from_path(
        activity_store_path(report_root),
    )
    .await;
    let browser_none = crate::activity_surface_store::load_browser_model_from_path(
        activity_store_path(report_root),
    )
    .await;
    let network_none = crate::activity_surface_store::load_network_model_from_path(
        activity_store_path(report_root),
    )
    .await;
    let app_game_none = crate::activity_surface_store::load_app_game_model_from_path(
        activity_store_path(report_root),
    )
    .await;
    let screen_none = crate::activity_surface_store::load_screen_summary_from_path(
        activity_store_path(report_root),
    )
    .await;
    let _local_snapshot = crate::activity_surface_store::local_store_snapshot().await;
    let _local_browser = crate::activity_surface_store::load_browser_model().await;
    let _local_network = crate::activity_surface_store::load_network_model().await;
    let _local_app_game = crate::activity_surface_store::load_app_game_model().await;
    let _local_screen = crate::activity_surface_store::load_screen_summary().await;
    let snapshot = crate::activity_surface_store::ActivitySurfaceStoreSnapshot {
        device_id: crate::activity_surface_store::ActivitySurfaceDeviceRefText(
            constants::activity_surface::DEFAULT_DEVICE_ID.to_string(),
        ),
        recent_returned: 0,
        last_event_id: None,
        last_observed_at: None,
        browser_returned: 0,
        network_returned: 0,
        games_returned: 0,
        screen_returned: 0,
    };
    let _ = crate::activity_surface_read_models::activity_screen_row_from_result;

    assert_eq!(
        history_payload.get(constants::field::ACTIVITY_REPORTS),
        Some(&LogFieldValue::String(history_json))
    );
    assert_eq!(
        report_payload.get(constants::field::ACTIVITY_REPORT_ID),
        Some(&LogFieldValue::String(report.report_id.clone()))
    );
    assert_eq!(screen_model.state, ActivityReadModelState::Unavailable);
    assert_eq!(browser_model.state, ActivityReadModelState::Unavailable);
    assert_eq!(network_model.state, ActivityReadModelState::Unavailable);
    assert_eq!(
        screen_payload.get(constants::field::ACTIVITY_READ_MODEL),
        Some(&LogFieldValue::String(screen_json))
    );
    assert!(snapshot_none.is_none());
    assert!(browser_none.is_none());
    assert!(network_none.is_none());
    assert!(app_game_none.is_none());
    assert!(screen_none.is_none());
    assert_eq!(snapshot.last_event_id, None);

    Ok(())
}

fn activity_surface_helper_modules_are_linked_family_sources() -> Result<(), TestText> {
    let family_sources_command = command_envelope(AgentCommandName::AgentActivityReportSave, {
        let mut payload = surface_command_payload();
        payload.insert(
            constants::field::ACTIVITY_FAMILY_SOURCES.to_string(),
            LogFieldValue::String(
                serialize_json(&vec![
                    crate::activity_family_sources::default_family_fanout_record(),
                    crate::activity_family_sources::family_source_error_record(),
                ])?
                .to_string(),
            ),
        );
        payload
    });
    let family_sources =
        crate::activity_family_sources::family_sources_from_command(&family_sources_command);
    let _ = crate::activity_family_sources::default_family_fanout_record();
    let _ = crate::activity_family_sources::family_source_error_record();
    assert_eq!(family_sources.len(), 2);

    Ok(())
}

fn activity_surface_helper_modules_are_linked_read_models(
    parsed_surface_request: &ocentra_parent_agent_protocol::activity_surface::ActivitySurfaceRequest,
) {
    let _ = crate::activity_surface_read_models::app_use::app_use_read_model(
        parsed_surface_request.clone(),
        Some(
            ocentra_parent_agent_protocol::activity_query::ActivityRecentSummary {
                schema_version:
                    ocentra_parent_agent_protocol::activity_query::ACTIVITY_QUERY_SCHEMA_VERSION,
                limit: 1,
                returned: 0,
                first_observed_at: None,
                last_observed_at: None,
                last_event_id: None,
                most_recent_kind: None,
                most_recent_observer: None,
                most_recent_subject_kind: None,
                most_recent_subject_id: None,
                most_recent_subject_name: None,
            },
        ),
    );
    let _ = crate::activity_surface_read_models::app_use::app_use_read_model(
        parsed_surface_request.clone(),
        None::<ocentra_parent_agent_protocol::app_game::AppGameServiceReadModel>,
    );
    let _ = crate::activity_surface_read_models::games::games_read_model(
        parsed_surface_request.clone(),
        None,
    );
}

async fn send_activity_surface_command(
    command: AgentCommandName,
    payload: LogFields,
) -> Result<AgentEventEnvelope, TestText> {
    let body = serialize_json(&command_envelope(command, payload))?;
    Ok(handle_local_command_text_for_test(body).await)
}

fn save_payload(report: &ActivityReportDocument) -> Result<LogFields, TestText> {
    let mut payload = surface_command_payload();
    payload.insert(
        constants::field::ACTIVITY_REPORT_DOCUMENT.to_string(),
        LogFieldValue::String(serialize_json(report)?.to_string()),
    );
    Ok(payload)
}

fn command_envelope(command: AgentCommandName, payload: LogFields) -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::parent_assistant::DEFAULT_MESSAGE_ID.to_string(),
        sent_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: constants::activity_surface::DEFAULT_DEVICE_ID.to_string(),
            platform:
                ocentra_parent_agent_protocol::policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS
                    .to_string(),
            route: AgentRoute::Localhost,
        },
        command,
        payload,
    }
}

fn surface_command_payload() -> LogFields {
    let mut payload = LogFields::new();
    payload.insert(
        constants::field::SCOPE_KIND.to_string(),
        LogFieldValue::String(constants::activity_surface::SCOPE_FAMILY.to_string()),
    );
    payload.insert(
        constants::field::FAMILY_ID.to_string(),
        LogFieldValue::String(constants::activity_surface::DEFAULT_FAMILY_ID.to_string()),
    );
    payload.insert(
        constants::field::REQUESTED_AT.to_string(),
        LogFieldValue::String(constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string()),
    );
    payload.insert(
        constants::field::RANGE_START.to_string(),
        LogFieldValue::String(constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string()),
    );
    payload.insert(
        constants::field::RANGE_END.to_string(),
        LogFieldValue::String(constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string()),
    );
    payload
}

fn report_request() -> ocentra_parent_agent_protocol::activity_surface::ActivityReportRequest {
    ocentra_parent_agent_protocol::activity_surface::ActivityReportRequest {
        schema_version: ocentra_parent_agent_protocol::ACTIVITY_SURFACE_SCHEMA_VERSION,
        frequency: ocentra_parent_agent_protocol::activity_surface::ActivityReportFrequency::Daily,
        scope: ocentra_parent_agent_protocol::activity_surface::ActivitySurfaceScope {
            scope_kind:
                ocentra_parent_agent_protocol::activity_surface::ActivitySurfaceScopeKind::Family,
            family_id: Some(constants::activity_surface::DEFAULT_FAMILY_ID.to_string()),
            device_id: None,
        },
        requested_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        range_start: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        range_end: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
    }
}

fn report_from_event(event: &AgentEventEnvelope) -> Result<ActivityReportDocument, TestText> {
    let payload = string_payload_field(event, constants::field::ACTIVITY_REPORT_DOCUMENT)?;
    serde_json::from_str(payload.0.as_str())
        .map_err(|error| TestText::from_display(format!("{error:?}")))
}

fn history_from_event(
    event: &AgentEventEnvelope,
) -> Result<ActivityHistoricalReportList, TestText> {
    let payload = string_payload_field(event, constants::field::ACTIVITY_REPORTS)?;
    serde_json::from_str(payload.0.as_str())
        .map_err(|error| TestText::from_display(format!("{error:?}")))
}

fn string_payload_field(
    event: &AgentEventEnvelope,
    field: impl std::fmt::Display,
) -> Result<TestText, TestText> {
    let field = field.to_string();
    match event.payload.get(field.as_str()) {
        Some(LogFieldValue::String(value)) => Ok(TestText::from_display(value.as_str())),
        _ => Err(TestText::from_display(
            constants::error::AGENT_EVENT_SERIALIZES,
        )),
    }
}

struct TempReportRoot {
    path: PathBuf,
}

impl TempReportRoot {
    fn path(&self) -> &Path {
        self.path.as_path()
    }
}

impl AsRef<Path> for TempReportRoot {
    fn as_ref(&self) -> &Path {
        self.path()
    }
}

fn activity_store_path(root: &TempReportRoot) -> crate::activity_surface_store::ActivityStorePath {
    crate::activity_surface_store::ActivityStorePath(root.as_ref().to_path_buf())
}

fn temp_report_root() -> TempReportRoot {
    let mut path = std::env::temp_dir();
    let mut name = String::from(constants::activity_store::TEST_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(&nanos_now().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(constants::dev_log::DEFAULT_DIR);
    path.push(name);
    TempReportRoot { path }
}

fn nanos_now() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_default()
}

fn serialize_json<T>(value: &T) -> Result<TestText, TestText>
where
    T: Serialize,
{
    serde_json::to_string(value)
        .map(TestText::from_display)
        .map_err(|error| {
            TestText::from_display(format!(
                "{}: {error}",
                constants::error::AGENT_EVENT_SERIALIZES
            ))
        })
}

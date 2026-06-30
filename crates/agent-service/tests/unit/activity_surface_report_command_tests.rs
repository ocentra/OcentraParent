use std::{
    env,
    error::Error,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

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
    lock_activity_report_env_for_test,
};
use serde::Serialize;

#[tokio::test]
async fn activity_report_save_and_history_commands_round_trip_real_json_storage(
) -> Result<(), Box<dyn Error>> {
    let _guard = lock_activity_report_env_for_test().await;
    let report_root = temp_report_root();
    crate::test_support::cleanup_report_dir(&report_root);
    env::set_var(constants::env_var::DEV_LOG_DIR, &report_root);
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
async fn activity_surface_helper_modules_remain_linked_without_panics() -> Result<(), Box<dyn Error>>
{
    let _guard = lock_activity_report_env_for_test().await;
    let report_root = temp_report_root();
    crate::test_support::cleanup_report_dir(&report_root);

    let report = build_activity_report_document_for_test(report_request());
    let save_command = command_envelope(
        AgentCommandName::AgentActivityReportSave,
        save_payload(&report)?,
    );
    let parsed_request = crate::activity_surface_request::report_request_from_command(
        &save_command,
        ActivityReportFrequency::Daily,
    );
    let parsed_surface_request =
        crate::activity_surface_request::surface_request_from_command(&save_command);
    let parsed_document =
        crate::activity_surface_request::report_document_from_command(&save_command)
            .ok_or_else(|| std::io::Error::other(constants::error::AGENT_EVENT_SERIALIZES))?;
    let saved_report = crate::activity_surface_report_store::save_report_document_to_dir(
        report.clone(),
        &report_root,
    );
    let saved_report_default =
        crate::activity_surface_report_store::save_report_document(report.clone());
    let history = crate::activity_surface_report_store::history_list_from_dir(
        parsed_surface_request.clone(),
        &report_root,
    );
    let history_default =
        crate::activity_surface_report_store::history_list(parsed_surface_request.clone());
    let report_payload = crate::activity_surface_payload::activity_report_document_payload(&report);
    let _report_metadata = crate::activity_surface_report_store::draft_metadata_for_report(&report);
    let _report_path = crate::activity_store_path::activity_db_path();
    let _journal_path = crate::activity_store_path::activity_journal_path();
    let _journal_key_path = crate::activity_store_path::activity_journal_key_path();
    let family_sources_command = command_envelope(AgentCommandName::AgentActivityReportSave, {
        let mut payload = surface_command_payload();
        payload.insert(
            constants::field::ACTIVITY_FAMILY_SOURCES.to_string(),
            LogFieldValue::String(serialize_json(&vec![
                crate::activity_family_sources::default_family_fanout_record(),
                crate::activity_family_sources::family_source_error_record(),
            ])?),
        );
        payload
    });
    let family_sources =
        crate::activity_family_sources::family_sources_from_command(&family_sources_command);
    let _ = crate::activity_family_sources::default_family_fanout_record();
    let _ = crate::activity_family_sources::family_source_error_record();
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
    assert_eq!(history.reports.len(), 1);
    assert_eq!(history.reports[0].parsed_report.report_id, report.report_id);
    assert!(!report_payload.is_empty());
    assert_eq!(family_sources.len(), 2);

    activity_surface_store_and_read_model_helpers_are_linked().await?;

    env::remove_var(constants::env_var::DEV_LOG_DIR);
    crate::test_support::cleanup_report_dir(&report_root);

    Ok(())
}

#[tokio::test]
async fn activity_surface_helper_modules_remain_linked_without_panics_read_models(
) -> Result<(), Box<dyn Error>> {
    activity_surface_store_and_read_model_helpers_are_linked().await
}

async fn activity_surface_store_and_read_model_helpers_are_linked() -> Result<(), Box<dyn Error>> {
    let _guard = lock_activity_report_env_for_test().await;
    let report_root = temp_report_root();
    crate::test_support::cleanup_report_dir(&report_root);

    let report = build_activity_report_document_for_test(report_request());
    let save_command = command_envelope(
        AgentCommandName::AgentActivityReportSave,
        save_payload(&report)?,
    );
    let parsed_surface_request =
        crate::activity_surface_request::surface_request_from_command(&save_command);
    let _ = crate::activity_surface_report_store::save_report_document_to_dir(
        report.clone(),
        &report_root,
    );
    let history = crate::activity_surface_report_store::history_list_from_dir(
        parsed_surface_request.clone(),
        &report_root,
    );
    let history_payload = crate::activity_surface_payload::activity_history_payload(&history);
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
    let screen_payload = crate::activity_surface_payload::activity_read_model_payload(
        "screen",
        screen_model.state,
        screen_model.rows.len(),
        serde_json::to_string(&screen_model)?,
    );
    let snapshot_none =
        crate::activity_surface_store::local_store_snapshot_from_path(report_root.clone()).await;
    let browser_none =
        crate::activity_surface_store::load_browser_model_from_path(report_root.clone()).await;
    let network_none =
        crate::activity_surface_store::load_network_model_from_path(report_root.clone()).await;
    let app_game_none =
        crate::activity_surface_store::load_app_game_model_from_path(report_root.clone()).await;
    let screen_none =
        crate::activity_surface_store::load_screen_summary_from_path(report_root.clone()).await;

    assert!(!history_payload.is_empty());
    assert_eq!(screen_model.state, ActivityReadModelState::Unavailable);
    assert_eq!(browser_model.state, ActivityReadModelState::Unavailable);
    assert_eq!(network_model.state, ActivityReadModelState::Unavailable);
    assert!(!screen_payload.is_empty());
    assert!(snapshot_none.is_none());
    assert!(browser_none.is_none());
    assert!(network_none.is_none());
    assert!(app_game_none.is_none());
    assert!(screen_none.is_none());

    crate::test_support::cleanup_report_dir(&report_root);

    Ok(())
}

async fn send_activity_surface_command(
    command: AgentCommandName,
    payload: LogFields,
) -> Result<AgentEventEnvelope, Box<dyn Error>> {
    let body = serialize_json(&command_envelope(command, payload))?;
    Ok(handle_local_command_text_for_test(&body).await)
}

fn save_payload(report: &ActivityReportDocument) -> Result<LogFields, Box<dyn Error>> {
    let mut payload = surface_command_payload();
    payload.insert(
        constants::field::ACTIVITY_REPORT_DOCUMENT.to_string(),
        LogFieldValue::String(serialize_json(report)?),
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

fn report_from_event(event: &AgentEventEnvelope) -> Result<ActivityReportDocument, Box<dyn Error>> {
    Ok(serde_json::from_str(string_payload_field(
        event,
        constants::field::ACTIVITY_REPORT_DOCUMENT,
    )?)?)
}

fn history_from_event(
    event: &AgentEventEnvelope,
) -> Result<ActivityHistoricalReportList, Box<dyn Error>> {
    Ok(serde_json::from_str(string_payload_field(
        event,
        constants::field::ACTIVITY_REPORTS,
    )?)?)
}

fn string_payload_field<'a>(
    event: &'a AgentEventEnvelope,
    field: &str,
) -> Result<&'a str, Box<dyn Error>> {
    match event.payload.get(field) {
        Some(LogFieldValue::String(value)) => Ok(value.as_str()),
        _ => Err(std::io::Error::other(constants::error::AGENT_EVENT_SERIALIZES).into()),
    }
}

fn temp_report_root() -> PathBuf {
    let mut path = std::env::temp_dir();
    let mut name = String::from(constants::activity_store::TEST_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(&nanos_now().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(constants::dev_log::DEFAULT_DIR);
    path.push(name);
    path
}

fn nanos_now() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_default()
}

fn serialize_json<T>(value: &T) -> Result<String, Box<dyn Error>>
where
    T: Serialize,
{
    Ok(serde_json::to_string(value)?)
}

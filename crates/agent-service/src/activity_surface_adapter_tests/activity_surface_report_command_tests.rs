use std::{
    env,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use ocentra_parent_agent_protocol::{
    constants, ActivityHistoricalReportList, ActivityReportDocument, ActivitySavedReportState,
    AgentCommandEnvelope, AgentCommandName, AgentEventEnvelope, AgentEventName, AgentMessageTarget,
    AgentPeer, AgentPeerRole, AgentRoute, LogFieldValue, LogFields, LogLevel,
    AGENT_PROTOCOL_SCHEMA_VERSION,
};

use crate::{
    activity_surface_api::{build_activity_report_history, build_activity_report_save},
    activity_surface_report::report_document,
};

static REPORT_ENV_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

#[tokio::test]
async fn activity_report_save_and_history_commands_round_trip_real_json_storage() {
    let _guard = REPORT_ENV_LOCK.lock().await;
    let report_root = temp_report_root();
    super::cleanup_report_dir(&report_root);
    env::set_var(constants::env_var::DEV_LOG_DIR, &report_root);
    let report = report_document(super::report_request(), None, Vec::new());

    let save_event = build_activity_report_save(save_command(report)).await;
    let history_event = build_activity_report_history(history_command()).await;

    env::remove_var(constants::env_var::DEV_LOG_DIR);
    super::cleanup_report_dir(&report_root);

    let saved_report = report_from_event(&save_event);
    let history = history_from_event(&history_event);

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
}

fn save_command(report: ActivityReportDocument) -> AgentCommandEnvelope {
    let mut payload = surface_command_payload();
    payload.insert(
        constants::field::ACTIVITY_REPORT_DOCUMENT.to_string(),
        LogFieldValue::String(
            serde_json::to_string(&report).expect(constants::error::AGENT_EVENT_SERIALIZES),
        ),
    );
    command(AgentCommandName::AgentActivityReportSave, payload)
}

fn history_command() -> AgentCommandEnvelope {
    command(
        AgentCommandName::AgentActivityReportHistoryList,
        surface_command_payload(),
    )
}

fn command(command: AgentCommandName, payload: LogFields) -> AgentCommandEnvelope {
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

fn report_from_event(event: &AgentEventEnvelope) -> ActivityReportDocument {
    let report_json = event
        .payload
        .get(constants::field::ACTIVITY_REPORT_DOCUMENT)
        .and_then(log_field_string)
        .expect(constants::error::AGENT_EVENT_SERIALIZES);
    serde_json::from_str(report_json).expect(constants::error::AGENT_EVENT_SERIALIZES)
}

fn history_from_event(event: &AgentEventEnvelope) -> ActivityHistoricalReportList {
    let history_json = event
        .payload
        .get(constants::field::ACTIVITY_REPORTS)
        .and_then(log_field_string)
        .expect(constants::error::AGENT_EVENT_SERIALIZES);
    serde_json::from_str(history_json).expect(constants::error::AGENT_EVENT_SERIALIZES)
}

fn log_field_string(value: &LogFieldValue) -> Option<&str> {
    match value {
        LogFieldValue::String(inner) => Some(inner.as_str()),
        _ => None,
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
        .expect(constants::error::AGENT_EVENT_SERIALIZES)
        .as_nanos()
}

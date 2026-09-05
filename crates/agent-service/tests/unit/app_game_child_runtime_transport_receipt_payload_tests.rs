use std::fs::remove_file;
use std::path::PathBuf as TestPathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use ocentra_parent_agent_protocol::app_game_child_runtime_transport_receipt::{
    AppGameChildRuntimeTransportReceiptReadModel,
    APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_READ_MODEL_ID,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventName, AgentMessageTarget, AgentPeer,
    AgentPeerRole, AgentRoute,
};
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;

use crate::{
    activity_report_env_lock::REPORT_ENV_LOCK, test_require_json_decode::require_json_decode,
    test_require_log_string_field::require_log_string_field,
};

use super::app_game_child_runtime_transport_receipt_payload::build_activity_app_game_child_runtime_transport_receipt_report;

const APP_GAME_TEST_TIMESTAMP: &str = "2026-06-03T22:15:00Z";

#[tokio::test]
async fn child_runtime_transport_receipt_report_serializes_parent_safe_empty_store_model() {
    let _guard = REPORT_ENV_LOCK.lock().await;
    let store_path = temp_path(constants::activity_store::TEST_STORE_SUFFIX);
    cleanup_path(&store_path);
    std::env::set_var(constants::env_var::ACTIVITY_DB_PATH, &store_path);

    let event =
        build_activity_app_game_child_runtime_transport_receipt_report(command_envelope()).await;
    let read_model = require_json_decode::<AppGameChildRuntimeTransportReceiptReadModel>(
        require_log_string_field(
            event
                .payload
                .get(constants::field::APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_READ_MODEL),
            constants::error::AGENT_EVENT_SERIALIZES,
        ),
        constants::error::AGENT_EVENT_SERIALIZES,
    );

    std::env::remove_var(constants::env_var::ACTIVITY_DB_PATH);
    cleanup_path(&store_path);

    assert_eq!(
        event.event,
        AgentEventName::AgentActivityAppGameChildRuntimeTransportReceiptReadModelReported
    );
    assert_eq!(
        read_model.read_model_id,
        APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_READ_MODEL_ID
    );
    assert_eq!(read_model.returned, 0);
    assert_eq!(read_model.transport_required_count, 0);
    assert_eq!(read_model.manual_required_count, 0);
    assert_eq!(read_model.unavailable_count, 0);
    assert!(!read_model.runtime_transport_executed);
    assert!(!read_model.runtime_receipt_ingested);
    assert!(!read_model.provider_delivery_executed);
    assert!(!read_model.platform_delivery_channel_claimed);
}

fn command_envelope() -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id:
            constants::event_id::ACTIVITY_APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_READ_MODEL_REPORTED
                .to_string(),
        sent_at: APP_GAME_TEST_TIMESTAMP.to_string(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform:
                ocentra_parent_agent_protocol::policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS
                    .to_string(),
            route: AgentRoute::Localhost,
        },
        command: AgentCommandName::AgentActivityAppGameChildRuntimeTransportReceiptReadModelGet,
        payload: LogFields::new(),
    }
}

fn temp_path(suffix: impl std::fmt::Display) -> TestPathBuf {
    let mut name = String::from(constants::activity_store::TEST_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(
        &SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
            .to_string(),
    );
    name.push(constants::delimiter::HYPHEN);
    name.push_str(constants::value::APP_GAME_TEST_CHILD_RUNTIME_TRANSPORT_RECEIPT_TEMP_SUFFIX);
    name.push(constants::delimiter::HYPHEN);
    name.push_str(&suffix.to_string());
    let mut path = std::env::temp_dir();
    path.push(name);
    path.set_extension(constants::activity_store::FILE_EXTENSION);
    path
}

fn cleanup_path(path: &std::path::Path) {
    let _ = remove_file(path);
    let _ = remove_file(path.with_extension(constants::activity_store::WAL_FILE_EXTENSION));
    let _ = remove_file(path.with_extension(constants::activity_store::SHM_FILE_EXTENSION));
}

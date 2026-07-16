use std::path::PathBuf as TestPathBuf;
use std::primitive::str as TestStr;
use std::{
    path::TestPathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::screen_settings::ScreenAnalysisParentSetting;
use ocentra_parent_agent_protocol::screen_settings::ScreenSettingsGetRequest;
use ocentra_parent_agent_protocol::screen_settings::ScreenSettingsRejectionReason;
use ocentra_parent_agent_protocol::screen_settings::ScreenSettingsUpdateKind;
use ocentra_parent_agent_protocol::screen_settings::ScreenSettingsUpdateResponse;
use ocentra_parent_agent_protocol::screen_settings::ScreenSettingsUpdateStatus;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentCommandName;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;
use ocentra_parent_agent_protocol::transport::AgentMessageTarget;
use ocentra_parent_agent_protocol::transport::AgentPeer;
use ocentra_parent_agent_protocol::transport::AgentPeerRole;
use ocentra_parent_agent_protocol::transport::AgentRoute;
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::SCREEN_EVIDENCE_SCHEMA_VERSION;

use crate::{
    screen_settings_api::build_screen_settings_event,
    screen_settings_runtime::ScreenSettingsRuntime,
    test_invariants::{require_json_decode, require_log_string_field, serialize_test_json},
};

#[tokio::test]
async fn screen_settings_replace_persists_and_get_reports_after_runtime_restart() {
    let path =
        temp_screen_settings_store_path(constants::screen_settings::TEST_PATH_SUFFIX_COMMAND);
    let runtime = ScreenSettingsRuntime::for_store_path(&path);
    let setting = strict_dry_run_setting(2);

    let replace_event =
        send_screen_settings_command(runtime, replace_command(None, &setting)).await;
    let replace_response = response_from_event(&replace_event);

    assert_eq!(
        replace_event.event,
        AgentEventName::AgentScreenSettingsReplaceAccepted
    );
    assert_eq!(
        replace_response.status,
        ScreenSettingsUpdateStatus::Accepted
    );
    assert_eq!(replace_response.setting, Some(setting.clone()));
    assert_eq!(
        replace_response.audit_event_id.as_deref(),
        Some(constants::screen_settings::TEST_AUDIT_EVENT_ID_1)
    );
    assert!(path.exists());

    let restarted = ScreenSettingsRuntime::for_store_path(&path);
    let get_event = send_screen_settings_command(restarted, get_command()).await;
    let get_response = response_from_event(&get_event);

    assert_eq!(get_event.event, AgentEventName::AgentScreenSettingsReported);
    assert_eq!(get_response.status, ScreenSettingsUpdateStatus::Accepted);
    assert_eq!(get_response.setting, Some(setting));
}

#[tokio::test]
async fn screen_settings_replace_rejects_raw_image_retention_before_persisting() {
    let path = temp_screen_settings_store_path(
        constants::screen_settings::REJECTION_RAW_RETENTION_FORBIDDEN,
    );
    let runtime = ScreenSettingsRuntime::for_store_path(&path);
    let mut setting = strict_dry_run_setting(2);
    setting.retain_raw_image = true;

    let rejected_event =
        send_screen_settings_command(runtime, replace_command(None, &setting)).await;
    let rejected_response = response_from_event(&rejected_event);

    assert_eq!(
        rejected_event.event,
        AgentEventName::AgentScreenSettingsReplaceRejected
    );
    assert_eq!(
        rejected_response.status,
        ScreenSettingsUpdateStatus::Rejected
    );
    assert_eq!(
        rejected_response.rejection_reason,
        Some(ScreenSettingsRejectionReason::RawRetentionForbidden)
    );
    assert_eq!(rejected_response.setting, None);
    assert!(!path.exists());
}

async fn send_screen_settings_command(
    runtime: ScreenSettingsRuntime,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let _ = serialize_test_json(&command);
    build_screen_settings_event(runtime, command).await
}

fn response_from_event(event: &AgentEventEnvelope) -> ScreenSettingsUpdateResponse {
    let text = require_log_string_field(
        event
            .payload
            .get(constants::field::SCREEN_SETTINGS_RESPONSE),
        constants::field::SCREEN_SETTINGS_RESPONSE,
    );
    require_json_decode(text, constants::error::AGENT_EVENT_SERIALIZES)
}

fn get_command() -> AgentCommandEnvelope {
    command_with_request(
        AgentCommandName::AgentScreenSettingsGet,
        ScreenSettingsGetRequest {
            schema_version: SCREEN_EVIDENCE_SCHEMA_VERSION,
            request_id: constants::screen_settings::REQUEST_ID_GET.to_string(),
            kind: ScreenSettingsUpdateKind::Get,
        },
    )
}

fn replace_command(
    base_setting_version: Option<u64>,
    setting: &ScreenAnalysisParentSetting,
) -> AgentCommandEnvelope {
    command_with_request(
        AgentCommandName::AgentScreenSettingsReplace,
        serde_json::json!({
            "schemaVersion": SCREEN_EVIDENCE_SCHEMA_VERSION,
            "requestId": constants::screen_settings::REQUEST_ID_REPLACE,
            "kind": ScreenSettingsUpdateKind::Replace,
            "baseSettingVersion": base_setting_version,
            "setting": setting,
        }),
    )
}

fn command_with_request<T>(command: AgentCommandName, request: T) -> AgentCommandEnvelope
where
    T: serde::Serialize,
{
    let mut payload = LogFields::new();
    payload.insert(
        constants::field::SCREEN_SETTINGS_REQUEST.to_string(),
        LogFieldValue::String(serialize_test_json(&request)),
    );
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::screen_settings::COMMAND_MESSAGE_ID.to_string(),
        sent_at: constants::screen_settings::TEST_SENT_AT.to_string(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform: constants::enforcement::PLATFORM_WINDOWS.to_string(),
            route: AgentRoute::Localhost,
        },
        command,
        payload,
    }
}

fn strict_dry_run_setting(version: u64) -> ScreenAnalysisParentSetting {
    ScreenAnalysisParentSetting {
        schema_version: SCREEN_EVIDENCE_SCHEMA_VERSION,
        screen_analysis_enabled: true,
        analysis_mode: constants::screen_settings::ANALYSIS_MODE_POLICY_DRY_RUN.to_string(),
        cadence_capture_enabled: true,
        cadence_seconds: constants::screen_settings::STRICT_CADENCE_SECONDS,
        strict_mode_enabled: true,
        trigger_capture_enabled: true,
        enabled_triggers: vec![
            constants::screen_settings::CAPTURE_TRIGGER_TIMED_CADENCE.to_string(),
            constants::screen_settings::CAPTURE_TRIGGER_NATIVE_APP_FOREGROUND.to_string(),
        ],
        allowed_capture_scope: constants::screen_settings::CAPTURE_SCOPE_ACTIVE_WINDOW.to_string(),
        ocr_text_enabled: true,
        ocr_text_snippet_limit: 8,
        redaction_mode: constants::screen_settings::REDACTION_MODE_LOCAL_SENSITIVE_TEXT.to_string(),
        ocr_text_retention_mode: constants::screen_settings::OCR_TEXT_RETENTION_REDACTED_SNIPPETS
            .to_string(),
        credential_suppression_enabled: true,
        pii_redaction_enabled: true,
        temporary_image_ttl_seconds: constants::screen_settings::DEFAULT_TTL_SECONDS,
        max_retry_count: constants::screen_settings::DEFAULT_RETRY_COUNT,
        delete_after_success: true,
        delete_after_expiry: true,
        retain_raw_image: false,
        policy_use_enabled: true,
        changed_by_parent_ref: constants::screen_settings::DEFAULT_CHANGED_BY_PARENT_REF
            .to_string(),
        changed_at: constants::screen_settings::DEFAULT_CHANGED_AT.to_string(),
        setting_version: version,
        reason: Some(constants::screen_settings::STRICT_REASON.to_string()),
    }
}

fn temp_screen_settings_store_path(path_label: &TestStr) -> TestPathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let mut path = std::env::temp_dir();
    let mut file_name = constants::screen_settings::TEST_STORE_FILE_PREFIX.to_string();
    file_name.push(constants::delimiter::HYPHEN);
    file_name.push_str(path_label);
    file_name.push(constants::delimiter::HYPHEN);
    file_name.push_str(&stamp.to_string());
    file_name.push(constants::delimiter::DOT);
    file_name.push_str(constants::screen_settings::TEST_JSON_EXTENSION);
    path.push(file_name);
    path
}

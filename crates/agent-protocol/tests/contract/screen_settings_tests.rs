use super::{
    constants, AgentCommandName, AgentEventName, ScreenAnalysisParentSetting,
    ScreenSettingsGetRequest, ScreenSettingsRejectionReason, ScreenSettingsUpdateKind,
    ScreenSettingsUpdateResponse, ScreenSettingsUpdateStatus, SCREEN_EVIDENCE_SCHEMA_VERSION,
};
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn screen_parent_setting_serializes_parent_opt_in_and_retention_fields() {
    let setting = strict_dry_run_setting(2);
    let serialized =
        serde_json::to_value(setting).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(serialized["screenAnalysisEnabled"], true);
    assert_eq!(
        serialized["analysisMode"],
        constants::screen_settings::ANALYSIS_MODE_POLICY_DRY_RUN
    );
    assert_eq!(serialized["cadenceSeconds"], 60);
    assert_eq!(serialized["deleteAfterSuccess"], true);
    assert_eq!(serialized["deleteAfterExpiry"], true);
    assert_eq!(serialized["retainRawImage"], false);
    assert_eq!(
        serialized["ocrTextRetentionMode"],
        constants::screen_settings::OCR_TEXT_RETENTION_REDACTED_SNIPPETS
    );
}

#[test]
fn screen_parent_setting_serializes_parent_approved_raw_retention_ttl() {
    let mut setting = strict_dry_run_setting(3);
    setting.retain_raw_image = true;
    setting.temporary_image_ttl_seconds = constants::screen_settings::RAW_RETENTION_MAX_TTL_SECONDS;
    setting.reason = Some(constants::screen_settings::RAW_RETENTION_LOCAL_TTL_REASON.to_string());
    let serialized =
        serde_json::to_value(setting).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(serialized["retainRawImage"], true);
    assert_eq!(
        serialized["temporaryImageTtlSeconds"],
        constants::screen_settings::RAW_RETENTION_MAX_TTL_SECONDS
    );
    assert_eq!(
        serialized["reason"],
        constants::screen_settings::RAW_RETENTION_LOCAL_TTL_REASON
    );
}

#[test]
fn screen_settings_request_rejects_unknown_fields() {
    let invalid = serde_json::json!({
        "schemaVersion": SCREEN_EVIDENCE_SCHEMA_VERSION,
        "requestId": constants::screen_settings::REQUEST_ID_GET,
        "kind": constants::screen_settings::UPDATE_KIND_GET,
        "rawScreenshotUploadEnabled": true
    });

    let parsed = serde_json::from_value::<ScreenSettingsGetRequest>(invalid);

    assert_eq!(
        parsed.err().map(|error| error.classify()),
        Some(serde_json::error::Category::Data)
    );
}

#[test]
fn screen_settings_response_serializes_rejection_reason() {
    let response = ScreenSettingsUpdateResponse {
        schema_version: SCREEN_EVIDENCE_SCHEMA_VERSION,
        request_id: constants::screen_settings::REQUEST_ID_REPLACE.to_string(),
        kind: ScreenSettingsUpdateKind::Replace,
        status: ScreenSettingsUpdateStatus::Rejected,
        setting: None,
        audit_event_id: None,
        rejection_reason: Some(ScreenSettingsRejectionReason::RawRetentionForbidden),
        message: Some(constants::screen_settings::MESSAGE_INVALID_SETTING.to_string()),
    };
    let serialized =
        serde_json::to_value(response).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(serialized["rejectionReason"], "raw-retention-forbidden");
    assert_eq!(
        serialized["message"],
        constants::screen_settings::MESSAGE_INVALID_SETTING
    );
}

#[test]
fn screen_settings_transport_names_serialize_for_service_commands() {
    let get = serde_json::to_value(AgentCommandName::AgentScreenSettingsGet)
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    let replace = serde_json::to_value(AgentCommandName::AgentScreenSettingsReplace)
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    let reported = serde_json::to_value(AgentEventName::AgentScreenSettingsReported)
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    let accepted = serde_json::to_value(AgentEventName::AgentScreenSettingsReplaceAccepted)
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    let rejected = serde_json::to_value(AgentEventName::AgentScreenSettingsReplaceRejected)
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        get.as_str(),
        Some(constants::screen_settings::COMMAND_NAME_GET)
    );
    assert_eq!(
        replace.as_str(),
        Some(constants::screen_settings::COMMAND_NAME_REPLACE)
    );
    assert_eq!(
        reported.as_str(),
        Some(constants::screen_settings::EVENT_NAME_REPORTED)
    );
    assert_eq!(
        accepted.as_str(),
        Some(constants::screen_settings::EVENT_NAME_REPLACE_ACCEPTED)
    );
    assert_eq!(
        rejected.as_str(),
        Some(constants::screen_settings::EVENT_NAME_REPLACE_REJECTED)
    );
}

pub(crate) fn strict_dry_run_setting(version: u64) -> ScreenAnalysisParentSetting {
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

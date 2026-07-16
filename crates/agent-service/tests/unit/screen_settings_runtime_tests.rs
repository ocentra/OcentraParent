use std::path::PathBuf as TestPathBuf;
use std::string::String as TestString;
use std::{fs, path::PathBuf};

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::screen_settings::ScreenAnalysisParentSetting;
use ocentra_parent_agent_protocol::screen_settings::ScreenSettingsGetRequest;
use ocentra_parent_agent_protocol::screen_settings::ScreenSettingsRejectionReason;
use ocentra_parent_agent_protocol::screen_settings::ScreenSettingsUpdateKind;
use ocentra_parent_agent_protocol::screen_settings::ScreenSettingsUpdateRequest;
use ocentra_parent_agent_protocol::screen_settings::ScreenSettingsUpdateStatus;
use ocentra_parent_agent_protocol::SCREEN_EVIDENCE_SCHEMA_VERSION;

use crate::{
    screen_settings_runtime::{default_disabled_setting, ScreenSettingsRuntime},
    test_invariants::{require_ok, require_some},
};

#[tokio::test]
async fn screen_settings_runtime_reports_disabled_default_without_persistence() {
    let runtime = test_runtime("default-disabled");

    let response = runtime
        .handle_request(ScreenSettingsUpdateRequest::Get(ScreenSettingsGetRequest {
            schema_version: SCREEN_EVIDENCE_SCHEMA_VERSION,
            request_id: constants::screen_settings::REQUEST_ID_GET.to_string(),
            kind: ScreenSettingsUpdateKind::Get,
        }))
        .await;

    assert_eq!(response.status, ScreenSettingsUpdateStatus::Accepted);
    let setting = require_some(
        response.setting,
        constants::screen_settings::TEST_SETTING_RETURNED,
    );
    assert!(!setting.screen_analysis_enabled);
    assert!(!setting.cadence_capture_enabled);
    assert!(!setting.trigger_capture_enabled);
    assert!(!setting.policy_use_enabled);
    assert!(!setting.retain_raw_image);
    assert!(setting.delete_after_success);
    assert!(setting.delete_after_expiry);
}

#[tokio::test]
async fn screen_settings_runtime_persists_parent_opt_in_across_reload() {
    let path = test_store_path(constants::screen_settings::TEST_PATH_SUFFIX_PERSISTENCE);
    let runtime = ScreenSettingsRuntime::for_store_path(&path);
    let strict = strict_dry_run_setting(2);

    let accepted = runtime
        .handle_request(replace_request(None, &TestStrict))
        .await;

    assert_eq!(accepted.status, ScreenSettingsUpdateStatus::Accepted);
    assert_eq!(
        accepted.audit_event_id,
        Some(constants::screen_settings::TEST_AUDIT_EVENT_ID_1.to_string())
    );
    let reloaded = ScreenSettingsRuntime::for_store_path(&path);
    let reported = reloaded
        .handle_request(ScreenSettingsUpdateRequest::Get(ScreenSettingsGetRequest {
            schema_version: SCREEN_EVIDENCE_SCHEMA_VERSION,
            request_id: constants::screen_settings::REQUEST_ID_GET.to_string(),
            kind: ScreenSettingsUpdateKind::Get,
        }))
        .await;
    let persisted = require_some(
        reported.setting,
        constants::screen_settings::TEST_PERSISTED_SETTING_RETURNED,
    );

    assert_eq!(reported.status, ScreenSettingsUpdateStatus::Accepted);
    assert_eq!(persisted, strict);
    assert!(require_ok(
        fs::read_to_string(&path),
        constants::screen_settings::TEST_STORE_READABLE,
    )
    .contains(constants::screen_settings::AUDIT_PREFIX));
}

#[tokio::test]
async fn screen_settings_runtime_accepts_parent_approved_short_ttl_raw_retention() {
    let runtime = test_runtime("raw-retention-accepted");
    let mut setting = strict_dry_run_setting(2);
    setting.retain_raw_image = true;
    setting.temporary_image_ttl_seconds = constants::screen_settings::RAW_RETENTION_MAX_TTL_SECONDS;
    setting.reason = Some(constants::screen_settings::RAW_RETENTION_LOCAL_TTL_REASON.to_string());

    let accepted = runtime
        .handle_request(replace_request(None, &setting))
        .await;

    assert_eq!(accepted.status, ScreenSettingsUpdateStatus::Accepted);
    assert_eq!(accepted.setting, Some(setting));
}

#[tokio::test]
async fn screen_settings_runtime_rejects_unsafe_raw_image_retention() {
    let runtime = test_runtime("raw-retention-rejected");
    let mut setting = strict_dry_run_setting(2);
    setting.retain_raw_image = true;
    setting.temporary_image_ttl_seconds = constants::screen_settings::DEFAULT_TTL_SECONDS;

    let rejected = runtime
        .handle_request(replace_request(None, &setting))
        .await;

    assert_eq!(rejected.status, ScreenSettingsUpdateStatus::Rejected);
    assert_eq!(
        rejected.rejection_reason,
        Some(ScreenSettingsRejectionReason::RawRetentionForbidden)
    );
    assert_eq!(rejected.setting, None);
}

#[tokio::test]
async fn screen_settings_runtime_rejects_observe_only_policy_use() {
    let runtime = test_runtime("observe-only-policy-use");
    let mut setting = strict_dry_run_setting(2);
    setting.analysis_mode = constants::screen_settings::ANALYSIS_MODE_OBSERVE_ONLY.to_string();

    let rejected = runtime
        .handle_request(replace_request(None, &setting))
        .await;

    assert_eq!(rejected.status, ScreenSettingsUpdateStatus::Rejected);
    assert_eq!(
        rejected.rejection_reason,
        Some(ScreenSettingsRejectionReason::PolicyModeInconsistent)
    );
}

#[tokio::test]
async fn screen_settings_runtime_rejects_stale_base_setting_version() {
    let runtime = test_runtime("stale-base-setting-version");
    let accepted = runtime
        .handle_request(replace_request(None, &TestStrict_dry_run_setting(2)))
        .await;
    assert_eq!(accepted.status, ScreenSettingsUpdateStatus::Accepted);

    let rejected = runtime
        .handle_request(replace_request(Some(1), &TestStrict_dry_run_setting(3)))
        .await;

    assert_eq!(rejected.status, ScreenSettingsUpdateStatus::Rejected);
    assert_eq!(
        rejected.rejection_reason,
        Some(ScreenSettingsRejectionReason::StaleRevision)
    );
}

fn strict_dry_run_setting(version: u64) -> ScreenAnalysisParentSetting {
    ScreenAnalysisParentSetting {
        setting_version: version,
        ..default_disabled_setting(constants::screen_settings::DEFAULT_CHANGED_AT)
    }
    .with_strict_dry_run_values()
}

trait ScreenSettingTestValues {
    fn with_strict_dry_run_values(self) -> Self;
}

impl ScreenSettingTestValues for ScreenAnalysisParentSetting {
    fn with_strict_dry_run_values(mut self) -> Self {
        self.screen_analysis_enabled = true;
        self.analysis_mode = constants::screen_settings::ANALYSIS_MODE_POLICY_DRY_RUN.to_string();
        self.cadence_capture_enabled = true;
        self.cadence_seconds = constants::screen_settings::STRICT_CADENCE_SECONDS;
        self.strict_mode_enabled = true;
        self.trigger_capture_enabled = true;
        self.enabled_triggers = vec![
            constants::screen_settings::CAPTURE_TRIGGER_TIMED_CADENCE.to_string(),
            constants::screen_settings::CAPTURE_TRIGGER_NATIVE_APP_FOREGROUND.to_string(),
        ];
        self.ocr_text_enabled = true;
        self.ocr_text_snippet_limit = 8;
        self.redaction_mode =
            constants::screen_settings::REDACTION_MODE_LOCAL_SENSITIVE_TEXT.to_string();
        self.ocr_text_retention_mode =
            constants::screen_settings::OCR_TEXT_RETENTION_REDACTED_SNIPPETS.to_string();
        self.pii_redaction_enabled = true;
        self.policy_use_enabled = true;
        self.reason = Some(constants::screen_settings::STRICT_REASON.to_string());
        self
    }
}

fn replace_request(
    base_setting_version: Option<u64>,
    setting: &ScreenAnalysisParentSetting,
) -> ScreenSettingsUpdateRequest {
    serde_json::from_value(serde_json::json!({
        "schemaVersion": SCREEN_EVIDENCE_SCHEMA_VERSION,
        "requestId": constants::screen_settings::REQUEST_ID_REPLACE,
        "kind": ScreenSettingsUpdateKind::Replace,
        "baseSettingVersion": base_setting_version,
        "setting": setting,
    }))
    .expect(constants::error::AGENT_EVENT_SERIALIZES)
}

fn test_store_path(store_suffix: crate::test_text::TestText) -> TestPathBuf {
    let mut path = std::env::temp_dir();
    let mut file_name = TestString::from(constants::screen_settings::TEST_STORE_FILE_PREFIX);
    file_name.push(constants::delimiter::HYPHEN);
    file_name.push_str(store_suffix.0.as_str());
    file_name.push(constants::delimiter::HYPHEN);
    file_name.push_str(&std::process::id().to_string());
    file_name.push(constants::delimiter::DOT);
    file_name.push_str(constants::screen_settings::TEST_JSON_EXTENSION);
    path.push(file_name);
    let _ = fs::remove_file(&path);
    path
}

fn test_runtime(store_suffix: crate::test_text::TestText) -> ScreenSettingsRuntime {
    ScreenSettingsRuntime::for_store_path(test_store_path(store_suffix))
}

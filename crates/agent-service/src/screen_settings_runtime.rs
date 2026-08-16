use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::screen_settings::ScreenAnalysisParentSetting;
use ocentra_parent_agent_protocol::screen_settings::ScreenSettingsRejectionReason;
use ocentra_parent_agent_protocol::screen_settings::ScreenSettingsUpdateKind;
use ocentra_parent_agent_protocol::screen_settings::ScreenSettingsUpdateRequest;
use ocentra_parent_agent_protocol::screen_settings::ScreenSettingsUpdateResponse;
use ocentra_parent_agent_protocol::screen_settings::ScreenSettingsUpdateStatus;
use ocentra_parent_agent_protocol::SCREEN_EVIDENCE_SCHEMA_VERSION;
use tokio::sync::Mutex as AsyncMutex;

use crate::screen_settings_store::{
    read_screen_settings_state, screen_settings_store_path_from_env, write_screen_settings_state,
    ScreenSettingsStoreError, ScreenSettingsStoredState,
};

#[path = "screen_settings_runtime/get_flow.rs"]
mod get_flow;
#[path = "screen_settings_runtime/replace_flow.rs"]
mod replace_flow;
#[path = "screen_settings_runtime/validation.rs"]
mod validation;

#[derive(Clone, Debug, PartialEq, Eq)]
struct ScreenSettingsRequestId(String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ScreenSettingsTimestamp(String);

#[derive(Clone, Debug, PartialEq, Eq)]
struct ScreenSettingsText(String);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ScreenSettingsIdPrefix(&'static str);

#[derive(Clone, Debug, PartialEq, Eq)]
struct ScreenSettingsId(String);

#[derive(Clone, Debug)]
pub(crate) struct ScreenSettingsRuntime {
    persistence: ScreenSettingsPersistence,
    io_lock: Arc<AsyncMutex<()>>,
}

#[derive(Clone, Debug)]
enum ScreenSettingsPersistence {
    LocalJson(PathBuf),
}

impl ScreenSettingsRuntime {
    pub(crate) fn from_env() -> Self {
        Self::for_store_path(screen_settings_store_path_from_env())
    }

    pub(crate) fn for_store_path(path: impl AsRef<Path>) -> Self {
        Self {
            persistence: ScreenSettingsPersistence::LocalJson(path.as_ref().to_path_buf()),
            io_lock: Arc::new(AsyncMutex::new(())),
        }
    }

    pub(crate) async fn handle_request(
        &self,
        request: ScreenSettingsUpdateRequest,
    ) -> ScreenSettingsUpdateResponse {
        let _guard = self.io_lock.lock().await;
        match request {
            ScreenSettingsUpdateRequest::Get(request) => {
                get_flow::handle_get(self, ScreenSettingsRequestId(request.request_id)).await
            }
            ScreenSettingsUpdateRequest::Replace(request) => {
                replace_flow::handle_replace(
                    self,
                    ScreenSettingsRequestId(request.request_id),
                    request.base_setting_version,
                    request.setting,
                )
                .await
            }
        }
    }

    async fn read_state(&self) -> Result<ScreenSettingsStoredState, ScreenSettingsStoreError> {
        match &self.persistence {
            ScreenSettingsPersistence::LocalJson(path) => read_screen_settings_state(path).await,
        }
    }

    async fn write_state(
        &self,
        state: &ScreenSettingsStoredState,
    ) -> Result<(), ScreenSettingsStoreError> {
        match &self.persistence {
            ScreenSettingsPersistence::LocalJson(path) => {
                write_screen_settings_state(path, state).await
            }
        }
    }
}

pub(crate) fn default_disabled_setting(
    generated_at: impl Into<ScreenSettingsTimestamp>,
) -> ScreenAnalysisParentSetting {
    let generated_at = generated_at.into().0;
    ScreenAnalysisParentSetting {
        schema_version: SCREEN_EVIDENCE_SCHEMA_VERSION,
        screen_analysis_enabled: false,
        analysis_mode: constants::screen_settings::ANALYSIS_MODE_OBSERVE_ONLY.to_string(),
        cadence_capture_enabled: false,
        cadence_seconds: constants::screen_settings::DEFAULT_DISABLED_CADENCE_SECONDS,
        strict_mode_enabled: false,
        trigger_capture_enabled: false,
        enabled_triggers: Vec::new(),
        allowed_capture_scope: constants::screen_settings::CAPTURE_SCOPE_ACTIVE_WINDOW.to_string(),
        ocr_text_enabled: false,
        ocr_text_snippet_limit: 0,
        redaction_mode: constants::screen_settings::REDACTION_MODE_DISABLED.to_string(),
        ocr_text_retention_mode: constants::screen_settings::OCR_TEXT_RETENTION_DISABLED
            .to_string(),
        credential_suppression_enabled: true,
        pii_redaction_enabled: false,
        temporary_image_ttl_seconds: constants::screen_settings::DEFAULT_TTL_SECONDS,
        max_retry_count: constants::screen_settings::DEFAULT_RETRY_COUNT,
        delete_after_success: true,
        delete_after_expiry: true,
        retain_raw_image: false,
        policy_use_enabled: false,
        changed_by_parent_ref: constants::screen_settings::DEFAULT_CHANGED_BY_PARENT_REF
            .to_string(),
        changed_at: generated_at,
        setting_version: 1,
        reason: Some(constants::screen_settings::DEFAULT_REASON.to_string()),
    }
}

fn accepted_response(
    request_id: ScreenSettingsRequestId,
    kind: ScreenSettingsUpdateKind,
    setting: ScreenAnalysisParentSetting,
    audit_event_id: Option<ScreenSettingsId>,
    message: ScreenSettingsText,
) -> ScreenSettingsUpdateResponse {
    ScreenSettingsUpdateResponse {
        schema_version: SCREEN_EVIDENCE_SCHEMA_VERSION,
        request_id: request_id.0,
        kind,
        status: ScreenSettingsUpdateStatus::Accepted,
        setting: Some(setting),
        audit_event_id: audit_event_id.map(|value| value.0),
        rejection_reason: None,
        message: Some(message.0),
    }
}

fn rejected_response(
    request_id: ScreenSettingsRequestId,
    kind: ScreenSettingsUpdateKind,
    rejection_reason: ScreenSettingsRejectionReason,
    message: ScreenSettingsText,
) -> ScreenSettingsUpdateResponse {
    ScreenSettingsUpdateResponse {
        schema_version: SCREEN_EVIDENCE_SCHEMA_VERSION,
        request_id: request_id.0,
        kind,
        status: ScreenSettingsUpdateStatus::Rejected,
        setting: None,
        audit_event_id: None,
        rejection_reason: Some(rejection_reason),
        message: Some(message.0),
    }
}

fn next_revision_id(state: &ScreenSettingsStoredState) -> ScreenSettingsId {
    suffixed_id(
        ScreenSettingsIdPrefix(constants::screen_settings::REVISION_PREFIX),
        state.settings.len() + 1,
    )
}

fn next_audit_event_id(state: &ScreenSettingsStoredState) -> ScreenSettingsId {
    suffixed_id(
        ScreenSettingsIdPrefix(constants::screen_settings::AUDIT_PREFIX),
        state.audit_events.len() + 1,
    )
}

fn suffixed_id(prefix: ScreenSettingsIdPrefix, suffix: usize) -> ScreenSettingsId {
    let mut value = String::from(prefix.0);
    value.push_str(&suffix.to_string());
    ScreenSettingsId(value)
}

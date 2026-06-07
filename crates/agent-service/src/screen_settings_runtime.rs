use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

#[cfg(test)]
use std::sync::Mutex;

use ocentra_parent_agent_protocol::{
    constants, ScreenAnalysisParentSetting, ScreenSettingsRejectionReason,
    ScreenSettingsUpdateKind, ScreenSettingsUpdateRequest, ScreenSettingsUpdateResponse,
    ScreenSettingsUpdateStatus, SCREEN_EVIDENCE_SCHEMA_VERSION,
};
use tokio::sync::Mutex as AsyncMutex;

use crate::{
    screen_settings_store::{
        read_screen_settings_state, screen_settings_store_path_from_env,
        write_screen_settings_state, ScreenSettingsAuditRecord, ScreenSettingsRevisionRecord,
        ScreenSettingsStoreError, ScreenSettingsStoredState,
    },
    time::timestamp_now,
};

#[derive(Clone, Debug)]
pub(crate) struct ScreenSettingsRuntime {
    persistence: ScreenSettingsPersistence,
    io_lock: Arc<AsyncMutex<()>>,
}

#[derive(Clone, Debug)]
enum ScreenSettingsPersistence {
    #[cfg(test)]
    InMemory(Arc<Mutex<ScreenSettingsStoredState>>),
    LocalJson(PathBuf),
}

impl ScreenSettingsRuntime {
    #[allow(dead_code)]
    pub(crate) fn from_env() -> Self {
        Self::for_store_path(screen_settings_store_path_from_env())
    }

    #[cfg(test)]
    pub(crate) fn in_memory() -> Self {
        Self {
            persistence: ScreenSettingsPersistence::InMemory(Arc::new(Mutex::new(
                ScreenSettingsStoredState::empty(),
            ))),
            io_lock: Arc::new(AsyncMutex::new(())),
        }
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
            ScreenSettingsUpdateRequest::Get(request) => self.handle_get(request.request_id).await,
            ScreenSettingsUpdateRequest::Replace(request) => {
                self.handle_replace(
                    request.request_id,
                    request.base_setting_version,
                    request.setting,
                )
                .await
            }
        }
    }

    async fn handle_get(&self, request_id: String) -> ScreenSettingsUpdateResponse {
        let generated_at = timestamp_now();
        match self.read_state().await {
            Ok(state) => {
                let setting = state
                    .active_setting()
                    .map(|record| record.setting.clone())
                    .unwrap_or_else(|| default_disabled_setting(&generated_at));
                accepted_response(
                    request_id,
                    ScreenSettingsUpdateKind::Get,
                    setting,
                    None,
                    constants::screen_settings::MESSAGE_REPORTED,
                )
            }
            Err(_) => rejected_response(
                request_id,
                ScreenSettingsUpdateKind::Get,
                ScreenSettingsRejectionReason::StorageUnavailable,
                constants::screen_settings::MESSAGE_STORAGE_UNAVAILABLE,
            ),
        }
    }

    async fn handle_replace(
        &self,
        request_id: String,
        base_setting_version: Option<u64>,
        setting: ScreenAnalysisParentSetting,
    ) -> ScreenSettingsUpdateResponse {
        let generated_at = timestamp_now();
        if let Err(reason) = validate_screen_setting(&setting) {
            return rejected_response(
                request_id,
                ScreenSettingsUpdateKind::Replace,
                reason,
                constants::screen_settings::MESSAGE_INVALID_SETTING,
            );
        }
        let mut state = match self.read_state().await {
            Ok(state) => state,
            Err(_) => {
                return rejected_response(
                    request_id,
                    ScreenSettingsUpdateKind::Replace,
                    ScreenSettingsRejectionReason::StorageUnavailable,
                    constants::screen_settings::MESSAGE_STORAGE_UNAVAILABLE,
                )
            }
        };
        if state.active_setting_version != base_setting_version {
            return rejected_response(
                request_id,
                ScreenSettingsUpdateKind::Replace,
                ScreenSettingsRejectionReason::StaleRevision,
                constants::screen_settings::MESSAGE_STALE_REVISION,
            );
        }
        let audit_event_id = next_audit_event_id(&state);
        state.active_setting_version = Some(setting.setting_version);
        state.settings.push(ScreenSettingsRevisionRecord {
            revision_id: next_revision_id(&state),
            setting: setting.clone(),
            created_at: generated_at.clone(),
            audit_event_id: audit_event_id.clone(),
        });
        state.audit_events.push(ScreenSettingsAuditRecord {
            audit_event_id: audit_event_id.clone(),
            request_id: request_id.clone(),
            kind: ScreenSettingsUpdateKind::Replace,
            setting_version: setting.setting_version,
            created_at: generated_at,
        });
        if self.write_state(&state).await.is_err() {
            return rejected_response(
                request_id,
                ScreenSettingsUpdateKind::Replace,
                ScreenSettingsRejectionReason::StorageUnavailable,
                constants::screen_settings::MESSAGE_STORAGE_UNAVAILABLE,
            );
        }
        accepted_response(
            request_id,
            ScreenSettingsUpdateKind::Replace,
            setting,
            Some(audit_event_id),
            constants::screen_settings::MESSAGE_ACCEPTED,
        )
    }

    async fn read_state(&self) -> Result<ScreenSettingsStoredState, ScreenSettingsStoreError> {
        match &self.persistence {
            #[cfg(test)]
            ScreenSettingsPersistence::InMemory(state) => state
                .lock()
                .map(|state| state.clone())
                .map_err(|_| ScreenSettingsStoreError::Unavailable),
            ScreenSettingsPersistence::LocalJson(path) => read_screen_settings_state(path).await,
        }
    }

    async fn write_state(
        &self,
        state: &ScreenSettingsStoredState,
    ) -> Result<(), ScreenSettingsStoreError> {
        match &self.persistence {
            #[cfg(test)]
            ScreenSettingsPersistence::InMemory(current) => current
                .lock()
                .map(|mut current| {
                    *current = state.clone();
                })
                .map_err(|_| ScreenSettingsStoreError::Unavailable),
            ScreenSettingsPersistence::LocalJson(path) => {
                write_screen_settings_state(path, state).await
            }
        }
    }
}

pub(crate) fn default_disabled_setting(generated_at: &str) -> ScreenAnalysisParentSetting {
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
        changed_at: generated_at.to_string(),
        setting_version: 1,
        reason: Some(constants::screen_settings::DEFAULT_REASON.to_string()),
    }
}

fn validate_screen_setting(
    setting: &ScreenAnalysisParentSetting,
) -> Result<(), ScreenSettingsRejectionReason> {
    if setting.schema_version != SCREEN_EVIDENCE_SCHEMA_VERSION
        || setting.setting_version == 0
        || setting.cadence_seconds < constants::screen_settings::MIN_CADENCE_SECONDS
        || setting.cadence_seconds > constants::screen_settings::MAX_CADENCE_SECONDS
        || setting.temporary_image_ttl_seconds < constants::screen_settings::MIN_TTL_SECONDS
        || setting.temporary_image_ttl_seconds > constants::screen_settings::MAX_TTL_SECONDS
        || setting.max_retry_count > constants::screen_settings::MAX_RETRY_COUNT
        || setting.ocr_text_snippet_limit > constants::screen_settings::MAX_OCR_SNIPPET_LIMIT
        || !setting.credential_suppression_enabled
        || !setting.delete_after_success
        || !setting.delete_after_expiry
    {
        return Err(ScreenSettingsRejectionReason::InvalidSetting);
    }
    if setting.retain_raw_image {
        return Err(ScreenSettingsRejectionReason::RawRetentionForbidden);
    }
    if !setting.screen_analysis_enabled
        && (setting.cadence_capture_enabled
            || setting.strict_mode_enabled
            || setting.trigger_capture_enabled
            || setting.policy_use_enabled)
    {
        return Err(ScreenSettingsRejectionReason::DisabledSettingInconsistent);
    }
    if setting.policy_use_enabled
        && (!setting.screen_analysis_enabled
            || setting.analysis_mode == constants::screen_settings::ANALYSIS_MODE_OBSERVE_ONLY)
    {
        return Err(ScreenSettingsRejectionReason::PolicyModeInconsistent);
    }
    if setting.strict_mode_enabled
        && (!setting.screen_analysis_enabled
            || !setting.cadence_capture_enabled
            || setting.cadence_seconds != constants::screen_settings::STRICT_CADENCE_SECONDS)
    {
        return Err(ScreenSettingsRejectionReason::StrictModeInconsistent);
    }
    if setting.trigger_capture_enabled
        && (!setting.screen_analysis_enabled || setting.enabled_triggers.is_empty())
    {
        return Err(ScreenSettingsRejectionReason::TriggerModeInconsistent);
    }
    if !setting.ocr_text_enabled
        && (setting.ocr_text_snippet_limit != 0
            || setting.redaction_mode != constants::screen_settings::REDACTION_MODE_DISABLED
            || setting.ocr_text_retention_mode
                != constants::screen_settings::OCR_TEXT_RETENTION_DISABLED
            || setting.pii_redaction_enabled)
    {
        return Err(ScreenSettingsRejectionReason::OcrModeInconsistent);
    }
    if setting.ocr_text_enabled
        && (setting.ocr_text_snippet_limit == 0
            || setting.redaction_mode == constants::screen_settings::REDACTION_MODE_DISABLED
            || setting.ocr_text_retention_mode
                == constants::screen_settings::OCR_TEXT_RETENTION_DISABLED)
    {
        return Err(ScreenSettingsRejectionReason::OcrModeInconsistent);
    }
    Ok(())
}

fn accepted_response(
    request_id: String,
    kind: ScreenSettingsUpdateKind,
    setting: ScreenAnalysisParentSetting,
    audit_event_id: Option<String>,
    message: &str,
) -> ScreenSettingsUpdateResponse {
    ScreenSettingsUpdateResponse {
        schema_version: SCREEN_EVIDENCE_SCHEMA_VERSION,
        request_id,
        kind,
        status: ScreenSettingsUpdateStatus::Accepted,
        setting: Some(setting),
        audit_event_id,
        rejection_reason: None,
        message: Some(message.to_string()),
    }
}

fn rejected_response(
    request_id: String,
    kind: ScreenSettingsUpdateKind,
    rejection_reason: ScreenSettingsRejectionReason,
    message: &str,
) -> ScreenSettingsUpdateResponse {
    ScreenSettingsUpdateResponse {
        schema_version: SCREEN_EVIDENCE_SCHEMA_VERSION,
        request_id,
        kind,
        status: ScreenSettingsUpdateStatus::Rejected,
        setting: None,
        audit_event_id: None,
        rejection_reason: Some(rejection_reason),
        message: Some(message.to_string()),
    }
}

fn next_revision_id(state: &ScreenSettingsStoredState) -> String {
    format!(
        "{}{}",
        constants::screen_settings::REVISION_PREFIX,
        state.settings.len() + 1
    )
}

fn next_audit_event_id(state: &ScreenSettingsStoredState) -> String {
    format!(
        "{}{}",
        constants::screen_settings::AUDIT_PREFIX,
        state.audit_events.len() + 1
    )
}

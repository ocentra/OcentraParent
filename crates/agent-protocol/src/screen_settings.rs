use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenSettingsUpdateKind {
    #[serde(rename = "get")]
    Get,
    #[serde(rename = "replace")]
    Replace,
}

impl ScreenSettingsUpdateKind {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Get => crate::constants::screen_settings::UPDATE_KIND_GET,
            Self::Replace => crate::constants::screen_settings::UPDATE_KIND_REPLACE,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenSettingsUpdateStatus {
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "rejected")]
    Rejected,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenSettingsRejectionReason {
    #[serde(rename = "storage-unavailable")]
    StorageUnavailable,
    #[serde(rename = "invalid-setting")]
    InvalidSetting,
    #[serde(rename = "stale-revision")]
    StaleRevision,
    #[serde(rename = "raw-retention-forbidden")]
    RawRetentionForbidden,
    #[serde(rename = "disabled-setting-inconsistent")]
    DisabledSettingInconsistent,
    #[serde(rename = "policy-mode-inconsistent")]
    PolicyModeInconsistent,
    #[serde(rename = "strict-mode-inconsistent")]
    StrictModeInconsistent,
    #[serde(rename = "trigger-mode-inconsistent")]
    TriggerModeInconsistent,
    #[serde(rename = "ocr-mode-inconsistent")]
    OcrModeInconsistent,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScreenSettingsUpdateRequest {
    Get(ScreenSettingsGetRequest),
    Replace(Box<ScreenSettingsReplaceRequest>),
}

impl ScreenSettingsUpdateRequest {
    pub fn request_id(&self) -> &str {
        match self {
            Self::Get(request) => &request.request_id,
            Self::Replace(request) => &request.request_id,
        }
    }

    pub fn kind(&self) -> ScreenSettingsUpdateKind {
        match self {
            Self::Get(_) => ScreenSettingsUpdateKind::Get,
            Self::Replace(_) => ScreenSettingsUpdateKind::Replace,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ScreenSettingsGetRequest {
    pub schema_version: u16,
    pub request_id: String,
    pub kind: ScreenSettingsUpdateKind,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ScreenSettingsReplaceRequest {
    pub schema_version: u16,
    pub request_id: String,
    pub kind: ScreenSettingsUpdateKind,
    pub base_setting_version: Option<u64>,
    pub setting: ScreenAnalysisParentSetting,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreenSettingsUpdateResponse {
    pub schema_version: u16,
    pub request_id: String,
    pub kind: ScreenSettingsUpdateKind,
    pub status: ScreenSettingsUpdateStatus,
    pub setting: Option<ScreenAnalysisParentSetting>,
    pub audit_event_id: Option<String>,
    pub rejection_reason: Option<ScreenSettingsRejectionReason>,
    pub message: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ScreenAnalysisParentSetting {
    pub schema_version: u16,
    pub screen_analysis_enabled: bool,
    pub analysis_mode: String,
    pub cadence_capture_enabled: bool,
    pub cadence_seconds: u64,
    pub strict_mode_enabled: bool,
    pub trigger_capture_enabled: bool,
    pub enabled_triggers: Vec<String>,
    pub allowed_capture_scope: String,
    pub ocr_text_enabled: bool,
    pub ocr_text_snippet_limit: u64,
    pub redaction_mode: String,
    pub ocr_text_retention_mode: String,
    pub credential_suppression_enabled: bool,
    pub pii_redaction_enabled: bool,
    pub temporary_image_ttl_seconds: u64,
    pub max_retry_count: u64,
    pub delete_after_success: bool,
    pub delete_after_expiry: bool,
    pub retain_raw_image: bool,
    pub policy_use_enabled: bool,
    pub changed_by_parent_ref: String,
    pub changed_at: String,
    pub setting_version: u64,
    pub reason: Option<String>,
}

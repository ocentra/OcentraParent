use serde::{Deserialize, Serialize};

pub const SOCIAL_SOURCE_CUSTODY_MUTATION_SCHEMA_VERSION: &str =
    "social-source-custody-mutation-proof";
pub const SOCIAL_SOURCE_CUSTODY_MUTATION_ID: &str = "social-source-custody-mutation-service";
pub const SOCIAL_SOURCE_CUSTODY_MUTATION_STATE_APPLIED: &str = "applied";
pub const SOCIAL_SOURCE_CUSTODY_SETTINGS_ID: &str = "social-source-custody-settings-service";
pub const SOCIAL_SOURCE_CUSTODY_CHILD_PROFILE_ID: &str = "child-social-source-custody";
pub const SOCIAL_SOURCE_CUSTODY_DEVICE_ID: &str = "device-social-source-custody";
pub const SOCIAL_SOURCE_CUSTODY_PRIVACY_EVIDENCE_ID: &str = "social-video-source-privacy-service";
pub const SOCIAL_SOURCE_CUSTODY_EVIDENCE_REF: &str = "evidence-social-source-custody-service";
pub const SOCIAL_SOURCE_CUSTODY_AUDIT_REF: &str = "audit-social-source-custody-service";
pub const SOCIAL_SOURCE_CUSTODY_SCOPE_MANAGED_BROWSER: &str = "managed-browser-social-route";
pub const SOCIAL_SOURCE_CUSTODY_PERMISSION_ENABLED: &str = "enabled";
pub const SOCIAL_SOURCE_CUSTODY_MODE_REDACTED_REFS: &str = "local-redacted-refs-only";
pub const SOCIAL_SOURCE_CUSTODY_RETENTION_REDACTED_JOURNAL: &str = "redacted-ref-journal-only";
pub const SOCIAL_SOURCE_CUSTODY_USE_AI_CANDIDATE: &str = "ai-candidate-input";
pub const SOCIAL_SOURCE_CUSTODY_USE_PARENT_EXPLANATION: &str = "parent-explanation";
pub const SOCIAL_SOURCE_CUSTODY_NO_RAW_MESSAGE: &str = "raw-message-content-not-allowed";
pub const SOCIAL_SOURCE_CUSTODY_NO_RAW_VIDEO: &str = "raw-video-content-not-allowed";
pub const SOCIAL_SOURCE_CUSTODY_NO_SCREENSHOT: &str = "screenshot-custody-not-allowed";
pub const SOCIAL_SOURCE_CUSTODY_NO_CONNECTOR_TOKEN: &str = "connector-token-not-stored";
pub const SOCIAL_SOURCE_CUSTODY_NO_CONNECTOR_API: &str = "connector-api-not-called";
pub const SOCIAL_SOURCE_CUSTODY_NO_RUNTIME_UI: &str = "runtime-settings-ui-not-claimed";
pub const SOCIAL_SOURCE_CUSTODY_NO_RUNTIME_CUSTODY_CLAIM: &str =
    "runtime-custody-mutation-not-claimed";
pub const SOCIAL_SOURCE_CUSTODY_NO_FINAL_POLICY: &str = "final-policy-decision-not-claimed";
pub const SOCIAL_SOURCE_CUSTODY_NO_ENFORCEMENT: &str = "enforcement-not-claimed";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialSourceCustodyMutationSnapshot {
    pub schema_version: String,
    pub mutation_id: String,
    pub requested_at: String,
    pub applied_at: String,
    pub mutation_state: String,
    pub settings: SocialSourceCustodySettingsSnapshot,
    pub evidence_refs: Vec<String>,
    pub audit_refs: Vec<String>,
    pub service_mutation_executed: bool,
    pub runtime_custody_mutation_applied: bool,
    pub raw_content_custody_claimed: bool,
    pub connector_api_called: bool,
    pub final_policy_decision_claimed: bool,
    pub enforcement_claimed: bool,
    pub product_claim_ready: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialSourceCustodySettingsSnapshot {
    pub schema_version: u16,
    pub settings_id: String,
    pub generated_at: String,
    pub child_profile_ref: String,
    pub device_id: String,
    pub source_privacy_evidence_ids: Vec<String>,
    pub evidence_refs: Vec<String>,
    pub setting_scope: String,
    pub permission_state: String,
    pub custody_mode: String,
    pub retention_mode: String,
    pub permitted_downstream_uses: Vec<String>,
    pub disabled_use_reasons: Vec<String>,
    pub parent_review_refs: Vec<String>,
    pub connector_authorization_refs: Vec<String>,
    pub manual_proof_requirements: Vec<String>,
    pub no_claim_labels: Vec<String>,
    pub raw_message_content_allowed: bool,
    pub raw_video_content_allowed: bool,
    pub screenshot_custody_allowed: bool,
    pub connector_token_stored: bool,
    pub connector_api_called: bool,
    pub runtime_settings_ui_claimed: bool,
    pub runtime_custody_mutation_claimed: bool,
    pub final_policy_decision_claimed: bool,
    pub enforcement_claimed: bool,
}

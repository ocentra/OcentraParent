use serde::{Deserialize, Serialize};

pub const APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION: &str = "v0.6";
pub const APP_GAME_PARENT_PLATFORM_WINDOWS: &str = "windows";
pub const APP_GAME_PARENT_PLATFORM_ANDROID: &str = "android";
pub const APP_GAME_PARENT_PLATFORM_LINUX: &str = "linux";
pub const APP_GAME_PARENT_ACTOR_ROLE_PARENT: &str = "parent";
pub const APP_GAME_PARENT_EVIDENCE_KIND_ACTIVITY_EVENT: &str = "activity-event";
pub const APP_GAME_POLICY_ACTION_BLOCK: &str = "block";
pub const APP_GAME_POLICY_TARGET_TYPE_APP: &str = "app";
pub const APP_GAME_CONTROL_POLICY_KIND_APP: &str = "app-control";
pub const APP_GAME_CONTROL_POLICY_KIND_GAME: &str = "game-control";
pub const APP_GAME_CONTROL_AUTHORITY_ACTIVE: &str = "active";
pub const APP_GAME_CONTROL_AUTHORITY_OBSERVE_ONLY: &str = "observe-only";
pub const APP_GAME_CONTROL_DECISION_APPROVED: &str = "approved";
pub const APP_GAME_CONTROL_DECISION_DENIED: &str = "denied";
pub const APP_GAME_CONTROL_UNANSWERED_FALLBACK_DENY: &str = "deny";
pub const APP_GAME_CONTROL_CHILD_REASON_NOT_REQUESTED: &str = "not-requested";
pub const APP_GAME_CONTROL_PARENT_RESPONSE_ALLOW_ONCE: &str = "allow-once";
pub const APP_GAME_CONTROL_PERSISTENCE_NOT_PERSISTED: &str = "not-persisted";
pub const APP_GAME_CONTROL_PERSISTENCE_REPLAYABLE: &str = "replayable";
pub const APP_GAME_CONTROL_APPROVAL_STATE_APPROVED: &str = "approved";
pub const APP_GAME_CONTROL_APPROVAL_STATE_MANUAL_REQUIRED: &str = "manual-required";
pub const APP_GAME_CONTROL_ACTION_STATUS_ENFORCED: &str = "enforced";
pub const APP_GAME_CONTROL_ACTION_STATUS_MANUAL_REQUIRED: &str = "manual-required";
pub const APP_GAME_CONTROL_EVIDENCE_PROOF_APP_IDENTITY: &str = "app-identity-proof";
pub const APP_GAME_CONTROL_EVIDENCE_PROOF_LAUNCHER_ONLY: &str = "launcher-only";
pub const APP_GAME_APPROVAL_CANDIDATE_NEW_INVENTORY_APP: &str = "new-inventory-app";
pub const APP_GAME_APPROVAL_CANDIDATE_SOURCE_INVENTORY: &str = "inventory";
pub const APP_GAME_ENFORCEMENT_ADAPTER_PROCESS_CONTROL: &str = "process-control";
pub const APP_GAME_ENFORCEMENT_MODE_TERMINATE_PROCESS: &str = "terminate-process";
pub const APP_GAME_ENFORCEMENT_MODE_BLOCK_PROCESS: &str = "block-process";
pub const APP_GAME_ENFORCEMENT_CAPABILITY_SUPPORTED: &str = "supported";
pub const APP_GAME_ENFORCEMENT_CAPABILITY_MANUAL_REQUIRED: &str = "manual-required";
pub const APP_GAME_ENFORCEMENT_PERMISSION_ALLOWED: &str = "allowed";
pub const APP_GAME_ENFORCEMENT_DEPENDENCY_INSTALLED: &str = "installed";
pub const APP_GAME_ENFORCEMENT_RESULT_ACTUALLY_ENFORCED: &str = "actually-enforced";
pub const APP_GAME_ENFORCEMENT_ADAPTER_RESULT_PROCESS_TERMINATED: &str = "process-terminated";
pub const APP_GAME_ENFORCEMENT_ROLLBACK_NOT_REQUIRED: &str = "not-required";
pub const APP_GAME_PLATFORM_TIER_DEVICE_OWNER: &str = "device-owner";
pub const APP_GAME_PLATFORM_TIER_MANUAL_REQUIRED: &str = "manual-required";
pub const APP_GAME_PLATFORM_ACTION_HIDE_APP: &str = "hide-app";
pub const APP_GAME_PLATFORM_ACTION_BLOCK_LAUNCH: &str = "block-launch";
pub const APP_GAME_PLATFORM_SETUP_DEVICE_OWNER_REQUIRED: &str = "device-owner-required";
pub const APP_GAME_PLATFORM_SETUP_MANUAL_REQUIRED: &str = "manual-required";
pub const APP_GAME_PLATFORM_PROOF_STATE_RUNTIME_ATTACHED: &str = "runtime-proof-attached";
pub const APP_GAME_PLATFORM_PROOF_STATE_MANUAL_REQUIRED: &str = "manual-required";
pub const APP_GAME_PLATFORM_PARENT_VISIBLE_MANAGED_DEVICE_REQUIRED: &str =
    "managed-device-required";
pub const APP_GAME_PLATFORM_PARENT_VISIBLE_MANUAL_REQUIRED: &str = "manual-required";
pub const APP_GAME_PLATFORM_PROOF_KIND_DEVICE_OWNER: &str = "device-owner-proof";
pub const APP_GAME_PLATFORM_PROOF_KIND_ROLLBACK: &str = "rollback-proof";
pub const APP_GAME_PLATFORM_PROOF_KIND_WINDOWS_APPLOCKER: &str = "windows-applocker-proof";
pub const APP_GAME_PLATFORM_PROOF_KIND_WINDOWS_APP_CONTROL: &str = "windows-app-control-proof";
pub const APP_GAME_AI_CLASSIFIER_PRODUCT_UNKNOWN_APP: &str = "unknownApp";
pub const APP_GAME_AI_CLASSIFIER_PRODUCT_UNKNOWN_GAME: &str = "unknownGame";
pub const APP_GAME_AI_CLASSIFIER_DIGEST_INVENTORY: &str = "inventoryEvidence";
pub const APP_GAME_AI_CLASSIFIER_DIGEST_SESSION_SUMMARY: &str = "sessionSummary";
pub const APP_GAME_AI_CLASSIFIER_CANDIDATE_UNKNOWN_IDENTITY: &str = "unknownIdentityCandidate";
pub const APP_GAME_AI_CLASSIFIER_CANDIDATE_GAME_CONTEXT: &str = "gameContextCandidate";
pub const APP_GAME_AI_CLASSIFIER_STATE_CANDIDATE: &str = "candidate";
pub const APP_GAME_AI_CLASSIFIER_STATE_PROVIDER_UNAVAILABLE: &str = "providerUnavailable";
pub const APP_GAME_AI_CLASSIFIER_HANDOFF_PARENT_REVIEW: &str = "parentReview";
pub const APP_GAME_AI_CLASSIFIER_HANDOFF_MANUAL_REVIEW: &str = "manualReview";
pub const APP_GAME_AI_CLASSIFIER_FALLBACK_NOT_NEEDED: &str = "notNeeded";
pub const APP_GAME_AI_CLASSIFIER_FALLBACK_LOCAL_MODEL_UNAVAILABLE: &str = "localModelUnavailable";
pub const APP_GAME_AI_CLASSIFIER_FORBIDDEN_KEYS: [&str; 14] = [
    "adapterAction",
    "block",
    "directAction",
    "durationMs",
    "enforcementAction",
    "fileScanRows",
    "foregroundDurationMs",
    "hide",
    "processScanRows",
    "rawOsScanResult",
    "runningDurationMs",
    "shield",
    "suspend",
    "terminate",
];
pub const APP_GAME_TEST_TIMESTAMP: &str = "2026-06-03T22:15:00Z";
pub const APP_GAME_TEST_POLICY_VERSION: &str = "policy-version-app-game-1";
pub const APP_GAME_TEST_PARENT_ACTOR_ID: &str = "parent-actor-app-game-1";
pub const APP_GAME_TEST_DEVICE_ID: &str = "device-windows-app-game-1";
pub const APP_GAME_TEST_CHILD_PROFILE_ID: &str = "child-app-game-1";
pub const APP_GAME_TEST_DEVICE_LABEL: &str = "Study PC";
pub const APP_GAME_TEST_EVIDENCE_REF_ID: &str = "evidence-app-game-session-1";
pub const APP_GAME_TEST_TARGET_ID: &str = "target-app-game-1";
pub const APP_GAME_TEST_TARGET_VALUE: &str = "process:ocentra-game.exe";
pub const APP_GAME_TEST_SETTING_ID: &str = "app.enforcement.allowedActions";
pub const APP_GAME_TEST_SETTING_PATH: &str = "/appPolicy/enforcement/allowedActions";
pub const APP_GAME_TEST_AUTHORITY_ID: &str = "authority-app-game-1";
pub const APP_GAME_TEST_REQUEST_ID: &str = "approval-request-app-game-1";
pub const APP_GAME_TEST_DECISION_ID: &str = "approval-decision-app-game-1";
pub const APP_GAME_TEST_ACTION_REFERENCE_ID: &str = "parent-action-app-game-1";
pub const APP_GAME_TEST_ACTION_RESULT_ID: &str = "action-result-app-game-1";
pub const APP_GAME_TEST_ENFORCEMENT_ACTION_ID: &str = "enforcement-action-app-game-1";
pub const APP_GAME_TEST_ENFORCEMENT_RESULT_ID: &str = "enforcement-result-app-game-1";
pub const APP_GAME_TEST_REASON_PARENT_APPROVED: &str = "parent-approved";
pub const APP_GAME_TEST_CANDIDATE_ID: &str = "candidate-new-app-1";
pub const APP_GAME_TEST_PLATFORM_MATRIX_ID: &str = "app-game-platform-authority-matrix";
pub const APP_GAME_TEST_ANDROID_ROW_ID: &str = "android-hide-row";
pub const APP_GAME_TEST_WINDOWS_ROW_ID: &str = "windows-block-launch-row";
pub const APP_GAME_TEST_DEVICE_OWNER_PROOF_REF: &str = "proof/android-device-owner.md";
pub const APP_GAME_TEST_ROLLBACK_PROOF_REF: &str = "proof/rollback.md";
pub const APP_GAME_TEST_APPLOCKER_PROOF_REF: &str = "proof/windows-applocker.md";
pub const APP_GAME_TEST_WINDOWS_LIMITATION: &str =
    "Broad installed-app blocking needs AppLocker or App Control proof before execution.";
pub const APP_GAME_TEST_ANDROID_LIMITATION: &str =
    "Requires Android Device Owner provisioning and rollback proof.";
pub const APP_GAME_TEST_CLASSIFIER_RUN_ID: &str = "classifier-run-app-game-1";
pub const APP_GAME_TEST_CLASSIFIER_DIGEST_REF: &str = "classifier-digest-app-game-1";
pub const APP_GAME_TEST_CLASSIFIER_EVIDENCE_REF: &str = "classifier-evidence-app-game-1";
pub const APP_GAME_TEST_CLASSIFIER_SESSION_REF: &str = "classifier-session-app-game-1";
pub const APP_GAME_TEST_CLASSIFIER_RUNTIME_REF: &str = "local-ai-runtime-app-game";
pub const APP_GAME_TEST_CLASSIFIER_PROMPT_REF: &str = "prompt-app-game-classifier";
pub const APP_GAME_TEST_CLASSIFIER_LABEL: &str = "Possible native game";
pub const APP_GAME_TEST_CLASSIFIER_REASON_CODE: &str = "unknown-game-like";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameParentActorReference {
    pub actor_id: String,
    pub role: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameParentDeviceReference {
    pub device_id: String,
    pub child_profile_id: Option<String>,
    pub label: String,
    pub platform: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameParentEvidenceReference {
    pub evidence_reference_id: String,
    pub kind: String,
    pub observed_at: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameParentActionReference {
    pub action_reference_id: String,
    pub actor: AppGameParentActorReference,
    pub policy_version: String,
    pub created_at: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGamePolicyTarget {
    pub target_id: String,
    pub target_type: String,
    pub target_value: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameControlSettingReference {
    pub setting_id: String,
    pub writes_to: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameControlApprovalCandidate {
    pub candidate_id: String,
    pub candidate_kind: String,
    pub candidate_source: String,
    pub detected_at: String,
    pub evidence_references: Vec<AppGameParentEvidenceReference>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameControlApprovalAuthority {
    pub schema_version: String,
    pub authority_id: String,
    pub actor: AppGameParentActorReference,
    pub device: AppGameParentDeviceReference,
    pub policy_version: String,
    pub authority_state: String,
    pub allowed_policy_kinds: Vec<String>,
    pub can_approve: bool,
    pub can_deny: bool,
    pub can_extend: bool,
    pub can_override: bool,
    pub can_observe_only: bool,
    pub checked_at: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameControlApprovalRequest {
    pub schema_version: String,
    pub request_id: String,
    pub policy_kind: String,
    pub device: AppGameParentDeviceReference,
    pub target: AppGamePolicyTarget,
    pub requested_action: String,
    pub requested_mode: Option<String>,
    pub requested_setting_refs: Vec<AppGameControlSettingReference>,
    pub evidence_references: Vec<AppGameParentEvidenceReference>,
    pub candidate: Option<AppGameControlApprovalCandidate>,
    pub child_reason_state: String,
    pub child_reason_references: Vec<String>,
    pub child_status_references: Vec<String>,
    pub expires_at: String,
    pub unanswered_fallback: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameControlApprovalDecision {
    pub schema_version: String,
    pub decision_id: String,
    pub request_id: String,
    pub policy_kind: String,
    pub decision_state: String,
    pub parent_action: Option<AppGameParentActionReference>,
    pub reason_codes: Vec<String>,
    pub policy_version: String,
    pub evidence_references: Vec<AppGameParentEvidenceReference>,
    pub response_scope: Option<String>,
    pub decision_expires_at: Option<String>,
    pub audit_references: Vec<String>,
    pub persistence_state: String,
    pub decided_at: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameEnforcementCapabilityStatus {
    pub schema_version: String,
    pub platform: String,
    pub adapter_kind: String,
    pub capability_state: String,
    pub permission_state: String,
    pub dependency_state: String,
    pub supported_actions: Vec<String>,
    pub degraded_reason: Option<String>,
    pub last_checked_at: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameEnforcementUnavailableStatus {
    pub schema_version: String,
    pub capability: AppGameEnforcementCapabilityStatus,
    pub unavailable_reason: String,
    pub retryable: bool,
    pub checked_at: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameEnforcementResult {
    pub schema_version: String,
    pub result_id: String,
    pub action_id: String,
    pub status: String,
    pub adapter_result_code: String,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub rollback_token: Option<String>,
    pub rollback_state: String,
    pub unavailable_reason: Option<String>,
    pub unavailable_status: Option<AppGameEnforcementUnavailableStatus>,
    pub failed_reason: Option<String>,
    pub next_check_at: Option<String>,
    pub capability: AppGameEnforcementCapabilityStatus,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameControlActionResult {
    pub schema_version: String,
    pub result_id: String,
    pub request: AppGameControlApprovalRequest,
    pub decision: AppGameControlApprovalDecision,
    pub approval_state: String,
    pub capability_state: String,
    pub capability: AppGameEnforcementCapabilityStatus,
    pub evidence_proof_kind: String,
    pub result_status: String,
    pub enforcement_result: Option<AppGameEnforcementResult>,
    pub recorded_at: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGamePlatformProofReference {
    pub proof_kind: String,
    pub artifact_ref: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGamePlatformAuthorityRow {
    pub schema_version: String,
    pub row_id: String,
    pub platform: String,
    pub action: String,
    pub authority_tier: String,
    pub setup_state: String,
    pub proof_state: String,
    pub capability_state: String,
    pub parent_visible_state: String,
    pub parent_visible_limitation: String,
    pub can_execute_adapter: bool,
    pub supported_modes: Vec<String>,
    pub proof_references: Vec<AppGamePlatformProofReference>,
    pub proof_needed_to_claim: Vec<String>,
    pub linux_mechanism: Option<String>,
    pub linux_distro: Option<String>,
    pub linux_session: Option<String>,
    pub last_checked_at: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGamePlatformAuthorityMatrix {
    pub schema_version: String,
    pub matrix_id: String,
    pub rows: Vec<AppGamePlatformAuthorityRow>,
    pub generated_at: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameAiClassifierResult {
    pub schema_version: u16,
    pub classifier_run_id: String,
    pub product_kind: String,
    pub digest_ref: String,
    pub source_digest_kind: String,
    pub source_evidence_refs: Vec<String>,
    pub source_session_refs: Vec<String>,
    pub candidate_kind: String,
    pub candidate_label: String,
    pub classifier_state: String,
    pub confidence: f64,
    pub uncertainty_reason_codes: Vec<String>,
    pub model_runtime_ref: String,
    pub prompt_template_ref: String,
    pub prompt_version: String,
    pub fallback_state: String,
    pub policy_handoff: String,
    pub generated_at: String,
    pub direct_action_requested: bool,
    pub raw_scan_included: bool,
    pub content_claim_included: bool,
}

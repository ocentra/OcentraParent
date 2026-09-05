use serde::{Deserialize, Serialize};

use crate::app_game_authority_classifier::APP_GAME_PARENT_PLATFORM_WINDOWS;

#[path = "app_game_platform_proof_status/windows_local_policy.rs"]
mod windows_local_policy;

pub const APP_GAME_PLATFORM_PROOF_STATUS_READ_MODEL_ID: &str = "app-game-platform-proof-status";
pub const APP_GAME_PLATFORM_PROOF_STATUS_CUSTODY_LABEL: &str = "app-game-platform-proof-status";
pub const APP_GAME_PLATFORM_PROOF_STATUS_CAPABILITY_PARTIAL: &str =
    "app-game-platform-proof-status-partial";
pub const APP_GAME_PLATFORM_PROOF_STATUS_ROW_ID_PREFIX: &str = "app-game-platform-proof-status-";
pub const APP_GAME_PLATFORM_PROOF_SCOPED_WINDOWS_EXECUTION: &str =
    "scoped-windows-execution-proved";
pub const APP_GAME_PLATFORM_PROOF_ANDROID_HOST_VISIBLE: &str = "android-host-visible";
pub const APP_GAME_PLATFORM_PROOF_ANDROID_HOST_NOT_DETECTED: &str = "android-host-not-detected";
pub const APP_GAME_PLATFORM_PROOF_LINUX_HOST_VISIBLE: &str = "linux-host-visible";
pub const APP_GAME_PLATFORM_PROOF_LINUX_HOST_NOT_DETECTED: &str = "linux-host-not-detected";
pub const APP_GAME_LINUX_DOCKER_PREFLIGHT_READY: &str = "ready";
pub const APP_GAME_LINUX_DOCKER_PREFLIGHT_PARTIAL: &str = "partial";
pub const APP_GAME_LINUX_DOCKER_PREFLIGHT_DAEMON_UNAVAILABLE: &str = "daemon-unavailable";
pub const APP_GAME_LINUX_DOCKER_PREFLIGHT_NOT_DETECTED: &str = "not-detected";
pub const APP_GAME_LINUX_DOCKER_PREFLIGHT_PROBE_UNAVAILABLE: &str = "probe-unavailable";
pub const APP_GAME_PLATFORM_PROOF_LOCAL_RUNTIME_NOT_APPLICABLE: &str =
    "local-runtime-not-applicable";
pub const APP_GAME_PLATFORM_AUTHORITY_SCOPED_EXECUTION_ONLY: &str = "scoped-execution-only";
pub const APP_GAME_PLATFORM_AUTHORITY_VISIBILITY_ONLY: &str = "visibility-only";
pub const APP_GAME_PLATFORM_AUTHORITY_NOT_LOCALLY_PROVABLE: &str = "not-locally-provable";
pub const APP_GAME_PLATFORM_GAP_BROAD_BLOCKING: &str = "broad-installed-app-blocking-not-proved";
pub const APP_GAME_PLATFORM_GAP_PLATFORM_ENFORCEMENT: &str = "platform-enforcement-not-proved";
pub const APP_GAME_PLATFORM_GAP_CHILD_DELIVERY: &str = "child-device-delivery-not-proved";
pub const APP_GAME_PLATFORM_GAP_ANDROID_DEVICE_OWNER: &str = "android-device-owner-not-proved";
pub const APP_GAME_PLATFORM_GAP_ANDROID_USAGE_STATS: &str = "android-usage-stats-not-proved";
pub const APP_GAME_PLATFORM_GAP_ANDROID_DURABLE_USAGE_REPLAY: &str =
    "android-durable-usage-events-replay-not-proved";
pub const APP_GAME_PLATFORM_GAP_LINUX_NATIVE_SERVICE: &str = "linux-native-service-not-proved";
pub const APP_GAME_PLATFORM_GAP_LINUX_FOREGROUND_CAPTURE: &str =
    "linux-foreground-capture-not-proved";
pub const APP_GAME_PLATFORM_GAP_LINUX_ROLLBACK: &str = "linux-rollback-not-proved";
pub const APP_GAME_PLATFORM_GAP_LINUX_DOCKER_PREFLIGHT: &str =
    "linux-docker-host-preflight-not-ready";
pub const APP_GAME_PLATFORM_GAP_LINUX_DOCKER_CLI: &str = "linux-docker-cli-not-visible";
pub const APP_GAME_PLATFORM_GAP_LINUX_DOCKER_DAEMON: &str = "linux-docker-daemon-not-visible";
pub const APP_GAME_PLATFORM_GAP_LINUX_DOCKER_CONTEXT_INVENTORY: &str =
    "linux-docker-context-inventory-not-visible";
pub const APP_GAME_PLATFORM_GAP_LINUX_DOCKER_OBJECT_INVENTORY: &str =
    "linux-docker-image-container-inventory-not-visible";
pub const APP_GAME_PLATFORM_GAP_MACOS_ARTIFACTS: &str = "macos-artifacts-not-available-on-windows";
pub const APP_GAME_PLATFORM_GAP_IOS_ARTIFACTS: &str = "ios-artifacts-not-available-on-windows";
pub const APP_GAME_WINDOWS_LOCAL_POLICY_EVIDENCE_REF_PREFIX: &str =
    "app-game-windows-local-policy-evidence:";
pub const APP_GAME_WINDOWS_LOCAL_POLICY_MAX_COLLECTION_COUNT: u64 = 16;
pub const APP_GAME_WINDOWS_LOCAL_POLICY_MAX_RULE_COUNT: u64 = 100_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGameWindowsLocalPolicyEvidenceState {
    Ready,
    Partial,
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGameWindowsLocalPolicyEvidenceGap {
    AppIdServiceUnavailable,
    AppLockerPolicyUnreadable,
    DeviceGuardUnavailable,
    AppControlNotConfigured,
    BroadBlockingNotProved,
    SystemAllowlistNotProved,
    RollbackNotProved,
    AuditCustodyNotProved,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppGameWindowsLocalPolicyEvidenceError {
    InvalidSchemaVersion,
    InvalidObservedAt,
    InvalidState,
    InvalidAppIdServiceState,
    InvalidAppLockerCounts,
    InvalidDeviceGuardState,
    InvalidAppControlState,
    IdentifiersNotRedacted,
    InvalidProofReference,
    DuplicateProofReference,
    MissingRequiredGap,
    DuplicateGap,
    UnsupportedClaim,
    PlatformMismatch,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AppGameWindowsLocalPolicyEvidence {
    pub schema_version: u16,
    pub state: AppGameWindowsLocalPolicyEvidenceState,
    pub observed_at: String,
    pub probe_supported: bool,
    pub app_id_service_query_succeeded: bool,
    pub app_id_service_present: bool,
    pub app_id_service_running: bool,
    pub app_locker_policy_readable: bool,
    pub app_locker_collection_count: u64,
    pub app_locker_rule_count: u64,
    pub device_guard_query_succeeded: bool,
    pub device_guard_configured: bool,
    pub device_guard_running: bool,
    pub app_control_configured: bool,
    pub app_control_audit_only: bool,
    pub app_control_policy_reports_enforced: bool,
    pub identifiers_redacted: bool,
    pub proof_refs: Vec<String>,
    pub open_gaps: Vec<AppGameWindowsLocalPolicyEvidenceGap>,
    pub adapter_dispatch_claimed: bool,
    pub broad_installed_app_blocking_claimed: bool,
    pub platform_enforcement_claimed: bool,
    pub rollback_claimed: bool,
    pub audit_custody_claimed: bool,
    pub provider_delivery_claimed: bool,
    pub child_device_delivery_claimed: bool,
    pub private_diagnostics_claimed: bool,
}

impl AppGameWindowsLocalPolicyEvidence {
    pub fn validate(&self) -> Result<(), AppGameWindowsLocalPolicyEvidenceError> {
        self.validate_schema_and_state()?;
        self.validate_observations()?;
        self.validate_redaction_and_nonclaims()?;
        self.validate_references_and_gaps()
    }

    fn validate_schema_and_state(&self) -> Result<(), AppGameWindowsLocalPolicyEvidenceError> {
        windows_local_policy::validate_schema_and_state(self)
    }

    fn validate_observations(&self) -> Result<(), AppGameWindowsLocalPolicyEvidenceError> {
        windows_local_policy::validate_observations(self)
    }

    fn validate_redaction_and_nonclaims(
        &self,
    ) -> Result<(), AppGameWindowsLocalPolicyEvidenceError> {
        windows_local_policy::validate_redaction_and_nonclaims(self)
    }

    fn validate_references_and_gaps(&self) -> Result<(), AppGameWindowsLocalPolicyEvidenceError> {
        windows_local_policy::validate_references_and_gaps(self)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameLinuxDockerHostPreflight {
    pub schema_version: u16,
    pub state: String,
    pub cli_visible: bool,
    pub daemon_visible: bool,
    pub context_inventory_visible: bool,
    pub context_count: u64,
    pub image_inventory_visible: bool,
    pub image_count: u64,
    pub container_inventory_visible: bool,
    pub container_count: u64,
    pub identifiers_redacted: bool,
    pub proof_refs: Vec<String>,
    pub open_gaps: Vec<String>,
    pub adapter_dispatch_claimed: bool,
    pub platform_enforcement_claimed: bool,
    pub provider_delivery_claimed: bool,
    pub child_device_delivery_claimed: bool,
    pub private_diagnostics_claimed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGamePlatformProofStatusRow {
    pub schema_version: u16,
    pub row_id: String,
    pub platform: String,
    pub proof_state: String,
    pub authority_state: String,
    pub host_capability_state: String,
    pub host_capability_evidence_refs: Vec<String>,
    pub host_capability_probe_refs: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linux_docker_host_preflight: Option<AppGameLinuxDockerHostPreflight>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub windows_local_policy_evidence: Option<AppGameWindowsLocalPolicyEvidence>,
    pub product_meanings: Vec<String>,
    pub proof_refs: Vec<String>,
    pub open_gaps: Vec<String>,
    pub adapter_dispatch_claimed: bool,
    pub broad_installed_app_blocking_claimed: bool,
    pub platform_enforcement_claimed: bool,
    pub provider_delivery_claimed: bool,
    pub child_device_delivery_claimed: bool,
    pub private_diagnostics_claimed: bool,
    pub last_checked_at: String,
}

impl AppGamePlatformProofStatusRow {
    pub fn validate_windows_local_policy_evidence(
        &self,
    ) -> Result<(), AppGameWindowsLocalPolicyEvidenceError> {
        let Some(evidence) = &self.windows_local_policy_evidence else {
            return Ok(());
        };
        if self.platform != APP_GAME_PARENT_PLATFORM_WINDOWS {
            return Err(AppGameWindowsLocalPolicyEvidenceError::PlatformMismatch);
        }
        evidence.validate()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGamePlatformProofStatusReadModel {
    pub schema_version: u16,
    pub read_model_id: String,
    pub generated_at: String,
    pub source_read_model_ids: Vec<String>,
    pub custody_label: String,
    pub capability_status: String,
    pub returned: u64,
    pub host_visible_count: u64,
    pub host_not_detected_count: u64,
    pub local_runtime_not_applicable_count: u64,
    pub enforcement_ready_count: u64,
    pub open_gap_count: u64,
    pub adapter_dispatch_claimed: bool,
    pub broad_installed_app_blocking_claimed: bool,
    pub platform_enforcement_claimed: bool,
    pub provider_delivery_claimed: bool,
    pub child_device_delivery_claimed: bool,
    pub private_diagnostics_claimed: bool,
    pub rows: Vec<AppGamePlatformProofStatusRow>,
}

use serde::{Deserialize, Serialize};

use crate::browser_policy_sections::{
    BrowserPolicyAuditPlan, BrowserPolicyRuleActionPlan, BrowserPolicyRuleTarget,
    BrowserPolicySchedule,
};
use crate::browser_policy_values::{
    BrowserPolicyActionExecutionState, BrowserPolicyAiAuthority, BrowserPolicyApprovalState,
    BrowserPolicyAuditState, BrowserPolicyCapabilityState, BrowserPolicyDefaultPosture,
    BrowserPolicyDownloadState, BrowserPolicyEvidenceProofLevel, BrowserPolicyExecutionMode,
    BrowserPolicyManagedBrowserMode, BrowserPolicyManagementMode, BrowserPolicyProofFallback,
    BrowserPolicyReportState, BrowserPolicyRetentionState, BrowserPolicyTargetProofRequirement,
    BrowserPolicyUnmanagedBrowserMode, BrowserPolicyUrlTargetType,
};
use crate::{
    BrowserPolicyApprovalRequiredFor, BrowserPolicyApprovalUnansweredDefault,
    BrowserPolicyAuditRequiredField, BrowserPolicyBrowserGameApprovalMode,
    BrowserPolicyBrowserGamePolicyMode, BrowserPolicyBudgetCountingMode, BrowserPolicyChildFacing,
    BrowserPolicyCustody, BrowserPolicyDownloadBlockedType, BrowserPolicyEvidenceNeverCollect,
    BrowserPolicyEvidenceUrlScope, BrowserPolicyFallbacks,
    BrowserPolicyManagedBrowserBridgeRequirement, BrowserPolicyManagedBrowserFamily,
    BrowserPolicyManagedBrowserIntegrationMechanism, BrowserPolicyManagedBrowserLaunchMode,
    BrowserPolicyManagedBrowserProfileMode, BrowserPolicyManagedPolicyWriterControl,
    BrowserPolicyManagedPolicyWriterFallback, BrowserPolicyPlatforms, BrowserPolicyPortalAi,
    BrowserPolicyReportVisibleField, BrowserPolicyRetentionExactUrl, BrowserPolicyRuleAction,
    BrowserPolicyUnmanagedBrowserClassificationTarget,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyValue {
    pub schema_version: String,
    pub policy_id: String,
    pub enabled: bool,
    #[serde(default)]
    pub execution_mode: BrowserPolicyExecutionMode,
    pub default_posture: BrowserPolicyDefaultPosture,
    pub fallback_posture: Option<BrowserPolicyDefaultPosture>,
    pub management_mode: BrowserPolicyManagementMode,
    #[serde(default)]
    pub discovery: BrowserPolicyDiscovery,
    pub managed_browser: BrowserPolicyManagedBrowser,
    pub unmanaged_browser: BrowserPolicyUnmanagedBrowser,
    pub evidence: BrowserPolicyEvidenceRequirement,
    pub rules: BrowserPolicyRules,
    pub budgets: BrowserPolicyBudgets,
    #[serde(default)]
    pub browser_games: BrowserPolicyBrowserGames,
    pub downloads: BrowserPolicyDownloads,
    pub approvals: BrowserPolicyApprovals,
    pub reports: BrowserPolicyReports,
    pub audit: BrowserPolicyAudit,
    pub retention: BrowserPolicyRetention,
    #[serde(default)]
    pub custody: BrowserPolicyCustody,
    #[serde(default)]
    pub schedules: Vec<BrowserPolicySchedule>,
    #[serde(default)]
    pub child_facing: BrowserPolicyChildFacing,
    #[serde(default)]
    pub portal_ai: BrowserPolicyPortalAi,
    #[serde(default)]
    pub platforms: BrowserPolicyPlatforms,
    #[serde(default)]
    pub fallbacks: BrowserPolicyFallbacks,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyDiscovery {
    #[serde(default)]
    pub scan_installed_browsers: bool,
    #[serde(default = "default_discovery_enabled")]
    pub scan_running_browsers: bool,
    #[serde(default = "default_discovery_enabled")]
    pub detect_unmanaged_browsers: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyManagedBrowser {
    pub mode: BrowserPolicyManagedBrowserMode,
    #[serde(default)]
    pub allowed_families: Vec<BrowserPolicyManagedBrowserFamily>,
    #[serde(default)]
    pub launch_mode: BrowserPolicyManagedBrowserLaunchMode,
    #[serde(default)]
    pub profile_mode: BrowserPolicyManagedBrowserProfileMode,
    #[serde(default)]
    pub bridge_requirements: Vec<BrowserPolicyManagedBrowserBridgeRequirement>,
    #[serde(default)]
    pub integration_mechanisms: Vec<BrowserPolicyManagedBrowserIntegrationMechanism>,
    #[serde(default)]
    pub policy_writer_controls: Vec<BrowserPolicyManagedPolicyWriterControl>,
    #[serde(default)]
    pub policy_writer_fallback: BrowserPolicyManagedPolicyWriterFallback,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyUnmanagedBrowser {
    pub mode: BrowserPolicyUnmanagedBrowserMode,
    #[serde(default)]
    pub grace_seconds: u32,
    #[serde(default)]
    pub allow_recover_launch_url: bool,
    #[serde(default)]
    pub classification_targets: Vec<BrowserPolicyUnmanagedBrowserClassificationTarget>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyEvidenceRequirement {
    #[serde(default)]
    pub url_scope: BrowserPolicyEvidenceUrlScope,
    pub required_proof: BrowserPolicyEvidenceProofLevel,
    pub proof_fallback: Option<BrowserPolicyProofFallback>,
    #[serde(default = "default_when_proof_unavailable")]
    pub when_proof_unavailable: BrowserPolicyProofFallback,
    #[serde(default)]
    pub never_collect: Vec<BrowserPolicyEvidenceNeverCollect>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyRules {
    pub allowed_target_types: Vec<BrowserPolicyUrlTargetType>,
    #[serde(default)]
    pub allowed_actions: Vec<BrowserPolicyRuleAction>,
    #[serde(default)]
    pub items: Vec<BrowserPolicyRule>,
    #[serde(default)]
    pub entries: Vec<BrowserPolicyRule>,
    #[serde(default)]
    pub url_allow_list: Vec<String>,
    #[serde(default)]
    pub url_block_list: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyRule {
    pub rule_id: String,
    #[serde(default)]
    pub target_type: Option<BrowserPolicyUrlTargetType>,
    #[serde(default)]
    pub target_value: Option<String>,
    pub enabled: bool,
    #[serde(default)]
    pub priority: Option<u32>,
    #[serde(default)]
    pub target: Option<BrowserPolicyRuleTarget>,
    #[serde(default)]
    pub action: Option<BrowserPolicyRuleActionPlan>,
    #[serde(default)]
    pub proof_requirement: Option<String>,
    #[serde(default)]
    pub schedule_id: Option<String>,
    #[serde(default)]
    pub budget_id: Option<String>,
    #[serde(default)]
    pub audit_level: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyBudgets {
    #[serde(default = "default_budget_enabled")]
    pub enabled: bool,
    pub default_daily_minutes: Option<u32>,
    #[serde(default)]
    pub counting_mode: BrowserPolicyBudgetCountingMode,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyBrowserGames {
    #[serde(default)]
    pub educational_game_mode: BrowserPolicyBrowserGamePolicyMode,
    #[serde(default = "default_unknown_game_mode")]
    pub unknown_game_mode: BrowserPolicyBrowserGamePolicyMode,
    #[serde(default)]
    pub cloud_gaming_approval: BrowserPolicyBrowserGameApprovalMode,
    #[serde(default)]
    pub purchase_account_approval: BrowserPolicyBrowserGameApprovalMode,
    #[serde(default = "default_unblocked_portal_mode")]
    pub unblocked_portal_mode: BrowserPolicyBrowserGamePolicyMode,
    #[serde(default)]
    pub webgl_canvas_mode: BrowserPolicyBrowserGamePolicyMode,
    #[serde(default = "default_browser_game_daily_minutes")]
    pub default_daily_minutes: Option<u32>,
}

impl Default for BrowserPolicyBrowserGames {
    fn default() -> Self {
        Self {
            educational_game_mode: BrowserPolicyBrowserGamePolicyMode::Allow,
            unknown_game_mode: default_unknown_game_mode(),
            cloud_gaming_approval: BrowserPolicyBrowserGameApprovalMode::AskParent,
            purchase_account_approval: BrowserPolicyBrowserGameApprovalMode::AskParent,
            unblocked_portal_mode: default_unblocked_portal_mode(),
            webgl_canvas_mode: BrowserPolicyBrowserGamePolicyMode::Observe,
            default_daily_minutes: default_browser_game_daily_minutes(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyDownloads {
    #[serde(default)]
    pub mode: BrowserPolicyDownloadState,
    #[serde(default)]
    pub blocked_types: Vec<BrowserPolicyDownloadBlockedType>,
    #[serde(default)]
    pub state: BrowserPolicyDownloadState,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyApprovals {
    #[serde(default)]
    pub required_for: Vec<BrowserPolicyApprovalRequiredFor>,
    #[serde(default)]
    pub unanswered_default: BrowserPolicyApprovalUnansweredDefault,
    #[serde(default)]
    pub state: BrowserPolicyApprovalState,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyReports {
    #[serde(default)]
    pub visible_fields: Vec<BrowserPolicyReportVisibleField>,
    #[serde(default)]
    pub state: BrowserPolicyReportState,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyAudit {
    #[serde(default)]
    pub required_fields: Vec<BrowserPolicyAuditRequiredField>,
    #[serde(default)]
    pub state: BrowserPolicyAuditState,
    #[serde(default)]
    pub plan: BrowserPolicyAuditPlan,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyRetention {
    #[serde(default)]
    pub exact_url: BrowserPolicyRetentionExactUrl,
    #[serde(default)]
    pub state: BrowserPolicyRetentionState,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyEffectivePolicy {
    pub schema_version: String,
    pub policy_id: String,
    pub revision_id: String,
    pub compiled_hash: String,
    pub compiled_at: String,
    pub execution_mode: BrowserPolicyExecutionMode,
    pub default_posture: BrowserPolicyDefaultPosture,
    pub fallback_posture: Option<BrowserPolicyDefaultPosture>,
    pub discovery: BrowserPolicyDiscovery,
    pub budgets: BrowserPolicyBudgets,
    pub rules: Vec<BrowserPolicyEffectiveRule>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyEffectiveRule {
    pub rule_id: String,
    pub target_type: BrowserPolicyUrlTargetType,
    pub target_value: String,
    pub default_posture: BrowserPolicyDefaultPosture,
    pub evidence: BrowserPolicyEvidenceRequirement,
    pub action: BrowserPolicyRuleAction,
    pub target_proof_requirement: BrowserPolicyTargetProofRequirement,
    pub capability_state: BrowserPolicyCapabilityState,
    pub action_execution: BrowserPolicyActionExecutionState,
    pub ai_authority: BrowserPolicyAiAuthority,
    pub compile_note: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyCapabilityRegistry {
    pub schema_version: String,
    pub generated_at: String,
    pub capabilities: Vec<BrowserPolicyCapability>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyCapability {
    pub capability_id: String,
    pub state: BrowserPolicyCapabilityState,
    pub label: String,
    pub affected_writes_to: Vec<String>,
    pub checked_at: String,
    pub reason: Option<String>,
}

fn default_budget_enabled() -> bool {
    true
}

fn default_discovery_enabled() -> bool {
    true
}

fn default_when_proof_unavailable() -> BrowserPolicyProofFallback {
    BrowserPolicyProofFallback::MarkUnavailable
}

fn default_unknown_game_mode() -> BrowserPolicyBrowserGamePolicyMode {
    BrowserPolicyBrowserGamePolicyMode::AskParent
}

fn default_unblocked_portal_mode() -> BrowserPolicyBrowserGamePolicyMode {
    BrowserPolicyBrowserGamePolicyMode::Warn
}

fn default_browser_game_daily_minutes() -> Option<u32> {
    Some(30)
}

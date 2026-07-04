use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::BrowserPolicyApprovalRequiredFor;
use ocentra_parent_agent_protocol::BrowserPolicyApprovalState;
use ocentra_parent_agent_protocol::BrowserPolicyApprovalUnansweredDefault;
use ocentra_parent_agent_protocol::BrowserPolicyAuditRequiredField;
use ocentra_parent_agent_protocol::BrowserPolicyAuditState;
use ocentra_parent_agent_protocol::BrowserPolicyBrowserGameApprovalMode;
use ocentra_parent_agent_protocol::BrowserPolicyBrowserGamePolicyMode;
use ocentra_parent_agent_protocol::BrowserPolicyBudgetCountingMode;
use ocentra_parent_agent_protocol::BrowserPolicyCustodyAllowedUse;
use ocentra_parent_agent_protocol::BrowserPolicyDefaultPosture;
use ocentra_parent_agent_protocol::BrowserPolicyDownloadBlockedType;
use ocentra_parent_agent_protocol::BrowserPolicyDownloadState;
use ocentra_parent_agent_protocol::BrowserPolicyEvidenceNeverCollect;
use ocentra_parent_agent_protocol::BrowserPolicyEvidenceProofLevel;
use ocentra_parent_agent_protocol::BrowserPolicyEvidenceUrlScope;
use ocentra_parent_agent_protocol::BrowserPolicyExecutionMode;
use ocentra_parent_agent_protocol::BrowserPolicyManagedBrowserBridgeRequirement;
use ocentra_parent_agent_protocol::BrowserPolicyManagedBrowserFamily;
use ocentra_parent_agent_protocol::BrowserPolicyManagedBrowserIntegrationMechanism;
use ocentra_parent_agent_protocol::BrowserPolicyManagedBrowserLaunchMode;
use ocentra_parent_agent_protocol::BrowserPolicyManagedBrowserMode;
use ocentra_parent_agent_protocol::BrowserPolicyManagedBrowserProfileMode;
use ocentra_parent_agent_protocol::BrowserPolicyManagedPolicyWriterControl;
use ocentra_parent_agent_protocol::BrowserPolicyManagedPolicyWriterFallback;
use ocentra_parent_agent_protocol::BrowserPolicyManagementMode;
use ocentra_parent_agent_protocol::BrowserPolicyPatch;
use ocentra_parent_agent_protocol::BrowserPolicyProofFallback;
use ocentra_parent_agent_protocol::BrowserPolicyRejectionReason;
use ocentra_parent_agent_protocol::BrowserPolicyReportState;
use ocentra_parent_agent_protocol::BrowserPolicyReportVisibleField;
use ocentra_parent_agent_protocol::BrowserPolicyRetentionExactUrl;
use ocentra_parent_agent_protocol::BrowserPolicyRetentionState;
use ocentra_parent_agent_protocol::BrowserPolicyRule;
use ocentra_parent_agent_protocol::BrowserPolicyRuleAction;
use ocentra_parent_agent_protocol::BrowserPolicyUnmanagedBrowserClassificationTarget;
use ocentra_parent_agent_protocol::BrowserPolicyUnmanagedBrowserMode;
use ocentra_parent_agent_protocol::BrowserPolicyUpdateKind;
use ocentra_parent_agent_protocol::BrowserPolicyUpdateRequest;
use ocentra_parent_agent_protocol::BrowserPolicyUrlTargetType;
use ocentra_parent_agent_protocol::BrowserPolicyValue;
use serde::de::DeserializeOwned;

const COMMAND_KIND_RULES: [(&str, BrowserPolicyUpdateKind); 4] = [
    (
        constants::browser_policy::COMMAND_PREVIEW,
        BrowserPolicyUpdateKind::Preview,
    ),
    (
        constants::browser_policy::COMMAND_PATCH,
        BrowserPolicyUpdateKind::Patch,
    ),
    (
        constants::browser_policy::COMMAND_REPLACE,
        BrowserPolicyUpdateKind::Replace,
    ),
    (
        constants::browser_policy::COMMAND_ROLLBACK,
        BrowserPolicyUpdateKind::Rollback,
    ),
];

pub(crate) fn parse_browser_policy_request(
    command: &AgentCommandEnvelope,
) -> Result<BrowserPolicyUpdateRequest, BrowserPolicyRejectionReason> {
    match command
        .payload
        .get(constants::field::BROWSER_POLICY_REQUEST)
    {
        Some(LogFieldValue::String(text)) => serde_json::from_str(text).map_err(|error| {
            let _ = error;
            BrowserPolicyRejectionReason::InvalidRequest
        }),
        _ => Err(BrowserPolicyRejectionReason::InvalidRequest),
    }
}

pub(crate) fn kind_for_command(command: &AgentCommandEnvelope) -> BrowserPolicyUpdateKind {
    let command_name = serde_json::to_value(&command.command)
        .ok()
        .and_then(|value| value.as_str().map(|name| name.to_string()));
    COMMAND_KIND_RULES
        .iter()
        .find(|(name, _)| command_name.as_deref() == Some(*name))
        .map(|(_, kind)| *kind)
        .unwrap_or(BrowserPolicyUpdateKind::Get)
}

pub(crate) fn apply_browser_policy_patches(
    mut policy: BrowserPolicyValue,
    patches: &[BrowserPolicyPatch],
) -> Result<BrowserPolicyValue, BrowserPolicyRejectionReason> {
    for patch in patches {
        apply_browser_policy_patch(&mut policy, patch)?;
    }
    Ok(policy)
}

#[derive(Clone, Copy)]
struct BrowserPolicyPatchRule {
    writes_to: &'static str,
    apply: BrowserPolicyPatchHandler,
}

type BrowserPolicyPatchHandler =
    fn(&mut BrowserPolicyValue, &BrowserPolicyPatch) -> Result<(), BrowserPolicyRejectionReason>;

#[derive(Clone, Copy)]
struct BrowserPolicyFieldId(&'static str);

macro_rules! browser_policy_field_id {
    ($value:expr) => {
        BrowserPolicyFieldId($value)
    };
}

macro_rules! typed_patch_handler {
    ($name:ident, $value_ty:ty, $field_id:expr, $setter:expr) => {
        fn $name(
            policy: &mut BrowserPolicyValue,
            patch: &BrowserPolicyPatch,
        ) -> Result<(), BrowserPolicyRejectionReason> {
            apply_browser_policy_typed_patch::<$value_ty, _>(
                policy,
                patch,
                browser_policy_field_id!($field_id),
                $setter,
            )
        }
    };
}

const BROWSER_POLICY_PATCH_RULES: &[BrowserPolicyPatchRule] = &[
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_ENABLED,
        apply: apply_enabled_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_EXECUTION_MODE,
        apply: apply_execution_mode_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_DEFAULT_POSTURE,
        apply: apply_default_posture_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_MANAGEMENT_MODE,
        apply: apply_management_mode_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_DAILY_BUDGET_MINUTES,
        apply: apply_daily_budget_minutes_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_BUDGETS_ENABLED,
        apply: apply_budgets_enabled_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_BUDGET_COUNTING_MODE,
        apply: apply_budget_counting_mode_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_DISCOVERY_SCAN_INSTALLED_BROWSERS,
        apply: apply_scan_installed_browsers_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_DISCOVERY_SCAN_RUNNING_BROWSERS,
        apply: apply_scan_running_browsers_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_DISCOVERY_DETECT_UNMANAGED_BROWSERS,
        apply: apply_detect_unmanaged_browsers_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_MANAGED_BROWSER_MODE,
        apply: apply_managed_browser_mode_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_MANAGED_BROWSER_ALLOWED_FAMILIES,
        apply: apply_managed_browser_allowed_families_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_MANAGED_BROWSER_LAUNCH_MODE,
        apply: apply_managed_browser_launch_mode_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_MANAGED_BROWSER_PROFILE_MODE,
        apply: apply_managed_browser_profile_mode_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_MANAGED_BROWSER_BRIDGE_REQUIREMENTS,
        apply: apply_managed_browser_bridge_requirements_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_MANAGED_BROWSER_INTEGRATION_MECHANISMS,
        apply: apply_managed_browser_integration_mechanisms_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_MANAGED_BROWSER_POLICY_WRITER_CONTROLS,
        apply: apply_managed_browser_policy_writer_controls_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_MANAGED_BROWSER_POLICY_WRITER_FALLBACK,
        apply: apply_managed_browser_policy_writer_fallback_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_UNMANAGED_BROWSER_MODE,
        apply: apply_unmanaged_browser_mode_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_UNMANAGED_BROWSER_GRACE_SECONDS,
        apply: apply_unmanaged_browser_grace_seconds_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_UNMANAGED_BROWSER_ALLOW_RECOVER_LAUNCH_URL,
        apply: apply_unmanaged_browser_allow_recover_launch_url_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_UNMANAGED_BROWSER_CLASSIFICATION_TARGETS,
        apply: apply_unmanaged_browser_classification_targets_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_EVIDENCE_URL_SCOPE,
        apply: apply_evidence_url_scope_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_REQUIRED_PROOF,
        apply: apply_required_proof_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_PROOF_FALLBACK,
        apply: apply_proof_fallback_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_WHEN_PROOF_UNAVAILABLE,
        apply: apply_when_proof_unavailable_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_EVIDENCE_NEVER_COLLECT,
        apply: apply_evidence_never_collect_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_ALLOWED_TARGET_TYPES,
        apply: apply_allowed_target_types_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_ALLOWED_ACTIONS,
        apply: apply_allowed_actions_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_RULE_ITEMS,
        apply: apply_rule_items_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_URL_ALLOW_LIST,
        apply: apply_url_allow_list_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_URL_BLOCK_LIST,
        apply: apply_url_block_list_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_DOWNLOAD_MODE,
        apply: apply_download_mode_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_DOWNLOAD_BLOCKED_TYPES,
        apply: apply_download_blocked_types_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_DOWNLOAD_STATE,
        apply: apply_download_state_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_APPROVAL_REQUIRED_FOR,
        apply: apply_approval_required_for_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_APPROVAL_UNANSWERED_DEFAULT,
        apply: apply_approval_unanswered_default_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_APPROVAL_STATE,
        apply: apply_approval_state_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_BROWSER_GAME_EDUCATIONAL_MODE,
        apply: apply_browser_game_educational_mode_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_BROWSER_GAME_UNKNOWN_MODE,
        apply: apply_browser_game_unknown_mode_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_BROWSER_GAME_CLOUD_GAMING_APPROVAL,
        apply: apply_browser_game_cloud_gaming_approval_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_BROWSER_GAME_PURCHASE_ACCOUNT_APPROVAL,
        apply: apply_browser_game_purchase_account_approval_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_BROWSER_GAME_UNBLOCKED_PORTAL_MODE,
        apply: apply_browser_game_unblocked_portal_mode_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_BROWSER_GAME_WEBGL_CANVAS_MODE,
        apply: apply_browser_game_webgl_canvas_mode_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_BROWSER_GAME_DAILY_BUDGET_MINUTES,
        apply: apply_browser_game_daily_budget_minutes_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_REPORT_VISIBLE_FIELDS,
        apply: apply_report_visible_fields_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_REPORT_STATE,
        apply: apply_report_state_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_RETENTION_EXACT_URL,
        apply: apply_retention_exact_url_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_RETENTION_STATE,
        apply: apply_retention_state_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_AUDIT_STATE,
        apply: apply_audit_state_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_CUSTODY_ALLOWED_USES,
        apply: apply_custody_allowed_uses_patch,
    },
    BrowserPolicyPatchRule {
        writes_to: constants::browser_policy::WRITES_TO_AUDIT_REQUIRED_FIELDS,
        apply: apply_audit_required_fields_patch,
    },
];

fn apply_browser_policy_patch(
    policy: &mut BrowserPolicyValue,
    patch: &BrowserPolicyPatch,
) -> Result<(), BrowserPolicyRejectionReason> {
    if patch.op != constants::browser_policy::PATCH_OPERATION_REPLACE {
        return Err(BrowserPolicyRejectionReason::InvalidRequest);
    }
    let Some(rule) = BROWSER_POLICY_PATCH_RULES
        .iter()
        .find(|rule| rule.writes_to == patch.writes_to.as_str())
    else {
        return Err(BrowserPolicyRejectionReason::UnknownWritesTo);
    };
    (rule.apply)(policy, patch)
}

fn apply_browser_policy_typed_patch<T, F>(
    policy: &mut BrowserPolicyValue,
    patch: &BrowserPolicyPatch,
    expected_field_id: BrowserPolicyFieldId,
    apply: F,
) -> Result<(), BrowserPolicyRejectionReason>
where
    T: DeserializeOwned,
    F: FnOnce(&mut BrowserPolicyValue, T),
{
    require_field(patch, expected_field_id)?;
    let value = parse_patch_value::<T>(patch)?;
    apply(policy, value);
    Ok(())
}

typed_patch_handler!(
    apply_enabled_patch,
    bool,
    constants::browser_policy::FIELD_ID_ENABLED,
    |policy: &mut BrowserPolicyValue, value| {
        policy.enabled = value;
    }
);
typed_patch_handler!(
    apply_execution_mode_patch,
    BrowserPolicyExecutionMode,
    constants::browser_policy::FIELD_ID_EXECUTION_MODE,
    |policy: &mut BrowserPolicyValue, value| {
        policy.execution_mode = value;
    }
);
typed_patch_handler!(
    apply_default_posture_patch,
    BrowserPolicyDefaultPosture,
    constants::browser_policy::FIELD_ID_DEFAULT_POSTURE,
    |policy: &mut BrowserPolicyValue, value| {
        policy.default_posture = value;
    }
);
typed_patch_handler!(
    apply_management_mode_patch,
    BrowserPolicyManagementMode,
    constants::browser_policy::FIELD_ID_MANAGEMENT_MODE,
    |policy: &mut BrowserPolicyValue, value| {
        policy.management_mode = value;
    }
);
typed_patch_handler!(
    apply_daily_budget_minutes_patch,
    Option<u32>,
    constants::browser_policy::FIELD_ID_DAILY_BUDGET_MINUTES,
    |policy: &mut BrowserPolicyValue, value| {
        policy.budgets.default_daily_minutes = value;
    }
);
typed_patch_handler!(
    apply_budgets_enabled_patch,
    bool,
    constants::browser_policy::FIELD_ID_BUDGETS_ENABLED,
    |policy: &mut BrowserPolicyValue, value| {
        policy.budgets.enabled = value;
    }
);
typed_patch_handler!(
    apply_budget_counting_mode_patch,
    BrowserPolicyBudgetCountingMode,
    constants::browser_policy::FIELD_ID_BUDGET_COUNTING_MODE,
    |policy: &mut BrowserPolicyValue, value| {
        policy.budgets.counting_mode = value;
    }
);
typed_patch_handler!(
    apply_scan_installed_browsers_patch,
    bool,
    constants::browser_policy::FIELD_ID_DISCOVERY_SCAN_INSTALLED_BROWSERS,
    |policy: &mut BrowserPolicyValue, value| {
        policy.discovery.scan_installed_browsers = value;
    }
);
typed_patch_handler!(
    apply_scan_running_browsers_patch,
    bool,
    constants::browser_policy::FIELD_ID_DISCOVERY_SCAN_RUNNING_BROWSERS,
    |policy: &mut BrowserPolicyValue, value| {
        policy.discovery.scan_running_browsers = value;
    }
);
typed_patch_handler!(
    apply_detect_unmanaged_browsers_patch,
    bool,
    constants::browser_policy::FIELD_ID_DISCOVERY_DETECT_UNMANAGED_BROWSERS,
    |policy: &mut BrowserPolicyValue, value| {
        policy.discovery.detect_unmanaged_browsers = value;
    }
);
typed_patch_handler!(
    apply_managed_browser_mode_patch,
    BrowserPolicyManagedBrowserMode,
    constants::browser_policy::FIELD_ID_MANAGED_BROWSER_MODE,
    |policy: &mut BrowserPolicyValue, value| {
        policy.managed_browser.mode = value;
    }
);
typed_patch_handler!(
    apply_managed_browser_allowed_families_patch,
    Vec<BrowserPolicyManagedBrowserFamily>,
    constants::browser_policy::FIELD_ID_MANAGED_BROWSER_ALLOWED_FAMILIES,
    |policy: &mut BrowserPolicyValue, value| {
        policy.managed_browser.allowed_families = value;
    }
);
typed_patch_handler!(
    apply_managed_browser_launch_mode_patch,
    BrowserPolicyManagedBrowserLaunchMode,
    constants::browser_policy::FIELD_ID_MANAGED_BROWSER_LAUNCH_MODE,
    |policy: &mut BrowserPolicyValue, value| {
        policy.managed_browser.launch_mode = value;
    }
);
typed_patch_handler!(
    apply_managed_browser_profile_mode_patch,
    BrowserPolicyManagedBrowserProfileMode,
    constants::browser_policy::FIELD_ID_MANAGED_BROWSER_PROFILE_MODE,
    |policy: &mut BrowserPolicyValue, value| {
        policy.managed_browser.profile_mode = value;
    }
);
typed_patch_handler!(
    apply_managed_browser_bridge_requirements_patch,
    Vec<BrowserPolicyManagedBrowserBridgeRequirement>,
    constants::browser_policy::FIELD_ID_MANAGED_BROWSER_BRIDGE_REQUIREMENTS,
    |policy: &mut BrowserPolicyValue, value| {
        policy.managed_browser.bridge_requirements = value;
    }
);
typed_patch_handler!(
    apply_managed_browser_integration_mechanisms_patch,
    Vec<BrowserPolicyManagedBrowserIntegrationMechanism>,
    constants::browser_policy::FIELD_ID_MANAGED_BROWSER_INTEGRATION_MECHANISMS,
    |policy: &mut BrowserPolicyValue, value| {
        policy.managed_browser.integration_mechanisms = value;
    }
);
typed_patch_handler!(
    apply_managed_browser_policy_writer_controls_patch,
    Vec<BrowserPolicyManagedPolicyWriterControl>,
    constants::browser_policy::FIELD_ID_MANAGED_BROWSER_POLICY_WRITER_CONTROLS,
    |policy: &mut BrowserPolicyValue, value| {
        policy.managed_browser.policy_writer_controls = value;
    }
);
typed_patch_handler!(
    apply_managed_browser_policy_writer_fallback_patch,
    BrowserPolicyManagedPolicyWriterFallback,
    constants::browser_policy::FIELD_ID_MANAGED_BROWSER_POLICY_WRITER_FALLBACK,
    |policy: &mut BrowserPolicyValue, value| {
        policy.managed_browser.policy_writer_fallback = value;
    }
);
typed_patch_handler!(
    apply_unmanaged_browser_mode_patch,
    BrowserPolicyUnmanagedBrowserMode,
    constants::browser_policy::FIELD_ID_UNMANAGED_BROWSER_MODE,
    |policy: &mut BrowserPolicyValue, value| {
        policy.unmanaged_browser.mode = value;
    }
);
typed_patch_handler!(
    apply_unmanaged_browser_grace_seconds_patch,
    u32,
    constants::browser_policy::FIELD_ID_UNMANAGED_BROWSER_GRACE_SECONDS,
    |policy: &mut BrowserPolicyValue, value| {
        policy.unmanaged_browser.grace_seconds = value;
    }
);
typed_patch_handler!(
    apply_unmanaged_browser_allow_recover_launch_url_patch,
    bool,
    constants::browser_policy::FIELD_ID_UNMANAGED_BROWSER_ALLOW_RECOVER_LAUNCH_URL,
    |policy: &mut BrowserPolicyValue, value| {
        policy.unmanaged_browser.allow_recover_launch_url = value;
    }
);
typed_patch_handler!(
    apply_unmanaged_browser_classification_targets_patch,
    Vec<BrowserPolicyUnmanagedBrowserClassificationTarget>,
    constants::browser_policy::FIELD_ID_UNMANAGED_BROWSER_CLASSIFICATION_TARGETS,
    |policy: &mut BrowserPolicyValue, value| {
        policy.unmanaged_browser.classification_targets = value;
    }
);
typed_patch_handler!(
    apply_evidence_url_scope_patch,
    BrowserPolicyEvidenceUrlScope,
    constants::browser_policy::FIELD_ID_EVIDENCE_URL_SCOPE,
    |policy: &mut BrowserPolicyValue, value| {
        policy.evidence.url_scope = value;
    }
);
typed_patch_handler!(
    apply_required_proof_patch,
    BrowserPolicyEvidenceProofLevel,
    constants::browser_policy::FIELD_ID_REQUIRED_PROOF,
    |policy: &mut BrowserPolicyValue, value| {
        policy.evidence.required_proof = value;
    }
);
typed_patch_handler!(
    apply_proof_fallback_patch,
    Option<BrowserPolicyProofFallback>,
    constants::browser_policy::FIELD_ID_PROOF_FALLBACK,
    |policy: &mut BrowserPolicyValue, value| {
        policy.evidence.proof_fallback = value;
    }
);
typed_patch_handler!(
    apply_when_proof_unavailable_patch,
    BrowserPolicyProofFallback,
    constants::browser_policy::FIELD_ID_WHEN_PROOF_UNAVAILABLE,
    |policy: &mut BrowserPolicyValue, value| {
        policy.evidence.when_proof_unavailable = value;
    }
);
typed_patch_handler!(
    apply_evidence_never_collect_patch,
    Vec<BrowserPolicyEvidenceNeverCollect>,
    constants::browser_policy::FIELD_ID_EVIDENCE_NEVER_COLLECT,
    |policy: &mut BrowserPolicyValue, value| {
        policy.evidence.never_collect = value;
    }
);
typed_patch_handler!(
    apply_allowed_target_types_patch,
    Vec<BrowserPolicyUrlTargetType>,
    constants::browser_policy::FIELD_ID_ALLOWED_TARGET_TYPES,
    |policy: &mut BrowserPolicyValue, value| {
        policy.rules.allowed_target_types = value;
    }
);
typed_patch_handler!(
    apply_allowed_actions_patch,
    Vec<BrowserPolicyRuleAction>,
    constants::browser_policy::FIELD_ID_ALLOWED_ACTIONS,
    |policy: &mut BrowserPolicyValue, value| {
        policy.rules.allowed_actions = value;
    }
);
typed_patch_handler!(
    apply_rule_items_patch,
    Vec<BrowserPolicyRule>,
    constants::browser_policy::FIELD_ID_RULE_ITEMS,
    |policy: &mut BrowserPolicyValue, value| {
        policy.rules.items = value;
    }
);
typed_patch_handler!(
    apply_url_allow_list_patch,
    Vec<String>,
    constants::browser_policy::FIELD_ID_URL_ALLOW_LIST,
    |policy: &mut BrowserPolicyValue, value| {
        policy.rules.url_allow_list = value;
    }
);
typed_patch_handler!(
    apply_url_block_list_patch,
    Vec<String>,
    constants::browser_policy::FIELD_ID_URL_BLOCK_LIST,
    |policy: &mut BrowserPolicyValue, value| {
        policy.rules.url_block_list = value;
    }
);
typed_patch_handler!(
    apply_download_mode_patch,
    BrowserPolicyDownloadState,
    constants::browser_policy::FIELD_ID_DOWNLOAD_MODE,
    |policy: &mut BrowserPolicyValue, value| {
        policy.downloads.mode = value;
    }
);
typed_patch_handler!(
    apply_download_blocked_types_patch,
    Vec<BrowserPolicyDownloadBlockedType>,
    constants::browser_policy::FIELD_ID_DOWNLOAD_BLOCKED_TYPES,
    |policy: &mut BrowserPolicyValue, value| {
        policy.downloads.blocked_types = value;
    }
);
typed_patch_handler!(
    apply_download_state_patch,
    BrowserPolicyDownloadState,
    constants::browser_policy::FIELD_ID_DOWNLOAD_STATE,
    |policy: &mut BrowserPolicyValue, value| {
        policy.downloads.state = value;
    }
);
typed_patch_handler!(
    apply_approval_required_for_patch,
    Vec<BrowserPolicyApprovalRequiredFor>,
    constants::browser_policy::FIELD_ID_APPROVAL_REQUIRED_FOR,
    |policy: &mut BrowserPolicyValue, value| {
        policy.approvals.required_for = value;
    }
);
typed_patch_handler!(
    apply_approval_unanswered_default_patch,
    BrowserPolicyApprovalUnansweredDefault,
    constants::browser_policy::FIELD_ID_APPROVAL_UNANSWERED_DEFAULT,
    |policy: &mut BrowserPolicyValue, value| {
        policy.approvals.unanswered_default = value;
    }
);
typed_patch_handler!(
    apply_approval_state_patch,
    BrowserPolicyApprovalState,
    constants::browser_policy::FIELD_ID_APPROVAL_STATE,
    |policy: &mut BrowserPolicyValue, value| {
        policy.approvals.state = value;
    }
);
typed_patch_handler!(
    apply_browser_game_educational_mode_patch,
    BrowserPolicyBrowserGamePolicyMode,
    constants::browser_policy::FIELD_ID_BROWSER_GAME_EDUCATIONAL_MODE,
    |policy: &mut BrowserPolicyValue, value| {
        policy.browser_games.educational_game_mode = value;
    }
);
typed_patch_handler!(
    apply_browser_game_unknown_mode_patch,
    BrowserPolicyBrowserGamePolicyMode,
    constants::browser_policy::FIELD_ID_BROWSER_GAME_UNKNOWN_MODE,
    |policy: &mut BrowserPolicyValue, value| {
        policy.browser_games.unknown_game_mode = value;
    }
);
typed_patch_handler!(
    apply_browser_game_cloud_gaming_approval_patch,
    BrowserPolicyBrowserGameApprovalMode,
    constants::browser_policy::FIELD_ID_BROWSER_GAME_CLOUD_GAMING_APPROVAL,
    |policy: &mut BrowserPolicyValue, value| {
        policy.browser_games.cloud_gaming_approval = value;
    }
);
typed_patch_handler!(
    apply_browser_game_purchase_account_approval_patch,
    BrowserPolicyBrowserGameApprovalMode,
    constants::browser_policy::FIELD_ID_BROWSER_GAME_PURCHASE_ACCOUNT_APPROVAL,
    |policy: &mut BrowserPolicyValue, value| {
        policy.browser_games.purchase_account_approval = value;
    }
);
typed_patch_handler!(
    apply_browser_game_unblocked_portal_mode_patch,
    BrowserPolicyBrowserGamePolicyMode,
    constants::browser_policy::FIELD_ID_BROWSER_GAME_UNBLOCKED_PORTAL_MODE,
    |policy: &mut BrowserPolicyValue, value| {
        policy.browser_games.unblocked_portal_mode = value;
    }
);
typed_patch_handler!(
    apply_browser_game_webgl_canvas_mode_patch,
    BrowserPolicyBrowserGamePolicyMode,
    constants::browser_policy::FIELD_ID_BROWSER_GAME_WEBGL_CANVAS_MODE,
    |policy: &mut BrowserPolicyValue, value| {
        policy.browser_games.webgl_canvas_mode = value;
    }
);
typed_patch_handler!(
    apply_browser_game_daily_budget_minutes_patch,
    Option<u32>,
    constants::browser_policy::FIELD_ID_BROWSER_GAME_DAILY_BUDGET_MINUTES,
    |policy: &mut BrowserPolicyValue, value| {
        policy.browser_games.default_daily_minutes = value;
    }
);
typed_patch_handler!(
    apply_report_visible_fields_patch,
    Vec<BrowserPolicyReportVisibleField>,
    constants::browser_policy::FIELD_ID_REPORT_VISIBLE_FIELDS,
    |policy: &mut BrowserPolicyValue, value| {
        policy.reports.visible_fields = value;
    }
);
typed_patch_handler!(
    apply_report_state_patch,
    BrowserPolicyReportState,
    constants::browser_policy::FIELD_ID_REPORT_STATE,
    |policy: &mut BrowserPolicyValue, value| {
        policy.reports.state = value;
    }
);
typed_patch_handler!(
    apply_retention_exact_url_patch,
    BrowserPolicyRetentionExactUrl,
    constants::browser_policy::FIELD_ID_RETENTION_EXACT_URL,
    |policy: &mut BrowserPolicyValue, value| {
        policy.retention.exact_url = value;
    }
);
typed_patch_handler!(
    apply_retention_state_patch,
    BrowserPolicyRetentionState,
    constants::browser_policy::FIELD_ID_RETENTION_STATE,
    |policy: &mut BrowserPolicyValue, value| {
        policy.retention.state = value;
    }
);
typed_patch_handler!(
    apply_audit_state_patch,
    BrowserPolicyAuditState,
    constants::browser_policy::FIELD_ID_AUDIT_STATE,
    |policy: &mut BrowserPolicyValue, value| {
        policy.audit.state = value;
    }
);
typed_patch_handler!(
    apply_custody_allowed_uses_patch,
    Vec<BrowserPolicyCustodyAllowedUse>,
    constants::browser_policy::FIELD_ID_CUSTODY_ALLOWED_USES,
    |policy: &mut BrowserPolicyValue, value| {
        policy.custody.allowed_uses = value;
    }
);
typed_patch_handler!(
    apply_audit_required_fields_patch,
    Vec<BrowserPolicyAuditRequiredField>,
    constants::browser_policy::FIELD_ID_AUDIT_REQUIRED_FIELDS,
    |policy: &mut BrowserPolicyValue, value| {
        policy.audit.required_fields = value;
    }
);

fn require_field(
    patch: &BrowserPolicyPatch,
    expected_field_id: BrowserPolicyFieldId,
) -> Result<(), BrowserPolicyRejectionReason> {
    if patch.field_id == expected_field_id.0 {
        Ok(())
    } else {
        Err(BrowserPolicyRejectionReason::UnknownField)
    }
}

fn parse_patch_value<T>(patch: &BrowserPolicyPatch) -> Result<T, BrowserPolicyRejectionReason>
where
    T: DeserializeOwned,
{
    serde_json::from_value(patch.value.clone()).map_err(|error| {
        let _ = error;
        BrowserPolicyRejectionReason::InvalidEnumValue
    })
}

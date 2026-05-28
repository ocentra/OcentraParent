use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, BrowserPolicyApprovalRequiredFor, BrowserPolicyApprovalState,
    BrowserPolicyApprovalUnansweredDefault, BrowserPolicyAuditRequiredField,
    BrowserPolicyAuditState, BrowserPolicyBudgetCountingMode, BrowserPolicyCustodyAllowedUse,
    BrowserPolicyDefaultPosture, BrowserPolicyDownloadBlockedType, BrowserPolicyDownloadState,
    BrowserPolicyEvidenceNeverCollect, BrowserPolicyEvidenceProofLevel,
    BrowserPolicyEvidenceUrlScope, BrowserPolicyManagedBrowserBridgeRequirement,
    BrowserPolicyManagedBrowserFamily, BrowserPolicyManagedBrowserIntegrationMechanism,
    BrowserPolicyManagedBrowserLaunchMode, BrowserPolicyManagedBrowserMode,
    BrowserPolicyManagedBrowserProfileMode, BrowserPolicyManagementMode, BrowserPolicyPatch,
    BrowserPolicyProofFallback, BrowserPolicyRejectionReason, BrowserPolicyReportState,
    BrowserPolicyReportVisibleField, BrowserPolicyRetentionExactUrl, BrowserPolicyRetentionState,
    BrowserPolicyRule, BrowserPolicyRuleAction, BrowserPolicyUnmanagedBrowserClassificationTarget,
    BrowserPolicyUnmanagedBrowserMode, BrowserPolicyUpdateKind, BrowserPolicyUpdateRequest,
    BrowserPolicyUrlTargetType, BrowserPolicyValue, LogFieldValue,
};

pub(crate) fn parse_browser_policy_request(
    command: &AgentCommandEnvelope,
) -> Result<BrowserPolicyUpdateRequest, BrowserPolicyRejectionReason> {
    match command
        .payload
        .get(constants::field::BROWSER_POLICY_REQUEST)
    {
        Some(LogFieldValue::String(text)) => {
            serde_json::from_str(text).map_err(|_| BrowserPolicyRejectionReason::InvalidRequest)
        }
        _ => Err(BrowserPolicyRejectionReason::InvalidRequest),
    }
}

pub(crate) fn request_id_from_command(command: &AgentCommandEnvelope) -> String {
    match command
        .payload
        .get(constants::field::BROWSER_POLICY_REQUEST)
    {
        Some(LogFieldValue::String(text)) => serde_json::from_str::<serde_json::Value>(text)
            .ok()
            .and_then(|value| {
                value
                    .get(constants::field::BROWSER_POLICY_REQUEST_ID)
                    .and_then(|request_id| request_id.as_str().map(ToString::to_string))
            })
            .unwrap_or_else(|| command.message_id.clone()),
        _ => command.message_id.clone(),
    }
}

pub(crate) fn kind_for_command(command: &AgentCommandEnvelope) -> BrowserPolicyUpdateKind {
    match command.command {
        ocentra_parent_agent_protocol::AgentCommandName::AgentBrowserPolicyPreview => {
            BrowserPolicyUpdateKind::Preview
        }
        ocentra_parent_agent_protocol::AgentCommandName::AgentBrowserPolicyPatch => {
            BrowserPolicyUpdateKind::Patch
        }
        ocentra_parent_agent_protocol::AgentCommandName::AgentBrowserPolicyReplace => {
            BrowserPolicyUpdateKind::Replace
        }
        ocentra_parent_agent_protocol::AgentCommandName::AgentBrowserPolicyRollback => {
            BrowserPolicyUpdateKind::Rollback
        }
        _ => BrowserPolicyUpdateKind::Get,
    }
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

fn apply_browser_policy_patch(
    policy: &mut BrowserPolicyValue,
    patch: &BrowserPolicyPatch,
) -> Result<(), BrowserPolicyRejectionReason> {
    if patch.op != constants::browser_policy::PATCH_OPERATION_REPLACE {
        return Err(BrowserPolicyRejectionReason::InvalidRequest);
    }
    if apply_browser_policy_core_patch(policy, patch)? {
        return Ok(());
    }
    if apply_browser_policy_managed_browser_patch(policy, patch)? {
        return Ok(());
    }
    if apply_browser_policy_unmanaged_browser_patch(policy, patch)? {
        return Ok(());
    }
    if apply_browser_policy_evidence_patch(policy, patch)? {
        return Ok(());
    }
    if apply_browser_policy_rules_patch(policy, patch)? {
        return Ok(());
    }
    if apply_browser_policy_state_patch(policy, patch)? {
        return Ok(());
    }
    Err(BrowserPolicyRejectionReason::UnknownWritesTo)
}

fn apply_browser_policy_core_patch(
    policy: &mut BrowserPolicyValue,
    patch: &BrowserPolicyPatch,
) -> Result<bool, BrowserPolicyRejectionReason> {
    match patch.writes_to.as_str() {
        constants::browser_policy::WRITES_TO_ENABLED => {
            require_field(patch, constants::browser_policy::FIELD_ID_ENABLED)?;
            policy.enabled = parse_patch_value(patch)?;
            Ok(true)
        }
        constants::browser_policy::WRITES_TO_DEFAULT_POSTURE => {
            require_field(patch, constants::browser_policy::FIELD_ID_DEFAULT_POSTURE)?;
            policy.default_posture = parse_patch_value::<BrowserPolicyDefaultPosture>(patch)?;
            Ok(true)
        }
        constants::browser_policy::WRITES_TO_MANAGEMENT_MODE => {
            require_field(patch, constants::browser_policy::FIELD_ID_MANAGEMENT_MODE)?;
            policy.management_mode = parse_patch_value::<BrowserPolicyManagementMode>(patch)?;
            Ok(true)
        }
        constants::browser_policy::WRITES_TO_DAILY_BUDGET_MINUTES => {
            require_field(
                patch,
                constants::browser_policy::FIELD_ID_DAILY_BUDGET_MINUTES,
            )?;
            policy.budgets.default_daily_minutes = parse_patch_value::<Option<u32>>(patch)?;
            Ok(true)
        }
        constants::browser_policy::WRITES_TO_BUDGETS_ENABLED => {
            require_field(patch, constants::browser_policy::FIELD_ID_BUDGETS_ENABLED)?;
            policy.budgets.enabled = parse_patch_value::<bool>(patch)?;
            Ok(true)
        }
        constants::browser_policy::WRITES_TO_BUDGET_COUNTING_MODE => {
            require_field(
                patch,
                constants::browser_policy::FIELD_ID_BUDGET_COUNTING_MODE,
            )?;
            policy.budgets.counting_mode =
                parse_patch_value::<BrowserPolicyBudgetCountingMode>(patch)?;
            Ok(true)
        }
        _ => Ok(false),
    }
}

fn apply_browser_policy_managed_browser_patch(
    policy: &mut BrowserPolicyValue,
    patch: &BrowserPolicyPatch,
) -> Result<bool, BrowserPolicyRejectionReason> {
    match patch.writes_to.as_str() {
        constants::browser_policy::WRITES_TO_MANAGED_BROWSER_MODE => {
            require_field(
                patch,
                constants::browser_policy::FIELD_ID_MANAGED_BROWSER_MODE,
            )?;
            policy.managed_browser.mode =
                parse_patch_value::<BrowserPolicyManagedBrowserMode>(patch)?;
            Ok(true)
        }
        constants::browser_policy::WRITES_TO_MANAGED_BROWSER_ALLOWED_FAMILIES => {
            require_field(
                patch,
                constants::browser_policy::FIELD_ID_MANAGED_BROWSER_ALLOWED_FAMILIES,
            )?;
            policy.managed_browser.allowed_families =
                parse_patch_value::<Vec<BrowserPolicyManagedBrowserFamily>>(patch)?;
            Ok(true)
        }
        constants::browser_policy::WRITES_TO_MANAGED_BROWSER_LAUNCH_MODE => {
            require_field(
                patch,
                constants::browser_policy::FIELD_ID_MANAGED_BROWSER_LAUNCH_MODE,
            )?;
            policy.managed_browser.launch_mode =
                parse_patch_value::<BrowserPolicyManagedBrowserLaunchMode>(patch)?;
            Ok(true)
        }
        constants::browser_policy::WRITES_TO_MANAGED_BROWSER_PROFILE_MODE => {
            require_field(
                patch,
                constants::browser_policy::FIELD_ID_MANAGED_BROWSER_PROFILE_MODE,
            )?;
            policy.managed_browser.profile_mode =
                parse_patch_value::<BrowserPolicyManagedBrowserProfileMode>(patch)?;
            Ok(true)
        }
        constants::browser_policy::WRITES_TO_MANAGED_BROWSER_BRIDGE_REQUIREMENTS => {
            require_field(
                patch,
                constants::browser_policy::FIELD_ID_MANAGED_BROWSER_BRIDGE_REQUIREMENTS,
            )?;
            policy.managed_browser.bridge_requirements =
                parse_patch_value::<Vec<BrowserPolicyManagedBrowserBridgeRequirement>>(patch)?;
            Ok(true)
        }
        constants::browser_policy::WRITES_TO_MANAGED_BROWSER_INTEGRATION_MECHANISMS => {
            require_field(
                patch,
                constants::browser_policy::FIELD_ID_MANAGED_BROWSER_INTEGRATION_MECHANISMS,
            )?;
            policy.managed_browser.integration_mechanisms =
                parse_patch_value::<Vec<BrowserPolicyManagedBrowserIntegrationMechanism>>(patch)?;
            Ok(true)
        }
        _ => Ok(false),
    }
}

fn apply_browser_policy_unmanaged_browser_patch(
    policy: &mut BrowserPolicyValue,
    patch: &BrowserPolicyPatch,
) -> Result<bool, BrowserPolicyRejectionReason> {
    match patch.writes_to.as_str() {
        constants::browser_policy::WRITES_TO_UNMANAGED_BROWSER_MODE => {
            require_field(
                patch,
                constants::browser_policy::FIELD_ID_UNMANAGED_BROWSER_MODE,
            )?;
            policy.unmanaged_browser.mode =
                parse_patch_value::<BrowserPolicyUnmanagedBrowserMode>(patch)?;
            Ok(true)
        }
        constants::browser_policy::WRITES_TO_UNMANAGED_BROWSER_GRACE_SECONDS => {
            require_field(
                patch,
                constants::browser_policy::FIELD_ID_UNMANAGED_BROWSER_GRACE_SECONDS,
            )?;
            policy.unmanaged_browser.grace_seconds = parse_patch_value::<u32>(patch)?;
            Ok(true)
        }
        constants::browser_policy::WRITES_TO_UNMANAGED_BROWSER_ALLOW_RECOVER_LAUNCH_URL => {
            require_field(
                patch,
                constants::browser_policy::FIELD_ID_UNMANAGED_BROWSER_ALLOW_RECOVER_LAUNCH_URL,
            )?;
            policy.unmanaged_browser.allow_recover_launch_url = parse_patch_value::<bool>(patch)?;
            Ok(true)
        }
        constants::browser_policy::WRITES_TO_UNMANAGED_BROWSER_CLASSIFICATION_TARGETS => {
            require_field(
                patch,
                constants::browser_policy::FIELD_ID_UNMANAGED_BROWSER_CLASSIFICATION_TARGETS,
            )?;
            policy.unmanaged_browser.classification_targets =
                parse_patch_value::<Vec<BrowserPolicyUnmanagedBrowserClassificationTarget>>(patch)?;
            Ok(true)
        }
        _ => Ok(false),
    }
}

fn apply_browser_policy_evidence_patch(
    policy: &mut BrowserPolicyValue,
    patch: &BrowserPolicyPatch,
) -> Result<bool, BrowserPolicyRejectionReason> {
    match patch.writes_to.as_str() {
        constants::browser_policy::WRITES_TO_EVIDENCE_URL_SCOPE => {
            require_field(
                patch,
                constants::browser_policy::FIELD_ID_EVIDENCE_URL_SCOPE,
            )?;
            policy.evidence.url_scope = parse_patch_value::<BrowserPolicyEvidenceUrlScope>(patch)?;
            Ok(true)
        }
        constants::browser_policy::WRITES_TO_REQUIRED_PROOF => {
            require_field(patch, constants::browser_policy::FIELD_ID_REQUIRED_PROOF)?;
            policy.evidence.required_proof =
                parse_patch_value::<BrowserPolicyEvidenceProofLevel>(patch)?;
            Ok(true)
        }
        constants::browser_policy::WRITES_TO_PROOF_FALLBACK => {
            require_field(patch, constants::browser_policy::FIELD_ID_PROOF_FALLBACK)?;
            policy.evidence.proof_fallback =
                parse_patch_value::<Option<BrowserPolicyProofFallback>>(patch)?;
            Ok(true)
        }
        constants::browser_policy::WRITES_TO_WHEN_PROOF_UNAVAILABLE => {
            require_field(
                patch,
                constants::browser_policy::FIELD_ID_WHEN_PROOF_UNAVAILABLE,
            )?;
            policy.evidence.when_proof_unavailable =
                parse_patch_value::<BrowserPolicyProofFallback>(patch)?;
            Ok(true)
        }
        constants::browser_policy::WRITES_TO_EVIDENCE_NEVER_COLLECT => {
            require_field(
                patch,
                constants::browser_policy::FIELD_ID_EVIDENCE_NEVER_COLLECT,
            )?;
            policy.evidence.never_collect =
                parse_patch_value::<Vec<BrowserPolicyEvidenceNeverCollect>>(patch)?;
            Ok(true)
        }
        _ => Ok(false),
    }
}

fn apply_browser_policy_rules_patch(
    policy: &mut BrowserPolicyValue,
    patch: &BrowserPolicyPatch,
) -> Result<bool, BrowserPolicyRejectionReason> {
    match patch.writes_to.as_str() {
        constants::browser_policy::WRITES_TO_ALLOWED_TARGET_TYPES => {
            require_field(
                patch,
                constants::browser_policy::FIELD_ID_ALLOWED_TARGET_TYPES,
            )?;
            policy.rules.allowed_target_types =
                parse_patch_value::<Vec<BrowserPolicyUrlTargetType>>(patch)?;
            Ok(true)
        }
        constants::browser_policy::WRITES_TO_ALLOWED_ACTIONS => {
            require_field(patch, constants::browser_policy::FIELD_ID_ALLOWED_ACTIONS)?;
            policy.rules.allowed_actions =
                parse_patch_value::<Vec<BrowserPolicyRuleAction>>(patch)?;
            Ok(true)
        }
        constants::browser_policy::WRITES_TO_RULE_ITEMS => {
            require_field(patch, constants::browser_policy::FIELD_ID_RULE_ITEMS)?;
            policy.rules.items = parse_patch_value::<Vec<BrowserPolicyRule>>(patch)?;
            Ok(true)
        }
        _ => Ok(false),
    }
}

fn apply_browser_policy_state_patch(
    policy: &mut BrowserPolicyValue,
    patch: &BrowserPolicyPatch,
) -> Result<bool, BrowserPolicyRejectionReason> {
    if apply_browser_policy_download_patch(policy, patch)? {
        return Ok(true);
    }
    if apply_browser_policy_approval_patch(policy, patch)? {
        return Ok(true);
    }
    if apply_browser_policy_report_retention_patch(policy, patch)? {
        return Ok(true);
    }
    if apply_browser_policy_audit_custody_patch(policy, patch)? {
        return Ok(true);
    }
    Ok(false)
}

fn apply_browser_policy_download_patch(
    policy: &mut BrowserPolicyValue,
    patch: &BrowserPolicyPatch,
) -> Result<bool, BrowserPolicyRejectionReason> {
    match patch.writes_to.as_str() {
        constants::browser_policy::WRITES_TO_DOWNLOAD_MODE => {
            require_field(patch, constants::browser_policy::FIELD_ID_DOWNLOAD_MODE)?;
            policy.downloads.mode = parse_patch_value::<BrowserPolicyDownloadState>(patch)?;
            Ok(true)
        }
        constants::browser_policy::WRITES_TO_DOWNLOAD_BLOCKED_TYPES => {
            require_field(
                patch,
                constants::browser_policy::FIELD_ID_DOWNLOAD_BLOCKED_TYPES,
            )?;
            policy.downloads.blocked_types =
                parse_patch_value::<Vec<BrowserPolicyDownloadBlockedType>>(patch)?;
            Ok(true)
        }
        constants::browser_policy::WRITES_TO_DOWNLOAD_STATE => {
            require_field(patch, constants::browser_policy::FIELD_ID_DOWNLOAD_STATE)?;
            policy.downloads.state = parse_patch_value::<BrowserPolicyDownloadState>(patch)?;
            Ok(true)
        }
        _ => Ok(false),
    }
}

fn apply_browser_policy_approval_patch(
    policy: &mut BrowserPolicyValue,
    patch: &BrowserPolicyPatch,
) -> Result<bool, BrowserPolicyRejectionReason> {
    match patch.writes_to.as_str() {
        constants::browser_policy::WRITES_TO_APPROVAL_REQUIRED_FOR => {
            require_field(
                patch,
                constants::browser_policy::FIELD_ID_APPROVAL_REQUIRED_FOR,
            )?;
            policy.approvals.required_for =
                parse_patch_value::<Vec<BrowserPolicyApprovalRequiredFor>>(patch)?;
            Ok(true)
        }
        constants::browser_policy::WRITES_TO_APPROVAL_UNANSWERED_DEFAULT => {
            require_field(
                patch,
                constants::browser_policy::FIELD_ID_APPROVAL_UNANSWERED_DEFAULT,
            )?;
            policy.approvals.unanswered_default =
                parse_patch_value::<BrowserPolicyApprovalUnansweredDefault>(patch)?;
            Ok(true)
        }
        constants::browser_policy::WRITES_TO_APPROVAL_STATE => {
            require_field(patch, constants::browser_policy::FIELD_ID_APPROVAL_STATE)?;
            policy.approvals.state = parse_patch_value::<BrowserPolicyApprovalState>(patch)?;
            Ok(true)
        }
        _ => Ok(false),
    }
}

fn apply_browser_policy_report_retention_patch(
    policy: &mut BrowserPolicyValue,
    patch: &BrowserPolicyPatch,
) -> Result<bool, BrowserPolicyRejectionReason> {
    match patch.writes_to.as_str() {
        constants::browser_policy::WRITES_TO_REPORT_VISIBLE_FIELDS => {
            require_field(
                patch,
                constants::browser_policy::FIELD_ID_REPORT_VISIBLE_FIELDS,
            )?;
            policy.reports.visible_fields =
                parse_patch_value::<Vec<BrowserPolicyReportVisibleField>>(patch)?;
            Ok(true)
        }
        constants::browser_policy::WRITES_TO_REPORT_STATE => {
            require_field(patch, constants::browser_policy::FIELD_ID_REPORT_STATE)?;
            policy.reports.state = parse_patch_value::<BrowserPolicyReportState>(patch)?;
            Ok(true)
        }
        constants::browser_policy::WRITES_TO_RETENTION_EXACT_URL => {
            require_field(
                patch,
                constants::browser_policy::FIELD_ID_RETENTION_EXACT_URL,
            )?;
            policy.retention.exact_url =
                parse_patch_value::<BrowserPolicyRetentionExactUrl>(patch)?;
            Ok(true)
        }
        constants::browser_policy::WRITES_TO_RETENTION_STATE => {
            require_field(patch, constants::browser_policy::FIELD_ID_RETENTION_STATE)?;
            policy.retention.state = parse_patch_value::<BrowserPolicyRetentionState>(patch)?;
            Ok(true)
        }
        _ => Ok(false),
    }
}

fn apply_browser_policy_audit_custody_patch(
    policy: &mut BrowserPolicyValue,
    patch: &BrowserPolicyPatch,
) -> Result<bool, BrowserPolicyRejectionReason> {
    match patch.writes_to.as_str() {
        constants::browser_policy::WRITES_TO_AUDIT_STATE => {
            require_field(patch, constants::browser_policy::FIELD_ID_AUDIT_STATE)?;
            policy.audit.state = parse_patch_value::<BrowserPolicyAuditState>(patch)?;
            Ok(true)
        }
        constants::browser_policy::WRITES_TO_CUSTODY_ALLOWED_USES => {
            require_field(
                patch,
                constants::browser_policy::FIELD_ID_CUSTODY_ALLOWED_USES,
            )?;
            policy.custody.allowed_uses =
                parse_patch_value::<Vec<BrowserPolicyCustodyAllowedUse>>(patch)?;
            Ok(true)
        }
        constants::browser_policy::WRITES_TO_AUDIT_REQUIRED_FIELDS => {
            require_field(
                patch,
                constants::browser_policy::FIELD_ID_AUDIT_REQUIRED_FIELDS,
            )?;
            policy.audit.required_fields =
                parse_patch_value::<Vec<BrowserPolicyAuditRequiredField>>(patch)?;
            Ok(true)
        }
        _ => Ok(false),
    }
}

fn require_field(
    patch: &BrowserPolicyPatch,
    expected_field_id: &str,
) -> Result<(), BrowserPolicyRejectionReason> {
    if patch.field_id == expected_field_id {
        Ok(())
    } else {
        Err(BrowserPolicyRejectionReason::UnknownField)
    }
}

fn parse_patch_value<T>(patch: &BrowserPolicyPatch) -> Result<T, BrowserPolicyRejectionReason>
where
    T: serde::de::DeserializeOwned,
{
    serde_json::from_value(patch.value.clone())
        .map_err(|_| BrowserPolicyRejectionReason::InvalidEnumValue)
}

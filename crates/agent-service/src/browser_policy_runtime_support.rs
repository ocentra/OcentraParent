use ocentra_parent_agent_protocol::{
    constants, policy_constants, BrowserPolicyApprovalState, BrowserPolicyApprovals,
    BrowserPolicyAudit, BrowserPolicyAuditState, BrowserPolicyBudgets, BrowserPolicyDefaultPosture,
    BrowserPolicyDownloadState, BrowserPolicyDownloads, BrowserPolicyEffectivePolicy,
    BrowserPolicyEvidenceProofLevel, BrowserPolicyEvidenceRequirement, BrowserPolicyManagedBrowser,
    BrowserPolicyManagedBrowserMode, BrowserPolicyManagementMode, BrowserPolicyRejectionReason,
    BrowserPolicyReportState, BrowserPolicyReports, BrowserPolicyRetention,
    BrowserPolicyRetentionState, BrowserPolicyRules, BrowserPolicyUnmanagedBrowser,
    BrowserPolicyUnmanagedBrowserMode, BrowserPolicyUpdateKind, BrowserPolicyUpdateResponse,
    BrowserPolicyUpdateStatus, BrowserPolicyUrlTargetType, BrowserPolicyValue,
};

use crate::{
    browser_policy_compiler::browser_policy_capability_registry,
    browser_policy_store::BrowserPolicyStoredState,
};

pub(crate) fn accepted_response(
    request_id: String,
    kind: BrowserPolicyUpdateKind,
    policy: BrowserPolicyValue,
    effective_policy: BrowserPolicyEffectivePolicy,
    audit_event_id: Option<String>,
    message: &str,
    generated_at: &str,
) -> BrowserPolicyUpdateResponse {
    BrowserPolicyUpdateResponse {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        request_id,
        kind,
        status: BrowserPolicyUpdateStatus::Accepted,
        policy: Some(policy),
        effective_policy: Some(effective_policy),
        capability_registry: Some(browser_policy_capability_registry(generated_at)),
        rejection_reason: None,
        audit_event_id,
        message: Some(message.to_string()),
    }
}

pub(crate) fn rejected_response(
    request_id: String,
    kind: BrowserPolicyUpdateKind,
    rejection_reason: BrowserPolicyRejectionReason,
    message: &str,
    generated_at: &str,
) -> BrowserPolicyUpdateResponse {
    BrowserPolicyUpdateResponse {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        request_id,
        kind,
        status: BrowserPolicyUpdateStatus::Rejected,
        policy: None,
        effective_policy: None,
        capability_registry: Some(browser_policy_capability_registry(generated_at)),
        rejection_reason: Some(rejection_reason),
        audit_event_id: None,
        message: Some(message.to_string()),
    }
}

pub(crate) fn base_revision_matches(
    state: &BrowserPolicyStoredState,
    base_revision_id: Option<&str>,
) -> Result<(), BrowserPolicyRejectionReason> {
    match (state.active_revision_id.as_deref(), base_revision_id) {
        (None, None) => Ok(()),
        (Some(active), Some(base)) if active == base => Ok(()),
        (Some(_), Some(_)) => Err(BrowserPolicyRejectionReason::StaleRevision),
        _ => Err(BrowserPolicyRejectionReason::RevisionNotFound),
    }
}

pub(crate) fn next_revision_id(state: &BrowserPolicyStoredState) -> String {
    let mut revision_id = constants::browser_policy::REVISION_PREFIX.to_string();
    revision_id.push_str(&(state.revisions.len() + 1).to_string());
    revision_id
}

pub(crate) fn next_audit_event_id(state: &BrowserPolicyStoredState) -> String {
    let mut audit_event_id = constants::browser_policy::AUDIT_PREFIX.to_string();
    audit_event_id.push_str(&(state.audit_events.len() + 1).to_string());
    audit_event_id
}

pub(crate) fn preview_revision_id() -> String {
    let mut revision_id = constants::browser_policy::REVISION_PREFIX.to_string();
    revision_id.push_str(constants::browser_policy::UPDATE_KIND_PREVIEW);
    revision_id
}

pub(crate) fn default_revision_id() -> String {
    let mut revision_id = constants::browser_policy::REVISION_PREFIX.to_string();
    revision_id.push_str(constants::browser_policy::UPDATE_KIND_GET);
    revision_id
}

pub(crate) fn default_policy(policy_id: String) -> BrowserPolicyValue {
    BrowserPolicyValue {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        policy_id,
        enabled: false,
        default_posture: BrowserPolicyDefaultPosture::Observe,
        fallback_posture: None,
        management_mode: BrowserPolicyManagementMode::Disabled,
        managed_browser: BrowserPolicyManagedBrowser {
            mode: BrowserPolicyManagedBrowserMode::NotRequired,
        },
        unmanaged_browser: BrowserPolicyUnmanagedBrowser {
            mode: BrowserPolicyUnmanagedBrowserMode::ObserveOnly,
        },
        evidence: BrowserPolicyEvidenceRequirement {
            required_proof: BrowserPolicyEvidenceProofLevel::None,
            proof_fallback: None,
        },
        rules: BrowserPolicyRules {
            allowed_target_types: vec![BrowserPolicyUrlTargetType::Domain],
            entries: Vec::new(),
        },
        budgets: BrowserPolicyBudgets {
            default_daily_minutes: None,
        },
        downloads: BrowserPolicyDownloads {
            state: BrowserPolicyDownloadState::NotConfigured,
        },
        approvals: BrowserPolicyApprovals {
            state: BrowserPolicyApprovalState::NotRequired,
        },
        reports: BrowserPolicyReports {
            state: BrowserPolicyReportState::Disabled,
        },
        audit: BrowserPolicyAudit {
            state: BrowserPolicyAuditState::LocalOnly,
        },
        retention: BrowserPolicyRetention {
            state: BrowserPolicyRetentionState::None,
        },
    }
}

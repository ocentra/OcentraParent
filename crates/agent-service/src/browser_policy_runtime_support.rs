use ocentra_parent_agent_protocol::{
    constants, policy_constants, BrowserPolicyApprovalState, BrowserPolicyApprovals,
    BrowserPolicyAudit, BrowserPolicyAuditState, BrowserPolicyBudgets, BrowserPolicyChildFacing,
    BrowserPolicyCustody, BrowserPolicyDefaultPosture, BrowserPolicyDownloadBlockedType,
    BrowserPolicyDownloadState, BrowserPolicyDownloads, BrowserPolicyEffectivePolicy,
    BrowserPolicyEvidenceNeverCollect, BrowserPolicyEvidenceProofLevel,
    BrowserPolicyEvidenceRequirement, BrowserPolicyEvidenceUrlScope, BrowserPolicyFallbacks,
    BrowserPolicyManagedBrowser, BrowserPolicyManagedBrowserBridgeRequirement,
    BrowserPolicyManagedBrowserFamily, BrowserPolicyManagedBrowserIntegrationMechanism,
    BrowserPolicyManagedBrowserLaunchMode, BrowserPolicyManagedBrowserMode,
    BrowserPolicyManagedBrowserProfileMode, BrowserPolicyManagementMode, BrowserPolicyPlatforms,
    BrowserPolicyPortalAi, BrowserPolicyProofFallback, BrowserPolicyRejectionReason,
    BrowserPolicyReportState, BrowserPolicyReports, BrowserPolicyRetention,
    BrowserPolicyRetentionExactUrl, BrowserPolicyRetentionState, BrowserPolicyRuleAction,
    BrowserPolicyRules, BrowserPolicyUnmanagedBrowser,
    BrowserPolicyUnmanagedBrowserClassificationTarget, BrowserPolicyUnmanagedBrowserMode,
    BrowserPolicyUpdateKind, BrowserPolicyUpdateResponse, BrowserPolicyUpdateStatus,
    BrowserPolicyUrlTargetType, BrowserPolicyValue,
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
        management_mode: BrowserPolicyManagementMode::LocalChildAgent,
        managed_browser: default_managed_browser(),
        unmanaged_browser: default_unmanaged_browser(),
        evidence: default_evidence_requirement(),
        rules: default_rules(),
        budgets: BrowserPolicyBudgets {
            enabled: true,
            default_daily_minutes: None,
            counting_mode: Default::default(),
        },
        downloads: BrowserPolicyDownloads {
            mode: BrowserPolicyDownloadState::Observe,
            blocked_types: vec![
                BrowserPolicyDownloadBlockedType::Executable,
                BrowserPolicyDownloadBlockedType::Script,
                BrowserPolicyDownloadBlockedType::Unknown,
            ],
            state: BrowserPolicyDownloadState::Observe,
        },
        approvals: BrowserPolicyApprovals {
            required_for: Vec::new(),
            unanswered_default: Default::default(),
            state: BrowserPolicyApprovalState::NotRequired,
        },
        reports: BrowserPolicyReports {
            visible_fields: Vec::new(),
            state: BrowserPolicyReportState::Disabled,
        },
        audit: BrowserPolicyAudit {
            required_fields: Vec::new(),
            state: BrowserPolicyAuditState::LocalOnly,
            plan: Default::default(),
        },
        retention: BrowserPolicyRetention {
            exact_url: BrowserPolicyRetentionExactUrl::SevenDays,
            state: BrowserPolicyRetentionState::None,
        },
        custody: BrowserPolicyCustody::default(),
        schedules: Vec::new(),
        child_facing: BrowserPolicyChildFacing::default(),
        portal_ai: BrowserPolicyPortalAi::default(),
        platforms: BrowserPolicyPlatforms::default(),
        fallbacks: BrowserPolicyFallbacks::default(),
    }
}

fn default_managed_browser() -> BrowserPolicyManagedBrowser {
    BrowserPolicyManagedBrowser {
        mode: BrowserPolicyManagedBrowserMode::AvailableForExactRules,
        allowed_families: vec![
            BrowserPolicyManagedBrowserFamily::EdgeStable,
            BrowserPolicyManagedBrowserFamily::ChromeStable,
            BrowserPolicyManagedBrowserFamily::ChromeForTesting,
        ],
        launch_mode: BrowserPolicyManagedBrowserLaunchMode::OcentraLauncher,
        profile_mode: BrowserPolicyManagedBrowserProfileMode::PersistentManagedProfile,
        bridge_requirements: default_bridge_requirements(),
        integration_mechanisms: default_integration_mechanisms(),
    }
}

fn default_bridge_requirements() -> Vec<BrowserPolicyManagedBrowserBridgeRequirement> {
    vec![
        BrowserPolicyManagedBrowserBridgeRequirement::OwnedProfile,
        BrowserPolicyManagedBrowserBridgeRequirement::LoopbackOnly,
        BrowserPolicyManagedBrowserBridgeRequirement::RandomPort,
        BrowserPolicyManagedBrowserBridgeRequirement::RejectDefaultProfile,
        BrowserPolicyManagedBrowserBridgeRequirement::RejectUnmanagedProfile,
        BrowserPolicyManagedBrowserBridgeRequirement::RedactedRefs,
        BrowserPolicyManagedBrowserBridgeRequirement::CloseOnSessionEnd,
        BrowserPolicyManagedBrowserBridgeRequirement::DegradeSafely,
    ]
}

fn default_integration_mechanisms() -> Vec<BrowserPolicyManagedBrowserIntegrationMechanism> {
    vec![
        BrowserPolicyManagedBrowserIntegrationMechanism::ChromiumCdp,
        BrowserPolicyManagedBrowserIntegrationMechanism::ManagedExtensionNativeHost,
        BrowserPolicyManagedBrowserIntegrationMechanism::BrowserPolicy,
    ]
}

fn default_unmanaged_browser() -> BrowserPolicyUnmanagedBrowser {
    BrowserPolicyUnmanagedBrowser {
        mode: BrowserPolicyUnmanagedBrowserMode::Monitor,
        grace_seconds: 0,
        allow_recover_launch_url: true,
        classification_targets: vec![
            BrowserPolicyUnmanagedBrowserClassificationTarget::KnownBrowser,
            BrowserPolicyUnmanagedBrowserClassificationTarget::PortableBrowser,
            BrowserPolicyUnmanagedBrowserClassificationTarget::RenamedBrowser,
            BrowserPolicyUnmanagedBrowserClassificationTarget::BrowserLikeProcess,
            BrowserPolicyUnmanagedBrowserClassificationTarget::PrivateOrTor,
        ],
    }
}

fn default_evidence_requirement() -> BrowserPolicyEvidenceRequirement {
    BrowserPolicyEvidenceRequirement {
        url_scope: BrowserPolicyEvidenceUrlScope::DomainOriginTitle,
        required_proof: BrowserPolicyEvidenceProofLevel::FreshManagedActiveTab,
        proof_fallback: None,
        when_proof_unavailable: BrowserPolicyProofFallback::Ask,
        never_collect: default_never_collect(),
    }
}

fn default_never_collect() -> Vec<BrowserPolicyEvidenceNeverCollect> {
    vec![
        BrowserPolicyEvidenceNeverCollect::PageBody,
        BrowserPolicyEvidenceNeverCollect::ChatContent,
        BrowserPolicyEvidenceNeverCollect::Screenshots,
        BrowserPolicyEvidenceNeverCollect::Keystrokes,
        BrowserPolicyEvidenceNeverCollect::FormValues,
        BrowserPolicyEvidenceNeverCollect::Secrets,
        BrowserPolicyEvidenceNeverCollect::DecryptedHttpsPayload,
        BrowserPolicyEvidenceNeverCollect::RawProtocolDumps,
    ]
}

fn default_rules() -> BrowserPolicyRules {
    BrowserPolicyRules {
        allowed_target_types: vec![
            BrowserPolicyUrlTargetType::ExactUrl,
            BrowserPolicyUrlTargetType::DomainOrigin,
            BrowserPolicyUrlTargetType::SiteCategory,
            BrowserPolicyUrlTargetType::BrowserSession,
            BrowserPolicyUrlTargetType::BrowserProcess,
            BrowserPolicyUrlTargetType::CapabilityState,
        ],
        allowed_actions: vec![
            BrowserPolicyRuleAction::Allow,
            BrowserPolicyRuleAction::Warn,
            BrowserPolicyRuleAction::Ask,
            BrowserPolicyRuleAction::Limit,
            BrowserPolicyRuleAction::Block,
        ],
        items: Vec::new(),
        entries: Vec::new(),
    }
}

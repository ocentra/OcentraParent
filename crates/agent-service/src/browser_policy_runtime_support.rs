use std::path::PathBuf;

use ocentra_parent_agent_protocol::browser_policy::BrowserPolicyRejectionReason;
use ocentra_parent_agent_protocol::browser_policy::BrowserPolicyUpdateKind;
use ocentra_parent_agent_protocol::browser_policy::BrowserPolicyUpdateResponse;
use ocentra_parent_agent_protocol::browser_policy::BrowserPolicyUpdateStatus;
use ocentra_parent_agent_protocol::browser_policy_catalog_values::BrowserPolicyDownloadBlockedType;
use ocentra_parent_agent_protocol::browser_policy_catalog_values::BrowserPolicyEvidenceNeverCollect;
use ocentra_parent_agent_protocol::browser_policy_catalog_values::BrowserPolicyEvidenceUrlScope;
use ocentra_parent_agent_protocol::browser_policy_catalog_values::BrowserPolicyManagedBrowserBridgeRequirement;
use ocentra_parent_agent_protocol::browser_policy_catalog_values::BrowserPolicyManagedBrowserFamily;
use ocentra_parent_agent_protocol::browser_policy_catalog_values::BrowserPolicyManagedBrowserIntegrationMechanism;
use ocentra_parent_agent_protocol::browser_policy_catalog_values::BrowserPolicyManagedBrowserLaunchMode;
use ocentra_parent_agent_protocol::browser_policy_catalog_values::BrowserPolicyManagedBrowserProfileMode;
use ocentra_parent_agent_protocol::browser_policy_catalog_values::BrowserPolicyRetentionExactUrl;
use ocentra_parent_agent_protocol::browser_policy_catalog_values::BrowserPolicyRuleAction;
use ocentra_parent_agent_protocol::browser_policy_catalog_values::BrowserPolicyUnmanagedBrowserClassificationTarget;
use ocentra_parent_agent_protocol::browser_policy_model::BrowserPolicyApprovals;
use ocentra_parent_agent_protocol::browser_policy_model::BrowserPolicyAudit;
use ocentra_parent_agent_protocol::browser_policy_model::BrowserPolicyBrowserGames;
use ocentra_parent_agent_protocol::browser_policy_model::BrowserPolicyBudgets;
use ocentra_parent_agent_protocol::browser_policy_model::BrowserPolicyDiscovery;
use ocentra_parent_agent_protocol::browser_policy_model::BrowserPolicyDownloads;
use ocentra_parent_agent_protocol::browser_policy_model::BrowserPolicyEffectivePolicy;
use ocentra_parent_agent_protocol::browser_policy_model::BrowserPolicyEvidenceRequirement;
use ocentra_parent_agent_protocol::browser_policy_model::BrowserPolicyManagedBrowser;
use ocentra_parent_agent_protocol::browser_policy_model::BrowserPolicyReports;
use ocentra_parent_agent_protocol::browser_policy_model::BrowserPolicyRetention;
use ocentra_parent_agent_protocol::browser_policy_model::BrowserPolicyRules;
use ocentra_parent_agent_protocol::browser_policy_model::BrowserPolicyUnmanagedBrowser;
use ocentra_parent_agent_protocol::browser_policy_model::BrowserPolicyValue;
use ocentra_parent_agent_protocol::browser_policy_sections::BrowserPolicyChildFacing;
use ocentra_parent_agent_protocol::browser_policy_sections::BrowserPolicyCustody;
use ocentra_parent_agent_protocol::browser_policy_sections::BrowserPolicyFallbacks;
use ocentra_parent_agent_protocol::browser_policy_sections::BrowserPolicyPlatforms;
use ocentra_parent_agent_protocol::browser_policy_sections::BrowserPolicyPortalAi;
use ocentra_parent_agent_protocol::browser_policy_values::BrowserPolicyApprovalState;
use ocentra_parent_agent_protocol::browser_policy_values::BrowserPolicyAuditState;
use ocentra_parent_agent_protocol::browser_policy_values::BrowserPolicyDefaultPosture;
use ocentra_parent_agent_protocol::browser_policy_values::BrowserPolicyDownloadState;
use ocentra_parent_agent_protocol::browser_policy_values::BrowserPolicyEvidenceProofLevel;
use ocentra_parent_agent_protocol::browser_policy_values::BrowserPolicyManagedBrowserMode;
use ocentra_parent_agent_protocol::browser_policy_values::BrowserPolicyManagementMode;
use ocentra_parent_agent_protocol::browser_policy_values::BrowserPolicyProofFallback;
use ocentra_parent_agent_protocol::browser_policy_values::BrowserPolicyReportState;
use ocentra_parent_agent_protocol::browser_policy_values::BrowserPolicyRetentionState;
use ocentra_parent_agent_protocol::browser_policy_values::BrowserPolicyUnmanagedBrowserMode;
use ocentra_parent_agent_protocol::browser_policy_values::BrowserPolicyUrlTargetType;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::policy_constants;

use crate::{
    browser_policy_compiler::browser_policy_capability_registry,
    browser_policy_store::BrowserPolicyStoredState,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct BrowserPolicyRequestId(pub(crate) String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct BrowserPolicyPolicyId(pub(crate) String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct BrowserPolicyRevisionId(pub(crate) String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct BrowserPolicyAuditEventId(pub(crate) String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct BrowserPolicyTimestamp(pub(crate) String);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct BrowserPolicyMessage(pub(crate) &'static str);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct BrowserPolicyStorePath(pub(crate) PathBuf);

pub(crate) fn accepted_response(
    request_id: BrowserPolicyRequestId,
    kind: BrowserPolicyUpdateKind,
    policy: BrowserPolicyValue,
    effective_policy: BrowserPolicyEffectivePolicy,
    audit_event_id: Option<BrowserPolicyAuditEventId>,
    message: BrowserPolicyMessage,
    generated_at: BrowserPolicyTimestamp,
) -> BrowserPolicyUpdateResponse {
    let BrowserPolicyTimestamp(generated_at) = generated_at;
    BrowserPolicyUpdateResponse {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        request_id: request_id.0,
        kind,
        status: BrowserPolicyUpdateStatus::Accepted,
        policy: Some(policy),
        effective_policy: Some(effective_policy),
        capability_registry: Some(browser_policy_capability_registry(
            crate::browser_policy_compiler::BrowserPolicyCapabilityRegistryRequest {
                generated_at: &generated_at,
            },
        )),
        rejection_reason: None,
        audit_event_id: audit_event_id.map(|value| value.0),
        message: Some(message.0.to_string()),
    }
}

pub(crate) fn rejected_response(
    request_id: BrowserPolicyRequestId,
    kind: BrowserPolicyUpdateKind,
    rejection_reason: BrowserPolicyRejectionReason,
    message: BrowserPolicyMessage,
    generated_at: BrowserPolicyTimestamp,
) -> BrowserPolicyUpdateResponse {
    let BrowserPolicyTimestamp(generated_at) = generated_at;
    BrowserPolicyUpdateResponse {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        request_id: request_id.0,
        kind,
        status: BrowserPolicyUpdateStatus::Rejected,
        policy: None,
        effective_policy: None,
        capability_registry: Some(browser_policy_capability_registry(
            crate::browser_policy_compiler::BrowserPolicyCapabilityRegistryRequest {
                generated_at: &generated_at,
            },
        )),
        rejection_reason: Some(rejection_reason),
        audit_event_id: None,
        message: Some(message.0.to_string()),
    }
}

pub(crate) fn base_revision_matches(
    state: &BrowserPolicyStoredState,
    base_revision_id: Option<&BrowserPolicyRevisionId>,
) -> Result<(), BrowserPolicyRejectionReason> {
    match (
        state.active_revision_id.as_deref(),
        base_revision_id.map(|revision_id| revision_id.0.as_str()),
    ) {
        (None, None) => Ok(()),
        (Some(active), Some(base)) if active == base => Ok(()),
        (Some(_), Some(_)) => Err(BrowserPolicyRejectionReason::StaleRevision),
        _ => Err(BrowserPolicyRejectionReason::RevisionNotFound),
    }
}

pub(crate) fn next_revision_id(state: &BrowserPolicyStoredState) -> BrowserPolicyRevisionId {
    let mut revision_id = constants::browser_policy::REVISION_PREFIX.to_string();
    revision_id.push_str(&(state.revisions.len() + 1).to_string());
    BrowserPolicyRevisionId(revision_id)
}

pub(crate) fn next_audit_event_id(state: &BrowserPolicyStoredState) -> BrowserPolicyAuditEventId {
    let mut audit_event_id = constants::browser_policy::AUDIT_PREFIX.to_string();
    audit_event_id.push_str(&(state.audit_events.len() + 1).to_string());
    BrowserPolicyAuditEventId(audit_event_id)
}

pub(crate) fn preview_revision_id() -> BrowserPolicyRevisionId {
    let mut revision_id = constants::browser_policy::REVISION_PREFIX.to_string();
    revision_id.push_str(constants::browser_policy::UPDATE_KIND_PREVIEW);
    BrowserPolicyRevisionId(revision_id)
}

pub(crate) fn default_revision_id() -> BrowserPolicyRevisionId {
    let mut revision_id = constants::browser_policy::REVISION_PREFIX.to_string();
    revision_id.push_str(constants::browser_policy::UPDATE_KIND_GET);
    BrowserPolicyRevisionId(revision_id)
}

pub(crate) fn default_policy(policy_id: BrowserPolicyPolicyId) -> BrowserPolicyValue {
    BrowserPolicyValue {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        policy_id: policy_id.0,
        enabled: false,
        execution_mode: Default::default(),
        default_posture: BrowserPolicyDefaultPosture::Observe,
        fallback_posture: None,
        management_mode: BrowserPolicyManagementMode::LocalChildAgent,
        discovery: default_discovery(),
        managed_browser: default_managed_browser(),
        unmanaged_browser: default_unmanaged_browser(),
        evidence: default_evidence_requirement(),
        rules: default_rules(),
        budgets: BrowserPolicyBudgets {
            enabled: true,
            default_daily_minutes: None,
            counting_mode: Default::default(),
        },
        browser_games: BrowserPolicyBrowserGames::default(),
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

pub(crate) fn browser_policy_store_path_from_env() -> BrowserPolicyStorePath {
    BrowserPolicyStorePath(
        std::env::var(constants::env_var::AGENT_BROWSER_POLICY_STORE_PATH)
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                let mut path = std::env::temp_dir();
                path.push(constants::browser_policy::STORE_FILE_NAME);
                path
            }),
    )
}

fn default_discovery() -> BrowserPolicyDiscovery {
    BrowserPolicyDiscovery {
        scan_installed_browsers: false,
        scan_running_browsers: true,
        detect_unmanaged_browsers: true,
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
        policy_writer_controls: Vec::new(),
        policy_writer_fallback: Default::default(),
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
        mode: BrowserPolicyUnmanagedBrowserMode::ReportOnly,
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
            BrowserPolicyRuleAction::TerminateProcess,
            BrowserPolicyRuleAction::RelaunchManaged,
        ],
        items: Vec::new(),
        entries: Vec::new(),
        url_allow_list: Vec::new(),
        url_block_list: Vec::new(),
    }
}

use std::path::PathBuf as TestPathBuf;
use std::primitive::str as TestStr;
use std::sync::atomic::{AtomicU64, Ordering};

use ocentra_parent_agent_protocol::browser_policy::BrowserPolicyPatchRequest;
use ocentra_parent_agent_protocol::browser_policy_sections::{
    BrowserPolicyRuleActionPlan, BrowserPolicyRuleTarget,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentMessageTarget, AgentPeer, AgentPeerRole,
    AgentRoute,
};
use ocentra_parent_agent_protocol::BrowserPolicyApprovalRequiredFor;
use ocentra_parent_agent_protocol::BrowserPolicyApprovalUnansweredDefault;
use ocentra_parent_agent_protocol::BrowserPolicyAuditRequiredField;
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
use ocentra_parent_agent_protocol::BrowserPolicyReportVisibleField;
use ocentra_parent_agent_protocol::BrowserPolicyRetentionExactUrl;
use ocentra_parent_agent_protocol::BrowserPolicyRule;
use ocentra_parent_agent_protocol::BrowserPolicyRuleAction;
use ocentra_parent_agent_protocol::BrowserPolicyUnmanagedBrowserClassificationTarget;
use ocentra_parent_agent_protocol::BrowserPolicyUnmanagedBrowserMode;
use ocentra_parent_agent_protocol::BrowserPolicyUpdateKind;
use ocentra_parent_agent_protocol::BrowserPolicyUpdateRequest;
use ocentra_parent_agent_protocol::BrowserPolicyUpdateStatus;
use ocentra_parent_agent_protocol::BrowserPolicyUrlTargetType;
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;
use ocentra_parent_agent_service::test_support::default_browser_policy_for_test;

use crate::test_require_ok::require_ok;
use crate::{
    browser_policy_compiler::compile_browser_policy,
    browser_policy_request::{
        apply_browser_policy_patches, kind_for_command, parse_browser_policy_request,
    },
    browser_policy_runtime_support::{
        accepted_response, base_revision_matches, default_revision_id, next_audit_event_id,
        next_revision_id, preview_revision_id, rejected_response, BrowserPolicyMessage,
        BrowserPolicyRequestId, BrowserPolicyRevisionId, BrowserPolicyTimestamp,
    },
    browser_policy_store::{
        browser_policy_store_path_from_env, read_browser_policy_state, write_browser_policy_state,
        BrowserPolicyStoredState,
    },
};

static TEST_POLICY_STORE_COUNTER: AtomicU64 = AtomicU64::new(0);

#[test]
fn browser_policy_patch_accepts_proposal_manifest_writes_to_paths() {
    let patched_policy = require_ok(
        apply_browser_policy_patches(default_policy(), &proposal_manifest_patches()),
        constants::error::AGENT_EVENT_SERIALIZES,
    );
    let effective_policy = require_ok(
        compile_browser_policy(
            &patched_policy,
            crate::browser_policy_compiler::BrowserPolicyCompileRequest {
                revision_id: constants::browser_policy::REVISION_ID,
                compiled_at: constants::browser_policy::TEST_SENT_AT,
            },
        ),
        constants::error::AGENT_EVENT_SERIALIZES,
    );

    assert_eq!(effective_policy.rules.len(), 1);
    assert_eq!(
        effective_policy.execution_mode,
        BrowserPolicyExecutionMode::Enforce
    );
    assert_eq!(
        [effective_policy.discovery.detect_unmanaged_browsers],
        [true]
    );
    assert_eq!(
        effective_policy.rules.first().map(|rule| rule.target_type),
        Some(BrowserPolicyUrlTargetType::DomainOrigin)
    );
}

#[test]
fn browser_policy_patch_command_parses_as_patch_request() {
    let request = BrowserPolicyPatchRequest {
        schema_version:
            ocentra_parent_agent_protocol::policy_constants::CONTRACT_SCHEMA_VERSION_V0_6
                .to_string(),
        request_id: constants::browser_policy::REQUEST_ID.to_string(),
        kind: BrowserPolicyUpdateKind::Patch,
        policy_id: constants::browser_policy::POLICY_ID.to_string(),
        base_revision_id: constants::browser_policy::REVISION_ID.to_string(),
        patches: proposal_manifest_patches(),
    };
    let request_json = require_ok(
        serde_json::to_string(&request),
        constants::error::AGENT_EVENT_SERIALIZES,
    );
    let mut payload = LogFields::new();
    payload.insert(
        constants::field::BROWSER_POLICY_REQUEST.to_string(),
        LogFieldValue::String(request_json),
    );
    let command = AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::browser_policy::COMMAND_MESSAGE_ID.to_string(),
        sent_at: constants::browser_policy::TEST_SENT_AT.to_string(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform: constants::enforcement::PLATFORM_WINDOWS.to_string(),
            route: AgentRoute::Localhost,
        },
        command: AgentCommandName::AgentBrowserPolicyPatch,
        payload,
    };

    assert_eq!(kind_for_command(&command), BrowserPolicyUpdateKind::Patch);
    assert_eq!(
        require_ok(
            parse_browser_policy_request(&command),
            constants::error::AGENT_EVENT_SERIALIZES,
        ),
        BrowserPolicyUpdateRequest::Patch(request)
    );
}

#[test]
fn browser_policy_runtime_rejects_dishonest_manifest_updates() {
    let policy = default_policy();
    let patch_cases = vec![
        (
            policy_patch(
                constants::browser_policy::FIELD_ID_ENABLED,
                constants::browser_policy::REJECTION_UNKNOWN_WRITES_TO,
                true,
            ),
            BrowserPolicyRejectionReason::UnknownWritesTo,
        ),
        (
            policy_patch(
                constants::browser_policy::FIELD_ID_DEFAULT_POSTURE,
                constants::browser_policy::WRITES_TO_ENABLED,
                true,
            ),
            BrowserPolicyRejectionReason::UnknownField,
        ),
        (
            policy_patch(
                constants::browser_policy::FIELD_ID_DEFAULT_POSTURE,
                constants::browser_policy::WRITES_TO_DEFAULT_POSTURE,
                constants::browser_policy::REJECTION_INVALID_ENUM_VALUE,
            ),
            BrowserPolicyRejectionReason::InvalidEnumValue,
        ),
    ];
    for (patch, reason) in patch_cases {
        assert_eq!(
            apply_browser_policy_patches(policy.clone(), &[patch]),
            Err(reason),
        );
    }

    let mut invalid_policy = default_policy();
    invalid_policy.default_posture = BrowserPolicyDefaultPosture::Limit;
    invalid_policy.budgets.enabled = false;
    invalid_policy.budgets.default_daily_minutes = None;
    invalid_policy.fallback_posture = None;
    assert_eq!(
        compile_browser_policy(
            &invalid_policy,
            crate::browser_policy_compiler::BrowserPolicyCompileRequest {
                revision_id: constants::browser_policy::REVISION_ID,
                compiled_at: constants::browser_policy::TEST_SENT_AT,
            },
        ),
        Err(BrowserPolicyRejectionReason::MissingBudgetOrFallback),
    );
}

#[tokio::test]
async fn browser_policy_store_and_runtime_support_helpers_round_trip_state() {
    let policy = default_policy();
    let effective_policy = require_ok(
        compile_browser_policy(
            &policy,
            crate::browser_policy_compiler::BrowserPolicyCompileRequest {
                revision_id: constants::browser_policy::REVISION_ID,
                compiled_at: constants::browser_policy::TEST_SENT_AT,
            },
        ),
        constants::error::AGENT_EVENT_SERIALIZES,
    );
    let state = BrowserPolicyStoredState::empty();
    let store_path = temp_policy_store_path(constants::browser_policy::UPDATE_KIND_GET);

    require_ok(
        write_browser_policy_state(&store_path, &state).await,
        constants::error::AGENT_EVENT_SERIALIZES,
    );
    let roundtrip = require_ok(
        read_browser_policy_state(&store_path).await,
        constants::error::AGENT_EVENT_SERIALIZES,
    );
    let _ = browser_policy_store_path_from_env();

    assert!(state.active_revision().is_none());
    assert!(state
        .revision_by_id(&BrowserPolicyRevisionId(
            constants::browser_policy::REVISION_ID.to_string(),
        ))
        .is_none());
    assert_eq!(roundtrip, state);
    require_ok(
        base_revision_matches(&state, None),
        constants::error::AGENT_EVENT_SERIALIZES,
    );
    assert_eq!(
        next_revision_id(&state).0,
        format!("{}1", constants::browser_policy::REVISION_PREFIX)
    );
    assert_eq!(
        next_audit_event_id(&state).0,
        format!("{}1", constants::browser_policy::AUDIT_PREFIX)
    );
    assert_eq!(
        default_revision_id().0,
        format!(
            "{}{}",
            constants::browser_policy::REVISION_PREFIX,
            constants::browser_policy::UPDATE_KIND_GET
        )
    );
    assert_eq!(
        preview_revision_id().0,
        format!(
            "{}{}",
            constants::browser_policy::REVISION_PREFIX,
            constants::browser_policy::UPDATE_KIND_PREVIEW
        )
    );
    assert_eq!(
        accepted_response(
            BrowserPolicyRequestId(constants::browser_policy::REQUEST_ID.to_string()),
            BrowserPolicyUpdateKind::Preview,
            policy.clone(),
            effective_policy,
            None,
            BrowserPolicyMessage("accepted"),
            BrowserPolicyTimestamp(constants::browser_policy::TEST_SENT_AT.to_string()),
        )
        .status,
        BrowserPolicyUpdateStatus::Accepted
    );
    assert_eq!(
        rejected_response(
            BrowserPolicyRequestId(constants::browser_policy::REQUEST_ID.to_string()),
            BrowserPolicyUpdateKind::Patch,
            BrowserPolicyRejectionReason::RevisionNotFound,
            BrowserPolicyMessage("rejected"),
            BrowserPolicyTimestamp(constants::browser_policy::TEST_SENT_AT.to_string()),
        )
        .status,
        BrowserPolicyUpdateStatus::Rejected
    );
}

fn default_policy() -> ocentra_parent_agent_protocol::BrowserPolicyValue {
    default_browser_policy_for_test(crate::test_support::default_browser_policy_id_for_test())
}

fn proposal_manifest_patches() -> Vec<BrowserPolicyPatch> {
    let mut patches = browser_management_patches();
    patches.extend(managed_browser_patches());
    patches.extend(unmanaged_browser_patches());
    patches.extend(evidence_and_rule_patches());
    patches.extend(policy_writer_and_browser_game_patches());
    patches.extend(policy_support_patches());
    patches
}

fn browser_management_patches() -> Vec<BrowserPolicyPatch> {
    vec![
        policy_patch(
            constants::browser_policy::FIELD_ID_ENABLED,
            constants::browser_policy::WRITES_TO_ENABLED,
            true,
        ),
        policy_patch(
            constants::browser_policy::FIELD_ID_EXECUTION_MODE,
            constants::browser_policy::WRITES_TO_EXECUTION_MODE,
            BrowserPolicyExecutionMode::Enforce,
        ),
        policy_patch(
            constants::browser_policy::FIELD_ID_DEFAULT_POSTURE,
            constants::browser_policy::WRITES_TO_DEFAULT_POSTURE,
            BrowserPolicyDefaultPosture::Warn,
        ),
        policy_patch(
            constants::browser_policy::FIELD_ID_MANAGEMENT_MODE,
            constants::browser_policy::WRITES_TO_MANAGEMENT_MODE,
            BrowserPolicyManagementMode::LocalChildAgent,
        ),
        policy_patch(
            constants::browser_policy::FIELD_ID_DISCOVERY_SCAN_INSTALLED_BROWSERS,
            constants::browser_policy::WRITES_TO_DISCOVERY_SCAN_INSTALLED_BROWSERS,
            true,
        ),
        policy_patch(
            constants::browser_policy::FIELD_ID_DISCOVERY_SCAN_RUNNING_BROWSERS,
            constants::browser_policy::WRITES_TO_DISCOVERY_SCAN_RUNNING_BROWSERS,
            true,
        ),
        policy_patch(
            constants::browser_policy::FIELD_ID_DISCOVERY_DETECT_UNMANAGED_BROWSERS,
            constants::browser_policy::WRITES_TO_DISCOVERY_DETECT_UNMANAGED_BROWSERS,
            true,
        ),
    ]
}

fn managed_browser_patches() -> Vec<BrowserPolicyPatch> {
    vec![
        policy_patch(
            constants::browser_policy::FIELD_ID_MANAGED_BROWSER_MODE,
            constants::browser_policy::WRITES_TO_MANAGED_BROWSER_MODE,
            BrowserPolicyManagedBrowserMode::RequiredForAllBrowsing,
        ),
        policy_patch(
            constants::browser_policy::FIELD_ID_MANAGED_BROWSER_ALLOWED_FAMILIES,
            constants::browser_policy::WRITES_TO_MANAGED_BROWSER_ALLOWED_FAMILIES,
            vec![
                BrowserPolicyManagedBrowserFamily::EdgeStable,
                BrowserPolicyManagedBrowserFamily::ChromeForTesting,
            ],
        ),
        policy_patch(
            constants::browser_policy::FIELD_ID_MANAGED_BROWSER_LAUNCH_MODE,
            constants::browser_policy::WRITES_TO_MANAGED_BROWSER_LAUNCH_MODE,
            BrowserPolicyManagedBrowserLaunchMode::OcentraLauncher,
        ),
        policy_patch(
            constants::browser_policy::FIELD_ID_MANAGED_BROWSER_PROFILE_MODE,
            constants::browser_policy::WRITES_TO_MANAGED_BROWSER_PROFILE_MODE,
            BrowserPolicyManagedBrowserProfileMode::Ephemeral,
        ),
        policy_patch(
            constants::browser_policy::FIELD_ID_MANAGED_BROWSER_BRIDGE_REQUIREMENTS,
            constants::browser_policy::WRITES_TO_MANAGED_BROWSER_BRIDGE_REQUIREMENTS,
            vec![
                BrowserPolicyManagedBrowserBridgeRequirement::OwnedProfile,
                BrowserPolicyManagedBrowserBridgeRequirement::LoopbackOnly,
            ],
        ),
        policy_patch(
            constants::browser_policy::FIELD_ID_MANAGED_BROWSER_INTEGRATION_MECHANISMS,
            constants::browser_policy::WRITES_TO_MANAGED_BROWSER_INTEGRATION_MECHANISMS,
            vec![
                BrowserPolicyManagedBrowserIntegrationMechanism::ChromiumCdp,
                BrowserPolicyManagedBrowserIntegrationMechanism::BrowserPolicy,
            ],
        ),
        policy_patch(
            constants::browser_policy::FIELD_ID_MANAGED_BROWSER_POLICY_WRITER_CONTROLS,
            constants::browser_policy::WRITES_TO_MANAGED_BROWSER_POLICY_WRITER_CONTROLS,
            vec![
                BrowserPolicyManagedPolicyWriterControl::DisableIncognito,
                BrowserPolicyManagedPolicyWriterControl::ForceSafeSearch,
                BrowserPolicyManagedPolicyWriterControl::UrlAllowList,
                BrowserPolicyManagedPolicyWriterControl::UrlBlockList,
            ],
        ),
        policy_patch(
            constants::browser_policy::FIELD_ID_MANAGED_BROWSER_POLICY_WRITER_FALLBACK,
            constants::browser_policy::WRITES_TO_MANAGED_BROWSER_POLICY_WRITER_FALLBACK,
            BrowserPolicyManagedPolicyWriterFallback::ManualRequired,
        ),
    ]
}

fn unmanaged_browser_patches() -> Vec<BrowserPolicyPatch> {
    vec![
        policy_patch(
            constants::browser_policy::FIELD_ID_UNMANAGED_BROWSER_MODE,
            constants::browser_policy::WRITES_TO_UNMANAGED_BROWSER_MODE,
            BrowserPolicyUnmanagedBrowserMode::RelaunchManaged,
        ),
        policy_patch(
            constants::browser_policy::FIELD_ID_UNMANAGED_BROWSER_GRACE_SECONDS,
            constants::browser_policy::WRITES_TO_UNMANAGED_BROWSER_GRACE_SECONDS,
            15_u32,
        ),
        policy_patch(
            constants::browser_policy::FIELD_ID_UNMANAGED_BROWSER_ALLOW_RECOVER_LAUNCH_URL,
            constants::browser_policy::WRITES_TO_UNMANAGED_BROWSER_ALLOW_RECOVER_LAUNCH_URL,
            true,
        ),
        policy_patch(
            constants::browser_policy::FIELD_ID_UNMANAGED_BROWSER_CLASSIFICATION_TARGETS,
            constants::browser_policy::WRITES_TO_UNMANAGED_BROWSER_CLASSIFICATION_TARGETS,
            vec![
                BrowserPolicyUnmanagedBrowserClassificationTarget::KnownBrowser,
                BrowserPolicyUnmanagedBrowserClassificationTarget::BrowserLikeProcess,
            ],
        ),
    ]
}

fn evidence_and_rule_patches() -> Vec<BrowserPolicyPatch> {
    vec![
        policy_patch(
            constants::browser_policy::FIELD_ID_EVIDENCE_URL_SCOPE,
            constants::browser_policy::WRITES_TO_EVIDENCE_URL_SCOPE,
            BrowserPolicyEvidenceUrlScope::DomainOriginTitle,
        ),
        policy_patch(
            constants::browser_policy::FIELD_ID_REQUIRED_PROOF,
            constants::browser_policy::WRITES_TO_REQUIRED_PROOF,
            BrowserPolicyEvidenceProofLevel::FreshManagedActiveTab,
        ),
        policy_patch(
            constants::browser_policy::FIELD_ID_WHEN_PROOF_UNAVAILABLE,
            constants::browser_policy::WRITES_TO_WHEN_PROOF_UNAVAILABLE,
            BrowserPolicyProofFallback::Ask,
        ),
        policy_patch(
            constants::browser_policy::FIELD_ID_EVIDENCE_NEVER_COLLECT,
            constants::browser_policy::WRITES_TO_EVIDENCE_NEVER_COLLECT,
            vec![
                BrowserPolicyEvidenceNeverCollect::PageBody,
                BrowserPolicyEvidenceNeverCollect::Screenshots,
                BrowserPolicyEvidenceNeverCollect::RawProtocolDumps,
            ],
        ),
        policy_patch(
            constants::browser_policy::FIELD_ID_ALLOWED_TARGET_TYPES,
            constants::browser_policy::WRITES_TO_ALLOWED_TARGET_TYPES,
            vec![
                BrowserPolicyUrlTargetType::ExactUrl,
                BrowserPolicyUrlTargetType::DomainOrigin,
            ],
        ),
        policy_patch(
            constants::browser_policy::FIELD_ID_ALLOWED_ACTIONS,
            constants::browser_policy::WRITES_TO_ALLOWED_ACTIONS,
            vec![
                BrowserPolicyRuleAction::Allow,
                BrowserPolicyRuleAction::Warn,
                BrowserPolicyRuleAction::Block,
            ],
        ),
        policy_patch(
            constants::browser_policy::FIELD_ID_RULE_ITEMS,
            constants::browser_policy::WRITES_TO_RULE_ITEMS,
            vec![proposal_rule()],
        ),
        policy_patch(
            constants::browser_policy::FIELD_ID_URL_ALLOW_LIST,
            constants::browser_policy::WRITES_TO_URL_ALLOW_LIST,
            vec![constants::browser_policy::DEFAULT_TARGET_VALUE.to_string()],
        ),
        policy_patch(
            constants::browser_policy::FIELD_ID_URL_BLOCK_LIST,
            constants::browser_policy::WRITES_TO_URL_BLOCK_LIST,
            vec![constants::browser_policy::REJECTION_INVALID_REQUEST.to_string()],
        ),
    ]
}

fn policy_writer_and_browser_game_patches() -> Vec<BrowserPolicyPatch> {
    vec![
        policy_patch(
            constants::browser_policy::FIELD_ID_BROWSER_GAME_EDUCATIONAL_MODE,
            constants::browser_policy::WRITES_TO_BROWSER_GAME_EDUCATIONAL_MODE,
            BrowserPolicyBrowserGamePolicyMode::Allow,
        ),
        policy_patch(
            constants::browser_policy::FIELD_ID_BROWSER_GAME_UNKNOWN_MODE,
            constants::browser_policy::WRITES_TO_BROWSER_GAME_UNKNOWN_MODE,
            BrowserPolicyBrowserGamePolicyMode::AskParent,
        ),
        policy_patch(
            constants::browser_policy::FIELD_ID_BROWSER_GAME_CLOUD_GAMING_APPROVAL,
            constants::browser_policy::WRITES_TO_BROWSER_GAME_CLOUD_GAMING_APPROVAL,
            BrowserPolicyBrowserGameApprovalMode::AskParent,
        ),
        policy_patch(
            constants::browser_policy::FIELD_ID_BROWSER_GAME_PURCHASE_ACCOUNT_APPROVAL,
            constants::browser_policy::WRITES_TO_BROWSER_GAME_PURCHASE_ACCOUNT_APPROVAL,
            BrowserPolicyBrowserGameApprovalMode::AskParent,
        ),
        policy_patch(
            constants::browser_policy::FIELD_ID_BROWSER_GAME_DAILY_BUDGET_MINUTES,
            constants::browser_policy::WRITES_TO_BROWSER_GAME_DAILY_BUDGET_MINUTES,
            Some(30_u32),
        ),
    ]
}

fn policy_support_patches() -> Vec<BrowserPolicyPatch> {
    vec![
        policy_patch(
            constants::browser_policy::FIELD_ID_BUDGETS_ENABLED,
            constants::browser_policy::WRITES_TO_BUDGETS_ENABLED,
            true,
        ),
        policy_patch(
            constants::browser_policy::FIELD_ID_DAILY_BUDGET_MINUTES,
            constants::browser_policy::WRITES_TO_DAILY_BUDGET_MINUTES,
            Some(45_u32),
        ),
        policy_patch(
            constants::browser_policy::FIELD_ID_BUDGET_COUNTING_MODE,
            constants::browser_policy::WRITES_TO_BUDGET_COUNTING_MODE,
            BrowserPolicyBudgetCountingMode::ForegroundBrowserTime,
        ),
        policy_patch(
            constants::browser_policy::FIELD_ID_DOWNLOAD_MODE,
            constants::browser_policy::WRITES_TO_DOWNLOAD_MODE,
            BrowserPolicyDownloadState::Ask,
        ),
        policy_patch(
            constants::browser_policy::FIELD_ID_DOWNLOAD_BLOCKED_TYPES,
            constants::browser_policy::WRITES_TO_DOWNLOAD_BLOCKED_TYPES,
            vec![
                BrowserPolicyDownloadBlockedType::Executable,
                BrowserPolicyDownloadBlockedType::Script,
            ],
        ),
        policy_patch(
            constants::browser_policy::FIELD_ID_APPROVAL_REQUIRED_FOR,
            constants::browser_policy::WRITES_TO_APPROVAL_REQUIRED_FOR,
            vec![
                BrowserPolicyApprovalRequiredFor::BlockedSite,
                BrowserPolicyApprovalRequiredFor::Download,
            ],
        ),
        policy_patch(
            constants::browser_policy::FIELD_ID_APPROVAL_UNANSWERED_DEFAULT,
            constants::browser_policy::WRITES_TO_APPROVAL_UNANSWERED_DEFAULT,
            BrowserPolicyApprovalUnansweredDefault::Deny,
        ),
        policy_patch(
            constants::browser_policy::FIELD_ID_REPORT_VISIBLE_FIELDS,
            constants::browser_policy::WRITES_TO_REPORT_VISIBLE_FIELDS,
            vec![
                BrowserPolicyReportVisibleField::ManagedStatus,
                BrowserPolicyReportVisibleField::PolicyDecisions,
            ],
        ),
        policy_patch(
            constants::browser_policy::FIELD_ID_RETENTION_EXACT_URL,
            constants::browser_policy::WRITES_TO_RETENTION_EXACT_URL,
            BrowserPolicyRetentionExactUrl::SevenDays,
        ),
        policy_patch(
            constants::browser_policy::FIELD_ID_CUSTODY_ALLOWED_USES,
            constants::browser_policy::WRITES_TO_CUSTODY_ALLOWED_USES,
            vec![
                BrowserPolicyCustodyAllowedUse::ChildLocal,
                BrowserPolicyCustodyAllowedUse::ParentReport,
            ],
        ),
        policy_patch(
            constants::browser_policy::FIELD_ID_AUDIT_REQUIRED_FIELDS,
            constants::browser_policy::WRITES_TO_AUDIT_REQUIRED_FIELDS,
            vec![
                BrowserPolicyAuditRequiredField::PolicyDecision,
                BrowserPolicyAuditRequiredField::EvidenceRef,
                BrowserPolicyAuditRequiredField::AdapterResult,
            ],
        ),
    ]
}

fn proposal_rule() -> BrowserPolicyRule {
    BrowserPolicyRule {
        rule_id: constants::browser_policy::DEFAULT_RULE_ID.to_string(),
        target_type: None,
        target_value: None,
        enabled: true,
        priority: Some(100),
        target: Some(BrowserPolicyRuleTarget {
            kind: BrowserPolicyUrlTargetType::DomainOrigin,
            values: vec![constants::browser_policy::DEFAULT_TARGET_VALUE.to_string()],
            match_mode: constants::browser_policy::DEFAULT_RULE_MATCH_MODE.to_string(),
        }),
        action: Some(BrowserPolicyRuleActionPlan {
            kind: BrowserPolicyRuleAction::Allow,
            budget_id: None,
            approval_kind: None,
            reason_code: Some(constants::browser_policy::DEFAULT_RULE_REASON_CODE.to_string()),
        }),
        proof_requirement: None,
        schedule_id: Some(constants::browser_policy::DEFAULT_RULE_SCHEDULE_ID.to_string()),
        budget_id: None,
        audit_level: Some(constants::browser_policy::DEFAULT_RULE_AUDIT_LEVEL.to_string()),
    }
}

fn policy_patch<T>(field_id: &TestStr, writes_to: &TestStr, value: T) -> BrowserPolicyPatch
where
    T: serde::Serialize,
{
    BrowserPolicyPatch {
        op: constants::browser_policy::PATCH_OPERATION_REPLACE.to_string(),
        field_id: field_id.to_string(),
        writes_to: writes_to.to_string(),
        value: serialize_test_value(value),
    }
}

fn serialize_test_value<T>(value: T) -> serde_json::Value
where
    T: serde::Serialize,
{
    require_ok(
        serde_json::to_value(value),
        constants::error::AGENT_EVENT_SERIALIZES,
    )
}

fn temp_policy_store_path(store_path_suffix: &TestStr) -> TestPathBuf {
    let sequence = TEST_POLICY_STORE_COUNTER.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "ocentra-browser-policy-manifest-{store_path_suffix}-{}-{sequence}.json",
        std::process::id(),
    ))
}

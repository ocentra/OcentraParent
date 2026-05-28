use ocentra_parent_agent_protocol::{
    constants, policy_constants as policy, AgentCommandEnvelope, AgentCommandName,
    AgentEventEnvelope, AgentEventName, AgentMessageTarget, AgentPeer, AgentPeerRole, AgentRoute,
    BrowserPolicyApprovalRequiredFor, BrowserPolicyApprovalUnansweredDefault,
    BrowserPolicyAuditRequiredField, BrowserPolicyBudgetCountingMode,
    BrowserPolicyCustodyAllowedUse, BrowserPolicyDefaultPosture, BrowserPolicyDownloadBlockedType,
    BrowserPolicyDownloadState, BrowserPolicyEvidenceNeverCollect, BrowserPolicyEvidenceProofLevel,
    BrowserPolicyEvidenceUrlScope, BrowserPolicyExecutionMode,
    BrowserPolicyManagedBrowserBridgeRequirement, BrowserPolicyManagedBrowserFamily,
    BrowserPolicyManagedBrowserIntegrationMechanism, BrowserPolicyManagedBrowserLaunchMode,
    BrowserPolicyManagedBrowserMode, BrowserPolicyManagedBrowserProfileMode,
    BrowserPolicyManagementMode, BrowserPolicyPatch, BrowserPolicyPatchRequest,
    BrowserPolicyProofFallback, BrowserPolicyRejectionReason, BrowserPolicyReportVisibleField,
    BrowserPolicyRetentionExactUrl, BrowserPolicyRule, BrowserPolicyRuleAction,
    BrowserPolicyRuleActionPlan, BrowserPolicyRuleTarget,
    BrowserPolicyUnmanagedBrowserClassificationTarget, BrowserPolicyUnmanagedBrowserMode,
    BrowserPolicyUpdateKind, BrowserPolicyUpdateResponse, BrowserPolicyUpdateStatus,
    BrowserPolicyUrlTargetType, LogFieldValue, LogFields, AGENT_PROTOCOL_SCHEMA_VERSION,
};

use crate::{
    browser_policy_runtime::BrowserPolicyRuntime, browser_policy_runtime_support::default_policy,
    lan_pairing::LanPairingRuntime, websocket::handle_command_text_with_browser_policy_for_test,
};

#[tokio::test]
async fn browser_policy_patch_accepts_proposal_manifest_writes_to_paths() {
    let runtime = BrowserPolicyRuntime::in_memory();
    let replace_event = send_browser_policy_command(
        runtime.clone(),
        replace_command(default_policy(
            constants::browser_policy::POLICY_ID.to_string(),
        )),
    )
    .await;
    assert_eq!(
        response_from_event(&replace_event).status,
        BrowserPolicyUpdateStatus::Accepted
    );

    let patch_event = send_browser_policy_command(runtime, manifest_patch_command()).await;
    let patch_response = response_from_event(&patch_event);

    assert_eq!(
        patch_event.event,
        AgentEventName::AgentBrowserPolicyPatchAccepted
    );
    assert_eq!(patch_response.status, BrowserPolicyUpdateStatus::Accepted);
    assert_eq!(
        patch_response
            .effective_policy
            .as_ref()
            .map(|policy| policy.rules.len()),
        Some(1)
    );
    assert_eq!(
        patch_response
            .effective_policy
            .as_ref()
            .map(|policy| policy.execution_mode),
        Some(BrowserPolicyExecutionMode::Enforce)
    );
    assert_eq!(
        patch_response
            .effective_policy
            .as_ref()
            .map(|policy| policy.discovery.detect_unmanaged_browsers),
        Some(true)
    );
    assert_eq!(
        patch_response
            .effective_policy
            .as_ref()
            .and_then(|policy| policy.rules.first())
            .map(|rule| rule.target_type),
        Some(BrowserPolicyUrlTargetType::DomainOrigin)
    );
}

#[tokio::test]
async fn browser_policy_runtime_rejects_dishonest_manifest_updates() {
    let runtime = BrowserPolicyRuntime::in_memory();
    let replace_event = send_browser_policy_command(
        runtime.clone(),
        replace_command(default_policy(
            constants::browser_policy::POLICY_ID.to_string(),
        )),
    )
    .await;
    assert_eq!(
        response_from_event(&replace_event).status,
        BrowserPolicyUpdateStatus::Accepted
    );

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
        let event = send_browser_policy_command(
            runtime.clone(),
            command_with_request(
                AgentCommandName::AgentBrowserPolicyPatch,
                BrowserPolicyPatchRequest {
                    schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
                    request_id: constants::browser_policy::REQUEST_ID.to_string(),
                    kind: BrowserPolicyUpdateKind::Patch,
                    policy_id: constants::browser_policy::POLICY_ID.to_string(),
                    base_revision_id: constants::browser_policy::REVISION_ID.to_string(),
                    patches: vec![patch],
                },
            ),
        )
        .await;
        assert_rejected_event(
            &event,
            AgentEventName::AgentBrowserPolicyPatchRejected,
            reason,
        );
    }

    let mut invalid_policy = default_policy(constants::browser_policy::POLICY_ID.to_string());
    invalid_policy.default_posture = BrowserPolicyDefaultPosture::Limit;
    invalid_policy.budgets.enabled = false;
    invalid_policy.budgets.default_daily_minutes = None;
    invalid_policy.fallback_posture = None;
    let replace_event = send_browser_policy_command(
        BrowserPolicyRuntime::in_memory(),
        replace_command(invalid_policy),
    )
    .await;
    assert_rejected_event(
        &replace_event,
        AgentEventName::AgentBrowserPolicyReplaceRejected,
        BrowserPolicyRejectionReason::MissingBudgetOrFallback,
    );
}

fn manifest_patch_command() -> AgentCommandEnvelope {
    command_with_request(AgentCommandName::AgentBrowserPolicyPatch, patch_request())
}

fn patch_request() -> BrowserPolicyPatchRequest {
    BrowserPolicyPatchRequest {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        request_id: constants::browser_policy::REQUEST_ID.to_string(),
        kind: BrowserPolicyUpdateKind::Patch,
        policy_id: constants::browser_policy::POLICY_ID.to_string(),
        base_revision_id: constants::browser_policy::REVISION_ID.to_string(),
        patches: proposal_manifest_patches(),
    }
}

fn proposal_manifest_patches() -> Vec<BrowserPolicyPatch> {
    let mut patches = browser_management_patches();
    patches.extend(managed_browser_patches());
    patches.extend(unmanaged_browser_patches());
    patches.extend(evidence_and_rule_patches());
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

fn policy_patch<T>(field_id: &str, writes_to: &str, value: T) -> BrowserPolicyPatch
where
    T: serde::Serialize,
{
    BrowserPolicyPatch {
        op: constants::browser_policy::PATCH_OPERATION_REPLACE.to_string(),
        field_id: field_id.to_string(),
        writes_to: writes_to.to_string(),
        value: serde_json::to_value(value).expect(constants::error::AGENT_EVENT_SERIALIZES),
    }
}

fn replace_command(
    policy_value: ocentra_parent_agent_protocol::BrowserPolicyValue,
) -> AgentCommandEnvelope {
    command_with_request(
        AgentCommandName::AgentBrowserPolicyReplace,
        ocentra_parent_agent_protocol::BrowserPolicyReplaceRequest {
            schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
            request_id: constants::browser_policy::REQUEST_ID.to_string(),
            kind: BrowserPolicyUpdateKind::Replace,
            base_revision_id: None,
            policy: policy_value,
        },
    )
}

fn command_with_request<T>(command: AgentCommandName, request: T) -> AgentCommandEnvelope
where
    T: serde::Serialize,
{
    let mut payload = LogFields::new();
    payload.insert(
        constants::field::BROWSER_POLICY_REQUEST.to_string(),
        LogFieldValue::String(
            serde_json::to_string(&request).expect(constants::error::AGENT_EVENT_SERIALIZES),
        ),
    );
    AgentCommandEnvelope {
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
        command,
        payload,
    }
}

async fn send_browser_policy_command(
    runtime: BrowserPolicyRuntime,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    handle_command_text_with_browser_policy_for_test(
        &serde_json::to_string(&command).expect(constants::error::AGENT_EVENT_SERIALIZES),
        LanPairingRuntime::empty(),
        runtime,
        None,
    )
    .await
}

fn response_from_event(event: &AgentEventEnvelope) -> BrowserPolicyUpdateResponse {
    match event.payload.get(constants::field::BROWSER_POLICY_RESPONSE) {
        Some(LogFieldValue::String(text)) => {
            serde_json::from_str(text).expect(constants::error::AGENT_EVENT_SERIALIZES)
        }
        _ => unreachable!(),
    }
}

fn assert_rejected_event(
    event: &AgentEventEnvelope,
    expected_event: AgentEventName,
    expected_reason: BrowserPolicyRejectionReason,
) {
    let response = response_from_event(event);
    assert_eq!(event.event, expected_event);
    assert_eq!(response.status, BrowserPolicyUpdateStatus::Rejected);
    assert_eq!(response.rejection_reason, Some(expected_reason));
}

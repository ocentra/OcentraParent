use std::{
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use ocentra_parent_agent_protocol::{
    constants, policy_constants as policy, AgentCommandEnvelope, AgentCommandName,
    AgentEventEnvelope, AgentEventName, AgentMessageTarget, AgentPeer, AgentPeerRole, AgentRoute,
    BrowserPolicyApprovalState, BrowserPolicyApprovals, BrowserPolicyAudit,
    BrowserPolicyAuditState, BrowserPolicyBrowserGames, BrowserPolicyBudgets,
    BrowserPolicyDefaultPosture, BrowserPolicyDiscovery, BrowserPolicyDownloadState,
    BrowserPolicyDownloads, BrowserPolicyEvidenceProofLevel, BrowserPolicyEvidenceRequirement,
    BrowserPolicyExecutionMode, BrowserPolicyManagedBrowser, BrowserPolicyManagedBrowserMode,
    BrowserPolicyManagementMode, BrowserPolicyPatch, BrowserPolicyProofFallback,
    BrowserPolicyRejectionReason, BrowserPolicyReportState, BrowserPolicyReports,
    BrowserPolicyRetention, BrowserPolicyRetentionState, BrowserPolicyRule, BrowserPolicyRules,
    BrowserPolicyUnmanagedBrowser, BrowserPolicyUnmanagedBrowserMode, BrowserPolicyUpdateKind,
    BrowserPolicyUpdateResponse, BrowserPolicyUpdateStatus, BrowserPolicyUrlTargetType,
    BrowserPolicyValue, LogFieldValue, LogFields, AGENT_PROTOCOL_SCHEMA_VERSION,
};

use crate::{
    browser_policy_runtime::BrowserPolicyRuntime, lan_pairing::LanPairingRuntime,
    websocket::handle_command_text_with_browser_policy_for_test,
};

#[tokio::test]
async fn browser_policy_replace_persists_and_get_reports_after_runtime_restart() {
    let path = temp_policy_store_path(constants::browser_policy::UPDATE_KIND_REPLACE);
    let runtime = BrowserPolicyRuntime::for_store_path(&path);
    let replace = replace_command(
        None,
        valid_policy(BrowserPolicyDefaultPosture::Limit, Some(60)),
    );

    let replace_event = send_browser_policy_command(runtime.clone(), replace).await;
    let replace_response = response_from_event(&replace_event);

    assert_eq!(
        replace_event.event,
        AgentEventName::AgentBrowserPolicyReplaceAccepted
    );
    assert_eq!(replace_response.status, BrowserPolicyUpdateStatus::Accepted);
    assert_eq!(
        replace_response
            .effective_policy
            .as_ref()
            .map(|value| value.revision_id.as_str()),
        Some(constants::browser_policy::REVISION_ID)
    );
    assert_eq!(
        replace_response.audit_event_id.as_deref(),
        Some(constants::browser_policy::AUDIT_EVENT_ID)
    );
    assert!(path.exists());

    let restarted = BrowserPolicyRuntime::for_store_path(&path);
    let get_event = send_browser_policy_command(restarted, get_command()).await;
    let get_response = response_from_event(&get_event);

    assert_eq!(get_event.event, AgentEventName::AgentBrowserPolicyReported);
    assert_eq!(get_response.status, BrowserPolicyUpdateStatus::Accepted);
    assert_eq!(
        get_response.policy.as_ref().map(|value| value.enabled),
        Some(true)
    );
    assert_eq!(
        get_response
            .effective_policy
            .as_ref()
            .map(|value| value.revision_id.as_str()),
        Some(constants::browser_policy::REVISION_ID)
    );
}

#[tokio::test]
async fn browser_policy_preview_compiles_without_persisting_policy_state() {
    let path = temp_policy_store_path(constants::browser_policy::UPDATE_KIND_PREVIEW);
    let runtime = BrowserPolicyRuntime::for_store_path(&path);
    let preview = preview_command(valid_policy(BrowserPolicyDefaultPosture::Allow, None));

    let preview_event = send_browser_policy_command(runtime.clone(), preview).await;
    let preview_response = response_from_event(&preview_event);

    assert_eq!(
        preview_event.event,
        AgentEventName::AgentBrowserPolicyPreviewed
    );
    assert_eq!(preview_response.status, BrowserPolicyUpdateStatus::Accepted);
    assert_eq!(
        preview_response
            .effective_policy
            .as_ref()
            .map(|value| value.rules.len()),
        Some(1)
    );
    assert!(!path.exists());

    let get_event = send_browser_policy_command(runtime, get_command()).await;
    let get_response = response_from_event(&get_event);

    assert_eq!(get_response.status, BrowserPolicyUpdateStatus::Accepted);
    assert_eq!(
        get_response.policy.as_ref().map(|value| value.enabled),
        Some(false)
    );
    assert_eq!(
        get_response
            .effective_policy
            .as_ref()
            .map(|value| value.default_posture),
        Some(BrowserPolicyDefaultPosture::Allow)
    );
}

#[tokio::test]
async fn browser_policy_patch_rejects_stale_revision_before_persisting() {
    let path = temp_policy_store_path(constants::browser_policy::UPDATE_KIND_PATCH);
    let runtime = BrowserPolicyRuntime::for_store_path(&path);
    let replace = replace_command(
        None,
        valid_policy(BrowserPolicyDefaultPosture::Limit, Some(60)),
    );
    let replace_event = send_browser_policy_command(runtime.clone(), replace).await;
    assert_eq!(
        response_from_event(&replace_event).status,
        BrowserPolicyUpdateStatus::Accepted
    );

    let patch = patch_command(stale_revision_id(), vec![daily_budget_patch(30)]);
    let patch_event = send_browser_policy_command(runtime, patch).await;
    let patch_response = response_from_event(&patch_event);

    assert_eq!(
        patch_event.event,
        AgentEventName::AgentBrowserPolicyPatchRejected
    );
    assert_eq!(patch_response.status, BrowserPolicyUpdateStatus::Rejected);
    assert_eq!(
        patch_response.rejection_reason,
        Some(BrowserPolicyRejectionReason::StaleRevision)
    );
}

#[tokio::test]
async fn browser_policy_rollback_restores_earlier_persisted_revision() {
    let path = temp_policy_store_path(constants::browser_policy::UPDATE_KIND_ROLLBACK);
    let runtime = BrowserPolicyRuntime::for_store_path(&path);
    let replace = replace_command(
        None,
        valid_policy(BrowserPolicyDefaultPosture::Limit, Some(60)),
    );
    let replace_event = send_browser_policy_command(runtime.clone(), replace).await;
    assert_eq!(
        response_from_event(&replace_event).status,
        BrowserPolicyUpdateStatus::Accepted
    );
    let patch = patch_command(
        constants::browser_policy::REVISION_ID.to_string(),
        vec![daily_budget_patch(30)],
    );
    let patch_event = send_browser_policy_command(runtime.clone(), patch).await;
    assert_eq!(
        response_from_event(&patch_event)
            .policy
            .as_ref()
            .and_then(|policy| policy.budgets.default_daily_minutes),
        Some(30)
    );

    let rollback_event = send_browser_policy_command(runtime.clone(), rollback_command()).await;
    let rollback_response = response_from_event(&rollback_event);
    let get_event = send_browser_policy_command(runtime, get_command()).await;
    let get_response = response_from_event(&get_event);

    assert_eq!(
        rollback_event.event,
        AgentEventName::AgentBrowserPolicyRollbackAccepted
    );
    assert_eq!(
        rollback_response.status,
        BrowserPolicyUpdateStatus::Accepted
    );
    assert_eq!(
        get_response
            .policy
            .as_ref()
            .and_then(|policy| policy.budgets.default_daily_minutes),
        Some(60)
    );
    assert_eq!(
        get_response
            .effective_policy
            .as_ref()
            .map(|value| value.revision_id.as_str()),
        Some(constants::browser_policy::REVISION_ID)
    );
}

#[tokio::test]
async fn browser_policy_preview_rejects_exact_url_without_managed_proof_or_fallback() {
    let runtime = BrowserPolicyRuntime::in_memory();
    let mut policy = valid_policy(BrowserPolicyDefaultPosture::Limit, Some(60));
    policy.managed_browser.mode = BrowserPolicyManagedBrowserMode::Preferred;
    policy.evidence.required_proof = BrowserPolicyEvidenceProofLevel::NetworkDomain;
    policy.evidence.proof_fallback = None;
    policy.evidence.when_proof_unavailable = BrowserPolicyProofFallback::MarkUnavailable;
    let preview = preview_command(policy);

    let event = send_browser_policy_command(runtime, preview).await;
    let response = response_from_event(&event);

    assert_eq!(event.event, AgentEventName::AgentBrowserPolicyPreviewed);
    assert_eq!(response.status, BrowserPolicyUpdateStatus::Rejected);
    assert_eq!(
        response.rejection_reason,
        Some(BrowserPolicyRejectionReason::MissingManagedProofOrFallback)
    );
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

fn get_command() -> AgentCommandEnvelope {
    command_with_request(
        AgentCommandName::AgentBrowserPolicyGet,
        serde_json::json!({
            "schemaVersion": policy::CONTRACT_SCHEMA_VERSION_V0_6,
            "requestId": constants::browser_policy::REQUEST_ID,
            "kind": BrowserPolicyUpdateKind::Get,
            "policyId": constants::browser_policy::POLICY_ID,
        }),
    )
}

fn preview_command(policy_value: BrowserPolicyValue) -> AgentCommandEnvelope {
    command_with_request(
        AgentCommandName::AgentBrowserPolicyPreview,
        serde_json::json!({
            "schemaVersion": policy::CONTRACT_SCHEMA_VERSION_V0_6,
            "requestId": constants::browser_policy::REQUEST_ID,
            "kind": BrowserPolicyUpdateKind::Preview,
            "policy": policy_value,
        }),
    )
}

fn replace_command(
    base_revision_id: Option<String>,
    policy_value: BrowserPolicyValue,
) -> AgentCommandEnvelope {
    command_with_request(
        AgentCommandName::AgentBrowserPolicyReplace,
        serde_json::json!({
            "schemaVersion": policy::CONTRACT_SCHEMA_VERSION_V0_6,
            "requestId": constants::browser_policy::REQUEST_ID,
            "kind": BrowserPolicyUpdateKind::Replace,
            "baseRevisionId": base_revision_id,
            "policy": policy_value,
        }),
    )
}

fn patch_command(
    base_revision_id: String,
    patches: Vec<BrowserPolicyPatch>,
) -> AgentCommandEnvelope {
    command_with_request(
        AgentCommandName::AgentBrowserPolicyPatch,
        serde_json::json!({
            "schemaVersion": policy::CONTRACT_SCHEMA_VERSION_V0_6,
            "requestId": constants::browser_policy::REQUEST_ID,
            "kind": BrowserPolicyUpdateKind::Patch,
            "policyId": constants::browser_policy::POLICY_ID,
            "baseRevisionId": base_revision_id,
            "patches": patches,
        }),
    )
}

fn rollback_command() -> AgentCommandEnvelope {
    command_with_request(
        AgentCommandName::AgentBrowserPolicyRollback,
        serde_json::json!({
            "schemaVersion": policy::CONTRACT_SCHEMA_VERSION_V0_6,
            "requestId": constants::browser_policy::REQUEST_ID,
            "kind": BrowserPolicyUpdateKind::Rollback,
            "policyId": constants::browser_policy::POLICY_ID,
            "targetRevisionId": constants::browser_policy::REVISION_ID,
        }),
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

fn daily_budget_patch(minutes: u32) -> BrowserPolicyPatch {
    BrowserPolicyPatch {
        op: constants::browser_policy::PATCH_OPERATION_REPLACE.to_string(),
        field_id: constants::browser_policy::FIELD_ID_DAILY_BUDGET_MINUTES.to_string(),
        writes_to: constants::browser_policy::WRITES_TO_DAILY_BUDGET_MINUTES.to_string(),
        value: serde_json::Value::Number(serde_json::Number::from(minutes)),
    }
}

fn valid_policy(
    default_posture: BrowserPolicyDefaultPosture,
    daily_minutes: Option<u32>,
) -> BrowserPolicyValue {
    let (managed_browser, unmanaged_browser, evidence, rules) = valid_policy_browser_boundary();
    BrowserPolicyValue {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        policy_id: constants::browser_policy::POLICY_ID.to_string(),
        enabled: true,
        execution_mode: BrowserPolicyExecutionMode::Enforce,
        default_posture,
        fallback_posture: None,
        management_mode: BrowserPolicyManagementMode::ManagedBrowser,
        discovery: BrowserPolicyDiscovery {
            scan_installed_browsers: true,
            scan_running_browsers: true,
            detect_unmanaged_browsers: true,
        },
        managed_browser,
        unmanaged_browser,
        evidence,
        rules,
        budgets: BrowserPolicyBudgets {
            enabled: true,
            default_daily_minutes: daily_minutes,
            counting_mode: Default::default(),
        },
        browser_games: BrowserPolicyBrowserGames::default(),
        downloads: BrowserPolicyDownloads {
            mode: BrowserPolicyDownloadState::AskParent,
            blocked_types: Vec::new(),
            state: BrowserPolicyDownloadState::AskParent,
        },
        approvals: BrowserPolicyApprovals {
            required_for: Vec::new(),
            unanswered_default: Default::default(),
            state: BrowserPolicyApprovalState::Required,
        },
        reports: BrowserPolicyReports {
            visible_fields: Vec::new(),
            state: BrowserPolicyReportState::Weekly,
        },
        audit: BrowserPolicyAudit {
            required_fields: Vec::new(),
            state: BrowserPolicyAuditState::LocalOnly,
            plan: Default::default(),
        },
        retention: BrowserPolicyRetention {
            exact_url: Default::default(),
            state: BrowserPolicyRetentionState::SevenDays,
        },
        custody: Default::default(),
        schedules: Vec::new(),
        child_facing: Default::default(),
        portal_ai: Default::default(),
        platforms: Default::default(),
        fallbacks: Default::default(),
    }
}

fn valid_policy_browser_boundary() -> (
    BrowserPolicyManagedBrowser,
    BrowserPolicyUnmanagedBrowser,
    BrowserPolicyEvidenceRequirement,
    BrowserPolicyRules,
) {
    (
        BrowserPolicyManagedBrowser {
            mode: BrowserPolicyManagedBrowserMode::RequiredForExactRules,
            allowed_families: Vec::new(),
            launch_mode: Default::default(),
            profile_mode: Default::default(),
            bridge_requirements: Vec::new(),
            integration_mechanisms: Vec::new(),
            policy_writer_controls: Vec::new(),
            policy_writer_fallback: Default::default(),
        },
        BrowserPolicyUnmanagedBrowser {
            mode: BrowserPolicyUnmanagedBrowserMode::NetworkDomainOnly,
            grace_seconds: 0,
            allow_recover_launch_url: false,
            classification_targets: Vec::new(),
        },
        BrowserPolicyEvidenceRequirement {
            url_scope: Default::default(),
            required_proof: BrowserPolicyEvidenceProofLevel::FreshManagedActiveTab,
            proof_fallback: Some(BrowserPolicyProofFallback::DowngradeToDomain),
            when_proof_unavailable: BrowserPolicyProofFallback::Ask,
            never_collect: Vec::new(),
        },
        BrowserPolicyRules {
            allowed_target_types: vec![
                BrowserPolicyUrlTargetType::Domain,
                BrowserPolicyUrlTargetType::UrlPrefix,
                BrowserPolicyUrlTargetType::ExactUrl,
            ],
            allowed_actions: Vec::new(),
            items: Vec::new(),
            entries: vec![BrowserPolicyRule {
                rule_id: constants::browser_policy::DEFAULT_RULE_ID.to_string(),
                target_type: Some(BrowserPolicyUrlTargetType::Domain),
                target_value: Some(constants::browser_policy::DEFAULT_TARGET_VALUE.to_string()),
                enabled: true,
                priority: None,
                target: None,
                action: None,
                proof_requirement: None,
                schedule_id: None,
                budget_id: None,
                audit_level: None,
            }],
            url_allow_list: Vec::new(),
            url_block_list: Vec::new(),
        },
    )
}

fn stale_revision_id() -> String {
    let mut revision_id = constants::browser_policy::REVISION_PREFIX.to_string();
    revision_id.push_str(constants::browser_policy::UPDATE_STATUS_REJECTED);
    revision_id
}

fn temp_policy_store_path(label: &str) -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect(constants::error::AGENT_EVENT_SERIALIZES)
        .as_nanos();
    let mut path = std::env::temp_dir();
    let mut file_name = constants::browser_policy::TEST_STORE_FILE_PREFIX.to_string();
    file_name.push(constants::delimiter::HYPHEN);
    file_name.push_str(label);
    file_name.push(constants::delimiter::HYPHEN);
    file_name.push_str(&stamp.to_string());
    file_name.push(constants::delimiter::DOT);
    file_name.push_str(constants::browser_policy::STORE_FILE_EXTENSION);
    path.push(file_name);
    path
}

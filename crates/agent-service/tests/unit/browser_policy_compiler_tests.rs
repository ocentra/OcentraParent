#[path = "../support/test_invariants.rs"]
mod test_invariants;

use std::path::{Path as TestPath, PathBuf as TestPathBuf};
use std::primitive::str as TestStr;
use std::string::String as TestString;

use ocentra_parent_agent_protocol::browser_policy_sections::BrowserPolicyRuleActionPlan;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::policy_constants as policy;
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventEnvelope, AgentMessageTarget, AgentPeer,
    AgentPeerRole, AgentRoute,
};
use ocentra_parent_agent_protocol::BrowserPolicyActionExecutionState;
use ocentra_parent_agent_protocol::BrowserPolicyAiAuthority;
use ocentra_parent_agent_protocol::BrowserPolicyApprovalState;
use ocentra_parent_agent_protocol::BrowserPolicyCapabilityState;
use ocentra_parent_agent_protocol::BrowserPolicyDefaultPosture;
use ocentra_parent_agent_protocol::BrowserPolicyEvidenceProofLevel;
use ocentra_parent_agent_protocol::BrowserPolicyExecutionMode;
use ocentra_parent_agent_protocol::BrowserPolicyManagedBrowserIntegrationMechanism;
use ocentra_parent_agent_protocol::BrowserPolicyProofFallback;
use ocentra_parent_agent_protocol::BrowserPolicyRule;
use ocentra_parent_agent_protocol::BrowserPolicyRuleAction;
use ocentra_parent_agent_protocol::BrowserPolicyTargetProofRequirement;
use ocentra_parent_agent_protocol::BrowserPolicyUnmanagedBrowserClassificationTarget;
use ocentra_parent_agent_protocol::BrowserPolicyUpdateKind;
use ocentra_parent_agent_protocol::BrowserPolicyUpdateResponse;
use ocentra_parent_agent_protocol::BrowserPolicyUpdateStatus;
use ocentra_parent_agent_protocol::BrowserPolicyUrlTargetType;
use ocentra_parent_agent_protocol::BrowserPolicyValue;
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;
use ocentra_parent_agent_service::test_support::{
    default_browser_policy_for_test, handle_local_command_text_with_browser_policy_store_for_test,
};

use crate::test_invariants::{require_log_string_field, require_ok, require_some};

#[tokio::test]
async fn browser_policy_preview_labels_compiler_target_requirements() {
    for target_case in compiler_target_cases() {
        let response = preview_response_for_policy(policy_for_target_case(target_case)).await;
        let rule = first_effective_rule(&response);

        assert_eq!(response.status, BrowserPolicyUpdateStatus::Accepted);
        assert_eq!(rule.target_type, target_case.target_type);
        assert_eq!(rule.target_proof_requirement, target_case.requirement);
        assert_eq!(rule.capability_state, target_case.capability_state);
    }
}

#[tokio::test]
async fn browser_policy_preview_keeps_ai_candidate_only_and_parent_policy_authoritative() {
    let mut policy = policy_for_target_case(CompilerTargetCase {
        target_type: BrowserPolicyUrlTargetType::SiteCategory,
        proof: BrowserPolicyEvidenceProofLevel::ClassifierCategory,
        approval_state: BrowserPolicyApprovalState::Required,
        requirement: BrowserPolicyTargetProofRequirement::ClassifierCategory,
        capability_state: BrowserPolicyCapabilityState::Ready,
    });
    policy.portal_ai.allow_rule_suggestions = true;

    let response = preview_response_for_policy(policy).await;
    let rule = first_effective_rule(&response);

    assert_eq!(rule.ai_authority, BrowserPolicyAiAuthority::AiCandidateOnly);
    assert_eq!(
        rule.action_execution,
        BrowserPolicyActionExecutionState::DeterministicParentPolicy
    );
}

#[tokio::test]
async fn browser_policy_preview_dry_run_and_observe_do_not_execute_adapters() {
    let mut dry_run_policy = policy_with_blocking_domain_rule();
    dry_run_policy.execution_mode = BrowserPolicyExecutionMode::DryRun;
    let dry_run_response = preview_response_for_policy(dry_run_policy).await;

    let mut observe_policy = policy_with_blocking_domain_rule();
    observe_policy.execution_mode = BrowserPolicyExecutionMode::Observe;
    let observe_response = preview_response_for_policy(observe_policy).await;

    assert_eq!(
        first_effective_rule(&dry_run_response).action_execution,
        BrowserPolicyActionExecutionState::DryRunNoExecution
    );
    assert_eq!(
        first_effective_rule(&observe_response).action_execution,
        BrowserPolicyActionExecutionState::ObserveOnly
    );
}

#[tokio::test]
async fn browser_policy_preview_requires_adapter_proof_for_blocking_actions() {
    let manual_response = preview_response_for_policy(policy_with_blocking_domain_rule()).await;
    let ready_response = preview_response_for_policy(policy_with_ready_action_adapter()).await;

    assert_eq!(
        first_effective_rule(&manual_response).action_execution,
        BrowserPolicyActionExecutionState::ManualRequired
    );
    assert_eq!(
        first_effective_rule(&ready_response).action_execution,
        BrowserPolicyActionExecutionState::AdapterReady
    );
    for action in [
        BrowserPolicyRuleAction::TerminateProcess,
        BrowserPolicyRuleAction::RelaunchManaged,
    ] {
        let mut unmanaged_policy = policy_for_target_case(target_case(
            BrowserPolicyUrlTargetType::BrowserProcess,
            BrowserPolicyEvidenceProofLevel::ProcessRunning,
            BrowserPolicyTargetProofRequirement::ProcessDetection,
            BrowserPolicyCapabilityState::Ready,
        ));
        unmanaged_policy.rules.entries = vec![rule_for_target(
            BrowserPolicyUrlTargetType::BrowserProcess,
            action,
        )];
        let mut ready_unmanaged_policy = unmanaged_policy.clone();
        ready_unmanaged_policy.platforms.windows.enabled = true;
        ready_unmanaged_policy.platforms.windows.state =
            Some(constants::browser_policy::CAPABILITY_STATE_READY.to_string());
        ready_unmanaged_policy.platforms.windows.allowed_adapters =
            vec![constants::browser_policy::ACTION_ADAPTER_CAPABILITY_ID.to_string()];

        let manual_response = preview_response_for_policy(unmanaged_policy).await;
        let ready_response = preview_response_for_policy(ready_unmanaged_policy).await;

        assert_eq!(
            first_effective_rule(&manual_response).target_proof_requirement,
            BrowserPolicyTargetProofRequirement::ProcessDetection
        );
        assert_eq!(
            first_effective_rule(&manual_response).capability_state,
            BrowserPolicyCapabilityState::Ready
        );
        assert_eq!(
            first_effective_rule(&manual_response).action_execution,
            BrowserPolicyActionExecutionState::ManualRequired
        );
        assert_eq!(
            first_effective_rule(&ready_response).action_execution,
            BrowserPolicyActionExecutionState::AdapterReady
        );
    }
}

#[tokio::test]
async fn browser_policy_preview_reports_policy_writer_as_manual_required_capability() {
    let response = preview_response_for_policy(policy_with_writer_controls()).await;
    let registry = require_some(
        response.capability_registry.as_ref(),
        constants::error::AGENT_EVENT_SERIALIZES,
    );

    assert!(registry.capabilities.iter().any(|capability| {
        capability.capability_id == constants::browser_policy::POLICY_WRITER_CAPABILITY_ID
            && capability.state == BrowserPolicyCapabilityState::ManualRequired
    }));
}

#[test]
fn browser_policy_compiler_direct_smoke_links_assessment_and_registry_helpers() {
    let policy = policy_for_target_case(target_case(
        BrowserPolicyUrlTargetType::Domain,
        BrowserPolicyEvidenceProofLevel::NetworkDomain,
        BrowserPolicyTargetProofRequirement::DomainOrManagedUrl,
        BrowserPolicyCapabilityState::Ready,
    ));
    let effective_policy = require_ok(
        crate::browser_policy_compiler::compile_browser_policy(
            &policy,
            crate::browser_policy_compiler::BrowserPolicyCompileRequest {
                revision_id: constants::browser_policy::REVISION_ID,
                compiled_at: constants::browser_policy::TEST_SENT_AT,
            },
        ),
        constants::error::AGENT_EVENT_SERIALIZES,
    );
    let capability_registry = crate::browser_policy_compiler::browser_policy_capability_registry(
        crate::browser_policy_compiler::BrowserPolicyCapabilityRegistryRequest {
            generated_at: constants::browser_policy::TEST_SENT_AT,
        },
    );
    let assessment = crate::browser_policy_compiler_assessment::compile_rule_assessment(
        &policy,
        BrowserPolicyUrlTargetType::Domain,
        BrowserPolicyRuleAction::Block,
    );

    assert_eq!(effective_policy.rules.len(), 1);
    assert!(capability_registry
        .capabilities
        .iter()
        .any(|capability| capability.capability_id
            == constants::browser_policy::POLICY_WRITER_CAPABILITY_ID));
    assert_eq!(
        assessment.target_proof_requirement,
        BrowserPolicyTargetProofRequirement::DomainOrManagedUrl
    );
    assert_eq!(
        crate::browser_policy_compiler_assessment::rule_action(
            &policy.rules.entries[0],
            BrowserPolicyDefaultPosture::Warn,
        ),
        BrowserPolicyRuleAction::Ask
    );
}

#[derive(Clone, Copy)]
struct CompilerTargetCase {
    target_type: BrowserPolicyUrlTargetType,
    proof: BrowserPolicyEvidenceProofLevel,
    approval_state: BrowserPolicyApprovalState,
    requirement: BrowserPolicyTargetProofRequirement,
    capability_state: BrowserPolicyCapabilityState,
}

fn compiler_target_cases() -> Vec<CompilerTargetCase> {
    vec![
        target_case(
            BrowserPolicyUrlTargetType::ExactUrl,
            BrowserPolicyEvidenceProofLevel::FreshManagedActiveTab,
            BrowserPolicyTargetProofRequirement::ManagedExactUrl,
            BrowserPolicyCapabilityState::ManualRequired,
        ),
        target_case(
            BrowserPolicyUrlTargetType::Domain,
            BrowserPolicyEvidenceProofLevel::NetworkDomain,
            BrowserPolicyTargetProofRequirement::DomainOrManagedUrl,
            BrowserPolicyCapabilityState::Ready,
        ),
        target_case(
            BrowserPolicyUrlTargetType::SiteCategory,
            BrowserPolicyEvidenceProofLevel::ClassifierCategory,
            BrowserPolicyTargetProofRequirement::ClassifierCategory,
            BrowserPolicyCapabilityState::Ready,
        ),
        target_case(
            BrowserPolicyUrlTargetType::SearchTerms,
            BrowserPolicyEvidenceProofLevel::UrlShapeMetadata,
            BrowserPolicyTargetProofRequirement::UrlShapeMetadata,
            BrowserPolicyCapabilityState::Ready,
        ),
        social_target_case(BrowserPolicyUrlTargetType::SocialFeed),
        target_case(
            BrowserPolicyUrlTargetType::UnknownSocialSite,
            BrowserPolicyEvidenceProofLevel::None,
            BrowserPolicyTargetProofRequirement::SocialRouteEvidence,
            BrowserPolicyCapabilityState::ManualRequired,
        ),
        target_case(
            BrowserPolicyUrlTargetType::CloudGaming,
            BrowserPolicyEvidenceProofLevel::BrowserGameRuntimeSignal,
            BrowserPolicyTargetProofRequirement::BrowserGameRuntimeSignal,
            BrowserPolicyCapabilityState::Ready,
        ),
        target_case(
            BrowserPolicyUrlTargetType::UnknownGame,
            BrowserPolicyEvidenceProofLevel::None,
            BrowserPolicyTargetProofRequirement::BrowserGameRuntimeSignal,
            BrowserPolicyCapabilityState::ManualRequired,
        ),
        target_case(
            BrowserPolicyUrlTargetType::BrowserProcess,
            BrowserPolicyEvidenceProofLevel::ProcessRunning,
            BrowserPolicyTargetProofRequirement::ProcessDetection,
            BrowserPolicyCapabilityState::Ready,
        ),
    ]
}

fn target_case(
    target_type: BrowserPolicyUrlTargetType,
    proof: BrowserPolicyEvidenceProofLevel,
    requirement: BrowserPolicyTargetProofRequirement,
    capability_state: BrowserPolicyCapabilityState,
) -> CompilerTargetCase {
    CompilerTargetCase {
        target_type,
        proof,
        approval_state: BrowserPolicyApprovalState::Required,
        requirement,
        capability_state,
    }
}

fn social_target_case(target_type: BrowserPolicyUrlTargetType) -> CompilerTargetCase {
    CompilerTargetCase {
        target_type,
        proof: BrowserPolicyEvidenceProofLevel::SocialRouteEvidence,
        approval_state: BrowserPolicyApprovalState::Approved,
        requirement: BrowserPolicyTargetProofRequirement::SocialRouteEvidence,
        capability_state: BrowserPolicyCapabilityState::Ready,
    }
}

fn policy_for_target_case(target_case: CompilerTargetCase) -> BrowserPolicyValue {
    let mut policy =
        default_browser_policy_for_test(crate::test_support::default_browser_policy_id_for_test());
    policy.enabled = true;
    policy.execution_mode = BrowserPolicyExecutionMode::Enforce;
    policy.default_posture = BrowserPolicyDefaultPosture::Warn;
    policy.rules.allowed_target_types = vec![target_case.target_type];
    policy.rules.entries = vec![rule_for_target(
        target_case.target_type,
        BrowserPolicyRuleAction::Ask,
    )];
    policy.evidence.required_proof = target_case.proof;
    policy.evidence.proof_fallback = Some(BrowserPolicyProofFallback::AskParent);
    policy.evidence.when_proof_unavailable = BrowserPolicyProofFallback::Ask;
    policy.approvals.state = target_case.approval_state;
    if target_case.target_type == BrowserPolicyUrlTargetType::BrowserProcess {
        policy.unmanaged_browser.classification_targets =
            vec![BrowserPolicyUnmanagedBrowserClassificationTarget::KnownBrowser];
    }
    policy
}

fn policy_with_blocking_domain_rule() -> BrowserPolicyValue {
    let mut policy = policy_for_target_case(target_case(
        BrowserPolicyUrlTargetType::Domain,
        BrowserPolicyEvidenceProofLevel::NetworkDomain,
        BrowserPolicyTargetProofRequirement::DomainOrManagedUrl,
        BrowserPolicyCapabilityState::Ready,
    ));
    policy.rules.entries = vec![rule_for_target(
        BrowserPolicyUrlTargetType::Domain,
        BrowserPolicyRuleAction::Block,
    )];
    policy
}

fn policy_with_ready_action_adapter() -> BrowserPolicyValue {
    let mut policy = policy_with_blocking_domain_rule();
    policy.platforms.windows.enabled = true;
    policy.platforms.windows.state =
        Some(constants::browser_policy::CAPABILITY_STATE_READY.to_string());
    policy.platforms.windows.allowed_adapters =
        vec![constants::browser_policy::ACTION_ADAPTER_CAPABILITY_ID.to_string()];
    policy
}

fn policy_with_writer_controls() -> BrowserPolicyValue {
    let mut policy = policy_with_ready_action_adapter();
    policy.managed_browser.integration_mechanisms =
        vec![BrowserPolicyManagedBrowserIntegrationMechanism::BrowserPolicy];
    policy.managed_browser.policy_writer_controls =
        vec![ocentra_parent_agent_protocol::browser_policy_catalog_values::BrowserPolicyManagedPolicyWriterControl::UrlBlockList];
    policy.rules.url_block_list = vec![constants::browser_policy::DEFAULT_TARGET_VALUE.to_string()];
    policy
}

fn rule_for_target(
    target_type: BrowserPolicyUrlTargetType,
    action: BrowserPolicyRuleAction,
) -> BrowserPolicyRule {
    BrowserPolicyRule {
        rule_id: constants::browser_policy::DEFAULT_RULE_ID.to_string(),
        target_type: Some(target_type),
        target_value: Some(constants::browser_policy::DEFAULT_TARGET_VALUE.to_string()),
        enabled: true,
        priority: None,
        target: None,
        action: Some(BrowserPolicyRuleActionPlan {
            kind: action,
            budget_id: None,
            approval_kind: None,
            reason_code: Some(constants::browser_policy::DEFAULT_RULE_REASON_CODE.to_string()),
        }),
        proof_requirement: None,
        schedule_id: None,
        budget_id: None,
        audit_level: None,
    }
}

async fn preview_response_for_policy(policy: BrowserPolicyValue) -> BrowserPolicyUpdateResponse {
    let event = send_browser_policy_command(
        &temp_policy_store_path(constants::browser_policy::UPDATE_KIND_PREVIEW),
        preview_command(&policy),
    )
    .await;
    parse_test_json(require_log_string_field(
        event.payload.get(constants::field::BROWSER_POLICY_RESPONSE),
        constants::error::AGENT_EVENT_SERIALIZES,
    ))
}

fn first_effective_rule(
    response: &BrowserPolicyUpdateResponse,
) -> &ocentra_parent_agent_protocol::browser_policy_model::BrowserPolicyEffectiveRule {
    let policy = require_some(
        response.effective_policy.as_ref(),
        constants::error::AGENT_EVENT_SERIALIZES,
    );
    require_some(
        policy.rules.first(),
        constants::error::AGENT_EVENT_SERIALIZES,
    )
}

async fn send_browser_policy_command(
    store_path: &std::path::Path,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    handle_local_command_text_with_browser_policy_store_for_test(
        &serialize_test_json(&command),
        store_path,
    )
    .await
}

fn preview_command(policy_value: &BrowserPolicyValue) -> AgentCommandEnvelope {
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

fn command_with_request<T>(command: AgentCommandName, request: T) -> AgentCommandEnvelope
where
    T: serde::Serialize,
{
    let mut payload = LogFields::new();
    payload.insert(
        constants::field::BROWSER_POLICY_REQUEST.to_string(),
        LogFieldValue::String(serialize_test_json(&request)),
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

fn serialize_test_json<T>(value: &T) -> TestString
where
    T: serde::Serialize + ?Sized,
{
    crate::test_invariants::serialize_test_json(value)
}

fn parse_test_json<T>(text: TestText) -> T
where
    T: serde::de::DeserializeOwned,
{
    crate::test_invariants::require_json_decode(
        text.as_ref().as_bytes(),
        constants::error::AGENT_EVENT_SERIALIZES,
    )
}

fn temp_policy_store_path(store_path_suffix: TestText) -> TestPathBuf {
    let store_path_suffix = store_path_suffix.as_ref();
    let millis = require_ok(
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH),
        constants::error::AGENT_EVENT_SERIALIZES,
    )
    .as_millis();
    std::env::temp_dir().join(format!(
        "ocentra-browser-policy-compiler-{store_path_suffix}-{millis}-{}.json",
        std::process::id()
    ))
}

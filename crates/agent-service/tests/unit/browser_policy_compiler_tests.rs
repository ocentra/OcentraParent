use ocentra_parent_agent_protocol::browser_policy_sections::BrowserPolicyRuleActionPlan;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::BrowserPolicyActionExecutionState;
use ocentra_parent_agent_protocol::BrowserPolicyAiAuthority;
use ocentra_parent_agent_protocol::BrowserPolicyApprovalState;
use ocentra_parent_agent_protocol::BrowserPolicyApprovals;
use ocentra_parent_agent_protocol::BrowserPolicyAudit;
use ocentra_parent_agent_protocol::BrowserPolicyAuditState;
use ocentra_parent_agent_protocol::BrowserPolicyBrowserGames;
use ocentra_parent_agent_protocol::BrowserPolicyBudgets;
use ocentra_parent_agent_protocol::BrowserPolicyCapabilityState;
use ocentra_parent_agent_protocol::BrowserPolicyDefaultPosture;
use ocentra_parent_agent_protocol::BrowserPolicyDiscovery;
use ocentra_parent_agent_protocol::BrowserPolicyDownloadState;
use ocentra_parent_agent_protocol::BrowserPolicyDownloads;
use ocentra_parent_agent_protocol::BrowserPolicyEvidenceProofLevel;
use ocentra_parent_agent_protocol::BrowserPolicyEvidenceRequirement;
use ocentra_parent_agent_protocol::BrowserPolicyExecutionMode;
use ocentra_parent_agent_protocol::BrowserPolicyManagedBrowser;
use ocentra_parent_agent_protocol::BrowserPolicyManagedBrowserMode;
use ocentra_parent_agent_protocol::BrowserPolicyManagementMode;
use ocentra_parent_agent_protocol::BrowserPolicyProofFallback;
use ocentra_parent_agent_protocol::BrowserPolicyReportState;
use ocentra_parent_agent_protocol::BrowserPolicyReports;
use ocentra_parent_agent_protocol::BrowserPolicyRetention;
use ocentra_parent_agent_protocol::BrowserPolicyRetentionState;
use ocentra_parent_agent_protocol::BrowserPolicyRule;
use ocentra_parent_agent_protocol::BrowserPolicyRuleAction;
use ocentra_parent_agent_protocol::BrowserPolicyRules;
use ocentra_parent_agent_protocol::BrowserPolicyTargetProofRequirement;
use ocentra_parent_agent_protocol::BrowserPolicyUnmanagedBrowser;
use ocentra_parent_agent_protocol::BrowserPolicyUnmanagedBrowserClassificationTarget;
use ocentra_parent_agent_protocol::BrowserPolicyUnmanagedBrowserMode;
use ocentra_parent_agent_protocol::BrowserPolicyUrlTargetType;
use ocentra_parent_agent_protocol::BrowserPolicyValue;

use crate::test_require_ok::require_ok;
use crate::test_require_some::require_some;

#[test]
fn browser_policy_compiler_labels_target_requirements() {
    for target_case in compiler_target_cases() {
        let policy = compile_policy(&policy_for_target_case(target_case));
        let rule = first_effective_rule(&policy);

        assert_eq!(rule.target_type, target_case.target_type);
        assert_eq!(rule.target_proof_requirement, target_case.requirement);
        assert_eq!(rule.capability_state, target_case.capability_state);
    }
}

#[test]
fn browser_policy_compiler_keeps_ai_candidate_only_and_parent_policy_authoritative() {
    let mut policy = policy_for_target_case(CompilerTargetCase {
        target_type: BrowserPolicyUrlTargetType::SiteCategory,
        proof: BrowserPolicyEvidenceProofLevel::ClassifierCategory,
        approval_state: BrowserPolicyApprovalState::Required,
        requirement: BrowserPolicyTargetProofRequirement::ClassifierCategory,
        capability_state: BrowserPolicyCapabilityState::Ready,
    });
    policy.portal_ai.allow_rule_suggestions = true;

    let policy = compile_policy(&policy);
    let rule = first_effective_rule(&policy);

    assert_eq!(rule.ai_authority, BrowserPolicyAiAuthority::AiCandidateOnly);
    assert_eq!(
        rule.action_execution,
        BrowserPolicyActionExecutionState::DeterministicParentPolicy
    );
}

#[test]
fn browser_policy_compiler_dry_run_and_observe_do_not_execute_adapters() {
    let mut dry_run_policy = policy_with_blocking_domain_rule();
    dry_run_policy.execution_mode = BrowserPolicyExecutionMode::DryRun;
    let dry_run_policy = compile_policy(&dry_run_policy);

    let mut observe_policy = policy_with_blocking_domain_rule();
    observe_policy.execution_mode = BrowserPolicyExecutionMode::Observe;
    let observe_policy = compile_policy(&observe_policy);

    assert_eq!(
        first_effective_rule(&dry_run_policy).action_execution,
        BrowserPolicyActionExecutionState::DryRunNoExecution
    );
    assert_eq!(
        first_effective_rule(&observe_policy).action_execution,
        BrowserPolicyActionExecutionState::ObserveOnly
    );
}

#[test]
fn browser_policy_compiler_requires_adapter_proof_for_blocking_actions() {
    let manual_policy = compile_policy(&policy_with_blocking_domain_rule());
    let ready_policy = compile_policy(&policy_with_ready_action_adapter());

    assert_eq!(
        first_effective_rule(&manual_policy).action_execution,
        BrowserPolicyActionExecutionState::ManualRequired
    );
    assert_eq!(
        first_effective_rule(&ready_policy).action_execution,
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

        let manual_policy = compile_policy(&unmanaged_policy);
        let ready_policy = compile_policy(&ready_unmanaged_policy);

        assert_eq!(
            first_effective_rule(&manual_policy).target_proof_requirement,
            BrowserPolicyTargetProofRequirement::ProcessDetection
        );
        assert_eq!(
            first_effective_rule(&manual_policy).capability_state,
            BrowserPolicyCapabilityState::Ready
        );
        assert_eq!(
            first_effective_rule(&manual_policy).action_execution,
            BrowserPolicyActionExecutionState::ManualRequired
        );
        assert_eq!(
            first_effective_rule(&ready_policy).action_execution,
            BrowserPolicyActionExecutionState::AdapterReady
        );
    }
}

#[test]
fn browser_policy_compiler_reports_policy_writer_as_manual_required_capability() {
    let registry = crate::browser_policy_compiler::browser_policy_capability_registry(
        crate::browser_policy_compiler::BrowserPolicyCapabilityRegistryRequest {
            generated_at: constants::browser_policy::TEST_SENT_AT,
        },
    );

    assert!(registry.capabilities.iter().any(|capability| {
        capability.capability_id == constants::browser_policy::POLICY_WRITER_CAPABILITY_ID
            && capability.state == BrowserPolicyCapabilityState::ManualRequired
    }));
}

#[test]
fn browser_policy_compiler_projects_rules_capabilities_and_assessment() {
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
    let mut policy = default_policy();
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
        policy.discovery.detect_unmanaged_browsers = true;
        policy.unmanaged_browser.classification_targets =
            vec![BrowserPolicyUnmanagedBrowserClassificationTarget::KnownBrowser];
    }
    policy
}

fn default_policy() -> BrowserPolicyValue {
    BrowserPolicyValue {
        schema_version:
            ocentra_parent_agent_protocol::policy_constants::CONTRACT_SCHEMA_VERSION_V0_6
                .to_string(),
        policy_id: constants::browser_policy::POLICY_ID.to_string(),
        enabled: false,
        execution_mode: BrowserPolicyExecutionMode::Observe,
        default_posture: BrowserPolicyDefaultPosture::Observe,
        fallback_posture: None,
        management_mode: BrowserPolicyManagementMode::LocalChildAgent,
        discovery: BrowserPolicyDiscovery::default(),
        managed_browser: BrowserPolicyManagedBrowser {
            mode: BrowserPolicyManagedBrowserMode::AvailableForExactRules,
            allowed_families: Vec::new(),
            launch_mode: Default::default(),
            profile_mode: Default::default(),
            bridge_requirements: Vec::new(),
            integration_mechanisms: Vec::new(),
            policy_writer_controls: Vec::new(),
            policy_writer_fallback: Default::default(),
        },
        unmanaged_browser: BrowserPolicyUnmanagedBrowser {
            mode: BrowserPolicyUnmanagedBrowserMode::NetworkDomainOnly,
            grace_seconds: 0,
            allow_recover_launch_url: false,
            classification_targets: Vec::new(),
        },
        evidence: BrowserPolicyEvidenceRequirement {
            url_scope: Default::default(),
            required_proof: BrowserPolicyEvidenceProofLevel::None,
            proof_fallback: None,
            when_proof_unavailable: BrowserPolicyProofFallback::Ask,
            never_collect: Vec::new(),
        },
        rules: BrowserPolicyRules {
            allowed_target_types: Vec::new(),
            allowed_actions: Vec::new(),
            items: Vec::new(),
            entries: Vec::new(),
            url_allow_list: Vec::new(),
            url_block_list: Vec::new(),
        },
        budgets: BrowserPolicyBudgets {
            enabled: true,
            default_daily_minutes: None,
            counting_mode: Default::default(),
        },
        browser_games: BrowserPolicyBrowserGames::default(),
        downloads: BrowserPolicyDownloads {
            mode: BrowserPolicyDownloadState::Observe,
            blocked_types: Vec::new(),
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
            exact_url: Default::default(),
            state: BrowserPolicyRetentionState::None,
        },
        custody: Default::default(),
        schedules: Vec::new(),
        child_facing: Default::default(),
        portal_ai: Default::default(),
        platforms: Default::default(),
        fallbacks: Default::default(),
    }
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

fn compile_policy(
    policy: &BrowserPolicyValue,
) -> ocentra_parent_agent_protocol::BrowserPolicyEffectivePolicy {
    require_ok(
        crate::browser_policy_compiler::compile_browser_policy(
            policy,
            crate::browser_policy_compiler::BrowserPolicyCompileRequest {
                revision_id: constants::browser_policy::REVISION_ID,
                compiled_at: constants::browser_policy::TEST_SENT_AT,
            },
        ),
        constants::error::AGENT_EVENT_SERIALIZES,
    )
}

fn first_effective_rule(
    policy: &ocentra_parent_agent_protocol::BrowserPolicyEffectivePolicy,
) -> &ocentra_parent_agent_protocol::browser_policy_model::BrowserPolicyEffectiveRule {
    require_some(
        policy.rules.first(),
        constants::error::AGENT_EVENT_SERIALIZES,
    )
}

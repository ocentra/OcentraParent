use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::BrowserPolicyActionExecutionState;
use ocentra_parent_agent_protocol::BrowserPolicyAiAuthority;
use ocentra_parent_agent_protocol::BrowserPolicyApprovalState;
use ocentra_parent_agent_protocol::BrowserPolicyCapabilityState;
use ocentra_parent_agent_protocol::BrowserPolicyDefaultPosture;
use ocentra_parent_agent_protocol::BrowserPolicyEvidenceProofLevel;
use ocentra_parent_agent_protocol::BrowserPolicyManagedBrowserIntegrationMechanism;
use ocentra_parent_agent_protocol::BrowserPolicyManagedBrowserMode;
use ocentra_parent_agent_protocol::BrowserPolicyRule;
use ocentra_parent_agent_protocol::BrowserPolicyRuleAction;
use ocentra_parent_agent_protocol::BrowserPolicyTargetProofRequirement;
use ocentra_parent_agent_protocol::BrowserPolicyUrlTargetType;
use ocentra_parent_agent_protocol::BrowserPolicyValue;

#[derive(Clone, Copy)]
pub(crate) struct RuleCompileAssessment {
    pub(crate) target_proof_requirement: BrowserPolicyTargetProofRequirement,
    pub(crate) capability_state: BrowserPolicyCapabilityState,
    pub(crate) action_execution: BrowserPolicyActionExecutionState,
    pub(crate) ai_authority: BrowserPolicyAiAuthority,
    pub(crate) compile_note: &'static str,
}

const ACTIONS_FOR_POSTURE: [BrowserPolicyRuleAction; 7] = [
    BrowserPolicyRuleAction::Monitor,
    BrowserPolicyRuleAction::Allow,
    BrowserPolicyRuleAction::Warn,
    BrowserPolicyRuleAction::Ask,
    BrowserPolicyRuleAction::Limit,
    BrowserPolicyRuleAction::Ask,
    BrowserPolicyRuleAction::Block,
];

const TARGET_PROOF_REQUIREMENTS: [BrowserPolicyTargetProofRequirement; 37] = [
    BrowserPolicyTargetProofRequirement::DomainOrManagedUrl,
    BrowserPolicyTargetProofRequirement::DomainOrManagedUrl,
    BrowserPolicyTargetProofRequirement::ManagedExactUrl,
    BrowserPolicyTargetProofRequirement::DomainOrManagedUrl,
    BrowserPolicyTargetProofRequirement::ClassifierCategory,
    BrowserPolicyTargetProofRequirement::UrlShapeMetadata,
    BrowserPolicyTargetProofRequirement::UrlShapeMetadata,
    BrowserPolicyTargetProofRequirement::None,
    BrowserPolicyTargetProofRequirement::ProcessDetection,
    BrowserPolicyTargetProofRequirement::CapabilityState,
    BrowserPolicyTargetProofRequirement::DownloadEvidence,
    BrowserPolicyTargetProofRequirement::SocialRouteEvidence,
    BrowserPolicyTargetProofRequirement::SocialRouteEvidence,
    BrowserPolicyTargetProofRequirement::SocialRouteEvidence,
    BrowserPolicyTargetProofRequirement::SocialRouteEvidence,
    BrowserPolicyTargetProofRequirement::SocialRouteEvidence,
    BrowserPolicyTargetProofRequirement::SocialRouteEvidence,
    BrowserPolicyTargetProofRequirement::SocialRouteEvidence,
    BrowserPolicyTargetProofRequirement::SocialRouteEvidence,
    BrowserPolicyTargetProofRequirement::SocialRouteEvidence,
    BrowserPolicyTargetProofRequirement::SocialRouteEvidence,
    BrowserPolicyTargetProofRequirement::SocialRouteEvidence,
    BrowserPolicyTargetProofRequirement::BrowserGameRuntimeSignal,
    BrowserPolicyTargetProofRequirement::BrowserGameRuntimeSignal,
    BrowserPolicyTargetProofRequirement::BrowserGameRuntimeSignal,
    BrowserPolicyTargetProofRequirement::BrowserGameRuntimeSignal,
    BrowserPolicyTargetProofRequirement::BrowserGameRuntimeSignal,
    BrowserPolicyTargetProofRequirement::BrowserGameRuntimeSignal,
    BrowserPolicyTargetProofRequirement::BrowserGameRuntimeSignal,
    BrowserPolicyTargetProofRequirement::BrowserGameRuntimeSignal,
    BrowserPolicyTargetProofRequirement::BrowserGameRuntimeSignal,
    BrowserPolicyTargetProofRequirement::BrowserGameRuntimeSignal,
    BrowserPolicyTargetProofRequirement::BrowserGameRuntimeSignal,
    BrowserPolicyTargetProofRequirement::BrowserGameRuntimeSignal,
    BrowserPolicyTargetProofRequirement::BrowserGameRuntimeSignal,
    BrowserPolicyTargetProofRequirement::BrowserGameRuntimeSignal,
    BrowserPolicyTargetProofRequirement::BrowserGameRuntimeSignal,
];

const EXPLICIT_ACTION_EXECUTION_STATES: [Option<BrowserPolicyActionExecutionState>; 4] = [
    Some(BrowserPolicyActionExecutionState::ObserveOnly),
    Some(BrowserPolicyActionExecutionState::DryRunNoExecution),
    None,
    None,
];

const GENERIC_ACTION_EXECUTION_STATES: [BrowserPolicyActionExecutionState; 4] = [
    BrowserPolicyActionExecutionState::DeterministicParentPolicy,
    BrowserPolicyActionExecutionState::AdapterReady,
    BrowserPolicyActionExecutionState::ManualRequired,
    BrowserPolicyActionExecutionState::ManualRequired,
];

const COMPILE_NOTES: [&str; 12] = [
    constants::browser_policy::COMPILE_NOTE_PARENT_POLICY,
    constants::browser_policy::COMPILE_NOTE_MANAGED_EXACT_URL,
    constants::browser_policy::COMPILE_NOTE_DOMAIN_OR_MANAGED,
    constants::browser_policy::COMPILE_NOTE_CLASSIFIER_REQUIRED,
    constants::browser_policy::COMPILE_NOTE_URL_METADATA_REQUIRED,
    constants::browser_policy::COMPILE_NOTE_SOCIAL_REQUIRED,
    constants::browser_policy::COMPILE_NOTE_GAME_REQUIRED,
    constants::browser_policy::COMPILE_NOTE_POLICY_WRITER_REQUIRED,
    constants::browser_policy::COMPILE_NOTE_PROCESS_REQUIRED,
    constants::browser_policy::COMPILE_NOTE_PARENT_POLICY,
    constants::browser_policy::COMPILE_NOTE_PARENT_POLICY,
    constants::browser_policy::COMPILE_NOTE_ACTION_ADAPTER_REQUIRED,
];

const OBSERVE_DRY_RUN_COMPILE_NOTE: &str = constants::browser_policy::COMPILE_NOTE_OBSERVE_DRY_RUN;

const COMPILE_NOTE_TABLE: [[&str; 12]; 6] = [
    [OBSERVE_DRY_RUN_COMPILE_NOTE; 12],
    [OBSERVE_DRY_RUN_COMPILE_NOTE; 12],
    COMPILE_NOTES,
    COMPILE_NOTES,
    COMPILE_NOTES,
    COMPILE_NOTES,
];

const DOMAIN_EVIDENCE_READY_PROOFS: [BrowserPolicyEvidenceProofLevel; 5] = [
    BrowserPolicyEvidenceProofLevel::NetworkDomain,
    BrowserPolicyEvidenceProofLevel::ManagedActiveTab,
    BrowserPolicyEvidenceProofLevel::ManagedTabList,
    BrowserPolicyEvidenceProofLevel::FreshManagedTabList,
    BrowserPolicyEvidenceProofLevel::FreshManagedActiveTab,
];

pub(crate) fn compile_rule_assessment(
    policy: &BrowserPolicyValue,
    target_type: BrowserPolicyUrlTargetType,
    action: BrowserPolicyRuleAction,
) -> RuleCompileAssessment {
    let target_proof_requirement = target_proof_requirement(target_type);
    let capability_state = target_capability_state(policy, target_proof_requirement);
    let action_execution = action_execution_state(policy, action, capability_state);
    RuleCompileAssessment {
        target_proof_requirement,
        capability_state,
        action_execution,
        ai_authority: ai_authority(policy),
        compile_note: COMPILE_NOTE_TABLE[action_execution as usize]
            [target_proof_requirement as usize],
    }
}

pub(crate) fn rule_action(
    rule: &BrowserPolicyRule,
    posture: BrowserPolicyDefaultPosture,
) -> BrowserPolicyRuleAction {
    rule.action
        .as_ref()
        .map(|action| action.kind)
        .unwrap_or_else(|| action_for_posture(posture))
}

fn action_for_posture(posture: BrowserPolicyDefaultPosture) -> BrowserPolicyRuleAction {
    ACTIONS_FOR_POSTURE[posture as usize]
}

fn target_proof_requirement(
    target_type: BrowserPolicyUrlTargetType,
) -> BrowserPolicyTargetProofRequirement {
    TARGET_PROOF_REQUIREMENTS[target_type as usize]
}

fn target_capability_state(
    policy: &BrowserPolicyValue,
    requirement: BrowserPolicyTargetProofRequirement,
) -> BrowserPolicyCapabilityState {
    match requirement {
        BrowserPolicyTargetProofRequirement::None => BrowserPolicyCapabilityState::Ready,
        BrowserPolicyTargetProofRequirement::ManagedExactUrl => ready_or_manual(
            exact_url_proof_configured(policy) && platform_browser_bridge_ready(policy),
        ),
        BrowserPolicyTargetProofRequirement::DomainOrManagedUrl => {
            ready_or_manual(DOMAIN_EVIDENCE_READY_PROOFS.contains(&policy.evidence.required_proof))
        }
        BrowserPolicyTargetProofRequirement::ClassifierCategory => {
            proof_state(policy, BrowserPolicyEvidenceProofLevel::ClassifierCategory)
        }
        BrowserPolicyTargetProofRequirement::UrlShapeMetadata => {
            proof_state(policy, BrowserPolicyEvidenceProofLevel::UrlShapeMetadata)
        }
        BrowserPolicyTargetProofRequirement::SocialRouteEvidence => ready_or_manual(
            policy.evidence.required_proof == BrowserPolicyEvidenceProofLevel::SocialRouteEvidence
                && policy.approvals.state == BrowserPolicyApprovalState::Approved,
        ),
        BrowserPolicyTargetProofRequirement::BrowserGameRuntimeSignal => proof_state(
            policy,
            BrowserPolicyEvidenceProofLevel::BrowserGameRuntimeSignal,
        ),
        BrowserPolicyTargetProofRequirement::BrowserPolicyWriter => ready_or_manual(
            policy
                .managed_browser
                .integration_mechanisms
                .contains(&BrowserPolicyManagedBrowserIntegrationMechanism::BrowserPolicy)
                && !policy.managed_browser.policy_writer_controls.is_empty()
                && platform_action_adapter_ready(policy),
        ),
        BrowserPolicyTargetProofRequirement::ProcessDetection => ready_or_manual(
            policy.discovery.detect_unmanaged_browsers
                && !policy.unmanaged_browser.classification_targets.is_empty()
                && policy.evidence.required_proof
                    == BrowserPolicyEvidenceProofLevel::ProcessRunning,
        ),
        BrowserPolicyTargetProofRequirement::DownloadEvidence
        | BrowserPolicyTargetProofRequirement::CapabilityState
        | BrowserPolicyTargetProofRequirement::AdapterAction => {
            BrowserPolicyCapabilityState::ManualRequired
        }
    }
}

fn action_execution_state(
    policy: &BrowserPolicyValue,
    action: BrowserPolicyRuleAction,
    target_state: BrowserPolicyCapabilityState,
) -> BrowserPolicyActionExecutionState {
    if let Some(state) = EXPLICIT_ACTION_EXECUTION_STATES[policy.execution_mode as usize] {
        return state;
    }
    generic_action_execution_state(policy, action, target_state)
}

fn generic_action_execution_state(
    policy: &BrowserPolicyValue,
    action: BrowserPolicyRuleAction,
    target_state: BrowserPolicyCapabilityState,
) -> BrowserPolicyActionExecutionState {
    let requires_adapter = matches!(
        action,
        BrowserPolicyRuleAction::Block
            | BrowserPolicyRuleAction::Redirect
            | BrowserPolicyRuleAction::CloseTab
            | BrowserPolicyRuleAction::CloseBrowser
            | BrowserPolicyRuleAction::TerminateProcess
            | BrowserPolicyRuleAction::RelaunchManaged
    );
    let manual_required = (requires_adapter && !platform_action_adapter_ready(policy))
        || target_state == BrowserPolicyCapabilityState::ManualRequired;
    let adapter_ready = requires_adapter && !manual_required;
    GENERIC_ACTION_EXECUTION_STATES[usize::from(manual_required) * 2 + usize::from(adapter_ready)]
}

fn ai_authority(policy: &BrowserPolicyValue) -> BrowserPolicyAiAuthority {
    [
        BrowserPolicyAiAuthority::ParentPolicyOnly,
        BrowserPolicyAiAuthority::AiCandidateOnly,
    ][usize::from(policy.portal_ai.allow_rule_suggestions || policy.portal_ai.allow_summaries)]
}

fn ready_or_manual(is_ready: bool) -> BrowserPolicyCapabilityState {
    const CAPABILITY_STATE_BY_READY: [BrowserPolicyCapabilityState; 2] = [
        BrowserPolicyCapabilityState::ManualRequired,
        BrowserPolicyCapabilityState::Ready,
    ];
    CAPABILITY_STATE_BY_READY[usize::from(is_ready)]
}

fn proof_state(
    policy: &BrowserPolicyValue,
    proof: BrowserPolicyEvidenceProofLevel,
) -> BrowserPolicyCapabilityState {
    ready_or_manual(policy.evidence.required_proof == proof)
}

fn platform_browser_bridge_ready(policy: &BrowserPolicyValue) -> bool {
    policy.platforms.windows.may_connect_to_browser_bridge
        && matches!(
            policy.platforms.windows.state.as_deref(),
            Some(constants::browser_policy::CAPABILITY_STATE_READY)
                | Some(constants::browser_policy::CAPABILITY_STATE_SUPPORTED)
        )
}

fn platform_action_adapter_ready(policy: &BrowserPolicyValue) -> bool {
    policy.platforms.windows.enabled
        && matches!(
            policy.platforms.windows.state.as_deref(),
            Some(constants::browser_policy::CAPABILITY_STATE_READY)
                | Some(constants::browser_policy::CAPABILITY_STATE_SUPPORTED)
        )
        && !policy.platforms.windows.allowed_adapters.is_empty()
}

fn exact_url_proof_configured(policy: &BrowserPolicyValue) -> bool {
    (policy.managed_browser.mode == BrowserPolicyManagedBrowserMode::RequiredForExactRules
        || policy.managed_browser.mode == BrowserPolicyManagedBrowserMode::RequiredForAllBrowsing)
        && policy.evidence.required_proof == BrowserPolicyEvidenceProofLevel::FreshManagedActiveTab
}

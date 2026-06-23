use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::BrowserPolicyActionExecutionState;
use ocentra_parent_agent_protocol::BrowserPolicyAiAuthority;
use ocentra_parent_agent_protocol::BrowserPolicyApprovalState;
use ocentra_parent_agent_protocol::BrowserPolicyCapabilityState;
use ocentra_parent_agent_protocol::BrowserPolicyDefaultPosture;
use ocentra_parent_agent_protocol::BrowserPolicyEvidenceProofLevel;
use ocentra_parent_agent_protocol::BrowserPolicyExecutionMode;
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
        compile_note: compile_note(target_proof_requirement, action_execution),
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
    match posture {
        BrowserPolicyDefaultPosture::Allow => BrowserPolicyRuleAction::Allow,
        BrowserPolicyDefaultPosture::Warn => BrowserPolicyRuleAction::Warn,
        BrowserPolicyDefaultPosture::Ask | BrowserPolicyDefaultPosture::AskParent => {
            BrowserPolicyRuleAction::Ask
        }
        BrowserPolicyDefaultPosture::Limit => BrowserPolicyRuleAction::Limit,
        BrowserPolicyDefaultPosture::Block => BrowserPolicyRuleAction::Block,
        BrowserPolicyDefaultPosture::Observe => BrowserPolicyRuleAction::Monitor,
    }
}

fn target_proof_requirement(
    target_type: BrowserPolicyUrlTargetType,
) -> BrowserPolicyTargetProofRequirement {
    match target_type {
        BrowserPolicyUrlTargetType::ExactUrl => {
            BrowserPolicyTargetProofRequirement::ManagedExactUrl
        }
        BrowserPolicyUrlTargetType::Domain
        | BrowserPolicyUrlTargetType::UrlPrefix
        | BrowserPolicyUrlTargetType::DomainOrigin => {
            BrowserPolicyTargetProofRequirement::DomainOrManagedUrl
        }
        BrowserPolicyUrlTargetType::SiteCategory => {
            BrowserPolicyTargetProofRequirement::ClassifierCategory
        }
        BrowserPolicyUrlTargetType::SearchTerms | BrowserPolicyUrlTargetType::VideoChannel => {
            BrowserPolicyTargetProofRequirement::UrlShapeMetadata
        }
        BrowserPolicyUrlTargetType::SocialPlatform
        | BrowserPolicyUrlTargetType::SocialRouteKind
        | BrowserPolicyUrlTargetType::SocialAccountCreation
        | BrowserPolicyUrlTargetType::SocialUnknownAccount
        | BrowserPolicyUrlTargetType::SocialSecondaryAccount
        | BrowserPolicyUrlTargetType::SocialFeed
        | BrowserPolicyUrlTargetType::SocialShortVideoFeed
        | BrowserPolicyUrlTargetType::SocialMessaging
        | BrowserPolicyUrlTargetType::SocialUploadPost
        | BrowserPolicyUrlTargetType::SocialLivestream
        | BrowserPolicyUrlTargetType::UnknownSocialSite => {
            BrowserPolicyTargetProofRequirement::SocialRouteEvidence
        }
        BrowserPolicyUrlTargetType::BrowserGame
        | BrowserPolicyUrlTargetType::BrowserGamePlatform
        | BrowserPolicyUrlTargetType::BrowserGamePortal
        | BrowserPolicyUrlTargetType::BrowserGameUrl
        | BrowserPolicyUrlTargetType::EducationalGame
        | BrowserPolicyUrlTargetType::CloudGaming
        | BrowserPolicyUrlTargetType::WebglCanvasGame
        | BrowserPolicyUrlTargetType::MultiplayerUgcGame
        | BrowserPolicyUrlTargetType::GameChat
        | BrowserPolicyUrlTargetType::GameAccount
        | BrowserPolicyUrlTargetType::GamePurchase
        | BrowserPolicyUrlTargetType::GameLootBox
        | BrowserPolicyUrlTargetType::UnknownGame
        | BrowserPolicyUrlTargetType::UnblockedGameSite => {
            BrowserPolicyTargetProofRequirement::BrowserGameRuntimeSignal
        }
        BrowserPolicyUrlTargetType::BrowserProcess => {
            BrowserPolicyTargetProofRequirement::ProcessDetection
        }
        BrowserPolicyUrlTargetType::Download => {
            BrowserPolicyTargetProofRequirement::DownloadEvidence
        }
        BrowserPolicyUrlTargetType::CapabilityState => {
            BrowserPolicyTargetProofRequirement::CapabilityState
        }
        BrowserPolicyUrlTargetType::BrowserSession => BrowserPolicyTargetProofRequirement::None,
    }
}

fn target_capability_state(
    policy: &BrowserPolicyValue,
    requirement: BrowserPolicyTargetProofRequirement,
) -> BrowserPolicyCapabilityState {
    match requirement {
        BrowserPolicyTargetProofRequirement::None => BrowserPolicyCapabilityState::Ready,
        BrowserPolicyTargetProofRequirement::ManagedExactUrl => managed_exact_url_state(policy),
        BrowserPolicyTargetProofRequirement::DomainOrManagedUrl => domain_evidence_state(policy),
        BrowserPolicyTargetProofRequirement::ClassifierCategory => {
            proof_state(policy, BrowserPolicyEvidenceProofLevel::ClassifierCategory)
        }
        BrowserPolicyTargetProofRequirement::UrlShapeMetadata => {
            proof_state(policy, BrowserPolicyEvidenceProofLevel::UrlShapeMetadata)
        }
        BrowserPolicyTargetProofRequirement::SocialRouteEvidence => social_evidence_state(policy),
        BrowserPolicyTargetProofRequirement::BrowserGameRuntimeSignal => proof_state(
            policy,
            BrowserPolicyEvidenceProofLevel::BrowserGameRuntimeSignal,
        ),
        BrowserPolicyTargetProofRequirement::BrowserPolicyWriter => policy_writer_state(policy),
        BrowserPolicyTargetProofRequirement::ProcessDetection => process_detection_state(policy),
        BrowserPolicyTargetProofRequirement::DownloadEvidence
        | BrowserPolicyTargetProofRequirement::CapabilityState
        | BrowserPolicyTargetProofRequirement::AdapterAction => {
            BrowserPolicyCapabilityState::ManualRequired
        }
    }
}

fn managed_exact_url_state(policy: &BrowserPolicyValue) -> BrowserPolicyCapabilityState {
    if exact_url_proof_configured(policy) && platform_browser_bridge_ready(policy) {
        BrowserPolicyCapabilityState::Ready
    } else {
        BrowserPolicyCapabilityState::ManualRequired
    }
}

fn domain_evidence_state(policy: &BrowserPolicyValue) -> BrowserPolicyCapabilityState {
    match policy.evidence.required_proof {
        BrowserPolicyEvidenceProofLevel::NetworkDomain
        | BrowserPolicyEvidenceProofLevel::ManagedActiveTab
        | BrowserPolicyEvidenceProofLevel::ManagedTabList
        | BrowserPolicyEvidenceProofLevel::FreshManagedTabList
        | BrowserPolicyEvidenceProofLevel::FreshManagedActiveTab => {
            BrowserPolicyCapabilityState::Ready
        }
        _ => BrowserPolicyCapabilityState::ManualRequired,
    }
}

fn proof_state(
    policy: &BrowserPolicyValue,
    proof: BrowserPolicyEvidenceProofLevel,
) -> BrowserPolicyCapabilityState {
    if policy.evidence.required_proof == proof {
        BrowserPolicyCapabilityState::Ready
    } else {
        BrowserPolicyCapabilityState::ManualRequired
    }
}

fn social_evidence_state(policy: &BrowserPolicyValue) -> BrowserPolicyCapabilityState {
    if policy.evidence.required_proof == BrowserPolicyEvidenceProofLevel::SocialRouteEvidence
        && policy.approvals.state == BrowserPolicyApprovalState::Approved
    {
        BrowserPolicyCapabilityState::Ready
    } else {
        BrowserPolicyCapabilityState::ManualRequired
    }
}

fn policy_writer_state(policy: &BrowserPolicyValue) -> BrowserPolicyCapabilityState {
    let has_policy_writer = policy
        .managed_browser
        .integration_mechanisms
        .contains(&BrowserPolicyManagedBrowserIntegrationMechanism::BrowserPolicy);
    if has_policy_writer
        && !policy.managed_browser.policy_writer_controls.is_empty()
        && platform_action_adapter_ready(policy)
    {
        BrowserPolicyCapabilityState::Ready
    } else {
        BrowserPolicyCapabilityState::ManualRequired
    }
}

fn process_detection_state(policy: &BrowserPolicyValue) -> BrowserPolicyCapabilityState {
    if policy.discovery.detect_unmanaged_browsers
        && !policy.unmanaged_browser.classification_targets.is_empty()
        && policy.evidence.required_proof == BrowserPolicyEvidenceProofLevel::ProcessRunning
    {
        BrowserPolicyCapabilityState::Ready
    } else {
        BrowserPolicyCapabilityState::ManualRequired
    }
}

fn action_execution_state(
    policy: &BrowserPolicyValue,
    action: BrowserPolicyRuleAction,
    target_state: BrowserPolicyCapabilityState,
) -> BrowserPolicyActionExecutionState {
    match policy.execution_mode {
        BrowserPolicyExecutionMode::Observe => BrowserPolicyActionExecutionState::ObserveOnly,
        BrowserPolicyExecutionMode::DryRun => BrowserPolicyActionExecutionState::DryRunNoExecution,
        _ if action_requires_adapter(action) && !platform_action_adapter_ready(policy) => {
            BrowserPolicyActionExecutionState::ManualRequired
        }
        _ if target_state == BrowserPolicyCapabilityState::ManualRequired => {
            BrowserPolicyActionExecutionState::ManualRequired
        }
        _ if action_requires_adapter(action) => BrowserPolicyActionExecutionState::AdapterReady,
        _ => BrowserPolicyActionExecutionState::DeterministicParentPolicy,
    }
}

fn action_requires_adapter(action: BrowserPolicyRuleAction) -> bool {
    matches!(
        action,
        BrowserPolicyRuleAction::Block
            | BrowserPolicyRuleAction::Redirect
            | BrowserPolicyRuleAction::CloseTab
            | BrowserPolicyRuleAction::CloseBrowser
            | BrowserPolicyRuleAction::TerminateProcess
            | BrowserPolicyRuleAction::RelaunchManaged
    )
}

fn ai_authority(policy: &BrowserPolicyValue) -> BrowserPolicyAiAuthority {
    if policy.portal_ai.allow_rule_suggestions || policy.portal_ai.allow_summaries {
        BrowserPolicyAiAuthority::AiCandidateOnly
    } else {
        BrowserPolicyAiAuthority::ParentPolicyOnly
    }
}

fn compile_note(
    requirement: BrowserPolicyTargetProofRequirement,
    action_execution: BrowserPolicyActionExecutionState,
) -> &'static str {
    if matches!(
        action_execution,
        BrowserPolicyActionExecutionState::ObserveOnly
            | BrowserPolicyActionExecutionState::DryRunNoExecution
    ) {
        return constants::browser_policy::COMPILE_NOTE_OBSERVE_DRY_RUN;
    }

    match requirement {
        BrowserPolicyTargetProofRequirement::ManagedExactUrl => {
            constants::browser_policy::COMPILE_NOTE_MANAGED_EXACT_URL
        }
        BrowserPolicyTargetProofRequirement::DomainOrManagedUrl => {
            constants::browser_policy::COMPILE_NOTE_DOMAIN_OR_MANAGED
        }
        BrowserPolicyTargetProofRequirement::ClassifierCategory => {
            constants::browser_policy::COMPILE_NOTE_CLASSIFIER_REQUIRED
        }
        BrowserPolicyTargetProofRequirement::UrlShapeMetadata => {
            constants::browser_policy::COMPILE_NOTE_URL_METADATA_REQUIRED
        }
        BrowserPolicyTargetProofRequirement::SocialRouteEvidence => {
            constants::browser_policy::COMPILE_NOTE_SOCIAL_REQUIRED
        }
        BrowserPolicyTargetProofRequirement::BrowserGameRuntimeSignal => {
            constants::browser_policy::COMPILE_NOTE_GAME_REQUIRED
        }
        BrowserPolicyTargetProofRequirement::BrowserPolicyWriter => {
            constants::browser_policy::COMPILE_NOTE_POLICY_WRITER_REQUIRED
        }
        BrowserPolicyTargetProofRequirement::ProcessDetection => {
            constants::browser_policy::COMPILE_NOTE_PROCESS_REQUIRED
        }
        BrowserPolicyTargetProofRequirement::AdapterAction => {
            constants::browser_policy::COMPILE_NOTE_ACTION_ADAPTER_REQUIRED
        }
        BrowserPolicyTargetProofRequirement::DownloadEvidence
        | BrowserPolicyTargetProofRequirement::CapabilityState
        | BrowserPolicyTargetProofRequirement::None => {
            constants::browser_policy::COMPILE_NOTE_PARENT_POLICY
        }
    }
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

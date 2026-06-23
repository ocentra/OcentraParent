use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::policy_constants;
use ocentra_parent_agent_protocol::BrowserPolicyBudgets;
use ocentra_parent_agent_protocol::BrowserPolicyCapability;
use ocentra_parent_agent_protocol::BrowserPolicyCapabilityRegistry;
use ocentra_parent_agent_protocol::BrowserPolicyCapabilityState;
use ocentra_parent_agent_protocol::BrowserPolicyDefaultPosture;
use ocentra_parent_agent_protocol::BrowserPolicyEffectivePolicy;
use ocentra_parent_agent_protocol::BrowserPolicyEffectiveRule;
use ocentra_parent_agent_protocol::BrowserPolicyEvidenceProofLevel;
use ocentra_parent_agent_protocol::BrowserPolicyManagedBrowserMode;
use ocentra_parent_agent_protocol::BrowserPolicyProofFallback;
use ocentra_parent_agent_protocol::BrowserPolicyRejectionReason;
use ocentra_parent_agent_protocol::BrowserPolicyRule;
use ocentra_parent_agent_protocol::BrowserPolicyUrlTargetType;
use ocentra_parent_agent_protocol::BrowserPolicyValue;

use crate::browser_policy_compiler_assessment::{compile_rule_assessment, rule_action};

pub(crate) fn compile_browser_policy(
    policy: &BrowserPolicyValue,
    revision_id: &str,
    compiled_at: &str,
) -> Result<BrowserPolicyEffectivePolicy, BrowserPolicyRejectionReason> {
    validate_browser_policy(policy)?;
    let rules = if policy.enabled {
        source_rules(policy)
            .iter()
            .filter(|rule| rule.enabled)
            .map(|rule| {
                rule_target(rule).map(|(target_type, target_value)| {
                    let action = rule_action(rule, policy.default_posture);
                    let assessment = compile_rule_assessment(policy, target_type, action);
                    BrowserPolicyEffectiveRule {
                        rule_id: rule.rule_id.clone(),
                        target_type,
                        target_value,
                        default_posture: policy.default_posture,
                        evidence: policy.evidence.clone(),
                        action,
                        target_proof_requirement: assessment.target_proof_requirement,
                        capability_state: assessment.capability_state,
                        action_execution: assessment.action_execution,
                        ai_authority: assessment.ai_authority,
                        compile_note: assessment.compile_note.to_string(),
                    }
                })
            })
            .collect::<Result<Vec<_>, _>>()?
    } else {
        Vec::new()
    };
    Ok(BrowserPolicyEffectivePolicy {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        policy_id: policy.policy_id.clone(),
        revision_id: revision_id.to_string(),
        compiled_hash: compiled_hash_for_revision(revision_id),
        compiled_at: compiled_at.to_string(),
        execution_mode: policy.execution_mode,
        default_posture: effective_default_posture(policy),
        fallback_posture: policy.fallback_posture,
        discovery: policy.discovery.clone(),
        budgets: BrowserPolicyBudgets {
            enabled: policy.budgets.enabled,
            default_daily_minutes: policy.budgets.default_daily_minutes,
            counting_mode: policy.budgets.counting_mode,
        },
        rules,
    })
}

pub(crate) fn browser_policy_capability_registry(
    generated_at: &str,
) -> BrowserPolicyCapabilityRegistry {
    BrowserPolicyCapabilityRegistry {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        generated_at: generated_at.to_string(),
        capabilities: browser_policy_capabilities(generated_at),
    }
}

fn browser_policy_capabilities(generated_at: &str) -> Vec<BrowserPolicyCapability> {
    vec![
        browser_policy_capability(
            constants::browser_policy::DEFAULT_CAPABILITY_ID,
            constants::browser_policy::DEFAULT_CAPABILITY_LABEL,
            BrowserPolicyCapabilityState::Unknown,
            constants::browser_policy::DEFAULT_CAPABILITY_REASON,
            generated_at,
            vec![
                constants::browser_policy::WRITES_TO_REQUIRED_PROOF,
                constants::browser_policy::WRITES_TO_WHEN_PROOF_UNAVAILABLE,
                constants::browser_policy::WRITES_TO_MANAGED_BROWSER_MODE,
            ],
        ),
        browser_policy_capability(
            constants::browser_policy::POLICY_WRITER_CAPABILITY_ID,
            constants::browser_policy::POLICY_WRITER_CAPABILITY_LABEL,
            BrowserPolicyCapabilityState::ManualRequired,
            constants::browser_policy::POLICY_WRITER_CAPABILITY_REASON,
            generated_at,
            vec![
                constants::browser_policy::WRITES_TO_MANAGED_BROWSER_POLICY_WRITER_CONTROLS,
                constants::browser_policy::WRITES_TO_MANAGED_BROWSER_POLICY_WRITER_FALLBACK,
                constants::browser_policy::WRITES_TO_URL_ALLOW_LIST,
                constants::browser_policy::WRITES_TO_URL_BLOCK_LIST,
            ],
        ),
        browser_policy_capability(
            constants::browser_policy::DOMAIN_CAPABILITY_ID,
            constants::browser_policy::DOMAIN_CAPABILITY_LABEL,
            BrowserPolicyCapabilityState::ManualRequired,
            constants::browser_policy::COMPILE_NOTE_DOMAIN_OR_MANAGED,
            generated_at,
            vec![constants::browser_policy::WRITES_TO_REQUIRED_PROOF],
        ),
        browser_policy_capability(
            constants::browser_policy::CLASSIFIER_CAPABILITY_ID,
            constants::browser_policy::CLASSIFIER_CAPABILITY_LABEL,
            BrowserPolicyCapabilityState::ManualRequired,
            constants::browser_policy::COMPILE_NOTE_CLASSIFIER_REQUIRED,
            generated_at,
            vec![constants::browser_policy::WRITES_TO_REQUIRED_PROOF],
        ),
        browser_policy_capability(
            constants::browser_policy::SOCIAL_CAPABILITY_ID,
            constants::browser_policy::SOCIAL_CAPABILITY_LABEL,
            BrowserPolicyCapabilityState::ManualRequired,
            constants::browser_policy::COMPILE_NOTE_SOCIAL_REQUIRED,
            generated_at,
            vec![constants::browser_policy::WRITES_TO_APPROVAL_STATE],
        ),
        browser_policy_capability(
            constants::browser_policy::GAME_CAPABILITY_ID,
            constants::browser_policy::GAME_CAPABILITY_LABEL,
            BrowserPolicyCapabilityState::ManualRequired,
            constants::browser_policy::COMPILE_NOTE_GAME_REQUIRED,
            generated_at,
            vec![
                constants::browser_policy::WRITES_TO_BROWSER_GAME_CLOUD_GAMING_APPROVAL,
                constants::browser_policy::WRITES_TO_BROWSER_GAME_DAILY_BUDGET_MINUTES,
            ],
        ),
        browser_policy_capability(
            constants::browser_policy::ACTION_ADAPTER_CAPABILITY_ID,
            constants::browser_policy::ACTION_ADAPTER_CAPABILITY_LABEL,
            BrowserPolicyCapabilityState::ManualRequired,
            constants::browser_policy::COMPILE_NOTE_ACTION_ADAPTER_REQUIRED,
            generated_at,
            vec![constants::browser_policy::WRITES_TO_ALLOWED_ACTIONS],
        ),
        browser_policy_capability(
            constants::browser_policy::PROCESS_CAPABILITY_ID,
            constants::browser_policy::PROCESS_CAPABILITY_LABEL,
            BrowserPolicyCapabilityState::ManualRequired,
            constants::browser_policy::COMPILE_NOTE_PROCESS_REQUIRED,
            generated_at,
            vec![constants::browser_policy::WRITES_TO_UNMANAGED_BROWSER_CLASSIFICATION_TARGETS],
        ),
    ]
}

fn browser_policy_capability(
    capability_id: &str,
    capability_display_name: &str,
    state: BrowserPolicyCapabilityState,
    reason: &str,
    generated_at: &str,
    affected_writes_to: Vec<&str>,
) -> BrowserPolicyCapability {
    BrowserPolicyCapability {
        capability_id: capability_id.to_string(),
        state,
        label: capability_display_name.to_string(),
        affected_writes_to: affected_writes_to
            .into_iter()
            .map(ToString::to_string)
            .collect(),
        checked_at: generated_at.to_string(),
        reason: Some(reason.to_string()),
    }
}

fn validate_browser_policy(
    policy: &BrowserPolicyValue,
) -> Result<(), BrowserPolicyRejectionReason> {
    if policy.default_posture == BrowserPolicyDefaultPosture::Limit
        && (!policy.budgets.enabled || policy.budgets.default_daily_minutes.is_none())
        && policy.fallback_posture.is_none()
    {
        return Err(BrowserPolicyRejectionReason::MissingBudgetOrFallback);
    }
    if policy
        .rules
        .allowed_target_types
        .contains(&BrowserPolicyUrlTargetType::ExactUrl)
        || source_rules(policy).iter().any(rule_uses_exact_url)
    {
        validate_exact_url_proof(policy)?;
    }
    Ok(())
}

fn validate_exact_url_proof(
    policy: &BrowserPolicyValue,
) -> Result<(), BrowserPolicyRejectionReason> {
    if policy.evidence.proof_fallback.is_some()
        || policy.evidence.when_proof_unavailable != BrowserPolicyProofFallback::MarkUnavailable
    {
        return Ok(());
    }
    if (policy.managed_browser.mode == BrowserPolicyManagedBrowserMode::RequiredForExactRules
        || policy.managed_browser.mode == BrowserPolicyManagedBrowserMode::RequiredForAllBrowsing)
        && policy.evidence.required_proof == BrowserPolicyEvidenceProofLevel::FreshManagedActiveTab
    {
        return Ok(());
    }
    Err(BrowserPolicyRejectionReason::MissingManagedProofOrFallback)
}

fn source_rules(policy: &BrowserPolicyValue) -> &[BrowserPolicyRule] {
    if policy.rules.items.is_empty() {
        &policy.rules.entries
    } else {
        &policy.rules.items
    }
}

fn rule_uses_exact_url(rule: &BrowserPolicyRule) -> bool {
    rule.target_type == Some(BrowserPolicyUrlTargetType::ExactUrl)
        || rule
            .target
            .as_ref()
            .map(|target| target.kind == BrowserPolicyUrlTargetType::ExactUrl)
            .unwrap_or(false)
}

fn rule_target(
    rule: &BrowserPolicyRule,
) -> Result<(BrowserPolicyUrlTargetType, String), BrowserPolicyRejectionReason> {
    if let (Some(target_type), Some(target_value)) = (rule.target_type, rule.target_value.clone()) {
        return Ok((target_type, target_value));
    }
    if let Some(target) = &rule.target {
        if let Some(target_value) = target.values.first() {
            return Ok((target.kind, target_value.clone()));
        }
    }
    Err(BrowserPolicyRejectionReason::InvalidRequest)
}

fn effective_default_posture(policy: &BrowserPolicyValue) -> BrowserPolicyDefaultPosture {
    if policy.enabled {
        policy.default_posture
    } else {
        BrowserPolicyDefaultPosture::Allow
    }
}

fn compiled_hash_for_revision(revision_id: &str) -> String {
    let mut compiled_hash = constants::browser_policy::COMPILED_HASH_PREFIX.to_string();
    compiled_hash.push_str(revision_id);
    compiled_hash
}

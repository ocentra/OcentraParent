use ocentra_parent_agent_protocol::{
    constants, policy_constants, BrowserPolicyBudgets, BrowserPolicyCapability,
    BrowserPolicyCapabilityRegistry, BrowserPolicyCapabilityState, BrowserPolicyDefaultPosture,
    BrowserPolicyEffectivePolicy, BrowserPolicyEffectiveRule, BrowserPolicyEvidenceProofLevel,
    BrowserPolicyManagedBrowserMode, BrowserPolicyProofFallback, BrowserPolicyRejectionReason,
    BrowserPolicyRule, BrowserPolicyUrlTargetType, BrowserPolicyValue,
};

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
                rule_target(rule).map(|(target_type, target_value)| BrowserPolicyEffectiveRule {
                    rule_id: rule.rule_id.clone(),
                    target_type,
                    target_value,
                    default_posture: policy.default_posture,
                    evidence: policy.evidence.clone(),
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
        capabilities: vec![BrowserPolicyCapability {
            capability_id: constants::browser_policy::DEFAULT_CAPABILITY_ID.to_string(),
            state: BrowserPolicyCapabilityState::Unknown,
            label: constants::browser_policy::DEFAULT_CAPABILITY_LABEL.to_string(),
            affected_writes_to: vec![
                constants::browser_policy::WRITES_TO_REQUIRED_PROOF.to_string(),
                constants::browser_policy::WRITES_TO_WHEN_PROOF_UNAVAILABLE.to_string(),
                constants::browser_policy::WRITES_TO_MANAGED_BROWSER_MODE.to_string(),
            ],
            checked_at: generated_at.to_string(),
            reason: Some(constants::browser_policy::DEFAULT_CAPABILITY_REASON.to_string()),
        }],
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

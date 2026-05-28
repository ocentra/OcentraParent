use ocentra_parent_agent_protocol::{
    constants, policy_constants, BrowserPolicyBudgets, BrowserPolicyCapability,
    BrowserPolicyCapabilityRegistry, BrowserPolicyCapabilityState, BrowserPolicyDefaultPosture,
    BrowserPolicyEffectivePolicy, BrowserPolicyEffectiveRule, BrowserPolicyEvidenceProofLevel,
    BrowserPolicyManagedBrowserMode, BrowserPolicyRejectionReason, BrowserPolicyUrlTargetType,
    BrowserPolicyValue,
};

pub(crate) fn compile_browser_policy(
    policy: &BrowserPolicyValue,
    revision_id: &str,
    compiled_at: &str,
) -> Result<BrowserPolicyEffectivePolicy, BrowserPolicyRejectionReason> {
    validate_browser_policy(policy)?;
    let rules = if policy.enabled {
        policy
            .rules
            .entries
            .iter()
            .filter(|rule| rule.enabled)
            .map(|rule| BrowserPolicyEffectiveRule {
                rule_id: rule.rule_id.clone(),
                target_type: rule.target_type,
                target_value: rule.target_value.clone(),
                default_posture: policy.default_posture,
                evidence: policy.evidence.clone(),
            })
            .collect()
    } else {
        Vec::new()
    };
    Ok(BrowserPolicyEffectivePolicy {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        policy_id: policy.policy_id.clone(),
        revision_id: revision_id.to_string(),
        compiled_hash: compiled_hash_for_revision(revision_id),
        compiled_at: compiled_at.to_string(),
        default_posture: effective_default_posture(policy),
        fallback_posture: policy.fallback_posture,
        budgets: BrowserPolicyBudgets {
            default_daily_minutes: policy.budgets.default_daily_minutes,
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
                constants::browser_policy::WRITES_TO_REQUIRED_PROOF.to_string()
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
        && policy.budgets.default_daily_minutes.is_none()
        && policy.fallback_posture.is_none()
    {
        return Err(BrowserPolicyRejectionReason::MissingBudgetOrFallback);
    }
    if policy
        .rules
        .allowed_target_types
        .contains(&BrowserPolicyUrlTargetType::ExactUrl)
        && policy.evidence.proof_fallback.is_none()
        && (policy.managed_browser.mode != BrowserPolicyManagedBrowserMode::RequiredForExactRules
            || policy.evidence.required_proof
                != BrowserPolicyEvidenceProofLevel::FreshManagedActiveTab)
    {
        return Err(BrowserPolicyRejectionReason::MissingManagedProofOrFallback);
    }
    Ok(())
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

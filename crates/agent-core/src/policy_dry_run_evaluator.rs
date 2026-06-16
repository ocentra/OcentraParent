use ocentra_parent_agent_protocol::{
    policy_constants as policy, LocalAiSafetyResult, LocalAiUnknownState, ParentEvidenceReference,
    PolicyAction, PolicyDecision, PolicyDecisionHandoffState, PolicyRule, PolicyTarget,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PolicyDryRunEvaluationInput {
    pub decision_id: String,
    pub evaluated_at: String,
    pub observed_target: PolicyTarget,
    pub observed_target_aliases: Vec<PolicyTarget>,
    pub parent_rules: Vec<PolicyRule>,
    pub local_ai_result: Option<LocalAiSafetyResult>,
    pub evidence_references: Vec<ParentEvidenceReference>,
    pub expires_at: Option<String>,
}

pub fn evaluate_policy_dry_run(input: PolicyDryRunEvaluationInput) -> PolicyDecision {
    let PolicyDryRunEvaluationInput {
        decision_id,
        evaluated_at,
        observed_target,
        observed_target_aliases,
        parent_rules,
        local_ai_result,
        evidence_references,
        expires_at,
    } = input;
    let input = PolicyDryRunEvaluationInput {
        decision_id,
        evaluated_at,
        observed_target,
        observed_target_aliases,
        parent_rules,
        local_ai_result,
        evidence_references,
        expires_at,
    };
    let evidence_references = decision_evidence_references(&input);
    let local_ai_result_id = input
        .local_ai_result
        .as_ref()
        .map(|result| result.result_id.clone());

    if evidence_references.is_empty() {
        return decision(
            &input,
            PolicyAction::Unknown,
            vec![policy::REASON_MISSING_EVIDENCE.to_string()],
            Vec::new(),
            evidence_references,
            local_ai_result_id,
        );
    }

    let applicable_rules = applicable_rules(&input);
    if !applicable_rules.is_empty() {
        return decision_from_rules(
            &input,
            applicable_rules,
            evidence_references,
            local_ai_result_id,
        );
    }

    let reason_codes = no_matching_rule_reason_codes(input.local_ai_result.as_ref());
    decision(
        &input,
        PolicyAction::Unknown,
        reason_codes,
        Vec::new(),
        evidence_references,
        local_ai_result_id,
    )
}

fn applicable_rules(input: &PolicyDryRunEvaluationInput) -> Vec<&PolicyRule> {
    let mut rules = input
        .parent_rules
        .iter()
        .filter(|rule| rule_applies(input, rule))
        .collect::<Vec<_>>();
    rules.sort_by(|left, right| {
        right
            .priority
            .cmp(&left.priority)
            .then_with(|| left.rule_id.cmp(&right.rule_id))
    });
    rules
}

fn rule_applies(input: &PolicyDryRunEvaluationInput, rule: &PolicyRule) -> bool {
    rule.enabled
        && target_applies(input, &rule.target)
        && rule_is_effective_at(&input.evaluated_at, rule)
}

fn target_applies(input: &PolicyDryRunEvaluationInput, rule_target: &PolicyTarget) -> bool {
    target_matches(&input.observed_target, rule_target)
        || input
            .observed_target_aliases
            .iter()
            .any(|target| target_matches(target, rule_target))
}

fn target_matches(observed: &PolicyTarget, rule_target: &PolicyTarget) -> bool {
    observed.target_type == rule_target.target_type
        && observed.target_value == rule_target.target_value
}

fn rule_is_effective_at(evaluated_at: &str, rule: &PolicyRule) -> bool {
    if let Some(effective_from) = &rule.effective_from {
        if effective_from.as_str() > evaluated_at {
            return false;
        }
    }

    if let Some(effective_until) = &rule.effective_until {
        if effective_until.as_str() <= evaluated_at {
            return false;
        }
    }

    true
}

fn decision_from_rules(
    input: &PolicyDryRunEvaluationInput,
    applicable_rules: Vec<&PolicyRule>,
    evidence_references: Vec<ParentEvidenceReference>,
    local_ai_result_id: Option<String>,
) -> PolicyDecision {
    let top_priority = applicable_rules[0].priority;
    let top_rules = applicable_rules
        .into_iter()
        .take_while(|rule| rule.priority == top_priority)
        .collect::<Vec<_>>();
    let top_action = top_rules[0].action;
    let has_conflict = top_rules.iter().any(|rule| rule.action != top_action);

    if has_conflict {
        let mut reason_codes = vec![policy::REASON_POLICY_CONFLICT.to_string()];
        for rule in &top_rules {
            push_unique(&mut reason_codes, rule.reason_code.clone());
        }
        let rule_ids = top_rules
            .iter()
            .map(|rule| rule.rule_id.clone())
            .collect::<Vec<_>>();
        return decision(
            input,
            PolicyAction::Unknown,
            reason_codes,
            rule_ids,
            evidence_references,
            local_ai_result_id,
        );
    }

    decision(
        input,
        top_action,
        vec![top_rules[0].reason_code.clone()],
        vec![top_rules[0].rule_id.clone()],
        evidence_references,
        local_ai_result_id,
    )
}

fn decision_evidence_references(
    input: &PolicyDryRunEvaluationInput,
) -> Vec<ParentEvidenceReference> {
    if !input.evidence_references.is_empty() {
        return input.evidence_references.clone();
    }

    input
        .local_ai_result
        .as_ref()
        .map(|result| result.evidence_references.clone())
        .unwrap_or_default()
}

fn no_matching_rule_reason_codes(local_ai_result: Option<&LocalAiSafetyResult>) -> Vec<String> {
    let mut reason_codes = Vec::new();
    push_unique(
        &mut reason_codes,
        policy::REASON_NO_MATCHING_PARENT_RULE.to_string(),
    );

    match local_ai_result {
        Some(result) => {
            if result.unknown_state != LocalAiUnknownState::None {
                push_unique(
                    &mut reason_codes,
                    result.unknown_state.as_protocol_str().to_string(),
                );
            }
            for reason_code in &result.reason_codes {
                push_unique(&mut reason_codes, reason_code.clone());
            }
        }
        None => push_unique(
            &mut reason_codes,
            policy::REASON_LOCAL_AI_RESULT_MISSING.to_string(),
        ),
    }

    reason_codes
}

fn decision(
    input: &PolicyDryRunEvaluationInput,
    action: PolicyAction,
    reason_codes: Vec<String>,
    rule_ids: Vec<String>,
    evidence_references: Vec<ParentEvidenceReference>,
    local_ai_result_id: Option<String>,
) -> PolicyDecision {
    PolicyDecision {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        decision_id: input.decision_id.clone(),
        action,
        reason_codes,
        evidence_references,
        rule_ids,
        local_ai_result_id,
        dry_run: true,
        enforcement_handoff_state: PolicyDecisionHandoffState::Disabled,
        expires_at: input.expires_at.clone(),
    }
}

fn push_unique(values: &mut Vec<String>, value: String) {
    if !values.contains(&value) {
        values.push(value);
    }
}

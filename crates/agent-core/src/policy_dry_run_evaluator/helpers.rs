use ocentra_parent_agent_protocol::activity::policy::ParentEvidenceReference;
use ocentra_parent_agent_protocol::activity::policy::PolicyAction;
use ocentra_parent_agent_protocol::activity::policy::PolicyRule;
use ocentra_parent_agent_protocol::activity::policy::PolicyTarget;
use ocentra_parent_agent_protocol::policy_constants as policy;

use super::{decision, PolicyDryRunEvaluationInput};

pub(super) fn applicable_rules(input: &PolicyDryRunEvaluationInput) -> Vec<&PolicyRule> {
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

pub(super) fn decision_from_rules(
    input: &PolicyDryRunEvaluationInput,
    applicable_rules: Vec<&PolicyRule>,
    evidence_references: Vec<ParentEvidenceReference>,
    local_ai_result_id: Option<String>,
) -> ocentra_parent_agent_protocol::activity::policy::PolicyDecision {
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
    rule.effective_from
        .as_ref()
        .is_none_or(|effective_from| effective_from.as_str() <= evaluated_at)
        && rule
            .effective_until
            .as_ref()
            .is_none_or(|effective_until| effective_until.as_str() > evaluated_at)
}

pub(super) fn push_unique(values: &mut Vec<String>, value: String) {
    if !values.contains(&value) {
        values.push(value);
    }
}

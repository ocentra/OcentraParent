use ocentra_parent_agent_protocol::activity::local_ai::{LocalAiSafetyResult, LocalAiUnknownState};
use ocentra_parent_agent_protocol::activity::policy::ParentEvidenceReference;
use ocentra_parent_agent_protocol::activity::policy::PolicyAction;
use ocentra_parent_agent_protocol::activity::policy::PolicyDecision;
use ocentra_parent_agent_protocol::activity::policy::PolicyDecisionHandoffState;
use ocentra_parent_agent_protocol::activity::policy::PolicyRule;
use ocentra_parent_agent_protocol::activity::policy::PolicyTarget;
use ocentra_parent_agent_protocol::policy_constants as policy;

#[path = "policy_dry_run_evaluator/helpers.rs"]
mod helpers;

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

    let applicable_rules = helpers::applicable_rules(&input);
    if !applicable_rules.is_empty() {
        return helpers::decision_from_rules(
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
    helpers::push_unique(
        &mut reason_codes,
        policy::REASON_NO_MATCHING_PARENT_RULE.to_string(),
    );

    match local_ai_result {
        Some(result) => {
            if result.unknown_state != LocalAiUnknownState::None {
                helpers::push_unique(
                    &mut reason_codes,
                    result.unknown_state.as_protocol_str().to_string(),
                );
            }
            for reason_code in &result.reason_codes {
                helpers::push_unique(&mut reason_codes, reason_code.clone());
            }
        }
        None => helpers::push_unique(
            &mut reason_codes,
            policy::REASON_LOCAL_AI_RESULT_MISSING.to_string(),
        ),
    }

    reason_codes
}

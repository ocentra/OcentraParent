use ocentra_parent_agent_protocol::activity::local_ai::LocalAiUnknownState;
use ocentra_parent_agent_protocol::activity::policy::PolicyAction;
use ocentra_parent_agent_protocol::activity::policy::PolicyDecisionHandoffState;
use ocentra_parent_agent_protocol::policy_constants as policy;

use crate::policy_dry_run_evaluator_support::{evidence, input_with_rules, local_ai_result, rule};
use ocentra_parent_agent_core::policy_dry_run_evaluator::evaluate_policy_dry_run;

#[test]
fn dry_run_block_rule_overrides_ambiguous_local_ai_allow_output() {
    let decision = evaluate_policy_dry_run(input_with_rules(
        vec![rule(
            policy::TEST_BLOCK_RULE_ID,
            PolicyAction::Block,
            policy::TEST_REASON_PARENT_BLOCK,
            10,
        )],
        Some(local_ai_result(
            PolicyAction::Allow,
            LocalAiUnknownState::LowConfidence,
            policy::TEST_REASON_AI_ALLOW,
        )),
        vec![evidence()],
    ));

    assert_eq!(decision.action, PolicyAction::Block);
    assert_eq!(
        decision.reason_codes,
        vec![policy::TEST_REASON_PARENT_BLOCK.to_string()]
    );
    assert_eq!(
        decision.rule_ids,
        vec![policy::TEST_BLOCK_RULE_ID.to_string()]
    );
    assert_eq!(
        decision.local_ai_result_id,
        Some(policy::TEST_AI_RESULT_ID.to_string())
    );
    assert!(decision.dry_run);
    assert_eq!(
        decision.enforcement_handoff_state,
        PolicyDecisionHandoffState::Disabled
    );
}

#[test]
fn dry_run_allow_rule_produces_disabled_allow_decision() {
    let decision = evaluate_policy_dry_run(input_with_rules(
        vec![rule(
            policy::TEST_ALLOW_RULE_ID,
            PolicyAction::Allow,
            policy::TEST_REASON_PARENT_ALLOW,
            10,
        )],
        Some(local_ai_result(
            PolicyAction::Allow,
            LocalAiUnknownState::None,
            policy::TEST_REASON_AI_ALLOW,
        )),
        vec![evidence()],
    ));

    assert_eq!(decision.action, PolicyAction::Allow);
    assert_eq!(
        decision.reason_codes,
        vec![policy::TEST_REASON_PARENT_ALLOW.to_string()]
    );
    assert_eq!(decision.rule_ids.len(), 1);
    assert_eq!(decision.evidence_references, vec![evidence()]);
    assert!(decision.dry_run);
}

#[test]
fn dry_run_time_limit_rule_produces_budget_preview_without_enforcement() {
    let decision = evaluate_policy_dry_run(input_with_rules(
        vec![rule(
            policy::TEST_TIME_LIMIT_RULE_ID,
            PolicyAction::TimeLimit,
            policy::TEST_REASON_PARENT_TIME_LIMIT,
            20,
        )],
        Some(local_ai_result(
            PolicyAction::Block,
            LocalAiUnknownState::LowConfidence,
            policy::TEST_REASON_AI_BLOCK,
        )),
        vec![evidence()],
    ));

    assert_eq!(decision.action, PolicyAction::TimeLimit);
    assert_eq!(
        decision.rule_ids,
        vec![policy::TEST_TIME_LIMIT_RULE_ID.to_string()]
    );
    assert_eq!(
        decision.expires_at,
        Some(policy::TEST_EXPIRES_AT.to_string())
    );
    assert_eq!(
        decision.enforcement_handoff_state,
        PolicyDecisionHandoffState::Disabled
    );
}

#[test]
fn dry_run_ask_parent_rule_returns_permission_decision_shape() {
    let decision = evaluate_policy_dry_run(input_with_rules(
        vec![rule(
            policy::TEST_ASK_PARENT_RULE_ID,
            PolicyAction::AskParent,
            policy::TEST_REASON_PARENT_ASK,
            30,
        )],
        Some(local_ai_result(
            PolicyAction::Allow,
            LocalAiUnknownState::None,
            policy::TEST_REASON_AI_ALLOW,
        )),
        vec![evidence()],
    ));

    assert_eq!(decision.action, PolicyAction::AskParent);
    assert_eq!(
        decision.reason_codes,
        vec![policy::TEST_REASON_PARENT_ASK.to_string()]
    );
    assert_eq!(
        decision.enforcement_handoff_state,
        PolicyDecisionHandoffState::Disabled
    );
}

use ocentra_parent_agent_protocol::activity::local_ai::LocalAiUnknownState;
use ocentra_parent_agent_protocol::activity::policy::ParentEvidenceReference;
use ocentra_parent_agent_protocol::activity::policy::PolicyAction;
use ocentra_parent_agent_protocol::policy_constants as policy;

use crate::policy_dry_run_evaluator_support::{evidence, input_with_rules, local_ai_result, rule};
use ocentra_parent_agent_core::policy_dry_run_evaluator::evaluate_policy_dry_run;

#[test]
fn dry_run_same_priority_parent_rule_conflict_returns_unknown() {
    let decision = evaluate_policy_dry_run(input_with_rules(
        vec![
            rule(
                policy::TEST_ALLOW_RULE_ID,
                PolicyAction::Allow,
                policy::TEST_REASON_PARENT_ALLOW,
                10,
            ),
            rule(
                policy::TEST_BLOCK_RULE_ID,
                PolicyAction::Block,
                policy::TEST_REASON_PARENT_BLOCK,
                10,
            ),
        ],
        Some(local_ai_result(
            PolicyAction::Allow,
            LocalAiUnknownState::None,
            policy::TEST_REASON_AI_ALLOW,
        )),
        vec![evidence()],
    ));

    assert_eq!(decision.action, PolicyAction::Unknown);
    assert_eq!(
        decision.reason_codes,
        vec![
            policy::REASON_POLICY_CONFLICT.to_string(),
            policy::TEST_REASON_PARENT_ALLOW.to_string(),
            policy::TEST_REASON_PARENT_BLOCK.to_string(),
        ]
    );
    assert_eq!(
        decision.rule_ids,
        vec![
            policy::TEST_ALLOW_RULE_ID.to_string(),
            policy::TEST_BLOCK_RULE_ID.to_string(),
        ]
    );
}

#[test]
fn dry_run_without_matching_enabled_rule_keeps_local_ai_as_evidence_only() {
    let mut disabled_rule = rule(
        policy::TEST_DISABLED_RULE_ID,
        PolicyAction::Block,
        policy::TEST_REASON_DISABLED,
        10,
    );
    disabled_rule.enabled = false;
    let mut expired_rule = rule(
        policy::TEST_EXPIRED_RULE_ID,
        PolicyAction::Allow,
        policy::TEST_REASON_EXPIRED,
        20,
    );
    expired_rule.effective_until = Some(policy::TEST_EVALUATED_AT.to_string());

    let decision = evaluate_policy_dry_run(input_with_rules(
        vec![disabled_rule, expired_rule],
        Some(local_ai_result(
            PolicyAction::Block,
            LocalAiUnknownState::None,
            policy::TEST_REASON_AI_BLOCK,
        )),
        vec![evidence()],
    ));

    assert_eq!(decision.action, PolicyAction::Unknown);
    assert_eq!(decision.rule_ids, Vec::<String>::new());
    assert_eq!(
        decision.reason_codes,
        vec![
            policy::REASON_NO_MATCHING_PARENT_RULE.to_string(),
            policy::TEST_REASON_AI_BLOCK.to_string(),
        ]
    );
}

#[test]
fn dry_run_missing_evidence_returns_unknown_without_inventing_citations() {
    let decision = evaluate_policy_dry_run(input_with_rules(
        vec![rule(
            policy::TEST_BLOCK_RULE_ID,
            PolicyAction::Block,
            policy::TEST_REASON_PARENT_BLOCK,
            10,
        )],
        None,
        Vec::new(),
    ));

    assert_eq!(decision.action, PolicyAction::Unknown);
    assert_eq!(
        decision.reason_codes,
        vec![policy::REASON_MISSING_EVIDENCE.to_string()]
    );
    assert_eq!(
        decision.evidence_references,
        Vec::<ParentEvidenceReference>::new()
    );
    assert_eq!(decision.rule_ids, Vec::<String>::new());
    assert_eq!(decision.local_ai_result_id, None);
}

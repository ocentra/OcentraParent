#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyContractAction {
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "block")]
    Block,
    #[serde(rename = "time-limit")]
    TimeLimit,
    #[serde(rename = "ask-parent")]
    AskParent,
    #[serde(rename = "unknown")]
    Unknown,
}

pub fn compare_policy_action_strictness(
    left: PolicyContractAction,
    right: PolicyContractAction,
) -> i16 {
    policy_action_strictness_rank(left) - policy_action_strictness_rank(right)
}

pub fn select_stricter_policy_action(
    parent_rule_action: PolicyContractAction,
    local_ai_action: PolicyContractAction,
) -> PolicyContractAction {
    if compare_policy_action_strictness(parent_rule_action, local_ai_action) >= 0 {
        parent_rule_action
    } else {
        local_ai_action
    }
}

fn policy_action_strictness_rank(action: PolicyContractAction) -> i16 {
    match action {
        PolicyContractAction::Allow => 0,
        PolicyContractAction::Warn => 10,
        PolicyContractAction::Unknown => 20,
        PolicyContractAction::AskParent => 30,
        PolicyContractAction::TimeLimit => 40,
        PolicyContractAction::Block => 50,
    }
}

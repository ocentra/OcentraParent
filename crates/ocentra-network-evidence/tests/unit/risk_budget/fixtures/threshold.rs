use ocentra_network_evidence::risk_budget::*;

pub(super) fn threshold_input(
    signals: Vec<NetworkRiskBudgetSignal>,
    prior_events: Vec<NetworkRiskBudgetPriorEvent>,
    household_policy: NetworkRiskBudgetHouseholdPolicy,
    adapter_proof_state: NetworkRiskBudgetAdapterProofState,
) -> NetworkRiskBudgetThresholdInput {
    NetworkRiskBudgetThresholdInput {
        evaluation_ref: "network-risk-budget-row48".to_owned(),
        child_profile_ref: "child-profile-middle-school".to_owned(),
        risk_budget_ref: "household-network-risk-budget".to_owned(),
        cascade_ref: "network-cascade-row48".to_owned(),
        age_band: NetworkRiskBudgetAgeBand::UnderTwelve,
        profile_risk_weight_points: 5,
        thresholds: NetworkRiskBudgetThresholds {
            monitor_points: 20,
            ask_parent_points: 40,
            warn_child_points: 60,
            limit_points: 80,
            block_points: 100,
            max_points: 120,
        },
        household_policy,
        signals,
        prior_events,
        adapter_proof_state,
        raw_pcap_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        exact_url_claimed: false,
        private_message_claimed: false,
        search_query_claimed: false,
        policy_authority_claimed: false,
        adapter_authority_claimed: false,
        enforcement_command_claimed: false,
        extra_privilege_grant_claimed: false,
        allowance_grant_claimed: false,
        time_grant_claimed: false,
    }
}

pub(super) fn default_policy() -> NetworkRiskBudgetHouseholdPolicy {
    NetworkRiskBudgetHouseholdPolicy {
        household_policy_ref: "household-policy-network-risk".to_owned(),
        parent_rule_refs: vec!["parent-rule-network-review".to_owned()],
        child_warning_allowed: true,
        limit_policy_allowed: true,
        block_policy_allowed: true,
        strict_block_policy_enabled: true,
        safe_behavior_credit_cap_points: 30,
        safe_behavior_credit_expiry_ref: Some("safe-credit-expiry-row48".to_owned()),
        safe_behavior_audit_reason_ref: Some("safe-credit-audit-reason-row48".to_owned()),
        safe_behavior_ui_explanation_ref: Some("safe-credit-ui-explanation-row48".to_owned()),
    }
}

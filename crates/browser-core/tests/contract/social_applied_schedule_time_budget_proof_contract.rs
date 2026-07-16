use ocentra_browser_core::social_applied_schedule_time_budget_proof::social_applied_schedule_time_budget_proof_typescript;

#[test]
fn social_applied_schedule_time_budget_proof_stays_rust_owned_without_policy_value_owner() {
    let source = social_applied_schedule_time_budget_proof_typescript();

    assert_eq!(
        source
            .matches("@ocentra-parent/schema-domain/policy-compiler';")
            .count(),
        0
    );
    assert_eq!(
        source
            .matches("@ocentra-parent/schema-domain/social-policy-compiler-values';")
            .count(),
        0
    );
    assert_eq!(
        source
            .matches("} from './social_applied_schedule_time_budget_proof_support';")
            .count(),
        1
    );
}

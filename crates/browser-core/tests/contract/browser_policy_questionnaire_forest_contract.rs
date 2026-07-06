use ocentra_browser_core::browser_policy_questionnaire_forest::browser_policy_questionnaire_forest_contract_typescript;

#[test]
fn browser_policy_questionnaire_forest_contract_typescript_stays_checked_in() {
    assert_eq!(
        include_str!(
            "../../../../packages/schema-domain/src/generated-browser-policy-questionnaire-forest-contract.ts"
        ),
        browser_policy_questionnaire_forest_contract_typescript()
    );
}

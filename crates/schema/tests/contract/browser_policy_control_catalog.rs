use ocentra_schema::browser_policy_control_catalog_ts::{
    browser_policy_control_catalog_contracts_typescript,
    browser_policy_control_catalog_helpers_typescript,
};

#[test]
fn generated_browser_policy_control_catalog_contracts_stay_checked_in() {
    let checked_in = include_str!(
        "../../../../packages/schema-domain/src/generated-browser-policy-control-catalog-contracts.ts"
    );
    let generated = browser_policy_control_catalog_contracts_typescript();

    assert_eq!(checked_in, generated);
}

#[test]
fn generated_browser_policy_control_catalog_helpers_stay_checked_in() {
    let checked_in = include_str!(
        "../../../../packages/schema-domain/src/generated-browser-policy-control-catalog-helpers.ts"
    );
    let generated = browser_policy_control_catalog_helpers_typescript();

    assert_eq!(checked_in, generated);
}

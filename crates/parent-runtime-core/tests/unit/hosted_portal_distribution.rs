use ocentra_parent_runtime_core::hosted_portal_distribution::hosted_portal_distribution_typescript;

#[test]
fn hosted_portal_distribution_generated_typescript_stays_checked_in() {
    let checked_in = include_str!(
        "../../../../packages/portal-domain/src/generated/hosted-portal-distribution.ts"
    );

    assert_eq!(checked_in, hosted_portal_distribution_typescript());
}

use ocentra_schema::policy_enforcement_ts::{
    generated_enforcement_typescript, generated_policy_typescript,
};

#[test]
fn generated_typescript_policy_stays_checked_in() {
    let checked_in = include_str!("../../../../packages/schema-domain/src/generated-policy.ts");
    let generated = generated_policy_typescript();

    assert_eq!(checked_in, generated);
}

#[test]
fn generated_typescript_enforcement_stays_checked_in() {
    let checked_in =
        include_str!("../../../../packages/schema-domain/src/generated-enforcement.ts");
    let generated = generated_enforcement_typescript();

    assert_eq!(checked_in, generated);
}

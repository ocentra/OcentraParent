pub fn social_policy_compiler_contract_typescript() -> String {
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../browser-core-generated/social_policy_compiler_contract.ts"
    ))
    .to_string()
}

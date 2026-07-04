pub fn social_applied_schedule_time_budget_proof_typescript() -> String {
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../browser-core-generated/social_applied_schedule_time_budget_proof.ts"
    ))
    .to_string()
}

pub fn social_alert_report_preference_preflight_typescript() -> String {
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../browser-core-generated/social_alert_report_preference_preflight.ts"
    ))
    .to_string()
}

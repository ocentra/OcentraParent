pub fn social_alert_report_provider_dispatch_execution_typescript() -> String {
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../browser-core-generated/social_alert_report_provider_dispatch_execution.ts"
    ))
    .to_string()
}

pub fn social_alert_report_local_outbox_bridge_typescript() -> String {
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../browser-core-generated/social_alert_report_local_outbox_bridge.ts"
    ))
    .to_string()
}

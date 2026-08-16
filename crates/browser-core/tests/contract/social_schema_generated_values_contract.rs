use ocentra_browser_core::social_schema_generated_values::{
    social_alert_report_intent_values_typescript,
    social_audit_explanation_read_model_values_typescript, social_dashboard_ux_values_typescript,
    social_policy_compiler_values_typescript,
};

#[test]
fn social_alert_report_intent_values_stay_rust_owned() {
    let source = social_alert_report_intent_values_typescript();

    assert_eq!(
        source.to_string().lines().next(),
        Some("/* generated from crates/browser-core/src/social_schema_generated_values.rs */")
    );
}

#[test]
fn social_audit_explanation_read_model_values_stay_rust_owned() {
    let source = social_audit_explanation_read_model_values_typescript();

    assert_eq!(
        source.to_string().lines().next(),
        Some("/* generated from crates/browser-core/src/social_schema_generated_values.rs */")
    );
}

#[test]
fn social_dashboard_ux_values_stay_rust_owned() {
    let source = social_dashboard_ux_values_typescript();

    assert_eq!(
        source.to_string().lines().next(),
        Some("/* generated from crates/browser-core/src/social_schema_generated_values.rs */")
    );
}

#[test]
fn social_policy_compiler_values_stay_rust_owned() {
    let source = social_policy_compiler_values_typescript();

    assert_eq!(
        source.to_string().lines().next(),
        Some("/* generated from crates/browser-core/src/social_schema_generated_values.rs */")
    );
}

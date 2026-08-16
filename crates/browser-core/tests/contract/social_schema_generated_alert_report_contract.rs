use ocentra_browser_core::social_schema_generated_alert_report::{
    social_alert_report_intent_typescript, social_alert_report_provider_preflight_proof_typescript,
    social_alert_report_provider_receipt_boundary_proof_typescript,
    social_alert_report_provider_status_handoff_proof_typescript,
};

#[test]
fn social_alert_report_intent_stay_rust_owned() {
    let source = social_alert_report_intent_typescript();

    assert_eq!(
        source.to_string().lines().next(),
        Some(
            "/* generated from crates/browser-core/src/social_schema_generated_alert_report.rs */"
        )
    );
}

#[test]
fn social_alert_report_provider_preflight_proof_stay_rust_owned() {
    let source = social_alert_report_provider_preflight_proof_typescript();

    assert_eq!(
        source.to_string().lines().next(),
        Some(
            "/* generated from crates/browser-core/src/social_schema_generated_alert_report.rs */"
        )
    );
}

#[test]
fn social_alert_report_provider_receipt_boundary_proof_stay_rust_owned() {
    let source = social_alert_report_provider_receipt_boundary_proof_typescript();

    assert_eq!(
        source.to_string().lines().next(),
        Some(
            "/* generated from crates/browser-core/src/social_schema_generated_alert_report.rs */"
        )
    );
}

#[test]
fn social_alert_report_provider_status_handoff_proof_stay_rust_owned() {
    let source = social_alert_report_provider_status_handoff_proof_typescript();

    assert_eq!(
        source.to_string().lines().next(),
        Some(
            "/* generated from crates/browser-core/src/social_schema_generated_alert_report.rs */"
        )
    );
}

use ocentra_browser_core::social_alert_report_preference_preflight::social_alert_report_preference_preflight_typescript;

#[test]
fn social_alert_report_preference_preflight_remains_generated_and_rust_loaded() {
    let source = social_alert_report_preference_preflight_typescript();

    assert_eq!(
        source.lines().next(),
        Some(
            "/* generated from crates/browser-core/src/social_alert_report_preference_preflight.rs */"
        )
    );
    assert_eq!(
        source
            .matches("export function buildSocialAlertReportPreferencePreflightReadModel(")
            .count(),
        1
    );
}

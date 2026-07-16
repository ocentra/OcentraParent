use ocentra_browser_core::social_alert_report_preference_status_handoff::social_alert_report_preference_status_handoff_typescript;

#[test]
fn social_alert_report_preference_status_handoff_remains_generated_and_rust_loaded() {
    let source = social_alert_report_preference_status_handoff_typescript();

    assert_eq!(
        source.lines().next(),
        Some(
            "/* generated from crates/browser-core/src/social_alert_report_preference_status_handoff.rs */"
        )
    );
    assert_eq!(
        source
            .matches("export function buildSocialAlertReportPreferenceStatusHandoffReadModel(")
            .count(),
        1
    );
}
